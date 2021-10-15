use anyhow::Result;

use ctp_rs::{ffi::*, Configuration, Response, ResumeType, TradeApi, TradeSpi};
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
        default_value = "tcp://180.168.146.187:10211"
    )]
    quote_addr: String,
    #[structopt(
        name = "trade_addr",
        long,
        default_value = "tcp://180.168.146.187:10201"
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
    env_logger::init_from_env(env);
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
    std::thread::sleep(std::time::Duration::from_secs(1));
    tapi.login()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    tapi.authenticate()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    tapi.subscribe_public_topic(ResumeType::THOST_TERT_RESTART)?;
    tapi.subscribe_private_topic(ResumeType::THOST_TERT_RESTART)?;
    tapi.wait()
}

#[derive(Debug, Clone)]
struct MyTradeSpi;

impl TradeSpi for MyTradeSpi {
    ///登录请求响应
    fn on_user_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::debug!("{:?} {:?}", info, result);
    }

    fn on_user_password_update(&self, info: &CThostFtdcUserPasswordUpdateField, result: &Response) {
        log::debug!("info {:?} result {:?}", info, result);
    }
}
