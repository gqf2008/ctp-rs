use anyhow::Result;

use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, Response, TradeApi, TradeSpi};
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
        name = "trade_addr",
        long,
        default_value = "tcp://180.168.146.187:10201"
    )]
    trade_addr: String,
    #[structopt(name = "passwd", short, long)]
    passwd: String,
    #[structopt(name = "new_passwd", short, long)]
    new_passwd: String,
    /// Output file
    #[structopt(long, parse(from_os_str), default_value = "./")]
    qpath: PathBuf,
}

//https://www.simnow.com.cn/product.action
fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("trade.api {}", TradeApi::version()?);
    let mut tapi =
        TradeApi::new(opt.qpath.to_str().unwrap_or("./"))?.with_configuration(Configuration {
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
    tapi.update_passwd(opt.new_passwd.as_str())?;
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