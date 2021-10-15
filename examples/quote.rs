use anyhow::Result;

use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, QuoteApi, QuoteSpi, Response, TradeApi};
use std::path::PathBuf;
use structopt::StructOpt;

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(long, default_value = "DEBUG")]
    level: String,
    #[structopt(name = "broker_id", long, default_value = "9999")]
    broker_id: String,
    #[structopt(name = "user_id", short, long)]
    user_id: String,
    #[structopt(long, default_value = "simnow_client_test")]
    appid: String,
    #[structopt(long, default_value = "0000000000000000")]
    auth_code: String,
    #[structopt(
        name = "quote_addr",
        long,
        default_value = "tcp://180.168.146.187:10131"
    )]
    quote_addr: String,
    #[structopt(short, long)]
    passwd: String,
    /// Output file
    #[structopt(long, parse(from_os_str), default_value = "./")]
    qpath: PathBuf,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    udp: bool,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    multicast: bool,
}

//https://www.simnow.com.cn/product.action
fn main() -> Result<()> {
    let opt = Opt::from_args();
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", opt.level)
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    log::info!("quote.api {}", QuoteApi::version()?);
    log::info!("trade.api {}", TradeApi::version()?);
    let (tx, rx) = channel::bounded(256);
    let mtx = tx.clone();
    let mut qapi = QuoteApi::new(opt.qpath.to_str().unwrap_or("./"), opt.udp, opt.multicast)?
        .with_configuration(Configuration {
            broker_id: opt.broker_id,
            user_id: opt.user_id,
            appid: opt.appid,
            auth_code: opt.auth_code,
            front_addr: opt.quote_addr,
            passwd: opt.passwd,
            ..Default::default()
        });
    qapi.register_front()?;
    qapi.register_fens_user_info()?;
    qapi.register_spi(Myquote(tx));
    qapi.init();
    loop {
        rx.iter().for_each(|ev| match ev {
            Event::Quote(q) => {
                log::info!(
                    r#"
                    TradingDay:{},ActionDay:{},ExchangeID:{},ExchangeInstID:{},InstrumentID:{},UpdateTime:{}
                    Avg: {},Last: {},Volume:{},Turnover:{}
                    Open({}),High({}),Low({}),Close({}) 
                    Ask1:{},{},Bid1:{},{}
                    Ask2:{},{},Bid2:{},{}
                    Ask3:{},{},Bid3:{},{}
                    Ask4:{},{},Bid4:{},{}
                    Ask5:{},{},Bid5:{},{}
                    "#,
                    String::from_c_buf(&q.ActionDay),
                    String::from_c_buf(&q.ExchangeID),
                    String::from_c_buf(&q.ExchangeInstID),
                    String::from_c_buf(&q.InstrumentID),
                    String::from_c_buf(&q.UpdateTime),
                    String::from_c_buf(&q.TradingDay),
                    q.AveragePrice,q.LastPrice,
                    q.Volume,q.Turnover,
                    q.OpenPrice,q.HighestPrice,q.LowestPrice,q.ClosePrice,
                    q.AskPrice1,
                    q.AskVolume1,
                    q.BidPrice1,
                    q.BidVolume1,
                    q.AskPrice2,
                    q.AskVolume2,
                    q.BidPrice2,
                    q.BidVolume2,
                    q.AskPrice3,
                    q.AskVolume3,
                    q.BidPrice3,
                    q.BidVolume3,
                    q.AskPrice4,
                    q.AskVolume4,
                    q.BidPrice4,
                    q.BidVolume4,
                    q.AskPrice5,
                    q.AskVolume5,
                    q.BidPrice5,
                    q.BidVolume5,
                );
            }
            Event::Login(login) => {
                log::info!("{:?}", login);
                qapi.subscribe_market_data(&["ag2110", "ag2111"]).ok();
                qapi.subscribe_for_quote(&["ag2110", "ag2111"]).ok();
            }
            Event::Connected => {
                if let Err(err) = qapi.login() {
                    log::error!("{}", err);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    mtx.send(Event::Connected).ok();
                }
            }
            Event::Error(err, msg) => {
                log::error!("{} {}", err, msg);
            }
            Event::Disconnected(reason) => {
                log::error!("{}", reason);
            }
            _ => {
                log::debug!("{:?}", ev);
            }
        });
    }
    //qapi.wait()
}

#[derive(Debug)]
enum Event {
    Connected,
    Disconnected(i32),
    HeartbeatWarn(i32),
    Error(i32, String),
    Login(CThostFtdcRspUserLoginField),
    Logout(CThostFtdcUserLogoutField),
    Quote(CThostFtdcDepthMarketDataField),
}
#[derive(Debug, Clone)]
struct Myquote(Sender<Event>);

impl QuoteSpi for Myquote {
    ///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    fn on_connected(&self) {
        self.0.send(Event::Connected).ok();
    }
    ///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
    ///@param nReason 错误原因
    ///        0x1001 网络读失败
    ///        0x1002 网络写失败
    ///        0x2001 接收心跳超时
    ///        0x2002 发送心跳失败
    ///        0x2003 收到错误报文
    fn on_disconnected(&self, reason: i32) {
        self.0.send(Event::Disconnected(reason)).ok();
    }
    ///心跳超时警告。当长时间未收到报文时，该方法被调用。
    ///@param nTimeLapse 距离上次接收报文的时间
    fn on_heartbeat_warning(&self, timelapse: i32) {
        self.0.send(Event::HeartbeatWarn(timelapse)).ok();
    }
    ///错误应答
    fn on_error(&self, result: &Response) {
        self.0
            .send(Event::Error(result.code, result.message.clone()))
            .ok();
    }
    ///登录请求响应
    fn on_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::debug!(
            "tradingday {} {:?} result {:?}",
            String::from_c_buf(&info.TradingDay),
            info,
            result
        );
        if result.code != 0 {
            self.0
                .send(Event::Error(result.code, result.message.clone()))
                .ok();
        } else {
            self.0.send(Event::Login(info.clone())).ok();
        }
    }
    ///登出请求响应
    fn on_logout(&self, info: &CThostFtdcUserLogoutField, result: &Response) {
        if result.code != 0 {
            self.0
                .send(Event::Error(result.code, result.message.clone()))
                .ok();
        } else {
            self.0.send(Event::Logout(info.clone())).ok();
        }
    }
    ///请求查询组播合约响应
    fn on_query_multicast_instrument(
        &self,
        info: &CThostFtdcMulticastInstrumentField,
        result: &Response,
    ) {
        log::debug!("{:?} {:?}", info, result);
    }
    ///订阅行情应答
    fn on_sub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("{:?} {:?}", info, result);
    }
    ///取消订阅行情应答
    fn on_unsub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("{:?} {:?}", info, result);
    }
    ///订阅询价应答
    fn on_sub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("{:?} {:?}", info, result);
    }

    ///取消订阅询价应答
    fn on_unsub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!("{:?} {:?}", info, result);
    }
    ///深度行情通知
    fn on_depth_market_data(&self, info: &CThostFtdcDepthMarketDataField) {
        self.0.send(Event::Quote(info.clone())).ok();
    }
    ///询价通知
    fn on_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::debug!("{:?}", info);
    }
}
