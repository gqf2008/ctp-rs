use anyhow::Result;
use ctp_rs::{Configuration, QuoteApi, QuoteSpi, TradeApi};
fn main() -> Result<()> {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    log::info!("quote.api {}", QuoteApi::version()?);
    log::info!("trade.api {}", TradeApi::version()?);
    let mut qapi = QuoteApi::new("", false, false)?.with_configuration(Configuration {
        broker_id: "9999".to_string(),
        user_id: "user_id".to_string(),
        appid: "simnow_client_test".to_string(),
        auth_code: "0000000000000000".to_string(),
        front_addr: "tcp://180.168.146.187:10131".to_string(),
        passwd: "passwd".to_string(),
        ..Default::default()
    });
    qapi.register_front()?;
    qapi.register_fens_user_info()?;
    qapi.register_spi(Myquote);
    qapi.init();
    loop {
        match qapi.login() {
            Ok(()) => break,
            Err(err) => {
                log::error!("login error {}, after 1 second retry...", err);
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    qapi.subscribe_market_data(&["ag2110", "ag2111"])?;
    qapi.subscribe_for_quote(&["ag2110", "ag2111"])?;

    qapi.join()
}

struct Myquote;

impl QuoteSpi for Myquote {}
