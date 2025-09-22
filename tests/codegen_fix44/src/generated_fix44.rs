// Generated automatically by MiniFixRust 0.1.0 on Mon, 22 Sep 2025 15:46:38 +0000.
//
// DO NOT MODIFY MANUALLY.
// DO NOT COMMIT TO VERSION CONTROL.
// ALL CHANGES WILL BE OVERWRITTEN.

// https://www.onixs.biz/fix-dictionary/4.4/index.html

use minifix::dict::{FieldLocation, FixDatatype};
use minifix::definitions::HardCodedFixFieldDefinition;
use minifix::FieldType;

/// Field type variants for [`UnsolicitedIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum UnsolicitedIndicator {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`TradeReportRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeReportRejectReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Successful,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidPartyInformation,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    UnknownInstrument,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UnauthorizedToReportTrades,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidTradeType,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`CollAsgnTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollAsgnTransType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Replace,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Cancel,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Release,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Reverse,
}

/// Field type variants for [`TradeRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeRequestType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    AllTrades,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MatchedTradesMatchingCriteriaProvidedOnRequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    UnmatchedTradesThatMatchCriteria,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UnreportedTradesThatMatchCriteria,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    AdvisoriesThatMatchCriteria,
}

/// Field type variants for [`PosMaintStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosMaintStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AcceptedWithWarnings,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Rejected,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Completed,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CompletedWithWarnings,
}

/// Field type variants for [`GapFillFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum GapFillFlag {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`ResponseTransportType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ResponseTransportType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    InbandTransportTheRequestWasSentOver,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    OutOfBandPreArrangedOutOfBandDeliveryMechanism,
}

/// Field type variants for [`LocateReqd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LocateReqd {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`Scope`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum Scope {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Local,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    National,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Global,
}

/// Field type variants for [`MdEntryType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MdEntryType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Bid,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Offer,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Trade,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    IndexValue,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    OpeningPrice,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ClosingPrice,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    SettlementPrice,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    TradingSessionHighPrice,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    TradingSessionLowPrice,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    TradingSessionVwapPrice,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Imbalance,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    TradeVolume,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    OpenInterest,
}

/// Field type variants for [`TradSesStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradSesStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Unknown,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Halted,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Open,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Closed,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    PreOpen,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PreClose,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    RequestRejected,
}

/// Field type variants for [`CrossType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CrossType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CrossTradeWhichIsExecutedCompletelyOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSideNoteTheCrossprioritzation,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInTheCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCrossIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
}

/// Field type variants for [`TargetStrategy`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TargetStrategy {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Vwap,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Participate,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MininizeMarketImpact,
}

/// Field type variants for [`OrdRejReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OrdRejReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Broker,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownSymbol,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ExchangeClosed,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    OrderExceedsLimit,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    TooLateToEnter,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    UnknownOrder,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    DuplicateOrder,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    DuplicateOfAVerballyCommunicatedOrder,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    StaleOrder,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    TradeAlongRequired,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    InvalidInvestorId,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    UnsupportedOrderCharacteristic12SurveillenceOption,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    IncorrectQuantity,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    IncorrectAllocatedQuantity,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    UnknownAccount,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`SecurityTradingStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityTradingStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    OpeningDelay,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TradingHalt,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Resume,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    NoOpenNoResume,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PriceIndication,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    TradingRangeIndication,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    MarketImbalanceBuy,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    MarketImbalanceSell,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    MarketOnCloseImbalanceBuy,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    MarketOnCloseImbalanceSell,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    NoMarketImbalance,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    NoMarketOnCloseImbalance,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    ItsPreOpening,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    NewPriceIndication,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    TradeDisseminationTime,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    ReadyToTrade,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    NotAvailableForTrading,
    /// Field variant '19'.
    #[minifix(variant = "19")]
    NotTradedOnThisMarket,
    /// Field variant '20'.
    #[minifix(variant = "20")]
    UnknownOrInvalid,
    /// Field variant '21'.
    #[minifix(variant = "21")]
    PreOpen,
    /// Field variant '22'.
    #[minifix(variant = "22")]
    OpeningRotation,
    /// Field variant '23'.
    #[minifix(variant = "23")]
    FastMarket,
}

/// Field type variants for [`CollAsgnRespType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollAsgnRespType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Received,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Accepted,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Declined,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Rejected,
}

/// Field type variants for [`CollInquiryQualifier`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollInquiryQualifier {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Tradedate,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    GcInstrument,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Collateralinstrument,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    SubstitutionEligible,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    NotAssigned,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PartiallyAssigned,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    FullyAssigned,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    OutstandingTrades,
}

/// Field type variants for [`QuoteStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CanceledForSymbol,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CanceledForSecurityType,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CanceledForUnderlying,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CanceledAll,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Rejected,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    RemovedFromMarket,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Expired,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Query,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    QuoteNotFound,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    Pending,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    Pass,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    LockedMarketWarning,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    CrossMarketWarning,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    CanceledDueToLockMarket,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    CanceledDueToCrossMarket,
}

/// Field type variants for [`CpProgram`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CpProgram {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    _3,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    _4,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`ConfirmTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ConfirmTransType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Replace,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Cancel,
}

/// Field type variants for [`Urgency`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum Urgency {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Normal,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Flash,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Background,
}

/// Field type variants for [`AffirmStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AffirmStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Received,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ConfirmRejectedIeNotAffirmed,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Affirmed,
}

/// Field type variants for [`PutOrCall`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PutOrCall {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Put,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Call,
}

/// Field type variants for [`MoneyLaunderingStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MoneyLaunderingStatus {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Passed,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    NotChecked,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExemptBelowTheLimit,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ExemptClientMoneyTypeExemption,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ExemptAuthorisedCreditOrFinancialInstitution,
}

/// Field type variants for [`PegScope`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PegScope {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Local,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    National,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Global,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    NationalExcludingLocal,
}

/// Field type variants for [`CollAsgnRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollAsgnRejectReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    UnknownDeal,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownOrInvalidInstrument,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    UnauthorizedTransaction,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InsufficientCollateral,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidTypeOfCollateral,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ExcessiveSubstitution,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`AvgPxIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AvgPxIndicator {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NoAveragePricing,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    TradeIsPartOfAnAveragePriceGroupIdentifiedByTheTradelinkid,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    LastTradeInTheAveragePriceGroupIdentifiedByTheTradelinkid,
}

/// Field type variants for [`QuoteCancelType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteCancelType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CancelForSymbol,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CancelForSecurityType,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CancelForUnderlyingSymbol,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CancelAllQuotes,
}

/// Field type variants for [`RoundingDirection`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum RoundingDirection {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    RoundToNearest,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    RoundDown,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    RoundUp,
}

/// Field type variants for [`LastFragment`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LastFragment {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`PartyIdSource`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PartyIdSource {
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Bic,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    GenerallyAcceptedMarketParticipantIdentifier,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    ProprietaryCustomCode,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    IsoCountryCode,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    SettlementEntityLocation,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Mic,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    CsdParticipantMemberCode,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    KoreanInvestorId,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TaiwaneseQualifiedForeignInvestorIdQfii,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    TaiwaneseTradingAccount,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    MalaysianCentralDepository,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ChineseBShare,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    UkNationalInsuranceOrPensionNumber,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    UsSocialSecurityNumber,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    UsEmployerIdentificationNumber,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    AustralianBusinessNumber,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    AustralianTaxFileNumber,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument,
}

/// Field type variants for [`CorporateAction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CorporateAction {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    ExDividend,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    ExDistribution,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    ExRights,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    New,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    ExInterest,
}

/// Field type variants for [`ContAmtType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ContAmtType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CommissionAmount,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Commission,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InitialChargeAmount,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InitialCharge,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    DiscountAmount,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Discount,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    DilutionLevyAmount,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    DilutionLevy,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    ExitChargeAmount,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    ExitCharge,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    FundBasedRenewalCommission,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    ProjectedFundValue,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    FundBasedRenewalCommissionAmount13,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    FundBasedRenewalCommissionAmount14,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    NetSettlementAmount,
}

/// Field type variants for [`BookingUnit`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BookingUnit {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    EachPartialExecutionIsABookableUnit,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AggregateExecutionsForThisSymbolSideAndSettlementDate,
}

/// Field type variants for [`PosReqResult`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosReqResult {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ValidRequest,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidOrUnsupportedRequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NoPositionsFoundThatMatchCriteria,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NotAuthorizedToRequestPositions,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    RequestForPositionNotSupported,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`ClearingInstruction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ClearingInstruction {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ProcessNormally,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExcludeFromAllNetting,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    BilateralNettingOnly,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ExClearing,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SpecialTrade,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    MultilateralNetting,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    ClearAgainstCentralCounterparty,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    ExcludeFromCentralCounterparty,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    ManualMode,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    AutomaticPostingMode,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    AutomaticGiveUpMode,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    QualifiedServiceRepresentative,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    CustomerTrade,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    SelfClearing,
}

/// Field type variants for [`PosMaintResult`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosMaintResult {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    SuccessfulCompletion,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Rejected,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`NoSides`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum NoSides {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    OneSide,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    BothSides,
}

/// Field type variants for [`MessageEncoding`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MessageEncoding {
    /// Field variant 'ISO-2022-JP'.
    #[minifix(variant = "ISO-2022-JP")]
    Jis,
    /// Field variant 'EUC-JP'.
    #[minifix(variant = "EUC-JP")]
    Euc,
    /// Field variant 'Shift_JIS'.
    #[minifix(variant = "Shift_JIS")]
    ForUsingSjis,
    /// Field variant 'UTF-8'.
    #[minifix(variant = "UTF-8")]
    Unicode,
}

/// Field type variants for [`MdImplicitDelete`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MdImplicitDelete {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`ConfirmType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ConfirmType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Status,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Confirmation,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ConfirmationRequestRejected,
}

/// Field type variants for [`DeliveryType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DeliveryType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    VersusPaymentDeliver,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    FreeDeliver,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TriParty,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    HoldInCustody,
}

/// Field type variants for [`TrdType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TrdType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    RegularTrade,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BlockTrade,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Efp,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Transfer,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    LateTrade,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    TTrade,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    WeightedAveragePriceTrade,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    BunchedTrade,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    LateBunchedTrade,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    PriorReferencePriceTrade,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    AfterHoursTrade,
}

/// Field type variants for [`HaltReasonChar`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum HaltReasonChar {
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    OrderImbalance,
    /// Field variant 'X'.
    #[minifix(variant = "X")]
    EquipmentChangeover,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    NewsPending,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    NewsDissemination,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    OrderInflux,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    AdditionalInformation,
}

/// Field type variants for [`AllocReportType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocReportType {
    /// Field variant '3'.
    #[minifix(variant = "3")]
    SellsideCalculatedUsingPreliminary,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SellsideCalculatedWithoutPreliminary,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    WarehouseRecap,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    RequestToIntermediary,
}

/// Field type variants for [`DeleteReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DeleteReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Cancelation,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Error,
}

/// Field type variants for [`AdjustmentType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AdjustmentType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ProcessRequestAsMarginDisposition,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DeltaPlus,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    DeltaMinus,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Final,
}

/// Field type variants for [`ConfirmRejReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ConfirmRejReason {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MismatchedAccount,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MissingSettlementInstructions,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`PossResend`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PossResend {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`MiscFeeBasis`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MiscFeeBasis {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Absolute,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    PerUnit,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Percentage,
}

/// Field type variants for [`TradeRequestResult`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeRequestResult {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Successful,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidOrUnknownInstrument,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InvalidTypeOfTradeRequested,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InvalidParties,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidTransportTypeRequested,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvalidDestinationRequested,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    TraderequesttypeNotSupported,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    UnauthorizedForTradeCaptureReportRequest,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`CommType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CommType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    PerUnit,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Percentage,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Absolute,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    _4,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    _5,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PointsPerBondOrContractSupplyContractmultiplier,
}

/// Field type variants for [`AdvSide`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AdvSide {
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Buy,
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Sell,
    /// Field variant 'X'.
    #[minifix(variant = "X")]
    Cross,
    /// Field variant 'T'.
    #[minifix(variant = "T")]
    Trade,
}

/// Field type variants for [`PreallocMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PreallocMethod {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ProRata,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DoNotProRataDiscussFirst,
}

/// Field type variants for [`InstrAttribType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum InstrAttribType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Flat,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ZeroCoupon,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InterestBearing,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    NoPeriodicPayments,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    VariableRate,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    LessFeeForPut,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    SteppedCoupon,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    CouponPeriod,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    WhenAndIfIssued,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    OriginalIssueDiscount,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    CallablePuttable,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    EscrowedToMaturity,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrattribvalue,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    Prerefunded,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    InDefault,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    Unrated,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    Taxable,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    Indexed,
    /// Field variant '19'.
    #[minifix(variant = "19")]
    SubjectToAlternativeMinimumTax,
    /// Field variant '20'.
    #[minifix(variant = "20")]
    OriginalIssueDiscountPriceSupplyPriceInTheInstrattribvalue,
    /// Field variant '21'.
    #[minifix(variant = "21")]
    CallableBelowMaturityValue,
    /// Field variant '22'.
    #[minifix(variant = "22")]
    CallableWithoutNoticeByMailToHolderUnlessRegistered,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrattribvalue,
}

/// Field type variants for [`SettlType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Regular,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Cash,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NextDay,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    TPlus2,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    TPlus3,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    TPlus4,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Future,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    WhenAndIfIssued,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    SellersOption,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    TPlus5,
}

/// Field type variants for [`FundRenewWaiv`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum FundRenewWaiv {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`OpenCloseSettlFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OpenCloseSettlFlag {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    DailyOpen,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SessionOpen,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    DeliverySettlementEntry,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ExpectedEntry,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    EntryFromPreviousBusinessDay,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    TheoreticalPriceValue,
}

/// Field type variants for [`Side`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum Side {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Buy,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Sell,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    BuyMinus,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SellPlus,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    SellShort,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    SellShortExempt,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Undisclosed,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Cross,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    CrossShort,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    CrossShortExempt,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    AsDefined,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Opposite,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Subscribe,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    Redeem,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    Lend,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Borrow,
}

/// Field type variants for [`NotifyBrokerOfCredit`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum NotifyBrokerOfCredit {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`TradeReportType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeReportType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Submit,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Alleged,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Accept,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Decline,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Addendum,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    NoWas,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    TradeReportCancel,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    LockedInTradeBreak,
}

/// Field type variants for [`CoveredOrUncovered`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CoveredOrUncovered {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Covered,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Uncovered,
}

/// Field type variants for [`LegSwapType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LegSwapType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ParForPar,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ModifiedDuration,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Risk,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Proceeds,
}

/// Field type variants for [`AllocLinkType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocLinkType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    FXNetting,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    FXSwap,
}

/// Field type variants for [`PegOffsetType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PegOffsetType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Price,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BasisPoints,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Ticks,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PriceTier,
}

/// Field type variants for [`BidDescriptorType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BidDescriptorType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Sector,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Country,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Index,
}

/// Field type variants for [`ApplQueueAction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ApplQueueAction {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NoActionTaken,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    QueueFlushed,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OverlayLast,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    EndSession,
}

/// Field type variants for [`GtBookingInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum GtBookingInst {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    BookOutAllTradesOnDayOfExecution,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AccumulateExecutionsUntilOrderIsFilledOrExpires,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AccumulateUntilVerballyNotifiedOtherwise,
}

/// Field type variants for [`MultiLegRptTypeReq`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MultiLegRptTypeReq {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ReportByMulitlegSecurityOnly,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ReportByMultilegSecurityAndByInstrumentLegsBelongingToTheMultilegSecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ReportByInstrumentLegsBelongingToTheMultilegSecurityOnly,
}

/// Field type variants for [`ResetSeqNumFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ResetSeqNumFlag {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`PosReqType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosReqType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Positions,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Trades,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Exercises,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Assignments,
}

/// Field type variants for [`PaymentMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PaymentMethod {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Crest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Nscc,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Euroclear,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Clearstream,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Cheque,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    TelegraphicTransfer,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Fedwire,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    DebitCard,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    DirectDebit,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    DirectCredit,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    CreditCard,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    AchDebit,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    AchCredit,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    Bpay,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    HighValueClearingSystem,
}

/// Field type variants for [`AllocSettlInstType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocSettlInstType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    UseDefaultInstructions,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DeriveFromParametersProvided,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    FullDetailsProvided,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    SsiDbIdsProvided,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    PhoneForInstructions,
}

/// Field type variants for [`ProgRptReqs`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ProgRptReqs {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BuysideExplicitlyRequestsStatusUsingStatusrequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    RealTimeExecutionReports,
}

/// Field type variants for [`DiscretionScope`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionScope {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Local,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    National,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Global,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    NationalExcludingLocal,
}

/// Field type variants for [`StandInstDbType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum StandInstDbType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Other,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DtcSid,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ThomsonAlert,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    AGlobalCustodian,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Accountnet,
}

/// Field type variants for [`SecurityRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityRequestType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    RequestSecurityIdentityAndSpecifications,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    RequestSecurityIdentityForTheSpecificationsProvided,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    RequestListSecurityTypes,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    RequestListSecurities,
}

/// Field type variants for [`SideMultiLegReportingType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SideMultiLegReportingType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SingleSecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    IndividualLegOfAMultiLegSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MultiLegSecurity,
}

/// Field type variants for [`QuoteType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Indicative,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Tradeable,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    RestrictedTradeable,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Counter,
}

/// Field type variants for [`SettlInstMode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlInstMode {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    StandingInstructionsProvided,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SpecificOrderForASingleAccount,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RequestReject,
}

/// Field type variants for [`ExerciseMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExerciseMethod {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Automatic,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    Manual,
}

/// Field type variants for [`AllocIntermedReqType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocIntermedReqType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    PendingAccept,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    PendingRelease,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PendingReversal,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Accept,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    BlockLevelReject,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    AccountLevelReject,
}

/// Field type variants for [`MultiLegReportingType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MultiLegReportingType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SingleSecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    IndividualLegOfAMultiLegSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MultiLegSecurity,
}

/// Field type variants for [`RegistTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum RegistTransType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Replace,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Cancel,
}

/// Field type variants for [`DeliveryForm`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DeliveryForm {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Bookentry,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Bearer,
}

/// Field type variants for [`OwnershipType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OwnershipType {
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    JointInvestors,
    /// Field variant 'T'.
    #[minifix(variant = "T")]
    TenantsInCommon,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    JointTrustees,
}

/// Field type variants for [`AllocTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocTransType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Replace,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Cancel,
}

/// Field type variants for [`ReportToExch`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ReportToExch {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`MdUpdateType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MdUpdateType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    FullRefresh,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    IncrementalRefresh,
}

/// Field type variants for [`IoiQualifier`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IoiQualifier {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    AllOrNone,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    MarketOnClose,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    AtTheClose,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Vwap,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    InTouchWith,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    Limit,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    MoreBehind,
    /// Field variant 'O'.
    #[minifix(variant = "O")]
    AtTheOpen,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    TakingAPosition,
    /// Field variant 'Q'.
    #[minifix(variant = "Q")]
    AtTheMarket,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    ReadyToTrade,
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    PortfolioShown,
    /// Field variant 'T'.
    #[minifix(variant = "T")]
    ThroughTheDay,
    /// Field variant 'V'.
    #[minifix(variant = "V")]
    Versus,
    /// Field variant 'W'.
    #[minifix(variant = "W")]
    Indication,
    /// Field variant 'X'.
    #[minifix(variant = "X")]
    CrossingOpportunity,
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    AtTheMidpoint,
    /// Field variant 'Z'.
    #[minifix(variant = "Z")]
    PreOpen,
}

/// Field type variants for [`DiscretionLimitType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionLimitType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    OrBetter,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    StrictLimitIsAStrictLimit,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OrWorseForABuyTheDiscretionPriceIsAMinimumAndForASellTheDiscretionPriceIsAMaximum,
}

/// Field type variants for [`LastCapacity`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LastCapacity {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Agent,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CrossAsAgent,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CrossAsPrincipal,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Principal,
}

/// Field type variants for [`BusinessRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BusinessRejectReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Other,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnkownId,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    UnknownSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UnsupportedMessageType,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    ApplicationNotAvailable,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ConditionallyRequiredFieldMissing,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    NotAuthorized,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    DelivertoFirmNotAvailableAtThisTime,
}

/// Field type variants for [`ConfirmStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ConfirmStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Received,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MismatchedAccount,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MissingSettlementInstructions,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Confirmed,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RequestRejected,
}

/// Field type variants for [`CollInquiryStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollInquiryStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AcceptedWithWarnings,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Completed,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CompletedWithWarnings,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Rejected,
}

/// Field type variants for [`ClearingFeeIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ClearingFeeIndicator {
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    CboeMember,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    NonMemberAndCustomer,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    EquityMemberAndClearingMember,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    FullAndAssociateMemberTradingForOwnAccountAndAsFloorBrokers,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    _106hAnd106jFirms,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    GimIdemAndComMembershipInterestHolders,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    LesseeAnd106fEmployees,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    AllOtherOwnershipTypes,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    _1stYearDelegateTradingForHisOwnAccount,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    _2ndYearDelegateTradingForHisOwnAccount,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    _3rdYearDelegateTradingForHisOwnAccount,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    _4thYearDelegateTradingForHisOwnAccount,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    _5thYearDelegateTradingForHisOwnAccount,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    _6thYearAndBeyondDelegateTradingForHisOwnAccount,
}

/// Field type variants for [`DlvyInstType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DlvyInstType {
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Securities,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Cash,
}

/// Field type variants for [`PossDupFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PossDupFlag {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`AggregatedBook`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AggregatedBook {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`PublishTrdIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PublishTrdIndicator {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`TestMessageIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TestMessageIndicator {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`CollInquiryResult`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollInquiryResult {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Successful,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidOrUnknownInstrument,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InvalidOrUnknownCollateralType,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InvalidParties,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidTransportTypeRequested,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvalidDestinationRequested,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    NoCollateralFoundForTheTradeSpecified,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    NoCollateralFoundForTheOrderSpecified,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    CollateralInquiryTypeNotSupported,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    UnauthorizedForCollateralInquiry,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`DayBookingInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DayBookingInst {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    CanTriggerBookingWithoutReferenceToTheOrderInitiator,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SpeakWithOrderInitiatorBeforeBooking,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Accumulate,
}

/// Field type variants for [`AllocCancReplaceReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocCancReplaceReason {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    OriginalDetailsIncompleteIncorrect,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ChangeInUnderlyingOrderDetails,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`PriceType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PriceType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Percentage,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    PerUnit,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    FixedAmount,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    DiscountPercentagePointsBelowPar,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PremiumPercentagePointsOverPar,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Spread,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    TedPrice,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    TedYield,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Yield,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    FixedCabinetTradePrice,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    VariableCabinetTradePrice,
}

/// Field type variants for [`SecurityListRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityListRequestType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Symbol,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SecuritytypeAndOrCficode,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Product,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Tradingsessionid,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    AllSecurities,
}

/// Field type variants for [`SecurityResponseType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityResponseType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AcceptSecurityProposalAsIs,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RejectSecurityProposal,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    CanNotMatchSelectionCriteria,
}

/// Field type variants for [`BidType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BidType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    NonDisclosedStyle,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    DisclosedStyle,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NoBiddingProcess,
}

/// Field type variants for [`MassCancelRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MassCancelRequestType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CancelOrdersForASecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CancelOrdersForAnUnderlyingSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CancelOrdersForAProduct,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CancelOrdersForACficode,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    CancelOrdersForASecuritytype,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    CancelOrdersForATradingSession,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    CancelAllOrders,
}

/// Field type variants for [`SettlInstSource`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlInstSource {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BrokersInstructions,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InstitutionsInstructions,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Investor,
}

/// Field type variants for [`DiscretionInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionInst {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    RelatedToDisplayedPrice,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    RelatedToMarketPrice,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    RelatedToPrimaryPrice,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    RelatedToLocalPrimaryPrice,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    RelatedToMidpointPrice,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RelatedToLastTradePrice,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    RelatedToVwap,
}

/// Field type variants for [`AllocStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BlockLevelReject,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AccountLevelReject,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Received,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Incomplete,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RejectedByIntermediary,
}

/// Field type variants for [`MatchType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MatchType {
    /// Field variant 'A1'.
    #[minifix(variant = "A1")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
    /// Field variant 'A2'.
    #[minifix(variant = "A2")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
    /// Field variant 'A3'.
    #[minifix(variant = "A3")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
    /// Field variant 'A4'.
    #[minifix(variant = "A4")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
    /// Field variant 'A5'.
    #[minifix(variant = "A5")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
    /// Field variant 'AQ'.
    #[minifix(variant = "AQ")]
    ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
    /// Field variant 'S1'.
    #[minifix(variant = "S1")]
    SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
    /// Field variant 'S2'.
    #[minifix(variant = "S2")]
    SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
    /// Field variant 'S3'.
    #[minifix(variant = "S3")]
    SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
    /// Field variant 'S4'.
    #[minifix(variant = "S4")]
    SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
    /// Field variant 'S5'.
    #[minifix(variant = "S5")]
    SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
    /// Field variant 'M1'.
    #[minifix(variant = "M1")]
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimesActM1Match,
    /// Field variant 'M2'.
    #[minifix(variant = "M2")]
    SummarizedMatchMinusBadgesAndTimesActM2Match,
    /// Field variant 'MT'.
    #[minifix(variant = "MT")]
    OcsLockedInNonAct,
    /// Field variant 'M3'.
    #[minifix(variant = "M3")]
    ActAcceptedTrade,
    /// Field variant 'M4'.
    #[minifix(variant = "M4")]
    ActDefaultTrade,
    /// Field variant 'M5'.
    #[minifix(variant = "M5")]
    ActDefaultAfterM2,
    /// Field variant 'M6'.
    #[minifix(variant = "M6")]
    ActM6Match,
}

/// Field type variants for [`MdReqRejReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MdReqRejReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    UnknownSymbol,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DuplicateMdreqid,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InsufficientBandwidth,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InsufficientPermissions,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    UnsupportedSubscriptionrequesttype,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    UnsupportedMarketdepth,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    UnsupportedMdupdatetype,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    UnsupportedAggregatedbook,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    UnsupportedMdentrytype,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    UnsupportedTradingsessionid,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    UnsupportedScope,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    UnsupportedOpenclosesettleflag,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    UnsupportedMdimplicitdelete,
}

/// Field type variants for [`IncTaxInd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IncTaxInd {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Net,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Gross,
}

/// Field type variants for [`TradSesMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradSesMethod {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Electronic,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OpenOutcry,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    TwoParty,
}

/// Field type variants for [`TrdRegTimestampType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TrdRegTimestampType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExecutionTime,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TimeIn,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    TimeOut,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    BrokerReceipt,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    BrokerExecution,
}

/// Field type variants for [`UserRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum UserRequestType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Logonuser,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Logoffuser,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Changepasswordforuser,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    RequestIndividualUserStatus,
}

/// Field type variants for [`QuotePriceType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuotePriceType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Percent,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    PerShare,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    FixedAmount,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    DiscountPercentagePointsBelowPar,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PremiumPercentagePointsOverPar,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    BasisPointsRelativeToBenchmark,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    TedPrice,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    TedYield,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    YieldSpread,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    Yield,
}

/// Field type variants for [`SecurityIdSource`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityIdSource {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Cusip,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Sedol,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Quik,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    IsinNumber,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RicCode,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    IsoCurrencyCode,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    IsoCountryCode,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    ExchangeSymbol,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    ConsolidatedTapeAssociation,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    BloombergSymbol,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Wertpapier,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Dutch,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Valoren,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    Sicovam,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    Belgian,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Common,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    ClearingHouse,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    IsdaFpmlProductSpecification,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    OptionsPriceReportingAuthority,
}

/// Field type variants for [`ApplQueueResolution`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ApplQueueResolution {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NoActionTaken,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    QueueFlushed,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OverlayLast,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    EndSession,
}

/// Field type variants for [`QuoteRequestRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteRequestRejectReason {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownSymbol,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Exchange,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    QuoteRequestExceedsLimit,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    TooLateToEnter,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvalidPrice,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    NotAuthorizedToRequestQuote,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    NoMatchForInquiry,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    NoMarketForInstrument,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    NoInventory,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    Pass,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`TimeInForce`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TimeInForce {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Day,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    GoodTillCancel,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AtTheOpening,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ImmediateOrCancel,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    FillOrKill,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    GoodTillCrossing,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    GoodTillDate,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    AtTheClose,
}

/// Field type variants for [`WorkingIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum WorkingIndicator {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`TaxAdvantageType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TaxAdvantageType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NoneNotApplicable,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MaxiIsa,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Tessa,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MiniCashIsa,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    MiniStocksAndSharesIsa,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    MiniInsuranceIsa,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    CurrentYearPayment,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    PriorYearPayment,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    AssetTransfer,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Employee,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    EmployeeCurrentYear,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    Employer,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    EmployerCurrentYear,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    NonFundPrototypeIra,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    NonFundQualifiedPlan,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    DefinedContributionPlan,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    IndividualRetirementAccount,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    IndividualRetirementAccountRollover,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    Keogh,
    /// Field variant '19'.
    #[minifix(variant = "19")]
    ProfitSharingPlan,
    /// Field variant '20'.
    #[minifix(variant = "20")]
    _401k,
    /// Field variant '21'.
    #[minifix(variant = "21")]
    SelfDirectedIra,
    /// Field variant '22'.
    #[minifix(variant = "22")]
    _403,
    /// Field variant '23'.
    #[minifix(variant = "23")]
    _457,
    /// Field variant '24'.
    #[minifix(variant = "24")]
    RothIra24,
    /// Field variant '25'.
    #[minifix(variant = "25")]
    RothIra25,
    /// Field variant '26'.
    #[minifix(variant = "26")]
    RothConversionIra26,
    /// Field variant '27'.
    #[minifix(variant = "27")]
    RothConversionIra27,
    /// Field variant '28'.
    #[minifix(variant = "28")]
    EducationIra28,
    /// Field variant '29'.
    #[minifix(variant = "29")]
    EducationIra29,
}

/// Field type variants for [`QuoteCondition`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteCondition {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Open,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Closed,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    ExchangeBest,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    ConsolidatedBest,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    Locked,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    Crossed,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Depth,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    FastTrading,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    NonFirm,
}

/// Field type variants for [`OrdStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OrdStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    PartiallyFilled,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Filled,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    DoneForDay,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Canceled,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PendingCancel,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Stopped,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Rejected,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Suspended,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    PendingNew,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Calculated,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Expired,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    AcceptedForBidding,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    PendingReplace,
}

/// Field type variants for [`LiquidityIndType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LiquidityIndType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    _5dayMovingAverage,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    _20DayMovingAverage,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NormalMarketSize,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Other,
}

/// Field type variants for [`DiscretionMoveType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionMoveType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Floating,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Fixed,
}

/// Field type variants for [`MassCancelResponse`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MassCancelResponse {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    CancelRequestRejected,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CancelOrdersForASecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    CancelOrdersForAnUnderlyingSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CancelOrdersForAProduct,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CancelOrdersForACficode,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    CancelOrdersForASecuritytype,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    CancelOrdersForATradingSession,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    CancelAllOrders,
}

/// Field type variants for [`Product`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum Product {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Agency,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Commodity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Corporate,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Currency,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Equity,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Government,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Index,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Loan,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Moneymarket,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    Mortgage,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    Municipal,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    Other,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    Financing,
}

/// Field type variants for [`ExecType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExecType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    DoneForDay,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Canceled,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Replace,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PendingCancel,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Stopped,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Rejected,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Suspended,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    PendingNew,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    Calculated,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Expired,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Restated,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    PendingReplace,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    Trade,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    TradeCorrect,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    TradeCancel,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    OrderStatus,
}

/// Field type variants for [`OddLot`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OddLot {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`CustOrderCapacity`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CustOrderCapacity {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MemberTradingForTheirOwnAccount,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ClearingFirmTradingForItsProprietaryAccount,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MemberTradingForAnotherMember,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    AllOther,
}

/// Field type variants for [`CashMargin`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CashMargin {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Cash,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MarginOpen,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MarginClose,
}

/// Field type variants for [`TradSesMode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradSesMode {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Testing,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Simulated,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Production,
}

/// Field type variants for [`DiscretionOffsetType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionOffsetType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Price,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BasisPoints,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Ticks,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PriceTier,
}

/// Field type variants for [`SettlDeliveryType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlDeliveryType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    VersusPaymentDeliver,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    FreeDeliver,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TriParty,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    HoldInCustody,
}

/// Field type variants for [`SecurityType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityType {
    /// Field variant 'FUT'.
    #[minifix(variant = "FUT")]
    Future,
    /// Field variant 'OPT'.
    #[minifix(variant = "OPT")]
    Option,
    /// Field variant 'EUSUPRA'.
    #[minifix(variant = "EUSUPRA")]
    EuroSupranationalCoupons,
    /// Field variant 'FAC'.
    #[minifix(variant = "FAC")]
    FederalAgencyCoupon,
    /// Field variant 'FADN'.
    #[minifix(variant = "FADN")]
    FederalAgencyDiscountNote,
    /// Field variant 'PEF'.
    #[minifix(variant = "PEF")]
    PrivateExportFunding,
    /// Field variant 'SUPRA'.
    #[minifix(variant = "SUPRA")]
    UsdSupranationalCoupons,
    /// Field variant 'CORP'.
    #[minifix(variant = "CORP")]
    CorporateBond,
    /// Field variant 'CPP'.
    #[minifix(variant = "CPP")]
    CorporatePrivatePlacement,
    /// Field variant 'CB'.
    #[minifix(variant = "CB")]
    ConvertibleBond,
    /// Field variant 'DUAL'.
    #[minifix(variant = "DUAL")]
    DualCurrency,
    /// Field variant 'EUCORP'.
    #[minifix(variant = "EUCORP")]
    EuroCorporateBond,
    /// Field variant 'XLINKD'.
    #[minifix(variant = "XLINKD")]
    IndexedLinked,
    /// Field variant 'STRUCT'.
    #[minifix(variant = "STRUCT")]
    StructuredNotes,
    /// Field variant 'YANK'.
    #[minifix(variant = "YANK")]
    YankeeCorporateBond,
    /// Field variant 'FOR'.
    #[minifix(variant = "FOR")]
    ForeignExchangeContract,
    /// Field variant 'CS'.
    #[minifix(variant = "CS")]
    CommonStock,
    /// Field variant 'PS'.
    #[minifix(variant = "PS")]
    PreferredStock,
    /// Field variant 'BRADY'.
    #[minifix(variant = "BRADY")]
    BradyBond,
    /// Field variant 'EUSOV'.
    #[minifix(variant = "EUSOV")]
    EuroSovereigns,
    /// Field variant 'TBOND'.
    #[minifix(variant = "TBOND")]
    UsTreasuryBond,
    /// Field variant 'TINT'.
    #[minifix(variant = "TINT")]
    InterestStripFromAnyBondOrNote,
    /// Field variant 'TIPS'.
    #[minifix(variant = "TIPS")]
    TreasuryInflationProtectedSecurities,
    /// Field variant 'TCAL'.
    #[minifix(variant = "TCAL")]
    PrincipalStripOfACallableBondOrNote,
    /// Field variant 'TPRN'.
    #[minifix(variant = "TPRN")]
    PrincipalStripFromANonCallableBondOrNote,
    /// Field variant 'UST'.
    #[minifix(variant = "UST")]
    UsTreasuryNoteUst,
    /// Field variant 'USTB'.
    #[minifix(variant = "USTB")]
    UsTreasuryBillUstb,
    /// Field variant 'TNOTE'.
    #[minifix(variant = "TNOTE")]
    UsTreasuryNoteTnote,
    /// Field variant 'TBILL'.
    #[minifix(variant = "TBILL")]
    UsTreasuryBillTbill,
    /// Field variant 'REPO'.
    #[minifix(variant = "REPO")]
    Repurchase,
    /// Field variant 'FORWARD'.
    #[minifix(variant = "FORWARD")]
    Forward,
    /// Field variant 'BUYSELL'.
    #[minifix(variant = "BUYSELL")]
    BuySellback,
    /// Field variant 'SECLOAN'.
    #[minifix(variant = "SECLOAN")]
    SecuritiesLoan,
    /// Field variant 'SECPLEDGE'.
    #[minifix(variant = "SECPLEDGE")]
    SecuritiesPledge,
    /// Field variant 'TERM'.
    #[minifix(variant = "TERM")]
    TermLoan,
    /// Field variant 'RVLV'.
    #[minifix(variant = "RVLV")]
    RevolverLoan,
    /// Field variant 'RVLVTRM'.
    #[minifix(variant = "RVLVTRM")]
    RevolverTermLoan,
    /// Field variant 'BRIDGE'.
    #[minifix(variant = "BRIDGE")]
    BridgeLoan,
    /// Field variant 'LOFC'.
    #[minifix(variant = "LOFC")]
    LetterOfCredit,
    /// Field variant 'SWING'.
    #[minifix(variant = "SWING")]
    SwingLineFacility,
    /// Field variant 'DINP'.
    #[minifix(variant = "DINP")]
    DebtorInPossession,
    /// Field variant 'DEFLTED'.
    #[minifix(variant = "DEFLTED")]
    Defaulted,
    /// Field variant 'WITHDRN'.
    #[minifix(variant = "WITHDRN")]
    Withdrawn,
    /// Field variant 'REPLACD'.
    #[minifix(variant = "REPLACD")]
    Replaced,
    /// Field variant 'MATURED'.
    #[minifix(variant = "MATURED")]
    Matured,
    /// Field variant 'AMENDED'.
    #[minifix(variant = "AMENDED")]
    AmendedRestated,
    /// Field variant 'RETIRED'.
    #[minifix(variant = "RETIRED")]
    Retired,
    /// Field variant 'BA'.
    #[minifix(variant = "BA")]
    BankersAcceptance,
    /// Field variant 'BN'.
    #[minifix(variant = "BN")]
    BankNotes,
    /// Field variant 'BOX'.
    #[minifix(variant = "BOX")]
    BillOfExchanges,
    /// Field variant 'CD'.
    #[minifix(variant = "CD")]
    CertificateOfDeposit,
    /// Field variant 'CL'.
    #[minifix(variant = "CL")]
    CallLoans,
    /// Field variant 'CP'.
    #[minifix(variant = "CP")]
    CommercialPaper,
    /// Field variant 'DN'.
    #[minifix(variant = "DN")]
    DepositNotes,
    /// Field variant 'EUCD'.
    #[minifix(variant = "EUCD")]
    EuroCertificateOfDeposit,
    /// Field variant 'EUCP'.
    #[minifix(variant = "EUCP")]
    EuroCommercialPaper,
    /// Field variant 'LQN'.
    #[minifix(variant = "LQN")]
    LiquidityNote,
    /// Field variant 'MTN'.
    #[minifix(variant = "MTN")]
    MediumTermNotes,
    /// Field variant 'ONITE'.
    #[minifix(variant = "ONITE")]
    Overnight,
    /// Field variant 'PN'.
    #[minifix(variant = "PN")]
    PromissoryNote,
    /// Field variant 'PZFJ'.
    #[minifix(variant = "PZFJ")]
    PlazosFijos,
    /// Field variant 'STN'.
    #[minifix(variant = "STN")]
    ShortTermLoanNote,
    /// Field variant 'TD'.
    #[minifix(variant = "TD")]
    TimeDeposit,
    /// Field variant 'XCN'.
    #[minifix(variant = "XCN")]
    ExtendedCommNote,
    /// Field variant 'YCD'.
    #[minifix(variant = "YCD")]
    YankeeCertificateOfDeposit,
    /// Field variant 'ABS'.
    #[minifix(variant = "ABS")]
    AssetBackedSecurities,
    /// Field variant 'CMBS'.
    #[minifix(variant = "CMBS")]
    CorpMortgageBackedSecurities,
    /// Field variant 'CMO'.
    #[minifix(variant = "CMO")]
    CollateralizedMortgageObligation,
    /// Field variant 'IET'.
    #[minifix(variant = "IET")]
    IoetteMortgage,
    /// Field variant 'MBS'.
    #[minifix(variant = "MBS")]
    MortgageBackedSecurities,
    /// Field variant 'MIO'.
    #[minifix(variant = "MIO")]
    MortgageInterestOnly,
    /// Field variant 'MPO'.
    #[minifix(variant = "MPO")]
    MortgagePrincipalOnly,
    /// Field variant 'MPP'.
    #[minifix(variant = "MPP")]
    MortgagePrivatePlacement,
    /// Field variant 'MPT'.
    #[minifix(variant = "MPT")]
    MiscellaneousPassThrough,
    /// Field variant 'PFAND'.
    #[minifix(variant = "PFAND")]
    Pfandbriefe,
    /// Field variant 'TBA'.
    #[minifix(variant = "TBA")]
    ToBeAnnounced,
    /// Field variant 'AN'.
    #[minifix(variant = "AN")]
    OtherAnticipationNotesBanGanEtc,
    /// Field variant 'COFO'.
    #[minifix(variant = "COFO")]
    CertificateOfObligation,
    /// Field variant 'COFP'.
    #[minifix(variant = "COFP")]
    CertificateOfParticipation,
    /// Field variant 'GO'.
    #[minifix(variant = "GO")]
    GeneralObligationBonds,
    /// Field variant 'MT'.
    #[minifix(variant = "MT")]
    MandatoryTender,
    /// Field variant 'RAN'.
    #[minifix(variant = "RAN")]
    RevenueAnticipationNote,
    /// Field variant 'REV'.
    #[minifix(variant = "REV")]
    RevenueBonds,
    /// Field variant 'SPCLA'.
    #[minifix(variant = "SPCLA")]
    SpecialAssessment,
    /// Field variant 'SPCLO'.
    #[minifix(variant = "SPCLO")]
    SpecialObligation,
    /// Field variant 'SPCLT'.
    #[minifix(variant = "SPCLT")]
    SpecialTax,
    /// Field variant 'TAN'.
    #[minifix(variant = "TAN")]
    TaxAnticipationNote,
    /// Field variant 'TAXA'.
    #[minifix(variant = "TAXA")]
    TaxAllocation,
    /// Field variant 'TECP'.
    #[minifix(variant = "TECP")]
    TaxExemptCommercialPaper,
    /// Field variant 'TRAN'.
    #[minifix(variant = "TRAN")]
    TaxRevenueAnticipationNote,
    /// Field variant 'VRDN'.
    #[minifix(variant = "VRDN")]
    VariableRateDemandNote,
    /// Field variant 'WAR'.
    #[minifix(variant = "WAR")]
    Warrant,
    /// Field variant 'MF'.
    #[minifix(variant = "MF")]
    MutualFund,
    /// Field variant 'MLEG'.
    #[minifix(variant = "MLEG")]
    MultiLegInstrument,
    /// Field variant 'NONE'.
    #[minifix(variant = "NONE")]
    NoSecurityType,
}

/// Field type variants for [`EmailType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum EmailType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Reply,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AdminReply,
}

/// Field type variants for [`ProcessCode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ProcessCode {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Regular,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SoftDollar,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    StepIn,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    StepOut,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SoftDollarStepIn,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    SoftDollarStepOut,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PlanSponsor,
}

/// Field type variants for [`TrdRptStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TrdRptStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Rejected,
}

/// Field type variants for [`BidTradeType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BidTradeType {
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    RiskTrade,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    VwapGuarantee,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Agency,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    GuaranteedClose,
}

/// Field type variants for [`TradeCondition`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeCondition {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Cash,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    AveragePriceTrade,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    CashTrade,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    NextDay,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    Opening,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    IntradayTradeDetail,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Rule127Trade,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    Rule155Trade,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    SoldLast,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    NextDayTrade,
    /// Field variant 'K'.
    #[minifix(variant = "K")]
    Opened,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    Seller,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    Sold,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    StoppedStock,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    ImbalanceMoreBuyers,
    /// Field variant 'Q'.
    #[minifix(variant = "Q")]
    ImbalanceMoreSellers,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    OpeningPrice,
}

/// Field type variants for [`PosType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosType {
    /// Field variant 'TQ'.
    #[minifix(variant = "TQ")]
    TransactionQuantity,
    /// Field variant 'IAS'.
    #[minifix(variant = "IAS")]
    IntraSpreadQty,
    /// Field variant 'IES'.
    #[minifix(variant = "IES")]
    InterSpreadQty,
    /// Field variant 'FIN'.
    #[minifix(variant = "FIN")]
    EndOfDayQty,
    /// Field variant 'SOD'.
    #[minifix(variant = "SOD")]
    StartOfDayQty,
    /// Field variant 'EX'.
    #[minifix(variant = "EX")]
    OptionExerciseQty,
    /// Field variant 'AS'.
    #[minifix(variant = "AS")]
    OptionAssignment,
    /// Field variant 'TX'.
    #[minifix(variant = "TX")]
    TransactionFromExercise,
    /// Field variant 'TA'.
    #[minifix(variant = "TA")]
    TransactionFromAssignment,
    /// Field variant 'PIT'.
    #[minifix(variant = "PIT")]
    PitTradeQty,
    /// Field variant 'TRF'.
    #[minifix(variant = "TRF")]
    TransferTradeQty,
    /// Field variant 'ETR'.
    #[minifix(variant = "ETR")]
    ElectronicTradeQty,
    /// Field variant 'ALC'.
    #[minifix(variant = "ALC")]
    AllocationTradeQty,
    /// Field variant 'PA'.
    #[minifix(variant = "PA")]
    AdjustmentQty,
    /// Field variant 'ASF'.
    #[minifix(variant = "ASF")]
    AsOfTradeQty,
    /// Field variant 'DLV'.
    #[minifix(variant = "DLV")]
    DeliveryQty,
    /// Field variant 'TOT'.
    #[minifix(variant = "TOT")]
    TotalTransactionQty,
    /// Field variant 'XM'.
    #[minifix(variant = "XM")]
    CrossMarginQty,
    /// Field variant 'SPL'.
    #[minifix(variant = "SPL")]
    IntegralSplit,
}

/// Field type variants for [`SubscriptionRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SubscriptionRequestType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Snapshot,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    SnapshotPlusUpdates,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    DisablePreviousSnapshotPlusUpdateRequest,
}

/// Field type variants for [`Adjustment`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum Adjustment {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Cancel,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Error,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Correction,
}

/// Field type variants for [`CxlRejReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CxlRejReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    TooLateToCancel,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownOrder,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Broker,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    OrderAlreadyInPendingCancelOrPendingReplaceStatus,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    UnableToProcessOrderMassCancelRequest,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Origordmodtime,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    DuplicateClordid,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`IoiQltyInd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IoiQltyInd {
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    Low,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    Medium,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    High,
}

/// Field type variants for [`FinancialStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum FinancialStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Bankrupt,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    PendingDelisting,
}

/// Field type variants for [`AdvTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AdvTransType {
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    New,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Cancel,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Replace,
}

/// Field type variants for [`AllocHandlInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocHandlInst {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Match,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Forward,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ForwardAndMatch,
}

/// Field type variants for [`AllocNoOrdersType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocNoOrdersType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NotSpecified,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExplicitListProvided,
}

/// Field type variants for [`ExchangeForPhysical`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExchangeForPhysical {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`ExpirationCycle`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExpirationCycle {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ExpireOnTradingSessionClose,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExpireOnTradingSessionOpen,
}

/// Field type variants for [`PegLimitType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PegLimitType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    OrBetter,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    StrictLimitIsAStrictLimit,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum,
}

/// Field type variants for [`MassCancelRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MassCancelRejectReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    MassCancelNotSupported,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidOrUnknownSecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InvalidOrUnknownUnderlying,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InvalidOrUnknownProduct,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidOrUnknownCficode,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvalidOrUnknownSecurityType,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    InvalidOrUnknownTradingSession,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`QuoteRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteRejectReason {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownSymbol,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Exchange,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    QuoteRequestExceedsLimit,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    TooLateToEnter,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    UnknownQuote,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    DuplicateQuote,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    InvalidBidAskSpread,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    InvalidPrice,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    NotAuthorizedToQuoteSecurity,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`ListExecInstType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ListExecInstType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Immediate,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    WaitForExecuteInstruction,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ExchangeSwitchCivOrderSellDriven,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    ExchangeSwitchCivOrderBuyDrivenCashTopUp,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ExchangeSwitchCivOrderBuyDrivenCashWithdraw,
}

/// Field type variants for [`MassStatusReqType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MassStatusReqType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    StatusForOrdersForASecurity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    StatusForOrdersForAnUnderlyingSecurity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    StatusForOrdersForAProduct,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    StatusForOrdersForACficode,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    StatusForOrdersForASecuritytype,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    StatusForOrdersForATradingSession,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    StatusForAllOrders,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    StatusForOrdersForAPartyid,
}

/// Field type variants for [`EventType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum EventType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Put,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Call,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Tender,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SinkingFundCall,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`SettlPriceType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlPriceType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Final,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Theoretical,
}

/// Field type variants for [`SettlInstReqRejCode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlInstReqRejCode {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    UnableToProcessRequest,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownAccount,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NoMatchingSettlementInstructionsFound,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`MiscFeeType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MiscFeeType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Regulatory,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Tax,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    LocalCommission,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    ExchangeFees,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Stamp,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Levy,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Other,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Markup,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    ConsumptionTax,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    PerTransaction,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    Conversion,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    Agent,
}

/// Field type variants for [`SecurityRequestResult`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SecurityRequestResult {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ValidRequest,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidOrUnsupportedRequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NoInstrumentsFoundThatMatchSelectionCriteria,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NotAuthorizedToRetrieveInstrumentData,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InstrumentDataTemporarilyUnavailable,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    RequestForInstrumentDataNotSupported,
}

/// Field type variants for [`BasisPxType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BasisPxType {
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ClosingPriceAtMorningSession,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ClosingPrice,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CurrentPrice,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Sq,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    VwapThroughADay,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    VwapThroughAMorningSession,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    VwapThroughAnAfternoonSession,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    VwapThroughADayExceptYori,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    VwapThroughAMorningSessionExceptYori,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    VwapThroughAnAfternoonSessionExceptYori,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Strike,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Open,
    /// Field variant 'Z'.
    #[minifix(variant = "Z")]
    Others,
}

/// Field type variants for [`IoiNaturalFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IoiNaturalFlag {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`AllocType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Calculated,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Preliminary,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ReadyToBook,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    WarehouseInstruction,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    RequestToIntermediary,
}

/// Field type variants for [`SettlSessId`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlSessId {
    /// Field variant 'ITD'.
    #[minifix(variant = "ITD")]
    Intraday,
    /// Field variant 'RTH'.
    #[minifix(variant = "RTH")]
    RegularTradingHours,
    /// Field variant 'ETH'.
    #[minifix(variant = "ETH")]
    ElectronicTradingHours,
}

/// Field type variants for [`ExecPriceType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExecPriceType {
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    BidPrice,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    CreationPrice,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    CreationPricePlusAdjustment,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    CreationPricePlusAdjustmentAmount,
    /// Field variant 'O'.
    #[minifix(variant = "O")]
    OfferPrice,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    OfferPriceMinusAdjustment,
    /// Field variant 'Q'.
    #[minifix(variant = "Q")]
    OfferPriceMinusAdjustmentAmount,
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    SinglePrice,
}

/// Field type variants for [`PegRoundDirection`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PegRoundDirection {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

/// Field type variants for [`SolicitedFlag`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SolicitedFlag {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`AssignmentMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AssignmentMethod {
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Random,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    Prorata,
}

/// Field type variants for [`OrderRestrictions`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OrderRestrictions {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ProgramTrade,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    IndexArbitrage,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NonIndexArbitrage,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CompetingMarketMaker,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ActingAsMarketMakerOrSpecialistInTheSecurity,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    ForeignEntity,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    ExternalMarketParticipant,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    ExternalInterConnectedMarketLinkage,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    RisklessArbitrage,
}

/// Field type variants for [`AllocRejCode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocRejCode {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    UnknownAccount,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    IncorrectQuantity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    IncorrectAveragePrice,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UnknownExecutingBrokerMnemonic,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    CommissionDifference,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    UnknownOrderid,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    UnknownListid,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Other,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    IncorrectAllocatedQuantity,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    CalculationDifference,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    UnknownOrStaleExecid,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    MismatchedDataValue,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    UnknownClordid,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    WarehouseRequestRejected,
}

/// Field type variants for [`CxlRejResponseTo`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CxlRejResponseTo {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    OrderCancelRequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    OrderCancelReplaceRequest,
}

/// Field type variants for [`PreviouslyReported`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PreviouslyReported {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`PositionEffect`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PositionEffect {
    /// Field variant 'O'.
    #[minifix(variant = "O")]
    Open,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Close,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Rolled,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    Fifo,
}

/// Field type variants for [`AcctIdSource`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AcctIdSource {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Bic,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    SidCode,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Tfm,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Omgeo,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    DtccCode,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`MatchStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MatchStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    ComparedMatchedOrAffirmed,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UncomparedUnmatchedOrUnaffirmed,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AdvisoryOrAlert,
}

/// Field type variants for [`NetworkRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum NetworkRequestType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Snapshot,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Subscribe,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    StopSubscribing,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    LevelOfDetailThenNocompidsBecomesRequired,
}

/// Field type variants for [`DkReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DkReason {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    UnknownSymbol,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    WrongSide,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    QuantityExceedsOrder,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    NoMatchingOrder,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    PriceExceedsLimit,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    CalculationDifference,
    /// Field variant 'Z'.
    #[minifix(variant = "Z")]
    Other,
}

/// Field type variants for [`MdUpdateAction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MdUpdateAction {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    New,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Change,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Delete,
}

/// Field type variants for [`CrossPrioritization`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CrossPrioritization {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    None,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    BuySideIsPrioritized,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    SellSideIsPrioritized,
}

/// Field type variants for [`InViewOfCommon`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum InViewOfCommon {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`PegMoveType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PegMoveType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Floating,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Fixed,
}

/// Field type variants for [`IoiQty`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IoiQty {
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Small,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    Medium,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    Large,
}

/// Field type variants for [`DistribPaymentMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DistribPaymentMethod {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Crest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Nscc,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Euroclear,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Clearstream,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Cheque,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    TelegraphicTransfer,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Fedwire,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    DirectCredit,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    AchCredit,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    Bpay,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    HighValueClearingSystem,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    ReinvestInFund,
}

/// Field type variants for [`ExecRestatementReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExecRestatementReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    GtCorporateAction,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    GtRenewal,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    VerbalChange,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    RepricingOfOrder,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    BrokerOption,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PartialDeclineOfOrderqty,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    CancelOnTradingHalt,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    CancelOnSystemFailure,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Market,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    CanceledNotBest,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    WarehouseRecap,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`ExecInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ExecInst {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    NotHeld,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Work,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    GoAlong,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    OverTheDay,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Held,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    ParticipateDontInitiate,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    StrictScale,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    TryToScale,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    StayOnBidside,
    /// Field variant '0'.
    #[minifix(variant = "0")]
    StayOnOfferside,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    NoCross,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    OkToCross,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    CallFirst,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    PercentOfVolume,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    DoNotIncrease,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    DoNotReduce,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    AllOrNone,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    ReinstateOnSystemFailure,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    InstitutionsOnly,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    ReinstateOnTradingHalt,
    /// Field variant 'K'.
    #[minifix(variant = "K")]
    CancelOnTradingHalt,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    LastPeg,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    MidPricePeg,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    NonNegotiable,
    /// Field variant 'O'.
    #[minifix(variant = "O")]
    OpeningPeg,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    MarketPeg,
    /// Field variant 'Q'.
    #[minifix(variant = "Q")]
    CancelOnSystemFailure,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    PrimaryPeg,
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Suspend,
    /// Field variant 'U'.
    #[minifix(variant = "U")]
    CustomerDisplayInstruction,
    /// Field variant 'V'.
    #[minifix(variant = "V")]
    Netting,
    /// Field variant 'W'.
    #[minifix(variant = "W")]
    PegToVwap,
    /// Field variant 'X'.
    #[minifix(variant = "X")]
    TradeAlong,
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    TryToStop,
    /// Field variant 'Z'.
    #[minifix(variant = "Z")]
    CancelIfNotBest,
    /// Field variant 'a'.
    #[minifix(variant = "a")]
    TrailingStopPeg,
    /// Field variant 'b'.
    #[minifix(variant = "b")]
    StrictLimit,
    /// Field variant 'c'.
    #[minifix(variant = "c")]
    IgnorePriceValidityChecks,
    /// Field variant 'd'.
    #[minifix(variant = "d")]
    PegToLimitPrice,
    /// Field variant 'e'.
    #[minifix(variant = "e")]
    WorkToTargetStrategy,
}

/// Field type variants for [`HandlInst`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum HandlInst {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AutomatedExecutionOrderPrivateNoBrokerIntervention,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AutomatedExecutionOrderPublicBrokerInterventionOk,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ManualOrderBestExecution,
}

/// Field type variants for [`QuoteResponseLevel`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteResponseLevel {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    NoAcknowledgement,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AcknowledgeOnlyNegativeOrErroneousQuotes,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AcknowledgeEachQuoteMessages,
}

/// Field type variants for [`RoutingType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum RoutingType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    TargetFirm,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TargetList,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    BlockFirm,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    BlockList,
}

/// Field type variants for [`NetworkStatusResponseType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum NetworkStatusResponseType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Full,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    IncrementalUpdate,
}

/// Field type variants for [`StatusValue`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum StatusValue {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Connected,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NotConnectedDownExpectedUp,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    NotConnectedDownExpectedDown,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InProcess,
}

/// Field type variants for [`OrdType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OrdType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Market,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Limit,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Stop,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    StopLimit,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    WithOrWithout,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    LimitOrBetter,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    LimitWithOrWithout,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    OnBasis,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    PreviouslyQuoted,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    PreviouslyIndicated,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Forex,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    Funari,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    MarketIfTouched,
    /// Field variant 'K'.
    #[minifix(variant = "K")]
    MarketWithLeftoverAsLimit,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    PreviousFundValuationPoint,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    NextFundValuationPoint,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    Pegged,
}

/// Field type variants for [`MsgType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MsgType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Heartbeat,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    TestRequest,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ResendRequest,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Reject,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    SequenceReset,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Logout,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    IndicationOfInterest,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Advertisement,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    ExecutionReport,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    OrderCancelReject,
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Logon,
    /// Field variant 'B'.
    #[minifix(variant = "B")]
    News,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Email,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    OrderSingle,
    /// Field variant 'E'.
    #[minifix(variant = "E")]
    OrderList,
    /// Field variant 'F'.
    #[minifix(variant = "F")]
    OrderCancelRequest,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    OrderCancelReplaceRequest,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    OrderStatusRequest,
    /// Field variant 'J'.
    #[minifix(variant = "J")]
    AllocationInstruction,
    /// Field variant 'K'.
    #[minifix(variant = "K")]
    ListCancelRequest,
    /// Field variant 'L'.
    #[minifix(variant = "L")]
    ListExecute,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    ListStatusRequest,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    ListStatus,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    AllocationInstructionAck,
    /// Field variant 'Q'.
    #[minifix(variant = "Q")]
    DontKnowTrade,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    QuoteRequest,
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Quote,
    /// Field variant 'T'.
    #[minifix(variant = "T")]
    SettlementInstructions,
    /// Field variant 'V'.
    #[minifix(variant = "V")]
    MarketDataRequest,
    /// Field variant 'W'.
    #[minifix(variant = "W")]
    MarketDataSnapshotFullRefresh,
    /// Field variant 'X'.
    #[minifix(variant = "X")]
    MarketDataIncrementalRefresh,
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    MarketDataRequestReject,
    /// Field variant 'Z'.
    #[minifix(variant = "Z")]
    QuoteCancel,
    /// Field variant 'a'.
    #[minifix(variant = "a")]
    QuoteStatusRequest,
    /// Field variant 'b'.
    #[minifix(variant = "b")]
    MassQuoteAcknowledgement,
    /// Field variant 'c'.
    #[minifix(variant = "c")]
    SecurityDefinitionRequest,
    /// Field variant 'd'.
    #[minifix(variant = "d")]
    SecurityDefinition,
    /// Field variant 'e'.
    #[minifix(variant = "e")]
    SecurityStatusRequest,
    /// Field variant 'f'.
    #[minifix(variant = "f")]
    SecurityStatus,
    /// Field variant 'g'.
    #[minifix(variant = "g")]
    TradingSessionStatusRequest,
    /// Field variant 'h'.
    #[minifix(variant = "h")]
    TradingSessionStatus,
    /// Field variant 'i'.
    #[minifix(variant = "i")]
    MassQuote,
    /// Field variant 'j'.
    #[minifix(variant = "j")]
    BusinessMessageReject,
    /// Field variant 'k'.
    #[minifix(variant = "k")]
    BidRequest,
    /// Field variant 'l'.
    #[minifix(variant = "l")]
    BidResponse,
    /// Field variant 'm'.
    #[minifix(variant = "m")]
    ListStrikePrice,
    /// Field variant 'n'.
    #[minifix(variant = "n")]
    XmlMessage,
    /// Field variant 'o'.
    #[minifix(variant = "o")]
    RegistrationInstructions,
    /// Field variant 'p'.
    #[minifix(variant = "p")]
    RegistrationInstructionsResponse,
    /// Field variant 'q'.
    #[minifix(variant = "q")]
    OrderMassCancelRequest,
    /// Field variant 'r'.
    #[minifix(variant = "r")]
    OrderMassCancelReport,
    /// Field variant 's'.
    #[minifix(variant = "s")]
    NewOrderS,
    /// Field variant 't'.
    #[minifix(variant = "t")]
    CrossOrderCancelReplaceRequest,
    /// Field variant 'u'.
    #[minifix(variant = "u")]
    CrossOrderCancelRequest,
    /// Field variant 'v'.
    #[minifix(variant = "v")]
    SecurityTypeRequest,
    /// Field variant 'w'.
    #[minifix(variant = "w")]
    SecurityTypes,
    /// Field variant 'x'.
    #[minifix(variant = "x")]
    SecurityListRequest,
    /// Field variant 'y'.
    #[minifix(variant = "y")]
    SecurityList,
    /// Field variant 'z'.
    #[minifix(variant = "z")]
    DerivativeSecurityListRequest,
    /// Field variant 'AA'.
    #[minifix(variant = "AA")]
    DerivativeSecurityList,
    /// Field variant 'AB'.
    #[minifix(variant = "AB")]
    NewOrderAb,
    /// Field variant 'AC'.
    #[minifix(variant = "AC")]
    MultilegOrderCancelReplace,
    /// Field variant 'AD'.
    #[minifix(variant = "AD")]
    TradeCaptureReportRequest,
    /// Field variant 'AE'.
    #[minifix(variant = "AE")]
    TradeCaptureReport,
    /// Field variant 'AF'.
    #[minifix(variant = "AF")]
    OrderMassStatusRequest,
    /// Field variant 'AG'.
    #[minifix(variant = "AG")]
    QuoteRequestReject,
    /// Field variant 'AH'.
    #[minifix(variant = "AH")]
    RfqRequest,
    /// Field variant 'AI'.
    #[minifix(variant = "AI")]
    QuoteStatusReport,
    /// Field variant 'AJ'.
    #[minifix(variant = "AJ")]
    QuoteResponse,
    /// Field variant 'AK'.
    #[minifix(variant = "AK")]
    Confirmation,
    /// Field variant 'AL'.
    #[minifix(variant = "AL")]
    PositionMaintenanceRequest,
    /// Field variant 'AM'.
    #[minifix(variant = "AM")]
    PositionMaintenanceReport,
    /// Field variant 'AN'.
    #[minifix(variant = "AN")]
    RequestForPositions,
    /// Field variant 'AO'.
    #[minifix(variant = "AO")]
    RequestForPositionsAck,
    /// Field variant 'AP'.
    #[minifix(variant = "AP")]
    PositionReport,
    /// Field variant 'AQ'.
    #[minifix(variant = "AQ")]
    TradeCaptureReportRequestAck,
    /// Field variant 'AR'.
    #[minifix(variant = "AR")]
    TradeCaptureReportAck,
    /// Field variant 'AS'.
    #[minifix(variant = "AS")]
    AllocationReport,
    /// Field variant 'AT'.
    #[minifix(variant = "AT")]
    AllocationReportAck,
    /// Field variant 'AU'.
    #[minifix(variant = "AU")]
    ConfirmationAck,
    /// Field variant 'AV'.
    #[minifix(variant = "AV")]
    SettlementInstructionRequest,
    /// Field variant 'AW'.
    #[minifix(variant = "AW")]
    AssignmentReport,
    /// Field variant 'AX'.
    #[minifix(variant = "AX")]
    CollateralRequest,
    /// Field variant 'AY'.
    #[minifix(variant = "AY")]
    CollateralAssignment,
    /// Field variant 'AZ'.
    #[minifix(variant = "AZ")]
    CollateralResponse,
    /// Field variant 'BA'.
    #[minifix(variant = "BA")]
    CollateralReport,
    /// Field variant 'BB'.
    #[minifix(variant = "BB")]
    CollateralInquiry,
    /// Field variant 'BC'.
    #[minifix(variant = "BC")]
    NetworkBc,
    /// Field variant 'BD'.
    #[minifix(variant = "BD")]
    NetworkBd,
    /// Field variant 'BE'.
    #[minifix(variant = "BE")]
    UserRequest,
    /// Field variant 'BF'.
    #[minifix(variant = "BF")]
    UserResponse,
    /// Field variant 'BG'.
    #[minifix(variant = "BG")]
    CollateralInquiryAck,
    /// Field variant 'BH'.
    #[minifix(variant = "BH")]
    ConfirmationRequest,
}

/// Field type variants for [`TradedFlatSwitch`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradedFlatSwitch {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`ShortSaleReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ShortSaleReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    DealerSoldShort,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    DealerSoldShortExempt,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    SellingCustomerSoldShort,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    SellingCustomerSoldShortExempt,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    QualifedServiceRepresentative,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    QsrOrAguContraSideSoldShortExempt,
}

/// Field type variants for [`DiscretionRoundDirection`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DiscretionRoundDirection {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

/// Field type variants for [`CollStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Unassigned,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    PartiallyAssigned,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AssignmentProposed,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Assigned,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Challenged,
}

/// Field type variants for [`StipulationType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum StipulationType {
    /// Field variant 'AMT'.
    #[minifix(variant = "AMT")]
    Amt,
    /// Field variant 'AUTOREINV'.
    #[minifix(variant = "AUTOREINV")]
    AutoReinvestmentAtRateOrBetter,
    /// Field variant 'BANKQUAL'.
    #[minifix(variant = "BANKQUAL")]
    BankQualified,
    /// Field variant 'BGNCON'.
    #[minifix(variant = "BGNCON")]
    BargainConditionsSee,
    /// Field variant 'COUPON'.
    #[minifix(variant = "COUPON")]
    CouponRange,
    /// Field variant 'CURRENCY'.
    #[minifix(variant = "CURRENCY")]
    IsoCurrencyCode,
    /// Field variant 'CUSTOMDATE'.
    #[minifix(variant = "CUSTOMDATE")]
    CustomStartEndDate,
    /// Field variant 'GEOG'.
    #[minifix(variant = "GEOG")]
    GeographicsAndRange,
    /// Field variant 'HAIRCUT'.
    #[minifix(variant = "HAIRCUT")]
    ValuationDiscount,
    /// Field variant 'INSURED'.
    #[minifix(variant = "INSURED")]
    Insured,
    /// Field variant 'ISSUE'.
    #[minifix(variant = "ISSUE")]
    YearOrYearMonthOfIssue,
    /// Field variant 'ISSUER'.
    #[minifix(variant = "ISSUER")]
    IssuersTicker,
    /// Field variant 'ISSUESIZE'.
    #[minifix(variant = "ISSUESIZE")]
    IssueSizeRange,
    /// Field variant 'LOOKBACK'.
    #[minifix(variant = "LOOKBACK")]
    LookbackDays,
    /// Field variant 'LOT'.
    #[minifix(variant = "LOT")]
    ExplicitLotIdentifier,
    /// Field variant 'LOTVAR'.
    #[minifix(variant = "LOTVAR")]
    LotVariance,
    /// Field variant 'MAT'.
    #[minifix(variant = "MAT")]
    MaturityYearAndMonth,
    /// Field variant 'MATURITY'.
    #[minifix(variant = "MATURITY")]
    MaturityRange,
    /// Field variant 'MAXSUBS'.
    #[minifix(variant = "MAXSUBS")]
    MaximumSubstitutions,
    /// Field variant 'MINQTY'.
    #[minifix(variant = "MINQTY")]
    MinimumQuantity,
    /// Field variant 'MININCR'.
    #[minifix(variant = "MININCR")]
    MinimumIncrement,
    /// Field variant 'MINDNOM'.
    #[minifix(variant = "MINDNOM")]
    MinimumDenomination,
    /// Field variant 'PAYFREQ'.
    #[minifix(variant = "PAYFREQ")]
    PaymentFrequencyCalendar,
    /// Field variant 'PIECES'.
    #[minifix(variant = "PIECES")]
    NumberOfPieces,
    /// Field variant 'PMAX'.
    #[minifix(variant = "PMAX")]
    PoolsMaximum,
    /// Field variant 'PPM'.
    #[minifix(variant = "PPM")]
    PoolsPerMillion,
    /// Field variant 'PPL'.
    #[minifix(variant = "PPL")]
    PoolsPerLot,
    /// Field variant 'PPT'.
    #[minifix(variant = "PPT")]
    PoolsPerTrade,
    /// Field variant 'PRICE'.
    #[minifix(variant = "PRICE")]
    PriceRange,
    /// Field variant 'PRICEFREQ'.
    #[minifix(variant = "PRICEFREQ")]
    PricingFrequency,
    /// Field variant 'PROD'.
    #[minifix(variant = "PROD")]
    ProductionYear,
    /// Field variant 'PROTECT'.
    #[minifix(variant = "PROTECT")]
    CallProtection,
    /// Field variant 'PURPOSE'.
    #[minifix(variant = "PURPOSE")]
    Purpose,
    /// Field variant 'PXSOURCE'.
    #[minifix(variant = "PXSOURCE")]
    BenchmarkPriceSource,
    /// Field variant 'RATING'.
    #[minifix(variant = "RATING")]
    RatingSourceAndRange,
    /// Field variant 'REDEMPTION'.
    #[minifix(variant = "REDEMPTION")]
    TypeOfRedemptionValuesAreNoncallableCallablePrefundedEscrowedtomaturityPutableConvertible,
    /// Field variant 'RESTRICTED'.
    #[minifix(variant = "RESTRICTED")]
    Restricted,
    /// Field variant 'SECTOR'.
    #[minifix(variant = "SECTOR")]
    MarketSector,
    /// Field variant 'SECTYPE'.
    #[minifix(variant = "SECTYPE")]
    SecuritytypeIncludedOrExcluded,
    /// Field variant 'STRUCT'.
    #[minifix(variant = "STRUCT")]
    Structure,
    /// Field variant 'SUBSFREQ'.
    #[minifix(variant = "SUBSFREQ")]
    SubstitutionsFrequency,
    /// Field variant 'SUBSLEFT'.
    #[minifix(variant = "SUBSLEFT")]
    SubstitutionsLeft,
    /// Field variant 'TEXT'.
    #[minifix(variant = "TEXT")]
    FreeformText,
    /// Field variant 'TRDVAR'.
    #[minifix(variant = "TRDVAR")]
    TradeVariance,
    /// Field variant 'WAC'.
    #[minifix(variant = "WAC")]
    WeightedAverageCouponvalueInPercent,
    /// Field variant 'WAL'.
    #[minifix(variant = "WAL")]
    WeightedAverageLifeCouponValueInPercent,
    /// Field variant 'WALA'.
    #[minifix(variant = "WALA")]
    WeightedAverageLoanAgeValueInMonths,
    /// Field variant 'WAM'.
    #[minifix(variant = "WAM")]
    WeightedAverageMaturityValueInMonths,
    /// Field variant 'WHOLE'.
    #[minifix(variant = "WHOLE")]
    WholePool,
    /// Field variant 'YIELD'.
    #[minifix(variant = "YIELD")]
    YieldRange,
}

/// Field type variants for [`MsgDirection`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum MsgDirection {
    /// Field variant 'S'.
    #[minifix(variant = "S")]
    Send,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Receive,
}

/// Field type variants for [`RegistStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum RegistStatus {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Accepted,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Rejected,
    /// Field variant 'H'.
    #[minifix(variant = "H")]
    Held,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    ReminderIeRegistrationInstructionsAreStillOutstanding,
}

/// Field type variants for [`ListStatusType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ListStatusType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Ack,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Response,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Timed,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Execstarted,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Alldone,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Alert,
}

/// Field type variants for [`AccountType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AccountType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AccountIsCarriedOnCustomerSideOfBooks,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AccountIsCarriedOnNonCustomerSideOfBooks,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    HouseTrader,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    FloorTrader,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    AccountIsHouseTraderAndIsCrossMargined,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    JointBackofficeAccount,
}

/// Field type variants for [`TerminationType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TerminationType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Overnight,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Term,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Flexible,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Open,
}

/// Field type variants for [`QtyType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QtyType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Units,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Contracts,
}

/// Field type variants for [`DueToRelated`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum DueToRelated {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`SettlInstTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlInstTransType {
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    New,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Cancel,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Replace,
    /// Field variant 'T'.
    #[minifix(variant = "T")]
    Restate,
}

/// Field type variants for [`OwnerType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OwnerType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    IndividualInvestor,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    PublicCompany,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PrivateCompany,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    IndividualTrustee,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    CompanyTrustee,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PensionPlan,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    CustodianUnderGiftsToMinorsAct,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    Trusts,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    Fiduciaries,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    NetworkingSubAccount,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    NonProfitOrganization,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    CorporateBody,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    Nominee,
}

/// Field type variants for [`SessionRejectReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SessionRejectReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    InvalidTagNumber,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    RequiredTagMissing,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TagNotDefinedForThisMessageType,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UndefinedTag,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    TagSpecifiedWithoutAValue,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ValueIsIncorrect,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    IncorrectDataFormatForValue,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    DecryptionProblem,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    SignatureProblem,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    CompidProblem,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    SendingtimeAccuracyProblem,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    InvalidMsgtype,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    XmlValidationError,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    TagAppearsMoreThanOnce,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    TagSpecifiedOutOfRequiredOrder,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    RepeatingGroupFieldsOutOfOrder,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    IncorrectNumingroupCountForRepeatingGroup,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    NonDataValueIncludesFieldDelimiter,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`NetGrossInd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum NetGrossInd {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Net,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Gross,
}

/// Field type variants for [`PosReqStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosReqStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Completed,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    CompletedWithWarnings,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Rejected,
}

/// Field type variants for [`AllocAccountType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum AllocAccountType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AccountIsCarriedOnCustomerSideOfBooks,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    AccountIsCarriedOnNonCustomerSideOfBooks,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    HouseTrader,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    FloorTrader,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    AccountIsHouseTraderAndIsCrossMargined,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    JointBackofficeAccount,
}

/// Field type variants for [`ForexReq`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ForexReq {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`LastLiquidityInd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LastLiquidityInd {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AddedLiquidity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    RemovedLiquidity,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    LiquidityRoutedOut,
}

/// Field type variants for [`PosQtyStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosQtyStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Submitted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Accepted,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Rejected,
}

/// Field type variants for [`PosTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosTransType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Exercise,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    DoNotExercise,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PositionAdjustment,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    PositionChangeSubmissionMarginDisposition,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Pledge,
}

/// Field type variants for [`EncryptMethod`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum EncryptMethod {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    None,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Pkcs,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Des,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    PkcsDes,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    PgpDes,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PgpDesMd5,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PemDesMd5,
}

/// Field type variants for [`TradeAllocIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeAllocIndicator {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    AllocationNotRequired,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    AllocationRequired,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    UseAllocationProvidedWithTheTrade,
}

/// Field type variants for [`IoiTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum IoiTransType {
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    New,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Cancel,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    Replace,
}

/// Field type variants for [`CollAsgnReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollAsgnReason {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Initial,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Scheduled,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TimeWarning,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    MarginDeficiency,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    MarginExcess,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    ForwardCollateralDemand,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    EventOfDefault,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    AdverseTaxEvent,
}

/// Field type variants for [`TradeRequestStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradeRequestStatus {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Accepted,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Completed,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Rejected,
}

/// Field type variants for [`QuoteRequestType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteRequestType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Manual,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Automatic,
}

/// Field type variants for [`BidRequestTransType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BidRequestTransType {
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    New,
    /// Field variant 'C'.
    #[minifix(variant = "C")]
    Cancel,
}

/// Field type variants for [`CollAction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CollAction {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    Retain,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Add,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Remove,
}

/// Field type variants for [`RegistRejReasonCode`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum RegistRejReasonCode {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    InvalidUnacceptableAccountType,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    InvalidUnacceptableTaxExemptType,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    InvalidUnacceptableOwnershipType,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    InvalidUnacceptableNoRegDetls,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvalidUnacceptableRegSeqNo,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    InvalidUnacceptableRegDtls,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    InvalidUnacceptableMailingDtls,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    InvalidUnacceptableMailingInst,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    InvalidUnacceptableInvestorId,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    InvalidUnacceptableInvestorIdSource,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    InvalidUnacceptableDateOfBirth,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    InvalidUnacceptableInvestorCountryOfResidence,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    InvalidUnacceptableNodistribinstns,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    InvalidUnacceptableDistribPercentage,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    InvalidUnacceptableDistribPaymentMethod,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    InvalidUnacceptableCashDistribAgentAcctName,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    InvalidUnacceptableCashDistribAgentCode,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    InvalidUnacceptableCashDistribAgentAcctNum,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`LegalConfirm`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum LegalConfirm {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    No,
}

/// Field type variants for [`TickDirection`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TickDirection {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    PlusTick,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ZeroPlusTick,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    MinusTick,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ZeroMinusTick,
}

/// Field type variants for [`UserStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum UserStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    LoggedIn,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    NotLoggedIn,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    UserNotRecognised,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    PasswordIncorrect,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    PasswordChanged,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Other,
}

/// Field type variants for [`BookingType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum BookingType {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    RegularBooking,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Cfd,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    TotalReturnSwap,
}

/// Field type variants for [`QuoteRespType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum QuoteRespType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    HitLift,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Counter,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Expired,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Cover,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    DoneAway,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    Pass,
}

/// Field type variants for [`SideValueInd`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SideValueInd {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Sidevalue1,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Sidevalue2,
}

/// Field type variants for [`YieldType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum YieldType {
    /// Field variant 'AFTERTAX'.
    #[minifix(variant = "AFTERTAX")]
    AfterTaxYield,
    /// Field variant 'ANNUAL'.
    #[minifix(variant = "ANNUAL")]
    AnnualYield,
    /// Field variant 'ATISSUE'.
    #[minifix(variant = "ATISSUE")]
    YieldAtIssue,
    /// Field variant 'AVGMATURITY'.
    #[minifix(variant = "AVGMATURITY")]
    YieldToAverageMaturity,
    /// Field variant 'BOOK'.
    #[minifix(variant = "BOOK")]
    BookYield,
    /// Field variant 'CALL'.
    #[minifix(variant = "CALL")]
    YieldToNextCall,
    /// Field variant 'CHANGE'.
    #[minifix(variant = "CHANGE")]
    YieldChangeSinceClose,
    /// Field variant 'CLOSE'.
    #[minifix(variant = "CLOSE")]
    ClosingYield,
    /// Field variant 'COMPOUND'.
    #[minifix(variant = "COMPOUND")]
    CompoundYield,
    /// Field variant 'CURRENT'.
    #[minifix(variant = "CURRENT")]
    CurrentYield,
    /// Field variant 'GROSS'.
    #[minifix(variant = "GROSS")]
    TrueGrossYield,
    /// Field variant 'GOVTEQUIV'.
    #[minifix(variant = "GOVTEQUIV")]
    GovernmentEquivalentYield,
    /// Field variant 'INFLATION'.
    #[minifix(variant = "INFLATION")]
    YieldWithInflationAssumption,
    /// Field variant 'INVERSEFLOATER'.
    #[minifix(variant = "INVERSEFLOATER")]
    InverseFloaterBondYield,
    /// Field variant 'LASTCLOSE'.
    #[minifix(variant = "LASTCLOSE")]
    MostRecentClosingYield,
    /// Field variant 'LASTMONTH'.
    #[minifix(variant = "LASTMONTH")]
    ClosingYieldMostRecentMonth,
    /// Field variant 'LASTQUARTER'.
    #[minifix(variant = "LASTQUARTER")]
    ClosingYieldMostRecentQuarter,
    /// Field variant 'LASTYEAR'.
    #[minifix(variant = "LASTYEAR")]
    ClosingYieldMostRecentYear,
    /// Field variant 'LONGAVGLIFE'.
    #[minifix(variant = "LONGAVGLIFE")]
    YieldToLongestAverageLife,
    /// Field variant 'MARK'.
    #[minifix(variant = "MARK")]
    MarkToMarketYield,
    /// Field variant 'MATURITY'.
    #[minifix(variant = "MATURITY")]
    YieldToMaturity,
    /// Field variant 'NEXTREFUND'.
    #[minifix(variant = "NEXTREFUND")]
    YieldToNextRefund,
    /// Field variant 'OPENAVG'.
    #[minifix(variant = "OPENAVG")]
    OpenAverageYield,
    /// Field variant 'PUT'.
    #[minifix(variant = "PUT")]
    YieldToNextPut,
    /// Field variant 'PREVCLOSE'.
    #[minifix(variant = "PREVCLOSE")]
    PreviousCloseYield,
    /// Field variant 'PROCEEDS'.
    #[minifix(variant = "PROCEEDS")]
    ProceedsYield,
    /// Field variant 'SEMIANNUAL'.
    #[minifix(variant = "SEMIANNUAL")]
    SemiAnnualYield,
    /// Field variant 'SHORTAVGLIFE'.
    #[minifix(variant = "SHORTAVGLIFE")]
    YieldToShortestAverageLife,
    /// Field variant 'SIMPLE'.
    #[minifix(variant = "SIMPLE")]
    SimpleYield,
    /// Field variant 'TAXEQUIV'.
    #[minifix(variant = "TAXEQUIV")]
    TaxEquivalentYield,
    /// Field variant 'TENDER'.
    #[minifix(variant = "TENDER")]
    YieldToTenderDate,
    /// Field variant 'TRUE'.
    #[minifix(variant = "TRUE")]
    TrueYield,
    /// Field variant 'VALUE1/32'.
    #[minifix(variant = "VALUE1/32")]
    YieldValueOf132,
    /// Field variant 'WORST'.
    #[minifix(variant = "WORST")]
    YieldToWorst,
}

/// Field type variants for [`OrderCapacity`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum OrderCapacity {
    /// Field variant 'A'.
    #[minifix(variant = "A")]
    Agency,
    /// Field variant 'G'.
    #[minifix(variant = "G")]
    Proprietary,
    /// Field variant 'I'.
    #[minifix(variant = "I")]
    Individual,
    /// Field variant 'P'.
    #[minifix(variant = "P")]
    Principal,
    /// Field variant 'R'.
    #[minifix(variant = "R")]
    RisklessPrincipal,
    /// Field variant 'W'.
    #[minifix(variant = "W")]
    AgentForOtherMember,
}

/// Field type variants for [`PartySubIdType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PartySubIdType {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Firm,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Person,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    System,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Application,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    FullLegalNameOfFirm,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    PostalAddress,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    PhoneNumber,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    EmailAddress,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    ContactName,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    SecuritiesAccountNumber,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    RegistrationNumber,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    RegisteredAddress12,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    RegulatoryStatus,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    RegistrationName,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    CashAccountNumber,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    Bic,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    CsdParticipantMemberCode,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    RegisteredAddress18,
    /// Field variant '19'.
    #[minifix(variant = "19")]
    FundAccountName,
    /// Field variant '20'.
    #[minifix(variant = "20")]
    TelexNumber,
    /// Field variant '21'.
    #[minifix(variant = "21")]
    FaxNumber,
    /// Field variant '22'.
    #[minifix(variant = "22")]
    SecuritiesAccountName,
    /// Field variant '23'.
    #[minifix(variant = "23")]
    CashAccountName,
    /// Field variant '24'.
    #[minifix(variant = "24")]
    Department,
    /// Field variant '25'.
    #[minifix(variant = "25")]
    Location,
    /// Field variant '26'.
    #[minifix(variant = "26")]
    PositionAccountType,
}

/// Field type variants for [`PartyRole`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PartyRole {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    ExecutingFirm,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    BrokerOfCredit,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    ClientId,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    ClearingFirm,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    InvestorId,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    IntroducingFirm,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    EnteringFirm,
    /// Field variant '8'.
    #[minifix(variant = "8")]
    LocateLendingFirm,
    /// Field variant '9'.
    #[minifix(variant = "9")]
    FundManagerClientId,
    /// Field variant '10'.
    #[minifix(variant = "10")]
    SettlementLocation,
    /// Field variant '11'.
    #[minifix(variant = "11")]
    OrderOriginationTrader,
    /// Field variant '12'.
    #[minifix(variant = "12")]
    ExecutingTrader,
    /// Field variant '13'.
    #[minifix(variant = "13")]
    OrderOriginationFirm,
    /// Field variant '14'.
    #[minifix(variant = "14")]
    GiveupClearingFirm,
    /// Field variant '15'.
    #[minifix(variant = "15")]
    CorrespondantClearingFirm,
    /// Field variant '16'.
    #[minifix(variant = "16")]
    ExecutingSystem,
    /// Field variant '17'.
    #[minifix(variant = "17")]
    ContraFirm,
    /// Field variant '18'.
    #[minifix(variant = "18")]
    ContraClearingFirm,
    /// Field variant '19'.
    #[minifix(variant = "19")]
    SponsoringFirm,
    /// Field variant '20'.
    #[minifix(variant = "20")]
    UnderlyingContraFirm,
    /// Field variant '21'.
    #[minifix(variant = "21")]
    ClearingOrganization,
    /// Field variant '22'.
    #[minifix(variant = "22")]
    Exchange,
    /// Field variant '24'.
    #[minifix(variant = "24")]
    CustomerAccount,
    /// Field variant '25'.
    #[minifix(variant = "25")]
    CorrespondentClearingOrganization,
    /// Field variant '26'.
    #[minifix(variant = "26")]
    CorrespondentBroker,
    /// Field variant '27'.
    #[minifix(variant = "27")]
    BuyerSeller,
    /// Field variant '28'.
    #[minifix(variant = "28")]
    Custodian,
    /// Field variant '29'.
    #[minifix(variant = "29")]
    Intermediary,
    /// Field variant '30'.
    #[minifix(variant = "30")]
    Agent,
    /// Field variant '31'.
    #[minifix(variant = "31")]
    SubCustodian,
    /// Field variant '32'.
    #[minifix(variant = "32")]
    Beneficiary,
    /// Field variant '33'.
    #[minifix(variant = "33")]
    InterestedParty,
    /// Field variant '34'.
    #[minifix(variant = "34")]
    RegulatoryBody,
    /// Field variant '35'.
    #[minifix(variant = "35")]
    LiquidityProvider,
    /// Field variant '36'.
    #[minifix(variant = "36")]
    EnteringTrader,
    /// Field variant '37'.
    #[minifix(variant = "37")]
    ContraTrader,
    /// Field variant '38'.
    #[minifix(variant = "38")]
    PositionAccount,
}

/// Field type variants for [`SettlCurrFxRateCalc`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum SettlCurrFxRateCalc {
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    Multiply,
    /// Field variant 'D'.
    #[minifix(variant = "D")]
    Divide,
}

/// Field type variants for [`ListOrderStatus`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum ListOrderStatus {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    Inbiddingprocess,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    Receivedforexecution,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    Executing,
    /// Field variant '4'.
    #[minifix(variant = "4")]
    Canceling,
    /// Field variant '5'.
    #[minifix(variant = "5")]
    Alert,
    /// Field variant '6'.
    #[minifix(variant = "6")]
    AllDone,
    /// Field variant '7'.
    #[minifix(variant = "7")]
    Reject,
}

/// Field type variants for [`TradSesStatusRejReason`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum TradSesStatusRejReason {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    UnknownOrInvalidTradingsessionid,
    /// Field variant '99'.
    #[minifix(variant = "99")]
    Other,
}

/// Field type variants for [`PriorityIndicator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PriorityIndicator {
    /// Field variant '0'.
    #[minifix(variant = "0")]
    PriorityUnchanged,
    /// Field variant '1'.
    #[minifix(variant = "1")]
    LostPriorityAsResultOfOrderChange,
}

/// Field type variants for [`PosMaintAction`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosMaintAction {
    /// Field variant '1'.
    #[minifix(variant = "1")]
    NewUsedToIncrementTheOverallTransactionQuantity,
    /// Field variant '2'.
    #[minifix(variant = "2")]
    ReplaceUsedToOverrideTheOverallTransactionQuantityOrSpecificAddMessagesBasedOnTheReferenceId,
    /// Field variant '3'.
    #[minifix(variant = "3")]
    CancelUsedToRemoveTheOverallTransactionOrSpecificAddMessagesBasedOnReferenceId,
}

/// Field type variants for [`CancellationRights`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum CancellationRights {
    /// Field variant 'Y'.
    #[minifix(variant = "Y")]
    Yes,
    /// Field variant 'N'.
    #[minifix(variant = "N")]
    NoExecutionOnly,
    /// Field variant 'M'.
    #[minifix(variant = "M")]
    NoWaiverAgreement,
    /// Field variant 'O'.
    #[minifix(variant = "O")]
    NoInstitutional,
}

/// Field type variants for [`PosAmtType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]

pub enum PosAmtType {
    /// Field variant 'FMTM'.
    #[minifix(variant = "FMTM")]
    FinalMarkToMarketAmount,
    /// Field variant 'IMTM'.
    #[minifix(variant = "IMTM")]
    IncrementalMarkToMarketAmount,
    /// Field variant 'TVAR'.
    #[minifix(variant = "TVAR")]
    TradeVariationAmount,
    /// Field variant 'SMTM'.
    #[minifix(variant = "SMTM")]
    StartOfDayMarkToMarketAmount,
    /// Field variant 'PREM'.
    #[minifix(variant = "PREM")]
    PremiumAmount,
    /// Field variant 'CRES'.
    #[minifix(variant = "CRES")]
    CashResidualAmount,
    /// Field variant 'CASH'.
    #[minifix(variant = "CASH")]
    CashAmount,
    /// Field variant 'VADJ'.
    #[minifix(variant = "VADJ")]
    ValueAdjustedAmount,
}

/// Field attributes for [`THRESHOLD_AMOUNT <834>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_834.html).
pub const THRESHOLD_AMOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ThresholdAmount",
    tag: 834,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED_PARTY_ROLE <538>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_538.html).
pub const NESTED_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NestedPartyRole",
    tag: 538,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNSOLICITED_INDICATOR <325>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_325.html).
pub const UNSOLICITED_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnsolicitedIndicator",
    tag: 325,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_NESTED2_PARTY_I_DS <756>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_756.html).
pub const NO_NESTED2_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNested2PartyIDs",
    tag: 756,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE_VALUE1 <396>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_396.html).
pub const SIDE_VALUE1: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SideValue1",
    tag: 396,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`MATURITY_DATE <541>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_541.html).
pub const MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaturityDate",
    tag: 541,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`EMAIL_THREAD_ID <164>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_164.html).
pub const EMAIL_THREAD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EmailThreadID",
    tag: 164,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_INPUT_DEVICE <821>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_821.html).
pub const ORDER_INPUT_DEVICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderInputDevice",
    tag: 821,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REPORT_REJECT_REASON <751>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_751.html).
pub const TRADE_REPORT_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeReportRejectReason",
    tag: 751,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_TRANS_TYPE <903>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_903.html).
pub const COLL_ASGN_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnTransType",
    tag: 903,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`APPL_QUEUE_MAX <812>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_812.html).
pub const APPL_QUEUE_MAX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ApplQueueMax",
    tag: 812,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`RAW_DATA_LENGTH <95>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_95.html).
pub const RAW_DATA_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RawDataLength",
    tag: 95,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRA_BROKER <375>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_375.html).
pub const CONTRA_BROKER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraBroker",
    tag: 375,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LIST_STATUS_TEXT <446>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_446.html).
pub const ENCODED_LIST_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedListStatusText",
    tag: 446,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CP_REG_TYPE <878>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_878.html).
pub const UNDERLYING_CP_REG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCPRegType",
    tag: 878,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_CONT_AMTS <518>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_518.html).
pub const NO_CONT_AMTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoContAmts",
    tag: 518,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_ORDERS <73>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_73.html).
pub const NO_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoOrders",
    tag: 73,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LIST_EXEC_INST <353>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_353.html).
pub const ENCODED_LIST_EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedListExecInst",
    tag: 353,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURR_BID_FX_RATE <656>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_656.html).
pub const SETTL_CURR_BID_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrBidFxRate",
    tag: 656,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`DAY_ORDER_QTY <424>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_424.html).
pub const DAY_ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DayOrderQty",
    tag: 424,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REQUEST_TYPE <569>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_569.html).
pub const TRADE_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeRequestType",
    tag: 569,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_DATE <64>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_64.html).
pub const SETTL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlDate",
    tag: 64,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_LAST_PX <651>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_651.html).
pub const UNDERLYING_LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingLastPx",
    tag: 651,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`REF_SUB_ID <931>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_931.html).
pub const REF_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefSubID",
    tag: 931,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_MAINT_STATUS <722>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_722.html).
pub const POS_MAINT_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosMaintStatus",
    tag: 722,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`GAP_FILL_FLAG <123>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_123.html).
pub const GAP_FILL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "GapFillFlag",
    tag: 123,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_MKT <275>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_275.html).
pub const MD_MKT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDMkt",
    tag: 275,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`NUMBER_OF_ORDERS <346>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_346.html).
pub const NUMBER_OF_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NumberOfOrders",
    tag: 346,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`RESPONSE_TRANSPORT_TYPE <725>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_725.html).
pub const RESPONSE_TRANSPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ResponseTransportType",
    tag: 725,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LOCATE_REQD <114>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_114.html).
pub const LOCATE_REQD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LocateReqd",
    tag: 114,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SETTL_INST <778>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_778.html).
pub const NO_SETTL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSettlInst",
    tag: 778,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SCOPE <546>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_546.html).
pub const SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Scope",
    tag: 546,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_ID <117>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_117.html).
pub const QUOTE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteID",
    tag: 117,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_TYPE <269>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_269.html).
pub const MD_ENTRY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryType",
    tag: 269,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`REVERSAL_INDICATOR <700>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_700.html).
pub const REVERSAL_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ReversalIndicator",
    tag: 700,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_STATUS <340>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_340.html).
pub const TRAD_SES_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesStatus",
    tag: 340,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CROSS_TYPE <549>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_549.html).
pub const CROSS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CrossType",
    tag: 549,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`HEART_BT_INT <108>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_108.html).
pub const HEART_BT_INT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HeartBtInt",
    tag: 108,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_QUALIFIER <695>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_695.html).
pub const QUOTE_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteQualifier",
    tag: 695,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_STRATEGY <847>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_847.html).
pub const TARGET_STRATEGY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetStrategy",
    tag: 847,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOWABLE_ONE_SIDEDNESS_VALUE <766>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_766.html).
pub const ALLOWABLE_ONE_SIDEDNESS_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllowableOneSidednessValue",
    tag: 766,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORD_REJ_REASON <103>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_103.html).
pub const ORD_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrdRejReason",
    tag: 103,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_REGIST_DTLS <473>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_473.html).
pub const NO_REGIST_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoRegistDtls",
    tag: 473,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_TRADING_STATUS <326>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_326.html).
pub const SECURITY_TRADING_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityTradingStatus",
    tag: 326,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_POS_AMT <753>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_753.html).
pub const NO_POS_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoPosAmt",
    tag: 753,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_MKT <30>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_30.html).
pub const LAST_MKT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastMkt",
    tag: 30,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_RESP_TYPE <905>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_905.html).
pub const COLL_ASGN_RESP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnRespType",
    tag: 905,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_TRADING_SESSION_ID <822>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_822.html).
pub const UNDERLYING_TRADING_SESSION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingTradingSessionID",
    tag: 822,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_ID <17>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_17.html).
pub const EXEC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecID",
    tag: 17,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_INQUIRY_QUALIFIER <896>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_896.html).
pub const COLL_INQUIRY_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollInquiryQualifier",
    tag: 896,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_STATUS <297>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_297.html).
pub const QUOTE_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteStatus",
    tag: 297,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SYMBOL <600>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_600.html).
pub const LEG_SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSymbol",
    tag: 600,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_ALT_ID_SOURCE <459>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_459.html).
pub const UNDERLYING_SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityAltIDSource",
    tag: 459,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_ENTRY_REJECT_REASON <368>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_368.html).
pub const QUOTE_ENTRY_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteEntryRejectReason",
    tag: 368,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BEGIN_STRING <8>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_8.html).
pub const BEGIN_STRING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BeginString",
    tag: 8,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`UNDERLYING_INSTR_REGISTRY <595>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_595.html).
pub const UNDERLYING_INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingInstrRegistry",
    tag: 595,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CP_PROGRAM <875>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_875.html).
pub const CP_PROGRAM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CPProgram",
    tag: 875,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_TRANS_TYPE <666>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_666.html).
pub const CONFIRM_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmTransType",
    tag: 666,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADV_REF_ID <3>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_3.html).
pub const ADV_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AdvRefID",
    tag: 3,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_PARTY_I_DS <453>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_453.html).
pub const NO_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoPartyIDs",
    tag: 453,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_TEXT_LEN <354>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_354.html).
pub const ENCODED_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedTextLen",
    tag: 354,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_PAR_PX <669>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_669.html).
pub const LAST_PAR_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastParPx",
    tag: 669,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`TEXT <58>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_58.html).
pub const TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Text",
    tag: 58,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED3_PARTY_ID <949>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_949.html).
pub const NESTED3_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested3PartyID",
    tag: 949,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`URGENCY <61>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_61.html).
pub const URGENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Urgency",
    tag: 61,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`AFFIRM_STATUS <940>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_940.html).
pub const AFFIRM_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AffirmStatus",
    tag: 940,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`COUPON_RATE <223>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_223.html).
pub const COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CouponRate",
    tag: 223,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_DATES <580>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_580.html).
pub const NO_DATES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoDates",
    tag: 580,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`CARD_START_DATE <503>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_503.html).
pub const CARD_START_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CardStartDate",
    tag: 503,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`DESK_ID <284>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_284.html).
pub const DESK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeskID",
    tag: 284,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SENDING_TIME <52>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_52.html).
pub const SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SendingTime",
    tag: 52,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Header,
};

/// Field attributes for [`SETTL_INST_REF_ID <214>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_214.html).
pub const SETTL_INST_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstRefID",
    tag: 214,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRICE_IMPROVEMENT <639>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_639.html).
pub const PRICE_IMPROVEMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriceImprovement",
    tag: 639,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_REQ_ID <791>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_791.html).
pub const SETTL_INST_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstReqID",
    tag: 791,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MKT_OFFER_PX <646>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_646.html).
pub const MKT_OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MktOfferPx",
    tag: 646,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`PUT_OR_CALL <201>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_201.html).
pub const PUT_OR_CALL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PutOrCall",
    tag: 201,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MONEY_LAUNDERING_STATUS <481>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_481.html).
pub const MONEY_LAUNDERING_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MoneyLaunderingStatus",
    tag: 481,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_PX <270>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_270.html).
pub const MD_ENTRY_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryPx",
    tag: 270,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD_REDEMPTION_PRICE <697>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_697.html).
pub const YIELD_REDEMPTION_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "YieldRedemptionPrice",
    tag: 697,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_QTY <38>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_38.html).
pub const ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderQty",
    tag: 38,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_QTY2 <192>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_192.html).
pub const ORDER_QTY2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderQty2",
    tag: 192,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTICIPATION_RATE <849>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_849.html).
pub const PARTICIPATION_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ParticipationRate",
    tag: 849,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_SCOPE <840>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_840.html).
pub const PEG_SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegScope",
    tag: 840,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_REPURCHASE_TERM <251>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_251.html).
pub const LEG_REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRepurchaseTerm",
    tag: 251,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_CFI_CODE <608>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_608.html).
pub const LEG_CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCFICode",
    tag: 608,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIQUIDITY_PCT_HIGH <403>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_403.html).
pub const LIQUIDITY_PCT_HIGH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LiquidityPctHigh",
    tag: 403,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SYMBOL_SFX <312>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_312.html).
pub const UNDERLYING_SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSymbolSfx",
    tag: 312,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MAILING_DTLS <474>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_474.html).
pub const MAILING_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MailingDtls",
    tag: 474,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_ISSUE_DATE <242>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_242.html).
pub const UNDERLYING_ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingIssueDate",
    tag: 242,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_NESTED_PARTY_I_DS <539>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_539.html).
pub const NO_NESTED_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNestedPartyIDs",
    tag: 539,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_REJECT_REASON <906>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_906.html).
pub const COLL_ASGN_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnRejectReason",
    tag: 906,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`AVG_PX_INDICATOR <819>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_819.html).
pub const AVG_PX_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AvgPxIndicator",
    tag: 819,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_ALLOC_ACCT_ID_SOURCE <674>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_674.html).
pub const LEG_ALLOC_ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegAllocAcctIDSource",
    tag: 674,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_REPURCHASE_RATE <245>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_245.html).
pub const UNDERLYING_REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingRepurchaseRate",
    tag: 245,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE_VALUE2 <397>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_397.html).
pub const SIDE_VALUE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SideValue2",
    tag: 397,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_CANCEL_TYPE <298>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_298.html).
pub const QUOTE_CANCEL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteCancelType",
    tag: 298,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ROUNDING_DIRECTION <468>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_468.html).
pub const ROUNDING_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RoundingDirection",
    tag: 468,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BENCHMARK_CURVE_NAME <677>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_677.html).
pub const LEG_BENCHMARK_CURVE_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBenchmarkCurveName",
    tag: 677,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD <236>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_236.html).
pub const YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Yield",
    tag: 236,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_FRAGMENT <893>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_893.html).
pub const LAST_FRAGMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastFragment",
    tag: 893,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_START_VALUE <884>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_884.html).
pub const UNDERLYING_START_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStartValue",
    tag: 884,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRACT_MULTIPLIER <231>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_231.html).
pub const CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContractMultiplier",
    tag: 231,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED_PARTY_ID <524>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_524.html).
pub const NESTED_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NestedPartyID",
    tag: 524,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTY_ID_SOURCE <447>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_447.html).
pub const PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PartyIDSource",
    tag: 447,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`CORPORATE_ACTION <292>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_292.html).
pub const CORPORATE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CorporateAction",
    tag: 292,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SECURITY_ALT_ID <454>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_454.html).
pub const NO_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSecurityAltID",
    tag: 454,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ACCRUED_INTEREST_RATE <158>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_158.html).
pub const ACCRUED_INTEREST_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AccruedInterestRate",
    tag: 158,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONT_AMT_TYPE <519>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_519.html).
pub const CONT_AMT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContAmtType",
    tag: 519,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED3_PARTY_ID_SOURCE <950>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_950.html).
pub const NESTED3_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested3PartyIDSource",
    tag: 950,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`BOOKING_UNIT <590>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_590.html).
pub const BOOKING_UNIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BookingUnit",
    tag: 590,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVER_TO_LOCATION_ID <145>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_145.html).
pub const DELIVER_TO_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliverToLocationID",
    tag: 145,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`DAY_CUM_QTY <425>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_425.html).
pub const DAY_CUM_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DayCumQty",
    tag: 425,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_REQ_RESULT <728>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_728.html).
pub const POS_REQ_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosReqResult",
    tag: 728,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_REJ_REASON_TEXT <496>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_496.html).
pub const REGIST_REJ_REASON_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistRejReasonText",
    tag: 496,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CLEARING_INSTRUCTION <577>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_577.html).
pub const CLEARING_INSTRUCTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClearingInstruction",
    tag: 577,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_MISC_FEES <136>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_136.html).
pub const NO_MISC_FEES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoMiscFees",
    tag: 136,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_ALLOC_ID <793>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_793.html).
pub const SECONDARY_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryAllocID",
    tag: 793,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_MAINT_RESULT <723>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_723.html).
pub const POS_MAINT_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosMaintResult",
    tag: 723,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_PARTY_ROLE <784>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_784.html).
pub const SETTL_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPartyRole",
    tag: 784,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_REQ_ID <131>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_131.html).
pub const QUOTE_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteReqID",
    tag: 131,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SIDES <552>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_552.html).
pub const NO_SIDES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSides",
    tag: 552,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`MESSAGE_ENCODING <347>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_347.html).
pub const MESSAGE_ENCODING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MessageEncoding",
    tag: 347,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`MAILING_INST <482>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_482.html).
pub const MAILING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MailingInst",
    tag: 482,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_IMPLICIT_DELETE <547>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_547.html).
pub const MD_IMPLICIT_DELETE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDImplicitDelete",
    tag: 547,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_STRATEGY_PERFORMANCE <850>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_850.html).
pub const TARGET_STRATEGY_PERFORMANCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetStrategyPerformance",
    tag: 850,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LEG_ISSUER_LEN <618>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_618.html).
pub const ENCODED_LEG_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedLegIssuerLen",
    tag: 618,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_FORWARD_POINTS <189>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_189.html).
pub const BID_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidForwardPoints",
    tag: 189,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_START_TIME <341>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_341.html).
pub const TRAD_SES_START_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesStartTime",
    tag: 341,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_AMT <708>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_708.html).
pub const POS_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosAmt",
    tag: 708,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`OUT_MAIN_CNTRY_U_INDEX <412>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_412.html).
pub const OUT_MAIN_CNTRY_U_INDEX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OutMainCntryUIndex",
    tag: 412,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LEG_SECURITY_DESC_LEN <621>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_621.html).
pub const ENCODED_LEG_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedLegSecurityDescLen",
    tag: 621,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_TYPE <773>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_773.html).
pub const CONFIRM_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmType",
    tag: 773,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOWABLE_ONE_SIDEDNESS_CURR <767>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_767.html).
pub const ALLOWABLE_ONE_SIDEDNESS_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllowableOneSidednessCurr",
    tag: 767,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVERY_TYPE <919>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_919.html).
pub const DELIVERY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliveryType",
    tag: 919,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_TYPE <828>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_828.html).
pub const TRD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdType",
    tag: 828,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`HALT_REASON_CHAR <327>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_327.html).
pub const HALT_REASON_CHAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HaltReasonChar",
    tag: 327,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_BID_DESCRIPTORS <398>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_398.html).
pub const NO_BID_DESCRIPTORS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoBidDescriptors",
    tag: 398,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_REQ_ID <894>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_894.html).
pub const COLL_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollReqID",
    tag: 894,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_AFFECTED_ORDERS <534>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_534.html).
pub const NO_AFFECTED_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoAffectedOrders",
    tag: 534,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIGNATURE <89>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_89.html).
pub const SIGNATURE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Signature",
    tag: 89,
    data_type: FixDatatype::Data,
    location: FieldLocation::Trailer,
};

/// Field attributes for [`LAST_MSG_SEQ_NUM_PROCESSED <369>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_369.html).
pub const LAST_MSG_SEQ_NUM_PROCESSED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastMsgSeqNumProcessed",
    tag: 369,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Header,
};

/// Field attributes for [`ALLOC_SETTL_CURRENCY <736>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_736.html).
pub const ALLOC_SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocSettlCurrency",
    tag: 736,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONT_AMT_CURR <521>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_521.html).
pub const CONT_AMT_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContAmtCurr",
    tag: 521,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_QTY <80>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_80.html).
pub const ALLOC_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocQty",
    tag: 80,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SETTL_PARTY_SUB_I_DS <801>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_801.html).
pub const NO_SETTL_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSettlPartySubIDs",
    tag: 801,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRACT_SETTL_MONTH <667>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_667.html).
pub const CONTRACT_SETTL_MONTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContractSettlMonth",
    tag: 667,
    data_type: FixDatatype::MonthYear,
    location: FieldLocation::Body,
};

/// Field attributes for [`STRIKE_CURRENCY <947>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_947.html).
pub const STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StrikeCurrency",
    tag: 947,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_DATE <75>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_75.html).
pub const TRADE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeDate",
    tag: 75,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_TEXT <355>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_355.html).
pub const ENCODED_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedText",
    tag: 355,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`DAY_AVG_PX <426>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_426.html).
pub const DAY_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DayAvgPx",
    tag: 426,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_ID <66>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_66.html).
pub const LIST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListID",
    tag: 66,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_REPORT_TYPE <794>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_794.html).
pub const ALLOC_REPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocReportType",
    tag: 794,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MIN_TRADE_VOL <562>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_562.html).
pub const MIN_TRADE_VOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MinTradeVol",
    tag: 562,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_EXEC_INST <69>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_69.html).
pub const LIST_EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListExecInst",
    tag: 69,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELETE_REASON <285>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_285.html).
pub const DELETE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeleteReason",
    tag: 285,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_LAST_QTY <652>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_652.html).
pub const UNDERLYING_LAST_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingLastQty",
    tag: 652,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_BID_COMPONENTS <420>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_420.html).
pub const NO_BID_COMPONENTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoBidComponents",
    tag: 420,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_COVERED_OR_UNCOVERED <565>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_565.html).
pub const LEG_COVERED_OR_UNCOVERED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCoveredOrUncovered",
    tag: 565,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_EXECS <124>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_124.html).
pub const NO_EXECS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoExecs",
    tag: 124,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`MIN_BID_SIZE <647>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_647.html).
pub const MIN_BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MinBidSize",
    tag: 647,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_CAPACITY_QTY <863>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_863.html).
pub const ORDER_CAPACITY_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderCapacityQty",
    tag: 863,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADJUSTMENT_TYPE <718>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_718.html).
pub const ADJUSTMENT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AdjustmentType",
    tag: 718,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURR_AMT <119>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_119.html).
pub const SETTL_CURR_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrAmt",
    tag: 119,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_SIZE <271>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_271.html).
pub const MD_ENTRY_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntrySize",
    tag: 271,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_OPEN_TIME <342>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_342.html).
pub const TRAD_SES_OPEN_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesOpenTime",
    tag: 342,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`SHORT_QTY <705>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_705.html).
pub const SHORT_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ShortQty",
    tag: 705,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`MIN_QTY <110>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_110.html).
pub const MIN_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MinQty",
    tag: 110,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`START_CASH <921>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_921.html).
pub const START_CASH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StartCash",
    tag: 921,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_REJ_REASON <774>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_774.html).
pub const CONFIRM_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmRejReason",
    tag: 774,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`POSS_RESEND <97>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_97.html).
pub const POSS_RESEND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PossResend",
    tag: 97,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Header,
};

/// Field attributes for [`LAST_RPT_REQUESTED <912>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_912.html).
pub const LAST_RPT_REQUESTED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastRptRequested",
    tag: 912,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_MATURITY_MONTH_YEAR <313>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_313.html).
pub const UNDERLYING_MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingMaturityMonthYear",
    tag: 313,
    data_type: FixDatatype::MonthYear,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BENCHMARK_PRICE_TYPE <680>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_680.html).
pub const LEG_BENCHMARK_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBenchmarkPriceType",
    tag: 680,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`INVESTOR_COUNTRY_OF_RESIDENCE <475>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_475.html).
pub const INVESTOR_COUNTRY_OF_RESIDENCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InvestorCountryOfResidence",
    tag: 475,
    data_type: FixDatatype::Country,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_REQ_ID <320>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_320.html).
pub const SECURITY_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityReqID",
    tag: 320,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SETTL_CURRENCY <675>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_675.html).
pub const LEG_SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSettlCurrency",
    tag: 675,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`MISC_FEE_BASIS <891>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_891.html).
pub const MISC_FEE_BASIS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MiscFeeBasis",
    tag: 891,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`OPEN_INTEREST <746>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_746.html).
pub const OPEN_INTEREST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OpenInterest",
    tag: 746,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_REF_ID <19>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_19.html).
pub const EXEC_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecRefID",
    tag: 19,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_ENTRY_ID <299>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_299.html).
pub const QUOTE_ENTRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteEntryID",
    tag: 299,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ROUNDING_MODULUS <469>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_469.html).
pub const ROUNDING_MODULUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RoundingModulus",
    tag: 469,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REQUEST_RESULT <749>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_749.html).
pub const TRADE_REQUEST_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeRequestResult",
    tag: 749,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CHECK_SUM <10>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_10.html).
pub const CHECK_SUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CheckSum",
    tag: 10,
    data_type: FixDatatype::String,
    location: FieldLocation::Trailer,
};

/// Field attributes for [`CASH_OUTSTANDING <901>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_901.html).
pub const CASH_OUTSTANDING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashOutstanding",
    tag: 901,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_PARTY_SUB_I_DS <802>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_802.html).
pub const NO_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoPartySubIDs",
    tag: 802,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`COMM_TYPE <13>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_13.html).
pub const COMM_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CommType",
    tag: 13,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_INTEREST_ACCRUAL_DATE <956>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_956.html).
pub const LEG_INTEREST_ACCRUAL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegInterestAccrualDate",
    tag: 956,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`DEF_BID_SIZE <293>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_293.html).
pub const DEF_BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DefBidSize",
    tag: 293,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_COUNTRY_OF_ISSUE <596>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_596.html).
pub const LEG_COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCountryOfIssue",
    tag: 596,
    data_type: FixDatatype::Country,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_ALT_ID <455>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_455.html).
pub const SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityAltID",
    tag: 455,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_UNDERLYING_SECURITY_DESC_LEN <364>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_364.html).
pub const ENCODED_UNDERLYING_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedUnderlyingSecurityDescLen",
    tag: 364,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADV_SIDE <4>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_4.html).
pub const ADV_SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AdvSide",
    tag: 4,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`PREALLOC_METHOD <591>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_591.html).
pub const PREALLOC_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PreallocMethod",
    tag: 591,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`INSTR_ATTRIB_TYPE <871>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_871.html).
pub const INSTR_ATTRIB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InstrAttribType",
    tag: 871,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_PRICE <662>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_662.html).
pub const BENCHMARK_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkPrice",
    tag: 662,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_TYPE <63>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_63.html).
pub const SETTL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlType",
    tag: 63,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ROUTING_ID <217>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_217.html).
pub const ROUTING_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RoutingID",
    tag: 217,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`FUND_RENEW_WAIV <497>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_497.html).
pub const FUND_RENEW_WAIV: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "FundRenewWaiv",
    tag: 497,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`OPEN_CLOSE_SETTL_FLAG <286>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_286.html).
pub const OPEN_CLOSE_SETTL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OpenCloseSettlFlag",
    tag: 286,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_STATUS_REQ_ID <649>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_649.html).
pub const QUOTE_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteStatusReqID",
    tag: 649,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE <54>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_54.html).
pub const SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Side",
    tag: 54,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NOTIFY_BROKER_OF_CREDIT <208>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_208.html).
pub const NOTIFY_BROKER_OF_CREDIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NotifyBrokerOfCredit",
    tag: 208,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`STATUS_TEXT <929>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_929.html).
pub const STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StatusText",
    tag: 929,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_CL_ORD_ID <41>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_41.html).
pub const ORIG_CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigClOrdID",
    tag: 41,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REPORT_TYPE <856>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_856.html).
pub const TRADE_REPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeReportType",
    tag: 856,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`COVERED_OR_UNCOVERED <203>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_203.html).
pub const COVERED_OR_UNCOVERED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CoveredOrUncovered",
    tag: 203,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SIDE <624>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_624.html).
pub const LEG_SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSide",
    tag: 624,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRANS_BKD_TIME <483>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_483.html).
pub const TRANS_BKD_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TransBkdTime",
    tag: 483,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`MARKET_DEPTH <264>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_264.html).
pub const MARKET_DEPTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MarketDepth",
    tag: 264,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_QTY <32>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_32.html).
pub const LAST_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastQty",
    tag: 32,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_SPOT_RATE <194>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_194.html).
pub const LAST_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastSpotRate",
    tag: 194,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LEG_ISSUER <619>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_619.html).
pub const ENCODED_LEG_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedLegIssuer",
    tag: 619,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`END_CASH <922>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_922.html).
pub const END_CASH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EndCash",
    tag: 922,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_REG_TIMESTAMP_ORIGIN <771>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_771.html).
pub const TRD_REG_TIMESTAMP_ORIGIN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdRegTimestampOrigin",
    tag: 771,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SWAP_TYPE <690>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_690.html).
pub const LEG_SWAP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSwapType",
    tag: 690,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_LINK_TYPE <197>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_197.html).
pub const ALLOC_LINK_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocLinkType",
    tag: 197,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CROSS_PERCENT <413>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_413.html).
pub const CROSS_PERCENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CrossPercent",
    tag: 413,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_RESP_ID <693>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_693.html).
pub const QUOTE_RESP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteRespID",
    tag: 693,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_REPURCHASE_RATE <252>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_252.html).
pub const LEG_REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRepurchaseRate",
    tag: 252,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_PRICE <845>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_845.html).
pub const DISCRETION_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionPrice",
    tag: 845,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_OFFSET_TYPE <836>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_836.html).
pub const PEG_OFFSET_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegOffsetType",
    tag: 836,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_REDEMPTION_DATE <247>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_247.html).
pub const UNDERLYING_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingRedemptionDate",
    tag: 247,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_ACCRUED_INTEREST_AMT <540>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_540.html).
pub const TOTAL_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalAccruedInterestAmt",
    tag: 540,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_DESCRIPTOR_TYPE <399>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_399.html).
pub const BID_DESCRIPTOR_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidDescriptorType",
    tag: 399,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_EXCHANGE <308>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_308.html).
pub const UNDERLYING_SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityExchange",
    tag: 308,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`COUNTRY_OF_ISSUE <470>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_470.html).
pub const COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CountryOfIssue",
    tag: 470,
    data_type: FixDatatype::Country,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONCESSION <238>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_238.html).
pub const CONCESSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Concession",
    tag: 238,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`AFFECTED_ORDER_ID <535>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_535.html).
pub const AFFECTED_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AffectedOrderID",
    tag: 535,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_ID <902>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_902.html).
pub const COLL_ASGN_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnID",
    tag: 902,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`APPL_QUEUE_ACTION <815>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_815.html).
pub const APPL_QUEUE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ApplQueueAction",
    tag: 815,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_ALT_ID_SOURCE <606>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_606.html).
pub const LEG_SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityAltIDSource",
    tag: 606,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ISSUE_DATE <225>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_225.html).
pub const ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IssueDate",
    tag: 225,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIQUIDITY_NUM_SECURITIES <441>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_441.html).
pub const LIQUIDITY_NUM_SECURITIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LiquidityNumSecurities",
    tag: 441,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DEF_OFFER_SIZE <294>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_294.html).
pub const DEF_OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DefOfferSize",
    tag: 294,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTY_ID <448>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_448.html).
pub const PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PartyID",
    tag: 448,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE <593>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_593.html).
pub const UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStateOrProvinceOfIssue",
    tag: 593,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_ORDER_QTY <152>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_152.html).
pub const CASH_ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashOrderQty",
    tag: 152,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`DATED_DATE <873>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_873.html).
pub const DATED_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DatedDate",
    tag: 873,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_EVENTS <864>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_864.html).
pub const NO_EVENTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoEvents",
    tag: 864,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SUBJECT <147>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_147.html).
pub const SUBJECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Subject",
    tag: 147,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REQUEST_ID <568>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_568.html).
pub const TRADE_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeRequestID",
    tag: 568,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`GT_BOOKING_INST <427>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_427.html).
pub const GT_BOOKING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "GTBookingInst",
    tag: 427,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_AGENT_NAME <498>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_498.html).
pub const CASH_DISTRIB_AGENT_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribAgentName",
    tag: 498,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MISC_FEE_CURR <138>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_138.html).
pub const MISC_FEE_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MiscFeeCurr",
    tag: 138,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`MULTI_LEG_RPT_TYPE_REQ <563>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_563.html).
pub const MULTI_LEG_RPT_TYPE_REQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MultiLegRptTypeReq",
    tag: 563,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`REF_COMP_ID <930>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_930.html).
pub const REF_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefCompID",
    tag: 930,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_YIELD <634>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_634.html).
pub const OFFER_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferYield",
    tag: 634,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`RESET_SEQ_NUM_FLAG <141>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_141.html).
pub const RESET_SEQ_NUM_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ResetSeqNumFlag",
    tag: 141,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`COUNTRY <421>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_421.html).
pub const COUNTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Country",
    tag: 421,
    data_type: FixDatatype::Country,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_REQ_TYPE <724>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_724.html).
pub const POS_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosReqType",
    tag: 724,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PAYMENT_METHOD <492>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_492.html).
pub const PAYMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PaymentMethod",
    tag: 492,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_LAST_PX <637>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_637.html).
pub const LEG_LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegLastPx",
    tag: 637,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_PX <132>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_132.html).
pub const BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidPx",
    tag: 132,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`NEXT_EXPECTED_MSG_SEQ_NUM <789>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_789.html).
pub const NEXT_EXPECTED_MSG_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NextExpectedMsgSeqNum",
    tag: 789,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRARY_INSTRUCTION_INDICATOR <719>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_719.html).
pub const CONTRARY_INSTRUCTION_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraryInstructionIndicator",
    tag: 719,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_SETTL_INST_TYPE <780>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_780.html).
pub const ALLOC_SETTL_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocSettlInstType",
    tag: 780,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_FORWARD_POINTS <191>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_191.html).
pub const OFFER_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferForwardPoints",
    tag: 191,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`CROSS_ID <548>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_548.html).
pub const CROSS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CrossID",
    tag: 548,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_PRE_CLOSE_TIME <343>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_343.html).
pub const TRAD_SES_PRE_CLOSE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesPreCloseTime",
    tag: 343,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`PROG_RPT_REQS <414>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_414.html).
pub const PROG_RPT_REQS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ProgRptReqs",
    tag: 414,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_SCOPE <846>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_846.html).
pub const DISCRETION_SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionScope",
    tag: 846,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_CONTRACT_MULTIPLIER <614>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_614.html).
pub const LEG_CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegContractMultiplier",
    tag: 614,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`STAND_INST_DB_TYPE <169>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_169.html).
pub const STAND_INST_DB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StandInstDbType",
    tag: 169,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_REQUEST_TYPE <321>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_321.html).
pub const SECURITY_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityRequestType",
    tag: 321,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE_MULTI_LEG_REPORTING_TYPE <752>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_752.html).
pub const SIDE_MULTI_LEG_REPORTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SideMultiLegReportingType",
    tag: 752,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_NAME <392>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_392.html).
pub const LIST_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListName",
    tag: 392,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_TYPE <537>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_537.html).
pub const QUOTE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteType",
    tag: 537,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_MODE <160>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_160.html).
pub const SETTL_INST_MODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstMode",
    tag: 160,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALT_MD_SOURCE_ID <817>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_817.html).
pub const ALT_MD_SOURCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AltMDSourceID",
    tag: 817,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXERCISE_METHOD <747>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_747.html).
pub const EXERCISE_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExerciseMethod",
    tag: 747,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MARGIN_EXCESS <899>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_899.html).
pub const MARGIN_EXCESS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MarginExcess",
    tag: 899,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_INTERMED_REQ_TYPE <808>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_808.html).
pub const ALLOC_INTERMED_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocIntermedReqType",
    tag: 808,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURE_DATA <91>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_91.html).
pub const SECURE_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecureData",
    tag: 91,
    data_type: FixDatatype::Data,
    location: FieldLocation::Header,
};

/// Field attributes for [`REF_TAG_ID <371>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_371.html).
pub const REF_TAG_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefTagID",
    tag: 371,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MULTI_LEG_REPORTING_TYPE <442>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_442.html).
pub const MULTI_LEG_REPORTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MultiLegReportingType",
    tag: 442,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_RPTS <82>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_82.html).
pub const NO_RPTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoRpts",
    tag: 82,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`INTEREST_ACCRUAL_DATE <874>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_874.html).
pub const INTEREST_ACCRUAL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InterestAccrualDate",
    tag: 874,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_TRANS_TYPE <514>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_514.html).
pub const REGIST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistTransType",
    tag: 514,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_DLVY_INST <85>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_85.html).
pub const NO_DLVY_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoDlvyInst",
    tag: 85,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_UNDERLYING_SECURITY_DESC <365>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_365.html).
pub const ENCODED_UNDERLYING_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedUnderlyingSecurityDesc",
    tag: 365,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVERY_FORM <668>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_668.html).
pub const DELIVERY_FORM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliveryForm",
    tag: 668,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CONTRACT_MULTIPLIER <436>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_436.html).
pub const UNDERLYING_CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingContractMultiplier",
    tag: 436,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`OWNERSHIP_TYPE <517>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_517.html).
pub const OWNERSHIP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OwnershipType",
    tag: 517,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_PRICE_TYPE <663>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_663.html).
pub const BENCHMARK_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkPriceType",
    tag: 663,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TIME_BRACKET <943>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_943.html).
pub const TIME_BRACKET: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TimeBracket",
    tag: 943,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRIOR_SETTL_PRICE <734>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_734.html).
pub const PRIOR_SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriorSettlPrice",
    tag: 734,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_TRANS_TYPE <71>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_71.html).
pub const ALLOC_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocTransType",
    tag: 71,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`SELLER_DAYS <287>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_287.html).
pub const SELLER_DAYS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SellerDays",
    tag: 287,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_STRIKES <422>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_422.html).
pub const TOT_NO_STRIKES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoStrikes",
    tag: 422,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_MAINT_RPT_ID <721>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_721.html).
pub const POS_MAINT_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosMaintRptID",
    tag: 721,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXPIRE_TIME <126>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_126.html).
pub const EXPIRE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExpireTime",
    tag: 126,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORD_STATUS_REQ_ID <790>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_790.html).
pub const ORD_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrdStatusReqID",
    tag: 790,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SECURITY_TYPES <558>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_558.html).
pub const NO_SECURITY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSecurityTypes",
    tag: 558,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`REPORT_TO_EXCH <113>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_113.html).
pub const REPORT_TO_EXCH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ReportToExch",
    tag: 113,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_UPDATE_TYPE <265>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_265.html).
pub const MD_UPDATE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDUpdateType",
    tag: 265,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD_REDEMPTION_DATE <696>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_696.html).
pub const YIELD_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "YieldRedemptionDate",
    tag: 696,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADING_SESSION_ID <336>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_336.html).
pub const TRADING_SESSION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradingSessionID",
    tag: 336,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED_PARTY_SUB_ID <545>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_545.html).
pub const NESTED_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NestedPartySubID",
    tag: 545,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_QUALIFIER <104>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_104.html).
pub const IOI_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOIQualifier",
    tag: 104,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`POOL <691>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_691.html).
pub const POOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Pool",
    tag: 691,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_LIMIT_TYPE <843>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_843.html).
pub const DISCRETION_LIMIT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionLimitType",
    tag: 843,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_SUB_TYPE <762>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_762.html).
pub const SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecuritySubType",
    tag: 762,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`STOP_PX <99>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_99.html).
pub const STOP_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StopPx",
    tag: 99,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_PUT_OR_CALL <315>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_315.html).
pub const UNDERLYING_PUT_OR_CALL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingPutOrCall",
    tag: 315,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_RESPONSE_ID <322>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_322.html).
pub const SECURITY_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityResponseID",
    tag: 322,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOWABLE_ONE_SIDEDNESS_PCT <765>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_765.html).
pub const ALLOWABLE_ONE_SIDEDNESS_PCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllowableOneSidednessPct",
    tag: 765,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_REF_ID <26>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_26.html).
pub const IOI_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOIRefID",
    tag: 26,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`END_DATE <917>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_917.html).
pub const END_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EndDate",
    tag: 917,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_TRADE_REPORT_ID <818>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_818.html).
pub const SECONDARY_TRADE_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryTradeReportID",
    tag: 818,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_CAPACITY <29>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_29.html).
pub const LAST_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastCapacity",
    tag: 29,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_RPT_ID <908>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_908.html).
pub const COLL_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollRptID",
    tag: 908,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_ID <309>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_309.html).
pub const UNDERLYING_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityID",
    tag: 309,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BENCHMARK_CURVE_CURRENCY <676>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_676.html).
pub const LEG_BENCHMARK_CURVE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBenchmarkCurveCurrency",
    tag: 676,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`STATE_OR_PROVINCE_OF_ISSUE <471>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_471.html).
pub const STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StateOrProvinceOfIssue",
    tag: 471,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BUSINESS_REJECT_REASON <380>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_380.html).
pub const BUSINESS_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BusinessRejectReason",
    tag: 380,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_PRODUCT <607>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_607.html).
pub const LEG_PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegProduct",
    tag: 607,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_UNDERLYING_STIPS <887>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_887.html).
pub const NO_UNDERLYING_STIPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoUnderlyingStips",
    tag: 887,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_ACCRUED_INTEREST_AMT <742>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_742.html).
pub const ALLOC_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocAccruedInterestAmt",
    tag: 742,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CURRENCY <15>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_15.html).
pub const CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Currency",
    tag: 15,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_QUOTE_ENTRIES <295>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_295.html).
pub const NO_QUOTE_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoQuoteEntries",
    tag: 295,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_PRICE <366>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_366.html).
pub const ALLOC_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocPrice",
    tag: 366,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_STATUS <665>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_665.html).
pub const CONFIRM_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmStatus",
    tag: 665,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`AVG_PX <6>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_6.html).
pub const AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AvgPx",
    tag: 6,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_INQUIRY_STATUS <945>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_945.html).
pub const COLL_INQUIRY_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollInquiryStatus",
    tag: 945,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_SUB_ID <57>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_57.html).
pub const TARGET_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetSubID",
    tag: 57,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`NO_COMP_I_DS <936>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_936.html).
pub const NO_COMP_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoCompIDs",
    tag: 936,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_CLEARING_INSTRUCTIONS <576>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_576.html).
pub const NO_CLEARING_INSTRUCTIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoClearingInstructions",
    tag: 576,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_AGENT_CODE <499>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_499.html).
pub const CASH_DISTRIB_AGENT_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribAgentCode",
    tag: 499,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_REF_ID <280>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_280.html).
pub const MD_ENTRY_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryRefID",
    tag: 280,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_ID <48>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_48.html).
pub const SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityID",
    tag: 48,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MAX_SHOW <210>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_210.html).
pub const MAX_SHOW: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaxShow",
    tag: 210,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`CLEARING_FEE_INDICATOR <635>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_635.html).
pub const CLEARING_FEE_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClearingFeeIndicator",
    tag: 635,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DLVY_INST_TYPE <787>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_787.html).
pub const DLVY_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DlvyInstType",
    tag: 787,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_FORWARD_POINTS2 <642>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_642.html).
pub const BID_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidForwardPoints2",
    tag: 642,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`POSS_DUP_FLAG <43>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_43.html).
pub const POSS_DUP_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PossDupFlag",
    tag: 43,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Header,
};

/// Field attributes for [`XML_DATA <213>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_213.html).
pub const XML_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "XmlData",
    tag: 213,
    data_type: FixDatatype::Data,
    location: FieldLocation::Header,
};

/// Field attributes for [`REGIST_ACCT_TYPE <493>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_493.html).
pub const REGIST_ACCT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistAcctType",
    tag: 493,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`AGGREGATED_BOOK <266>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_266.html).
pub const AGGREGATED_BOOK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AggregatedBook",
    tag: 266,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`MKT_BID_PX <645>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_645.html).
pub const MKT_BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MktBidPx",
    tag: 645,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`MSG_SEQ_NUM <34>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_34.html).
pub const MSG_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MsgSeqNum",
    tag: 34,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Header,
};

/// Field attributes for [`REPORTED_PX <861>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_861.html).
pub const REPORTED_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ReportedPx",
    tag: 861,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_ID <37>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_37.html).
pub const ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderID",
    tag: 37,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PUBLISH_TRD_INDICATOR <852>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_852.html).
pub const PUBLISH_TRD_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PublishTrdIndicator",
    tag: 852,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_IOI_QUALIFIERS <199>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_199.html).
pub const NO_IOI_QUALIFIERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoIOIQualifiers",
    tag: 199,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_DESC <620>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_620.html).
pub const LEG_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityDesc",
    tag: 620,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PROG_PERIOD_INTERVAL <415>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_415.html).
pub const PROG_PERIOD_INTERVAL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ProgPeriodInterval",
    tag: 415,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BASIS_FEATURE_PRICE <260>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_260.html).
pub const BASIS_FEATURE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BasisFeaturePrice",
    tag: 260,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_REDEMPTION_DATE <254>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_254.html).
pub const LEG_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRedemptionDate",
    tag: 254,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_COUPON_RATE <615>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_615.html).
pub const LEG_COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCouponRate",
    tag: 615,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`AGREEMENT_CURRENCY <918>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_918.html).
pub const AGREEMENT_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AgreementCurrency",
    tag: 918,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_PRICE_TYPE <686>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_686.html).
pub const LEG_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegPriceType",
    tag: 686,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_COUPON_PAYMENT_DATE <241>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_241.html).
pub const UNDERLYING_COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCouponPaymentDate",
    tag: 241,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_RELATED_SYM <393>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_393.html).
pub const TOT_NO_RELATED_SYM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoRelatedSym",
    tag: 393,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_TYPE <310>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_310.html).
pub const UNDERLYING_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityType",
    tag: 310,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TEST_MESSAGE_INDICATOR <464>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_464.html).
pub const TEST_MESSAGE_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TestMessageIndicator",
    tag: 464,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_ALLOC_QTY <673>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_673.html).
pub const LEG_ALLOC_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegAllocQty",
    tag: 673,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_STIPULATIONS <232>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_232.html).
pub const NO_STIPULATIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoStipulations",
    tag: 232,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_STIP_VALUE <889>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_889.html).
pub const UNDERLYING_STIP_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStipValue",
    tag: 889,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_MATCH_ID <880>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_880.html).
pub const TRD_MATCH_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdMatchID",
    tag: 880,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REPURCHASE_RATE <227>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_227.html).
pub const REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RepurchaseRate",
    tag: 227,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONT_AMT_VALUE <520>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_520.html).
pub const CONT_AMT_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContAmtValue",
    tag: 520,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`STRIKE_TIME <443>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_443.html).
pub const STRIKE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StrikeTime",
    tag: 443,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_BUYER <288>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_288.html).
pub const MD_ENTRY_BUYER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryBuyer",
    tag: 288,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_NET_MONEY <154>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_154.html).
pub const ALLOC_NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocNetMoney",
    tag: 154,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_VALUATION_POINT <515>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_515.html).
pub const EXEC_VALUATION_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecValuationPoint",
    tag: 515,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_INQUIRY_RESULT <946>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_946.html).
pub const COLL_INQUIRY_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollInquiryResult",
    tag: 946,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_ORD_MOD_TIME <586>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_586.html).
pub const ORIG_ORD_MOD_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigOrdModTime",
    tag: 586,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`NUM_DAYS_INTEREST <157>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_157.html).
pub const NUM_DAYS_INTEREST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NumDaysInterest",
    tag: 157,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRA_TRADE_QTY <437>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_437.html).
pub const CONTRA_TRADE_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraTradeQty",
    tag: 437,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_REF_ID <508>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_508.html).
pub const REGIST_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistRefID",
    tag: 508,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DAY_BOOKING_INST <589>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_589.html).
pub const DAY_BOOKING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DayBookingInst",
    tag: 589,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`HEADLINE <148>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_148.html).
pub const HEADLINE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Headline",
    tag: 148,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PCT_AT_RISK <869>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_869.html).
pub const PCT_AT_RISK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PctAtRisk",
    tag: 869,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_QUOTE_QUALIFIERS <735>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_735.html).
pub const NO_QUOTE_QUALIFIERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoQuoteQualifiers",
    tag: 735,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_CANC_REPLACE_REASON <796>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_796.html).
pub const ALLOC_CANC_REPLACE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocCancReplaceReason",
    tag: 796,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_LOCATION_ID <143>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_143.html).
pub const TARGET_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetLocationID",
    tag: 143,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`LEG_POSITION_EFFECT <564>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_564.html).
pub const LEG_POSITION_EFFECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegPositionEffect",
    tag: 564,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRICE_TYPE <423>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_423.html).
pub const PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriceType",
    tag: 423,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DESIGNATION <494>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_494.html).
pub const DESIGNATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Designation",
    tag: 494,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_SIZE <134>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_134.html).
pub const BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidSize",
    tag: 134,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_LIST_REQUEST_TYPE <559>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_559.html).
pub const SECURITY_LIST_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityListRequestType",
    tag: 559,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_CAPACITIES <862>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_862.html).
pub const NO_CAPACITIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoCapacities",
    tag: 862,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`HOP_REF_ID <630>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_630.html).
pub const HOP_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HopRefID",
    tag: 630,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRA_TRADER <337>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_337.html).
pub const CONTRA_TRADER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraTrader",
    tag: 337,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LONG_QTY <704>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_704.html).
pub const LONG_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LongQty",
    tag: 704,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`VALUE_OF_FUTURES <408>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_408.html).
pub const VALUE_OF_FUTURES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ValueOfFutures",
    tag: 408,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_ISSUER <617>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_617.html).
pub const LEG_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegIssuer",
    tag: 617,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_REG_TIMESTAMP <769>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_769.html).
pub const TRD_REG_TIMESTAMP: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdRegTimestamp",
    tag: 769,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_SUB_TYPE <763>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_763.html).
pub const UNDERLYING_SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecuritySubType",
    tag: 763,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`AGREEMENT_DATE <915>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_915.html).
pub const AGREEMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AgreementDate",
    tag: 915,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_LEG_REF_ID <824>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_824.html).
pub const TRADE_LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeLegRefID",
    tag: 824,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`STAND_INST_DB_ID <171>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_171.html).
pub const STAND_INST_DB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StandInstDbID",
    tag: 171,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_RESPONSE_TYPE <323>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_323.html).
pub const SECURITY_RESPONSE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityResponseType",
    tag: 323,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_TYPE <394>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_394.html).
pub const BID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidType",
    tag: 394,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_ID <162>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_162.html).
pub const SETTL_INST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstID",
    tag: 162,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MATURITY_NET_MONEY <890>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_890.html).
pub const MATURITY_NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaturityNetMoney",
    tag: 890,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`MASS_CANCEL_REQUEST_TYPE <530>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_530.html).
pub const MASS_CANCEL_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MassCancelRequestType",
    tag: 530,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_SOURCE <165>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_165.html).
pub const SETTL_INST_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstSource",
    tag: 165,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`GROSS_TRADE_AMT <381>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_381.html).
pub const GROSS_TRADE_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "GrossTradeAmt",
    tag: 381,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NUM_TRADE_REPORTS <748>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_748.html).
pub const TOT_NUM_TRADE_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNumTradeReports",
    tag: 748,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_INST <388>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_388.html).
pub const DISCRETION_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionInst",
    tag: 388,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_AFFECTED_ORDERS <533>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_533.html).
pub const TOTAL_AFFECTED_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalAffectedOrders",
    tag: 533,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`APPL_QUEUE_DEPTH <813>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_813.html).
pub const APPL_QUEUE_DEPTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ApplQueueDepth",
    tag: 813,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVERY_DATE <743>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_743.html).
pub const DELIVERY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliveryDate",
    tag: 743,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_NESTED_PARTY_SUB_I_DS <804>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_804.html).
pub const NO_NESTED_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNestedPartySubIDs",
    tag: 804,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_STATUS <87>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_87.html).
pub const ALLOC_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocStatus",
    tag: 87,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_SET_VALID_UNTIL_TIME <367>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_367.html).
pub const QUOTE_SET_VALID_UNTIL_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteSetValidUntilTime",
    tag: 367,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRA_TRADE_TIME <438>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_438.html).
pub const CONTRA_TRADE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraTradeTime",
    tag: 438,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_ALLOCS <78>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_78.html).
pub const NO_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoAllocs",
    tag: 78,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_INSTR_ATTRIB <870>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_870.html).
pub const NO_INSTR_ATTRIB: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoInstrAttrib",
    tag: 870,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`MATCH_TYPE <574>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_574.html).
pub const MATCH_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MatchType",
    tag: 574,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SYMBOL_SFX <65>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_65.html).
pub const SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SymbolSfx",
    tag: 65,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_REQ_REJ_REASON <281>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_281.html).
pub const MD_REQ_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDReqRejReason",
    tag: 281,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MIN_OFFER_SIZE <648>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_648.html).
pub const MIN_OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MinOfferSize",
    tag: 648,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`INC_TAX_IND <416>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_416.html).
pub const INC_TAX_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IncTaxInd",
    tag: 416,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ROUND_LOT <561>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_561.html).
pub const ROUND_LOT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RoundLot",
    tag: 561,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURRENCY <120>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_120.html).
pub const SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrency",
    tag: 120,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_FORWARD_POINTS2 <643>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_643.html).
pub const OFFER_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferForwardPoints2",
    tag: 643,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_REQ_ID <859>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_859.html).
pub const CONFIRM_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmReqID",
    tag: 859,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_MAINT_RPT_REF_ID <714>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_714.html).
pub const POS_MAINT_RPT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosMaintRptRefID",
    tag: 714,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ON_BEHALF_OF_COMP_ID <115>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_115.html).
pub const ON_BEHALF_OF_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OnBehalfOfCompID",
    tag: 115,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`NO_MD_ENTRY_TYPES <267>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_267.html).
pub const NO_MD_ENTRY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoMDEntryTypes",
    tag: 267,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_METHOD <338>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_338.html).
pub const TRAD_SES_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesMethod",
    tag: 338,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_SESS_SUB_ID <717>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_717.html).
pub const SETTL_SESS_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlSessSubID",
    tag: 717,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ISSUER <106>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_106.html).
pub const ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Issuer",
    tag: 106,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_REG_TIMESTAMP_TYPE <770>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_770.html).
pub const TRD_REG_TIMESTAMP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdRegTimestampType",
    tag: 770,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`USER_REQUEST_TYPE <924>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_924.html).
pub const USER_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UserRequestType",
    tag: 924,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_PRICE_TYPE <692>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_692.html).
pub const QUOTE_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuotePriceType",
    tag: 692,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`HIGH_PX <332>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_332.html).
pub const HIGH_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HighPx",
    tag: 332,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`EX_DESTINATION <100>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_100.html).
pub const EX_DESTINATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExDestination",
    tag: 100,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_QTY <687>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_687.html).
pub const LEG_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegQty",
    tag: 687,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEGGED_PRICE <839>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_839.html).
pub const PEGGED_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PeggedPrice",
    tag: 839,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED2_PARTY_ID_SOURCE <758>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_758.html).
pub const NESTED2_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested2PartyIDSource",
    tag: 758,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_PX <31>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_31.html).
pub const LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastPx",
    tag: 31,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SYMBOL <311>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_311.html).
pub const UNDERLYING_SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSymbol",
    tag: 311,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_CONTRA_BROKERS <382>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_382.html).
pub const NO_CONTRA_BROKERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoContraBrokers",
    tag: 382,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ASSIGNMENT_UNIT <745>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_745.html).
pub const ASSIGNMENT_UNIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AssignmentUnit",
    tag: 745,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_ID_SOURCE <22>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_22.html).
pub const SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityIDSource",
    tag: 22,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_TRADES <897>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_897.html).
pub const NO_TRADES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoTrades",
    tag: 897,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`APPL_QUEUE_RESOLUTION <814>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_814.html).
pub const APPL_QUEUE_RESOLUTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ApplQueueResolution",
    tag: 814,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BODY_LENGTH <9>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_9.html).
pub const BODY_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BodyLength",
    tag: 9,
    data_type: FixDatatype::Length,
    location: FieldLocation::Header,
};

/// Field attributes for [`NO_NESTED3_PARTY_SUB_I_DS <952>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_952.html).
pub const NO_NESTED3_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNested3PartySubIDs",
    tag: 952,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_SELLER <289>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_289.html).
pub const MD_ENTRY_SELLER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntrySeller",
    tag: 289,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_COUNTRY_OF_ISSUE <592>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_592.html).
pub const UNDERLYING_COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCountryOfIssue",
    tag: 592,
    data_type: FixDatatype::Country,
    location: FieldLocation::Body,
};

/// Field attributes for [`NET_CHG_PREV_DAY <451>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_451.html).
pub const NET_CHG_PREV_DAY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetChgPrevDay",
    tag: 451,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_ALLOC_TEXT_LEN <360>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_360.html).
pub const ENCODED_ALLOC_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedAllocTextLen",
    tag: 360,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SETTL_TYPE <587>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_587.html).
pub const LEG_SETTL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSettlType",
    tag: 587,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`EVENT_PX <867>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_867.html).
pub const EVENT_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EventPx",
    tag: 867,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_REQUEST_REJECT_REASON <658>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_658.html).
pub const QUOTE_REQUEST_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteRequestRejectReason",
    tag: 658,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TIME_IN_FORCE <59>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_59.html).
pub const TIME_IN_FORCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TimeInForce",
    tag: 59,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_DTLS <509>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_509.html).
pub const REGIST_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistDtls",
    tag: 509,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_ORIGINATOR <282>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_282.html).
pub const MD_ENTRY_ORIGINATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryOriginator",
    tag: 282,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_ACCT_ID_SOURCE <661>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_661.html).
pub const ALLOC_ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocAcctIDSource",
    tag: 661,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SENDER_SUB_ID <50>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_50.html).
pub const SENDER_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SenderSubID",
    tag: 50,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`BENCHMARK_CURVE_CURRENCY <220>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_220.html).
pub const BENCHMARK_CURVE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkCurveCurrency",
    tag: 220,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_STRIKE_CURRENCY <941>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_941.html).
pub const UNDERLYING_STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStrikeCurrency",
    tag: 941,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUANTITY <53>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_53.html).
pub const QUANTITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Quantity",
    tag: 53,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NETWORK_RESPONSE_ID <932>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_932.html).
pub const NETWORK_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetworkResponseID",
    tag: 932,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_ROUTING_I_DS <215>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_215.html).
pub const NO_ROUTING_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoRoutingIDs",
    tag: 215,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`WORKING_INDICATOR <636>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_636.html).
pub const WORKING_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "WorkingIndicator",
    tag: 636,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`TAX_ADVANTAGE_TYPE <495>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_495.html).
pub const TAX_ADVANTAGE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TaxAdvantageType",
    tag: 495,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_CONDITION <276>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_276.html).
pub const QUOTE_CONDITION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteCondition",
    tag: 276,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRICE <44>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_44.html).
pub const PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Price",
    tag: 44,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`OPT_ATTRIBUTE <206>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_206.html).
pub const OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OptAttribute",
    tag: 206,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MID_PX <631>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_631.html).
pub const MID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MidPx",
    tag: 631,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_PARTY_ID_SOURCE <783>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_783.html).
pub const SETTL_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPartyIDSource",
    tag: 783,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_POSITIONS <702>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_702.html).
pub const NO_POSITIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoPositions",
    tag: 702,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORD_STATUS <39>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_39.html).
pub const ORD_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrdStatus",
    tag: 39,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_DATE2 <193>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_193.html).
pub const SETTL_DATE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlDate2",
    tag: 193,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIQUIDITY_IND_TYPE <409>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_409.html).
pub const LIQUIDITY_IND_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LiquidityIndType",
    tag: 409,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_REQ_ID <262>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_262.html).
pub const MD_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDReqID",
    tag: 262,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_STIPULATION_VALUE <689>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_689.html).
pub const LEG_STIPULATION_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegStipulationValue",
    tag: 689,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_COUPON_PAYMENT_DATE <248>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_248.html).
pub const LEG_COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCouponPaymentDate",
    tag: 248,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_MOVE_TYPE <841>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_841.html).
pub const DISCRETION_MOVE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionMoveType",
    tag: 841,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NUM_ASSIGNMENT_REPORTS <832>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_832.html).
pub const TOT_NUM_ASSIGNMENT_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNumAssignmentReports",
    tag: 832,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE <243>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_243.html).
pub const UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingRepoCollateralSecurityType",
    tag: 243,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`AFFECTED_SECONDARY_ORDER_ID <536>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_536.html).
pub const AFFECTED_SECONDARY_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AffectedSecondaryOrderID",
    tag: 536,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NUM_TICKETS <395>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_395.html).
pub const NUM_TICKETS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NumTickets",
    tag: 395,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_QUOTE_ENTRIES <304>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_304.html).
pub const TOT_NO_QUOTE_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoQuoteEntries",
    tag: 304,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BOOKING_REF_ID <466>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_466.html).
pub const BOOKING_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BookingRefID",
    tag: 466,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`STIPULATION_VALUE <234>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_234.html).
pub const STIPULATION_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StipulationValue",
    tag: 234,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MASS_CANCEL_RESPONSE <531>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_531.html).
pub const MASS_CANCEL_RESPONSE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MassCancelResponse",
    tag: 531,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MARGIN_RATIO <898>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_898.html).
pub const MARGIN_RATIO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MarginRatio",
    tag: 898,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRICE_DELTA <811>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_811.html).
pub const PRICE_DELTA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriceDelta",
    tag: 811,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_ID <602>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_602.html).
pub const LEG_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityID",
    tag: 602,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_TAKEDOWN <237>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_237.html).
pub const TOTAL_TAKEDOWN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalTakedown",
    tag: 237,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_OFFSET_VALUE <389>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_389.html).
pub const DISCRETION_OFFSET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionOffsetValue",
    tag: 389,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_POSITION_NO <290>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_290.html).
pub const MD_ENTRY_POSITION_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryPositionNo",
    tag: 290,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRODUCT <460>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_460.html).
pub const PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Product",
    tag: 460,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_ALT_ID <605>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_605.html).
pub const LEG_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityAltID",
    tag: 605,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`FACTOR <228>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_228.html).
pub const FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Factor",
    tag: 228,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CURRENT_VALUE <885>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_885.html).
pub const UNDERLYING_CURRENT_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCurrentValue",
    tag: 885,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CP_REG_TYPE <876>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_876.html).
pub const CP_REG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CPRegType",
    tag: 876,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ACCRUED_INTEREST_AMT <159>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_159.html).
pub const ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AccruedInterestAmt",
    tag: 159,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_PERCENT <516>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_516.html).
pub const ORDER_PERCENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderPercent",
    tag: 516,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_DISTRIB_INSTS <510>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_510.html).
pub const NO_DISTRIB_INSTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoDistribInsts",
    tag: 510,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_TYPE <150>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_150.html).
pub const EXEC_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecType",
    tag: 150,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ODD_LOT <575>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_575.html).
pub const ODD_LOT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OddLot",
    tag: 575,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_STRIKE_CURRENCY <942>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_942.html).
pub const LEG_STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegStrikeCurrency",
    tag: 942,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`CUST_ORDER_CAPACITY <582>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_582.html).
pub const CUST_ORDER_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CustOrderCapacity",
    tag: 582,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MISC_FEE_AMT <137>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_137.html).
pub const MISC_FEE_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MiscFeeAmt",
    tag: 137,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`NUM_BIDDERS <417>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_417.html).
pub const NUM_BIDDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NumBidders",
    tag: 417,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRIOR_SPREAD_INDICATOR <720>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_720.html).
pub const PRIOR_SPREAD_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriorSpreadIndicator",
    tag: 720,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`CARD_HOLDER_NAME <488>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_488.html).
pub const CARD_HOLDER_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CardHolderName",
    tag: 488,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MID_YIELD <633>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_633.html).
pub const MID_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MidYield",
    tag: 633,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVER_TO_COMP_ID <128>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_128.html).
pub const DELIVER_TO_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliverToCompID",
    tag: 128,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`SETTL_PARTY_SUB_ID <785>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_785.html).
pub const SETTL_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPartySubID",
    tag: 785,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CLEARING_BUSINESS_DATE <715>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_715.html).
pub const CLEARING_BUSINESS_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClearingBusinessDate",
    tag: 715,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`INDIVIDUAL_ALLOC_REJ_CODE <776>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_776.html).
pub const INDIVIDUAL_ALLOC_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IndividualAllocRejCode",
    tag: 776,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_MARGIN <544>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_544.html).
pub const CASH_MARGIN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashMargin",
    tag: 544,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_MODE <339>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_339.html).
pub const TRAD_SES_MODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesMode",
    tag: 339,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`WT_AVERAGE_LIQUIDITY <410>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_410.html).
pub const WT_AVERAGE_LIQUIDITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "WtAverageLiquidity",
    tag: 410,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_OFFSET_TYPE <842>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_842.html).
pub const DISCRETION_OFFSET_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionOffsetType",
    tag: 842,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_MATURITY_MONTH_YEAR <610>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_610.html).
pub const LEG_MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegMaturityMonthYear",
    tag: 610,
    data_type: FixDatatype::MonthYear,
    location: FieldLocation::Body,
};

/// Field attributes for [`LOW_PX <333>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_333.html).
pub const LOW_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LowPx",
    tag: 333,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_SUB_TYPE <764>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_764.html).
pub const LEG_SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecuritySubType",
    tag: 764,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIQUIDITY_VALUE <404>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_404.html).
pub const LIQUIDITY_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LiquidityValue",
    tag: 404,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_OPT_ATTRIBUTE <613>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_613.html).
pub const LEG_OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegOptAttribute",
    tag: 613,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_DELIVERY_TYPE <172>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_172.html).
pub const SETTL_DELIVERY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlDeliveryType",
    tag: 172,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_SUB_TYPE <829>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_829.html).
pub const TRD_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdSubType",
    tag: 829,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED2_PARTY_ROLE <759>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_759.html).
pub const NESTED2_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested2PartyRole",
    tag: 759,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NUM_REPORTS <911>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_911.html).
pub const TOT_NUM_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNumReports",
    tag: 911,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_LINK_ID <820>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_820.html).
pub const TRADE_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeLinkID",
    tag: 820,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_TYPE <167>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_167.html).
pub const SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityType",
    tag: 167,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MAX_MESSAGE_SIZE <383>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_383.html).
pub const MAX_MESSAGE_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaxMessageSize",
    tag: 383,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_ID <390>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_390.html).
pub const BID_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidID",
    tag: 390,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EMAIL_TYPE <94>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_94.html).
pub const EMAIL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EmailType",
    tag: 94,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_END_VALUE <886>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_886.html).
pub const UNDERLYING_END_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingEndValue",
    tag: 886,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_CL_ORD_ID <526>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_526.html).
pub const SECONDARY_CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryClOrdID",
    tag: 526,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PROCESS_CODE <81>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_81.html).
pub const PROCESS_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ProcessCode",
    tag: 81,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_ALLOC_TEXT <361>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_361.html).
pub const ENCODED_ALLOC_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedAllocText",
    tag: 361,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_ID <664>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_664.html).
pub const CONFIRM_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmID",
    tag: 664,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXPIRE_DATE <432>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_432.html).
pub const EXPIRE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExpireDate",
    tag: 432,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_ID <513>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_513.html).
pub const REGIST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistID",
    tag: 513,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REF_ALLOC_ID <72>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_72.html).
pub const REF_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefAllocID",
    tag: 72,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE_COMPLIANCE_ID <659>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_659.html).
pub const SIDE_COMPLIANCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SideComplianceID",
    tag: 659,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRD_RPT_STATUS <939>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_939.html).
pub const TRD_RPT_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TrdRptStatus",
    tag: 939,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_PRICE <730>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_730.html).
pub const SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPrice",
    tag: 730,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_SEQ_NO <67>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_67.html).
pub const LIST_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListSeqNo",
    tag: 67,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LOCATION_ID <283>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_283.html).
pub const LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LocationID",
    tag: 283,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_TRADE_TYPE <418>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_418.html).
pub const BID_TRADE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidTradeType",
    tag: 418,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SETTL_PRICE_TYPE <733>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_733.html).
pub const UNDERLYING_SETTL_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSettlPriceType",
    tag: 733,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_SENDING_TIME <122>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_122.html).
pub const ORIG_SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigSendingTime",
    tag: 122,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Header,
};

/// Field attributes for [`SETTL_PARTY_SUB_ID_TYPE <786>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_786.html).
pub const SETTL_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPartySubIDType",
    tag: 786,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PASSWORD <554>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_554.html).
pub const PASSWORD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Password",
    tag: 554,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_CONDITION <277>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_277.html).
pub const TRADE_CONDITION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeCondition",
    tag: 277,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`RFQ_REQ_ID <644>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_644.html).
pub const RFQ_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RFQReqID",
    tag: 644,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_ISSUER_LEN <348>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_348.html).
pub const ENCODED_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedIssuerLen",
    tag: 348,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_SECURITY_TYPES <557>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_557.html).
pub const TOT_NO_SECURITY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoSecurityTypes",
    tag: 557,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ON_BEHALF_OF_SUB_ID <116>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_116.html).
pub const ON_BEHALF_OF_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OnBehalfOfSubID",
    tag: 116,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`POS_TYPE <703>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_703.html).
pub const POS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosType",
    tag: 703,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_TRD_TYPE <855>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_855.html).
pub const SECONDARY_TRD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryTrdType",
    tag: 855,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_REQ_ID <710>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_710.html).
pub const POS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosReqID",
    tag: 710,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MAX_FLOOR <111>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_111.html).
pub const MAX_FLOOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaxFloor",
    tag: 111,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`SUBSCRIPTION_REQUEST_TYPE <263>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_263.html).
pub const SUBSCRIPTION_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SubscriptionRequestType",
    tag: 263,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADJUSTMENT <334>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_334.html).
pub const ADJUSTMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Adjustment",
    tag: 334,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_SECURITY_ID_SOURCE <761>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_761.html).
pub const BENCHMARK_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkSecurityIDSource",
    tag: 761,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CXL_REJ_REASON <102>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_102.html).
pub const CXL_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CxlRejReason",
    tag: 102,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`AGREEMENT_DESC <913>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_913.html).
pub const AGREEMENT_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AgreementDesc",
    tag: 913,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRANSFER_REASON <830>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_830.html).
pub const TRANSFER_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TransferReason",
    tag: 830,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_QLTY_IND <25>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_25.html).
pub const IOI_QLTY_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOIQltyInd",
    tag: 25,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_RESP_ID <904>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_904.html).
pub const COLL_RESP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollRespID",
    tag: 904,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_ID_SOURCE <305>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_305.html).
pub const UNDERLYING_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityIDSource",
    tag: 305,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_INDIVIDUAL_ALLOC_ID <672>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_672.html).
pub const LEG_INDIVIDUAL_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegIndividualAllocID",
    tag: 672,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`INDIVIDUAL_ALLOC_ID <467>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_467.html).
pub const INDIVIDUAL_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IndividualAllocID",
    tag: 467,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COMPLIANCE_ID <376>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_376.html).
pub const COMPLIANCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ComplianceID",
    tag: 376,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`END_SEQ_NO <16>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_16.html).
pub const END_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EndSeqNo",
    tag: 16,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_ID_SOURCE <603>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_603.html).
pub const LEG_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityIDSource",
    tag: 603,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_END_PRICE <883>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_883.html).
pub const UNDERLYING_END_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingEndPrice",
    tag: 883,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`INTEREST_AT_MATURITY <738>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_738.html).
pub const INTEREST_AT_MATURITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InterestAtMaturity",
    tag: 738,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CL_ORD_ID <11>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_11.html).
pub const CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClOrdID",
    tag: 11,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`FINANCIAL_STATUS <291>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_291.html).
pub const FINANCIAL_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "FinancialStatus",
    tag: 291,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`CFI_CODE <461>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_461.html).
pub const CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CFICode",
    tag: 461,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_UNDERLYING_ISSUER_LEN <362>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_362.html).
pub const ENCODED_UNDERLYING_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedUnderlyingIssuerLen",
    tag: 362,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_INTEREST_AT_MATURITY <741>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_741.html).
pub const ALLOC_INTEREST_AT_MATURITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocInterestAtMaturity",
    tag: 741,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADV_ID <2>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_2.html).
pub const ADV_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AdvId",
    tag: 2,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ADV_TRANS_TYPE <5>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_5.html).
pub const ADV_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AdvTransType",
    tag: 5,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_NESTED3_PARTY_I_DS <948>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_948.html).
pub const NO_NESTED3_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNested3PartyIDs",
    tag: 948,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SETTL_DATE <588>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_588.html).
pub const LEG_SETTL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSettlDate",
    tag: 588,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_EMAIL <511>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_511.html).
pub const REGIST_EMAIL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistEmail",
    tag: 511,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_SUBJECT_LEN <356>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_356.html).
pub const ENCODED_SUBJECT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedSubjectLen",
    tag: 356,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRANSACT_TIME <60>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_60.html).
pub const TRANSACT_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TransactTime",
    tag: 60,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_CURVE_POINT <222>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_222.html).
pub const BENCHMARK_CURVE_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkCurvePoint",
    tag: 222,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CL_ORD_LINK_ID <583>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_583.html).
pub const CL_ORD_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClOrdLinkID",
    tag: 583,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_AVG_PX <799>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_799.html).
pub const ORDER_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderAvgPx",
    tag: 799,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_REF_ID <654>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_654.html).
pub const LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRefID",
    tag: 654,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SYMBOL <55>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_55.html).
pub const SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Symbol",
    tag: 55,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_HANDL_INST <209>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_209.html).
pub const ALLOC_HANDL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocHandlInst",
    tag: 209,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CARD_NUMBER <489>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_489.html).
pub const CARD_NUMBER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CardNumber",
    tag: 489,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_ID <278>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_278.html).
pub const MD_ENTRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryID",
    tag: 278,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_FORWARD_POINTS2 <641>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_641.html).
pub const LAST_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastForwardPoints2",
    tag: 641,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`MATURITY_MONTH_YEAR <200>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_200.html).
pub const MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MaturityMonthYear",
    tag: 200,
    data_type: FixDatatype::MonthYear,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_NO_ORDERS_TYPE <857>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_857.html).
pub const ALLOC_NO_ORDERS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocNoOrdersType",
    tag: 857,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_LINES_OF_TEXT <33>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_33.html).
pub const NO_LINES_OF_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoLinesOfText",
    tag: 33,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_STRATEGY_PARAMETERS <848>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_848.html).
pub const TARGET_STRATEGY_PARAMETERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetStrategyParameters",
    tag: 848,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_FORWARD_POINTS <195>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_195.html).
pub const LAST_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastForwardPoints",
    tag: 195,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_EXCHANGE <616>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_616.html).
pub const LEG_SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityExchange",
    tag: 616,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXCHANGE_FOR_PHYSICAL <411>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_411.html).
pub const EXCHANGE_FOR_PHYSICAL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExchangeForPhysical",
    tag: 411,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CREDIT_RATING <256>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_256.html).
pub const UNDERLYING_CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCreditRating",
    tag: 256,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_REPO_COLLATERAL_SECURITY_TYPE <250>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_250.html).
pub const LEG_REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRepoCollateralSecurityType",
    tag: 250,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_MATURITY_DATE <611>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_611.html).
pub const LEG_MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegMaturityDate",
    tag: 611,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`AGREEMENT_ID <914>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_914.html).
pub const AGREEMENT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AgreementID",
    tag: 914,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXPIRATION_CYCLE <827>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_827.html).
pub const EXPIRATION_CYCLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExpirationCycle",
    tag: 827,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_IOI_QTY <682>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_682.html).
pub const LEG_IOI_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegIOIQty",
    tag: 682,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_FACTOR <253>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_253.html).
pub const LEG_FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegFactor",
    tag: 253,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`EFP_TRACKING_ERROR <405>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_405.html).
pub const EFP_TRACKING_ERROR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EFPTrackingError",
    tag: 405,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_ISSUER <306>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_306.html).
pub const UNDERLYING_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingIssuer",
    tag: 306,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PAYMENT_REF <476>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_476.html).
pub const PAYMENT_REF: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PaymentRef",
    tag: 476,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_REPURCHASE_TERM <244>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_244.html).
pub const UNDERLYING_REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingRepurchaseTerm",
    tag: 244,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_LIMIT_TYPE <837>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_837.html).
pub const PEG_LIMIT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegLimitType",
    tag: 837,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_ALLOCS <892>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_892.html).
pub const TOT_NO_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoAllocs",
    tag: 892,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`REPO_COLLATERAL_SECURITY_TYPE <239>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_239.html).
pub const REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RepoCollateralSecurityType",
    tag: 239,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MASS_CANCEL_REJECT_REASON <532>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_532.html).
pub const MASS_CANCEL_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MassCancelRejectReason",
    tag: 532,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`CLIENT_BID_ID <391>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_391.html).
pub const CLIENT_BID_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ClientBidID",
    tag: 391,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_REJECT_REASON <300>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_300.html).
pub const QUOTE_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteRejectReason",
    tag: 300,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_PRODUCT <462>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_462.html).
pub const UNDERLYING_PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingProduct",
    tag: 462,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`EX_DATE <230>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_230.html).
pub const EX_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExDate",
    tag: 230,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_EXEC_ID <527>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_527.html).
pub const SECONDARY_EXEC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryExecID",
    tag: 527,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED2_PARTY_SUB_ID_TYPE <807>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_807.html).
pub const NESTED2_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested2PartySubIDType",
    tag: 807,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_LOCALE_OF_ISSUE <598>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_598.html).
pub const LEG_LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegLocaleOfIssue",
    tag: 598,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_AVG_PX <153>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_153.html).
pub const ALLOC_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocAvgPx",
    tag: 153,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_EXEC_INST_TYPE <433>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_433.html).
pub const LIST_EXEC_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListExecInstType",
    tag: 433,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`PAYMENT_DATE <504>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_504.html).
pub const PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PaymentDate",
    tag: 504,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`MASS_STATUS_REQ_TYPE <585>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_585.html).
pub const MASS_STATUS_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MassStatusReqType",
    tag: 585,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ON_BEHALF_OF_LOCATION_ID <144>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_144.html).
pub const ON_BEHALF_OF_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OnBehalfOfLocationID",
    tag: 144,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`EVENT_TYPE <865>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_865.html).
pub const EVENT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EventType",
    tag: 865,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_PRICE_TYPE <731>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_731.html).
pub const SETTL_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPriceType",
    tag: 731,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_REQ_REJ_CODE <792>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_792.html).
pub const SETTL_INST_REQ_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstReqRejCode",
    tag: 792,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MISC_FEE_TYPE <139>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_139.html).
pub const MISC_FEE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MiscFeeType",
    tag: 139,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_REQUEST_RESULT <560>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_560.html).
pub const SECURITY_REQUEST_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityRequestResult",
    tag: 560,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BASIS_PX_TYPE <419>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_419.html).
pub const BASIS_PX_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BasisPxType",
    tag: 419,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`CARD_EXP_DATE <490>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_490.html).
pub const CARD_EXP_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CardExpDate",
    tag: 490,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_NATURAL_FLAG <130>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_130.html).
pub const IOI_NATURAL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOINaturalFlag",
    tag: 130,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_LEGS <555>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_555.html).
pub const NO_LEGS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoLegs",
    tag: 555,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SHARED_COMMISSION <858>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_858.html).
pub const SHARED_COMMISSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SharedCommission",
    tag: 858,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_TYPE <626>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_626.html).
pub const ALLOC_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocType",
    tag: 626,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_PX <133>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_133.html).
pub const OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferPx",
    tag: 133,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_ISSUER <349>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_349.html).
pub const ENCODED_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedIssuer",
    tag: 349,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_SESS_ID <716>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_716.html).
pub const SETTL_SESS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlSessID",
    tag: 716,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_PRICE_TYPE <484>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_484.html).
pub const EXEC_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecPriceType",
    tag: 484,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`HOP_SENDING_TIME <629>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_629.html).
pub const HOP_SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HopSendingTime",
    tag: 629,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_SPOT_RATE <188>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_188.html).
pub const BID_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidSpotRate",
    tag: 188,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_SETTL_PARTY_I_DS <781>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_781.html).
pub const NO_SETTL_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoSettlPartyIDs",
    tag: 781,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_UNDERLYINGS <711>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_711.html).
pub const NO_UNDERLYINGS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoUnderlyings",
    tag: 711,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`USER_STATUS_TEXT <927>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_927.html).
pub const USER_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UserStatusText",
    tag: 927,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONFIRM_REF_ID <772>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_772.html).
pub const CONFIRM_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ConfirmRefID",
    tag: 772,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_REQ_ID <335>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_335.html).
pub const TRAD_SES_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesReqID",
    tag: 335,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`FAIR_VALUE <406>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_406.html).
pub const FAIR_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "FairValue",
    tag: 406,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_ROUND_DIRECTION <838>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_838.html).
pub const PEG_ROUND_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegRoundDirection",
    tag: 838,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_MATURITY_DATE <542>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_542.html).
pub const UNDERLYING_MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingMaturityDate",
    tag: 542,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_TEXT <161>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_161.html).
pub const ALLOC_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocText",
    tag: 161,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SOLICITED_FLAG <377>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_377.html).
pub const SOLICITED_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SolicitedFlag",
    tag: 377,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`ASSIGNMENT_METHOD <744>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_744.html).
pub const ASSIGNMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AssignmentMethod",
    tag: 744,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_MSG_TYPES <384>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_384.html).
pub const NO_MSG_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoMsgTypes",
    tag: 384,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_RESTRICTIONS <529>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_529.html).
pub const ORDER_RESTRICTIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderRestrictions",
    tag: 529,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_REJ_CODE <88>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_88.html).
pub const ALLOC_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocRejCode",
    tag: 88,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_DATED_DATE <739>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_739.html).
pub const LEG_DATED_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegDatedDate",
    tag: 739,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_CONTRACT_SETTL_MONTH <955>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_955.html).
pub const LEG_CONTRACT_SETTL_MONTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegContractSettlMonth",
    tag: 955,
    data_type: FixDatatype::MonthYear,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_BOOKING_QTY <800>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_800.html).
pub const ORDER_BOOKING_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderBookingQty",
    tag: 800,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`RPT_SEQ <83>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_83.html).
pub const RPT_SEQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RptSeq",
    tag: 83,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_UNDERLYING_ISSUER <363>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_363.html).
pub const ENCODED_UNDERLYING_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedUnderlyingIssuer",
    tag: 363,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`CXL_REJ_RESPONSE_TO <434>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_434.html).
pub const CXL_REJ_RESPONSE_TO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CxlRejResponseTo",
    tag: 434,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`AVG_PX_PRECISION <74>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_74.html).
pub const AVG_PX_PRECISION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AvgPxPrecision",
    tag: 74,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`EVENT_DATE <866>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_866.html).
pub const EVENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EventDate",
    tag: 866,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`PREVIOUSLY_REPORTED <570>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_570.html).
pub const PREVIOUSLY_REPORTED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PreviouslyReported",
    tag: 570,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`POSITION_EFFECT <77>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_77.html).
pub const POSITION_EFFECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PositionEffect",
    tag: 77,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_SUBJECT <357>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_357.html).
pub const ENCODED_SUBJECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedSubject",
    tag: 357,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`ACCT_ID_SOURCE <660>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_660.html).
pub const ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AcctIDSource",
    tag: 660,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_STRIKES <428>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_428.html).
pub const NO_STRIKES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoStrikes",
    tag: 428,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`MATCH_STATUS <573>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_573.html).
pub const MATCH_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MatchStatus",
    tag: 573,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOT_NO_ORDERS <68>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_68.html).
pub const TOT_NO_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotNoOrders",
    tag: 68,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CONTRA_LEG_REF_ID <655>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_655.html).
pub const CONTRA_LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ContraLegRefID",
    tag: 655,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NETWORK_REQUEST_TYPE <935>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_935.html).
pub const NETWORK_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetworkRequestType",
    tag: 935,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`RESPONSE_DESTINATION <726>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_726.html).
pub const RESPONSE_DESTINATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ResponseDestination",
    tag: 726,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DK_REASON <127>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_127.html).
pub const DK_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DKReason",
    tag: 127,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_UPDATE_ACTION <279>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_279.html).
pub const MD_UPDATE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDUpdateAction",
    tag: 279,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_SECURITY_DESC_LEN <350>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_350.html).
pub const ENCODED_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedSecurityDescLen",
    tag: 350,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_POS_REQ_REF_ID <713>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_713.html).
pub const ORIG_POS_REQ_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigPosReqRefID",
    tag: 713,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NET_MONEY <118>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_118.html).
pub const NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetMoney",
    tag: 118,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_PARTY_ID <782>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_782.html).
pub const SETTL_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlPartyID",
    tag: 782,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CROSS_PRIORITIZATION <550>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_550.html).
pub const CROSS_PRIORITIZATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CrossPrioritization",
    tag: 550,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`END_ACCRUED_INTEREST_AMT <920>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_920.html).
pub const END_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EndAccruedInterestAmt",
    tag: 920,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_CREDIT_RATING <257>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_257.html).
pub const LEG_CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCreditRating",
    tag: 257,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_STIPULATION_TYPE <688>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_688.html).
pub const LEG_STIPULATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegStipulationType",
    tag: 688,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`IN_VIEW_OF_COMMON <328>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_328.html).
pub const IN_VIEW_OF_COMMON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InViewOfCommon",
    tag: 328,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`RAW_DATA <96>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_96.html).
pub const RAW_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RawData",
    tag: 96,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_LEG_STIPULATIONS <683>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_683.html).
pub const NO_LEG_STIPULATIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoLegStipulations",
    tag: 683,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_MOVE_TYPE <835>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_835.html).
pub const PEG_MOVE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegMoveType",
    tag: 835,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`AUTO_ACCEPT_INDICATOR <754>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_754.html).
pub const AUTO_ACCEPT_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AutoAcceptIndicator",
    tag: 754,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_QTY <27>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_27.html).
pub const IOI_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOIQty",
    tag: 27,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_DESC <307>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_307.html).
pub const UNDERLYING_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityDesc",
    tag: 307,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISTRIB_PAYMENT_METHOD <477>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_477.html).
pub const DISTRIB_PAYMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DistribPaymentMethod",
    tag: 477,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_RESTATEMENT_REASON <378>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_378.html).
pub const EXEC_RESTATEMENT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecRestatementReason",
    tag: 378,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED2_PARTY_ID <757>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_757.html).
pub const NESTED2_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested2PartyID",
    tag: 757,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXEC_INST <18>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_18.html).
pub const EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecInst",
    tag: 18,
    data_type: FixDatatype::MultipleCharValue,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_INQUIRY_ID <909>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_909.html).
pub const COLL_INQUIRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollInquiryID",
    tag: 909,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_PX <810>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_810.html).
pub const UNDERLYING_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingPx",
    tag: 810,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`HANDL_INST <21>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_21.html).
pub const HANDL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HandlInst",
    tag: 21,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_NET_VALUE <900>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_900.html).
pub const TOTAL_NET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalNetValue",
    tag: 900,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_RESPONSE_LEVEL <301>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_301.html).
pub const QUOTE_RESPONSE_LEVEL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteResponseLevel",
    tag: 301,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_LEG_SECURITY_ALT_ID <604>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_604.html).
pub const NO_LEG_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoLegSecurityAltID",
    tag: 604,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CFI_CODE <463>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_463.html).
pub const UNDERLYING_CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCFICode",
    tag: 463,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REF_MSG_TYPE <372>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_372.html).
pub const REF_MSG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefMsgType",
    tag: 372,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COMMISSION <12>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_12.html).
pub const COMMISSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Commission",
    tag: 12,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_INSTR_REGISTRY <599>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_599.html).
pub const LEG_INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegInstrRegistry",
    tag: 599,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_QTY <879>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_879.html).
pub const UNDERLYING_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingQty",
    tag: 879,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_LEG_ALLOCS <670>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_670.html).
pub const NO_LEG_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoLegAllocs",
    tag: 670,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`BEGIN_SEQ_NO <7>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_7.html).
pub const BEGIN_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BeginSeqNo",
    tag: 7,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`PAYMENT_REMITTER_ID <505>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_505.html).
pub const PAYMENT_REMITTER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PaymentRemitterID",
    tag: 505,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_HEADLINE_LEN <358>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_358.html).
pub const ENCODED_HEADLINE_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedHeadlineLen",
    tag: 358,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURR_OFFER_FX_RATE <657>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_657.html).
pub const SETTL_CURR_OFFER_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrOfferFxRate",
    tag: 657,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`VALID_UNTIL_TIME <62>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_62.html).
pub const VALID_UNTIL_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ValidUntilTime",
    tag: 62,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`ROUTING_TYPE <216>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_216.html).
pub const ROUTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RoutingType",
    tag: 216,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NETWORK_STATUS_RESPONSE_TYPE <937>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_937.html).
pub const NETWORK_STATUS_RESPONSE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetworkStatusResponseType",
    tag: 937,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SENDER_COMP_ID <49>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_49.html).
pub const SENDER_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SenderCompID",
    tag: 49,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`STATUS_VALUE <928>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_928.html).
pub const STATUS_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StatusValue",
    tag: 928,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PEG_OFFSET_VALUE <211>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_211.html).
pub const PEG_OFFSET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PegOffsetValue",
    tag: 211,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_YIELD <632>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_632.html).
pub const BID_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidYield",
    tag: 632,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`CARD_ISS_NUM <491>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_491.html).
pub const CARD_ISS_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CardIssNum",
    tag: 491,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_DATE <272>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_272.html).
pub const MD_ENTRY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryDate",
    tag: 272,
    data_type: FixDatatype::UtcDateOnly,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORD_TYPE <40>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_40.html).
pub const ORD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrdType",
    tag: 40,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`STRIKE_PRICE <202>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_202.html).
pub const STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StrikePrice",
    tag: 202,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_HOPS <627>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_627.html).
pub const NO_HOPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoHops",
    tag: 627,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_UPDATE_TIME <779>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_779.html).
pub const LAST_UPDATE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastUpdateTime",
    tag: 779,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD_REDEMPTION_PRICE_TYPE <698>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_698.html).
pub const YIELD_REDEMPTION_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "YieldRedemptionPriceType",
    tag: 698,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MSG_TYPE <35>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_35.html).
pub const MSG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MsgType",
    tag: 35,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`EXEC_PRICE_ADJUSTMENT <485>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_485.html).
pub const EXEC_PRICE_ADJUSTMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExecPriceAdjustment",
    tag: 485,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADED_FLAT_SWITCH <258>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_258.html).
pub const TRADED_FLAT_SWITCH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradedFlatSwitch",
    tag: 258,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD_CALC_DATE <701>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_701.html).
pub const YIELD_CALC_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "YieldCalcDate",
    tag: 701,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_LINK_ID <196>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_196.html).
pub const ALLOC_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocLinkID",
    tag: 196,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SHORT_SALE_REASON <853>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_853.html).
pub const SHORT_SALE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ShortSaleReason",
    tag: 853,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISCRETION_ROUND_DIRECTION <844>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_844.html).
pub const DISCRETION_ROUND_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DiscretionRoundDirection",
    tag: 844,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CREDIT_RATING <255>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_255.html).
pub const CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CreditRating",
    tag: 255,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_STRIKE_PRICE <612>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_612.html).
pub const LEG_STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegStrikePrice",
    tag: 612,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`OUTSIDE_INDEX_PCT <407>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_407.html).
pub const OUTSIDE_INDEX_PCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OutsideIndexPct",
    tag: 407,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_STRIKE_PRICE <316>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_316.html).
pub const UNDERLYING_STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStrikePrice",
    tag: 316,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_CURR <478>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_478.html).
pub const CASH_DISTRIB_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribCurr",
    tag: 478,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_FACTOR <246>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_246.html).
pub const UNDERLYING_FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingFactor",
    tag: 246,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`INSTR_REGISTRY <543>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_543.html).
pub const INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InstrRegistry",
    tag: 543,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_STATUS <910>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_910.html).
pub const COLL_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollStatus",
    tag: 910,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_TRADING_SESSION_SUB_ID <823>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_823.html).
pub const UNDERLYING_TRADING_SESSION_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingTradingSessionSubID",
    tag: 823,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BENCHMARK_CURVE_POINT <678>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_678.html).
pub const LEG_BENCHMARK_CURVE_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBenchmarkCurvePoint",
    tag: 678,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`STIPULATION_TYPE <233>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_233.html).
pub const STIPULATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StipulationType",
    tag: 233,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`MSG_DIRECTION <385>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_385.html).
pub const MSG_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MsgDirection",
    tag: 385,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_SET_ID <302>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_302.html).
pub const QUOTE_SET_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteSetID",
    tag: 302,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_ALT_ID_SOURCE <456>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_456.html).
pub const SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityAltIDSource",
    tag: 456,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SYMBOL_SFX <601>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_601.html).
pub const LEG_SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSymbolSfx",
    tag: 601,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COUPON_PAYMENT_DATE <224>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_224.html).
pub const COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CouponPaymentDate",
    tag: 224,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_TRADE_REPORT_REF_ID <881>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_881.html).
pub const SECONDARY_TRADE_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryTradeReportRefID",
    tag: 881,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`INSTR_ATTRIB_VALUE <872>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_872.html).
pub const INSTR_ATTRIB_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "InstrAttribValue",
    tag: 872,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURR_FX_RATE <155>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_155.html).
pub const SETTL_CURR_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrFxRate",
    tag: 155,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`DISTRIB_PERCENTAGE <512>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_512.html).
pub const DISTRIB_PERCENTAGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DistribPercentage",
    tag: 512,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_COUPON_RATE <435>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_435.html).
pub const UNDERLYING_COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCouponRate",
    tag: 435,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_STATUS <506>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_506.html).
pub const REGIST_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistStatus",
    tag: 506,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_RELATED_SYM <146>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_146.html).
pub const NO_RELATED_SYM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoRelatedSym",
    tag: 146,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REPORT_ID <571>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_571.html).
pub const TRADE_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeReportID",
    tag: 571,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_COLL_INQUIRY_QUALIFIER <938>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_938.html).
pub const NO_COLL_INQUIRY_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoCollInquiryQualifier",
    tag: 938,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_INPUT_SOURCE <578>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_578.html).
pub const TRADE_INPUT_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeInputSource",
    tag: 578,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`URL_LINK <149>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_149.html).
pub const URL_LINK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "URLLink",
    tag: 149,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_STATUS_TYPE <429>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_429.html).
pub const LIST_STATUS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListStatusType",
    tag: 429,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SETTL_PRICE <732>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_732.html).
pub const UNDERLYING_SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSettlPrice",
    tag: 732,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_AGENT_ACCT_NUMBER <500>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_500.html).
pub const CASH_DISTRIB_AGENT_ACCT_NUMBER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribAgentAcctNumber",
    tag: 500,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ACCOUNT_TYPE <581>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_581.html).
pub const ACCOUNT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AccountType",
    tag: 581,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PREV_CLOSE_PX <140>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_140.html).
pub const PREV_CLOSE_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PrevClosePx",
    tag: 140,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`COPY_MSG_INDICATOR <797>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_797.html).
pub const COPY_MSG_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CopyMsgIndicator",
    tag: 797,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_NUM_POS_REPORTS <727>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_727.html).
pub const TOTAL_NUM_POS_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalNumPosReports",
    tag: 727,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TERMINATION_TYPE <788>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_788.html).
pub const TERMINATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TerminationType",
    tag: 788,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_SIZE <135>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_135.html).
pub const OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferSize",
    tag: 135,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_CURRENCY <556>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_556.html).
pub const LEG_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegCurrency",
    tag: 556,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_SECURITY_DESC <351>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_351.html).
pub const ENCODED_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedSecurityDesc",
    tag: 351,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`DATE_OF_BIRTH <486>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_486.html).
pub const DATE_OF_BIRTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DateOfBirth",
    tag: 486,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`OFFER_SPOT_RATE <190>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_190.html).
pub const OFFER_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OfferSpotRate",
    tag: 190,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_CROSS_ID <551>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_551.html).
pub const ORIG_CROSS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigCrossID",
    tag: 551,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QTY_TYPE <854>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_854.html).
pub const QTY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QtyType",
    tag: 854,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LEG_SECURITY_DESC <622>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_622.html).
pub const ENCODED_LEG_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedLegSecurityDesc",
    tag: 622,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`DUE_TO_RELATED <329>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_329.html).
pub const DUE_TO_RELATED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DueToRelated",
    tag: 329,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED2_PARTY_SUB_ID <760>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_760.html).
pub const NESTED2_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested2PartySubID",
    tag: 760,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_DESCRIPTOR <400>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_400.html).
pub const BID_DESCRIPTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidDescriptor",
    tag: 400,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_SECURITY_TYPE <609>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_609.html).
pub const LEG_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegSecurityType",
    tag: 609,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EFFECTIVE_TIME <168>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_168.html).
pub const EFFECTIVE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EffectiveTime",
    tag: 168,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`EXCHANGE_RULE <825>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_825.html).
pub const EXCHANGE_RULE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ExchangeRule",
    tag: 825,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_REPORT_ID <755>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_755.html).
pub const ALLOC_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocReportID",
    tag: 755,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_REF_ID <907>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_907.html).
pub const COLL_ASGN_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnRefID",
    tag: 907,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_ALT_MD_SOURCE <816>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_816.html).
pub const NO_ALT_MD_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoAltMDSource",
    tag: 816,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_TRANS_TYPE <163>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_163.html).
pub const SETTL_INST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstTransType",
    tag: 163,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`BUSINESS_REJECT_REF_ID <379>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_379.html).
pub const BUSINESS_REJECT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BusinessRejectRefID",
    tag: 379,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_TRADING_SESSIONS <386>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_386.html).
pub const NO_TRADING_SESSIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoTradingSessions",
    tag: 386,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURE_DATA_LEN <90>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_90.html).
pub const SECURE_DATA_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecureDataLen",
    tag: 90,
    data_type: FixDatatype::Length,
    location: FieldLocation::Header,
};

/// Field attributes for [`UNDERLYING_DIRTY_PRICE <882>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_882.html).
pub const UNDERLYING_DIRTY_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingDirtyPrice",
    tag: 882,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`OWNER_TYPE <522>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_522.html).
pub const OWNER_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OwnerType",
    tag: 522,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIGNATURE_LENGTH <93>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_93.html).
pub const SIGNATURE_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SignatureLength",
    tag: 93,
    data_type: FixDatatype::Length,
    location: FieldLocation::Trailer,
};

/// Field attributes for [`SESSION_REJECT_REASON <373>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_373.html).
pub const SESSION_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SessionRejectReason",
    tag: 373,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_POOL <740>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_740.html).
pub const LEG_POOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegPool",
    tag: 740,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_STATUS_TEXT <444>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_444.html).
pub const LIST_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListStatusText",
    tag: 444,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED_PARTY_ID_SOURCE <525>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_525.html).
pub const NESTED_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NestedPartyIDSource",
    tag: 525,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`CXL_QTY <84>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_84.html).
pub const CXL_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CxlQty",
    tag: 84,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED_PARTY_SUB_ID_TYPE <805>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_805.html).
pub const NESTED_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NestedPartySubIDType",
    tag: 805,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_ALLOC_ACCOUNT <671>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_671.html).
pub const LEG_ALLOC_ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegAllocAccount",
    tag: 671,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED3_PARTY_ROLE <951>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_951.html).
pub const NESTED3_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested3PartyRole",
    tag: 951,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_ACCOUNT <79>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_79.html).
pub const ALLOC_ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocAccount",
    tag: 79,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_HEADLINE <359>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_359.html).
pub const ENCODED_HEADLINE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedHeadline",
    tag: 359,
    data_type: FixDatatype::Data,
    location: FieldLocation::Body,
};

/// Field attributes for [`NET_GROSS_IND <430>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_430.html).
pub const NET_GROSS_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetGrossInd",
    tag: 430,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_REQ_STATUS <729>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_729.html).
pub const POS_REQ_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosReqStatus",
    tag: 729,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_ID <70>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_70.html).
pub const ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocID",
    tag: 70,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_ACCOUNT_TYPE <798>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_798.html).
pub const ALLOC_ACCOUNT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocAccountType",
    tag: 798,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_PRICE <566>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_566.html).
pub const LEG_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegPrice",
    tag: 566,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`FOREX_REQ <121>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_121.html).
pub const FOREX_REQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ForexReq",
    tag: 121,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`MD_ENTRY_TIME <273>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_273.html).
pub const MD_ENTRY_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MDEntryTime",
    tag: 273,
    data_type: FixDatatype::UtcTimeOnly,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRICE2 <640>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_640.html).
pub const PRICE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Price2",
    tag: 640,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRAD_SES_CLOSE_TIME <344>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_344.html).
pub const TRAD_SES_CLOSE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesCloseTime",
    tag: 344,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`USERNAME <553>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_553.html).
pub const USERNAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Username",
    tag: 553,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TEST_REQ_ID <112>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_112.html).
pub const TEST_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TestReqID",
    tag: 112,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_SECURITY_ID <699>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_699.html).
pub const BENCHMARK_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkSecurityID",
    tag: 699,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_LIQUIDITY_IND <851>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_851.html).
pub const LAST_LIQUIDITY_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastLiquidityInd",
    tag: 851,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_QTY_STATUS <706>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_706.html).
pub const POS_QTY_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosQtyStatus",
    tag: 706,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_DESC <107>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_107.html).
pub const SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityDesc",
    tag: 107,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`BASIS_FEATURE_DATE <259>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_259.html).
pub const BASIS_FEATURE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BasisFeatureDate",
    tag: 259,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`BUY_VOLUME <330>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_330.html).
pub const BUY_VOLUME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BuyVolume",
    tag: 330,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_TRANS_TYPE <709>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_709.html).
pub const POS_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosTransType",
    tag: 709,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCRYPT_METHOD <98>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_98.html).
pub const ENCRYPT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncryptMethod",
    tag: 98,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NEW_PASSWORD <925>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_925.html).
pub const NEW_PASSWORD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NewPassword",
    tag: 925,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_ALLOC_INDICATOR <826>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_826.html).
pub const TRADE_ALLOC_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeAllocIndicator",
    tag: 826,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`START_DATE <916>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_916.html).
pub const START_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StartDate",
    tag: 916,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_OPT_ATTRIBUTE <317>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_317.html).
pub const UNDERLYING_OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingOptAttribute",
    tag: 317,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_OFFER_PX <684>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_684.html).
pub const LEG_OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegOfferPx",
    tag: 684,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`COMM_CURRENCY <479>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_479.html).
pub const COMM_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CommCurrency",
    tag: 479,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_STATUS_REQ_ID <324>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_324.html).
pub const SECURITY_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityStatusReqID",
    tag: 324,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOI_TRANS_TYPE <28>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_28.html).
pub const IOI_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOITransType",
    tag: 28,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BENCHMARK_PRICE <679>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_679.html).
pub const LEG_BENCHMARK_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBenchmarkPrice",
    tag: 679,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ASGN_REASON <895>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_895.html).
pub const COLL_ASGN_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAsgnReason",
    tag: 895,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REQUEST_STATUS <750>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_750.html).
pub const TRADE_REQUEST_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeRequestStatus",
    tag: 750,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`IOIID <23>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_23.html).
pub const IOIID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "IOIID",
    tag: 23,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_REQUEST_TYPE <303>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_303.html).
pub const QUOTE_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteRequestType",
    tag: 303,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_UNDERLYING_SECURITY_ALT_ID <457>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_457.html).
pub const NO_UNDERLYING_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoUnderlyingSecurityAltID",
    tag: 457,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`BID_REQUEST_TRANS_TYPE <374>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_374.html).
pub const BID_REQUEST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BidRequestTransType",
    tag: 374,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_SETTL_CURR_AMT <737>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_737.html).
pub const ALLOC_SETTL_CURR_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocSettlCurrAmt",
    tag: 737,
    data_type: FixDatatype::Amt,
    location: FieldLocation::Body,
};

/// Field attributes for [`CUM_QTY <14>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_14.html).
pub const CUM_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CumQty",
    tag: 14,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED3_PARTY_SUB_ID <953>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_953.html).
pub const NESTED3_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested3PartySubID",
    tag: 953,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_NESTED2_PARTY_SUB_I_DS <806>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_806.html).
pub const NO_NESTED2_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoNested2PartySubIDs",
    tag: 806,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`ACCOUNT <1>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_1.html).
pub const ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Account",
    tag: 1,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`COLL_ACTION <944>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_944.html).
pub const COLL_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CollAction",
    tag: 944,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`MASS_STATUS_REQ_ID <584>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_584.html).
pub const MASS_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "MassStatusReqID",
    tag: 584,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REGIST_REJ_REASON_CODE <507>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_507.html).
pub const REGIST_REJ_REASON_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RegistRejReasonCode",
    tag: 507,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LIST_EXEC_INST_LEN <352>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_352.html).
pub const ENCODED_LIST_EXEC_INST_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedListExecInstLen",
    tag: 352,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`TARGET_COMP_ID <56>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_56.html).
pub const TARGET_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TargetCompID",
    tag: 56,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`SPREAD <218>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_218.html).
pub const SPREAD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Spread",
    tag: 218,
    data_type: FixDatatype::PriceOffset,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_INPUT_DEVICE <579>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_579.html).
pub const TRADE_INPUT_DEVICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeInputDevice",
    tag: 579,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ALLOC_REPORT_REF_ID <795>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_795.html).
pub const ALLOC_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AllocReportRefID",
    tag: 795,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEGAL_CONFIRM <650>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_650.html).
pub const LEGAL_CONFIRM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegalConfirm",
    tag: 650,
    data_type: FixDatatype::Boolean,
    location: FieldLocation::Body,
};

/// Field attributes for [`BENCHMARK_CURVE_NAME <221>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_221.html).
pub const BENCHMARK_CURVE_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BenchmarkCurveName",
    tag: 221,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_PAY_REF <501>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_501.html).
pub const CASH_DISTRIB_PAY_REF: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribPayRef",
    tag: 501,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TICK_DIRECTION <274>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_274.html).
pub const TICK_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TickDirection",
    tag: 274,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORIG_TIME <42>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_42.html).
pub const ORIG_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrigTime",
    tag: 42,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`XML_DATA_LEN <212>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_212.html).
pub const XML_DATA_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "XmlDataLen",
    tag: 212,
    data_type: FixDatatype::Length,
    location: FieldLocation::Header,
};

/// Field attributes for [`NETWORK_REQUEST_ID <933>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_933.html).
pub const NETWORK_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NetworkRequestID",
    tag: 933,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REF_SEQ_NUM <45>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_45.html).
pub const REF_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RefSeqNum",
    tag: 45,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`AVG_PAR_PX <860>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_860.html).
pub const AVG_PAR_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AvgParPx",
    tag: 860,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECURITY_EXCHANGE <207>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_207.html).
pub const SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecurityExchange",
    tag: 207,
    data_type: FixDatatype::Exchange,
    location: FieldLocation::Body,
};

/// Field attributes for [`HOP_COMP_ID <628>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_628.html).
pub const HOP_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "HopCompID",
    tag: 628,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REPORT_TRANS_TYPE <487>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_487.html).
pub const TRADE_REPORT_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeReportTransType",
    tag: 487,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_MD_ENTRIES <268>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_268.html).
pub const NO_MD_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoMDEntries",
    tag: 268,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`NEW_SEQ_NO <36>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_36.html).
pub const NEW_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NewSeqNo",
    tag: 36,
    data_type: FixDatatype::SeqNum,
    location: FieldLocation::Body,
};

/// Field attributes for [`SECONDARY_ORDER_ID <198>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_198.html).
pub const SECONDARY_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SecondaryOrderID",
    tag: 198,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_RATIO_QTY <623>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_623.html).
pub const LEG_RATIO_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegRatioQty",
    tag: 623,
    data_type: FixDatatype::Float,
    location: FieldLocation::Body,
};

/// Field attributes for [`USER_STATUS <926>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_926.html).
pub const USER_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UserStatus",
    tag: 926,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`BOOKING_TYPE <775>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_775.html).
pub const BOOKING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "BookingType",
    tag: 775,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`QUOTE_RESP_TYPE <694>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_694.html).
pub const QUOTE_RESP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "QuoteRespType",
    tag: 694,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_ISSUE_DATE <249>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_249.html).
pub const LEG_ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegIssueDate",
    tag: 249,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`SIDE_VALUE_IND <401>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_401.html).
pub const SIDE_VALUE_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SideValueInd",
    tag: 401,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CURRENCY <318>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_318.html).
pub const UNDERLYING_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCurrency",
    tag: 318,
    data_type: FixDatatype::Currency,
    location: FieldLocation::Body,
};

/// Field attributes for [`LOCALE_OF_ISSUE <472>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_472.html).
pub const LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LocaleOfIssue",
    tag: 472,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_BID_PX <681>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_681.html).
pub const LEG_BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegBidPx",
    tag: 681,
    data_type: FixDatatype::Price,
    location: FieldLocation::Body,
};

/// Field attributes for [`REDEMPTION_DATE <240>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_240.html).
pub const REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RedemptionDate",
    tag: 240,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`ASGN_RPT_ID <833>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_833.html).
pub const ASGN_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "AsgnRptID",
    tag: 833,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_STIP_TYPE <888>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_888.html).
pub const UNDERLYING_STIP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingStipType",
    tag: 888,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`YIELD_TYPE <235>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_235.html).
pub const YIELD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "YieldType",
    tag: 235,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`ORDER_CAPACITY <528>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_528.html).
pub const ORDER_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "OrderCapacity",
    tag: 528,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TOTAL_VOLUME_TRADED <387>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_387.html).
pub const TOTAL_VOLUME_TRADED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TotalVolumeTraded",
    tag: 387,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_QUOTE_SETS <296>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_296.html).
pub const NO_QUOTE_SETS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoQuoteSets",
    tag: 296,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_SECURITY_ALT_ID <458>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_458.html).
pub const UNDERLYING_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingSecurityAltID",
    tag: 458,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`REPURCHASE_TERM <226>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_226.html).
pub const REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "RepurchaseTerm",
    tag: 226,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTY_SUB_ID <523>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_523.html).
pub const PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PartySubID",
    tag: 523,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NESTED3_PARTY_SUB_ID_TYPE <954>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_954.html).
pub const NESTED3_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "Nested3PartySubIDType",
    tag: 954,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTY_SUB_ID_TYPE <803>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_803.html).
pub const PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PartySubIDType",
    tag: 803,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_LOCALE_OF_ISSUE <594>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_594.html).
pub const UNDERLYING_LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingLocaleOfIssue",
    tag: 594,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_ORIGINATION_DATE <229>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_229.html).
pub const TRADE_ORIGINATION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeOriginationDate",
    tag: 229,
    data_type: FixDatatype::LocalMktDate,
    location: FieldLocation::Body,
};

/// Field attributes for [`ENCODED_LIST_STATUS_TEXT_LEN <445>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_445.html).
pub const ENCODED_LIST_STATUS_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EncodedListStatusTextLen",
    tag: 445,
    data_type: FixDatatype::Length,
    location: FieldLocation::Body,
};

/// Field attributes for [`PARTY_ROLE <452>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_452.html).
pub const PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PartyRole",
    tag: 452,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEG_STATE_OR_PROVINCE_OF_ISSUE <597>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_597.html).
pub const LEG_STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LegStateOrProvinceOfIssue",
    tag: 597,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_CURR_FX_RATE_CALC <156>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_156.html).
pub const SETTL_CURR_FX_RATE_CALC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlCurrFxRateCalc",
    tag: 156,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`UNDERLYING_CP_PROGRAM <877>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_877.html).
pub const UNDERLYING_CP_PROGRAM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UnderlyingCPProgram",
    tag: 877,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`EVENT_TEXT <868>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_868.html).
pub const EVENT_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "EventText",
    tag: 868,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LEAVES_QTY <151>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_151.html).
pub const LEAVES_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LeavesQty",
    tag: 151,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADE_REPORT_REF_ID <572>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_572.html).
pub const TRADE_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradeReportRefID",
    tag: 572,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIST_ORDER_STATUS <431>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_431.html).
pub const LIST_ORDER_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "ListOrderStatus",
    tag: 431,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CASH_DISTRIB_AGENT_ACCT_NAME <502>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_502.html).
pub const CASH_DISTRIB_AGENT_ACCT_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CashDistribAgentAcctName",
    tag: 502,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SENDER_LOCATION_ID <142>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_142.html).
pub const SENDER_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SenderLocationID",
    tag: 142,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`TRAD_SES_STATUS_REJ_REASON <567>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_567.html).
pub const TRAD_SES_STATUS_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesStatusRejReason",
    tag: 567,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`LAST_NETWORK_RESPONSE_ID <934>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_934.html).
pub const LAST_NETWORK_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LastNetworkResponseID",
    tag: 934,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`PRIORITY_INDICATOR <638>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_638.html).
pub const PRIORITY_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PriorityIndicator",
    tag: 638,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`DELIVER_TO_SUB_ID <129>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_129.html).
pub const DELIVER_TO_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "DeliverToSubID",
    tag: 129,
    data_type: FixDatatype::String,
    location: FieldLocation::Header,
};

/// Field attributes for [`TRAD_SES_END_TIME <345>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_345.html).
pub const TRAD_SES_END_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradSesEndTime",
    tag: 345,
    data_type: FixDatatype::UtcTimestamp,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_MAINT_ACTION <712>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_712.html).
pub const POS_MAINT_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosMaintAction",
    tag: 712,
    data_type: FixDatatype::Int,
    location: FieldLocation::Body,
};

/// Field attributes for [`CANCELLATION_RIGHTS <480>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_480.html).
pub const CANCELLATION_RIGHTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "CancellationRights",
    tag: 480,
    data_type: FixDatatype::Char,
    location: FieldLocation::Body,
};

/// Field attributes for [`TRADING_SESSION_SUB_ID <625>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_625.html).
pub const TRADING_SESSION_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "TradingSessionSubID",
    tag: 625,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`SETTL_INST_MSG_ID <777>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_777.html).
pub const SETTL_INST_MSG_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SettlInstMsgID",
    tag: 777,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`POS_AMT_TYPE <707>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_707.html).
pub const POS_AMT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "PosAmtType",
    tag: 707,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`USER_REQUEST_ID <923>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_923.html).
pub const USER_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "UserRequestID",
    tag: 923,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};

/// Field attributes for [`NO_TRD_REG_TIMESTAMPS <768>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_768.html).
pub const NO_TRD_REG_TIMESTAMPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "NoTrdRegTimestamps",
    tag: 768,
    data_type: FixDatatype::NumInGroup,
    location: FieldLocation::Body,
};

/// Field attributes for [`SELL_VOLUME <331>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_331.html).
pub const SELL_VOLUME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "SellVolume",
    tag: 331,
    data_type: FixDatatype::Qty,
    location: FieldLocation::Body,
};

/// Field attributes for [`LIQUIDITY_PCT_LOW <402>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_402.html).
pub const LIQUIDITY_PCT_LOW: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "LiquidityPctLow",
    tag: 402,
    data_type: FixDatatype::Percentage,
    location: FieldLocation::Body,
};

/// Field attributes for [`STAND_INST_DB_NAME <170>`](https://www.onixs.biz/fix-dictionary/4.4/tagnum_170.html).
pub const STAND_INST_DB_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {
    name: "StandInstDbName",
    tag: 170,
    data_type: FixDatatype::String,
    location: FieldLocation::Body,
};