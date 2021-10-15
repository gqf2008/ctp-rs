mod stub_ffi;

use super::{Configuration, Response};
use crate::ffi::*;
use crate::{FromCBuf, ToArray};
use anyhow::anyhow;
use anyhow::Result;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

impl Drop for QuoteSpiStub {
    fn drop(&mut self) {
        // unsafe { self.destruct() }
        unsafe { QuoteSpiStub_Destructor(self) }
        //unreachable!("QuoteSpiStub should be manually dropped!")
    }
}

pub struct QuoteApi {
    api: *mut CThostFtdcMdApi,
    stub: Option<*mut QuoteSpiStub>,
    seq: AtomicUsize,
    conf: Configuration,
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
        Ok(Self {
            api,
            stub: None,
            seq: AtomicUsize::new(0),
            conf: Default::default(),
        })
    }

    pub fn with_configuration(mut self, conf: Configuration) -> Self {
        self.conf = conf;
        self
    }

    pub fn version<'a>() -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Quote_GetApiVersion();
            CStr::from_ptr(ptr).to_str()
        }
    }

    pub fn register_front(&self) -> Result<()> {
        let addr = CString::new(self.conf.front_addr.as_str())?;
        unsafe {
            Quote_RegisterFront(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_name_server(&self) -> Result<()> {
        let addr = CString::new(self.conf.ns_addr.as_str())?;
        unsafe {
            Quote_RegisterNameServer(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_fens_user_info(&self) -> Result<()> {
        let mut info = CThostFtdcFensUserInfoField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.LoginMode = self.conf.login_mode;
            Quote_RegisterFensUserInfo(self.api, &mut info);
        }
        Ok(())
    }

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.stub = Some(ptr);
        unsafe { Quote_RegisterSpi(self.api, ptr as *mut CThostFtdcMdSpi) };
    }

    pub fn init(&self) {
        unsafe { Quote_Init(self.api) }
    }

    pub fn login(&self) -> Result<()> {
        let mut info = <CThostFtdcReqUserLoginField as From<&Configuration>>::from(&self.conf);
        match unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Quote_ReqUserLogin(self.api, &mut info, seq as i32)
        } {
            0 => Ok(()),
            ret => Err(anyhow!("Error({})", ret)),
        }
    }

    pub fn logout(&self) -> Result<()> {
        let mut info = CThostFtdcUserLogoutField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);

            Quote_ReqUserLogout(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn wait(&self) -> Result<()> {
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

    pub fn subscribe_market_data(&self, symbols: &[&str]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| CString::new(*symbol).unwrap().as_c_str().as_ptr() as *mut c_char)
                .collect();
            Quote_SubscribeMarketData(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn unsubscribe_market_data(&self, symbols: &[&str]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| CString::new(*symbol).unwrap().as_c_str().as_ptr() as *mut c_char)
                .collect();
            Quote_UnSubscribeMarketData(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn subscribe_for_quote(&self, symbols: &[&str]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| CString::new(*symbol).unwrap().as_c_str().as_ptr() as *mut c_char)
                .collect();
            Quote_SubscribeForQuoteRsp(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn unsubscribe_for_quote(&self, symbols: &[&str]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| CString::new(*symbol).unwrap().as_c_str().as_ptr() as *mut c_char)
                .collect();
            Quote_UnSubscribeForQuoteRsp(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn query_multicast_instrument(&self, symbol: &str, topic_id: i32) -> Result<()> {
        let mut info = CThostFtdcQryMulticastInstrumentField::default();
        unsafe {
            info.InstrumentID = symbol.into_array::<81>();
            info.TopicID = topic_id;
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Quote_ReqQryMulticastInstrument(self.api, &mut info, seq as i32);
        }
        Ok(())
    }
}

pub trait QuoteSpi: Send {
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
    fn on_heartbeat_warning(&self, timelapse: i32) {
        log::debug!("on_disconnected timelapse {}", timelapse);
    }
    ///错误应答
    fn on_error(&self, result: &Response) {
        log::debug!("on_login result {:?}", result);
    }
    ///登录请求响应
    fn on_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::debug!(
            "on_login tradingday {} {:?} {:?}",
            String::from_c_buf(&info.TradingDay),
            info,
            result
        );
    }
    ///登出请求响应
    fn on_logout(&self, info: &CThostFtdcUserLogoutField, result: &Response) {
        log::debug!("on_logout info {:?} result {:?}", info, result);
    }
    ///请求查询组播合约响应
    fn on_query_multicast_instrument(
        &self,
        info: &CThostFtdcMulticastInstrumentField,
        result: &Response,
    ) {
        log::debug!(
            "on_query_multicast_instrument info {:?} result {:?}",
            info,
            result
        );
    }
    ///订阅行情应答
    fn on_sub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        println!("reserve1:{}", String::from_c_buf(&info.reserve1),);
        log::debug!("on_sub_market_data info {:?} result {:?}", info, result);
    }
    ///取消订阅行情应答
    fn on_unsub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("on_unsub_market_data info {:?} result {:?}", info, result);
    }
    ///订阅询价应答
    fn on_sub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        println!("reserve1:{}", String::from_c_buf(&info.reserve1));
        log::debug!("on_sub_for_quote info {:?} result {:?}", info, result);
    }

    ///取消订阅询价应答
    fn on_unsub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("on_unsub_for_quote info {:?} result {:?}", info, result);
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
