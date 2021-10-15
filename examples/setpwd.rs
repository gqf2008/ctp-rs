use anyhow::Result;

use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, Response, TradeApi, TradeSpi};
use std::path::PathBuf;
use structopt::StructOpt;

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
        name = "trade_addr",
        long,
        default_value = "tcp://180.168.146.187:10201"
    )]
    trade_addr: String,
    #[structopt(name = "login_passwd", short, long)]
    login_passwd: String,
    #[structopt(name = "trade_passwd", short, long)]
    trade_passwd: String,
    #[structopt(name = "new_login_passwd", short, long)]
    new_login_passwd: String,
    #[structopt(name = "new_trade_passwd", short, long)]
    new_trade_passwd: String,
    /// Output file
    #[structopt(long, parse(from_os_str), default_value = "./")]
    tpath: PathBuf,
}

//https://www.simnow.com.cn/product.action
fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("trade.api {}", TradeApi::version()?);
    let mut tapi =
        TradeApi::new(opt.tpath.to_str().unwrap_or("./"))?.with_configuration(Configuration {
            broker_id: opt.broker_id,
            user_id: opt.user_id,
            appid: opt.appid,
            auth_code: opt.auth_code,
            front_addr: opt.trade_addr,
            passwd: opt.login_passwd.clone(),
            ..Default::default()
        });
    tapi.register_front()?;
    tapi.register_fens_user_info()?;
    tapi.register_spi(MyTradeSpi)?;
    tapi.init();
    std::thread::sleep(std::time::Duration::from_secs(1));
    if let Err(err) = tapi.login() {
        log::error!("{}", err);
        tapi.update_passwd(opt.new_login_passwd.as_str())?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        tapi.update_trade_account_passwd(
            "ctp_dev_1",
            "cny",
            opt.trade_passwd.as_str(),
            opt.new_trade_passwd.as_str(),
        )?;
    }
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}

#[derive(Debug, Clone)]
struct MyTradeSpi;

impl TradeSpi for MyTradeSpi {
    ///登录请求响应
    fn on_user_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        println!("{:?} {:?}", info, result);
    }

    fn on_user_password_update(&self, info: &CThostFtdcUserPasswordUpdateField, result: &Response) {
        println!("info {:?} result {:?}", info, result);
    }
}
