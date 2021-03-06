use anyhow::Result;

use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, QuoteApi, QuoteSpi, Response};
use std::ffi::CString;
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
    #[structopt(name = "user_id", short, long, default_value = "uid")]
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
    #[structopt(short, long, default_value = "passwd")]
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

    let  qapi = QuoteApi::new(qopt.qpath.to_str().unwrap_or("./"), opt.udp, opt.multicast)?
        .with_configuration(Configuration {
            broker_id: qopt.broker_id,
            user_id: qopt.user_id,
            appid: qopt.appid,
            auth_code: qopt.auth_code,
            front_addr: qopt.quote_addr,
            passwd: qopt.passwd,
            ..Default::default()
        }).with_spi(Myquote(tx));
    qapi.register_front()?;
    qapi.register_fens_user_info()?;
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
                String::from_c_buf(&q.TradingDay),
                String::from_c_buf(&q.ActionDay),
                String::from_c_buf(&q.ExchangeID),
                String::from_c_buf(&q.ExchangeInstID),
                String::from_c_buf(&q.InstrumentID),
                String::from_c_buf(&q.UpdateTime),
                q.AveragePrice,q.LastPrice,
                q.Volume,q.Turnover,
                if q.OpenPrice!=f64::MAX {q.OpenPrice}else {f64::NAN},if q.HighestPrice!=f64::MAX {q.HighestPrice}else {f64::NAN}, if q.LowestPrice!=f64::MAX {q.LowestPrice}else {f64::NAN},if q.ClosePrice!=f64::MAX {q.ClosePrice}else {f64::NAN},
                if q.AskPrice1!=f64::MAX {q.AskPrice1}else {f64::NAN},
                q.AskVolume1,
                if q.BidPrice1!=f64::MAX {q.BidPrice1}else {f64::NAN},
                q.BidVolume1,
                if q.AskPrice2!=f64::MAX {q.AskPrice2}else {f64::NAN},
                q.AskVolume2,
                if q.BidPrice2!=f64::MAX {q.BidPrice2}else {f64::NAN},
                q.BidVolume2,
                if q.AskPrice3!=f64::MAX {q.AskPrice3}else {f64::NAN},
                q.AskVolume3,
                if q.BidPrice3!=f64::MAX {q.BidPrice3}else {f64::NAN},
                q.BidVolume3,
                if q.AskPrice4!=f64::MAX {q.AskPrice4}else {f64::NAN},
                q.AskVolume4,
                if q.BidPrice4!=f64::MAX {q.BidPrice4}else {f64::NAN},
                q.BidVolume4,
                if q.AskPrice5!=f64::MAX {q.AskPrice5}else {f64::NAN},
                q.AskVolume5,
                if q.BidPrice5!=f64::MAX {q.BidPrice5}else {f64::NAN},
                q.BidVolume5,
            );
        }
        Event::Login(login) => {
            log::info!("trading day {} {:?}",String::from_c_buf(&login.TradingDay), login);
            use std::io::{ BufRead,BufReader};
            let file = std::fs::File::open("symbols.txt").unwrap();
            let symbols:Vec<CString> =BufReader::new(file).lines().map(|x| CString::new(x.unwrap()).unwrap()).collect();
            qapi.subscribe_market_data(&symbols).ok();
            log::debug!("sub {:?}",symbols);
           // qapi.subscribe_for_quote(&symbols).ok();
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
                0x1001=>"???????????????",
                0x1002=>"???????????????",
                0x2001=>"??????????????????",
                0x2002=>"??????????????????",
                0x2003=>"??????????????????",
                _=>"????????????",
            };
            log::error!("0x{:#04x} {}", reason,str);
        }
        _ => {
            log::debug!("{:?}", ev);
        }
    });
    Ok(())
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
    ///????????????????????????????????????????????????????????????????????????????????????????????????
    fn on_connected(&self) {
        self.0.send(Event::Connected).ok();
    }
    ///???????????????????????????????????????????????????????????????????????????????????????????????????API???????????????????????????????????????????????????
    ///@param nReason ????????????
    ///        0x1001 ???????????????
    ///        0x1002 ???????????????
    ///        0x2001 ??????????????????
    ///        0x2002 ??????????????????
    ///        0x2003 ??????????????????
    fn on_disconnected(&self, reason: i32) {
        self.0.send(Event::Disconnected(reason)).ok();
    }
    ///???????????????????????????????????????????????????????????????????????????
    ///@param nTimeLapse ?????????????????????????????????
    fn on_heartbeat_warning(&self, timelapse: i32) {
        self.0.send(Event::HeartbeatWarn(timelapse)).ok();
    }
    ///????????????
    fn on_error(&self, result: &Response) {
        self.0
            .send(Event::Error(result.code, result.message.clone()))
            .ok();
    }
    ///??????????????????
    fn on_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        if result.code != 0 {
            self.0
                .send(Event::Error(result.code, result.message.clone()))
                .ok();
        } else {
            self.0.send(Event::Login(info.clone())).ok();
        }
    }
    ///??????????????????
    fn on_logout(&self, info: &CThostFtdcUserLogoutField, result: &Response) {
        if result.code != 0 {
            self.0
                .send(Event::Error(result.code, result.message.clone()))
                .ok();
        } else {
            self.0.send(Event::Logout(info.clone())).ok();
        }
    }
    ///??????????????????????????????
    fn on_query_multicast_instrument(
        &self,
        info: &CThostFtdcMulticastInstrumentField,
        result: &Response,
    ) {
        log::debug!("{:?} {:?}", info, result);
    }
    ///??????????????????
    fn on_sub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::debug!(
            "InstrumentID:{} {}",
            String::from_c_buf(&info.InstrumentID),
            result.code
        );
    }
    ///????????????????????????
    fn on_unsub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        if result.code != 0 {
            log::warn!("{:?} {:?}", info, result);
        }
    }
    ///??????????????????
    fn on_sub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        if result.code != 0 {
            log::warn!("{:?} {:?}", info, result);
        }
    }

    ///????????????????????????
    fn on_unsub_for_quote(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        if result.code != 0 {
            log::warn!("{:?} {:?}", info, result);
        }
    }
    ///??????????????????
    fn on_depth_market_data(&self, info: &CThostFtdcDepthMarketDataField) {
        // log::info!("{:?}", info);
        self.0.send(Event::Quote(info.clone())).ok();
    }
    ///????????????
    fn on_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::info!("{:?}", info);
    }
}
