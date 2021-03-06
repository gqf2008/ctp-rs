mod stub_ffi;

use super::{Configuration, Response};
use crate::ffi::*;
use crate::{FromCBuf, ToArray};
use anyhow::anyhow;
use anyhow::Result;
use libc::c_void;
use std::any;
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
    pub fn version<'a>() -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Quote_GetApiVersion();
            CStr::from_ptr(ptr).to_str()
        }
    }

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

    pub fn with_spi<T: QuoteSpi>(mut self, spi: T) -> Self {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.stub = Some(ptr);
        unsafe { Quote_RegisterSpi(self.api, ptr as *mut CThostFtdcMdSpi) };
        self
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

    pub fn subscribe_market_data(&self, symbols: &[CString]) -> Result<()> {
        match unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| symbol.as_ptr() as *mut c_char)
                .collect();
            Quote_SubscribeMarketData(self.api, symbols.as_mut_ptr(), symbols.len() as i32)
        } {
            0 => Ok(()),
            ret => Err(anyhow!("error {}", ret)),
        }
    }

    pub fn unsubscribe_market_data(&self, symbols: &[CString]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| symbol.as_ptr() as *mut c_char)
                .collect();
            Quote_UnSubscribeMarketData(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn subscribe_for_quote(&self, symbols: &[CString]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| symbol.as_ptr() as *mut c_char)
                .collect();
            Quote_SubscribeForQuoteRsp(self.api, symbols.as_mut_ptr(), symbols.len() as i32);
        }
        Ok(())
    }

    pub fn unsubscribe_for_quote(&self, symbols: &[CString]) -> Result<()> {
        unsafe {
            let mut symbols: Vec<*mut c_char> = symbols
                .iter()
                .map(|symbol| symbol.as_ptr() as *mut c_char)
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
    ///????????????????????????????????????????????????????????????????????????????????????????????????
    fn on_connected(&self) {
        log::trace!("on_connected");
    }
    ///???????????????????????????????????????????????????????????????????????????????????????????????????API???????????????????????????????????????????????????
    ///@param nReason ????????????
    ///        0x1001 ???????????????
    ///        0x1002 ???????????????
    ///        0x2001 ??????????????????
    ///        0x2002 ??????????????????
    ///        0x2003 ??????????????????
    fn on_disconnected(&self, reason: i32) {
        log::trace!("on_disconnected reason {}", reason);
    }
    ///???????????????????????????????????????????????????????????????????????????
    ///@param nTimeLapse ?????????????????????????????????
    fn on_heartbeat_warning(&self, timelapse: i32) {
        log::trace!("on_disconnected timelapse {}", timelapse);
    }
    ///????????????
    fn on_error(&self, result: &Response) {
        log::trace!("on_login result {:?}", result);
    }
    ///??????????????????
    fn on_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::trace!(
            "on_login tradingday {} {:?} {:?}",
            String::from_c_buf(&info.TradingDay),
            info,
            result
        );
    }
    ///??????????????????
    fn on_logout(&self, info: &CThostFtdcUserLogoutField, result: &Response) {
        log::trace!("on_logout info {:?} result {:?}", info, result);
    }
    ///??????????????????????????????
    fn on_query_multicast_instrument(
        &self,
        info: &CThostFtdcMulticastInstrumentField,
        result: &Response,
    ) {
        log::trace!(
            "on_query_multicast_instrument info {:?} result {:?}",
            info,
            result
        );
    }
    ///??????????????????
    fn on_sub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        println!("reserve1:{}", String::from_c_buf(&info.reserve1),);
        log::trace!("on_sub_market_data info {:?} result {:?}", info, result);
    }
    ///????????????????????????
    fn on_unsub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::trace!("on_unsub_market_data info {:?} result {:?}", info, result);
    }
    ///??????????????????
    fn on_sub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        println!("reserve1:{}", String::from_c_buf(&info.reserve1));
        log::trace!("on_sub_for_quote info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_unsub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::trace!("on_unsub_for_quote info {:?} result {:?}", info, result);
    }
    ///??????????????????
    fn on_depth_market_data(&self, info: &CThostFtdcDepthMarketDataField) {
        log::trace!("on_depth_market_data info {:?}", info);
    }
    ///????????????
    fn on_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::trace!("on_for_quote info {:?}", info);
    }
}
