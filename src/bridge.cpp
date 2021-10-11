
#include "bridge.hpp"

/////////////Quote Api////////////////
extern "C" CThostFtdcMdApi *CreateFtdcMdApi(const char *pszFlowPath, const bool bIsUsingUdp, const bool bIsMulticast)
{
    return CThostFtdcMdApi::CreateFtdcMdApi(pszFlowPath, bIsUsingUdp, bIsMulticast);
}

extern "C" const char *Quote_GetApiVersion()
{
    return CThostFtdcMdApi::GetApiVersion();
}
extern "C" void Quote_Init(CThostFtdcMdApi *self)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->Init();
}
extern "C" int Quote_Join(CThostFtdcMdApi *self)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->Join();
}
extern "C" const char *Quote_GetTradingDay(CThostFtdcMdApi *self)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->GetTradingDay();
}
extern "C" void Quote_RegisterFront(CThostFtdcMdApi *self, char *pszFrontAddress)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->RegisterFront(pszFrontAddress);
}
extern "C" void Quote_RegisterNameServer(CThostFtdcMdApi *self, char *pszNsAddress)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->RegisterNameServer(pszNsAddress);
}
extern "C" void Quote_RegisterFensUserInfo(CThostFtdcMdApi *self, CThostFtdcFensUserInfoField *pFensUserInfo)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->RegisterFensUserInfo(pFensUserInfo);
}
extern "C" void Quote_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi *pSpi)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->RegisterSpi(pSpi);
}
extern "C" int Quote_SubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->SubscribeMarketData(ppInstrumentID, nCount);
}
extern "C" int Quote_UnSubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->UnSubscribeMarketData(ppInstrumentID, nCount);
}
extern "C" int Quote_SubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->SubscribeForQuoteRsp(ppInstrumentID, nCount);
}
extern "C" int Quote_UnSubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->UnSubscribeForQuoteRsp(ppInstrumentID, nCount);
}
extern "C" int Quote_ReqUserLogin(CThostFtdcMdApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->ReqUserLogin(pReqUserLoginField, nRequestID);
}
extern "C" int Quote_ReqUserLogout(CThostFtdcMdApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->ReqUserLogout(pUserLogout, nRequestID);
}
extern "C" int Quote_ReqQryMulticastInstrument(CThostFtdcMdApi *self, CThostFtdcQryMulticastInstrumentField *pQryMulticastInstrument, int nRequestID)
{
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->ReqQryMulticastInstrument(pQryMulticastInstrument, nRequestID);
}

/////////////Trade Api////////////////
extern "C" CThostFtdcTraderApi *CreateFtdcTraderApi(const char *pszFlowPath)
{
    return CThostFtdcTraderApi::CreateFtdcTraderApi(pszFlowPath);
}
extern "C" const char *Trade_GetApiVersion()
{
    return CThostFtdcTraderApi::GetApiVersion();
}
extern "C" void Trade_Release(CThostFtdcTraderApi *self)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->Release();
}
extern "C" void Trade_Init(CThostFtdcTraderApi *self)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->Init();
}
extern "C" int Trade_Join(CThostFtdcTraderApi *self)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->Join();
}
extern "C" const char *Trade_GetTradingDay(CThostFtdcTraderApi *self)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->GetTradingDay();
}
extern "C" void Trade_RegisterFront(CThostFtdcTraderApi *self, char *pszFrontAddress)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->RegisterFront(pszFrontAddress);
}
extern "C" void Trade_RegisterNameServer(CThostFtdcTraderApi *self, char *pszNsAddress)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->RegisterNameServer(pszNsAddress);
}
extern "C" void Trade_RegisterFensUserInfo(CThostFtdcTraderApi *self, CThostFtdcFensUserInfoField *pFensUserInfo)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->RegisterFensUserInfo(pFensUserInfo);
}
extern "C" void Trade_RegisterSpi(CThostFtdcTraderApi *self, CThostFtdcTraderSpi *pSpi)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->RegisterSpi(pSpi);
}
extern "C" void Trade_SubscribePrivateTopic(CThostFtdcTraderApi *self, THOST_TE_RESUME_TYPE nResumeType)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->SubscribePrivateTopic(nResumeType);
}
extern "C" void Trade_SubscribePublicTopic(CThostFtdcTraderApi *self, THOST_TE_RESUME_TYPE nResumeType)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->SubscribePublicTopic(nResumeType);
}
extern "C" int Trade_ReqAuthenticate(CThostFtdcTraderApi *self, CThostFtdcReqAuthenticateField *pReqAuthenticateField, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqAuthenticate(pReqAuthenticateField, nRequestID);
}
extern "C" int Trade_RegisterUserSystemInfo(CThostFtdcTraderApi *self, CThostFtdcUserSystemInfoField *pUserSystemInfo)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->RegisterUserSystemInfo(pUserSystemInfo);
}
extern "C" int Trade_SubmitUserSystemInfo(CThostFtdcTraderApi *self, CThostFtdcUserSystemInfoField *pUserSystemInfo)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->SubmitUserSystemInfo(pUserSystemInfo);
}
extern "C" int Trade_ReqUserLogin(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserLogin(pReqUserLoginField, nRequestID);
}
extern "C" int Trade_ReqUserLogout(CThostFtdcTraderApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserLogout(pUserLogout, nRequestID);
}
extern "C" int Trade_ReqUserPasswordUpdate(CThostFtdcTraderApi *self, CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserPasswordUpdate(pUserPasswordUpdate, nRequestID);
}
extern "C" int Trade_ReqTradingAccountPasswordUpdate(CThostFtdcTraderApi *self, CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqTradingAccountPasswordUpdate(pTradingAccountPasswordUpdate, nRequestID);
}
extern "C" int Trade_ReqUserAuthMethod(CThostFtdcTraderApi *self, CThostFtdcReqUserAuthMethodField *pReqUserAuthMethod, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserAuthMethod(pReqUserAuthMethod, nRequestID);
}
extern "C" int Trade_ReqGenUserCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqGenUserCaptchaField *pReqGenUserCaptcha, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqGenUserCaptcha(pReqGenUserCaptcha, nRequestID);
}
extern "C" int Trade_ReqGenUserText(CThostFtdcTraderApi *self, CThostFtdcReqGenUserTextField *pReqGenUserText, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqGenUserText(pReqGenUserText, nRequestID);
}
extern "C" int Trade_ReqUserLoginWithCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithCaptchaField *pReqUserLoginWithCaptcha, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserLoginWithCaptcha(pReqUserLoginWithCaptcha, nRequestID);
}
extern "C" int Trade_ReqUserLoginWithText(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithTextField *pReqUserLoginWithText, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserLoginWithText(pReqUserLoginWithText, nRequestID);
}
extern "C" int Trade_ReqUserLoginWithOTP(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithOTPField *pReqUserLoginWithOTP, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqUserLoginWithOTP(pReqUserLoginWithOTP, nRequestID);
}
extern "C" int Trade_ReqOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputOrderField *pInputOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqOrderInsert(pInputOrder, nRequestID);
}
extern "C" int Trade_ReqParkedOrderInsert(CThostFtdcTraderApi *self, CThostFtdcParkedOrderField *pParkedOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqParkedOrderInsert(pParkedOrder, nRequestID);
}
extern "C" int Trade_ReqParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcParkedOrderActionField *pParkedOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqParkedOrderAction(pParkedOrderAction, nRequestID);
}
extern "C" int Trade_ReqOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputOrderActionField *pInputOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqOrderAction(pInputOrderAction, nRequestID);
}
extern "C" int Trade_ReqQryMaxOrderVolume(CThostFtdcTraderApi *self, CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryMaxOrderVolume(pQryMaxOrderVolume, nRequestID);
}
extern "C" int Trade_ReqSettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqSettlementInfoConfirm(pSettlementInfoConfirm, nRequestID);
}
extern "C" int Trade_ReqRemoveParkedOrder(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqRemoveParkedOrder(pRemoveParkedOrder, nRequestID);
}
extern "C" int Trade_ReqRemoveParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqRemoveParkedOrderAction(pRemoveParkedOrderAction, nRequestID);
}
extern "C" int Trade_ReqExecOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderField *pInputExecOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqExecOrderInsert(pInputExecOrder, nRequestID);
}
extern "C" int Trade_ReqExecOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderActionField *pInputExecOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqExecOrderAction(pInputExecOrderAction, nRequestID);
}
extern "C" int Trade_ReqForQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputForQuoteField *pInputForQuote, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqForQuoteInsert(pInputForQuote, nRequestID);
}
extern "C" int Trade_ReqQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputQuoteField *pInputQuote, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQuoteInsert(pInputQuote, nRequestID);
}
extern "C" int Trade_ReqQuoteAction(CThostFtdcTraderApi *self, CThostFtdcInputQuoteActionField *pInputQuoteAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQuoteAction(pInputQuoteAction, nRequestID);
}
extern "C" int Trade_ReqBatchOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqBatchOrderAction(pInputBatchOrderAction, nRequestID);
}
extern "C" int Trade_ReqOptionSelfCloseInsert(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqOptionSelfCloseInsert(pInputOptionSelfClose, nRequestID);
}
extern "C" int Trade_ReqOptionSelfCloseAction(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqOptionSelfCloseAction(pInputOptionSelfCloseAction, nRequestID);
}
extern "C" int Trade_ReqCombActionInsert(CThostFtdcTraderApi *self, CThostFtdcInputCombActionField *pInputCombAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqCombActionInsert(pInputCombAction, nRequestID);
}
extern "C" int Trade_ReqQryOrder(CThostFtdcTraderApi *self, CThostFtdcQryOrderField *pQryOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryOrder(pQryOrder, nRequestID);
}
extern "C" int Trade_ReqQryTrade(CThostFtdcTraderApi *self, CThostFtdcQryTradeField *pQryTrade, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTrade(pQryTrade, nRequestID);
}
extern "C" int Trade_ReqQryInvestorPosition(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionField *pQryInvestorPosition, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestorPosition(pQryInvestorPosition, nRequestID);
}
extern "C" int Trade_ReqQryTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTradingAccount(pQryTradingAccount, nRequestID);
}
extern "C" int Trade_ReqQryInvestor(CThostFtdcTraderApi *self, CThostFtdcQryInvestorField *pQryInvestor, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestor(pQryInvestor, nRequestID);
}
extern "C" int Trade_ReqQryTradingCode(CThostFtdcTraderApi *self, CThostFtdcQryTradingCodeField *pQryTradingCode, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTradingCode(pQryTradingCode, nRequestID);
}
extern "C" int Trade_ReqQryInstrumentMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentMarginRateField *pQryInstrumentMarginRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInstrumentMarginRate(pQryInstrumentMarginRate, nRequestID);
}
extern "C" int Trade_ReqQryInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentCommissionRateField *pQryInstrumentCommissionRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInstrumentCommissionRate(pQryInstrumentCommissionRate, nRequestID);
}
extern "C" int Trade_ReqQryExchange(CThostFtdcTraderApi *self, CThostFtdcQryExchangeField *pQryExchange, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryExchange(pQryExchange, nRequestID);
}
extern "C" int Trade_ReqQryProduct(CThostFtdcTraderApi *self, CThostFtdcQryProductField *pQryProduct, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryProduct(pQryProduct, nRequestID);
}
extern "C" int Trade_ReqQryInstrument(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentField *pQryInstrument, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInstrument(pQryInstrument, nRequestID);
}
extern "C" int Trade_ReqQryDepthMarketData(CThostFtdcTraderApi *self, CThostFtdcQryDepthMarketDataField *pQryDepthMarketData, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryDepthMarketData(pQryDepthMarketData, nRequestID);
}
extern "C" int Trade_ReqQrySettlementInfo(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoField *pQrySettlementInfo, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySettlementInfo(pQrySettlementInfo, nRequestID);
}
extern "C" int Trade_ReqQryTransferBank(CThostFtdcTraderApi *self, CThostFtdcQryTransferBankField *pQryTransferBank, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTransferBank(pQryTransferBank, nRequestID);
}
extern "C" int Trade_ReqQryInvestorPositionDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionDetailField *pQryInvestorPositionDetail, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestorPositionDetail(pQryInvestorPositionDetail, nRequestID);
}
extern "C" int Trade_ReqQryNotice(CThostFtdcTraderApi *self, CThostFtdcQryNoticeField *pQryNotice, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryNotice(pQryNotice, nRequestID);
}
extern "C" int Trade_ReqQrySettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoConfirmField *pQrySettlementInfoConfirm, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySettlementInfoConfirm(pQrySettlementInfoConfirm, nRequestID);
}
extern "C" int Trade_ReqQryInvestorPositionCombineDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionCombineDetailField *pQryInvestorPositionCombineDetail, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestorPositionCombineDetail(pQryInvestorPositionCombineDetail, nRequestID);
}
extern "C" int Trade_ReqQryCFMMCTradingAccountKey(CThostFtdcTraderApi *self, CThostFtdcQryCFMMCTradingAccountKeyField *pQryCFMMCTradingAccountKey, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryCFMMCTradingAccountKey(pQryCFMMCTradingAccountKey, nRequestID);
}
extern "C" int Trade_ReqQryEWarrantOffset(CThostFtdcTraderApi *self, CThostFtdcQryEWarrantOffsetField *pQryEWarrantOffset, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryEWarrantOffset(pQryEWarrantOffset, nRequestID);
}
extern "C" int Trade_ReqQryInvestorProductGroupMargin(CThostFtdcTraderApi *self, CThostFtdcQryInvestorProductGroupMarginField *pQryInvestorProductGroupMargin, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestorProductGroupMargin(pQryInvestorProductGroupMargin, nRequestID);
}
extern "C" int Trade_ReqQryExchangeMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateField *pQryExchangeMarginRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryExchangeMarginRate(pQryExchangeMarginRate, nRequestID);
}
extern "C" int Trade_ReqQryExchangeMarginRateAdjust(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateAdjustField *pQryExchangeMarginRateAdjust, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryExchangeMarginRateAdjust(pQryExchangeMarginRateAdjust, nRequestID);
}
extern "C" int Trade_ReqQryExchangeRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeRateField *pQryExchangeRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryExchangeRate(pQryExchangeRate, nRequestID);
}
extern "C" int Trade_ReqQrySecAgentACIDMap(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentACIDMapField *pQrySecAgentACIDMap, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySecAgentACIDMap(pQrySecAgentACIDMap, nRequestID);
}
extern "C" int Trade_ReqQryProductExchRate(CThostFtdcTraderApi *self, CThostFtdcQryProductExchRateField *pQryProductExchRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryProductExchRate(pQryProductExchRate, nRequestID);
}
extern "C" int Trade_ReqQryProductGroup(CThostFtdcTraderApi *self, CThostFtdcQryProductGroupField *pQryProductGroup, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryProductGroup(pQryProductGroup, nRequestID);
}
extern "C" int Trade_ReqQryMMInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryMMInstrumentCommissionRateField *pQryMMInstrumentCommissionRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryMMInstrumentCommissionRate(pQryMMInstrumentCommissionRate, nRequestID);
}
extern "C" int Trade_ReqQryMMOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryMMOptionInstrCommRateField *pQryMMOptionInstrCommRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryMMOptionInstrCommRate(pQryMMOptionInstrCommRate, nRequestID);
}
extern "C" int Trade_ReqQryInstrumentOrderCommRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentOrderCommRateField *pQryInstrumentOrderCommRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInstrumentOrderCommRate(pQryInstrumentOrderCommRate, nRequestID);
}
extern "C" int Trade_ReqQrySecAgentTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySecAgentTradingAccount(pQryTradingAccount, nRequestID);
}
extern "C" int Trade_ReqQrySecAgentCheckMode(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentCheckModeField *pQrySecAgentCheckMode, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySecAgentCheckMode(pQrySecAgentCheckMode, nRequestID);
}
extern "C" int Trade_ReqQrySecAgentTradeInfo(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentTradeInfoField *pQrySecAgentTradeInfo, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQrySecAgentTradeInfo(pQrySecAgentTradeInfo, nRequestID);
}
extern "C" int Trade_ReqQryOptionInstrTradeCost(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrTradeCostField *pQryOptionInstrTradeCost, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryOptionInstrTradeCost(pQryOptionInstrTradeCost, nRequestID);
}
extern "C" int Trade_ReqQryOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrCommRateField *pQryOptionInstrCommRate, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryOptionInstrCommRate(pQryOptionInstrCommRate, nRequestID);
}
extern "C" int Trade_ReqQryExecOrder(CThostFtdcTraderApi *self, CThostFtdcQryExecOrderField *pQryExecOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryExecOrder(pQryExecOrder, nRequestID);
}
extern "C" int Trade_ReqQryForQuote(CThostFtdcTraderApi *self, CThostFtdcQryForQuoteField *pQryForQuote, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryForQuote(pQryForQuote, nRequestID);
}
extern "C" int Trade_ReqQryQuote(CThostFtdcTraderApi *self, CThostFtdcQryQuoteField *pQryQuote, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryQuote(pQryQuote, nRequestID);
}
extern "C" int Trade_ReqQryOptionSelfClose(CThostFtdcTraderApi *self, CThostFtdcQryOptionSelfCloseField *pQryOptionSelfClose, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryOptionSelfClose(pQryOptionSelfClose, nRequestID);
}
extern "C" int Trade_ReqQryInvestUnit(CThostFtdcTraderApi *self, CThostFtdcQryInvestUnitField *pQryInvestUnit, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryInvestUnit(pQryInvestUnit, nRequestID);
}
extern "C" int Trade_ReqQryCombInstrumentGuard(CThostFtdcTraderApi *self, CThostFtdcQryCombInstrumentGuardField *pQryCombInstrumentGuard, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryCombInstrumentGuard(pQryCombInstrumentGuard, nRequestID);
}
extern "C" int Trade_ReqQryCombAction(CThostFtdcTraderApi *self, CThostFtdcQryCombActionField *pQryCombAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryCombAction(pQryCombAction, nRequestID);
}
extern "C" int Trade_ReqQryTransferSerial(CThostFtdcTraderApi *self, CThostFtdcQryTransferSerialField *pQryTransferSerial, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTransferSerial(pQryTransferSerial, nRequestID);
}
extern "C" int Trade_ReqQryAccountregister(CThostFtdcTraderApi *self, CThostFtdcQryAccountregisterField *pQryAccountregister, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryAccountregister(pQryAccountregister, nRequestID);
}
extern "C" int Trade_ReqQryContractBank(CThostFtdcTraderApi *self, CThostFtdcQryContractBankField *pQryContractBank, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryContractBank(pQryContractBank, nRequestID);
}
extern "C" int Trade_ReqQryParkedOrder(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderField *pQryParkedOrder, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryParkedOrder(pQryParkedOrder, nRequestID);
}
extern "C" int Trade_ReqQryParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderActionField *pQryParkedOrderAction, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryParkedOrderAction(pQryParkedOrderAction, nRequestID);
}
extern "C" int Trade_ReqQryTradingNotice(CThostFtdcTraderApi *self, CThostFtdcQryTradingNoticeField *pQryTradingNotice, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryTradingNotice(pQryTradingNotice, nRequestID);
}
extern "C" int Trade_ReqQryBrokerTradingParams(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingParamsField *pQryBrokerTradingParams, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryBrokerTradingParams(pQryBrokerTradingParams, nRequestID);
}
extern "C" int Trade_ReqQryBrokerTradingAlgos(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingAlgosField *pQryBrokerTradingAlgos, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryBrokerTradingAlgos(pQryBrokerTradingAlgos, nRequestID);
}
extern "C" int Trade_ReqQueryCFMMCTradingAccountToken(CThostFtdcTraderApi *self, CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQueryCFMMCTradingAccountToken(pQueryCFMMCTradingAccountToken, nRequestID);
}
extern "C" int Trade_ReqFromBankToFutureByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqFromBankToFutureByFuture(pReqTransfer, nRequestID);
}
extern "C" int Trade_ReqFromFutureToBankByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqFromFutureToBankByFuture(pReqTransfer, nRequestID);
}
extern "C" int Trade_ReqQueryBankAccountMoneyByFuture(CThostFtdcTraderApi *self, CThostFtdcReqQueryAccountField *pReqQueryAccount, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQueryBankAccountMoneyByFuture(pReqQueryAccount, nRequestID);
}
extern "C" int Trade_ReqQryClassifiedInstrument(CThostFtdcTraderApi *self, CThostFtdcQryClassifiedInstrumentField *pQryClassifiedInstrument, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryClassifiedInstrument(pQryClassifiedInstrument, nRequestID);
}
extern "C" int Trade_ReqQryCombPromotionParam(CThostFtdcTraderApi *self, CThostFtdcQryCombPromotionParamField *pQryCombPromotionParam, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryCombPromotionParam(pQryCombPromotionParam, nRequestID);
}
extern "C" int Trade_ReqQryRiskSettleInvstPosition(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleInvstPositionField *pQryRiskSettleInvstPosition, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryRiskSettleInvstPosition(pQryRiskSettleInvstPosition, nRequestID);
}
extern "C" int Trade_ReqQryRiskSettleProductStatus(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleProductStatusField *pQryRiskSettleProductStatus, int nRequestID)
{
    auto s = static_cast<CThostFtdcTraderApi *>(self);
    return s->ReqQryRiskSettleProductStatus(pQryRiskSettleProductStatus, nRequestID);
}

// *********************** QuoteSpiStub **************************
QuoteSpiStub::QuoteSpiStub(void *rust_object) : rust_object(rust_object) {}
QuoteSpiStub::~QuoteSpiStub()
{
    QuoteSpiStub_Rust_Destructor(this->rust_object);
}
void QuoteSpiStub::OnFrontConnected()
{
    return QuoteSpiStub_Rust_OnFrontConnected(this->rust_object);
}
void QuoteSpiStub::OnFrontDisconnected(int nReason)
{
    return QuoteSpiStub_Rust_OnFrontDisconnected(this->rust_object, nReason);
}
void QuoteSpiStub::OnHeartBeatWarning(int nTimeLapse)
{
    return QuoteSpiStub_Rust_OnHeartBeatWarning(this->rust_object, nTimeLapse);
}
void QuoteSpiStub::OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspUserLogin(this->rust_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspUserLogout(this->rust_object, pUserLogout, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField *pMulticastInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspQryMulticastInstrument(this->rust_object, pMulticastInstrument, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspError(this->rust_object, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspSubMarketData(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspUnSubMarketData(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspSubForQuoteRsp(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return QuoteSpiStub_Rust_OnRspUnSubForQuoteRsp(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}
void QuoteSpiStub::OnRtnDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData)
{
    return QuoteSpiStub_Rust_OnRtnDepthMarketData(this->rust_object, pDepthMarketData);
}
void QuoteSpiStub::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp)
{
    return QuoteSpiStub_Rust_OnRtnForQuoteRsp(this->rust_object, pForQuoteRsp);
}

// ***********************TradeSpiStub **************************
TradeSpiStub::TradeSpiStub(void *rust_object) : rust_object(rust_object) {}
TradeSpiStub::~TradeSpiStub()
{
    TradeSpiStub_Rust_Destructor(this->rust_object);
}

///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
void TradeSpiStub::OnFrontConnected()
{
    return TradeSpiStub_Rust_OnFrontConnected(this->rust_object);
}

///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
///@param nReason 错误原因
///        0x1001 网络读失败
///        0x1002 网络写失败
///        0x2001 接收心跳超时
///        0x2002 发送心跳失败
///        0x2003 收到错误报文
void TradeSpiStub::OnFrontDisconnected(int nReason)
{
    return TradeSpiStub_Rust_OnFrontDisconnected(this->rust_object, nReason);
}

///心跳超时警告。当长时间未收到报文时，该方法被调用。
///@param nTimeLapse 距离上次接收报文的时间
void TradeSpiStub::OnHeartBeatWarning(int nTimeLapse)
{
    return TradeSpiStub_Rust_OnHeartBeatWarning(this->rust_object, nTimeLapse);
}

///客户端认证响应
void TradeSpiStub::OnRspAuthenticate(CThostFtdcRspAuthenticateField *pRspAuthenticateField, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspAuthenticate(this->rust_object, pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);
}

///登录请求响应
void TradeSpiStub::OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspUserLogin(this->rust_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

///登出请求响应
void TradeSpiStub::OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspUserLogout(this->rust_object, pUserLogout, pRspInfo, nRequestID, bIsLast);
}

///用户口令更新请求响应
void TradeSpiStub::OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspUserPasswordUpdate(this->rust_object, pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}

///资金账户口令更新请求响应
void TradeSpiStub::OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspTradingAccountPasswordUpdate(this->rust_object, pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}

///查询用户当前支持的认证模式的回复
void TradeSpiStub::OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField *pRspUserAuthMethod, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspUserAuthMethod(this->rust_object, pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);
}

///获取图形验证码请求的回复
void TradeSpiStub::OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField *pRspGenUserCaptcha, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspGenUserCaptcha(this->rust_object, pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);
}

///获取短信验证码请求的回复
void TradeSpiStub::OnRspGenUserText(CThostFtdcRspGenUserTextField *pRspGenUserText, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspGenUserText(this->rust_object, pRspGenUserText, pRspInfo, nRequestID, bIsLast);
}

///报单录入请求响应
void TradeSpiStub::OnRspOrderInsert(CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspOrderInsert(this->rust_object, pInputOrder, pRspInfo, nRequestID, bIsLast);
}

///预埋单录入请求响应
void TradeSpiStub::OnRspParkedOrderInsert(CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspParkedOrderInsert(this->rust_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

///预埋撤单录入请求响应
void TradeSpiStub::OnRspParkedOrderAction(CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspParkedOrderAction(this->rust_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

///报单操作请求响应
void TradeSpiStub::OnRspOrderAction(CThostFtdcInputOrderActionField *pInputOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)

{
    return TradeSpiStub_Rust_OnRspOrderAction(this->rust_object, pInputOrderAction, pRspInfo, nRequestID, bIsLast);
}

///查询最大报单数量响应
void TradeSpiStub::OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryMaxOrderVolume(this->rust_object, pQryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);
}

///投资者结算结果确认响应
void TradeSpiStub::OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspSettlementInfoConfirm(this->rust_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

///删除预埋单响应
void TradeSpiStub::OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspRemoveParkedOrder(this->rust_object, pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);
}

///删除预埋撤单响应
void TradeSpiStub::OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspRemoveParkedOrderAction(this->rust_object, pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

///执行宣告录入请求响应
void TradeSpiStub::OnRspExecOrderInsert(CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspExecOrderInsert(this->rust_object, pInputExecOrder, pRspInfo, nRequestID, bIsLast);
}

///执行宣告操作请求响应
void TradeSpiStub::OnRspExecOrderAction(CThostFtdcInputExecOrderActionField *pInputExecOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspExecOrderAction(this->rust_object, pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);
}

///询价录入请求响应
void TradeSpiStub::OnRspForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspForQuoteInsert(this->rust_object, pInputForQuote, pRspInfo, nRequestID, bIsLast);
}

///报价录入请求响应
void TradeSpiStub::OnRspQuoteInsert(CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQuoteInsert(this->rust_object, pInputQuote, pRspInfo, nRequestID, bIsLast);
}

///报价操作请求响应
void TradeSpiStub::OnRspQuoteAction(CThostFtdcInputQuoteActionField *pInputQuoteAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQuoteAction(this->rust_object, pInputQuoteAction, pRspInfo, nRequestID, bIsLast);
}

///批量报单操作请求响应
void TradeSpiStub::OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspBatchOrderAction(this->rust_object, pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);
}

///期权自对冲录入请求响应
void TradeSpiStub::OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspOptionSelfCloseInsert(this->rust_object, pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

///期权自对冲操作请求响应
void TradeSpiStub::OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspOptionSelfCloseAction(this->rust_object, pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);
}

///申请组合录入请求响应
void TradeSpiStub::OnRspCombActionInsert(CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspCombActionInsert(this->rust_object, pInputCombAction, pRspInfo, nRequestID, bIsLast);
}

///请求查询报单响应
void TradeSpiStub::OnRspQryOrder(CThostFtdcOrderField *pOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryOrder(this->rust_object, pOrder, pRspInfo, nRequestID, bIsLast);
}

///请求查询成交响应
void TradeSpiStub::OnRspQryTrade(CThostFtdcTradeField *pTrade, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTrade(this->rust_object, pTrade, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者持仓响应
void TradeSpiStub::OnRspQryInvestorPosition(CThostFtdcInvestorPositionField *pInvestorPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestorPosition(this->rust_object, pInvestorPosition, pRspInfo, nRequestID, bIsLast);
}

///请求查询资金账户响应
void TradeSpiStub::OnRspQryTradingAccount(CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTradingAccount(this->rust_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者响应
void TradeSpiStub::OnRspQryInvestor(CThostFtdcInvestorField *pInvestor, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestor(this->rust_object, pInvestor, pRspInfo, nRequestID, bIsLast);
}

///请求查询交易编码响应
void TradeSpiStub::OnRspQryTradingCode(CThostFtdcTradingCodeField *pTradingCode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTradingCode(this->rust_object, pTradingCode, pRspInfo, nRequestID, bIsLast);
}

///请求查询合约保证金率响应
void TradeSpiStub::OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField *pInstrumentMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInstrumentMarginRate(this->rust_object, pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询合约手续费率响应
void TradeSpiStub::OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField *pInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInstrumentCommissionRate(this->rust_object, pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询交易所响应
void TradeSpiStub::OnRspQryExchange(CThostFtdcExchangeField *pExchange, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryExchange(this->rust_object, pExchange, pRspInfo, nRequestID, bIsLast);
}

///请求查询产品响应
void TradeSpiStub::OnRspQryProduct(CThostFtdcProductField *pProduct, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryProduct(this->rust_object, pProduct, pRspInfo, nRequestID, bIsLast);
}

///请求查询合约响应
void TradeSpiStub::OnRspQryInstrument(CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInstrument(this->rust_object, pInstrument, pRspInfo, nRequestID, bIsLast);
}

///请求查询行情响应
void TradeSpiStub::OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryDepthMarketData(this->rust_object, pDepthMarketData, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者结算结果响应
void TradeSpiStub::OnRspQrySettlementInfo(CThostFtdcSettlementInfoField *pSettlementInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySettlementInfo(this->rust_object, pSettlementInfo, pRspInfo, nRequestID, bIsLast);
}

///请求查询转帐银行响应
void TradeSpiStub::OnRspQryTransferBank(CThostFtdcTransferBankField *pTransferBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTransferBank(this->rust_object, pTransferBank, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者持仓明细响应
void TradeSpiStub::OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField *pInvestorPositionDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestorPositionDetail(this->rust_object, pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);
}

///请求查询客户通知响应
void TradeSpiStub::OnRspQryNotice(CThostFtdcNoticeField *pNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryNotice(this->rust_object, pNotice, pRspInfo, nRequestID, bIsLast);
}

///请求查询结算信息确认响应
void TradeSpiStub::OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySettlementInfoConfirm(this->rust_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者持仓明细响应
void TradeSpiStub::OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField *pInvestorPositionCombineDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestorPositionCombineDetail(this->rust_object, pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);
}

///查询保证金监管系统经纪公司资金账户密钥响应
void TradeSpiStub::OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField *pCFMMCTradingAccountKey, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryCFMMCTradingAccountKey(this->rust_object, pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);
}

///请求查询仓单折抵信息响应
void TradeSpiStub::OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField *pEWarrantOffset, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryEWarrantOffset(this->rust_object, pEWarrantOffset, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资者品种/跨品种保证金响应
void TradeSpiStub::OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField *pInvestorProductGroupMargin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestorProductGroupMargin(this->rust_object, pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);
}

///请求查询交易所保证金率响应
void TradeSpiStub::OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField *pExchangeMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryExchangeMarginRate(this->rust_object, pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询交易所调整保证金率响应
void TradeSpiStub::OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField *pExchangeMarginRateAdjust, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryExchangeMarginRateAdjust(this->rust_object, pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);
}

///请求查询汇率响应
void TradeSpiStub::OnRspQryExchangeRate(CThostFtdcExchangeRateField *pExchangeRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryExchangeRate(this->rust_object, pExchangeRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询二级代理操作员银期权限响应
void TradeSpiStub::OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField *pSecAgentACIDMap, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySecAgentACIDMap(this->rust_object, pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);
}

///请求查询产品报价汇率
void TradeSpiStub::OnRspQryProductExchRate(CThostFtdcProductExchRateField *pProductExchRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryProductExchRate(this->rust_object, pProductExchRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询产品组
void TradeSpiStub::OnRspQryProductGroup(CThostFtdcProductGroupField *pProductGroup, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryProductGroup(this->rust_object, pProductGroup, pRspInfo, nRequestID, bIsLast);
}

///请求查询做市商合约手续费率响应
void TradeSpiStub::OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField *pMMInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryMMInstrumentCommissionRate(this->rust_object, pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询做市商期权合约手续费响应
void TradeSpiStub::OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField *pMMOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryMMOptionInstrCommRate(this->rust_object, pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询报单手续费响应
void TradeSpiStub::OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField *pInstrumentOrderCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInstrumentOrderCommRate(this->rust_object, pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询资金账户响应
void TradeSpiStub::OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySecAgentTradingAccount(this->rust_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

///请求查询二级代理商资金校验模式响应
void TradeSpiStub::OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField *pSecAgentCheckMode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySecAgentCheckMode(this->rust_object, pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);
}

///请求查询二级代理商信息响应
void TradeSpiStub::OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField *pSecAgentTradeInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQrySecAgentTradeInfo(this->rust_object, pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);
}

///请求查询期权交易成本响应
void TradeSpiStub::OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField *pOptionInstrTradeCost, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryOptionInstrTradeCost(this->rust_object, pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);
}

///请求查询期权合约手续费响应
void TradeSpiStub::OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField *pOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryOptionInstrCommRate(this->rust_object, pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

///请求查询执行宣告响应
void TradeSpiStub::OnRspQryExecOrder(CThostFtdcExecOrderField *pExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryExecOrder(this->rust_object, pExecOrder, pRspInfo, nRequestID, bIsLast);
}

///请求查询询价响应
void TradeSpiStub::OnRspQryForQuote(CThostFtdcForQuoteField *pForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryForQuote(this->rust_object, pForQuote, pRspInfo, nRequestID, bIsLast);
}

///请求查询报价响应
void TradeSpiStub::OnRspQryQuote(CThostFtdcQuoteField *pQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryQuote(this->rust_object, pQuote, pRspInfo, nRequestID, bIsLast);
}

///请求查询期权自对冲响应
void TradeSpiStub::OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField *pOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryOptionSelfClose(this->rust_object, pOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

///请求查询投资单元响应
void TradeSpiStub::OnRspQryInvestUnit(CThostFtdcInvestUnitField *pInvestUnit, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryInvestUnit(this->rust_object, pInvestUnit, pRspInfo, nRequestID, bIsLast);
}

///请求查询组合合约安全系数响应
void TradeSpiStub::OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField *pCombInstrumentGuard, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryCombInstrumentGuard(this->rust_object, pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);
}

///请求查询申请组合响应
void TradeSpiStub::OnRspQryCombAction(CThostFtdcCombActionField *pCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryCombAction(this->rust_object, pCombAction, pRspInfo, nRequestID, bIsLast);
}

///请求查询转帐流水响应
void TradeSpiStub::OnRspQryTransferSerial(CThostFtdcTransferSerialField *pTransferSerial, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTransferSerial(this->rust_object, pTransferSerial, pRspInfo, nRequestID, bIsLast);
}

///请求查询银期签约关系响应
void TradeSpiStub::OnRspQryAccountregister(CThostFtdcAccountregisterField *pAccountregister, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryAccountregister(this->rust_object, pAccountregister, pRspInfo, nRequestID, bIsLast);
}

///错误应答
void TradeSpiStub::OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspError(this->rust_object, pRspInfo, nRequestID, bIsLast);
}

///报单通知
void TradeSpiStub::OnRtnOrder(CThostFtdcOrderField *pOrder)
{
    return TradeSpiStub_Rust_OnRtnOrder(this->rust_object, pOrder);
}

///成交通知
void TradeSpiStub::OnRtnTrade(CThostFtdcTradeField *pTrade)
{
    return TradeSpiStub_Rust_OnRtnTrade(this->rust_object, pTrade);
}

///报单录入错误回报
void TradeSpiStub::OnErrRtnOrderInsert(CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnOrderInsert(this->rust_object, pInputOrder, pRspInfo);
}

///报单操作错误回报
void TradeSpiStub::OnErrRtnOrderAction(CThostFtdcOrderActionField *pOrderAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnOrderAction(this->rust_object, pOrderAction, pRspInfo);
}

///合约交易状态通知
void TradeSpiStub::OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField *pInstrumentStatus)
{
    return TradeSpiStub_Rust_OnRtnInstrumentStatus(this->rust_object, pInstrumentStatus);
}

///交易所公告通知
void TradeSpiStub::OnRtnBulletin(CThostFtdcBulletinField *pBulletin)
{
    return TradeSpiStub_Rust_OnRtnBulletin(this->rust_object, pBulletin);
}

///交易通知
void TradeSpiStub::OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField *pTradingNoticeInfo)
{
    return TradeSpiStub_Rust_OnRtnTradingNotice(this->rust_object, pTradingNoticeInfo);
}

///提示条件单校验错误
void TradeSpiStub::OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField *pErrorConditionalOrder)
{
    return TradeSpiStub_Rust_OnRtnErrorConditionalOrder(this->rust_object, pErrorConditionalOrder);
}

///执行宣告通知
void TradeSpiStub::OnRtnExecOrder(CThostFtdcExecOrderField *pExecOrder)
{
    return TradeSpiStub_Rust_OnRtnExecOrder(this->rust_object, pExecOrder);
}

///执行宣告录入错误回报
void TradeSpiStub::OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnExecOrderInsert(this->rust_object, pInputExecOrder, pRspInfo);
}

///执行宣告操作错误回报
void TradeSpiStub::OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField *pExecOrderAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnExecOrderAction(this->rust_object, pExecOrderAction, pRspInfo);
}

///询价录入错误回报
void TradeSpiStub::OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnForQuoteInsert(this->rust_object, pInputForQuote, pRspInfo);
}

///报价通知
void TradeSpiStub::OnRtnQuote(CThostFtdcQuoteField *pQuote)
{
    return TradeSpiStub_Rust_OnRtnQuote(this->rust_object, pQuote);
}

///报价录入错误回报
void TradeSpiStub::OnErrRtnQuoteInsert(CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnQuoteInsert(this->rust_object, pInputQuote, pRspInfo);
}

///报价操作错误回报
void TradeSpiStub::OnErrRtnQuoteAction(CThostFtdcQuoteActionField *pQuoteAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnQuoteAction(this->rust_object, pQuoteAction, pRspInfo);
}

///询价通知
void TradeSpiStub::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp)
{
    return TradeSpiStub_Rust_OnRtnForQuoteRsp(this->rust_object, pForQuoteRsp);
}

///保证金监控中心用户令牌
void TradeSpiStub::OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField *pCFMMCTradingAccountToken)
{
    return TradeSpiStub_Rust_OnRtnCFMMCTradingAccountToken(this->rust_object, pCFMMCTradingAccountToken);
}

///批量报单操作错误回报
void TradeSpiStub::OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField *pBatchOrderAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnBatchOrderAction(this->rust_object, pBatchOrderAction, pRspInfo);
}

///期权自对冲通知
void TradeSpiStub::OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField *pOptionSelfClose)
{
    return TradeSpiStub_Rust_OnRtnOptionSelfClose(this->rust_object, pOptionSelfClose);
}

///期权自对冲录入错误回报
void TradeSpiStub::OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnOptionSelfCloseInsert(this->rust_object, pInputOptionSelfClose, pRspInfo);
}

///期权自对冲操作错误回报
void TradeSpiStub::OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField *pOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnOptionSelfCloseAction(this->rust_object, pOptionSelfCloseAction, pRspInfo);
}

///申请组合通知
void TradeSpiStub::OnRtnCombAction(CThostFtdcCombActionField *pCombAction)
{
    return TradeSpiStub_Rust_OnRtnCombAction(this->rust_object, pCombAction);
}

///申请组合录入错误回报
void TradeSpiStub::OnErrRtnCombActionInsert(CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnCombActionInsert(this->rust_object, pInputCombAction, pRspInfo);
}

///请求查询签约银行响应
void TradeSpiStub::OnRspQryContractBank(CThostFtdcContractBankField *pContractBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryContractBank(this->rust_object, pContractBank, pRspInfo, nRequestID, bIsLast);
}

///请求查询预埋单响应
void TradeSpiStub::OnRspQryParkedOrder(CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryParkedOrder(this->rust_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

///请求查询预埋撤单响应
void TradeSpiStub::OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryParkedOrderAction(this->rust_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

///请求查询交易通知响应
void TradeSpiStub::OnRspQryTradingNotice(CThostFtdcTradingNoticeField *pTradingNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryTradingNotice(this->rust_object, pTradingNotice, pRspInfo, nRequestID, bIsLast);
}

///请求查询经纪公司交易参数响应
void TradeSpiStub::OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField *pBrokerTradingParams, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryBrokerTradingParams(this->rust_object, pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);
}

///请求查询经纪公司交易算法响应
void TradeSpiStub::OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField *pBrokerTradingAlgos, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryBrokerTradingAlgos(this->rust_object, pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);
}

///请求查询监控中心用户令牌
void TradeSpiStub::OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQueryCFMMCTradingAccountToken(this->rust_object, pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast);
}

///银行发起银行资金转期货通知
void TradeSpiStub::OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField *pRspTransfer)
{
    return TradeSpiStub_Rust_OnRtnFromBankToFutureByBank(this->rust_object, pRspTransfer);
}

///银行发起期货资金转银行通知
void TradeSpiStub::OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField *pRspTransfer)
{
    return TradeSpiStub_Rust_OnRtnFromFutureToBankByBank(this->rust_object, pRspTransfer);
}

///银行发起冲正银行转期货通知
void TradeSpiStub::OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByBank(this->rust_object, pRspRepeal);
}

///银行发起冲正期货转银行通知
void TradeSpiStub::OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByBank(this->rust_object, pRspRepeal);
}

///期货发起银行资金转期货通知
void TradeSpiStub::OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField *pRspTransfer)
{
    return TradeSpiStub_Rust_OnRtnFromBankToFutureByFuture(this->rust_object, pRspTransfer);
}

///期货发起期货资金转银行通知
void TradeSpiStub::OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField *pRspTransfer)
{
    return TradeSpiStub_Rust_OnRtnFromFutureToBankByFuture(this->rust_object, pRspTransfer);
}

///系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
void TradeSpiStub::OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFutureManual(this->rust_object, pRspRepeal);
}

///系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
void TradeSpiStub::OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFutureManual(this->rust_object, pRspRepeal);
}

///期货发起查询银行余额通知
void TradeSpiStub::OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField *pNotifyQueryAccount)
{
    return TradeSpiStub_Rust_OnRtnQueryBankBalanceByFuture(this->rust_object, pNotifyQueryAccount);
}

///期货发起银行资金转期货错误回报
void TradeSpiStub::OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnBankToFutureByFuture(this->rust_object, pReqTransfer, pRspInfo);
}

///期货发起期货资金转银行错误回报
void TradeSpiStub::OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnFutureToBankByFuture(this->rust_object, pReqTransfer, pRspInfo);
}

///系统运行时期货端手工发起冲正银行转期货错误回报
void TradeSpiStub::OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnRepealBankToFutureByFutureManual(this->rust_object, pReqRepeal, pRspInfo);
}

///系统运行时期货端手工发起冲正期货转银行错误回报
void TradeSpiStub::OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnRepealFutureToBankByFutureManual(this->rust_object, pReqRepeal, pRspInfo);
}

///期货发起查询银行余额错误回报
void TradeSpiStub::OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo)
{
    return TradeSpiStub_Rust_OnErrRtnQueryBankBalanceByFuture(this->rust_object, pReqQueryAccount, pRspInfo);
}

///期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
void TradeSpiStub::OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFuture(this->rust_object, pRspRepeal);
}

///期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
void TradeSpiStub::OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField *pRspRepeal)
{
    return TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFuture(this->rust_object, pRspRepeal);
}

///期货发起银行资金转期货应答
void TradeSpiStub::OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspFromBankToFutureByFuture(this->rust_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

///期货发起期货资金转银行应答
void TradeSpiStub::OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspFromFutureToBankByFuture(this->rust_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

///期货发起查询银行余额应答
void TradeSpiStub::OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQueryBankAccountMoneyByFuture(this->rust_object, pReqQueryAccount, pRspInfo, nRequestID, bIsLast);
}

///银行发起银期开户通知
void TradeSpiStub::OnRtnOpenAccountByBank(CThostFtdcOpenAccountField *pOpenAccount)
{
    return TradeSpiStub_Rust_OnRtnOpenAccountByBank(this->rust_object, pOpenAccount);
}

///银行发起银期销户通知
void TradeSpiStub::OnRtnCancelAccountByBank(CThostFtdcCancelAccountField *pCancelAccount)
{
    return TradeSpiStub_Rust_OnRtnCancelAccountByBank(this->rust_object, pCancelAccount);
}

///银行发起变更银行账号通知
void TradeSpiStub::OnRtnChangeAccountByBank(CThostFtdcChangeAccountField *pChangeAccount)
{
    return TradeSpiStub_Rust_OnRtnChangeAccountByBank(this->rust_object, pChangeAccount);
}

///请求查询分类合约响应
void TradeSpiStub::OnRspQryClassifiedInstrument(CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryClassifiedInstrument(this->rust_object, pInstrument, pRspInfo, nRequestID, bIsLast);
}

///请求组合优惠比例响应
void TradeSpiStub::OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField *pCombPromotionParam, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryCombPromotionParam(this->rust_object, pCombPromotionParam, pRspInfo, nRequestID, bIsLast);
}

///投资者风险结算持仓查询响应
void TradeSpiStub::OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField *pRiskSettleInvstPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryRiskSettleInvstPosition(this->rust_object, pRiskSettleInvstPosition, pRspInfo, nRequestID, bIsLast);
}

///风险结算产品查询响应
void TradeSpiStub::OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField *pRiskSettleProductStatus, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    return TradeSpiStub_Rust_OnRspQryRiskSettleProductStatus(this->rust_object, pRiskSettleProductStatus, pRspInfo, nRequestID, bIsLast);
}
