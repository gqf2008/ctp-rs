#ifndef CTP_RUST_BRIDGE_H_
#define CTP_RUST_BRIDGE_H_

#include "../lib/include/ThostFtdcUserApiDataType.h"
#include "../lib/include/ThostFtdcUserApiStruct.h"
#include "../lib/include/ThostFtdcMdApi.h"
#include "../lib/include/ThostFtdcTraderApi.h"

/////////////Quote Api////////////////
extern "C" CThostFtdcMdApi *CreateFtdcMdApi(const char *pszFlowPath, const bool bIsUsingUdp, const bool bIsMulticast);
extern "C" const char *Quote_GetApiVersion();
extern "C" void Quote_Release(CThostFtdcMdApi *self);
extern "C" void Quote_Init(CThostFtdcMdApi *self);
extern "C" int Quote_Join(CThostFtdcMdApi *self);
extern "C" const char *Quote_GetTradingDay(CThostFtdcMdApi *self);
extern "C" void Quote_RegisterFront(CThostFtdcMdApi *self, char *pszFrontAddress);
extern "C" void Quote_RegisterNameServer(CThostFtdcMdApi *self, char *pszNsAddress);
extern "C" void Quote_RegisterFensUserInfo(CThostFtdcMdApi *self, CThostFtdcFensUserInfoField *pFensUserInfo);
extern "C" void Quote_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi *pSpi);
extern "C" int Quote_SubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
extern "C" int Quote_UnSubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
extern "C" int Quote_SubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
extern "C" int Quote_UnSubscribeForQuoteRsp(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
extern "C" int Quote_ReqUserLogin(CThostFtdcMdApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID);
extern "C" int Quote_ReqUserLogout(CThostFtdcMdApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID);
extern "C" int Quote_ReqQryMulticastInstrument(CThostFtdcMdApi *self, CThostFtdcQryMulticastInstrumentField *pQryMulticastInstrument, int nRequestID);

/////////////Trader Api////////////////
extern "C" CThostFtdcTraderApi *CreateFtdcTraderApi(const char *pszFlowPath);
extern "C" const char *Trade_GetApiVersion();
extern "C" void Trade_Release(CThostFtdcTraderApi *self);
extern "C" void Trade_Init(CThostFtdcTraderApi *self);
extern "C" int Trade_Join(CThostFtdcTraderApi *self);
extern "C" const char *Trade_GetTradingDay(CThostFtdcTraderApi *self);
extern "C" void Trade_RegisterFront(CThostFtdcTraderApi *self, char *pszFrontAddress);
extern "C" void Trade_RegisterNameServer(CThostFtdcTraderApi *self, char *pszNsAddress);
extern "C" void Trade_RegisterFensUserInfo(CThostFtdcTraderApi *self, CThostFtdcFensUserInfoField *pFensUserInfo);
extern "C" void Trade_RegisterSpi(CThostFtdcTraderApi *self, CThostFtdcTraderSpi *pSpi);
extern "C" void Trade_SubscribePrivateTopic(CThostFtdcTraderApi *self, THOST_TE_RESUME_TYPE nResumeType);
extern "C" void Trade_SubscribePublicTopic(CThostFtdcTraderApi *self, THOST_TE_RESUME_TYPE nResumeType);
extern "C" int Trade_ReqAuthenticate(CThostFtdcTraderApi *self, CThostFtdcReqAuthenticateField *pReqAuthenticateField, int nRequestID);
extern "C" int Trade_RegisterUserSystemInfo(CThostFtdcTraderApi *self, CThostFtdcUserSystemInfoField *pUserSystemInfo);
extern "C" int Trade_SubmitUserSystemInfo(CThostFtdcTraderApi *self, CThostFtdcUserSystemInfoField *pUserSystemInfo);
extern "C" int Trade_ReqUserLogin(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID);
extern "C" int Trade_ReqUserLogout(CThostFtdcTraderApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID);
extern "C" int Trade_ReqUserPasswordUpdate(CThostFtdcTraderApi *self, CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate, int nRequestID);
extern "C" int Trade_ReqTradingAccountPasswordUpdate(CThostFtdcTraderApi *self, CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate, int nRequestID);
extern "C" int Trade_ReqUserAuthMethod(CThostFtdcTraderApi *self, CThostFtdcReqUserAuthMethodField *pReqUserAuthMethod, int nRequestID);
extern "C" int Trade_ReqGenUserCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqGenUserCaptchaField *pReqGenUserCaptcha, int nRequestID);
extern "C" int Trade_ReqGenUserText(CThostFtdcTraderApi *self, CThostFtdcReqGenUserTextField *pReqGenUserText, int nRequestID);
extern "C" int Trade_ReqUserLoginWithCaptcha(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithCaptchaField *pReqUserLoginWithCaptcha, int nRequestID);
extern "C" int Trade_ReqUserLoginWithText(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithTextField *pReqUserLoginWithText, int nRequestID);
extern "C" int Trade_ReqUserLoginWithOTP(CThostFtdcTraderApi *self, CThostFtdcReqUserLoginWithOTPField *pReqUserLoginWithOTP, int nRequestID);
extern "C" int Trade_ReqOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputOrderField *pInputOrder, int nRequestID);
extern "C" int Trade_ReqParkedOrderInsert(CThostFtdcTraderApi *self, CThostFtdcParkedOrderField *pParkedOrder, int nRequestID);
extern "C" int Trade_ReqParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcParkedOrderActionField *pParkedOrderAction, int nRequestID);
extern "C" int Trade_ReqOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputOrderActionField *pInputOrderAction, int nRequestID);
extern "C" int Trade_ReqQryMaxOrderVolume(CThostFtdcTraderApi *self, CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, int nRequestID);
extern "C" int Trade_ReqSettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, int nRequestID);
extern "C" int Trade_ReqRemoveParkedOrder(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, int nRequestID);
extern "C" int Trade_ReqRemoveParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, int nRequestID);
extern "C" int Trade_ReqExecOrderInsert(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderField *pInputExecOrder, int nRequestID);
extern "C" int Trade_ReqExecOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputExecOrderActionField *pInputExecOrderAction, int nRequestID);
extern "C" int Trade_ReqForQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputForQuoteField *pInputForQuote, int nRequestID);
extern "C" int Trade_ReqQuoteInsert(CThostFtdcTraderApi *self, CThostFtdcInputQuoteField *pInputQuote, int nRequestID);
extern "C" int Trade_ReqQuoteAction(CThostFtdcTraderApi *self, CThostFtdcInputQuoteActionField *pInputQuoteAction, int nRequestID);
extern "C" int Trade_ReqBatchOrderAction(CThostFtdcTraderApi *self, CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, int nRequestID);
extern "C" int Trade_ReqOptionSelfCloseInsert(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, int nRequestID);
extern "C" int Trade_ReqOptionSelfCloseAction(CThostFtdcTraderApi *self, CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, int nRequestID);
extern "C" int Trade_ReqCombActionInsert(CThostFtdcTraderApi *self, CThostFtdcInputCombActionField *pInputCombAction, int nRequestID);
extern "C" int Trade_ReqQryOrder(CThostFtdcTraderApi *self, CThostFtdcQryOrderField *pQryOrder, int nRequestID);
extern "C" int Trade_ReqQryTrade(CThostFtdcTraderApi *self, CThostFtdcQryTradeField *pQryTrade, int nRequestID);
extern "C" int Trade_ReqQryInvestorPosition(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionField *pQryInvestorPosition, int nRequestID);
extern "C" int Trade_ReqQryTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID);
extern "C" int Trade_ReqQryInvestor(CThostFtdcTraderApi *self, CThostFtdcQryInvestorField *pQryInvestor, int nRequestID);
extern "C" int Trade_ReqQryTradingCode(CThostFtdcTraderApi *self, CThostFtdcQryTradingCodeField *pQryTradingCode, int nRequestID);
extern "C" int Trade_ReqQryInstrumentMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentMarginRateField *pQryInstrumentMarginRate, int nRequestID);
extern "C" int Trade_ReqQryInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentCommissionRateField *pQryInstrumentCommissionRate, int nRequestID);
extern "C" int Trade_ReqQryExchange(CThostFtdcTraderApi *self, CThostFtdcQryExchangeField *pQryExchange, int nRequestID);
extern "C" int Trade_ReqQryProduct(CThostFtdcTraderApi *self, CThostFtdcQryProductField *pQryProduct, int nRequestID);
extern "C" int Trade_ReqQryInstrument(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentField *pQryInstrument, int nRequestID);
extern "C" int Trade_ReqQryDepthMarketData(CThostFtdcTraderApi *self, CThostFtdcQryDepthMarketDataField *pQryDepthMarketData, int nRequestID);
extern "C" int Trade_ReqQrySettlementInfo(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoField *pQrySettlementInfo, int nRequestID);
extern "C" int Trade_ReqQryTransferBank(CThostFtdcTraderApi *self, CThostFtdcQryTransferBankField *pQryTransferBank, int nRequestID);
extern "C" int Trade_ReqQryInvestorPositionDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionDetailField *pQryInvestorPositionDetail, int nRequestID);
extern "C" int Trade_ReqQryNotice(CThostFtdcTraderApi *self, CThostFtdcQryNoticeField *pQryNotice, int nRequestID);
extern "C" int Trade_ReqQrySettlementInfoConfirm(CThostFtdcTraderApi *self, CThostFtdcQrySettlementInfoConfirmField *pQrySettlementInfoConfirm, int nRequestID);
extern "C" int Trade_ReqQryInvestorPositionCombineDetail(CThostFtdcTraderApi *self, CThostFtdcQryInvestorPositionCombineDetailField *pQryInvestorPositionCombineDetail, int nRequestID);
extern "C" int Trade_ReqQryCFMMCTradingAccountKey(CThostFtdcTraderApi *self, CThostFtdcQryCFMMCTradingAccountKeyField *pQryCFMMCTradingAccountKey, int nRequestID);
extern "C" int Trade_ReqQryEWarrantOffset(CThostFtdcTraderApi *self, CThostFtdcQryEWarrantOffsetField *pQryEWarrantOffset, int nRequestID);
extern "C" int Trade_ReqQryInvestorProductGroupMargin(CThostFtdcTraderApi *self, CThostFtdcQryInvestorProductGroupMarginField *pQryInvestorProductGroupMargin, int nRequestID);
extern "C" int Trade_ReqQryExchangeMarginRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateField *pQryExchangeMarginRate, int nRequestID);
extern "C" int Trade_ReqQryExchangeMarginRateAdjust(CThostFtdcTraderApi *self, CThostFtdcQryExchangeMarginRateAdjustField *pQryExchangeMarginRateAdjust, int nRequestID);
extern "C" int Trade_ReqQryExchangeRate(CThostFtdcTraderApi *self, CThostFtdcQryExchangeRateField *pQryExchangeRate, int nRequestID);
extern "C" int Trade_ReqQrySecAgentACIDMap(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentACIDMapField *pQrySecAgentACIDMap, int nRequestID);
extern "C" int Trade_ReqQryProductExchRate(CThostFtdcTraderApi *self, CThostFtdcQryProductExchRateField *pQryProductExchRate, int nRequestID);
extern "C" int Trade_ReqQryProductGroup(CThostFtdcTraderApi *self, CThostFtdcQryProductGroupField *pQryProductGroup, int nRequestID);
extern "C" int Trade_ReqQryMMInstrumentCommissionRate(CThostFtdcTraderApi *self, CThostFtdcQryMMInstrumentCommissionRateField *pQryMMInstrumentCommissionRate, int nRequestID);
extern "C" int Trade_ReqQryMMOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryMMOptionInstrCommRateField *pQryMMOptionInstrCommRate, int nRequestID);
extern "C" int Trade_ReqQryInstrumentOrderCommRate(CThostFtdcTraderApi *self, CThostFtdcQryInstrumentOrderCommRateField *pQryInstrumentOrderCommRate, int nRequestID);
extern "C" int Trade_ReqQrySecAgentTradingAccount(CThostFtdcTraderApi *self, CThostFtdcQryTradingAccountField *pQryTradingAccount, int nRequestID);
extern "C" int Trade_ReqQrySecAgentCheckMode(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentCheckModeField *pQrySecAgentCheckMode, int nRequestID);
extern "C" int Trade_ReqQrySecAgentTradeInfo(CThostFtdcTraderApi *self, CThostFtdcQrySecAgentTradeInfoField *pQrySecAgentTradeInfo, int nRequestID);
extern "C" int Trade_ReqQryOptionInstrTradeCost(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrTradeCostField *pQryOptionInstrTradeCost, int nRequestID);
extern "C" int Trade_ReqQryOptionInstrCommRate(CThostFtdcTraderApi *self, CThostFtdcQryOptionInstrCommRateField *pQryOptionInstrCommRate, int nRequestID);
extern "C" int Trade_ReqQryExecOrder(CThostFtdcTraderApi *self, CThostFtdcQryExecOrderField *pQryExecOrder, int nRequestID);
extern "C" int Trade_ReqQryForQuote(CThostFtdcTraderApi *self, CThostFtdcQryForQuoteField *pQryForQuote, int nRequestID);
extern "C" int Trade_ReqQryQuote(CThostFtdcTraderApi *self, CThostFtdcQryQuoteField *pQryQuote, int nRequestID);
extern "C" int Trade_ReqQryOptionSelfClose(CThostFtdcTraderApi *self, CThostFtdcQryOptionSelfCloseField *pQryOptionSelfClose, int nRequestID);
extern "C" int Trade_ReqQryInvestUnit(CThostFtdcTraderApi *self, CThostFtdcQryInvestUnitField *pQryInvestUnit, int nRequestID);
extern "C" int Trade_ReqQryCombInstrumentGuard(CThostFtdcTraderApi *self, CThostFtdcQryCombInstrumentGuardField *pQryCombInstrumentGuard, int nRequestID);
extern "C" int Trade_ReqQryCombAction(CThostFtdcTraderApi *self, CThostFtdcQryCombActionField *pQryCombAction, int nRequestID);
extern "C" int Trade_ReqQryTransferSerial(CThostFtdcTraderApi *self, CThostFtdcQryTransferSerialField *pQryTransferSerial, int nRequestID);
extern "C" int Trade_ReqQryAccountregister(CThostFtdcTraderApi *self, CThostFtdcQryAccountregisterField *pQryAccountregister, int nRequestID);
extern "C" int Trade_ReqQryContractBank(CThostFtdcTraderApi *self, CThostFtdcQryContractBankField *pQryContractBank, int nRequestID);
extern "C" int Trade_ReqQryParkedOrder(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderField *pQryParkedOrder, int nRequestID);
extern "C" int Trade_ReqQryParkedOrderAction(CThostFtdcTraderApi *self, CThostFtdcQryParkedOrderActionField *pQryParkedOrderAction, int nRequestID);
extern "C" int Trade_ReqQryTradingNotice(CThostFtdcTraderApi *self, CThostFtdcQryTradingNoticeField *pQryTradingNotice, int nRequestID);
extern "C" int Trade_ReqQryBrokerTradingParams(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingParamsField *pQryBrokerTradingParams, int nRequestID);
extern "C" int Trade_ReqQryBrokerTradingAlgos(CThostFtdcTraderApi *self, CThostFtdcQryBrokerTradingAlgosField *pQryBrokerTradingAlgos, int nRequestID);
extern "C" int Trade_ReqQueryCFMMCTradingAccountToken(CThostFtdcTraderApi *self, CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, int nRequestID);
extern "C" int Trade_ReqFromBankToFutureByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID);
extern "C" int Trade_ReqFromFutureToBankByFuture(CThostFtdcTraderApi *self, CThostFtdcReqTransferField *pReqTransfer, int nRequestID);
extern "C" int Trade_ReqQueryBankAccountMoneyByFuture(CThostFtdcTraderApi *self, CThostFtdcReqQueryAccountField *pReqQueryAccount, int nRequestID);
extern "C" int Trade_ReqQryClassifiedInstrument(CThostFtdcTraderApi *self, CThostFtdcQryClassifiedInstrumentField *pQryClassifiedInstrument, int nRequestID);
extern "C" int Trade_ReqQryCombPromotionParam(CThostFtdcTraderApi *self, CThostFtdcQryCombPromotionParamField *pQryCombPromotionParam, int nRequestID);
extern "C" int Trade_ReqQryRiskSettleInvstPosition(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleInvstPositionField *pQryRiskSettleInvstPosition, int nRequestID);
extern "C" int Trade_ReqQryRiskSettleProductStatus(CThostFtdcTraderApi *self, CThostFtdcQryRiskSettleProductStatusField *pQryRiskSettleProductStatus, int nRequestID);

class QuoteSpiStub : CThostFtdcMdSpi
{
public:
    void *rust_object;
    QuoteSpiStub(void *rust_object);
    ~QuoteSpiStub();
    void OnFrontConnected();
    void OnFrontDisconnected(int nReason);
    void OnHeartBeatWarning(int nTimeLapse);
    void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField *pMulticastInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData);
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp);
};

extern "C" void QuoteSpiStub_Destructor(QuoteSpiStub *stub) { delete stub; }
extern "C" void QuoteSpiStub_Rust_OnFrontConnected(void *rust_object);
extern "C" void QuoteSpiStub_Rust_OnFrontDisconnected(void *rust_object, int nReason);
extern "C" void QuoteSpiStub_Rust_OnHeartBeatWarning(void *rust_object, int nTimeLapse);
extern "C" void QuoteSpiStub_Rust_OnRspUserLogin(void *rust_object, CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspUserLogout(void *rust_object, CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspQryMulticastInstrument(void *rust_object, CThostFtdcMulticastInstrumentField *pMulticastInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspError(void *rust_object, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspSubMarketData(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspUnSubMarketData(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspSubForQuoteRsp(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRspUnSubForQuoteRsp(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpiStub_Rust_OnRtnDepthMarketData(void *rust_object, CThostFtdcDepthMarketDataField *pDepthMarketData);
extern "C" void QuoteSpiStub_Rust_OnRtnForQuoteRsp(void *rust_object, CThostFtdcForQuoteRspField *pForQuoteRsp);
extern "C" void QuoteSpiStub_Rust_Destructor(void *rust_object);

class TradeSpiStub : CThostFtdcTraderSpi
{
public:
    void *rust_object;
    TradeSpiStub(void *rust_object);
    ~TradeSpiStub();
    ///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    void OnFrontConnected();

    ///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
    ///@param nReason 错误原因
    ///        0x1001 网络读失败
    ///        0x1002 网络写失败
    ///        0x2001 接收心跳超时
    ///        0x2002 发送心跳失败
    ///        0x2003 收到错误报文
    void OnFrontDisconnected(int nReason);

    ///心跳超时警告。当长时间未收到报文时，该方法被调用。
    ///@param nTimeLapse 距离上次接收报文的时间
    void OnHeartBeatWarning(int nTimeLapse);

    ///客户端认证响应
    void OnRspAuthenticate(CThostFtdcRspAuthenticateField *pRspAuthenticateField, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///登录请求响应
    void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///登出请求响应
    void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///用户口令更新请求响应
    void OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///资金账户口令更新请求响应
    void OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///查询用户当前支持的认证模式的回复
    void OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField *pRspUserAuthMethod, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///获取图形验证码请求的回复
    void OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField *pRspGenUserCaptcha, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///获取短信验证码请求的回复
    void OnRspGenUserText(CThostFtdcRspGenUserTextField *pRspGenUserText, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///报单录入请求响应
    void OnRspOrderInsert(CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///预埋单录入请求响应
    void OnRspParkedOrderInsert(CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///预埋撤单录入请求响应
    void OnRspParkedOrderAction(CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///报单操作请求响应
    void OnRspOrderAction(CThostFtdcInputOrderActionField *pInputOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///查询最大报单数量响应
    void OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///投资者结算结果确认响应
    void OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///删除预埋单响应
    void OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///删除预埋撤单响应
    void OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///执行宣告录入请求响应
    void OnRspExecOrderInsert(CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///执行宣告操作请求响应
    void OnRspExecOrderAction(CThostFtdcInputExecOrderActionField *pInputExecOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///询价录入请求响应
    void OnRspForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///报价录入请求响应
    void OnRspQuoteInsert(CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///报价操作请求响应
    void OnRspQuoteAction(CThostFtdcInputQuoteActionField *pInputQuoteAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///批量报单操作请求响应
    void OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///期权自对冲录入请求响应
    void OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///期权自对冲操作请求响应
    void OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///申请组合录入请求响应
    void OnRspCombActionInsert(CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询报单响应
    void OnRspQryOrder(CThostFtdcOrderField *pOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询成交响应
    void OnRspQryTrade(CThostFtdcTradeField *pTrade, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者持仓响应
    void OnRspQryInvestorPosition(CThostFtdcInvestorPositionField *pInvestorPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询资金账户响应
    void OnRspQryTradingAccount(CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者响应
    void OnRspQryInvestor(CThostFtdcInvestorField *pInvestor, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询交易编码响应
    void OnRspQryTradingCode(CThostFtdcTradingCodeField *pTradingCode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询合约保证金率响应
    void OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField *pInstrumentMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询合约手续费率响应
    void OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField *pInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询交易所响应
    void OnRspQryExchange(CThostFtdcExchangeField *pExchange, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询产品响应
    void OnRspQryProduct(CThostFtdcProductField *pProduct, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询合约响应
    void OnRspQryInstrument(CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询行情响应
    void OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者结算结果响应
    void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField *pSettlementInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询转帐银行响应
    void OnRspQryTransferBank(CThostFtdcTransferBankField *pTransferBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者持仓明细响应
    void OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField *pInvestorPositionDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询客户通知响应
    void OnRspQryNotice(CThostFtdcNoticeField *pNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询结算信息确认响应
    void OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者持仓明细响应
    void OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField *pInvestorPositionCombineDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///查询保证金监管系统经纪公司资金账户密钥响应
    void OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField *pCFMMCTradingAccountKey, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询仓单折抵信息响应
    void OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField *pEWarrantOffset, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资者品种/跨品种保证金响应
    void OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField *pInvestorProductGroupMargin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询交易所保证金率响应
    void OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField *pExchangeMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询交易所调整保证金率响应
    void OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField *pExchangeMarginRateAdjust, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询汇率响应
    void OnRspQryExchangeRate(CThostFtdcExchangeRateField *pExchangeRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询二级代理操作员银期权限响应
    void OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField *pSecAgentACIDMap, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询产品报价汇率
    void OnRspQryProductExchRate(CThostFtdcProductExchRateField *pProductExchRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询产品组
    void OnRspQryProductGroup(CThostFtdcProductGroupField *pProductGroup, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询做市商合约手续费率响应
    void OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField *pMMInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询做市商期权合约手续费响应
    void OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField *pMMOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询报单手续费响应
    void OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField *pInstrumentOrderCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询资金账户响应
    void OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询二级代理商资金校验模式响应
    void OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField *pSecAgentCheckMode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询二级代理商信息响应
    void OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField *pSecAgentTradeInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询期权交易成本响应
    void OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField *pOptionInstrTradeCost, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询期权合约手续费响应
    void OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField *pOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询执行宣告响应
    void OnRspQryExecOrder(CThostFtdcExecOrderField *pExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询询价响应
    void OnRspQryForQuote(CThostFtdcForQuoteField *pForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询报价响应
    void OnRspQryQuote(CThostFtdcQuoteField *pQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询期权自对冲响应
    void OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField *pOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询投资单元响应
    void OnRspQryInvestUnit(CThostFtdcInvestUnitField *pInvestUnit, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询组合合约安全系数响应
    void OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField *pCombInstrumentGuard, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询申请组合响应
    void OnRspQryCombAction(CThostFtdcCombActionField *pCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询转帐流水响应
    void OnRspQryTransferSerial(CThostFtdcTransferSerialField *pTransferSerial, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询银期签约关系响应
    void OnRspQryAccountregister(CThostFtdcAccountregisterField *pAccountregister, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///错误应答
    void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///报单通知
    void OnRtnOrder(CThostFtdcOrderField *pOrder);

    ///成交通知
    void OnRtnTrade(CThostFtdcTradeField *pTrade);

    ///报单录入错误回报
    void OnErrRtnOrderInsert(CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo);

    ///报单操作错误回报
    void OnErrRtnOrderAction(CThostFtdcOrderActionField *pOrderAction, CThostFtdcRspInfoField *pRspInfo);

    ///合约交易状态通知
    void OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField *pInstrumentStatus);

    ///交易所公告通知
    void OnRtnBulletin(CThostFtdcBulletinField *pBulletin);

    ///交易通知
    void OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField *pTradingNoticeInfo);

    ///提示条件单校验错误
    void OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField *pErrorConditionalOrder);

    ///执行宣告通知
    void OnRtnExecOrder(CThostFtdcExecOrderField *pExecOrder);

    ///执行宣告录入错误回报
    void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo);

    ///执行宣告操作错误回报
    void OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField *pExecOrderAction, CThostFtdcRspInfoField *pRspInfo);

    ///询价录入错误回报
    void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo);

    ///报价通知
    void OnRtnQuote(CThostFtdcQuoteField *pQuote);

    ///报价录入错误回报
    void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo);

    ///报价操作错误回报
    void OnErrRtnQuoteAction(CThostFtdcQuoteActionField *pQuoteAction, CThostFtdcRspInfoField *pRspInfo);

    ///询价通知
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp);

    ///保证金监控中心用户令牌
    void OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField *pCFMMCTradingAccountToken);

    ///批量报单操作错误回报
    void OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField *pBatchOrderAction, CThostFtdcRspInfoField *pRspInfo);

    ///期权自对冲通知
    void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField *pOptionSelfClose);

    ///期权自对冲录入错误回报
    void OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo);

    ///期权自对冲操作错误回报
    void OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField *pOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo);

    ///申请组合通知
    void OnRtnCombAction(CThostFtdcCombActionField *pCombAction);

    ///申请组合录入错误回报
    void OnErrRtnCombActionInsert(CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo);

    ///请求查询签约银行响应
    void OnRspQryContractBank(CThostFtdcContractBankField *pContractBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询预埋单响应
    void OnRspQryParkedOrder(CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询预埋撤单响应
    void OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询交易通知响应
    void OnRspQryTradingNotice(CThostFtdcTradingNoticeField *pTradingNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询经纪公司交易参数响应
    void OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField *pBrokerTradingParams, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询经纪公司交易算法响应
    void OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField *pBrokerTradingAlgos, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求查询监控中心用户令牌
    void OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///银行发起银行资金转期货通知
    void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField *pRspTransfer);

    ///银行发起期货资金转银行通知
    void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField *pRspTransfer);

    ///银行发起冲正银行转期货通知
    void OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField *pRspRepeal);

    ///银行发起冲正期货转银行通知
    void OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField *pRspRepeal);

    ///期货发起银行资金转期货通知
    void OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField *pRspTransfer);

    ///期货发起期货资金转银行通知
    void OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField *pRspTransfer);

    ///系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField *pRspRepeal);

    ///系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField *pRspRepeal);

    ///期货发起查询银行余额通知
    void OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField *pNotifyQueryAccount);

    ///期货发起银行资金转期货错误回报
    void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo);

    ///期货发起期货资金转银行错误回报
    void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo);

    ///系统运行时期货端手工发起冲正银行转期货错误回报
    void OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo);

    ///系统运行时期货端手工发起冲正期货转银行错误回报
    void OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo);

    ///期货发起查询银行余额错误回报
    void OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo);

    ///期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField *pRspRepeal);

    ///期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField *pRspRepeal);

    ///期货发起银行资金转期货应答
    void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///期货发起期货资金转银行应答
    void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///期货发起查询银行余额应答
    void OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///银行发起银期开户通知
    void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField *pOpenAccount);

    ///银行发起银期销户通知
    void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField *pCancelAccount);

    ///银行发起变更银行账号通知
    void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField *pChangeAccount);

    ///请求查询分类合约响应
    void OnRspQryClassifiedInstrument(CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///请求组合优惠比例响应
    void OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField *pCombPromotionParam, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///投资者风险结算持仓查询响应
    void OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField *pRiskSettleInvstPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    ///风险结算产品查询响应
    void OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField *pRiskSettleProductStatus, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
};

extern "C" void TradeSpiStub_Destructor(TradeSpiStub *stub) { delete stub; }
///当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
extern "C" void TradeSpiStub_Rust_OnFrontConnected(void *rust_object);

///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
///@param nReason 错误原因
///        0x1001 网络读失败
///        0x1002 网络写失败
///        0x2001 接收心跳超时
///        0x2002 发送心跳失败
///        0x2003 收到错误报文
extern "C" void TradeSpiStub_Rust_OnFrontDisconnected(void *rust_object, int nReason);

///心跳超时警告。当长时间未收到报文时，该方法被调用。
///@param nTimeLapse 距离上次接收报文的时间
extern "C" void TradeSpiStub_Rust_OnHeartBeatWarning(void *rust_object, int nTimeLapse);

///客户端认证响应
extern "C" void TradeSpiStub_Rust_OnRspAuthenticate(void *rust_object, CThostFtdcRspAuthenticateField *pRspAuthenticateField, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///登录请求响应
extern "C" void TradeSpiStub_Rust_OnRspUserLogin(void *rust_object, CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///登出请求响应
extern "C" void TradeSpiStub_Rust_OnRspUserLogout(void *rust_object, CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///用户口令更新请求响应
extern "C" void TradeSpiStub_Rust_OnRspUserPasswordUpdate(void *rust_object, CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///资金账户口令更新请求响应
extern "C" void TradeSpiStub_Rust_OnRspTradingAccountPasswordUpdate(void *rust_object, CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///查询用户当前支持的认证模式的回复
extern "C" void TradeSpiStub_Rust_OnRspUserAuthMethod(void *rust_object, CThostFtdcRspUserAuthMethodField *pRspUserAuthMethod, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///获取图形验证码请求的回复
extern "C" void TradeSpiStub_Rust_OnRspGenUserCaptcha(void *rust_object, CThostFtdcRspGenUserCaptchaField *pRspGenUserCaptcha, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///获取短信验证码请求的回复
extern "C" void TradeSpiStub_Rust_OnRspGenUserText(void *rust_object, CThostFtdcRspGenUserTextField *pRspGenUserText, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///报单录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspOrderInsert(void *rust_object, CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///预埋单录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspParkedOrderInsert(void *rust_object, CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///预埋撤单录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspParkedOrderAction(void *rust_object, CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///报单操作请求响应
extern "C" void TradeSpiStub_Rust_OnRspOrderAction(void *rust_object, CThostFtdcInputOrderActionField *pInputOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///查询最大报单数量响应
extern "C" void TradeSpiStub_Rust_OnRspQryMaxOrderVolume(void *rust_object, CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///投资者结算结果确认响应
extern "C" void TradeSpiStub_Rust_OnRspSettlementInfoConfirm(void *rust_object, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///删除预埋单响应
extern "C" void TradeSpiStub_Rust_OnRspRemoveParkedOrder(void *rust_object, CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///删除预埋撤单响应
extern "C" void TradeSpiStub_Rust_OnRspRemoveParkedOrderAction(void *rust_object, CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///执行宣告录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspExecOrderInsert(void *rust_object, CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///执行宣告操作请求响应
extern "C" void TradeSpiStub_Rust_OnRspExecOrderAction(void *rust_object, CThostFtdcInputExecOrderActionField *pInputExecOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///询价录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspForQuoteInsert(void *rust_object, CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///报价录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspQuoteInsert(void *rust_object, CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///报价操作请求响应
extern "C" void TradeSpiStub_Rust_OnRspQuoteAction(void *rust_object, CThostFtdcInputQuoteActionField *pInputQuoteAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///批量报单操作请求响应
extern "C" void TradeSpiStub_Rust_OnRspBatchOrderAction(void *rust_object, CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///期权自对冲录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspOptionSelfCloseInsert(void *rust_object, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///期权自对冲操作请求响应
extern "C" void TradeSpiStub_Rust_OnRspOptionSelfCloseAction(void *rust_object, CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///申请组合录入请求响应
extern "C" void TradeSpiStub_Rust_OnRspCombActionInsert(void *rust_object, CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询报单响应
extern "C" void TradeSpiStub_Rust_OnRspQryOrder(void *rust_object, CThostFtdcOrderField *pOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询成交响应
extern "C" void TradeSpiStub_Rust_OnRspQryTrade(void *rust_object, CThostFtdcTradeField *pTrade, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者持仓响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestorPosition(void *rust_object, CThostFtdcInvestorPositionField *pInvestorPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询资金账户响应
extern "C" void TradeSpiStub_Rust_OnRspQryTradingAccount(void *rust_object, CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestor(void *rust_object, CThostFtdcInvestorField *pInvestor, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询交易编码响应
extern "C" void TradeSpiStub_Rust_OnRspQryTradingCode(void *rust_object, CThostFtdcTradingCodeField *pTradingCode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询合约保证金率响应
extern "C" void TradeSpiStub_Rust_OnRspQryInstrumentMarginRate(void *rust_object, CThostFtdcInstrumentMarginRateField *pInstrumentMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询合约手续费率响应
extern "C" void TradeSpiStub_Rust_OnRspQryInstrumentCommissionRate(void *rust_object, CThostFtdcInstrumentCommissionRateField *pInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询交易所响应
extern "C" void TradeSpiStub_Rust_OnRspQryExchange(void *rust_object, CThostFtdcExchangeField *pExchange, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询产品响应
extern "C" void TradeSpiStub_Rust_OnRspQryProduct(void *rust_object, CThostFtdcProductField *pProduct, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询合约响应
extern "C" void TradeSpiStub_Rust_OnRspQryInstrument(void *rust_object, CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询行情响应
extern "C" void TradeSpiStub_Rust_OnRspQryDepthMarketData(void *rust_object, CThostFtdcDepthMarketDataField *pDepthMarketData, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者结算结果响应
extern "C" void TradeSpiStub_Rust_OnRspQrySettlementInfo(void *rust_object, CThostFtdcSettlementInfoField *pSettlementInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询转帐银行响应
extern "C" void TradeSpiStub_Rust_OnRspQryTransferBank(void *rust_object, CThostFtdcTransferBankField *pTransferBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者持仓明细响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestorPositionDetail(void *rust_object, CThostFtdcInvestorPositionDetailField *pInvestorPositionDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询客户通知响应
extern "C" void TradeSpiStub_Rust_OnRspQryNotice(void *rust_object, CThostFtdcNoticeField *pNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询结算信息确认响应
extern "C" void TradeSpiStub_Rust_OnRspQrySettlementInfoConfirm(void *rust_object, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者持仓明细响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestorPositionCombineDetail(void *rust_object, CThostFtdcInvestorPositionCombineDetailField *pInvestorPositionCombineDetail, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///查询保证金监管系统经纪公司资金账户密钥响应
extern "C" void TradeSpiStub_Rust_OnRspQryCFMMCTradingAccountKey(void *rust_object, CThostFtdcCFMMCTradingAccountKeyField *pCFMMCTradingAccountKey, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询仓单折抵信息响应
extern "C" void TradeSpiStub_Rust_OnRspQryEWarrantOffset(void *rust_object, CThostFtdcEWarrantOffsetField *pEWarrantOffset, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资者品种/跨品种保证金响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestorProductGroupMargin(void *rust_object, CThostFtdcInvestorProductGroupMarginField *pInvestorProductGroupMargin, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询交易所保证金率响应
extern "C" void TradeSpiStub_Rust_OnRspQryExchangeMarginRate(void *rust_object, CThostFtdcExchangeMarginRateField *pExchangeMarginRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询交易所调整保证金率响应
extern "C" void TradeSpiStub_Rust_OnRspQryExchangeMarginRateAdjust(void *rust_object, CThostFtdcExchangeMarginRateAdjustField *pExchangeMarginRateAdjust, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询汇率响应
extern "C" void TradeSpiStub_Rust_OnRspQryExchangeRate(void *rust_object, CThostFtdcExchangeRateField *pExchangeRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询二级代理操作员银期权限响应
extern "C" void TradeSpiStub_Rust_OnRspQrySecAgentACIDMap(void *rust_object, CThostFtdcSecAgentACIDMapField *pSecAgentACIDMap, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询产品报价汇率
extern "C" void TradeSpiStub_Rust_OnRspQryProductExchRate(void *rust_object, CThostFtdcProductExchRateField *pProductExchRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询产品组
extern "C" void TradeSpiStub_Rust_OnRspQryProductGroup(void *rust_object, CThostFtdcProductGroupField *pProductGroup, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询做市商合约手续费率响应
extern "C" void TradeSpiStub_Rust_OnRspQryMMInstrumentCommissionRate(void *rust_object, CThostFtdcMMInstrumentCommissionRateField *pMMInstrumentCommissionRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询做市商期权合约手续费响应
extern "C" void TradeSpiStub_Rust_OnRspQryMMOptionInstrCommRate(void *rust_object, CThostFtdcMMOptionInstrCommRateField *pMMOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询报单手续费响应
extern "C" void TradeSpiStub_Rust_OnRspQryInstrumentOrderCommRate(void *rust_object, CThostFtdcInstrumentOrderCommRateField *pInstrumentOrderCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询资金账户响应
extern "C" void TradeSpiStub_Rust_OnRspQrySecAgentTradingAccount(void *rust_object, CThostFtdcTradingAccountField *pTradingAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询二级代理商资金校验模式响应
extern "C" void TradeSpiStub_Rust_OnRspQrySecAgentCheckMode(void *rust_object, CThostFtdcSecAgentCheckModeField *pSecAgentCheckMode, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询二级代理商信息响应
extern "C" void TradeSpiStub_Rust_OnRspQrySecAgentTradeInfo(void *rust_object, CThostFtdcSecAgentTradeInfoField *pSecAgentTradeInfo, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询期权交易成本响应
extern "C" void TradeSpiStub_Rust_OnRspQryOptionInstrTradeCost(void *rust_object, CThostFtdcOptionInstrTradeCostField *pOptionInstrTradeCost, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询期权合约手续费响应
extern "C" void TradeSpiStub_Rust_OnRspQryOptionInstrCommRate(void *rust_object, CThostFtdcOptionInstrCommRateField *pOptionInstrCommRate, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询执行宣告响应
extern "C" void TradeSpiStub_Rust_OnRspQryExecOrder(void *rust_object, CThostFtdcExecOrderField *pExecOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询询价响应
extern "C" void TradeSpiStub_Rust_OnRspQryForQuote(void *rust_object, CThostFtdcForQuoteField *pForQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询报价响应
extern "C" void TradeSpiStub_Rust_OnRspQryQuote(void *rust_object, CThostFtdcQuoteField *pQuote, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询期权自对冲响应
extern "C" void TradeSpiStub_Rust_OnRspQryOptionSelfClose(void *rust_object, CThostFtdcOptionSelfCloseField *pOptionSelfClose, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询投资单元响应
extern "C" void TradeSpiStub_Rust_OnRspQryInvestUnit(void *rust_object, CThostFtdcInvestUnitField *pInvestUnit, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询组合合约安全系数响应
extern "C" void TradeSpiStub_Rust_OnRspQryCombInstrumentGuard(void *rust_object, CThostFtdcCombInstrumentGuardField *pCombInstrumentGuard, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询申请组合响应
extern "C" void TradeSpiStub_Rust_OnRspQryCombAction(void *rust_object, CThostFtdcCombActionField *pCombAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询转帐流水响应
extern "C" void TradeSpiStub_Rust_OnRspQryTransferSerial(void *rust_object, CThostFtdcTransferSerialField *pTransferSerial, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询银期签约关系响应
extern "C" void TradeSpiStub_Rust_OnRspQryAccountregister(void *rust_object, CThostFtdcAccountregisterField *pAccountregister, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///错误应答
extern "C" void TradeSpiStub_Rust_OnRspError(void *rust_object, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///报单通知
extern "C" void TradeSpiStub_Rust_OnRtnOrder(void *rust_object, CThostFtdcOrderField *pOrder);

///成交通知
extern "C" void TradeSpiStub_Rust_OnRtnTrade(void *rust_object, CThostFtdcTradeField *pTrade);

///报单录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnOrderInsert(void *rust_object, CThostFtdcInputOrderField *pInputOrder, CThostFtdcRspInfoField *pRspInfo);

///报单操作错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnOrderAction(void *rust_object, CThostFtdcOrderActionField *pOrderAction, CThostFtdcRspInfoField *pRspInfo);

///合约交易状态通知
extern "C" void TradeSpiStub_Rust_OnRtnInstrumentStatus(void *rust_object, CThostFtdcInstrumentStatusField *pInstrumentStatus);

///交易所公告通知
extern "C" void TradeSpiStub_Rust_OnRtnBulletin(void *rust_object, CThostFtdcBulletinField *pBulletin);

///交易通知
extern "C" void TradeSpiStub_Rust_OnRtnTradingNotice(void *rust_object, CThostFtdcTradingNoticeInfoField *pTradingNoticeInfo);

///提示条件单校验错误
extern "C" void TradeSpiStub_Rust_OnRtnErrorConditionalOrder(void *rust_object, CThostFtdcErrorConditionalOrderField *pErrorConditionalOrder);

///执行宣告通知
extern "C" void TradeSpiStub_Rust_OnRtnExecOrder(void *rust_object, CThostFtdcExecOrderField *pExecOrder);

///执行宣告录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnExecOrderInsert(void *rust_object, CThostFtdcInputExecOrderField *pInputExecOrder, CThostFtdcRspInfoField *pRspInfo);

///执行宣告操作错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnExecOrderAction(void *rust_object, CThostFtdcExecOrderActionField *pExecOrderAction, CThostFtdcRspInfoField *pRspInfo);

///询价录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnForQuoteInsert(void *rust_object, CThostFtdcInputForQuoteField *pInputForQuote, CThostFtdcRspInfoField *pRspInfo);

///报价通知
extern "C" void TradeSpiStub_Rust_OnRtnQuote(void *rust_object, CThostFtdcQuoteField *pQuote);

///报价录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnQuoteInsert(void *rust_object, CThostFtdcInputQuoteField *pInputQuote, CThostFtdcRspInfoField *pRspInfo);

///报价操作错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnQuoteAction(void *rust_object, CThostFtdcQuoteActionField *pQuoteAction, CThostFtdcRspInfoField *pRspInfo);

///询价通知
extern "C" void TradeSpiStub_Rust_OnRtnForQuoteRsp(void *rust_object, CThostFtdcForQuoteRspField *pForQuoteRsp);

///保证金监控中心用户令牌
extern "C" void TradeSpiStub_Rust_OnRtnCFMMCTradingAccountToken(void *rust_object, CThostFtdcCFMMCTradingAccountTokenField *pCFMMCTradingAccountToken);

///批量报单操作错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnBatchOrderAction(void *rust_object, CThostFtdcBatchOrderActionField *pBatchOrderAction, CThostFtdcRspInfoField *pRspInfo);

///期权自对冲通知
extern "C" void TradeSpiStub_Rust_OnRtnOptionSelfClose(void *rust_object, CThostFtdcOptionSelfCloseField *pOptionSelfClose);

///期权自对冲录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnOptionSelfCloseInsert(void *rust_object, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose, CThostFtdcRspInfoField *pRspInfo);

///期权自对冲操作错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnOptionSelfCloseAction(void *rust_object, CThostFtdcOptionSelfCloseActionField *pOptionSelfCloseAction, CThostFtdcRspInfoField *pRspInfo);

///申请组合通知
extern "C" void TradeSpiStub_Rust_OnRtnCombAction(void *rust_object, CThostFtdcCombActionField *pCombAction);

///申请组合录入错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnCombActionInsert(void *rust_object, CThostFtdcInputCombActionField *pInputCombAction, CThostFtdcRspInfoField *pRspInfo);

///请求查询签约银行响应
extern "C" void TradeSpiStub_Rust_OnRspQryContractBank(void *rust_object, CThostFtdcContractBankField *pContractBank, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询预埋单响应
extern "C" void TradeSpiStub_Rust_OnRspQryParkedOrder(void *rust_object, CThostFtdcParkedOrderField *pParkedOrder, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询预埋撤单响应
extern "C" void TradeSpiStub_Rust_OnRspQryParkedOrderAction(void *rust_object, CThostFtdcParkedOrderActionField *pParkedOrderAction, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询交易通知响应
extern "C" void TradeSpiStub_Rust_OnRspQryTradingNotice(void *rust_object, CThostFtdcTradingNoticeField *pTradingNotice, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询经纪公司交易参数响应
extern "C" void TradeSpiStub_Rust_OnRspQryBrokerTradingParams(void *rust_object, CThostFtdcBrokerTradingParamsField *pBrokerTradingParams, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询经纪公司交易算法响应
extern "C" void TradeSpiStub_Rust_OnRspQryBrokerTradingAlgos(void *rust_object, CThostFtdcBrokerTradingAlgosField *pBrokerTradingAlgos, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求查询监控中心用户令牌
extern "C" void TradeSpiStub_Rust_OnRspQueryCFMMCTradingAccountToken(void *rust_object, CThostFtdcQueryCFMMCTradingAccountTokenField *pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///银行发起银行资金转期货通知
extern "C" void TradeSpiStub_Rust_OnRtnFromBankToFutureByBank(void *rust_object, CThostFtdcRspTransferField *pRspTransfer);

///银行发起期货资金转银行通知
extern "C" void TradeSpiStub_Rust_OnRtnFromFutureToBankByBank(void *rust_object, CThostFtdcRspTransferField *pRspTransfer);

///银行发起冲正银行转期货通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByBank(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///银行发起冲正期货转银行通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByBank(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///期货发起银行资金转期货通知
extern "C" void TradeSpiStub_Rust_OnRtnFromBankToFutureByFuture(void *rust_object, CThostFtdcRspTransferField *pRspTransfer);

///期货发起期货资金转银行通知
extern "C" void TradeSpiStub_Rust_OnRtnFromFutureToBankByFuture(void *rust_object, CThostFtdcRspTransferField *pRspTransfer);

///系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFutureManual(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFutureManual(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///期货发起查询银行余额通知
extern "C" void TradeSpiStub_Rust_OnRtnQueryBankBalanceByFuture(void *rust_object, CThostFtdcNotifyQueryAccountField *pNotifyQueryAccount);

///期货发起银行资金转期货错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnBankToFutureByFuture(void *rust_object, CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo);

///期货发起期货资金转银行错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnFutureToBankByFuture(void *rust_object, CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo);

///系统运行时期货端手工发起冲正银行转期货错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnRepealBankToFutureByFutureManual(void *rust_object, CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo);

///系统运行时期货端手工发起冲正期货转银行错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnRepealFutureToBankByFutureManual(void *rust_object, CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo);

///期货发起查询银行余额错误回报
extern "C" void TradeSpiStub_Rust_OnErrRtnQueryBankBalanceByFuture(void *rust_object, CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo);

///期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromBankToFutureByFuture(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
extern "C" void TradeSpiStub_Rust_OnRtnRepealFromFutureToBankByFuture(void *rust_object, CThostFtdcRspRepealField *pRspRepeal);

///期货发起银行资金转期货应答
extern "C" void TradeSpiStub_Rust_OnRspFromBankToFutureByFuture(void *rust_object, CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///期货发起期货资金转银行应答
extern "C" void TradeSpiStub_Rust_OnRspFromFutureToBankByFuture(void *rust_object, CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///期货发起查询银行余额应答
extern "C" void TradeSpiStub_Rust_OnRspQueryBankAccountMoneyByFuture(void *rust_object, CThostFtdcReqQueryAccountField *pReqQueryAccount, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///银行发起银期开户通知
extern "C" void TradeSpiStub_Rust_OnRtnOpenAccountByBank(void *rust_object, CThostFtdcOpenAccountField *pOpenAccount);

///银行发起银期销户通知
extern "C" void TradeSpiStub_Rust_OnRtnCancelAccountByBank(void *rust_object, CThostFtdcCancelAccountField *pCancelAccount);

///银行发起变更银行账号通知
extern "C" void TradeSpiStub_Rust_OnRtnChangeAccountByBank(void *rust_object, CThostFtdcChangeAccountField *pChangeAccount);

///请求查询分类合约响应
extern "C" void TradeSpiStub_Rust_OnRspQryClassifiedInstrument(void *rust_object, CThostFtdcInstrumentField *pInstrument, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///请求组合优惠比例响应
extern "C" void TradeSpiStub_Rust_OnRspQryCombPromotionParam(void *rust_object, CThostFtdcCombPromotionParamField *pCombPromotionParam, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///投资者风险结算持仓查询响应
extern "C" void TradeSpiStub_Rust_OnRspQryRiskSettleInvstPosition(void *rust_object, CThostFtdcRiskSettleInvstPositionField *pRiskSettleInvstPosition, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

///风险结算产品查询响应
extern "C" void TradeSpiStub_Rust_OnRspQryRiskSettleProductStatus(void *rust_object, CThostFtdcRiskSettleProductStatusField *pRiskSettleProductStatus, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

extern "C" void TradeSpiStub_Rust_Destructor(void *rust_object);

#endif