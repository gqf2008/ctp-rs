use super::QuoteSpi;
use crate::ffi::*;
use libc::c_void;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRtnForQuoteRsp(
    spi: *mut c_void,
    rsp: *const CThostFtdcForQuoteRspField,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_for_quote(&*rsp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn QuoteSpi>;
    let _: Box<Box<dyn QuoteSpi>> = Box::from_raw(spi);
}
