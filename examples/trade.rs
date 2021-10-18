use anyhow::Result;

use ctp_rs::{ffi::*, Configuration, FromCBuf, Response, ResumeType, ToArray, TradeApi, TradeSpi};
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
        name = "trade_addr",
        long,
        default_value = "tcp://180.168.146.187:10130"
    )]
    trade_addr: String,
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
        .filter_or("MY_LOG_LEVEL", qopt.level.as_str())
        .write_style_or("MY_LOG_STYLE", "always");
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
    log::info!("trade.api {}", TradeApi::version()?);
    let mut tapi =
        TradeApi::new(opt.tpath.to_str().unwrap_or("./"))?.with_configuration(Configuration {
            broker_id: opt.broker_id,
            user_id: opt.user_id,
            appid: opt.appid,
            auth_code: opt.auth_code,
            front_addr: opt.trade_addr,
            passwd: opt.passwd,
            ..Default::default()
        });
    tapi.register_front()?;
    tapi.register_fens_user_info()?;
    tapi.register_spi(MyTradeSpi)?;
    tapi.init();
    tapi.authenticate()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    loop {
        if let Err(err) = tapi.login() {
            log::error!("{}", err);
            std::thread::sleep(std::time::Duration::from_secs(1));
        } else {
            break;
        }
    }
    tapi.subscribe_public_topic(ResumeType::THOST_TERT_QUICK)?;
    tapi.subscribe_private_topic(ResumeType::THOST_TERT_QUICK)?;
    let mut req = CThostFtdcQryInstrumentField::default();
    req.ExchangeID = "SHFE".into_array::<9>();
    req.ExchangeInstID = "sn".into_array::<81>();
    tapi.query_instrument(&mut req)?;
    tapi.wait()
}

#[derive(Debug, Clone)]
struct MyTradeSpi;

impl TradeSpi for MyTradeSpi {
    ///登录请求响应
    fn on_user_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::info!(
            "TradingDay:{}, LoginTime:{}, BrokerID:{}, UserID:{}, SystemName:{}, FrontID:{}, SessionID:{}, MaxOrderRef:{} {} {}",
            String::from_c_buf(&info.TradingDay),
            String::from_c_buf(&info.LoginTime),
            String::from_c_buf(&info.BrokerID),
            String::from_c_buf(&info.UserID),
            String::from_c_buf(&info.SystemName),
            info.FrontID,
            info.SessionID,
            String::from_c_buf(&info.MaxOrderRef),
            result.code,
            result.message,
        );
    }

    fn on_user_password_update(&self, info: &CThostFtdcUserPasswordUpdateField, result: &Response) {
        log::debug!("info {:?} result {:?}", info, result);
    }
    ///合约交易状态通知
    fn on_rtn_instrument_status(&self, info: &CThostFtdcInstrumentStatusField) {
        log::info!(
            "Exchange:{}, Instrument:{},SettlementGroup:{}, EnterTime:{}, ExchangeInstID:{}",
            String::from_c_buf(&info.ExchangeID),
            String::from_c_buf(&info.InstrumentID),
            String::from_c_buf(&info.SettlementGroupID),
            String::from_c_buf(&info.EnterTime),
            String::from_c_buf(&info.ExchangeInstID)
        );
    }
}
