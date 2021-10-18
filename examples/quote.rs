use anyhow::Result;

use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, QuoteApi, QuoteSpi, Response};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(long, default_value = "debug")]
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
    #[structopt(long, parse(from_os_str), default_value = "./")]
    tpath: PathBuf,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    udp: bool,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    multicast: bool,
}

//https://www.simnow.com.cn/product.action
fn main() -> Result<()> {
    let opt = Opt::from_args();
    let qopt = opt.clone();
    let env = env_logger::Env::default()
        .filter_or("quote", qopt.level.as_str())
        .write_style_or("quote", "always");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}:{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();
    log::info!("quote.api {}", QuoteApi::version()?);
    let (tx, rx) = channel::bounded(256);
    let mtx = tx.clone();

    let mut qapi = QuoteApi::new(qopt.qpath.to_str().unwrap_or("./"), opt.udp, opt.multicast)?
        .with_configuration(Configuration {
            broker_id: qopt.broker_id,
            user_id: qopt.user_id,
            appid: qopt.appid,
            auth_code: qopt.auth_code,
            front_addr: qopt.quote_addr,
            passwd: qopt.passwd,
            ..Default::default()
        });
    qapi.register_front()?;
    qapi.register_fens_user_info()?;
    qapi.register_spi(Myquote(tx));
    qapi.init();
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
            log::info!("trading day {} {:?}",String::from_c_buf(&login.TradingDay), login);
            use std::io::{ BufRead,BufReader};
            let file = std::fs::File::open("symbols.txt").unwrap();
           
            let symbols:Vec<String> =BufReader::new(file).lines().map(|x| x.unwrap()).collect();
            let symbols: Vec<&str> = symbols.iter().map(|s| &s[..]).collect();
            qapi.subscribe_market_data(&symbols).ok();
            qapi.subscribe_for_quote(&symbols).ok();
        }
        Event::Connected => {
            log::info!("connected ok");
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
            let str = match reason {
                0x1001=>"网络读失败",
                0x1002=>"网络写失败",
                0x2001=>"接收心跳超时",
                0x2002=>"发送心跳失败",
                0x2003=>"收到错误报文",
                _=>"未知错误",
            };
            log::error!("0x{:#04x} {}", reason,str);
        }
        _ => {
            log::debug!("{:?}", ev);
        }
    });
    qapi.wait()
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
        log::info!("{:?}", info);
        self.0.send(Event::Quote(info.clone())).ok();
    }
    ///询价通知
    fn on_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::info!("{:?}", info);
    }
}
