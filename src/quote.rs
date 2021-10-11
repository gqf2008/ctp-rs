mod spi;

use crate::ffi::*;
use anyhow::anyhow;
use anyhow::Result;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct QuoteApi {
    api: *mut CThostFtdcMdApi,
    stub: Option<*mut QuoteSpiStub>,
}

impl Drop for QuoteApi {
    fn drop(&mut self) {
        unsafe { Quote_Release(self.api) };
        if let Some(stub) = self.stub {
            unsafe { QuoteSpiStub_Destructor(stub) };
        }
    }
}

unsafe impl Send for QuoteApi {}
unsafe impl Sync for QuoteApi {}

impl QuoteApi {
    pub fn new(path: &str, udp: bool, multicast: bool) -> Result<Self> {
        let path = CString::new(path)?;
        let api = unsafe { CreateFtdcMdApi(path.as_ptr(), udp, multicast) };
        Ok(Self { api, stub: None })
    }

    pub fn version<'a>() -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Quote_GetApiVersion();
            CStr::from_ptr(ptr).to_str()
        }
    }

    pub fn init(&self) {
        unsafe { Quote_Init(self.api) }
    }

    pub fn join(&self) -> Result<()> {
        let ret = unsafe { Quote_Join(self.api) };
        if ret == 0 {
            Ok(())
        } else {
            Err(anyhow!("join error {}", ret))
        }
    }
    pub fn get_trading_day<'a>(&'a self) -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Quote_GetTradingDay(self.api);
            CStr::from_ptr(ptr).to_str()
        }
    }
    pub fn register_front(&self, addr: &str) -> Result<()> {
        let addr = CString::new(addr)?;
        unsafe {
            Quote_RegisterFront(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }
    pub fn register_name_server(&self, addr: &str) -> Result<()> {
        let addr = CString::new(addr)?;
        unsafe {
            Quote_RegisterNameServer(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }
    pub fn register_fens_user_info(&self, broker_id: &str, user_id: &str, mode: i8) -> Result<()> {
        let mut info = CThostFtdcFensUserInfoField::default();
        unsafe {
            info.BrokerID
                .clone_from_slice(std::mem::transmute(broker_id));
            info.UserID.clone_from_slice(std::mem::transmute(user_id));
            info.LoginMode = mode;
            Quote_RegisterFensUserInfo(self.api, &mut info);
        }
        Ok(())
    }
    // extern "C" void Quote_RegisterFensUserInfo(CThostFtdcMdApi *self, CThostFtdcFensUserInfoField *pFensUserInfo);
    // extern "C" void Quote_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi *pSpi);
    // extern "C" int Quote_SubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
    // extern "C" int Quote_UnSubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
    // extern "C" int Quote_SubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
    // extern "C" int Quote_UnSubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
    // extern "C" int Quote_ReqUserLogin(CThostFtdcMdApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID);
    // extern "C" int Quote_ReqUserLogout(CThostFtdcMdApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID);
    // extern "C" int Quote_ReqQryMulticastInstrument(CThostFtdcMdApi *self, CThostFtdcQryMulticastInstrumentField *pQryMulticastInstrument, int nRequestID);

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.stub = Some(ptr);
        unsafe { Quote_RegisterSpi(self.api, ptr as *mut CThostFtdcMdSpi) };
    }
}

pub trait QuoteSpi {
    ///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    fn on_connected(&self) {
        log::debug!("on_connected");
    }
    ///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
    ///@param nReason 错误原因
    ///        0x1001 网络读失败
    ///        0x1002 网络写失败
    ///        0x2001 接收心跳超时
    ///        0x2002 发送心跳失败
    ///        0x2003 收到错误报文
    fn on_disconnected(&self, reason: i32) {
        log::debug!("on_disconnected reason {}", reason);
    }
    ///心跳超时警告。当长时间未收到报文时，该方法被调用。
    ///@param nTimeLapse 距离上次接收报文的时间
    fn on_heart_beat_warning(&self, timelapse: i32) {
        log::debug!("on_disconnected timelapse {}", timelapse);
    }
    ///错误应答
    fn on_error(&self, result: &CThostFtdcRspInfoField, req_id: i32, is_last: bool) {
        log::debug!(
            "on_login req_id {} is_last {} result {:?}",
            req_id,
            is_last,
            result
        );
    }
    ///登录请求响应
    fn on_login(
        &self,
        info: &CThostFtdcRspUserLoginField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_login req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///登出请求响应
    fn on_logout(
        &self,
        info: &CThostFtdcUserLogoutField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_logout req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///请求查询组播合约响应
    fn on_query_multicast_instrument(
        &self,
        info: &CThostFtdcMulticastInstrumentField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_query_multicast_instrument req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///订阅行情应答
    fn on_sub_market_data(
        &self,
        info: &CThostFtdcSpecificInstrumentField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_sub_market_data req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///取消订阅行情应答
    fn on_unsub_market_data(
        &self,
        info: &CThostFtdcSpecificInstrumentField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_unsub_market_data req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///订阅询价应答
    fn on_sub_for_quote(
        &self,
        info: &CThostFtdcSpecificInstrumentField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_sub_for_quote req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///取消订阅询价应答
    fn on_unsub_for_quote(
        &self,
        info: &CThostFtdcSpecificInstrumentField,
        result: &CThostFtdcRspInfoField,
        req_id: i32,
        is_last: bool,
    ) {
        log::debug!(
            "on_unsub_for_quote req_id {} is_last {} info {:?} result {:?}",
            req_id,
            is_last,
            info,
            result
        );
    }
    ///深度行情通知
    fn on_depth_market_data(&self, info: &CThostFtdcDepthMarketDataField) {
        log::debug!("on_depth_market_data info {:?}", info);
    }
    ///询价通知
    fn on_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::debug!("on_for_quote info {:?}", info);
    }
}
