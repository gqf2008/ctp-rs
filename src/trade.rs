use super::{impl_ffi_convert, Configuration, FromCBuf};

use crate::ffi::*;
use anyhow::anyhow;
use anyhow::Result;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

impl Drop for TradeSpiStub {
    fn drop(&mut self) {
        unsafe { TradeSpiStub_Destructor(self) }
        // unreachable!("TraderSpiStub should be manually dropped!")
    }
}

pub struct TradeApi {
    api: *mut CThostFtdcTraderApi,
    stub: Option<*mut TradeSpiStub>,
    seq: AtomicUsize,
    conf: Configuration,
}

impl Drop for TradeApi {
    fn drop(&mut self) {
        unsafe { Trade_Release(self.api) };
        if let Some(stub) = self.stub {
            unsafe { TradeSpiStub_Destructor(stub) };
        }
    }
}

unsafe impl Send for TradeApi {}
unsafe impl Sync for TradeApi {}

impl TradeApi {
    pub fn new(path: &str) -> Result<Self> {
        let path = CString::new(path)?;
        let api = unsafe { CreateFtdcTraderApi(path.as_ptr()) };
        Ok(Self {
            api,
            stub: None,
            seq: AtomicUsize::new(0),
            conf: Default::default(),
        })
    }
    pub fn with_user_login(mut self, conf: Configuration) -> Self {
        self.conf = conf;
        self
    }

    pub fn version<'a>() -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Trade_GetApiVersion();
            CStr::from_ptr(ptr).to_str()
        }
    }

    pub fn init(&self) {
        unsafe { Trade_Init(self.api) }
    }

    pub fn join(&self) -> Result<()> {
        let ret = unsafe { Trade_Join(self.api) };
        if ret == 0 {
            Ok(())
        } else {
            Err(anyhow!("join error {}", ret))
        }
    }

    pub fn register_front(&self, addr: &str) -> Result<()> {
        let addr = CString::new(addr)?;
        unsafe {
            Trade_RegisterFront(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_name_server(&self, addr: &str) -> Result<()> {
        let addr = CString::new(addr)?;
        unsafe {
            Trade_RegisterNameServer(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_fens_user_info(&self, mode: i8) -> Result<()> {
        let mut info = CThostFtdcFensUserInfoField::default();
        unsafe {
            info.BrokerID
                .clone_from_slice(std::mem::transmute(self.conf.broker_id.as_str()));
            info.UserID
                .clone_from_slice(std::mem::transmute(self.conf.user_id.as_str()));
            info.LoginMode = mode;
            Trade_RegisterFensUserInfo(self.api, &mut info);
        }
        Ok(())
    }

    pub fn register_spi<T: TradeSpi>(&mut self, spi: T) -> Result<()> {
        let trait_object_box: Box<Box<dyn TradeSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn TradeSpi> as *mut c_void;

        let trade_spi_stub = unsafe { TradeSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(trade_spi_stub));
        self.stub = Some(ptr);
        unsafe { Trade_RegisterSpi(self.api, ptr as *mut CThostFtdcTraderSpi) };
        Ok(())
    }
    pub fn subscribe_private_topic(&self, resume_type: ResumeType) -> Result<()> {
        unsafe {
            Trade_SubscribePrivateTopic(self.api, resume_type);
        }
        Ok(())
    }
    pub fn subscribe_public_topic(&self, resume_type: ResumeType) -> Result<()> {
        unsafe {
            Trade_SubscribePublicTopic(self.api, resume_type);
        }
        Ok(())
    }

    pub fn authenticate(
        &self,
        broker_id: &str,
        user_id: &str,
        user_product_info: &str,
        auth_code: &str,
        appid: &str,
    ) -> Result<()> {
        let mut auth = CThostFtdcReqAuthenticateField::default();
        unsafe {
            auth.BrokerID
                .clone_from_slice(std::mem::transmute(broker_id));
            auth.UserID.clone_from_slice(std::mem::transmute(user_id));
            auth.UserProductInfo
                .clone_from_slice(std::mem::transmute(user_product_info));
            auth.AuthCode
                .clone_from_slice(std::mem::transmute(auth_code));
            auth.AppID.clone_from_slice(std::mem::transmute(appid));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqAuthenticate(self.api, &mut auth, seq as i32);
        }
        Ok(())
    }
    pub fn register_user_system_info(&self) -> Result<()> {
        let mut info = <CThostFtdcUserSystemInfoField as From<&Configuration>>::from(&self.conf);
        unsafe {
            Trade_RegisterUserSystemInfo(self.api, &mut info);
        }
        Ok(())
    }

    pub fn submit_user_system_info(&self) -> Result<()> {
        let mut info = <CThostFtdcUserSystemInfoField as From<&Configuration>>::from(&self.conf);
        unsafe {
            Trade_SubmitUserSystemInfo(self.api, &mut info);
        }
        Ok(())
    }
    pub fn login(&self) -> Result<()> {
        let mut info = <CThostFtdcReqUserLoginField as From<&Configuration>>::from(&self.conf);
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLogin(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn logout(&self) -> Result<()> {
        let mut info = CThostFtdcUserLogoutField::default();
        unsafe {
            info.BrokerID
                .copy_from_slice(std::mem::transmute(self.conf.broker_id.as_str()));
            info.UserID
                .copy_from_slice(std::mem::transmute(self.conf.user_id.as_str()));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLogout(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn update_passwd(&self, new_passwd: &str) -> Result<()> {
        let mut info = CThostFtdcUserPasswordUpdateField::default();
        unsafe {
            info.BrokerID
                .copy_from_slice(std::mem::transmute(self.conf.broker_id.as_str()));
            info.UserID
                .copy_from_slice(std::mem::transmute(self.conf.user_id.as_str()));
            info.OldPassword
                .copy_from_slice(std::mem::transmute(self.conf.passwd.as_str()));
            info.NewPassword
                .copy_from_slice(std::mem::transmute(new_passwd));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserPasswordUpdate(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn update_trade_account_passwd(
        &self,
        account_id: &str,
        cny_id: &str,
        old_passwd: &str,
        new_passwd: &str,
    ) -> Result<()> {
        let mut info = CThostFtdcTradingAccountPasswordUpdateField::default();
        unsafe {
            info.BrokerID
                .copy_from_slice(std::mem::transmute(self.conf.broker_id.as_str()));
            info.AccountID
                .copy_from_slice(std::mem::transmute(account_id));
            info.CurrencyID.copy_from_slice(std::mem::transmute(cny_id));
            info.OldPassword
                .copy_from_slice(std::mem::transmute(old_passwd));
            info.NewPassword
                .copy_from_slice(std::mem::transmute(new_passwd));
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqTradingAccountPasswordUpdate(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    // extern "C" int Trade_ReqUserAuthMethod(CThostFtdcTraderApi *self, CThostFtdcReqUserAuthMethodField *pReqUserAuthMethod, int nRequestID);
    // extern "C" int Trade_ReqGenUserCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqGenUserCaptchaField *pReqGenUserCaptcha, int nRequestID);
    // extern "C" int Trade_ReqGenUserText(CThostFtdcTraderApi *self, CThostFtdcReqGenUserTextField *pReqGenUserText, int nRequestID);
    // extern "C" int Trade_ReqUserLoginWithCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithCaptchaField *pReqUserLoginWithCaptcha, int nRequestID);
    // extern "C" int Trade_ReqUserLoginWithText(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithTextField *pReqUserLoginWithText, int nRequestID);
    // extern "C" int Trade_ReqUserLoginWithOTP(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithOTPField *pReqUserLoginWithOTP, int nRequestID);
    // extern "C" int Trade_ReqOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputOrderField *pInputOrder, int nRequestID);
    // extern "C" int Trade_ReqParkedOrderInsert(CThostFtdcTraderApi *self, CThostFtdcParkedOrderField *pParkedOrder, int nRequestID);
    // extern "C" int Trade_ReqParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcParkedOrderActionField *pParkedOrderAction, int nRequestID);
    // extern "C" int Trade_ReqOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputOrderActionField *pInputOrderAction, int nRequestID);
    // extern "C" int Trade_ReqQryMaxOrderVolume(CThostFtdcTraderApi *self, CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, int nRequestID);
    // extern "C" int Trade_ReqSettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, int nRequestID);
    // extern "C" int Trade_ReqRemoveParkedOrder(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, int nRequestID);
    // extern "C" int Trade_ReqRemoveParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, int nRequestID);
    // extern "C" int Trade_ReqExecOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderField *pInputExecOrder, int nRequestID);
    // extern "C" int Trade_ReqExecOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderActionField *pInputExecOrderAction, int nRequestID);
    // extern "C" int Trade_ReqForQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputForQuoteField *pInputForQuote, int nRequestID);
    // extern "C" int Trade_ReqQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputQuoteField *pInputQuote, int nRequestID);
    // extern "C" int Trade_ReqQuoteAction(CThostFtdcTraderApi *self, CThostFtdcInputQuoteActionField *pInputQuoteAction, int nRequestID);
    // extern "C" int Trade_ReqBatchOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, int nRequestID);
    // extern "C" int Trade_ReqOptionSelfCloseInsert(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, int nRequestID);
    // extern "C" int Trade_ReqOptionSelfCloseAction(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, int nRequestID);
    // extern "C" int Trade_ReqCombActionInsert(CThostFtdcTraderApi *self, CThostFtdcInputCombActionField *pInputCombAction, int nRequestID);
    // extern "C" int Trade_ReqQryOrder(CThostFtdcTraderApi *self, CThostFtdcQryOrderField *pQryOrder, int nRequestID);
    // extern "C" int Trade_ReqQryTrade(CThostFtdcTraderApi *self, CThostFtdcQryTradeField *pQryTrade, int nRequestID);
    // extern "C" int Trade_ReqQryInvestorPosition(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionField *pQryInvestorPosition, int nRequestID);
    // extern "C" int Trade_ReqQryTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID);
    // extern "C" int Trade_ReqQryInvestor(CThostFtdcTraderApi *self, CThostFtdcQryInvestorField *pQryInvestor, int nRequestID);
    // extern "C" int Trade_ReqQryTradingCode(CThostFtdcTraderApi *self, CThostFtdcQryTradingCodeField *pQryTradingCode, int nRequestID);
    // extern "C" int Trade_ReqQryInstrumentMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentMarginRateField *pQryInstrumentMarginRate, int nRequestID);
    // extern "C" int Trade_ReqQryInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentCommissionRateField *pQryInstrumentCommissionRate, int nRequestID);
    // extern "C" int Trade_ReqQryExchange(CThostFtdcTraderApi *self, CThostFtdcQryExchangeField *pQryExchange, int nRequestID);
    // extern "C" int Trade_ReqQryProduct(CThostFtdcTraderApi *self, CThostFtdcQryProductField *pQryProduct, int nRequestID);
    // extern "C" int Trade_ReqQryInstrument(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentField *pQryInstrument, int nRequestID);
    // extern "C" int Trade_ReqQryDepthMarketData(CThostFtdcTraderApi *self, CThostFtdcQryDepthMarketDataField *pQryDepthMarketData, int nRequestID);
    // extern "C" int Trade_ReqQrySettlementInfo(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoField *pQrySettlementInfo, int nRequestID);
    // extern "C" int Trade_ReqQryTransferBank(CThostFtdcTraderApi *self, CThostFtdcQryTransferBankField *pQryTransferBank, int nRequestID);
    // extern "C" int Trade_ReqQryInvestorPositionDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionDetailField *pQryInvestorPositionDetail, int nRequestID);
    // extern "C" int Trade_ReqQryNotice(CThostFtdcTraderApi *self, CThostFtdcQryNoticeField *pQryNotice, int nRequestID);
    // extern "C" int Trade_ReqQrySettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoConfirmField *pQrySettlementInfoConfirm, int nRequestID);
    // extern "C" int Trade_ReqQryInvestorPositionCombineDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionCombineDetailField *pQryInvestorPositionCombineDetail, int nRequestID);
    // extern "C" int Trade_ReqQryCFMMCTradingAccountKey(CThostFtdcTraderApi *self, CThostFtdcQryCFMMCTradingAccountKeyField *pQryCFMMCTradingAccountKey, int nRequestID);
    // extern "C" int Trade_ReqQryEWarrantOffset(CThostFtdcTraderApi *self, CThostFtdcQryEWarrantOffsetField *pQryEWarrantOffset, int nRequestID);
    // extern "C" int Trade_ReqQryInvestorProductGroupMargin(CThostFtdcTraderApi *self, CThostFtdcQryInvestorProductGroupMarginField *pQryInvestorProductGroupMargin, int nRequestID);
    // extern "C" int Trade_ReqQryExchangeMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateField *pQryExchangeMarginRate, int nRequestID);
    // extern "C" int Trade_ReqQryExchangeMarginRateAdjust(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateAdjustField *pQryExchangeMarginRateAdjust, int nRequestID);
    // extern "C" int Trade_ReqQryExchangeRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeRateField *pQryExchangeRate, int nRequestID);
    // extern "C" int Trade_ReqQrySecAgentACIDMap(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentACIDMapField *pQrySecAgentACIDMap, int nRequestID);
    // extern "C" int Trade_ReqQryProductExchRate(CThostFtdcTraderApi *self, CThostFtdcQryProductExchRateField *pQryProductExchRate, int nRequestID);
    // extern "C" int Trade_ReqQryProductGroup(CThostFtdcTraderApi *self, CThostFtdcQryProductGroupField *pQryProductGroup, int nRequestID);
    // extern "C" int Trade_ReqQryMMInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryMMInstrumentCommissionRateField *pQryMMInstrumentCommissionRate, int nRequestID);
    // extern "C" int Trade_ReqQryMMOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryMMOptionInstrCommRateField *pQryMMOptionInstrCommRate, int nRequestID);
    // extern "C" int Trade_ReqQryInstrumentOrderCommRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentOrderCommRateField *pQryInstrumentOrderCommRate, int nRequestID);
    // extern "C" int Trade_ReqQrySecAgentTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID);
    // extern "C" int Trade_ReqQrySecAgentCheckMode(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentCheckModeField *pQrySecAgentCheckMode, int nRequestID);
    // extern "C" int Trade_ReqQrySecAgentTradeInfo(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentTradeInfoField *pQrySecAgentTradeInfo, int nRequestID);
    // extern "C" int Trade_ReqQryOptionInstrTradeCost(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrTradeCostField *pQryOptionInstrTradeCost, int nRequestID);
    // extern "C" int Trade_ReqQryOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrCommRateField *pQryOptionInstrCommRate, int nRequestID);
    // extern "C" int Trade_ReqQryExecOrder(CThostFtdcTraderApi *self, CThostFtdcQryExecOrderField *pQryExecOrder, int nRequestID);
    // extern "C" int Trade_ReqQryForQuote(CThostFtdcTraderApi *self, CThostFtdcQryForQuoteField *pQryForQuote, int nRequestID);
    // extern "C" int Trade_ReqQryQuote(CThostFtdcTraderApi *self, CThostFtdcQryQuoteField *pQryQuote, int nRequestID);
    // extern "C" int Trade_ReqQryOptionSelfClose(CThostFtdcTraderApi *self, CThostFtdcQryOptionSelfCloseField *pQryOptionSelfClose, int nRequestID);
    // extern "C" int Trade_ReqQryInvestUnit(CThostFtdcTraderApi *self, CThostFtdcQryInvestUnitField *pQryInvestUnit, int nRequestID);
    // extern "C" int Trade_ReqQryCombInstrumentGuard(CThostFtdcTraderApi *self, CThostFtdcQryCombInstrumentGuardField *pQryCombInstrumentGuard, int nRequestID);
    // extern "C" int Trade_ReqQryCombAction(CThostFtdcTraderApi *self, CThostFtdcQryCombActionField *pQryCombAction, int nRequestID);
    // extern "C" int Trade_ReqQryTransferSerial(CThostFtdcTraderApi *self, CThostFtdcQryTransferSerialField *pQryTransferSerial, int nRequestID);
    // extern "C" int Trade_ReqQryAccountregister(CThostFtdcTraderApi *self, CThostFtdcQryAccountregisterField *pQryAccountregister, int nRequestID);
    // extern "C" int Trade_ReqQryContractBank(CThostFtdcTraderApi *self, CThostFtdcQryContractBankField *pQryContractBank, int nRequestID);
    // extern "C" int Trade_ReqQryParkedOrder(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderField *pQryParkedOrder, int nRequestID);
    // extern "C" int Trade_ReqQryParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderActionField *pQryParkedOrderAction, int nRequestID);
    // extern "C" int Trade_ReqQryTradingNotice(CThostFtdcTraderApi *self, CThostFtdcQryTradingNoticeField *pQryTradingNotice, int nRequestID);
    // extern "C" int Trade_ReqQryBrokerTradingParams(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingParamsField *pQryBrokerTradingParams, int nRequestID);
    // extern "C" int Trade_ReqQryBrokerTradingAlgos(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingAlgosField *pQryBrokerTradingAlgos, int nRequestID);
    // extern "C" int Trade_ReqQueryCFMMCTradingAccountToken(CThostFtdcTraderApi *self, CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, int nRequestID);
    // extern "C" int Trade_ReqFromBankToFutureByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID);
    // extern "C" int Trade_ReqFromFutureToBankByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID);
    // extern "C" int Trade_ReqQueryBankAccountMoneyByFuture(CThostFtdcTraderApi *self, CThostFtdcReqQueryAccountField *pReqQueryAccount, int nRequestID);
    // extern "C" int Trade_ReqQryClassifiedInstrument(CThostFtdcTraderApi *self, CThostFtdcQryClassifiedInstrumentField *pQryClassifiedInstrument, int nRequestID);
    // extern "C" int Trade_ReqQryCombPromotionParam(CThostFtdcTraderApi *self, CThostFtdcQryCombPromotionParamField *pQryCombPromotionParam, int nRequestID);
    // extern "C" int Trade_ReqQryRiskSettleInvstPosition(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleInvstPositionField *pQryRiskSettleInvstPosition, int nRequestID);
    // extern "C" int Trade_ReqQryRiskSettleProductStatus(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleProductStatusField *pQryRiskSettleProductStatus, int nRequestID);
}

pub trait TradeSpi {}

#[doc = "重传方式"]
pub type ResumeType = THOST_TE_RESUME_TYPE;

impl<'a> From<&'a Configuration> for CThostFtdcUserSystemInfoField {
    fn from(r: &'a Configuration) -> CThostFtdcUserSystemInfoField {
        let mut field = CThostFtdcUserSystemInfoField::default();
        unsafe {
            field
                .BrokerID
                .clone_from_slice(std::mem::transmute(r.broker_id.as_str()));
            field
                .UserID
                .clone_from_slice(std::mem::transmute(r.user_id.as_str()));
            field
                .ClientSystemInfo
                .clone_from_slice(std::mem::transmute(r.system_info.as_str()));
            field.ClientSystemInfoLen = r.system_info.as_bytes().len() as i32;
            field.ClientIPPort = r.port;
            field
                .ClientLoginTime
                .clone_from_slice(std::mem::transmute(r.login_time.as_str()));
            field
                .ClientAppID
                .clone_from_slice(std::mem::transmute(r.appid.as_str()));
            field
                .ClientPublicIP
                .clone_from_slice(std::mem::transmute(r.public_ip.as_str()));
            field
                .ClientLoginRemark
                .clone_from_slice(std::mem::transmute(r.login_remark.as_str()));
        }
        field
    }
}

impl<'a> From<&'a Configuration> for CThostFtdcReqUserLoginField {
    fn from(r: &'a Configuration) -> CThostFtdcReqUserLoginField {
        let mut field = CThostFtdcReqUserLoginField::default();
        unsafe {
            field
                .BrokerID
                .clone_from_slice(std::mem::transmute(r.broker_id.as_str()));
            field
                .UserID
                .clone_from_slice(std::mem::transmute(r.user_id.as_str()));
            field
                .Password
                .clone_from_slice(std::mem::transmute(r.passwd.as_str()));

            field.ClientIPPort = r.port;
            field
                .UserProductInfo
                .clone_from_slice(std::mem::transmute(r.user_product_info.as_str()));
            field
                .InterfaceProductInfo
                .clone_from_slice(std::mem::transmute(r.interface_product_info.as_str()));
            field
                .ProtocolInfo
                .clone_from_slice(std::mem::transmute(r.protocol_info.as_str()));
            field
                .MacAddress
                .clone_from_slice(std::mem::transmute(r.mac_addr.as_str()));
            field
                .OneTimePassword
                .clone_from_slice(std::mem::transmute(r.one_time_passwd.as_str()));
            field
                .LoginRemark
                .clone_from_slice(std::mem::transmute(r.login_remark.as_str()));
            field
                .ClientIPAddress
                .clone_from_slice(std::mem::transmute(r.ip_addr.as_str()));
        }
        field
    }
}
