#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::TradeSpi;
use crate::ffi::*;
use crate::Response;
use libc::c_void;

///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnFrontConnected(spi: *mut c_void) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_connected()
}

///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
///@param nReason 错误原因
///        0x1001 网络读失败
///        0x1002 网络写失败
///        0x2001 接收心跳超时
///        0x2002 发送心跳失败
///        0x2003 收到错误报文
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnFrontDisconnected(spi: *mut c_void, nReason: i32) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_disconnected(nReason)
}

///心跳超时警告。当长时间未收到报文时，该方法被调用。
///@param nTimeLapse 距离上次接收报文的时间
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnHeartBeatWarning(spi: *mut c_void, nTimeLapse: i32) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_heart_beat_warning(nTimeLapse)
}

///客户端认证响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspAuthenticate(
    spi: *mut c_void,
    pRspAuthenticateField: *const CThostFtdcRspAuthenticateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_authenticate(&*pRspAuthenticateField, &result)
}

///登录请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspUserLogin(
    spi: *mut c_void,
    pRspUserLogin: *const CThostFtdcRspUserLoginField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_user_login(&*pRspUserLogin, &result)
}

///登出请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspUserLogout(
    spi: *mut c_void,
    pUserLogout: *const CThostFtdcUserLogoutField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_user_logout(&*pUserLogout, &result)
}

///用户口令更新请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspUserPasswordUpdate(
    spi: *mut c_void,
    pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    if let Some(info) = pUserPasswordUpdate.as_ref() {
        spi.on_user_password_update(info, &result)
    }
}

///资金账户口令更新请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspTradingAccountPasswordUpdate(
    spi: *mut c_void,
    pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    if let Some(info) = pTradingAccountPasswordUpdate.as_ref() {
        spi.on_trading_account_password_update(info, &result)
    }
}

///查询用户当前支持的认证模式的回复
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspUserAuthMethod(
    spi: *mut c_void,
    pRspUserAuthMethod: *const CThostFtdcRspUserAuthMethodField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_user_auth_method(&*pRspUserAuthMethod, &result)
}

///获取图形验证码请求的回复
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspGenUserCaptcha(
    spi: *mut c_void,
    pRspGenUserCaptcha: *const CThostFtdcRspGenUserCaptchaField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_gen_user_captcha(&*pRspGenUserCaptcha, &result)
}

///获取短信验证码请求的回复
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspGenUserText(
    spi: *mut c_void,
    pRspGenUserText: *const CThostFtdcRspGenUserTextField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_gen_user_text(&*pRspGenUserText, &result)
}

///报单录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspOrderInsert(
    spi: *mut c_void,
    pInputOrder: *const CThostFtdcInputOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_order_insert(&*pInputOrder, &result)
}

///预埋单录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspParkedOrderInsert(
    spi: *mut c_void,
    pParkedOrder: *const CThostFtdcParkedOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_parked_order_insert(&*pParkedOrder, &result)
}

///预埋撤单录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspParkedOrderAction(
    spi: *mut c_void,
    pParkedOrderAction: *const CThostFtdcParkedOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_parked_order_action(&*pParkedOrderAction, &result)
}

///报单操作请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspOrderAction(
    spi: *mut c_void,
    pInputOrderAction: *const CThostFtdcInputOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_order_action(&*pInputOrderAction, &result)
}

///查询最大报单数量响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryMaxOrderVolume(
    spi: *mut c_void,
    pQryMaxOrderVolume: *const CThostFtdcQryMaxOrderVolumeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_max_order_volume(&*pQryMaxOrderVolume, &result)
}

///投资者结算结果确认响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspSettlementInfoConfirm(
    spi: *mut c_void,
    pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_settlement_info_confirm(&*pSettlementInfoConfirm, &result)
}

///删除预埋单响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspRemoveParkedOrder(
    spi: *mut c_void,
    pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_remove_parked_order(&*pRemoveParkedOrder, &result)
}

///删除预埋撤单响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspRemoveParkedOrderAction(
    spi: *mut c_void,
    pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_remove_parked_order_action(&*pRemoveParkedOrderAction, &result)
}

///执行宣告录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspExecOrderInsert(
    spi: *mut c_void,
    pInputExecOrder: *const CThostFtdcInputExecOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_exec_order_insert(&*pInputExecOrder, &result)
}

///执行宣告操作请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspExecOrderAction(
    spi: *mut c_void,
    pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_exec_order_action(&*pInputExecOrderAction, &result)
}

///询价录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspForQuoteInsert(
    spi: *mut c_void,
    pInputForQuote: *const CThostFtdcInputForQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_for_quote_insert(&*pInputForQuote, &result)
}

///报价录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQuoteInsert(
    spi: *mut c_void,
    pInputQuote: *const CThostFtdcInputQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_quote_insert(&*pInputQuote, &result)
}

///报价操作请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQuoteAction(
    spi: *mut c_void,
    pInputQuoteAction: *const CThostFtdcInputQuoteActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_quote_action(&*pInputQuoteAction, &result)
}

///批量报单操作请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspBatchOrderAction(
    spi: *mut c_void,
    pInputBatchOrderAction: *const CThostFtdcInputBatchOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_batch_order_action(&*pInputBatchOrderAction, &result)
}

///期权自对冲录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspOptionSelfCloseInsert(
    spi: *mut c_void,
    pInputOptionSelfClose: *const CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_option_self_close_insert(&*pInputOptionSelfClose, &result)
}

///期权自对冲操作请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspOptionSelfCloseAction(
    spi: *mut c_void,
    pInputOptionSelfCloseAction: *const CThostFtdcInputOptionSelfCloseActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_option_self_close_action(&*pInputOptionSelfCloseAction, &result)
}

///申请组合录入请求响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspCombActionInsert(
    spi: *mut c_void,
    pInputCombAction: *const CThostFtdcInputCombActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_comb_action_insert(&*pInputCombAction, &result)
}

///请求查询报单响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryOrder(
    spi: *mut c_void,
    pOrder: *const CThostFtdcOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_order(&*pOrder, &result)
}

///请求查询成交响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTrade(
    spi: *mut c_void,
    pTrade: *const CThostFtdcTradeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_trade(&*pTrade, &result)
}

///请求查询投资者持仓响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestorPosition(
    spi: *mut c_void,
    pInvestorPosition: *const CThostFtdcInvestorPositionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_investor_position(&*pInvestorPosition, &result)
}

///请求查询资金账户响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTradingAccount(
    spi: *mut c_void,
    pTradingAccount: *const CThostFtdcTradingAccountField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_trading_account(&*pTradingAccount, &result)
}

///请求查询投资者响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestor(
    spi: *mut c_void,
    pInvestor: *const CThostFtdcInvestorField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_investor(&*pInvestor, &result)
}

///请求查询交易编码响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTradingCode(
    spi: *mut c_void,
    pTradingCode: *const CThostFtdcTradingCodeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_trading_code(&*pTradingCode, &result)
}

///请求查询合约保证金率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInstrumentMarginRate(
    spi: *mut c_void,
    pInstrumentMarginRate: *const CThostFtdcInstrumentMarginRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_instrument_margin_rate(&*pInstrumentMarginRate, &result)
}

///请求查询合约手续费率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInstrumentCommissionRate(
    spi: *mut c_void,
    pInstrumentCommissionRate: *const CThostFtdcInstrumentCommissionRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_instrument_commission_rate(&*pInstrumentCommissionRate, &result)
}

///请求查询交易所响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryExchange(
    spi: *mut c_void,
    pExchange: *const CThostFtdcExchangeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_exchange(&*pExchange, &result)
}

///请求查询产品响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryProduct(
    spi: *mut c_void,
    pProduct: *const CThostFtdcProductField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_product(&*pProduct, &result)
}

///请求查询合约响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInstrument(
    spi: *mut c_void,
    pInstrument: *const CThostFtdcInstrumentField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    log::info!("TradeSpiStub_Rust_OnRspQryInstrument");
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    if pInstrument.is_null() {
        spi.on_qry_instrument(Some(&*pInstrument), &result)
    } else {
        spi.on_qry_instrument(None, &result)
    }
}

///请求查询行情响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryDepthMarketData(
    spi: *mut c_void,
    pDepthMarketData: *const CThostFtdcDepthMarketDataField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_depth_market_data(&*pDepthMarketData, &result)
}

///请求查询投资者结算结果响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySettlementInfo(
    spi: *mut c_void,
    pSettlementInfo: *const CThostFtdcSettlementInfoField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_settlement_info(&*pSettlementInfo, &result)
}

///请求查询转帐银行响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTransferBank(
    spi: *mut c_void,
    pTransferBank: *const CThostFtdcTransferBankField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_transfer_bank(&*pTransferBank, &result)
}

///请求查询投资者持仓明细响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestorPositionDetail(
    spi: *mut c_void,
    pInvestorPositionDetail: *const CThostFtdcInvestorPositionDetailField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_investor_position_detail(&*pInvestorPositionDetail, &result)
}

///请求查询客户通知响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryNotice(
    spi: *mut c_void,
    pNotice: *const CThostFtdcNoticeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_notice(&*pNotice, &result)
}

///请求查询结算信息确认响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySettlementInfoConfirm(
    spi: *mut c_void,
    pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_settlement_info_confirm(&*pSettlementInfoConfirm, &result)
}

///请求查询投资者持仓明细响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestorPositionCombineDetail(
    spi: *mut c_void,
    pInvestorPositionCombineDetail: *const CThostFtdcInvestorPositionCombineDetailField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_investor_position_combine_detail(&*pInvestorPositionCombineDetail, &result)
}

///查询保证金监管系统经纪公司资金账户密钥响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryCFMMCTradingAccountKey(
    spi: *mut c_void,
    pCFMMCTradingAccountKey: *const CThostFtdcCFMMCTradingAccountKeyField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_cfmmc_trading_account_key(&*pCFMMCTradingAccountKey, &result)
}

///请求查询仓单折抵信息响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryEWarrantOffset(
    spi: *mut c_void,
    pEWarrantOffset: *const CThostFtdcEWarrantOffsetField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_ewarrant_offset(&*pEWarrantOffset, &result)
}

///请求查询投资者品种/跨品种保证金响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestorProductGroupMargin(
    spi: *mut c_void,
    pInvestorProductGroupMargin: *const CThostFtdcInvestorProductGroupMarginField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_investor_product_group_margin(&*pInvestorProductGroupMargin, &result)
}

///请求查询交易所保证金率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryExchangeMarginRate(
    spi: *mut c_void,
    pExchangeMarginRate: *const CThostFtdcExchangeMarginRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_exchange_margin_rate(&*pExchangeMarginRate, &result)
}

///请求查询交易所调整保证金率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryExchangeMarginRateAdjust(
    spi: *mut c_void,
    pExchangeMarginRateAdjust: *const CThostFtdcExchangeMarginRateAdjustField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_exchange_margin_rate_adjust(&*pExchangeMarginRateAdjust, &result)
}

///请求查询汇率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryExchangeRate(
    spi: *mut c_void,
    pExchangeRate: *const CThostFtdcExchangeRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_exchange_rate(&*pExchangeRate, &result)
}

///请求查询二级代理操作员银期权限响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySecAgentACIDMap(
    spi: *mut c_void,
    pSecAgentACIDMap: *const CThostFtdcSecAgentACIDMapField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_sec_agent_acid_map(&*pSecAgentACIDMap, &result)
}

///请求查询产品报价汇率
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryProductExchRate(
    spi: *mut c_void,
    pProductExchRate: *const CThostFtdcProductExchRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_product_exch_rate(&*pProductExchRate, &result)
}

///请求查询产品组
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryProductGroup(
    spi: *mut c_void,
    pProductGroup: *const CThostFtdcProductGroupField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_product_group(&*pProductGroup, &result)
}

///请求查询做市商合约手续费率响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryMMInstrumentCommissionRate(
    spi: *mut c_void,
    pMMInstrumentCommissionRate: *const CThostFtdcMMInstrumentCommissionRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_mm_instrument_commission_rate(&*pMMInstrumentCommissionRate, &result)
}

///请求查询做市商期权合约手续费响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryMMOptionInstrCommRate(
    spi: *mut c_void,
    pMMOptionInstrCommRate: *const CThostFtdcMMOptionInstrCommRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_mm_option_instr_comm_rate(&*pMMOptionInstrCommRate, &result)
}

///请求查询报单手续费响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInstrumentOrderCommRate(
    spi: *mut c_void,
    pInstrumentOrderCommRate: *const CThostFtdcInstrumentOrderCommRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_instrument_order_comm_rate(&*pInstrumentOrderCommRate, &result)
}

///请求查询资金账户响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySecAgentTradingAccount(
    spi: *mut c_void,
    pTradingAccount: *const CThostFtdcTradingAccountField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_sec_agent_trading_account(&*pTradingAccount, &result)
}

///请求查询二级代理商资金校验模式响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySecAgentCheckMode(
    spi: *mut c_void,
    pSecAgentCheckMode: *const CThostFtdcSecAgentCheckModeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_sec_agent_check_mode(&*pSecAgentCheckMode, &result)
}

///请求查询二级代理商信息响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQrySecAgentTradeInfo(
    spi: *mut c_void,
    pSecAgentTradeInfo: *const CThostFtdcSecAgentTradeInfoField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_sec_agent_trade_info(&*pSecAgentTradeInfo, &result)
}

///请求查询期权交易成本响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryOptionInstrTradeCost(
    spi: *mut c_void,
    pOptionInstrTradeCost: *const CThostFtdcOptionInstrTradeCostField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_option_instr_trade_cost(&*pOptionInstrTradeCost, &result)
}

///请求查询期权合约手续费响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryOptionInstrCommRate(
    spi: *mut c_void,
    pOptionInstrCommRate: *const CThostFtdcOptionInstrCommRateField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_option_instr_comm_rate(&*pOptionInstrCommRate, &result)
}

///请求查询执行宣告响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryExecOrder(
    spi: *mut c_void,
    pExecOrder: *const CThostFtdcExecOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_exec_order(&*pExecOrder, &result)
}

///请求查询询价响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryForQuote(
    spi: *mut c_void,
    pForQuote: *const CThostFtdcForQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_for_quote(&*pForQuote, &result)
}

///请求查询报价响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryQuote(
    spi: *mut c_void,
    pQuote: *const CThostFtdcQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_quote(&*pQuote, &result)
}

///请求查询期权自对冲响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryOptionSelfClose(
    spi: *mut c_void,
    pOptionSelfClose: *const CThostFtdcOptionSelfCloseField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_option_self_slose(&*pOptionSelfClose, &result)
}

///请求查询投资单元响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryInvestUnit(
    spi: *mut c_void,
    pInvestUnit: *const CThostFtdcInvestUnitField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_invest_unit(&*pInvestUnit, &result)
}

///请求查询组合合约安全系数响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryCombInstrumentGuard(
    spi: *mut c_void,
    pCombInstrumentGuard: *const CThostFtdcCombInstrumentGuardField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_comb_instrument_guard(&*pCombInstrumentGuard, &result)
}

///请求查询申请组合响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryCombAction(
    spi: *mut c_void,
    pCombAction: *const CThostFtdcCombActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_comb_action(&*pCombAction, &result)
}

///请求查询转帐流水响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTransferSerial(
    spi: *mut c_void,
    pTransferSerial: *const CThostFtdcTransferSerialField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_transfer_serial(&*pTransferSerial, &result)
}

///请求查询银期签约关系响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryAccountregister(
    spi: *mut c_void,
    pAccountregister: *const CThostFtdcAccountregisterField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_account_register(&*pAccountregister, &result)
}

///错误应答
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspError(
    spi: *mut c_void,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_error(&result)
}

///报单通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnOrder(
    spi: *mut c_void,
    pOrder: *const CThostFtdcOrderField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);

    spi.on_rtn_order(&*pOrder)
}

///成交通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnTrade(
    spi: *mut c_void,
    pTrade: *const CThostFtdcTradeField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_trade(&*pTrade)
}

///报单录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnOrderInsert(
    spi: *mut c_void,
    pInputOrder: *const CThostFtdcInputOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);

    spi.on_err_rtn_order_insert(&*pInputOrder, &result)
}

///报单操作错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnOrderAction(
    spi: *mut c_void,
    pOrderAction: *const CThostFtdcOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);

    spi.on_err_rtn_order_action(&*pOrderAction, &result)
}

///合约交易状态通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnInstrumentStatus(
    spi: *mut c_void,
    pInstrumentStatus: *const CThostFtdcInstrumentStatusField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_instrument_status(&*pInstrumentStatus)
}

///交易所公告通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnBulletin(
    spi: *mut c_void,
    pBulletin: *const CThostFtdcBulletinField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_bulletin(&*pBulletin)
}

///交易通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnTradingNotice(
    spi: *mut c_void,
    pTradingNoticeInfo: *const CThostFtdcTradingNoticeInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_trading_notice(&*pTradingNoticeInfo)
}

///提示条件单校验错误
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnErrorConditionalOrder(
    spi: *mut c_void,
    pErrorConditionalOrder: *const CThostFtdcErrorConditionalOrderField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_error_conditional_order(&*pErrorConditionalOrder)
}

///执行宣告通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnExecOrder(
    spi: *mut c_void,
    pExecOrder: *const CThostFtdcExecOrderField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_exec_order(&*pExecOrder)
}

///执行宣告录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnExecOrderInsert(
    spi: *mut c_void,
    pInputExecOrder: *const CThostFtdcInputExecOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_exec_order_insert(&*pInputExecOrder, &result)
}

///执行宣告操作错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnExecOrderAction(
    spi: *mut c_void,
    pExecOrderAction: *const CThostFtdcExecOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_exec_order_action(&*pExecOrderAction, &result)
}

///询价录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnForQuoteInsert(
    spi: *mut c_void,
    pInputForQuote: *const CThostFtdcInputForQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_for_quote_insert(&*pInputForQuote, &result)
}

///报价通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnQuote(
    spi: *mut c_void,
    pQuote: *const CThostFtdcQuoteField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_quote(&*pQuote)
}

///报价录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnQuoteInsert(
    spi: *mut c_void,
    pInputQuote: *const CThostFtdcInputQuoteField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_quote_insert(&*pInputQuote, &result)
}

///报价操作错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnQuoteAction(
    spi: *mut c_void,
    pQuoteAction: *const CThostFtdcQuoteActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_quote_action(&*pQuoteAction, &result)
}

///询价通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnForQuoteRsp(
    spi: *mut c_void,
    pForQuoteRsp: *const CThostFtdcForQuoteRspField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_for_quote(&*pForQuoteRsp)
}

///保证金监控中心用户令牌
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnCFMMCTradingAccountToken(
    spi: *mut c_void,
    pCFMMCTradingAccountToken: *const CThostFtdcCFMMCTradingAccountTokenField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_cfmmc_trading_account_token(&*pCFMMCTradingAccountToken)
}

///批量报单操作错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnBatchOrderAction(
    spi: *mut c_void,
    pBatchOrderAction: *const CThostFtdcBatchOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_batch_order_action(&*pBatchOrderAction, &result)
}

///期权自对冲通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnOptionSelfClose(
    spi: *mut c_void,
    pOptionSelfClose: *const CThostFtdcOptionSelfCloseField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_option_self_close(&*pOptionSelfClose)
}

///期权自对冲录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnOptionSelfCloseInsert(
    spi: *mut c_void,
    pInputOptionSelfClose: *const CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_option_self_close_insert(&*pInputOptionSelfClose, &result)
}

///期权自对冲操作错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnOptionSelfCloseAction(
    spi: *mut c_void,
    pOptionSelfCloseAction: *const CThostFtdcOptionSelfCloseActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_option_self_close_action(&*pOptionSelfCloseAction, &result)
}

///申请组合通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnCombAction(
    spi: *mut c_void,
    pCombAction: *const CThostFtdcCombActionField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_comb_action(&*pCombAction)
}

///申请组合录入错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnCombActionInsert(
    spi: *mut c_void,
    pInputCombAction: *const CThostFtdcInputCombActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_comb_action_insert(&*pInputCombAction, &result)
}

///请求查询签约银行响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryContractBank(
    spi: *mut c_void,
    pContractBank: *const CThostFtdcContractBankField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_contract_bank(&*pContractBank, &result)
}

///请求查询预埋单响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryParkedOrder(
    spi: *mut c_void,
    pParkedOrder: *const CThostFtdcParkedOrderField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_parked_order(&*pParkedOrder, &result)
}

///请求查询预埋撤单响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryParkedOrderAction(
    spi: *mut c_void,
    pParkedOrderAction: *const CThostFtdcParkedOrderActionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_parked_order_action(&*pParkedOrderAction, &result)
}

///请求查询交易通知响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryTradingNotice(
    spi: *mut c_void,
    pTradingNotice: *const CThostFtdcTradingNoticeField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_trading_notice(&*pTradingNotice, &result)
}

///请求查询经纪公司交易参数响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryBrokerTradingParams(
    spi: *mut c_void,
    pBrokerTradingParams: *const CThostFtdcBrokerTradingParamsField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_broker_trading_params(&*pBrokerTradingParams, &result)
}

///请求查询经纪公司交易算法响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryBrokerTradingAlgos(
    spi: *mut c_void,
    pBrokerTradingAlgos: *const CThostFtdcBrokerTradingAlgosField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_broker_trading_algos(&*pBrokerTradingAlgos, &result)
}

///请求查询监控中心用户令牌
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQueryCFMMCTradingAccountToken(
    spi: *mut c_void,
    pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_query_cfmmc_trading_account_token(&*pQueryCFMMCTradingAccountToken, &result)
}

///银行发起银行资金转期货通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnFromBankToFutureByBank(
    spi: *mut c_void,
    pRspTransfer: *const CThostFtdcRspTransferField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_from_bank_to_future_by_bank(&*pRspTransfer)
}

///银行发起期货资金转银行通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnFromFutureToBankByBank(
    spi: *mut c_void,
    pRspTransfer: *const CThostFtdcRspTransferField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_from_future_to_bank_by_bank(&*pRspTransfer)
}

///银行发起冲正银行转期货通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByBank(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_bank_to_future_by_bank(&*pRspRepeal)
}

///银行发起冲正期货转银行通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByBank(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_future_to_bank_by_bank(&*pRspRepeal)
}

///期货发起银行资金转期货通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnFromBankToFutureByFuture(
    spi: *mut c_void,
    pRspTransfer: *const CThostFtdcRspTransferField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_from_bank_to_future_by_future(&*pRspTransfer)
}

///期货发起期货资金转银行通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnFromFutureToBankByFuture(
    spi: *mut c_void,
    pRspTransfer: *const CThostFtdcRspTransferField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_from_future_to_bank_by_future(&*pRspTransfer)
}

///系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFutureManual(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_bank_to_future_by_future_manual(&*pRspRepeal)
}

///系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFutureManual(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_future_to_bank_by_future_manual(&*pRspRepeal)
}

///期货发起查询银行余额通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnQueryBankBalanceByFuture(
    spi: *mut c_void,
    pNotifyQueryAccount: *const CThostFtdcNotifyQueryAccountField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_query_bank_balance_by_future(&*pNotifyQueryAccount)
}

///期货发起银行资金转期货错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnBankToFutureByFuture(
    spi: *mut c_void,
    pReqTransfer: *const CThostFtdcReqTransferField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_bank_to_future_by_future(&*pReqTransfer, &result)
}

///期货发起期货资金转银行错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnFutureToBankByFuture(
    spi: *mut c_void,
    pReqTransfer: *const CThostFtdcReqTransferField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_future_to_bank_by_future(&*pReqTransfer, &result)
}

///系统运行时期货端手工发起冲正银行转期货错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnRepealBankToFutureByFutureManual(
    spi: *mut c_void,
    pReqRepeal: *const CThostFtdcReqRepealField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_repeal_bank_to_future_by_future_manual(&*pReqRepeal, &result)
}

///系统运行时期货端手工发起冲正期货转银行错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnRepealFutureToBankByFutureManual(
    spi: *mut c_void,
    pReqRepeal: *const CThostFtdcReqRepealField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_repeal_future_to_bank_by_future_manual(&*pReqRepeal, &result)
}

///期货发起查询银行余额错误回报
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnErrRtnQueryBankBalanceByFuture(
    spi: *mut c_void,
    pReqQueryAccount: *const CThostFtdcReqQueryAccountField,
    pRspInfo: *const CThostFtdcRspInfoField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo);
    spi.on_err_rtn_query_bank_balance_by_future(&*pReqQueryAccount, &result)
}

///期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFuture(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_bank_to_future_by_future(&*pRspRepeal)
}

///期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFuture(
    spi: *mut c_void,
    pRspRepeal: *const CThostFtdcRspRepealField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_repeal_from_future_to_bank_by_future(&*pRspRepeal)
}

///期货发起银行资金转期货应答
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspFromBankToFutureByFuture(
    spi: *mut c_void,
    pReqTransfer: *const CThostFtdcReqTransferField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_from_bank_to_future_by_future(&*pReqTransfer, &result)
}

///期货发起期货资金转银行应答
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspFromFutureToBankByFuture(
    spi: *mut c_void,
    pReqTransfer: *const CThostFtdcReqTransferField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_from_future_to_bank_by_future(&*pReqTransfer, &result)
}

///期货发起查询银行余额应答
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQueryBankAccountMoneyByFuture(
    spi: *mut c_void,
    pReqQueryAccount: *const CThostFtdcReqQueryAccountField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_query_bank_account_money_by_future(&*pReqQueryAccount, &result)
}

///银行发起银期开户通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnOpenAccountByBank(
    spi: *mut c_void,
    pOpenAccount: *const CThostFtdcOpenAccountField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_open_account_by_bank(&*pOpenAccount)
}

///银行发起银期销户通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnCancelAccountByBank(
    spi: *mut c_void,
    pCancelAccount: *const CThostFtdcCancelAccountField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_cancel_account_by_bank(&*pCancelAccount)
}

///银行发起变更银行账号通知
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRtnChangeAccountByBank(
    spi: *mut c_void,
    pChangeAccount: *const CThostFtdcChangeAccountField,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    spi.on_rtn_change_account_by_bank(&*pChangeAccount)
}

///请求查询分类合约响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryClassifiedInstrument(
    spi: *mut c_void,
    pInstrument: *const CThostFtdcInstrumentField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_classified_instrument(&*pInstrument, &result)
}

///请求组合优惠比例响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryCombPromotionParam(
    spi: *mut c_void,
    pCombPromotionParam: *const CThostFtdcCombPromotionParamField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_comb_promotion_param(&*pCombPromotionParam, &result)
}

///投资者风险结算持仓查询响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryRiskSettleInvstPosition(
    spi: *mut c_void,
    pRiskSettleInvstPosition: *const CThostFtdcRiskSettleInvstPositionField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_risk_settle_invst_position(&*pRiskSettleInvstPosition, &result)
}

///风险结算产品查询响应
#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_OnRspQryRiskSettleProductStatus(
    spi: *mut c_void,
    pRiskSettleProductStatus: *const CThostFtdcRiskSettleProductStatusField,
    pRspInfo: *const CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
) {
    let spi = &mut **(spi as *mut *mut dyn TradeSpi);
    let result = Response::from(&*pRspInfo)
        .with_req_id(nRequestID)
        .with_is_last(bIsLast);
    spi.on_qry_risk_settle_product_status(&*pRiskSettleProductStatus, &result)
}

#[no_mangle]
pub unsafe extern "C" fn TradeSpiStub_Rust_Destructor(spi: *mut c_void) {
    let spi = spi as *mut Box<dyn TradeSpi>;
    let _: Box<Box<dyn TradeSpi>> = Box::from_raw(spi);
}
