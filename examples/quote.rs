use ctp_rs::{QuoteApi, TradeApi};
fn main() {
    println!("{}", QuoteApi::version().unwrap());
    println!("{}", TradeApi::version().unwrap());
}
