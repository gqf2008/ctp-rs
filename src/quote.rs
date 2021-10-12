mod stub_ffi;

use super::FromCBuf;
use crate::ffi::*;
use anyhow::anyhow;
use anyhow::Result;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct QuoteApi {
    api: *mut CThostFtdcMdApi,
    stub: Option<*mut QuoteSpiStub>,
    seq: AtomicUsize,
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
        })
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

    pub fn register_spi<T: QuoteSpi>(&mut self, spi: T) {
        let trait_object_box: Box<Box<dyn QuoteSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn QuoteSpi> as *mut c_void;

        let quote_spi_stub = unsafe { QuoteSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(quote_spi_stub));
        self.stub = Some(ptr);
        unsafe { Quote_RegisterSpi(self.api, ptr as *mut CThostFtdcMdSpi) };
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

    pub fn login(&self, broker_id: &str, user_id: &str, mode: i8) -> Result<()> {
        let mut info = CThostFtdcReqUserLoginField::default();
        unsafe {
            info.BrokerID
                .clone_from_slice(std::mem::transmute(broker_id));
            info.UserID.clone_from_slice(std::mem::transmute(user_id));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);

            Quote_ReqUserLogin(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn logout(&self, broker_id: &str, user_id: &str, mode: i8) -> Result<()> {
        let mut info = CThostFtdcUserLogoutField::default();
        unsafe {
            info.BrokerID
                .clone_from_slice(std::mem::transmute(broker_id));
            info.UserID.clone_from_slice(std::mem::transmute(user_id));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);

            Quote_ReqUserLogout(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn query_multicast_instrument(&self, symbol: &str, topic_id: i32) -> Result<()> {
        let mut info = CThostFtdcQryMulticastInstrumentField::default();
        unsafe {
            info.InstrumentID
                .clone_from_slice(std::mem::transmute(symbol));
            info.TopicID = topic_id;
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Quote_ReqQryMulticastInstrument(self.api, &mut info, seq as i32);
        }
        Ok(())
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
    fn on_error(&self, result: &Response) {
        log::debug!("on_login result {:?}", result);
    }
    ///登录请求响应
    fn on_login(&self, info: &UserLogin, result: &Response) {
        log::debug!("on_login info {:?} result {:?}", info, result);
    }
    ///登出请求响应
    fn on_logout(&self, info: &UserLogout, result: &Response) {
        log::debug!("on_logout info {:?} result {:?}", info, result);
    }
    ///请求查询组播合约响应
    fn on_query_multicast_instrument(&self, info: &MulticastInstrument, result: &Response) {
        log::debug!(
            "on_query_multicast_instrument info {:?} result {:?}",
            info,
            result
        );
    }
    ///订阅行情应答
    fn on_sub_market_data(&self, info: &SpecificInstrument, result: &Response) {
        log::debug!("on_sub_market_data info {:?} result {:?}", info, result);
    }
    ///取消订阅行情应答
    fn on_unsub_market_data(&self, info: &SpecificInstrument, result: &Response) {
        log::debug!("on_unsub_market_data info {:?} result {:?}", info, result);
    }
    ///订阅询价应答
    fn on_sub_for_quote(&self, info: &SpecificInstrument, result: &Response) {
        log::debug!("on_sub_for_quote info {:?} result {:?}", info, result);
    }

    ///取消订阅询价应答
    fn on_unsub_for_quote(&self, info: &SpecificInstrument, result: &Response) {
        log::debug!("on_unsub_for_quote info {:?} result {:?}", info, result);
    }
    ///深度行情通知
    fn on_depth_market_data(&self, info: &DepthMarketData) {
        log::debug!("on_depth_market_data info {:?}", info);
    }
    ///询价通知
    fn on_for_quote(&self, info: &ForQuote) {
        log::debug!("on_for_quote info {:?}", info);
    }
}

#[doc = "接口应答"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    pub code: i32,
    pub message: String,
    pub req_id: Option<i32>,
    pub is_last: Option<bool>,
}

impl Response {
    pub fn with_req_id(mut self, req_id: i32) -> Self {
        self.req_id = Some(req_id);
        self
    }
    pub fn with_is_last(mut self, is_last: bool) -> Self {
        self.is_last = Some(is_last);
        self
    }
}

impl<'a> From<&'a CThostFtdcRspInfoField> for Response {
    fn from(field: &'a CThostFtdcRspInfoField) -> Self {
        Response {
            code: field.ErrorID,
            message: String::from_c_buf(&field.ErrorMsg),
            req_id: None,
            is_last: None,
        }
    }
}

#[doc = "用户登陆信息"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserLogin {
    #[doc = "交易日"]
    pub trading_day: String,
    #[doc = "登录成功时间"]
    pub login_time: String,
    #[doc = "经纪公司代码"]
    pub broker_id: String,
    #[doc = "用户代码"]
    pub user_id: String,
    #[doc = "交易系统名称"]
    pub system_name: String,
    #[doc = "前置编号"]
    pub front_id: i32,
    #[doc = "会话编号"]
    pub session_id: i32,
    #[doc = "最大报单引用"]
    pub max_order_ref: String,
    #[doc = "上期所时间"]
    pub shfe_time: String,
    #[doc = "大商所时间"]
    pub dce_time: String,
    #[doc = "郑商所时间"]
    pub czce_time: String,
    #[doc = "中金所时间"]
    pub ffex_time: String,
    #[doc = "能源中心时间"]
    pub ine_time: String,
}

impl<'a> From<&'a CThostFtdcRspUserLoginField> for UserLogin {
    fn from(field: &'a CThostFtdcRspUserLoginField) -> Self {
        UserLogin {
            front_id: field.FrontID,
            session_id: field.SessionID,
            trading_day: String::from_c_buf(&field.TradingDay),
            login_time: String::from_c_buf(&field.LoginTime),
            system_name: String::from_c_buf(&field.SystemName),
            max_order_ref: String::from_c_buf(&field.MaxOrderRef),
            shfe_time: String::from_c_buf(&field.SHFETime),
            dce_time: String::from_c_buf(&field.DCETime),
            czce_time: String::from_c_buf(&field.CZCETime),
            ffex_time: String::from_c_buf(&field.FFEXTime),
            ine_time: String::from_c_buf(&field.INETime),
            broker_id: String::from_c_buf(&field.BrokerID),
            user_id: String::from_c_buf(&field.UserID),
        }
    }
}

#[doc = "用户登出信息"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserLogout {
    pub broker_id: String,
    pub user_id: String,
}

impl<'a> From<&'a CThostFtdcUserLogoutField> for UserLogout {
    fn from(field: &'a CThostFtdcUserLogoutField) -> Self {
        UserLogout {
            broker_id: String::from_c_buf(&field.BrokerID),
            user_id: String::from_c_buf(&field.UserID),
        }
    }
}

#[doc = "组播合约信息"]
#[derive(Debug, Clone, PartialEq)]
pub struct MulticastInstrument {
    #[doc = "主题号"]
    pub topic_id: i32,
    #[doc = "合约编号"]
    pub instrument_no: i32,
    #[doc = "基准价"]
    pub code_price: f64,
    #[doc = "合约数量乘数"]
    pub volume_multiple: i32,
    #[doc = "最小变动价位"]
    pub price_tick: f64,
    #[doc = "合约代码"]
    pub instrument_id: String,
}

impl<'a> From<&'a CThostFtdcMulticastInstrumentField> for MulticastInstrument {
    fn from(field: &'a CThostFtdcMulticastInstrumentField) -> Self {
        MulticastInstrument {
            topic_id: field.TopicID,
            instrument_no: field.InstrumentNo,
            code_price: field.CodePrice,
            volume_multiple: field.VolumeMultiple,
            price_tick: field.PriceTick,
            instrument_id: String::from_c_buf(&field.InstrumentID),
        }
    }
}
#[doc = "指定的合约"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SpecificInstrument {
    #[doc = "合约代码"]
    pub instrument_id: String,
}

impl<'a> From<&'a CThostFtdcSpecificInstrumentField> for SpecificInstrument {
    fn from(field: &'a CThostFtdcSpecificInstrumentField) -> Self {
        SpecificInstrument {
            instrument_id: String::from_c_buf(&field.InstrumentID),
        }
    }
}

#[doc = "深度行情"]
#[derive(Debug, Clone, PartialEq)]
pub struct DepthMarketData {
    #[doc = "交易日"]
    pub trading_day: String,
    #[doc = "交易所代码"]
    pub exchange_id: String,
    #[doc = "最新价"]
    pub last_price: f64,
    #[doc = "上次结算价"]
    pub pre_settlement_price: f64,
    #[doc = "昨收盘"]
    pub pre_close_price: f64,
    #[doc = "昨持仓量"]
    pub pre_open_interest: f64,
    #[doc = "今开盘"]
    pub open_price: f64,
    #[doc = "最高价"]
    pub highest_price: f64,
    #[doc = "最低价"]
    pub lowest_price: f64,
    #[doc = "数量"]
    pub volume: i32,
    #[doc = "成交金额"]
    pub turnover: f64,
    #[doc = "持仓量"]
    pub open_interest: f64,
    #[doc = "今收盘"]
    pub close_price: f64,
    #[doc = "本次结算价"]
    pub settlement_price: f64,
    #[doc = "涨停板价"]
    pub upper_limit_price: f64,
    #[doc = "跌停板价"]
    pub lower_limit_price: f64,
    #[doc = "昨虚实度"]
    pub pre_delta: f64,
    #[doc = "今虚实度"]
    pub curr_delta: f64,
    #[doc = "最后修改时间"]
    pub update_time: String,
    #[doc = "最后修改毫秒"]
    pub update_millisec: i32,
    #[doc = "申买价一"]
    pub bid_price1: f64,
    #[doc = "申买量一"]
    pub bid_volume1: i32,
    #[doc = "申卖价一"]
    pub ask_price1: f64,
    #[doc = "申卖量一"]
    pub ask_volume1: i32,
    #[doc = "申买价二"]
    pub bid_price2: f64,
    #[doc = "申买量二"]
    pub bid_volume2: i32,
    #[doc = "申卖价二"]
    pub ask_price2: f64,
    #[doc = "申卖量二"]
    pub ask_volume2: i32,
    #[doc = "申买价三"]
    pub bid_price3: f64,
    #[doc = "申买量三"]
    pub bid_volume3: i32,
    #[doc = "申卖价三"]
    pub ask_price3: f64,
    #[doc = "申卖量三"]
    pub ask_volume3: i32,
    #[doc = "申买价四"]
    pub bid_price4: f64,
    #[doc = "申买量四"]
    pub bid_volume4: i32,
    #[doc = "申卖价四"]
    pub ask_price4: f64,
    #[doc = "申卖量四"]
    pub ask_volume4: i32,
    #[doc = "申买价五"]
    pub bid_price5: f64,
    #[doc = "申买量五"]
    pub bid_volume5: i32,
    #[doc = "申卖价五"]
    pub ask_price5: f64,
    #[doc = "申卖量五"]
    pub ask_volume5: i32,
    #[doc = "当日均价"]
    pub average_price: f64,
    #[doc = "业务日期"]
    pub action_day: String,
    #[doc = "合约代码"]
    pub instrument_id: String,
    #[doc = "合约在交易所的代码"]
    pub exchange_inst_id: String,
    #[doc = "上带价"]
    pub banding_upper_price: f64,
    #[doc = "下带价"]
    pub banding_lower_price: f64,
}

impl<'a> From<&'a CThostFtdcDepthMarketDataField> for DepthMarketData {
    fn from(field: &'a CThostFtdcDepthMarketDataField) -> Self {
        DepthMarketData {
            trading_day: String::from_c_buf(&field.TradingDay),
            exchange_id: String::from_c_buf(&field.ExchangeID),
            last_price: field.LastPrice,
            pre_settlement_price: field.PreSettlementPrice,
            pre_close_price: field.PreClosePrice,
            pre_open_interest: field.PreOpenInterest,
            open_price: field.OpenPrice,
            #[doc = "最高价"]
            highest_price: field.HighestPrice,
            #[doc = "最低价"]
            lowest_price: field.LowestPrice,
            #[doc = "数量"]
            volume: field.Volume,
            #[doc = "成交金额"]
            turnover: field.Turnover,
            #[doc = "持仓量"]
            open_interest: field.OpenInterest,
            #[doc = "今收盘"]
            close_price: field.ClosePrice,
            #[doc = "本次结算价"]
            settlement_price: field.SettlementPrice,
            #[doc = "涨停板价"]
            upper_limit_price: field.UpperLimitPrice,
            #[doc = "跌停板价"]
            lower_limit_price: field.LowerLimitPrice,
            #[doc = "昨虚实度"]
            pre_delta: field.PreDelta,
            #[doc = "今虚实度"]
            curr_delta: field.CurrDelta,
            #[doc = "最后修改时间"]
            update_time: String::from_c_buf(&field.UpdateTime),
            #[doc = "最后修改毫秒"]
            update_millisec: field.UpdateMillisec,
            #[doc = "申买价一"]
            bid_price1: field.BidPrice1,
            #[doc = "申买量一"]
            bid_volume1: field.BidVolume1,
            #[doc = "申卖价一"]
            ask_price1: field.AskPrice1,
            #[doc = "申卖量一"]
            ask_volume1: field.AskVolume1,
            #[doc = "申买价二"]
            bid_price2: field.BidPrice2,
            #[doc = "申买量二"]
            bid_volume2: field.BidVolume2,
            #[doc = "申卖价二"]
            ask_price2: field.AskPrice2,
            #[doc = "申卖量二"]
            ask_volume2: field.AskVolume2,
            #[doc = "申买价三"]
            bid_price3: field.BidPrice3,
            #[doc = "申买量三"]
            bid_volume3: field.BidVolume3,
            #[doc = "申卖价三"]
            ask_price3: field.AskPrice3,
            #[doc = "申卖量三"]
            ask_volume3: field.AskVolume3,
            #[doc = "申买价四"]
            bid_price4: field.BidPrice4,
            #[doc = "申买量四"]
            bid_volume4: field.BidVolume4,
            #[doc = "申卖价四"]
            ask_price4: field.AskPrice4,
            #[doc = "申卖量四"]
            ask_volume4: field.AskVolume4,
            #[doc = "申买价五"]
            bid_price5: field.BidPrice5,
            #[doc = "申买量五"]
            bid_volume5: field.BidVolume5,
            #[doc = "申卖价五"]
            ask_price5: field.AskPrice5,
            #[doc = "申卖量五"]
            ask_volume5: field.AskVolume5,
            #[doc = "当日均价"]
            average_price: field.AveragePrice,
            #[doc = "业务日期"]
            action_day: String::from_c_buf(&field.ActionDay),
            #[doc = "合约代码"]
            instrument_id: String::from_c_buf(&field.InstrumentID),
            exchange_inst_id: String::from_c_buf(&field.ExchangeInstID),
            banding_upper_price: field.BandingUpperPrice,
            banding_lower_price: field.BandingLowerPrice,
        }
    }
}

#[doc = "发给做市商的询价请求"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ForQuote {
    #[doc = "交易日"]
    pub trading_day: String,
    #[doc = "询价编号"]
    pub for_quote_sys_id: String,
    #[doc = "询价时间"]
    pub for_quote_time: String,
    #[doc = "业务日期"]
    pub action_day: String,
    #[doc = "交易所代码"]
    pub exchange_id: String,
    #[doc = "合约代码"]
    pub instrument_id: String,
}

impl<'a> From<&'a CThostFtdcForQuoteRspField> for ForQuote {
    fn from(field: &'a CThostFtdcForQuoteRspField) -> Self {
        ForQuote {
            trading_day: String::from_c_buf(&field.TradingDay),
            for_quote_sys_id: String::from_c_buf(&field.ForQuoteSysID),
            for_quote_time: String::from_c_buf(&field.ForQuoteTime),
            action_day: String::from_c_buf(&field.ActionDay),
            exchange_id: String::from_c_buf(&field.ExchangeID),
            instrument_id: String::from_c_buf(&field.InstrumentID),
        }
    }
}
