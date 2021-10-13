use ctp_rs::quote::QuoteApi;
fn main() {
    println!("{}", QuoteApi::version().unwrap());
}
