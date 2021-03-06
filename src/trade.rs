mod stub_ffi;

use super::{Configuration, Response, ToArray};

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
    pub fn version<'a>() -> core::result::Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let ptr = Trade_GetApiVersion();
            CStr::from_ptr(ptr).to_str()
        }
    }

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
    pub fn with_configuration(mut self, conf: Configuration) -> Self {
        self.conf = conf;
        self
    }
    pub fn with_spi<T: TradeSpi>(mut self, spi: T) -> Self {
        let trait_object_box: Box<Box<dyn TradeSpi>> = Box::new(Box::new(spi));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<dyn TradeSpi> as *mut c_void;

        let trade_spi_stub = unsafe { TradeSpiStub::new(trait_object_pointer) };

        let ptr = Box::into_raw(Box::new(trade_spi_stub));
        self.stub = Some(ptr);
        unsafe { Trade_RegisterSpi(self.api, ptr as *mut CThostFtdcTraderSpi) };
        self
    }

    pub fn init(&self) {
        unsafe { Trade_Init(self.api) }
    }

    pub fn wait(&self) -> Result<()> {
        let ret = unsafe { Trade_Join(self.api) };
        if ret == 0 {
            Ok(())
        } else {
            Err(anyhow!("join error {}", ret))
        }
    }

    pub fn register_front(&self) -> Result<()> {
        let addr = CString::new(self.conf.front_addr.as_str())?;
        unsafe {
            Trade_RegisterFront(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_name_server(&self) -> Result<()> {
        let addr = CString::new(self.conf.ns_addr.as_str())?;
        unsafe {
            Trade_RegisterNameServer(self.api, addr.as_ptr() as *mut c_char);
        }
        Ok(())
    }

    pub fn register_fens_user_info(&self) -> Result<()> {
        let mut info = CThostFtdcFensUserInfoField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.LoginMode = self.conf.login_mode;
            Trade_RegisterFensUserInfo(self.api, &mut info);
        }
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

    pub fn authenticate(&self) -> Result<()> {
        let mut auth = CThostFtdcReqAuthenticateField::default();
        unsafe {
            auth.BrokerID = self.conf.broker_id.into_array::<11>();
            auth.UserID = self.conf.user_id.into_array::<16>();
            auth.UserProductInfo = self.conf.user_product_info.into_array::<11>();
            auth.AuthCode = self.conf.auth_code.into_array::<17>();
            auth.AppID = self.conf.appid.into_array::<33>();
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
        match unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLogin(self.api, &mut info, seq as i32)
        } {
            0 => Ok(()),
            ret => Err(anyhow!("logine error {}", ret)),
        }
    }

    pub fn logout(&self) -> Result<()> {
        let mut info = CThostFtdcUserLogoutField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLogout(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn update_passwd(&self, new_passwd: &str) -> Result<()> {
        let mut info = CThostFtdcUserPasswordUpdateField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.OldPassword = self.conf.passwd.into_array::<41>();
            info.NewPassword = new_passwd.into_array::<41>();
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
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.AccountID = account_id.into_array::<13>();
            info.CurrencyID = cny_id.into_array::<4>();
            info.OldPassword = old_passwd.into_array::<41>();
            info.NewPassword = new_passwd.into_array::<41>();

            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqTradingAccountPasswordUpdate(self.api, &mut info, seq as i32);
        }
        Ok(())
    }
    pub fn auth_method(&self, trading_day: &str) -> Result<()> {
        let mut info = CThostFtdcReqUserAuthMethodField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.TradingDay = trading_day.into_array::<9>();
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserAuthMethod(self.api, &mut info, seq as i32);
        }
        Ok(())
    }
    pub fn gen_user_captcha(&self, trading_day: &str) -> Result<()> {
        let mut info = CThostFtdcReqGenUserCaptchaField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.TradingDay = trading_day.into_array::<9>();
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqGenUserCaptcha(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn gen_user_text(&self, trading_day: &str) -> Result<()> {
        let mut info = CThostFtdcReqGenUserTextField::default();
        unsafe {
            info.BrokerID = self.conf.broker_id.into_array::<11>();
            info.UserID = self.conf.user_id.into_array::<16>();
            info.TradingDay = trading_day.into_array::<9>();
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqGenUserText(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn login_with_captcha(&self) -> Result<()> {
        let mut info =
            <CThostFtdcReqUserLoginWithCaptchaField as From<&Configuration>>::from(&self.conf);
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLoginWithCaptcha(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn login_with_text(&self) -> Result<()> {
        let mut info =
            <CThostFtdcReqUserLoginWithTextField as From<&Configuration>>::from(&self.conf);
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLoginWithText(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn login_with_otp(&self) -> Result<()> {
        let mut info =
            <CThostFtdcReqUserLoginWithOTPField as From<&Configuration>>::from(&self.conf);
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqUserLoginWithOTP(self.api, &mut info, seq as i32);
        }
        Ok(())
    }

    pub fn order_insert(&self, req: &mut CThostFtdcInputOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqOrderInsert(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn parked_order_insert(&self, req: &mut CThostFtdcParkedOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqParkedOrderInsert(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn parked_order_action(&self, req: &mut CThostFtdcParkedOrderActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqParkedOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn order_action(&self, req: &mut CThostFtdcInputOrderActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn query_max_order_volume(&self, req: &mut CThostFtdcQryMaxOrderVolumeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryMaxOrderVolume(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn settlement_info_confirm(
        &self,
        req: &mut CThostFtdcSettlementInfoConfirmField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqSettlementInfoConfirm(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn remove_parked_order(&self, req: &mut CThostFtdcRemoveParkedOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqRemoveParkedOrder(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn remove_parked_order_action(
        &self,
        req: &mut CThostFtdcRemoveParkedOrderActionField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqRemoveParkedOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn exec_order_insert(&self, req: &mut CThostFtdcInputExecOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqExecOrderInsert(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn exec_order_action(&self, req: &mut CThostFtdcInputExecOrderActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqExecOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn for_quote_insert(&self, req: &mut CThostFtdcInputForQuoteField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqForQuoteInsert(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn quote_insert(&self, req: &mut CThostFtdcInputQuoteField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQuoteInsert(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn quote_action(&self, req: &mut CThostFtdcInputQuoteActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQuoteAction(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn batch_order_action(&self, req: &mut CThostFtdcInputBatchOrderActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqBatchOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn option_self_close_insert(
        &self,
        req: &mut CThostFtdcInputOptionSelfCloseField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqOptionSelfCloseInsert(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn option_self_close_action(
        &self,
        req: &mut CThostFtdcInputOptionSelfCloseActionField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqOptionSelfCloseAction(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn comb_action_insert(&self, req: &mut CThostFtdcInputCombActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqCombActionInsert(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn query_order(&self, req: &mut CThostFtdcQryOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryOrder(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_trade(&self, req: &mut CThostFtdcQryTradeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTrade(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn query_investor_position(
        &self,
        req: &mut CThostFtdcQryInvestorPositionField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestorPosition(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn query_trading_account(&self, req: &mut CThostFtdcQryTradingAccountField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTradingAccount(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_investor(&self, req: &mut CThostFtdcQryInvestorField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestor(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_trading_code(&self, req: &mut CThostFtdcQryTradingCodeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTradingCode(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_instrument_margin_rate(
        &self,
        req: &mut CThostFtdcQryInstrumentMarginRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInstrumentMarginRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_instrument_commission_rate(
        &self,
        req: &mut CThostFtdcQryInstrumentCommissionRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInstrumentCommissionRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_exchange(&self, req: &mut CThostFtdcQryExchangeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryExchange(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_product(&self, req: &mut CThostFtdcQryProductField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryProduct(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_instrument(&self, req: &mut CThostFtdcQryInstrumentField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInstrument(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_depth_market_data(
        &self,
        req: &mut CThostFtdcQryDepthMarketDataField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryDepthMarketData(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_settlement_info(&self, req: &mut CThostFtdcQrySettlementInfoField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySettlementInfo(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_transfer_bank(&self, req: &mut CThostFtdcQryTransferBankField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTransferBank(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_investor_position_detail(
        &self,
        req: &mut CThostFtdcQryInvestorPositionDetailField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestorPositionDetail(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_notice(&self, req: &mut CThostFtdcQryNoticeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryNotice(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_settlement_info_confirm(
        &self,
        req: &mut CThostFtdcQrySettlementInfoConfirmField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySettlementInfoConfirm(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_investor_position_combine_detail(
        &self,
        req: &mut CThostFtdcQryInvestorPositionCombineDetailField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestorPositionCombineDetail(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_cfmmc_trading_account_key(
        &self,
        req: &mut CThostFtdcQryCFMMCTradingAccountKeyField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryCFMMCTradingAccountKey(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_ewarrant_offset(&self, req: &mut CThostFtdcQryEWarrantOffsetField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryEWarrantOffset(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_investor_product_group_margin(
        &self,
        req: &mut CThostFtdcQryInvestorProductGroupMarginField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestorProductGroupMargin(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_exchange_margin_rate(
        &self,
        req: &mut CThostFtdcQryExchangeMarginRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryExchangeMarginRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_exchange_margin_rate_adjust(
        &self,
        req: &mut CThostFtdcQryExchangeMarginRateAdjustField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryExchangeMarginRateAdjust(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_exchange_rate(&self, req: &mut CThostFtdcQryExchangeRateField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryExchangeRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_sec_agent_acid_map(
        &self,
        req: &mut CThostFtdcQrySecAgentACIDMapField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySecAgentACIDMap(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_product_exch_rate(
        &self,
        req: &mut CThostFtdcQryProductExchRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryProductExchRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_product_group(&self, req: &mut CThostFtdcQryProductGroupField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryProductGroup(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_mm_instrument_commission_rate(
        &self,
        req: &mut CThostFtdcQryMMInstrumentCommissionRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryMMInstrumentCommissionRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_mm_option_instr_comm_rate(
        &self,
        req: &mut CThostFtdcQryMMOptionInstrCommRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryMMOptionInstrCommRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_instrument_order_comm_rate(
        &self,
        req: &mut CThostFtdcQryInstrumentOrderCommRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInstrumentOrderCommRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_sec_agent_trading_account(
        &self,
        req: &mut CThostFtdcQryTradingAccountField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySecAgentTradingAccount(self.api, req, seq as i32);
        }
        Ok(())
    }

    pub fn query_sec_agent_check_mode(
        &self,
        req: &mut CThostFtdcQrySecAgentCheckModeField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySecAgentCheckMode(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_sec_agent_trade_info(
        &self,
        req: &mut CThostFtdcQrySecAgentTradeInfoField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQrySecAgentTradeInfo(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_option_instr_trade_cost(
        &self,
        req: &mut CThostFtdcQryOptionInstrTradeCostField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryOptionInstrTradeCost(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_option_instr_comm_rate(
        &self,
        req: &mut CThostFtdcQryOptionInstrCommRateField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryOptionInstrCommRate(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_exec_order(&self, req: &mut CThostFtdcQryExecOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryExecOrder(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_for_quote(&self, req: &mut CThostFtdcQryForQuoteField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryForQuote(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_quote(&self, req: &mut CThostFtdcQryQuoteField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryQuote(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_option_self_close(
        &self,
        req: &mut CThostFtdcQryOptionSelfCloseField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryOptionSelfClose(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_invest_unit(&self, req: &mut CThostFtdcQryInvestUnitField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryInvestUnit(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_comb_instrument_guard(
        &self,
        req: &mut CThostFtdcQryCombInstrumentGuardField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryCombInstrumentGuard(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_comb_action(&self, req: &mut CThostFtdcQryCombActionField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryCombAction(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_transfer_serial(&self, req: &mut CThostFtdcQryTransferSerialField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTransferSerial(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_account_register(
        &self,
        req: &mut CThostFtdcQryAccountregisterField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryAccountregister(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_contract_bank(&self, req: &mut CThostFtdcQryContractBankField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryContractBank(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_parked_order(&self, req: &mut CThostFtdcQryParkedOrderField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryParkedOrder(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_parked_order_action(
        &self,
        req: &mut CThostFtdcQryParkedOrderActionField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryParkedOrderAction(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_trading_notice(&self, req: &mut CThostFtdcQryTradingNoticeField) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryTradingNotice(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_broker_trading_params(
        &self,
        req: &mut CThostFtdcQryBrokerTradingParamsField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryBrokerTradingParams(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_broker_trading_algos(
        &self,
        req: &mut CThostFtdcQryBrokerTradingAlgosField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryBrokerTradingAlgos(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_cfmmc_trading_account_token(
        &self,
        req: &mut CThostFtdcQueryCFMMCTradingAccountTokenField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQueryCFMMCTradingAccountToken(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn from_bank_to_future_by_future(
        &self,
        req: &mut CThostFtdcReqTransferField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqFromBankToFutureByFuture(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn from_future_to_bank_by_future(
        &self,
        req: &mut CThostFtdcReqTransferField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqFromFutureToBankByFuture(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_bank_account_money_by_future(
        &self,
        req: &mut CThostFtdcReqQueryAccountField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQueryBankAccountMoneyByFuture(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_classified_instrument(
        &self,
        req: &mut CThostFtdcQryClassifiedInstrumentField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryClassifiedInstrument(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_comb_promotion_param(
        &self,
        req: &mut CThostFtdcQryCombPromotionParamField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryCombPromotionParam(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_risk_settle_invst_position(
        &self,
        req: &mut CThostFtdcQryRiskSettleInvstPositionField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryRiskSettleInvstPosition(self.api, req, seq as i32);
        }
        Ok(())
    }
    pub fn query_risk_settle_product_status(
        &self,
        req: &mut CThostFtdcQryRiskSettleProductStatusField,
    ) -> Result<()> {
        unsafe {
            let seq = self.seq.fetch_add(1, Ordering::SeqCst);
            Trade_ReqQryRiskSettleProductStatus(self.api, req, seq as i32);
        }
        Ok(())
    }
}

pub trait TradeSpi {
    ///????????????????????????????????????????????????????????????????????????????????????????????????
    fn on_connected(&self) {}

    ///???????????????????????????????????????????????????????????????????????????????????????????????????API???????????????????????????????????????????????????
    ///@param nReason ????????????
    ///        0x1001 ???????????????
    ///        0x1002 ???????????????
    ///        0x2001 ??????????????????
    ///        0x2002 ??????????????????
    ///        0x2003 ??????????????????
    fn on_disconnected(&self, reason: i32) {
        log::trace!("reason {}", reason);
    }

    ///???????????????????????????????????????????????????????????????????????????
    ///@param nTimeLapse ?????????????????????????????????
    fn on_heart_beat_warning(&self, timelapse: i32) {
        log::trace!("timelapse {}", timelapse);
    }

    ///?????????????????????
    fn on_authenticate(&self, info: &CThostFtdcRspAuthenticateField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????
    fn on_user_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????
    fn on_user_logout(&self, info: &CThostFtdcUserLogoutField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????

    ///??????????????????????????????
    fn on_user_password_update(&self, info: &CThostFtdcUserPasswordUpdateField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_trading_account_password_update(
        &self,
        info: &CThostFtdcTradingAccountPasswordUpdateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????????????????
    fn on_user_auth_method(&self, info: &CThostFtdcRspUserAuthMethodField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_gen_user_captcha(&self, info: &CThostFtdcRspGenUserCaptchaField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_gen_user_text(&self, info: &CThostFtdcRspGenUserTextField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_order_insert(&self, info: &CThostFtdcInputOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????
    fn on_parked_order_insert(&self, info: &CThostFtdcParkedOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_parked_order_action(&self, info: &CThostFtdcParkedOrderActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_order_action(&self, info: &CThostFtdcInputOrderActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_max_order_volume(&self, info: &CThostFtdcQryMaxOrderVolumeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_settlement_info_confirm(
        &self,
        info: &CThostFtdcSettlementInfoConfirmField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????
    fn on_remove_parked_order(&self, info: &CThostFtdcRemoveParkedOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_remove_parked_order_action(
        &self,
        info: &CThostFtdcRemoveParkedOrderActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_exec_order_insert(&self, info: &CThostFtdcInputExecOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_exec_order_action(&self, info: &CThostFtdcInputExecOrderActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_for_quote_insert(&self, info: &CThostFtdcInputForQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_quote_insert(&self, info: &CThostFtdcInputQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_quote_action(&self, info: &CThostFtdcInputQuoteActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_batch_order_action(
        &self,
        info: &CThostFtdcInputBatchOrderActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_option_self_close_insert(
        &self,
        info: &CThostFtdcInputOptionSelfCloseField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_option_self_close_action(
        &self,
        info: &CThostFtdcInputOptionSelfCloseActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_comb_action_insert(&self, info: &CThostFtdcInputCombActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_order(&self, info: &CThostFtdcOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_trade(&self, info: &CThostFtdcTradeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_qry_investor_position(&self, info: &CThostFtdcInvestorPositionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_trading_account(&self, info: &CThostFtdcTradingAccountField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????
    fn on_qry_investor(&self, info: &CThostFtdcInvestorField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_trading_code(&self, info: &CThostFtdcTradingCodeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_instrument_margin_rate(
        &self,
        info: &CThostFtdcInstrumentMarginRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_instrument_commission_rate(
        &self,
        info: &CThostFtdcInstrumentCommissionRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????
    fn on_qry_exchange(&self, info: &CThostFtdcExchangeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_product(&self, info: &CThostFtdcProductField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_instrument(&self, info: Option<&CThostFtdcInstrumentField>, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_depth_market_data(&self, info: &CThostFtdcDepthMarketDataField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_settlement_info(&self, info: &CThostFtdcSettlementInfoField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_transfer_bank(&self, info: &CThostFtdcTransferBankField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_investor_position_detail(
        &self,
        info: &CThostFtdcInvestorPositionDetailField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_notice(&self, info: &CThostFtdcNoticeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_settlement_info_confirm(
        &self,
        info: &CThostFtdcSettlementInfoConfirmField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_investor_position_combine_detail(
        &self,
        info: &CThostFtdcInvestorPositionCombineDetailField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????????????????????????????
    fn on_qry_cfmmc_trading_account_key(
        &self,
        info: &CThostFtdcCFMMCTradingAccountKeyField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_ewarrant_offset(&self, info: &CThostFtdcEWarrantOffsetField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????/????????????????????????
    fn on_qry_investor_product_group_margin(
        &self,
        info: &CThostFtdcInvestorProductGroupMarginField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_exchange_margin_rate(
        &self,
        info: &CThostFtdcExchangeMarginRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????????????????
    fn on_qry_exchange_margin_rate_adjust(
        &self,
        info: &CThostFtdcExchangeMarginRateAdjustField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_exchange_rate(&self, info: &CThostFtdcExchangeRateField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????????????????
    fn on_qry_sec_agent_acid_map(&self, info: &CThostFtdcSecAgentACIDMapField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_product_exch_rate(&self, info: &CThostFtdcProductExchRateField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????
    fn on_qry_product_group(&self, info: &CThostFtdcProductGroupField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????????????????
    fn on_qry_mm_instrument_commission_rate(
        &self,
        info: &CThostFtdcMMInstrumentCommissionRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????????????????
    fn on_qry_mm_option_instr_comm_rate(
        &self,
        info: &CThostFtdcMMOptionInstrCommRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_qry_instrument_order_comm_rate(
        &self,
        info: &CThostFtdcInstrumentOrderCommRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_sec_agent_trading_account(
        &self,
        info: &CThostFtdcTradingAccountField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????????????????
    fn on_qry_sec_agent_check_mode(
        &self,
        info: &CThostFtdcSecAgentCheckModeField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_sec_agent_trade_info(
        &self,
        info: &CThostFtdcSecAgentTradeInfoField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_option_instr_trade_cost(
        &self,
        info: &CThostFtdcOptionInstrTradeCostField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_option_instr_comm_rate(
        &self,
        info: &CThostFtdcOptionInstrCommRateField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_exec_order(&self, info: &CThostFtdcExecOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_for_quote(&self, info: &CThostFtdcForQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_qry_quote(&self, info: &CThostFtdcQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_qry_option_self_slose(&self, info: &CThostFtdcOptionSelfCloseField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_invest_unit(&self, info: &CThostFtdcInvestUnitField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????????????????
    fn on_qry_comb_instrument_guard(
        &self,
        info: &CThostFtdcCombInstrumentGuardField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_comb_action(&self, info: &CThostFtdcCombActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_transfer_serial(&self, info: &CThostFtdcTransferSerialField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_qry_account_register(&self, info: &CThostFtdcAccountregisterField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????
    fn on_error(&self, result: &Response) {
        log::trace!("result {:?}", result);
    }

    ///????????????
    fn on_rtn_order(&self, info: &CThostFtdcOrderField) {
        log::trace!("info {:?}", info);
    }

    ///????????????
    fn on_rtn_trade(&self, info: &CThostFtdcTradeField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????
    fn on_err_rtn_order_insert(&self, info: &CThostFtdcInputOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_err_rtn_order_action(&self, info: &CThostFtdcOrderActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_rtn_instrument_status(&self, info: &CThostFtdcInstrumentStatusField) {
        log::trace!("info {:?}", info);
    }

    ///?????????????????????
    fn on_rtn_bulletin(&self, info: &CThostFtdcBulletinField) {
        log::trace!("info {:?}", info);
    }

    ///????????????
    fn on_rtn_trading_notice(&self, info: &CThostFtdcTradingNoticeInfoField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????
    fn on_rtn_error_conditional_order(&self, info: &CThostFtdcErrorConditionalOrderField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????
    fn on_rtn_exec_order(&self, info: &CThostFtdcExecOrderField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????????????????
    fn on_err_rtn_exec_order_insert(
        &self,
        info: &CThostFtdcInputExecOrderField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_err_rtn_exec_order_action(
        &self,
        info: &CThostFtdcExecOrderActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_err_rtn_for_quote_insert(&self, info: &CThostFtdcInputForQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????
    fn on_rtn_quote(&self, info: &CThostFtdcQuoteField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????
    fn on_err_rtn_quote_insert(&self, info: &CThostFtdcInputQuoteField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????
    fn on_err_rtn_quote_action(&self, info: &CThostFtdcQuoteActionField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????
    fn on_rtn_for_quote(&self, info: &CThostFtdcForQuoteRspField) {
        log::trace!("info {:?}", info);
    }

    ///?????????????????????????????????
    fn on_rtn_cfmmc_trading_account_token(&self, info: &CThostFtdcCFMMCTradingAccountTokenField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????????????????
    fn on_err_rtn_batch_order_action(
        &self,
        info: &CThostFtdcBatchOrderActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????
    fn on_rtn_option_self_close(&self, info: &CThostFtdcOptionSelfCloseField) {
        log::trace!("info {:?}", info);
    }

    ///?????????????????????????????????
    fn on_err_rtn_option_self_close_insert(
        &self,
        info: &CThostFtdcInputOptionSelfCloseField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????
    fn on_err_rtn_option_self_close_action(
        &self,
        info: &CThostFtdcOptionSelfCloseActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????
    fn on_rtn_comb_action(&self, info: &CThostFtdcCombActionField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????????????????
    fn on_err_rtn_comb_action_insert(
        &self,
        info: &CThostFtdcInputCombActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_contract_bank(&self, info: &CThostFtdcContractBankField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????
    fn on_qry_parked_order(&self, info: &CThostFtdcParkedOrderField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_parked_order_action(
        &self,
        info: &CThostFtdcParkedOrderActionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_trading_notice(&self, info: &CThostFtdcTradingNoticeField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????????????????
    fn on_qry_broker_trading_params(
        &self,
        info: &CThostFtdcBrokerTradingParamsField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????????????????
    fn on_qry_broker_trading_algos(
        &self,
        info: &CThostFtdcBrokerTradingAlgosField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_query_cfmmc_trading_account_token(
        &self,
        info: &CThostFtdcQueryCFMMCTradingAccountTokenField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_rtn_from_bank_to_future_by_bank(&self, info: &CThostFtdcRspTransferField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_rtn_from_future_to_bank_by_bank(&self, info: &CThostFtdcRspTransferField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_rtn_repeal_from_bank_to_future_by_bank(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_rtn_repeal_from_future_to_bank_by_bank(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_rtn_from_bank_to_future_by_future(&self, info: &CThostFtdcRspTransferField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_rtn_from_future_to_bank_by_future(&self, info: &CThostFtdcRspTransferField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????????????????????????????????????????????????????????????????????????????????????????
    fn on_rtn_repeal_from_bank_to_future_by_future_manual(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????????????????????????????????????????????????????????????????????????????????????????
    fn on_rtn_repeal_from_future_to_bank_by_future_manual(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????????????????
    fn on_rtn_query_bank_balance_by_future(&self, info: &CThostFtdcNotifyQueryAccountField) {
        log::trace!("info {:?}", info);
    }

    ///?????????????????????????????????????????????
    fn on_err_rtn_bank_to_future_by_future(
        &self,
        info: &CThostFtdcReqTransferField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????????????????
    fn on_err_rtn_future_to_bank_by_future(
        &self,
        info: &CThostFtdcReqTransferField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????????????????????????????????????????
    fn on_err_rtn_repeal_bank_to_future_by_future_manual(
        &self,
        info: &CThostFtdcReqRepealField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///?????????????????????????????????????????????????????????????????????
    fn on_err_rtn_repeal_future_to_bank_by_future_manual(
        &self,
        info: &CThostFtdcReqRepealField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????????????????
    fn on_err_rtn_query_bank_balance_by_future(
        &self,
        info: &CThostFtdcReqQueryAccountField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????????????????????????????????????????????????????
    fn on_rtn_repeal_from_bank_to_future_by_future(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????????????????????????????????????????????????????????????????
    fn on_rtn_repeal_from_future_to_bank_by_future(&self, info: &CThostFtdcRspRepealField) {
        log::trace!("info {:?}", info);
    }

    ///???????????????????????????????????????
    fn on_from_bank_to_future_by_future(
        &self,
        info: &CThostFtdcReqTransferField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_from_future_to_bank_by_future(
        &self,
        info: &CThostFtdcReqTransferField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///????????????????????????????????????
    fn on_query_bank_account_money_by_future(
        &self,
        info: &CThostFtdcReqQueryAccountField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_rtn_open_account_by_bank(&self, info: &CThostFtdcOpenAccountField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????????????????
    fn on_rtn_cancel_account_by_bank(&self, info: &CThostFtdcCancelAccountField) {
        log::trace!("info {:?}", info);
    }

    ///????????????????????????????????????
    fn on_rtn_change_account_by_bank(&self, info: &CThostFtdcChangeAccountField) {
        log::trace!("info {:?}", info);
    }

    ///??????????????????????????????
    fn on_qry_classified_instrument(&self, info: &CThostFtdcInstrumentField, result: &Response) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_comb_promotion_param(
        &self,
        info: &CThostFtdcCombPromotionParamField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///???????????????????????????????????????
    fn on_qry_risk_settle_invst_position(
        &self,
        info: &CThostFtdcRiskSettleInvstPositionField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }

    ///??????????????????????????????
    fn on_qry_risk_settle_product_status(
        &self,
        info: &CThostFtdcRiskSettleProductStatusField,
        result: &Response,
    ) {
        log::trace!("info {:?} result {:?}", info, result);
    }
}

#[doc = "????????????"]
pub type ResumeType = THOST_TE_RESUME_TYPE;
