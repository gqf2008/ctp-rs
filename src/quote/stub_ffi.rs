#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use super::QuoteSpi;
use super::*;
use crate::ffi::*;
use libc::c_void;

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn QuoteSpi>;
    let _: Box<Box<dyn QuoteSpi>> = Box::from_raw(spi);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnFrontConnected(spi: *mut c_void) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_connected();
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnFrontDisconnected(spi: *mut c_void, reason: i32) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_disconnected(reason);
}
#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspError(
    spi: *mut c_void,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_error(&resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnHeartBeatWarning(spi: *mut c_void, timelapse: i32) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_heart_beat_warning(timelapse);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspUserLogin(
    spi: *mut c_void,
    info: *const CThostFtdcRspUserLoginField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_login(&*info, &resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspUserLogout(
    spi: *mut c_void,
    info: *const CThostFtdcUserLogoutField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_logout(&*info, &resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspQryMulticastInstrument(
    spi: *mut c_void,
    info: *const CThostFtdcMulticastInstrumentField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);

    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_query_multicast_instrument(&*info, &resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspSubMarketData(
    spi: *mut c_void,
    info: *const CThostFtdcSpecificInstrumentField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);

    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_sub_market_data(&*info, &resp);
}
#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspUnSubMarketData(
    spi: *mut c_void,
    info: *const CThostFtdcSpecificInstrumentField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);

    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_unsub_market_data(&*info, &resp);
}
#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspSubForQuoteRsp(
    spi: *mut c_void,
    info: *const CThostFtdcSpecificInstrumentField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);

    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_sub_for_quote(&*info, &resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRspUnSubForQuoteRsp(
    spi: *mut c_void,
    info: *const CThostFtdcSpecificInstrumentField,
    result: *const CThostFtdcRspInfoField,
    req_id: i32,
    is_last: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);

    let resp = Response::from(&*result)
        .with_req_id(req_id)
        .with_is_last(is_last);
    spi.on_unsub_for_quote(&*info, &resp);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRtnDepthMarketData(
    spi: *mut c_void,
    data: *const CThostFtdcDepthMarketDataField,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_depth_market_data(&*data);
}

#[no_mangle]
pub unsafe extern "C" fn QuoteSpiStub_Rust_OnRtnForQuoteRsp(
    spi: *mut c_void,
    info: *const CThostFtdcForQuoteRspField,
) {
    let spi = &mut **(spi as *mut *mut dyn QuoteSpi);
    spi.on_for_quote(&*info);
}
