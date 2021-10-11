mod quote;
mod trade;
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::{QuoteSpiStub, TradeSpiStub};

impl Drop for QuoteSpiStub {
    fn drop(&mut self) {
        unreachable!("QuoteSpiStub should be manually dropped!")
    }
}

impl Drop for TradeSpiStub {
    fn drop(&mut self) {
        unreachable!("TraderSpiStub should be manually dropped!")
    }
}
