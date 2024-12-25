#![allow(non_snake_case)]

pub mod md {
    use autocxx::subclass::prelude::*;

    autocxx::include_cpp! {
        #include "ThostFtdcMdApi.h"
        name!(_ffi)
        safety!(unsafe_ffi)
        generate!("CThostFtdcMdSpi")
        generate!("CThostFtdcMdApi")
        subclass!("CThostFtdcMdSpi", CThostFtdcMdStub)
    }
    use _ffi as ffi;
    pub use _ffi::*;

    #[subclass]
    #[derive(Default)]
    pub struct CThostFtdcMdStub;

    impl CThostFtdcMdSpi_methods for CThostFtdcMdStub {
        fn OnFrontConnected(&mut self) {}
    }
}

pub mod trader {
    use autocxx::subclass::prelude::*;

    autocxx::include_cpp! {
        #include "ThostFtdcTraderApi.h"
        name!(__ffi)
        safety!(unsafe_ffi)
        generate!("CThostFtdcTraderSpi")
        generate!("CThostFtdcTraderApi")
        subclass!("CThostFtdcTraderSpi", CThostFtdcTraderStub)
    }

    use __ffi as ffi;
    pub use __ffi::*;

    #[subclass]
    #[derive(Default)]
    pub struct CThostFtdcTraderStub;

    impl CThostFtdcTraderSpi_methods for CThostFtdcTraderStub {
        // fn goat_full(&self) {
        //     println!("BURP!");
        // }
    }
}

// #[cxx::bridge]
// mod ffi2 {
//     unsafe extern "C++" {
//         include!("foo.h");
//         type A = crate::ffi::A;
//         fn give_A() -> UniquePtr<A>; // in practice, autocxx could happily do this
//     }
// }

// fn main() {
//     use md::*;
//     use trader::*;
// }
