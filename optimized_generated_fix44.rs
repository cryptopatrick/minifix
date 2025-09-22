// Generated automatically by MiniFixRust 0.1.0 on Mon, 22 Sep 2025 20:01:56 +0000.
// OPTIMIZED VERSION - Reduced from 13,900 to ~2,500 lines.
//
// DO NOT MODIFY MANUALLY.
// DO NOT COMMIT TO VERSION CONTROL.
// ALL CHANGES WILL BE OVERWRITTEN.

// https://www.onixs.biz/fix-dictionary/4.4/index.html

use minifix::dict::{FieldLocation, FixDatatype};
use minifix::definitions::HardCodedFixFieldDefinition;
use minifix::FieldType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum UnsolicitedIndicator {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeReportRejectReason {
    #[minifix(variant = "0")] Successful,
    #[minifix(variant = "1")] InvalidPartyInformation,
    #[minifix(variant = "2")] UnknownInstrument,
    #[minifix(variant = "3")] UnauthorizedToReportTrades,
    #[minifix(variant = "4")] InvalidTradeType,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollAsgnTransType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Replace,
    #[minifix(variant = "2")] Cancel,
    #[minifix(variant = "3")] Release,
    #[minifix(variant = "4")] Reverse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeRequestType {
    #[minifix(variant = "0")] AllTrades,
    #[minifix(variant = "1")] MatchedTradesMatchingCriteriaProvidedOnRequest,
    #[minifix(variant = "2")] UnmatchedTradesThatMatchCriteria,
    #[minifix(variant = "3")] UnreportedTradesThatMatchCriteria,
    #[minifix(variant = "4")] AdvisoriesThatMatchCriteria,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosMaintStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] AcceptedWithWarnings,
    #[minifix(variant = "2")] Rejected,
    #[minifix(variant = "3")] Completed,
    #[minifix(variant = "4")] CompletedWithWarnings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum GapFillFlag {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ResponseTransportType {
    #[minifix(variant = "0")] InbandTransportTheRequestWasSentOver,
    #[minifix(variant = "1")] OutOfBandPreArrangedOutOfBandDeliveryMechanism,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LocateReqd {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum Scope {
    #[minifix(variant = "1")] Local,
    #[minifix(variant = "2")] National,
    #[minifix(variant = "3")] Global,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MdEntryType {
    #[minifix(variant = "0")] Bid,
    #[minifix(variant = "1")] Offer,
    #[minifix(variant = "2")] Trade,
    #[minifix(variant = "3")] IndexValue,
    #[minifix(variant = "4")] OpeningPrice,
    #[minifix(variant = "5")] ClosingPrice,
    #[minifix(variant = "6")] SettlementPrice,
    #[minifix(variant = "7")] TradingSessionHighPrice,
    #[minifix(variant = "8")] TradingSessionLowPrice,
    #[minifix(variant = "9")] TradingSessionVwapPrice,
    #[minifix(variant = "A")] Imbalance,
    #[minifix(variant = "B")] TradeVolume,
    #[minifix(variant = "C")] OpenInterest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradSesStatus {
    #[minifix(variant = "0")] Unknown,
    #[minifix(variant = "1")] Halted,
    #[minifix(variant = "2")] Open,
    #[minifix(variant = "3")] Closed,
    #[minifix(variant = "4")] PreOpen,
    #[minifix(variant = "5")] PreClose,
    #[minifix(variant = "6")] RequestRejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CrossType {
    #[minifix(variant = "1")] CrossTradeWhichIsExecutedCompletelyOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone,
    #[minifix(variant = "2")] CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSideNoteTheCrossprioritzation,
    #[minifix(variant = "3")] CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted,
    #[minifix(variant = "4")] CrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInTheCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCrossIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TargetStrategy {
    #[minifix(variant = "1")] Vwap,
    #[minifix(variant = "2")] Participate,
    #[minifix(variant = "3")] MininizeMarketImpact,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OrdRejReason {
    #[minifix(variant = "0")] Broker,
    #[minifix(variant = "1")] UnknownSymbol,
    #[minifix(variant = "2")] ExchangeClosed,
    #[minifix(variant = "3")] OrderExceedsLimit,
    #[minifix(variant = "4")] TooLateToEnter,
    #[minifix(variant = "5")] UnknownOrder,
    #[minifix(variant = "6")] DuplicateOrder,
    #[minifix(variant = "7")] DuplicateOfAVerballyCommunicatedOrder,
    #[minifix(variant = "8")] StaleOrder,
    #[minifix(variant = "9")] TradeAlongRequired,
    #[minifix(variant = "10")] InvalidInvestorId,
    #[minifix(variant = "11")] UnsupportedOrderCharacteristic12SurveillenceOption,
    #[minifix(variant = "13")] IncorrectQuantity,
    #[minifix(variant = "14")] IncorrectAllocatedQuantity,
    #[minifix(variant = "15")] UnknownAccount,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityTradingStatus {
    #[minifix(variant = "1")] OpeningDelay,
    #[minifix(variant = "2")] TradingHalt,
    #[minifix(variant = "3")] Resume,
    #[minifix(variant = "4")] NoOpenNoResume,
    #[minifix(variant = "5")] PriceIndication,
    #[minifix(variant = "6")] TradingRangeIndication,
    #[minifix(variant = "7")] MarketImbalanceBuy,
    #[minifix(variant = "8")] MarketImbalanceSell,
    #[minifix(variant = "9")] MarketOnCloseImbalanceBuy,
    #[minifix(variant = "10")] MarketOnCloseImbalanceSell,
    #[minifix(variant = "12")] NoMarketImbalance,
    #[minifix(variant = "13")] NoMarketOnCloseImbalance,
    #[minifix(variant = "14")] ItsPreOpening,
    #[minifix(variant = "15")] NewPriceIndication,
    #[minifix(variant = "16")] TradeDisseminationTime,
    #[minifix(variant = "17")] ReadyToTrade,
    #[minifix(variant = "18")] NotAvailableForTrading,
    #[minifix(variant = "19")] NotTradedOnThisMarket,
    #[minifix(variant = "20")] UnknownOrInvalid,
    #[minifix(variant = "21")] PreOpen,
    #[minifix(variant = "22")] OpeningRotation,
    #[minifix(variant = "23")] FastMarket,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollAsgnRespType {
    #[minifix(variant = "0")] Received,
    #[minifix(variant = "1")] Accepted,
    #[minifix(variant = "2")] Declined,
    #[minifix(variant = "3")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollInquiryQualifier {
    #[minifix(variant = "0")] Tradedate,
    #[minifix(variant = "1")] GcInstrument,
    #[minifix(variant = "2")] Collateralinstrument,
    #[minifix(variant = "3")] SubstitutionEligible,
    #[minifix(variant = "4")] NotAssigned,
    #[minifix(variant = "5")] PartiallyAssigned,
    #[minifix(variant = "6")] FullyAssigned,
    #[minifix(variant = "7")] OutstandingTrades,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] CanceledForSymbol,
    #[minifix(variant = "2")] CanceledForSecurityType,
    #[minifix(variant = "3")] CanceledForUnderlying,
    #[minifix(variant = "4")] CanceledAll,
    #[minifix(variant = "5")] Rejected,
    #[minifix(variant = "6")] RemovedFromMarket,
    #[minifix(variant = "7")] Expired,
    #[minifix(variant = "8")] Query,
    #[minifix(variant = "9")] QuoteNotFound,
    #[minifix(variant = "10")] Pending,
    #[minifix(variant = "11")] Pass,
    #[minifix(variant = "12")] LockedMarketWarning,
    #[minifix(variant = "13")] CrossMarketWarning,
    #[minifix(variant = "14")] CanceledDueToLockMarket,
    #[minifix(variant = "15")] CanceledDueToCrossMarket,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CpProgram {
    #[minifix(variant = "1")] _3,
    #[minifix(variant = "2")] _4,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ConfirmTransType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Replace,
    #[minifix(variant = "2")] Cancel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum Urgency {
    #[minifix(variant = "0")] Normal,
    #[minifix(variant = "1")] Flash,
    #[minifix(variant = "2")] Background,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AffirmStatus {
    #[minifix(variant = "1")] Received,
    #[minifix(variant = "2")] ConfirmRejectedIeNotAffirmed,
    #[minifix(variant = "3")] Affirmed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PutOrCall {
    #[minifix(variant = "0")] Put,
    #[minifix(variant = "1")] Call,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MoneyLaunderingStatus {
    #[minifix(variant = "Y")] Passed,
    #[minifix(variant = "N")] NotChecked,
    #[minifix(variant = "1")] ExemptBelowTheLimit,
    #[minifix(variant = "2")] ExemptClientMoneyTypeExemption,
    #[minifix(variant = "3")] ExemptAuthorisedCreditOrFinancialInstitution,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PegScope {
    #[minifix(variant = "1")] Local,
    #[minifix(variant = "2")] National,
    #[minifix(variant = "3")] Global,
    #[minifix(variant = "4")] NationalExcludingLocal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollAsgnRejectReason {
    #[minifix(variant = "0")] UnknownDeal,
    #[minifix(variant = "1")] UnknownOrInvalidInstrument,
    #[minifix(variant = "2")] UnauthorizedTransaction,
    #[minifix(variant = "3")] InsufficientCollateral,
    #[minifix(variant = "4")] InvalidTypeOfCollateral,
    #[minifix(variant = "5")] ExcessiveSubstitution,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AvgPxIndicator {
    #[minifix(variant = "0")] NoAveragePricing,
    #[minifix(variant = "1")] TradeIsPartOfAnAveragePriceGroupIdentifiedByTheTradelinkid,
    #[minifix(variant = "2")] LastTradeInTheAveragePriceGroupIdentifiedByTheTradelinkid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteCancelType {
    #[minifix(variant = "1")] CancelForSymbol,
    #[minifix(variant = "2")] CancelForSecurityType,
    #[minifix(variant = "3")] CancelForUnderlyingSymbol,
    #[minifix(variant = "4")] CancelAllQuotes,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum RoundingDirection {
    #[minifix(variant = "0")] RoundToNearest,
    #[minifix(variant = "1")] RoundDown,
    #[minifix(variant = "2")] RoundUp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LastFragment {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PartyIdSource {
    #[minifix(variant = "B")] Bic,
    #[minifix(variant = "C")] GenerallyAcceptedMarketParticipantIdentifier,
    #[minifix(variant = "D")] ProprietaryCustomCode,
    #[minifix(variant = "E")] IsoCountryCode,
    #[minifix(variant = "F")] SettlementEntityLocation,
    #[minifix(variant = "G")] Mic,
    #[minifix(variant = "H")] CsdParticipantMemberCode,
    #[minifix(variant = "1")] KoreanInvestorId,
    #[minifix(variant = "2")] TaiwaneseQualifiedForeignInvestorIdQfii,
    #[minifix(variant = "3")] TaiwaneseTradingAccount,
    #[minifix(variant = "4")] MalaysianCentralDepository,
    #[minifix(variant = "5")] ChineseBShare,
    #[minifix(variant = "6")] UkNationalInsuranceOrPensionNumber,
    #[minifix(variant = "7")] UsSocialSecurityNumber,
    #[minifix(variant = "8")] UsEmployerIdentificationNumber,
    #[minifix(variant = "9")] AustralianBusinessNumber,
    #[minifix(variant = "A")] AustralianTaxFileNumber,
    #[minifix(variant = "I")] DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CorporateAction {
    #[minifix(variant = "A")] ExDividend,
    #[minifix(variant = "B")] ExDistribution,
    #[minifix(variant = "C")] ExRights,
    #[minifix(variant = "D")] New,
    #[minifix(variant = "E")] ExInterest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ContAmtType {
    #[minifix(variant = "1")] CommissionAmount,
    #[minifix(variant = "2")] Commission,
    #[minifix(variant = "3")] InitialChargeAmount,
    #[minifix(variant = "4")] InitialCharge,
    #[minifix(variant = "5")] DiscountAmount,
    #[minifix(variant = "6")] Discount,
    #[minifix(variant = "7")] DilutionLevyAmount,
    #[minifix(variant = "8")] DilutionLevy,
    #[minifix(variant = "9")] ExitChargeAmount,
    #[minifix(variant = "10")] ExitCharge,
    #[minifix(variant = "11")] FundBasedRenewalCommission,
    #[minifix(variant = "12")] ProjectedFundValue,
    #[minifix(variant = "13")] FundBasedRenewalCommissionAmount13,
    #[minifix(variant = "14")] FundBasedRenewalCommissionAmount14,
    #[minifix(variant = "15")] NetSettlementAmount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BookingUnit {
    #[minifix(variant = "0")] EachPartialExecutionIsABookableUnit,
    #[minifix(variant = "1")] AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder,
    #[minifix(variant = "2")] AggregateExecutionsForThisSymbolSideAndSettlementDate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosReqResult {
    #[minifix(variant = "0")] ValidRequest,
    #[minifix(variant = "1")] InvalidOrUnsupportedRequest,
    #[minifix(variant = "2")] NoPositionsFoundThatMatchCriteria,
    #[minifix(variant = "3")] NotAuthorizedToRequestPositions,
    #[minifix(variant = "4")] RequestForPositionNotSupported,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ClearingInstruction {
    #[minifix(variant = "0")] ProcessNormally,
    #[minifix(variant = "1")] ExcludeFromAllNetting,
    #[minifix(variant = "2")] BilateralNettingOnly,
    #[minifix(variant = "3")] ExClearing,
    #[minifix(variant = "4")] SpecialTrade,
    #[minifix(variant = "5")] MultilateralNetting,
    #[minifix(variant = "6")] ClearAgainstCentralCounterparty,
    #[minifix(variant = "7")] ExcludeFromCentralCounterparty,
    #[minifix(variant = "8")] ManualMode,
    #[minifix(variant = "9")] AutomaticPostingMode,
    #[minifix(variant = "10")] AutomaticGiveUpMode,
    #[minifix(variant = "11")] QualifiedServiceRepresentative,
    #[minifix(variant = "12")] CustomerTrade,
    #[minifix(variant = "13")] SelfClearing,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosMaintResult {
    #[minifix(variant = "0")] SuccessfulCompletion,
    #[minifix(variant = "1")] Rejected,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum NoSides {
    #[minifix(variant = "1")] OneSide,
    #[minifix(variant = "2")] BothSides,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MessageEncoding {
    #[minifix(variant = "ISO-2022-JP")] Jis,
    #[minifix(variant = "EUC-JP")] Euc,
    #[minifix(variant = "Shift_JIS")] ForUsingSjis,
    #[minifix(variant = "UTF-8")] Unicode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MdImplicitDelete {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ConfirmType {
    #[minifix(variant = "1")] Status,
    #[minifix(variant = "2")] Confirmation,
    #[minifix(variant = "3")] ConfirmationRequestRejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DeliveryType {
    #[minifix(variant = "0")] VersusPaymentDeliver,
    #[minifix(variant = "1")] FreeDeliver,
    #[minifix(variant = "2")] TriParty,
    #[minifix(variant = "3")] HoldInCustody,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TrdType {
    #[minifix(variant = "0")] RegularTrade,
    #[minifix(variant = "1")] BlockTrade,
    #[minifix(variant = "2")] Efp,
    #[minifix(variant = "3")] Transfer,
    #[minifix(variant = "4")] LateTrade,
    #[minifix(variant = "5")] TTrade,
    #[minifix(variant = "6")] WeightedAveragePriceTrade,
    #[minifix(variant = "7")] BunchedTrade,
    #[minifix(variant = "8")] LateBunchedTrade,
    #[minifix(variant = "9")] PriorReferencePriceTrade,
    #[minifix(variant = "10")] AfterHoursTrade,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum HaltReasonChar {
    #[minifix(variant = "I")] OrderImbalance,
    #[minifix(variant = "X")] EquipmentChangeover,
    #[minifix(variant = "P")] NewsPending,
    #[minifix(variant = "D")] NewsDissemination,
    #[minifix(variant = "E")] OrderInflux,
    #[minifix(variant = "M")] AdditionalInformation,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocReportType {
    #[minifix(variant = "3")] SellsideCalculatedUsingPreliminary,
    #[minifix(variant = "4")] SellsideCalculatedWithoutPreliminary,
    #[minifix(variant = "5")] WarehouseRecap,
    #[minifix(variant = "8")] RequestToIntermediary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DeleteReason {
    #[minifix(variant = "0")] Cancelation,
    #[minifix(variant = "1")] Error,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AdjustmentType {
    #[minifix(variant = "0")] ProcessRequestAsMarginDisposition,
    #[minifix(variant = "1")] DeltaPlus,
    #[minifix(variant = "2")] DeltaMinus,
    #[minifix(variant = "3")] Final,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ConfirmRejReason {
    #[minifix(variant = "1")] MismatchedAccount,
    #[minifix(variant = "2")] MissingSettlementInstructions,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PossResend {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MiscFeeBasis {
    #[minifix(variant = "0")] Absolute,
    #[minifix(variant = "1")] PerUnit,
    #[minifix(variant = "2")] Percentage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeRequestResult {
    #[minifix(variant = "0")] Successful,
    #[minifix(variant = "1")] InvalidOrUnknownInstrument,
    #[minifix(variant = "2")] InvalidTypeOfTradeRequested,
    #[minifix(variant = "3")] InvalidParties,
    #[minifix(variant = "4")] InvalidTransportTypeRequested,
    #[minifix(variant = "5")] InvalidDestinationRequested,
    #[minifix(variant = "8")] TraderequesttypeNotSupported,
    #[minifix(variant = "9")] UnauthorizedForTradeCaptureReportRequest,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CommType {
    #[minifix(variant = "1")] PerUnit,
    #[minifix(variant = "2")] Percentage,
    #[minifix(variant = "3")] Absolute,
    #[minifix(variant = "4")] _4,
    #[minifix(variant = "5")] _5,
    #[minifix(variant = "6")] PointsPerBondOrContractSupplyContractmultiplier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AdvSide {
    #[minifix(variant = "B")] Buy,
    #[minifix(variant = "S")] Sell,
    #[minifix(variant = "X")] Cross,
    #[minifix(variant = "T")] Trade,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PreallocMethod {
    #[minifix(variant = "0")] ProRata,
    #[minifix(variant = "1")] DoNotProRataDiscussFirst,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum InstrAttribType {
    #[minifix(variant = "1")] Flat,
    #[minifix(variant = "2")] ZeroCoupon,
    #[minifix(variant = "3")] InterestBearing,
    #[minifix(variant = "4")] NoPeriodicPayments,
    #[minifix(variant = "5")] VariableRate,
    #[minifix(variant = "6")] LessFeeForPut,
    #[minifix(variant = "7")] SteppedCoupon,
    #[minifix(variant = "8")] CouponPeriod,
    #[minifix(variant = "9")] WhenAndIfIssued,
    #[minifix(variant = "10")] OriginalIssueDiscount,
    #[minifix(variant = "11")] CallablePuttable,
    #[minifix(variant = "12")] EscrowedToMaturity,
    #[minifix(variant = "13")] EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrattribvalue,
    #[minifix(variant = "14")] Prerefunded,
    #[minifix(variant = "15")] InDefault,
    #[minifix(variant = "16")] Unrated,
    #[minifix(variant = "17")] Taxable,
    #[minifix(variant = "18")] Indexed,
    #[minifix(variant = "19")] SubjectToAlternativeMinimumTax,
    #[minifix(variant = "20")] OriginalIssueDiscountPriceSupplyPriceInTheInstrattribvalue,
    #[minifix(variant = "21")] CallableBelowMaturityValue,
    #[minifix(variant = "22")] CallableWithoutNoticeByMailToHolderUnlessRegistered,
    #[minifix(variant = "99")] TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrattribvalue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlType {
    #[minifix(variant = "0")] Regular,
    #[minifix(variant = "1")] Cash,
    #[minifix(variant = "2")] NextDay,
    #[minifix(variant = "3")] TPlus2,
    #[minifix(variant = "4")] TPlus3,
    #[minifix(variant = "5")] TPlus4,
    #[minifix(variant = "6")] Future,
    #[minifix(variant = "7")] WhenAndIfIssued,
    #[minifix(variant = "8")] SellersOption,
    #[minifix(variant = "9")] TPlus5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum FundRenewWaiv {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OpenCloseSettlFlag {
    #[minifix(variant = "0")] DailyOpen,
    #[minifix(variant = "1")] SessionOpen,
    #[minifix(variant = "2")] DeliverySettlementEntry,
    #[minifix(variant = "3")] ExpectedEntry,
    #[minifix(variant = "4")] EntryFromPreviousBusinessDay,
    #[minifix(variant = "5")] TheoreticalPriceValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum Side {
    #[minifix(variant = "1")] Buy,
    #[minifix(variant = "2")] Sell,
    #[minifix(variant = "3")] BuyMinus,
    #[minifix(variant = "4")] SellPlus,
    #[minifix(variant = "5")] SellShort,
    #[minifix(variant = "6")] SellShortExempt,
    #[minifix(variant = "7")] Undisclosed,
    #[minifix(variant = "8")] Cross,
    #[minifix(variant = "9")] CrossShort,
    #[minifix(variant = "A")] CrossShortExempt,
    #[minifix(variant = "B")] AsDefined,
    #[minifix(variant = "C")] Opposite,
    #[minifix(variant = "D")] Subscribe,
    #[minifix(variant = "E")] Redeem,
    #[minifix(variant = "F")] Lend,
    #[minifix(variant = "G")] Borrow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum NotifyBrokerOfCredit {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeReportType {
    #[minifix(variant = "0")] Submit,
    #[minifix(variant = "1")] Alleged,
    #[minifix(variant = "2")] Accept,
    #[minifix(variant = "3")] Decline,
    #[minifix(variant = "4")] Addendum,
    #[minifix(variant = "5")] NoWas,
    #[minifix(variant = "6")] TradeReportCancel,
    #[minifix(variant = "7")] LockedInTradeBreak,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CoveredOrUncovered {
    #[minifix(variant = "0")] Covered,
    #[minifix(variant = "1")] Uncovered,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LegSwapType {
    #[minifix(variant = "1")] ParForPar,
    #[minifix(variant = "2")] ModifiedDuration,
    #[minifix(variant = "4")] Risk,
    #[minifix(variant = "5")] Proceeds,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocLinkType {
    #[minifix(variant = "0")] FXNetting,
    #[minifix(variant = "1")] FXSwap,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PegOffsetType {
    #[minifix(variant = "0")] Price,
    #[minifix(variant = "1")] BasisPoints,
    #[minifix(variant = "2")] Ticks,
    #[minifix(variant = "3")] PriceTier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BidDescriptorType {
    #[minifix(variant = "1")] Sector,
    #[minifix(variant = "2")] Country,
    #[minifix(variant = "3")] Index,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ApplQueueAction {
    #[minifix(variant = "0")] NoActionTaken,
    #[minifix(variant = "1")] QueueFlushed,
    #[minifix(variant = "2")] OverlayLast,
    #[minifix(variant = "3")] EndSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum GtBookingInst {
    #[minifix(variant = "0")] BookOutAllTradesOnDayOfExecution,
    #[minifix(variant = "1")] AccumulateExecutionsUntilOrderIsFilledOrExpires,
    #[minifix(variant = "2")] AccumulateUntilVerballyNotifiedOtherwise,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MultiLegRptTypeReq {
    #[minifix(variant = "0")] ReportByMulitlegSecurityOnly,
    #[minifix(variant = "1")] ReportByMultilegSecurityAndByInstrumentLegsBelongingToTheMultilegSecurity,
    #[minifix(variant = "2")] ReportByInstrumentLegsBelongingToTheMultilegSecurityOnly,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ResetSeqNumFlag {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosReqType {
    #[minifix(variant = "0")] Positions,
    #[minifix(variant = "1")] Trades,
    #[minifix(variant = "2")] Exercises,
    #[minifix(variant = "3")] Assignments,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PaymentMethod {
    #[minifix(variant = "1")] Crest,
    #[minifix(variant = "2")] Nscc,
    #[minifix(variant = "3")] Euroclear,
    #[minifix(variant = "4")] Clearstream,
    #[minifix(variant = "5")] Cheque,
    #[minifix(variant = "6")] TelegraphicTransfer,
    #[minifix(variant = "7")] Fedwire,
    #[minifix(variant = "8")] DebitCard,
    #[minifix(variant = "9")] DirectDebit,
    #[minifix(variant = "10")] DirectCredit,
    #[minifix(variant = "11")] CreditCard,
    #[minifix(variant = "12")] AchDebit,
    #[minifix(variant = "13")] AchCredit,
    #[minifix(variant = "14")] Bpay,
    #[minifix(variant = "15")] HighValueClearingSystem,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocSettlInstType {
    #[minifix(variant = "0")] UseDefaultInstructions,
    #[minifix(variant = "1")] DeriveFromParametersProvided,
    #[minifix(variant = "2")] FullDetailsProvided,
    #[minifix(variant = "3")] SsiDbIdsProvided,
    #[minifix(variant = "4")] PhoneForInstructions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ProgRptReqs {
    #[minifix(variant = "1")] BuysideExplicitlyRequestsStatusUsingStatusrequest,
    #[minifix(variant = "2")] SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod,
    #[minifix(variant = "3")] RealTimeExecutionReports,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionScope {
    #[minifix(variant = "1")] Local,
    #[minifix(variant = "2")] National,
    #[minifix(variant = "3")] Global,
    #[minifix(variant = "4")] NationalExcludingLocal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum StandInstDbType {
    #[minifix(variant = "0")] Other,
    #[minifix(variant = "1")] DtcSid,
    #[minifix(variant = "2")] ThomsonAlert,
    #[minifix(variant = "3")] AGlobalCustodian,
    #[minifix(variant = "4")] Accountnet,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityRequestType {
    #[minifix(variant = "0")] RequestSecurityIdentityAndSpecifications,
    #[minifix(variant = "1")] RequestSecurityIdentityForTheSpecificationsProvided,
    #[minifix(variant = "2")] RequestListSecurityTypes,
    #[minifix(variant = "3")] RequestListSecurities,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SideMultiLegReportingType {
    #[minifix(variant = "1")] SingleSecurity,
    #[minifix(variant = "2")] IndividualLegOfAMultiLegSecurity,
    #[minifix(variant = "3")] MultiLegSecurity,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteType {
    #[minifix(variant = "0")] Indicative,
    #[minifix(variant = "1")] Tradeable,
    #[minifix(variant = "2")] RestrictedTradeable,
    #[minifix(variant = "3")] Counter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlInstMode {
    #[minifix(variant = "1")] StandingInstructionsProvided,
    #[minifix(variant = "4")] SpecificOrderForASingleAccount,
    #[minifix(variant = "5")] RequestReject,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExerciseMethod {
    #[minifix(variant = "A")] Automatic,
    #[minifix(variant = "M")] Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocIntermedReqType {
    #[minifix(variant = "1")] PendingAccept,
    #[minifix(variant = "2")] PendingRelease,
    #[minifix(variant = "3")] PendingReversal,
    #[minifix(variant = "4")] Accept,
    #[minifix(variant = "5")] BlockLevelReject,
    #[minifix(variant = "6")] AccountLevelReject,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MultiLegReportingType {
    #[minifix(variant = "1")] SingleSecurity,
    #[minifix(variant = "2")] IndividualLegOfAMultiLegSecurity,
    #[minifix(variant = "3")] MultiLegSecurity,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum RegistTransType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Replace,
    #[minifix(variant = "2")] Cancel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DeliveryForm {
    #[minifix(variant = "1")] Bookentry,
    #[minifix(variant = "2")] Bearer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OwnershipType {
    #[minifix(variant = "J")] JointInvestors,
    #[minifix(variant = "T")] TenantsInCommon,
    #[minifix(variant = "2")] JointTrustees,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocTransType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Replace,
    #[minifix(variant = "2")] Cancel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ReportToExch {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MdUpdateType {
    #[minifix(variant = "0")] FullRefresh,
    #[minifix(variant = "1")] IncrementalRefresh,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IoiQualifier {
    #[minifix(variant = "A")] AllOrNone,
    #[minifix(variant = "B")] MarketOnClose,
    #[minifix(variant = "C")] AtTheClose,
    #[minifix(variant = "D")] Vwap,
    #[minifix(variant = "I")] InTouchWith,
    #[minifix(variant = "L")] Limit,
    #[minifix(variant = "M")] MoreBehind,
    #[minifix(variant = "O")] AtTheOpen,
    #[minifix(variant = "P")] TakingAPosition,
    #[minifix(variant = "Q")] AtTheMarket,
    #[minifix(variant = "R")] ReadyToTrade,
    #[minifix(variant = "S")] PortfolioShown,
    #[minifix(variant = "T")] ThroughTheDay,
    #[minifix(variant = "V")] Versus,
    #[minifix(variant = "W")] Indication,
    #[minifix(variant = "X")] CrossingOpportunity,
    #[minifix(variant = "Y")] AtTheMidpoint,
    #[minifix(variant = "Z")] PreOpen,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionLimitType {
    #[minifix(variant = "0")] OrBetter,
    #[minifix(variant = "1")] StrictLimitIsAStrictLimit,
    #[minifix(variant = "2")] OrWorseForABuyTheDiscretionPriceIsAMinimumAndForASellTheDiscretionPriceIsAMaximum,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LastCapacity {
    #[minifix(variant = "1")] Agent,
    #[minifix(variant = "2")] CrossAsAgent,
    #[minifix(variant = "3")] CrossAsPrincipal,
    #[minifix(variant = "4")] Principal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BusinessRejectReason {
    #[minifix(variant = "0")] Other,
    #[minifix(variant = "1")] UnkownId,
    #[minifix(variant = "2")] UnknownSecurity,
    #[minifix(variant = "3")] UnsupportedMessageType,
    #[minifix(variant = "4")] ApplicationNotAvailable,
    #[minifix(variant = "5")] ConditionallyRequiredFieldMissing,
    #[minifix(variant = "6")] NotAuthorized,
    #[minifix(variant = "7")] DelivertoFirmNotAvailableAtThisTime,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ConfirmStatus {
    #[minifix(variant = "1")] Received,
    #[minifix(variant = "2")] MismatchedAccount,
    #[minifix(variant = "3")] MissingSettlementInstructions,
    #[minifix(variant = "4")] Confirmed,
    #[minifix(variant = "5")] RequestRejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollInquiryStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] AcceptedWithWarnings,
    #[minifix(variant = "2")] Completed,
    #[minifix(variant = "3")] CompletedWithWarnings,
    #[minifix(variant = "4")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ClearingFeeIndicator {
    #[minifix(variant = "B")] CboeMember,
    #[minifix(variant = "C")] NonMemberAndCustomer,
    #[minifix(variant = "E")] EquityMemberAndClearingMember,
    #[minifix(variant = "F")] FullAndAssociateMemberTradingForOwnAccountAndAsFloorBrokers,
    #[minifix(variant = "H")] _106hAnd106jFirms,
    #[minifix(variant = "I")] GimIdemAndComMembershipInterestHolders,
    #[minifix(variant = "L")] LesseeAnd106fEmployees,
    #[minifix(variant = "M")] AllOtherOwnershipTypes,
    #[minifix(variant = "1")] _1stYearDelegateTradingForHisOwnAccount,
    #[minifix(variant = "2")] _2ndYearDelegateTradingForHisOwnAccount,
    #[minifix(variant = "3")] _3rdYearDelegateTradingForHisOwnAccount,
    #[minifix(variant = "4")] _4thYearDelegateTradingForHisOwnAccount,
    #[minifix(variant = "5")] _5thYearDelegateTradingForHisOwnAccount,
    #[minifix(variant = "9")] _6thYearAndBeyondDelegateTradingForHisOwnAccount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DlvyInstType {
    #[minifix(variant = "S")] Securities,
    #[minifix(variant = "C")] Cash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PossDupFlag {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AggregatedBook {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PublishTrdIndicator {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TestMessageIndicator {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollInquiryResult {
    #[minifix(variant = "0")] Successful,
    #[minifix(variant = "1")] InvalidOrUnknownInstrument,
    #[minifix(variant = "2")] InvalidOrUnknownCollateralType,
    #[minifix(variant = "3")] InvalidParties,
    #[minifix(variant = "4")] InvalidTransportTypeRequested,
    #[minifix(variant = "5")] InvalidDestinationRequested,
    #[minifix(variant = "6")] NoCollateralFoundForTheTradeSpecified,
    #[minifix(variant = "7")] NoCollateralFoundForTheOrderSpecified,
    #[minifix(variant = "8")] CollateralInquiryTypeNotSupported,
    #[minifix(variant = "9")] UnauthorizedForCollateralInquiry,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DayBookingInst {
    #[minifix(variant = "0")] CanTriggerBookingWithoutReferenceToTheOrderInitiator,
    #[minifix(variant = "1")] SpeakWithOrderInitiatorBeforeBooking,
    #[minifix(variant = "2")] Accumulate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocCancReplaceReason {
    #[minifix(variant = "1")] OriginalDetailsIncompleteIncorrect,
    #[minifix(variant = "2")] ChangeInUnderlyingOrderDetails,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PriceType {
    #[minifix(variant = "1")] Percentage,
    #[minifix(variant = "2")] PerUnit,
    #[minifix(variant = "3")] FixedAmount,
    #[minifix(variant = "4")] DiscountPercentagePointsBelowPar,
    #[minifix(variant = "5")] PremiumPercentagePointsOverPar,
    #[minifix(variant = "6")] Spread,
    #[minifix(variant = "7")] TedPrice,
    #[minifix(variant = "8")] TedYield,
    #[minifix(variant = "9")] Yield,
    #[minifix(variant = "10")] FixedCabinetTradePrice,
    #[minifix(variant = "11")] VariableCabinetTradePrice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityListRequestType {
    #[minifix(variant = "0")] Symbol,
    #[minifix(variant = "1")] SecuritytypeAndOrCficode,
    #[minifix(variant = "2")] Product,
    #[minifix(variant = "3")] Tradingsessionid,
    #[minifix(variant = "4")] AllSecurities,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityResponseType {
    #[minifix(variant = "1")] AcceptSecurityProposalAsIs,
    #[minifix(variant = "2")] AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage,
    #[minifix(variant = "5")] RejectSecurityProposal,
    #[minifix(variant = "6")] CanNotMatchSelectionCriteria,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BidType {
    #[minifix(variant = "1")] NonDisclosedStyle,
    #[minifix(variant = "2")] DisclosedStyle,
    #[minifix(variant = "3")] NoBiddingProcess,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MassCancelRequestType {
    #[minifix(variant = "1")] CancelOrdersForASecurity,
    #[minifix(variant = "2")] CancelOrdersForAnUnderlyingSecurity,
    #[minifix(variant = "3")] CancelOrdersForAProduct,
    #[minifix(variant = "4")] CancelOrdersForACficode,
    #[minifix(variant = "5")] CancelOrdersForASecuritytype,
    #[minifix(variant = "6")] CancelOrdersForATradingSession,
    #[minifix(variant = "7")] CancelAllOrders,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlInstSource {
    #[minifix(variant = "1")] BrokersInstructions,
    #[minifix(variant = "2")] InstitutionsInstructions,
    #[minifix(variant = "3")] Investor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionInst {
    #[minifix(variant = "0")] RelatedToDisplayedPrice,
    #[minifix(variant = "1")] RelatedToMarketPrice,
    #[minifix(variant = "2")] RelatedToPrimaryPrice,
    #[minifix(variant = "3")] RelatedToLocalPrimaryPrice,
    #[minifix(variant = "4")] RelatedToMidpointPrice,
    #[minifix(variant = "5")] RelatedToLastTradePrice,
    #[minifix(variant = "6")] RelatedToVwap,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] BlockLevelReject,
    #[minifix(variant = "2")] AccountLevelReject,
    #[minifix(variant = "3")] Received,
    #[minifix(variant = "4")] Incomplete,
    #[minifix(variant = "5")] RejectedByIntermediary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MatchType {
    #[minifix(variant = "A1")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
    #[minifix(variant = "A2")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
    #[minifix(variant = "A3")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
    #[minifix(variant = "A4")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
    #[minifix(variant = "A5")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
    #[minifix(variant = "AQ")] ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
    #[minifix(variant = "S1")] SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
    #[minifix(variant = "S2")] SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
    #[minifix(variant = "S3")] SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
    #[minifix(variant = "S4")] SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
    #[minifix(variant = "S5")] SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
    #[minifix(variant = "M1")] ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimesActM1Match,
    #[minifix(variant = "M2")] SummarizedMatchMinusBadgesAndTimesActM2Match,
    #[minifix(variant = "MT")] OcsLockedInNonAct,
    #[minifix(variant = "M3")] ActAcceptedTrade,
    #[minifix(variant = "M4")] ActDefaultTrade,
    #[minifix(variant = "M5")] ActDefaultAfterM2,
    #[minifix(variant = "M6")] ActM6Match,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MdReqRejReason {
    #[minifix(variant = "0")] UnknownSymbol,
    #[minifix(variant = "1")] DuplicateMdreqid,
    #[minifix(variant = "2")] InsufficientBandwidth,
    #[minifix(variant = "3")] InsufficientPermissions,
    #[minifix(variant = "4")] UnsupportedSubscriptionrequesttype,
    #[minifix(variant = "5")] UnsupportedMarketdepth,
    #[minifix(variant = "6")] UnsupportedMdupdatetype,
    #[minifix(variant = "7")] UnsupportedAggregatedbook,
    #[minifix(variant = "8")] UnsupportedMdentrytype,
    #[minifix(variant = "9")] UnsupportedTradingsessionid,
    #[minifix(variant = "A")] UnsupportedScope,
    #[minifix(variant = "B")] UnsupportedOpenclosesettleflag,
    #[minifix(variant = "C")] UnsupportedMdimplicitdelete,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IncTaxInd {
    #[minifix(variant = "1")] Net,
    #[minifix(variant = "2")] Gross,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradSesMethod {
    #[minifix(variant = "1")] Electronic,
    #[minifix(variant = "2")] OpenOutcry,
    #[minifix(variant = "3")] TwoParty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TrdRegTimestampType {
    #[minifix(variant = "1")] ExecutionTime,
    #[minifix(variant = "2")] TimeIn,
    #[minifix(variant = "3")] TimeOut,
    #[minifix(variant = "4")] BrokerReceipt,
    #[minifix(variant = "5")] BrokerExecution,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum UserRequestType {
    #[minifix(variant = "1")] Logonuser,
    #[minifix(variant = "2")] Logoffuser,
    #[minifix(variant = "3")] Changepasswordforuser,
    #[minifix(variant = "4")] RequestIndividualUserStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuotePriceType {
    #[minifix(variant = "1")] Percent,
    #[minifix(variant = "2")] PerShare,
    #[minifix(variant = "3")] FixedAmount,
    #[minifix(variant = "4")] DiscountPercentagePointsBelowPar,
    #[minifix(variant = "5")] PremiumPercentagePointsOverPar,
    #[minifix(variant = "6")] BasisPointsRelativeToBenchmark,
    #[minifix(variant = "7")] TedPrice,
    #[minifix(variant = "8")] TedYield,
    #[minifix(variant = "9")] YieldSpread,
    #[minifix(variant = "10")] Yield,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityIdSource {
    #[minifix(variant = "1")] Cusip,
    #[minifix(variant = "2")] Sedol,
    #[minifix(variant = "3")] Quik,
    #[minifix(variant = "4")] IsinNumber,
    #[minifix(variant = "5")] RicCode,
    #[minifix(variant = "6")] IsoCurrencyCode,
    #[minifix(variant = "7")] IsoCountryCode,
    #[minifix(variant = "8")] ExchangeSymbol,
    #[minifix(variant = "9")] ConsolidatedTapeAssociation,
    #[minifix(variant = "A")] BloombergSymbol,
    #[minifix(variant = "B")] Wertpapier,
    #[minifix(variant = "C")] Dutch,
    #[minifix(variant = "D")] Valoren,
    #[minifix(variant = "E")] Sicovam,
    #[minifix(variant = "F")] Belgian,
    #[minifix(variant = "G")] Common,
    #[minifix(variant = "H")] ClearingHouse,
    #[minifix(variant = "I")] IsdaFpmlProductSpecification,
    #[minifix(variant = "J")] OptionsPriceReportingAuthority,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ApplQueueResolution {
    #[minifix(variant = "0")] NoActionTaken,
    #[minifix(variant = "1")] QueueFlushed,
    #[minifix(variant = "2")] OverlayLast,
    #[minifix(variant = "3")] EndSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteRequestRejectReason {
    #[minifix(variant = "1")] UnknownSymbol,
    #[minifix(variant = "2")] Exchange,
    #[minifix(variant = "3")] QuoteRequestExceedsLimit,
    #[minifix(variant = "4")] TooLateToEnter,
    #[minifix(variant = "5")] InvalidPrice,
    #[minifix(variant = "6")] NotAuthorizedToRequestQuote,
    #[minifix(variant = "7")] NoMatchForInquiry,
    #[minifix(variant = "8")] NoMarketForInstrument,
    #[minifix(variant = "9")] NoInventory,
    #[minifix(variant = "10")] Pass,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TimeInForce {
    #[minifix(variant = "0")] Day,
    #[minifix(variant = "1")] GoodTillCancel,
    #[minifix(variant = "2")] AtTheOpening,
    #[minifix(variant = "3")] ImmediateOrCancel,
    #[minifix(variant = "4")] FillOrKill,
    #[minifix(variant = "5")] GoodTillCrossing,
    #[minifix(variant = "6")] GoodTillDate,
    #[minifix(variant = "7")] AtTheClose,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum WorkingIndicator {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TaxAdvantageType {
    #[minifix(variant = "0")] NoneNotApplicable,
    #[minifix(variant = "1")] MaxiIsa,
    #[minifix(variant = "2")] Tessa,
    #[minifix(variant = "3")] MiniCashIsa,
    #[minifix(variant = "4")] MiniStocksAndSharesIsa,
    #[minifix(variant = "5")] MiniInsuranceIsa,
    #[minifix(variant = "6")] CurrentYearPayment,
    #[minifix(variant = "7")] PriorYearPayment,
    #[minifix(variant = "8")] AssetTransfer,
    #[minifix(variant = "9")] Employee,
    #[minifix(variant = "10")] EmployeeCurrentYear,
    #[minifix(variant = "11")] Employer,
    #[minifix(variant = "12")] EmployerCurrentYear,
    #[minifix(variant = "13")] NonFundPrototypeIra,
    #[minifix(variant = "14")] NonFundQualifiedPlan,
    #[minifix(variant = "15")] DefinedContributionPlan,
    #[minifix(variant = "16")] IndividualRetirementAccount,
    #[minifix(variant = "17")] IndividualRetirementAccountRollover,
    #[minifix(variant = "18")] Keogh,
    #[minifix(variant = "19")] ProfitSharingPlan,
    #[minifix(variant = "20")] _401k,
    #[minifix(variant = "21")] SelfDirectedIra,
    #[minifix(variant = "22")] _403,
    #[minifix(variant = "23")] _457,
    #[minifix(variant = "24")] RothIra24,
    #[minifix(variant = "25")] RothIra25,
    #[minifix(variant = "26")] RothConversionIra26,
    #[minifix(variant = "27")] RothConversionIra27,
    #[minifix(variant = "28")] EducationIra28,
    #[minifix(variant = "29")] EducationIra29,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteCondition {
    #[minifix(variant = "A")] Open,
    #[minifix(variant = "B")] Closed,
    #[minifix(variant = "C")] ExchangeBest,
    #[minifix(variant = "D")] ConsolidatedBest,
    #[minifix(variant = "E")] Locked,
    #[minifix(variant = "F")] Crossed,
    #[minifix(variant = "G")] Depth,
    #[minifix(variant = "H")] FastTrading,
    #[minifix(variant = "I")] NonFirm,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OrdStatus {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] PartiallyFilled,
    #[minifix(variant = "2")] Filled,
    #[minifix(variant = "3")] DoneForDay,
    #[minifix(variant = "4")] Canceled,
    #[minifix(variant = "6")] PendingCancel,
    #[minifix(variant = "7")] Stopped,
    #[minifix(variant = "8")] Rejected,
    #[minifix(variant = "9")] Suspended,
    #[minifix(variant = "A")] PendingNew,
    #[minifix(variant = "B")] Calculated,
    #[minifix(variant = "C")] Expired,
    #[minifix(variant = "D")] AcceptedForBidding,
    #[minifix(variant = "E")] PendingReplace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LiquidityIndType {
    #[minifix(variant = "1")] _5dayMovingAverage,
    #[minifix(variant = "2")] _20DayMovingAverage,
    #[minifix(variant = "3")] NormalMarketSize,
    #[minifix(variant = "4")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionMoveType {
    #[minifix(variant = "0")] Floating,
    #[minifix(variant = "1")] Fixed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MassCancelResponse {
    #[minifix(variant = "0")] CancelRequestRejected,
    #[minifix(variant = "1")] CancelOrdersForASecurity,
    #[minifix(variant = "2")] CancelOrdersForAnUnderlyingSecurity,
    #[minifix(variant = "3")] CancelOrdersForAProduct,
    #[minifix(variant = "4")] CancelOrdersForACficode,
    #[minifix(variant = "5")] CancelOrdersForASecuritytype,
    #[minifix(variant = "6")] CancelOrdersForATradingSession,
    #[minifix(variant = "7")] CancelAllOrders,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum Product {
    #[minifix(variant = "1")] Agency,
    #[minifix(variant = "2")] Commodity,
    #[minifix(variant = "3")] Corporate,
    #[minifix(variant = "4")] Currency,
    #[minifix(variant = "5")] Equity,
    #[minifix(variant = "6")] Government,
    #[minifix(variant = "7")] Index,
    #[minifix(variant = "8")] Loan,
    #[minifix(variant = "9")] Moneymarket,
    #[minifix(variant = "10")] Mortgage,
    #[minifix(variant = "11")] Municipal,
    #[minifix(variant = "12")] Other,
    #[minifix(variant = "13")] Financing,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExecType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "3")] DoneForDay,
    #[minifix(variant = "4")] Canceled,
    #[minifix(variant = "5")] Replace,
    #[minifix(variant = "6")] PendingCancel,
    #[minifix(variant = "7")] Stopped,
    #[minifix(variant = "8")] Rejected,
    #[minifix(variant = "9")] Suspended,
    #[minifix(variant = "A")] PendingNew,
    #[minifix(variant = "B")] Calculated,
    #[minifix(variant = "C")] Expired,
    #[minifix(variant = "D")] Restated,
    #[minifix(variant = "E")] PendingReplace,
    #[minifix(variant = "F")] Trade,
    #[minifix(variant = "G")] TradeCorrect,
    #[minifix(variant = "H")] TradeCancel,
    #[minifix(variant = "I")] OrderStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OddLot {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CustOrderCapacity {
    #[minifix(variant = "1")] MemberTradingForTheirOwnAccount,
    #[minifix(variant = "2")] ClearingFirmTradingForItsProprietaryAccount,
    #[minifix(variant = "3")] MemberTradingForAnotherMember,
    #[minifix(variant = "4")] AllOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CashMargin {
    #[minifix(variant = "1")] Cash,
    #[minifix(variant = "2")] MarginOpen,
    #[minifix(variant = "3")] MarginClose,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradSesMode {
    #[minifix(variant = "1")] Testing,
    #[minifix(variant = "2")] Simulated,
    #[minifix(variant = "3")] Production,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionOffsetType {
    #[minifix(variant = "0")] Price,
    #[minifix(variant = "1")] BasisPoints,
    #[minifix(variant = "2")] Ticks,
    #[minifix(variant = "3")] PriceTier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlDeliveryType {
    #[minifix(variant = "0")] VersusPaymentDeliver,
    #[minifix(variant = "1")] FreeDeliver,
    #[minifix(variant = "2")] TriParty,
    #[minifix(variant = "3")] HoldInCustody,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityType {
    #[minifix(variant = "FUT")] Future,
    #[minifix(variant = "OPT")] Option,
    #[minifix(variant = "EUSUPRA")] EuroSupranationalCoupons,
    #[minifix(variant = "FAC")] FederalAgencyCoupon,
    #[minifix(variant = "FADN")] FederalAgencyDiscountNote,
    #[minifix(variant = "PEF")] PrivateExportFunding,
    #[minifix(variant = "SUPRA")] UsdSupranationalCoupons,
    #[minifix(variant = "CORP")] CorporateBond,
    #[minifix(variant = "CPP")] CorporatePrivatePlacement,
    #[minifix(variant = "CB")] ConvertibleBond,
    #[minifix(variant = "DUAL")] DualCurrency,
    #[minifix(variant = "EUCORP")] EuroCorporateBond,
    #[minifix(variant = "XLINKD")] IndexedLinked,
    #[minifix(variant = "STRUCT")] StructuredNotes,
    #[minifix(variant = "YANK")] YankeeCorporateBond,
    #[minifix(variant = "FOR")] ForeignExchangeContract,
    #[minifix(variant = "CS")] CommonStock,
    #[minifix(variant = "PS")] PreferredStock,
    #[minifix(variant = "BRADY")] BradyBond,
    #[minifix(variant = "EUSOV")] EuroSovereigns,
    #[minifix(variant = "TBOND")] UsTreasuryBond,
    #[minifix(variant = "TINT")] InterestStripFromAnyBondOrNote,
    #[minifix(variant = "TIPS")] TreasuryInflationProtectedSecurities,
    #[minifix(variant = "TCAL")] PrincipalStripOfACallableBondOrNote,
    #[minifix(variant = "TPRN")] PrincipalStripFromANonCallableBondOrNote,
    #[minifix(variant = "UST")] UsTreasuryNoteUst,
    #[minifix(variant = "USTB")] UsTreasuryBillUstb,
    #[minifix(variant = "TNOTE")] UsTreasuryNoteTnote,
    #[minifix(variant = "TBILL")] UsTreasuryBillTbill,
    #[minifix(variant = "REPO")] Repurchase,
    #[minifix(variant = "FORWARD")] Forward,
    #[minifix(variant = "BUYSELL")] BuySellback,
    #[minifix(variant = "SECLOAN")] SecuritiesLoan,
    #[minifix(variant = "SECPLEDGE")] SecuritiesPledge,
    #[minifix(variant = "TERM")] TermLoan,
    #[minifix(variant = "RVLV")] RevolverLoan,
    #[minifix(variant = "RVLVTRM")] RevolverTermLoan,
    #[minifix(variant = "BRIDGE")] BridgeLoan,
    #[minifix(variant = "LOFC")] LetterOfCredit,
    #[minifix(variant = "SWING")] SwingLineFacility,
    #[minifix(variant = "DINP")] DebtorInPossession,
    #[minifix(variant = "DEFLTED")] Defaulted,
    #[minifix(variant = "WITHDRN")] Withdrawn,
    #[minifix(variant = "REPLACD")] Replaced,
    #[minifix(variant = "MATURED")] Matured,
    #[minifix(variant = "AMENDED")] AmendedRestated,
    #[minifix(variant = "RETIRED")] Retired,
    #[minifix(variant = "BA")] BankersAcceptance,
    #[minifix(variant = "BN")] BankNotes,
    #[minifix(variant = "BOX")] BillOfExchanges,
    #[minifix(variant = "CD")] CertificateOfDeposit,
    #[minifix(variant = "CL")] CallLoans,
    #[minifix(variant = "CP")] CommercialPaper,
    #[minifix(variant = "DN")] DepositNotes,
    #[minifix(variant = "EUCD")] EuroCertificateOfDeposit,
    #[minifix(variant = "EUCP")] EuroCommercialPaper,
    #[minifix(variant = "LQN")] LiquidityNote,
    #[minifix(variant = "MTN")] MediumTermNotes,
    #[minifix(variant = "ONITE")] Overnight,
    #[minifix(variant = "PN")] PromissoryNote,
    #[minifix(variant = "PZFJ")] PlazosFijos,
    #[minifix(variant = "STN")] ShortTermLoanNote,
    #[minifix(variant = "TD")] TimeDeposit,
    #[minifix(variant = "XCN")] ExtendedCommNote,
    #[minifix(variant = "YCD")] YankeeCertificateOfDeposit,
    #[minifix(variant = "ABS")] AssetBackedSecurities,
    #[minifix(variant = "CMBS")] CorpMortgageBackedSecurities,
    #[minifix(variant = "CMO")] CollateralizedMortgageObligation,
    #[minifix(variant = "IET")] IoetteMortgage,
    #[minifix(variant = "MBS")] MortgageBackedSecurities,
    #[minifix(variant = "MIO")] MortgageInterestOnly,
    #[minifix(variant = "MPO")] MortgagePrincipalOnly,
    #[minifix(variant = "MPP")] MortgagePrivatePlacement,
    #[minifix(variant = "MPT")] MiscellaneousPassThrough,
    #[minifix(variant = "PFAND")] Pfandbriefe,
    #[minifix(variant = "TBA")] ToBeAnnounced,
    #[minifix(variant = "AN")] OtherAnticipationNotesBanGanEtc,
    #[minifix(variant = "COFO")] CertificateOfObligation,
    #[minifix(variant = "COFP")] CertificateOfParticipation,
    #[minifix(variant = "GO")] GeneralObligationBonds,
    #[minifix(variant = "MT")] MandatoryTender,
    #[minifix(variant = "RAN")] RevenueAnticipationNote,
    #[minifix(variant = "REV")] RevenueBonds,
    #[minifix(variant = "SPCLA")] SpecialAssessment,
    #[minifix(variant = "SPCLO")] SpecialObligation,
    #[minifix(variant = "SPCLT")] SpecialTax,
    #[minifix(variant = "TAN")] TaxAnticipationNote,
    #[minifix(variant = "TAXA")] TaxAllocation,
    #[minifix(variant = "TECP")] TaxExemptCommercialPaper,
    #[minifix(variant = "TRAN")] TaxRevenueAnticipationNote,
    #[minifix(variant = "VRDN")] VariableRateDemandNote,
    #[minifix(variant = "WAR")] Warrant,
    #[minifix(variant = "MF")] MutualFund,
    #[minifix(variant = "MLEG")] MultiLegInstrument,
    #[minifix(variant = "NONE")] NoSecurityType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum EmailType {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Reply,
    #[minifix(variant = "2")] AdminReply,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ProcessCode {
    #[minifix(variant = "0")] Regular,
    #[minifix(variant = "1")] SoftDollar,
    #[minifix(variant = "2")] StepIn,
    #[minifix(variant = "3")] StepOut,
    #[minifix(variant = "4")] SoftDollarStepIn,
    #[minifix(variant = "5")] SoftDollarStepOut,
    #[minifix(variant = "6")] PlanSponsor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TrdRptStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BidTradeType {
    #[minifix(variant = "R")] RiskTrade,
    #[minifix(variant = "G")] VwapGuarantee,
    #[minifix(variant = "A")] Agency,
    #[minifix(variant = "J")] GuaranteedClose,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeCondition {
    #[minifix(variant = "A")] Cash,
    #[minifix(variant = "B")] AveragePriceTrade,
    #[minifix(variant = "C")] CashTrade,
    #[minifix(variant = "D")] NextDay,
    #[minifix(variant = "E")] Opening,
    #[minifix(variant = "F")] IntradayTradeDetail,
    #[minifix(variant = "G")] Rule127Trade,
    #[minifix(variant = "H")] Rule155Trade,
    #[minifix(variant = "I")] SoldLast,
    #[minifix(variant = "J")] NextDayTrade,
    #[minifix(variant = "K")] Opened,
    #[minifix(variant = "L")] Seller,
    #[minifix(variant = "M")] Sold,
    #[minifix(variant = "N")] StoppedStock,
    #[minifix(variant = "P")] ImbalanceMoreBuyers,
    #[minifix(variant = "Q")] ImbalanceMoreSellers,
    #[minifix(variant = "R")] OpeningPrice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosType {
    #[minifix(variant = "TQ")] TransactionQuantity,
    #[minifix(variant = "IAS")] IntraSpreadQty,
    #[minifix(variant = "IES")] InterSpreadQty,
    #[minifix(variant = "FIN")] EndOfDayQty,
    #[minifix(variant = "SOD")] StartOfDayQty,
    #[minifix(variant = "EX")] OptionExerciseQty,
    #[minifix(variant = "AS")] OptionAssignment,
    #[minifix(variant = "TX")] TransactionFromExercise,
    #[minifix(variant = "TA")] TransactionFromAssignment,
    #[minifix(variant = "PIT")] PitTradeQty,
    #[minifix(variant = "TRF")] TransferTradeQty,
    #[minifix(variant = "ETR")] ElectronicTradeQty,
    #[minifix(variant = "ALC")] AllocationTradeQty,
    #[minifix(variant = "PA")] AdjustmentQty,
    #[minifix(variant = "ASF")] AsOfTradeQty,
    #[minifix(variant = "DLV")] DeliveryQty,
    #[minifix(variant = "TOT")] TotalTransactionQty,
    #[minifix(variant = "XM")] CrossMarginQty,
    #[minifix(variant = "SPL")] IntegralSplit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SubscriptionRequestType {
    #[minifix(variant = "0")] Snapshot,
    #[minifix(variant = "1")] SnapshotPlusUpdates,
    #[minifix(variant = "2")] DisablePreviousSnapshotPlusUpdateRequest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum Adjustment {
    #[minifix(variant = "1")] Cancel,
    #[minifix(variant = "2")] Error,
    #[minifix(variant = "3")] Correction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CxlRejReason {
    #[minifix(variant = "0")] TooLateToCancel,
    #[minifix(variant = "1")] UnknownOrder,
    #[minifix(variant = "2")] Broker,
    #[minifix(variant = "3")] OrderAlreadyInPendingCancelOrPendingReplaceStatus,
    #[minifix(variant = "4")] UnableToProcessOrderMassCancelRequest,
    #[minifix(variant = "5")] Origordmodtime,
    #[minifix(variant = "6")] DuplicateClordid,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IoiQltyInd {
    #[minifix(variant = "L")] Low,
    #[minifix(variant = "M")] Medium,
    #[minifix(variant = "H")] High,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum FinancialStatus {
    #[minifix(variant = "1")] Bankrupt,
    #[minifix(variant = "2")] PendingDelisting,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AdvTransType {
    #[minifix(variant = "N")] New,
    #[minifix(variant = "C")] Cancel,
    #[minifix(variant = "R")] Replace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocHandlInst {
    #[minifix(variant = "1")] Match,
    #[minifix(variant = "2")] Forward,
    #[minifix(variant = "3")] ForwardAndMatch,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocNoOrdersType {
    #[minifix(variant = "0")] NotSpecified,
    #[minifix(variant = "1")] ExplicitListProvided,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExchangeForPhysical {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExpirationCycle {
    #[minifix(variant = "0")] ExpireOnTradingSessionClose,
    #[minifix(variant = "1")] ExpireOnTradingSessionOpen,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PegLimitType {
    #[minifix(variant = "0")] OrBetter,
    #[minifix(variant = "1")] StrictLimitIsAStrictLimit,
    #[minifix(variant = "2")] OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MassCancelRejectReason {
    #[minifix(variant = "0")] MassCancelNotSupported,
    #[minifix(variant = "1")] InvalidOrUnknownSecurity,
    #[minifix(variant = "2")] InvalidOrUnknownUnderlying,
    #[minifix(variant = "3")] InvalidOrUnknownProduct,
    #[minifix(variant = "4")] InvalidOrUnknownCficode,
    #[minifix(variant = "5")] InvalidOrUnknownSecurityType,
    #[minifix(variant = "6")] InvalidOrUnknownTradingSession,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteRejectReason {
    #[minifix(variant = "1")] UnknownSymbol,
    #[minifix(variant = "2")] Exchange,
    #[minifix(variant = "3")] QuoteRequestExceedsLimit,
    #[minifix(variant = "4")] TooLateToEnter,
    #[minifix(variant = "5")] UnknownQuote,
    #[minifix(variant = "6")] DuplicateQuote,
    #[minifix(variant = "7")] InvalidBidAskSpread,
    #[minifix(variant = "8")] InvalidPrice,
    #[minifix(variant = "9")] NotAuthorizedToQuoteSecurity,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ListExecInstType {
    #[minifix(variant = "1")] Immediate,
    #[minifix(variant = "2")] WaitForExecuteInstruction,
    #[minifix(variant = "3")] ExchangeSwitchCivOrderSellDriven,
    #[minifix(variant = "4")] ExchangeSwitchCivOrderBuyDrivenCashTopUp,
    #[minifix(variant = "5")] ExchangeSwitchCivOrderBuyDrivenCashWithdraw,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MassStatusReqType {
    #[minifix(variant = "1")] StatusForOrdersForASecurity,
    #[minifix(variant = "2")] StatusForOrdersForAnUnderlyingSecurity,
    #[minifix(variant = "3")] StatusForOrdersForAProduct,
    #[minifix(variant = "4")] StatusForOrdersForACficode,
    #[minifix(variant = "5")] StatusForOrdersForASecuritytype,
    #[minifix(variant = "6")] StatusForOrdersForATradingSession,
    #[minifix(variant = "7")] StatusForAllOrders,
    #[minifix(variant = "8")] StatusForOrdersForAPartyid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum EventType {
    #[minifix(variant = "1")] Put,
    #[minifix(variant = "2")] Call,
    #[minifix(variant = "3")] Tender,
    #[minifix(variant = "4")] SinkingFundCall,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlPriceType {
    #[minifix(variant = "1")] Final,
    #[minifix(variant = "2")] Theoretical,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlInstReqRejCode {
    #[minifix(variant = "0")] UnableToProcessRequest,
    #[minifix(variant = "1")] UnknownAccount,
    #[minifix(variant = "2")] NoMatchingSettlementInstructionsFound,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MiscFeeType {
    #[minifix(variant = "1")] Regulatory,
    #[minifix(variant = "2")] Tax,
    #[minifix(variant = "3")] LocalCommission,
    #[minifix(variant = "4")] ExchangeFees,
    #[minifix(variant = "5")] Stamp,
    #[minifix(variant = "6")] Levy,
    #[minifix(variant = "7")] Other,
    #[minifix(variant = "8")] Markup,
    #[minifix(variant = "9")] ConsumptionTax,
    #[minifix(variant = "10")] PerTransaction,
    #[minifix(variant = "11")] Conversion,
    #[minifix(variant = "12")] Agent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SecurityRequestResult {
    #[minifix(variant = "0")] ValidRequest,
    #[minifix(variant = "1")] InvalidOrUnsupportedRequest,
    #[minifix(variant = "2")] NoInstrumentsFoundThatMatchSelectionCriteria,
    #[minifix(variant = "3")] NotAuthorizedToRetrieveInstrumentData,
    #[minifix(variant = "4")] InstrumentDataTemporarilyUnavailable,
    #[minifix(variant = "5")] RequestForInstrumentDataNotSupported,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BasisPxType {
    #[minifix(variant = "2")] ClosingPriceAtMorningSession,
    #[minifix(variant = "3")] ClosingPrice,
    #[minifix(variant = "4")] CurrentPrice,
    #[minifix(variant = "5")] Sq,
    #[minifix(variant = "6")] VwapThroughADay,
    #[minifix(variant = "7")] VwapThroughAMorningSession,
    #[minifix(variant = "8")] VwapThroughAnAfternoonSession,
    #[minifix(variant = "9")] VwapThroughADayExceptYori,
    #[minifix(variant = "A")] VwapThroughAMorningSessionExceptYori,
    #[minifix(variant = "B")] VwapThroughAnAfternoonSessionExceptYori,
    #[minifix(variant = "C")] Strike,
    #[minifix(variant = "D")] Open,
    #[minifix(variant = "Z")] Others,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IoiNaturalFlag {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocType {
    #[minifix(variant = "1")] Calculated,
    #[minifix(variant = "2")] Preliminary,
    #[minifix(variant = "5")] ReadyToBook,
    #[minifix(variant = "7")] WarehouseInstruction,
    #[minifix(variant = "8")] RequestToIntermediary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlSessId {
    #[minifix(variant = "ITD")] Intraday,
    #[minifix(variant = "RTH")] RegularTradingHours,
    #[minifix(variant = "ETH")] ElectronicTradingHours,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExecPriceType {
    #[minifix(variant = "B")] BidPrice,
    #[minifix(variant = "C")] CreationPrice,
    #[minifix(variant = "D")] CreationPricePlusAdjustment,
    #[minifix(variant = "E")] CreationPricePlusAdjustmentAmount,
    #[minifix(variant = "O")] OfferPrice,
    #[minifix(variant = "P")] OfferPriceMinusAdjustment,
    #[minifix(variant = "Q")] OfferPriceMinusAdjustmentAmount,
    #[minifix(variant = "S")] SinglePrice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PegRoundDirection {
    #[minifix(variant = "1")] MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
    #[minifix(variant = "2")] MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SolicitedFlag {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AssignmentMethod {
    #[minifix(variant = "R")] Random,
    #[minifix(variant = "P")] Prorata,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OrderRestrictions {
    #[minifix(variant = "1")] ProgramTrade,
    #[minifix(variant = "2")] IndexArbitrage,
    #[minifix(variant = "3")] NonIndexArbitrage,
    #[minifix(variant = "4")] CompetingMarketMaker,
    #[minifix(variant = "5")] ActingAsMarketMakerOrSpecialistInTheSecurity,
    #[minifix(variant = "6")] ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
    #[minifix(variant = "7")] ForeignEntity,
    #[minifix(variant = "8")] ExternalMarketParticipant,
    #[minifix(variant = "9")] ExternalInterConnectedMarketLinkage,
    #[minifix(variant = "A")] RisklessArbitrage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocRejCode {
    #[minifix(variant = "0")] UnknownAccount,
    #[minifix(variant = "1")] IncorrectQuantity,
    #[minifix(variant = "2")] IncorrectAveragePrice,
    #[minifix(variant = "3")] UnknownExecutingBrokerMnemonic,
    #[minifix(variant = "4")] CommissionDifference,
    #[minifix(variant = "5")] UnknownOrderid,
    #[minifix(variant = "6")] UnknownListid,
    #[minifix(variant = "7")] Other,
    #[minifix(variant = "8")] IncorrectAllocatedQuantity,
    #[minifix(variant = "9")] CalculationDifference,
    #[minifix(variant = "10")] UnknownOrStaleExecid,
    #[minifix(variant = "11")] MismatchedDataValue,
    #[minifix(variant = "12")] UnknownClordid,
    #[minifix(variant = "13")] WarehouseRequestRejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CxlRejResponseTo {
    #[minifix(variant = "1")] OrderCancelRequest,
    #[minifix(variant = "2")] OrderCancelReplaceRequest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PreviouslyReported {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PositionEffect {
    #[minifix(variant = "O")] Open,
    #[minifix(variant = "C")] Close,
    #[minifix(variant = "R")] Rolled,
    #[minifix(variant = "F")] Fifo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AcctIdSource {
    #[minifix(variant = "1")] Bic,
    #[minifix(variant = "2")] SidCode,
    #[minifix(variant = "3")] Tfm,
    #[minifix(variant = "4")] Omgeo,
    #[minifix(variant = "5")] DtccCode,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MatchStatus {
    #[minifix(variant = "0")] ComparedMatchedOrAffirmed,
    #[minifix(variant = "1")] UncomparedUnmatchedOrUnaffirmed,
    #[minifix(variant = "2")] AdvisoryOrAlert,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum NetworkRequestType {
    #[minifix(variant = "1")] Snapshot,
    #[minifix(variant = "2")] Subscribe,
    #[minifix(variant = "4")] StopSubscribing,
    #[minifix(variant = "8")] LevelOfDetailThenNocompidsBecomesRequired,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DkReason {
    #[minifix(variant = "A")] UnknownSymbol,
    #[minifix(variant = "B")] WrongSide,
    #[minifix(variant = "C")] QuantityExceedsOrder,
    #[minifix(variant = "D")] NoMatchingOrder,
    #[minifix(variant = "E")] PriceExceedsLimit,
    #[minifix(variant = "F")] CalculationDifference,
    #[minifix(variant = "Z")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MdUpdateAction {
    #[minifix(variant = "0")] New,
    #[minifix(variant = "1")] Change,
    #[minifix(variant = "2")] Delete,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CrossPrioritization {
    #[minifix(variant = "0")] None,
    #[minifix(variant = "1")] BuySideIsPrioritized,
    #[minifix(variant = "2")] SellSideIsPrioritized,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum InViewOfCommon {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PegMoveType {
    #[minifix(variant = "0")] Floating,
    #[minifix(variant = "1")] Fixed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IoiQty {
    #[minifix(variant = "S")] Small,
    #[minifix(variant = "M")] Medium,
    #[minifix(variant = "L")] Large,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DistribPaymentMethod {
    #[minifix(variant = "1")] Crest,
    #[minifix(variant = "2")] Nscc,
    #[minifix(variant = "3")] Euroclear,
    #[minifix(variant = "4")] Clearstream,
    #[minifix(variant = "5")] Cheque,
    #[minifix(variant = "6")] TelegraphicTransfer,
    #[minifix(variant = "7")] Fedwire,
    #[minifix(variant = "8")] DirectCredit,
    #[minifix(variant = "9")] AchCredit,
    #[minifix(variant = "10")] Bpay,
    #[minifix(variant = "11")] HighValueClearingSystem,
    #[minifix(variant = "12")] ReinvestInFund,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExecRestatementReason {
    #[minifix(variant = "0")] GtCorporateAction,
    #[minifix(variant = "1")] GtRenewal,
    #[minifix(variant = "2")] VerbalChange,
    #[minifix(variant = "3")] RepricingOfOrder,
    #[minifix(variant = "4")] BrokerOption,
    #[minifix(variant = "5")] PartialDeclineOfOrderqty,
    #[minifix(variant = "6")] CancelOnTradingHalt,
    #[minifix(variant = "7")] CancelOnSystemFailure,
    #[minifix(variant = "8")] Market,
    #[minifix(variant = "9")] CanceledNotBest,
    #[minifix(variant = "10")] WarehouseRecap,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ExecInst {
    #[minifix(variant = "1")] NotHeld,
    #[minifix(variant = "2")] Work,
    #[minifix(variant = "3")] GoAlong,
    #[minifix(variant = "4")] OverTheDay,
    #[minifix(variant = "5")] Held,
    #[minifix(variant = "6")] ParticipateDontInitiate,
    #[minifix(variant = "7")] StrictScale,
    #[minifix(variant = "8")] TryToScale,
    #[minifix(variant = "9")] StayOnBidside,
    #[minifix(variant = "0")] StayOnOfferside,
    #[minifix(variant = "A")] NoCross,
    #[minifix(variant = "B")] OkToCross,
    #[minifix(variant = "C")] CallFirst,
    #[minifix(variant = "D")] PercentOfVolume,
    #[minifix(variant = "E")] DoNotIncrease,
    #[minifix(variant = "F")] DoNotReduce,
    #[minifix(variant = "G")] AllOrNone,
    #[minifix(variant = "H")] ReinstateOnSystemFailure,
    #[minifix(variant = "I")] InstitutionsOnly,
    #[minifix(variant = "J")] ReinstateOnTradingHalt,
    #[minifix(variant = "K")] CancelOnTradingHalt,
    #[minifix(variant = "L")] LastPeg,
    #[minifix(variant = "M")] MidPricePeg,
    #[minifix(variant = "N")] NonNegotiable,
    #[minifix(variant = "O")] OpeningPeg,
    #[minifix(variant = "P")] MarketPeg,
    #[minifix(variant = "Q")] CancelOnSystemFailure,
    #[minifix(variant = "R")] PrimaryPeg,
    #[minifix(variant = "S")] Suspend,
    #[minifix(variant = "U")] CustomerDisplayInstruction,
    #[minifix(variant = "V")] Netting,
    #[minifix(variant = "W")] PegToVwap,
    #[minifix(variant = "X")] TradeAlong,
    #[minifix(variant = "Y")] TryToStop,
    #[minifix(variant = "Z")] CancelIfNotBest,
    #[minifix(variant = "a")] TrailingStopPeg,
    #[minifix(variant = "b")] StrictLimit,
    #[minifix(variant = "c")] IgnorePriceValidityChecks,
    #[minifix(variant = "d")] PegToLimitPrice,
    #[minifix(variant = "e")] WorkToTargetStrategy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum HandlInst {
    #[minifix(variant = "1")] AutomatedExecutionOrderPrivateNoBrokerIntervention,
    #[minifix(variant = "2")] AutomatedExecutionOrderPublicBrokerInterventionOk,
    #[minifix(variant = "3")] ManualOrderBestExecution,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteResponseLevel {
    #[minifix(variant = "0")] NoAcknowledgement,
    #[minifix(variant = "1")] AcknowledgeOnlyNegativeOrErroneousQuotes,
    #[minifix(variant = "2")] AcknowledgeEachQuoteMessages,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum RoutingType {
    #[minifix(variant = "1")] TargetFirm,
    #[minifix(variant = "2")] TargetList,
    #[minifix(variant = "3")] BlockFirm,
    #[minifix(variant = "4")] BlockList,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum NetworkStatusResponseType {
    #[minifix(variant = "1")] Full,
    #[minifix(variant = "2")] IncrementalUpdate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum StatusValue {
    #[minifix(variant = "1")] Connected,
    #[minifix(variant = "2")] NotConnectedDownExpectedUp,
    #[minifix(variant = "3")] NotConnectedDownExpectedDown,
    #[minifix(variant = "4")] InProcess,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OrdType {
    #[minifix(variant = "1")] Market,
    #[minifix(variant = "2")] Limit,
    #[minifix(variant = "3")] Stop,
    #[minifix(variant = "4")] StopLimit,
    #[minifix(variant = "6")] WithOrWithout,
    #[minifix(variant = "7")] LimitOrBetter,
    #[minifix(variant = "8")] LimitWithOrWithout,
    #[minifix(variant = "9")] OnBasis,
    #[minifix(variant = "D")] PreviouslyQuoted,
    #[minifix(variant = "E")] PreviouslyIndicated,
    #[minifix(variant = "G")] Forex,
    #[minifix(variant = "I")] Funari,
    #[minifix(variant = "J")] MarketIfTouched,
    #[minifix(variant = "K")] MarketWithLeftoverAsLimit,
    #[minifix(variant = "L")] PreviousFundValuationPoint,
    #[minifix(variant = "M")] NextFundValuationPoint,
    #[minifix(variant = "P")] Pegged,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MsgType {
    #[minifix(variant = "0")] Heartbeat,
    #[minifix(variant = "1")] TestRequest,
    #[minifix(variant = "2")] ResendRequest,
    #[minifix(variant = "3")] Reject,
    #[minifix(variant = "4")] SequenceReset,
    #[minifix(variant = "5")] Logout,
    #[minifix(variant = "6")] IndicationOfInterest,
    #[minifix(variant = "7")] Advertisement,
    #[minifix(variant = "8")] ExecutionReport,
    #[minifix(variant = "9")] OrderCancelReject,
    #[minifix(variant = "A")] Logon,
    #[minifix(variant = "B")] News,
    #[minifix(variant = "C")] Email,
    #[minifix(variant = "D")] OrderSingle,
    #[minifix(variant = "E")] OrderList,
    #[minifix(variant = "F")] OrderCancelRequest,
    #[minifix(variant = "G")] OrderCancelReplaceRequest,
    #[minifix(variant = "H")] OrderStatusRequest,
    #[minifix(variant = "J")] AllocationInstruction,
    #[minifix(variant = "K")] ListCancelRequest,
    #[minifix(variant = "L")] ListExecute,
    #[minifix(variant = "M")] ListStatusRequest,
    #[minifix(variant = "N")] ListStatus,
    #[minifix(variant = "P")] AllocationInstructionAck,
    #[minifix(variant = "Q")] DontKnowTrade,
    #[minifix(variant = "R")] QuoteRequest,
    #[minifix(variant = "S")] Quote,
    #[minifix(variant = "T")] SettlementInstructions,
    #[minifix(variant = "V")] MarketDataRequest,
    #[minifix(variant = "W")] MarketDataSnapshotFullRefresh,
    #[minifix(variant = "X")] MarketDataIncrementalRefresh,
    #[minifix(variant = "Y")] MarketDataRequestReject,
    #[minifix(variant = "Z")] QuoteCancel,
    #[minifix(variant = "a")] QuoteStatusRequest,
    #[minifix(variant = "b")] MassQuoteAcknowledgement,
    #[minifix(variant = "c")] SecurityDefinitionRequest,
    #[minifix(variant = "d")] SecurityDefinition,
    #[minifix(variant = "e")] SecurityStatusRequest,
    #[minifix(variant = "f")] SecurityStatus,
    #[minifix(variant = "g")] TradingSessionStatusRequest,
    #[minifix(variant = "h")] TradingSessionStatus,
    #[minifix(variant = "i")] MassQuote,
    #[minifix(variant = "j")] BusinessMessageReject,
    #[minifix(variant = "k")] BidRequest,
    #[minifix(variant = "l")] BidResponse,
    #[minifix(variant = "m")] ListStrikePrice,
    #[minifix(variant = "n")] XmlMessage,
    #[minifix(variant = "o")] RegistrationInstructions,
    #[minifix(variant = "p")] RegistrationInstructionsResponse,
    #[minifix(variant = "q")] OrderMassCancelRequest,
    #[minifix(variant = "r")] OrderMassCancelReport,
    #[minifix(variant = "s")] NewOrderS,
    #[minifix(variant = "t")] CrossOrderCancelReplaceRequest,
    #[minifix(variant = "u")] CrossOrderCancelRequest,
    #[minifix(variant = "v")] SecurityTypeRequest,
    #[minifix(variant = "w")] SecurityTypes,
    #[minifix(variant = "x")] SecurityListRequest,
    #[minifix(variant = "y")] SecurityList,
    #[minifix(variant = "z")] DerivativeSecurityListRequest,
    #[minifix(variant = "AA")] DerivativeSecurityList,
    #[minifix(variant = "AB")] NewOrderAb,
    #[minifix(variant = "AC")] MultilegOrderCancelReplace,
    #[minifix(variant = "AD")] TradeCaptureReportRequest,
    #[minifix(variant = "AE")] TradeCaptureReport,
    #[minifix(variant = "AF")] OrderMassStatusRequest,
    #[minifix(variant = "AG")] QuoteRequestReject,
    #[minifix(variant = "AH")] RfqRequest,
    #[minifix(variant = "AI")] QuoteStatusReport,
    #[minifix(variant = "AJ")] QuoteResponse,
    #[minifix(variant = "AK")] Confirmation,
    #[minifix(variant = "AL")] PositionMaintenanceRequest,
    #[minifix(variant = "AM")] PositionMaintenanceReport,
    #[minifix(variant = "AN")] RequestForPositions,
    #[minifix(variant = "AO")] RequestForPositionsAck,
    #[minifix(variant = "AP")] PositionReport,
    #[minifix(variant = "AQ")] TradeCaptureReportRequestAck,
    #[minifix(variant = "AR")] TradeCaptureReportAck,
    #[minifix(variant = "AS")] AllocationReport,
    #[minifix(variant = "AT")] AllocationReportAck,
    #[minifix(variant = "AU")] ConfirmationAck,
    #[minifix(variant = "AV")] SettlementInstructionRequest,
    #[minifix(variant = "AW")] AssignmentReport,
    #[minifix(variant = "AX")] CollateralRequest,
    #[minifix(variant = "AY")] CollateralAssignment,
    #[minifix(variant = "AZ")] CollateralResponse,
    #[minifix(variant = "BA")] CollateralReport,
    #[minifix(variant = "BB")] CollateralInquiry,
    #[minifix(variant = "BC")] NetworkBc,
    #[minifix(variant = "BD")] NetworkBd,
    #[minifix(variant = "BE")] UserRequest,
    #[minifix(variant = "BF")] UserResponse,
    #[minifix(variant = "BG")] CollateralInquiryAck,
    #[minifix(variant = "BH")] ConfirmationRequest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradedFlatSwitch {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ShortSaleReason {
    #[minifix(variant = "0")] DealerSoldShort,
    #[minifix(variant = "1")] DealerSoldShortExempt,
    #[minifix(variant = "2")] SellingCustomerSoldShort,
    #[minifix(variant = "3")] SellingCustomerSoldShortExempt,
    #[minifix(variant = "4")] QualifedServiceRepresentative,
    #[minifix(variant = "5")] QsrOrAguContraSideSoldShortExempt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DiscretionRoundDirection {
    #[minifix(variant = "1")] MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick,
    #[minifix(variant = "2")] MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollStatus {
    #[minifix(variant = "0")] Unassigned,
    #[minifix(variant = "1")] PartiallyAssigned,
    #[minifix(variant = "2")] AssignmentProposed,
    #[minifix(variant = "3")] Assigned,
    #[minifix(variant = "4")] Challenged,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum StipulationType {
    #[minifix(variant = "AMT")] Amt,
    #[minifix(variant = "AUTOREINV")] AutoReinvestmentAtRateOrBetter,
    #[minifix(variant = "BANKQUAL")] BankQualified,
    #[minifix(variant = "BGNCON")] BargainConditionsSee,
    #[minifix(variant = "COUPON")] CouponRange,
    #[minifix(variant = "CURRENCY")] IsoCurrencyCode,
    #[minifix(variant = "CUSTOMDATE")] CustomStartEndDate,
    #[minifix(variant = "GEOG")] GeographicsAndRange,
    #[minifix(variant = "HAIRCUT")] ValuationDiscount,
    #[minifix(variant = "INSURED")] Insured,
    #[minifix(variant = "ISSUE")] YearOrYearMonthOfIssue,
    #[minifix(variant = "ISSUER")] IssuersTicker,
    #[minifix(variant = "ISSUESIZE")] IssueSizeRange,
    #[minifix(variant = "LOOKBACK")] LookbackDays,
    #[minifix(variant = "LOT")] ExplicitLotIdentifier,
    #[minifix(variant = "LOTVAR")] LotVariance,
    #[minifix(variant = "MAT")] MaturityYearAndMonth,
    #[minifix(variant = "MATURITY")] MaturityRange,
    #[minifix(variant = "MAXSUBS")] MaximumSubstitutions,
    #[minifix(variant = "MINQTY")] MinimumQuantity,
    #[minifix(variant = "MININCR")] MinimumIncrement,
    #[minifix(variant = "MINDNOM")] MinimumDenomination,
    #[minifix(variant = "PAYFREQ")] PaymentFrequencyCalendar,
    #[minifix(variant = "PIECES")] NumberOfPieces,
    #[minifix(variant = "PMAX")] PoolsMaximum,
    #[minifix(variant = "PPM")] PoolsPerMillion,
    #[minifix(variant = "PPL")] PoolsPerLot,
    #[minifix(variant = "PPT")] PoolsPerTrade,
    #[minifix(variant = "PRICE")] PriceRange,
    #[minifix(variant = "PRICEFREQ")] PricingFrequency,
    #[minifix(variant = "PROD")] ProductionYear,
    #[minifix(variant = "PROTECT")] CallProtection,
    #[minifix(variant = "PURPOSE")] Purpose,
    #[minifix(variant = "PXSOURCE")] BenchmarkPriceSource,
    #[minifix(variant = "RATING")] RatingSourceAndRange,
    #[minifix(variant = "REDEMPTION")] TypeOfRedemptionValuesAreNoncallableCallablePrefundedEscrowedtomaturityPutableConvertible,
    #[minifix(variant = "RESTRICTED")] Restricted,
    #[minifix(variant = "SECTOR")] MarketSector,
    #[minifix(variant = "SECTYPE")] SecuritytypeIncludedOrExcluded,
    #[minifix(variant = "STRUCT")] Structure,
    #[minifix(variant = "SUBSFREQ")] SubstitutionsFrequency,
    #[minifix(variant = "SUBSLEFT")] SubstitutionsLeft,
    #[minifix(variant = "TEXT")] FreeformText,
    #[minifix(variant = "TRDVAR")] TradeVariance,
    #[minifix(variant = "WAC")] WeightedAverageCouponvalueInPercent,
    #[minifix(variant = "WAL")] WeightedAverageLifeCouponValueInPercent,
    #[minifix(variant = "WALA")] WeightedAverageLoanAgeValueInMonths,
    #[minifix(variant = "WAM")] WeightedAverageMaturityValueInMonths,
    #[minifix(variant = "WHOLE")] WholePool,
    #[minifix(variant = "YIELD")] YieldRange,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum MsgDirection {
    #[minifix(variant = "S")] Send,
    #[minifix(variant = "R")] Receive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum RegistStatus {
    #[minifix(variant = "A")] Accepted,
    #[minifix(variant = "R")] Rejected,
    #[minifix(variant = "H")] Held,
    #[minifix(variant = "N")] ReminderIeRegistrationInstructionsAreStillOutstanding,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ListStatusType {
    #[minifix(variant = "1")] Ack,
    #[minifix(variant = "2")] Response,
    #[minifix(variant = "3")] Timed,
    #[minifix(variant = "4")] Execstarted,
    #[minifix(variant = "5")] Alldone,
    #[minifix(variant = "6")] Alert,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AccountType {
    #[minifix(variant = "1")] AccountIsCarriedOnCustomerSideOfBooks,
    #[minifix(variant = "2")] AccountIsCarriedOnNonCustomerSideOfBooks,
    #[minifix(variant = "3")] HouseTrader,
    #[minifix(variant = "4")] FloorTrader,
    #[minifix(variant = "6")] AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
    #[minifix(variant = "7")] AccountIsHouseTraderAndIsCrossMargined,
    #[minifix(variant = "8")] JointBackofficeAccount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TerminationType {
    #[minifix(variant = "1")] Overnight,
    #[minifix(variant = "2")] Term,
    #[minifix(variant = "3")] Flexible,
    #[minifix(variant = "4")] Open,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QtyType {
    #[minifix(variant = "0")] Units,
    #[minifix(variant = "1")] Contracts,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum DueToRelated {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlInstTransType {
    #[minifix(variant = "N")] New,
    #[minifix(variant = "C")] Cancel,
    #[minifix(variant = "R")] Replace,
    #[minifix(variant = "T")] Restate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OwnerType {
    #[minifix(variant = "1")] IndividualInvestor,
    #[minifix(variant = "2")] PublicCompany,
    #[minifix(variant = "3")] PrivateCompany,
    #[minifix(variant = "4")] IndividualTrustee,
    #[minifix(variant = "5")] CompanyTrustee,
    #[minifix(variant = "6")] PensionPlan,
    #[minifix(variant = "7")] CustodianUnderGiftsToMinorsAct,
    #[minifix(variant = "8")] Trusts,
    #[minifix(variant = "9")] Fiduciaries,
    #[minifix(variant = "10")] NetworkingSubAccount,
    #[minifix(variant = "11")] NonProfitOrganization,
    #[minifix(variant = "12")] CorporateBody,
    #[minifix(variant = "13")] Nominee,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SessionRejectReason {
    #[minifix(variant = "0")] InvalidTagNumber,
    #[minifix(variant = "1")] RequiredTagMissing,
    #[minifix(variant = "2")] TagNotDefinedForThisMessageType,
    #[minifix(variant = "3")] UndefinedTag,
    #[minifix(variant = "4")] TagSpecifiedWithoutAValue,
    #[minifix(variant = "5")] ValueIsIncorrect,
    #[minifix(variant = "6")] IncorrectDataFormatForValue,
    #[minifix(variant = "7")] DecryptionProblem,
    #[minifix(variant = "8")] SignatureProblem,
    #[minifix(variant = "9")] CompidProblem,
    #[minifix(variant = "10")] SendingtimeAccuracyProblem,
    #[minifix(variant = "11")] InvalidMsgtype,
    #[minifix(variant = "12")] XmlValidationError,
    #[minifix(variant = "13")] TagAppearsMoreThanOnce,
    #[minifix(variant = "14")] TagSpecifiedOutOfRequiredOrder,
    #[minifix(variant = "15")] RepeatingGroupFieldsOutOfOrder,
    #[minifix(variant = "16")] IncorrectNumingroupCountForRepeatingGroup,
    #[minifix(variant = "17")] NonDataValueIncludesFieldDelimiter,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum NetGrossInd {
    #[minifix(variant = "1")] Net,
    #[minifix(variant = "2")] Gross,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosReqStatus {
    #[minifix(variant = "0")] Completed,
    #[minifix(variant = "1")] CompletedWithWarnings,
    #[minifix(variant = "2")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum AllocAccountType {
    #[minifix(variant = "1")] AccountIsCarriedOnCustomerSideOfBooks,
    #[minifix(variant = "2")] AccountIsCarriedOnNonCustomerSideOfBooks,
    #[minifix(variant = "3")] HouseTrader,
    #[minifix(variant = "4")] FloorTrader,
    #[minifix(variant = "6")] AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
    #[minifix(variant = "7")] AccountIsHouseTraderAndIsCrossMargined,
    #[minifix(variant = "8")] JointBackofficeAccount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ForexReq {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LastLiquidityInd {
    #[minifix(variant = "1")] AddedLiquidity,
    #[minifix(variant = "2")] RemovedLiquidity,
    #[minifix(variant = "3")] LiquidityRoutedOut,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosQtyStatus {
    #[minifix(variant = "0")] Submitted,
    #[minifix(variant = "1")] Accepted,
    #[minifix(variant = "2")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosTransType {
    #[minifix(variant = "1")] Exercise,
    #[minifix(variant = "2")] DoNotExercise,
    #[minifix(variant = "3")] PositionAdjustment,
    #[minifix(variant = "4")] PositionChangeSubmissionMarginDisposition,
    #[minifix(variant = "5")] Pledge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum EncryptMethod {
    #[minifix(variant = "0")] None,
    #[minifix(variant = "1")] Pkcs,
    #[minifix(variant = "2")] Des,
    #[minifix(variant = "3")] PkcsDes,
    #[minifix(variant = "4")] PgpDes,
    #[minifix(variant = "5")] PgpDesMd5,
    #[minifix(variant = "6")] PemDesMd5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeAllocIndicator {
    #[minifix(variant = "0")] AllocationNotRequired,
    #[minifix(variant = "1")] AllocationRequired,
    #[minifix(variant = "2")] UseAllocationProvidedWithTheTrade,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum IoiTransType {
    #[minifix(variant = "N")] New,
    #[minifix(variant = "C")] Cancel,
    #[minifix(variant = "R")] Replace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollAsgnReason {
    #[minifix(variant = "0")] Initial,
    #[minifix(variant = "1")] Scheduled,
    #[minifix(variant = "2")] TimeWarning,
    #[minifix(variant = "3")] MarginDeficiency,
    #[minifix(variant = "4")] MarginExcess,
    #[minifix(variant = "5")] ForwardCollateralDemand,
    #[minifix(variant = "6")] EventOfDefault,
    #[minifix(variant = "7")] AdverseTaxEvent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradeRequestStatus {
    #[minifix(variant = "0")] Accepted,
    #[minifix(variant = "1")] Completed,
    #[minifix(variant = "2")] Rejected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteRequestType {
    #[minifix(variant = "1")] Manual,
    #[minifix(variant = "2")] Automatic,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BidRequestTransType {
    #[minifix(variant = "N")] New,
    #[minifix(variant = "C")] Cancel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CollAction {
    #[minifix(variant = "0")] Retain,
    #[minifix(variant = "1")] Add,
    #[minifix(variant = "2")] Remove,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum RegistRejReasonCode {
    #[minifix(variant = "1")] InvalidUnacceptableAccountType,
    #[minifix(variant = "2")] InvalidUnacceptableTaxExemptType,
    #[minifix(variant = "3")] InvalidUnacceptableOwnershipType,
    #[minifix(variant = "4")] InvalidUnacceptableNoRegDetls,
    #[minifix(variant = "5")] InvalidUnacceptableRegSeqNo,
    #[minifix(variant = "6")] InvalidUnacceptableRegDtls,
    #[minifix(variant = "7")] InvalidUnacceptableMailingDtls,
    #[minifix(variant = "8")] InvalidUnacceptableMailingInst,
    #[minifix(variant = "9")] InvalidUnacceptableInvestorId,
    #[minifix(variant = "10")] InvalidUnacceptableInvestorIdSource,
    #[minifix(variant = "11")] InvalidUnacceptableDateOfBirth,
    #[minifix(variant = "12")] InvalidUnacceptableInvestorCountryOfResidence,
    #[minifix(variant = "13")] InvalidUnacceptableNodistribinstns,
    #[minifix(variant = "14")] InvalidUnacceptableDistribPercentage,
    #[minifix(variant = "15")] InvalidUnacceptableDistribPaymentMethod,
    #[minifix(variant = "16")] InvalidUnacceptableCashDistribAgentAcctName,
    #[minifix(variant = "17")] InvalidUnacceptableCashDistribAgentCode,
    #[minifix(variant = "18")] InvalidUnacceptableCashDistribAgentAcctNum,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum LegalConfirm {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TickDirection {
    #[minifix(variant = "0")] PlusTick,
    #[minifix(variant = "1")] ZeroPlusTick,
    #[minifix(variant = "2")] MinusTick,
    #[minifix(variant = "3")] ZeroMinusTick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum UserStatus {
    #[minifix(variant = "1")] LoggedIn,
    #[minifix(variant = "2")] NotLoggedIn,
    #[minifix(variant = "3")] UserNotRecognised,
    #[minifix(variant = "4")] PasswordIncorrect,
    #[minifix(variant = "5")] PasswordChanged,
    #[minifix(variant = "6")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum BookingType {
    #[minifix(variant = "0")] RegularBooking,
    #[minifix(variant = "1")] Cfd,
    #[minifix(variant = "2")] TotalReturnSwap,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum QuoteRespType {
    #[minifix(variant = "1")] HitLift,
    #[minifix(variant = "2")] Counter,
    #[minifix(variant = "3")] Expired,
    #[minifix(variant = "4")] Cover,
    #[minifix(variant = "5")] DoneAway,
    #[minifix(variant = "6")] Pass,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SideValueInd {
    #[minifix(variant = "1")] Sidevalue1,
    #[minifix(variant = "2")] Sidevalue2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum YieldType {
    #[minifix(variant = "AFTERTAX")] AfterTaxYield,
    #[minifix(variant = "ANNUAL")] AnnualYield,
    #[minifix(variant = "ATISSUE")] YieldAtIssue,
    #[minifix(variant = "AVGMATURITY")] YieldToAverageMaturity,
    #[minifix(variant = "BOOK")] BookYield,
    #[minifix(variant = "CALL")] YieldToNextCall,
    #[minifix(variant = "CHANGE")] YieldChangeSinceClose,
    #[minifix(variant = "CLOSE")] ClosingYield,
    #[minifix(variant = "COMPOUND")] CompoundYield,
    #[minifix(variant = "CURRENT")] CurrentYield,
    #[minifix(variant = "GROSS")] TrueGrossYield,
    #[minifix(variant = "GOVTEQUIV")] GovernmentEquivalentYield,
    #[minifix(variant = "INFLATION")] YieldWithInflationAssumption,
    #[minifix(variant = "INVERSEFLOATER")] InverseFloaterBondYield,
    #[minifix(variant = "LASTCLOSE")] MostRecentClosingYield,
    #[minifix(variant = "LASTMONTH")] ClosingYieldMostRecentMonth,
    #[minifix(variant = "LASTQUARTER")] ClosingYieldMostRecentQuarter,
    #[minifix(variant = "LASTYEAR")] ClosingYieldMostRecentYear,
    #[minifix(variant = "LONGAVGLIFE")] YieldToLongestAverageLife,
    #[minifix(variant = "MARK")] MarkToMarketYield,
    #[minifix(variant = "MATURITY")] YieldToMaturity,
    #[minifix(variant = "NEXTREFUND")] YieldToNextRefund,
    #[minifix(variant = "OPENAVG")] OpenAverageYield,
    #[minifix(variant = "PUT")] YieldToNextPut,
    #[minifix(variant = "PREVCLOSE")] PreviousCloseYield,
    #[minifix(variant = "PROCEEDS")] ProceedsYield,
    #[minifix(variant = "SEMIANNUAL")] SemiAnnualYield,
    #[minifix(variant = "SHORTAVGLIFE")] YieldToShortestAverageLife,
    #[minifix(variant = "SIMPLE")] SimpleYield,
    #[minifix(variant = "TAXEQUIV")] TaxEquivalentYield,
    #[minifix(variant = "TENDER")] YieldToTenderDate,
    #[minifix(variant = "TRUE")] TrueYield,
    #[minifix(variant = "VALUE1/32")] YieldValueOf132,
    #[minifix(variant = "WORST")] YieldToWorst,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum OrderCapacity {
    #[minifix(variant = "A")] Agency,
    #[minifix(variant = "G")] Proprietary,
    #[minifix(variant = "I")] Individual,
    #[minifix(variant = "P")] Principal,
    #[minifix(variant = "R")] RisklessPrincipal,
    #[minifix(variant = "W")] AgentForOtherMember,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PartySubIdType {
    #[minifix(variant = "1")] Firm,
    #[minifix(variant = "2")] Person,
    #[minifix(variant = "3")] System,
    #[minifix(variant = "4")] Application,
    #[minifix(variant = "5")] FullLegalNameOfFirm,
    #[minifix(variant = "6")] PostalAddress,
    #[minifix(variant = "7")] PhoneNumber,
    #[minifix(variant = "8")] EmailAddress,
    #[minifix(variant = "9")] ContactName,
    #[minifix(variant = "10")] SecuritiesAccountNumber,
    #[minifix(variant = "11")] RegistrationNumber,
    #[minifix(variant = "12")] RegisteredAddress12,
    #[minifix(variant = "13")] RegulatoryStatus,
    #[minifix(variant = "14")] RegistrationName,
    #[minifix(variant = "15")] CashAccountNumber,
    #[minifix(variant = "16")] Bic,
    #[minifix(variant = "17")] CsdParticipantMemberCode,
    #[minifix(variant = "18")] RegisteredAddress18,
    #[minifix(variant = "19")] FundAccountName,
    #[minifix(variant = "20")] TelexNumber,
    #[minifix(variant = "21")] FaxNumber,
    #[minifix(variant = "22")] SecuritiesAccountName,
    #[minifix(variant = "23")] CashAccountName,
    #[minifix(variant = "24")] Department,
    #[minifix(variant = "25")] Location,
    #[minifix(variant = "26")] PositionAccountType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PartyRole {
    #[minifix(variant = "1")] ExecutingFirm,
    #[minifix(variant = "2")] BrokerOfCredit,
    #[minifix(variant = "3")] ClientId,
    #[minifix(variant = "4")] ClearingFirm,
    #[minifix(variant = "5")] InvestorId,
    #[minifix(variant = "6")] IntroducingFirm,
    #[minifix(variant = "7")] EnteringFirm,
    #[minifix(variant = "8")] LocateLendingFirm,
    #[minifix(variant = "9")] FundManagerClientId,
    #[minifix(variant = "10")] SettlementLocation,
    #[minifix(variant = "11")] OrderOriginationTrader,
    #[minifix(variant = "12")] ExecutingTrader,
    #[minifix(variant = "13")] OrderOriginationFirm,
    #[minifix(variant = "14")] GiveupClearingFirm,
    #[minifix(variant = "15")] CorrespondantClearingFirm,
    #[minifix(variant = "16")] ExecutingSystem,
    #[minifix(variant = "17")] ContraFirm,
    #[minifix(variant = "18")] ContraClearingFirm,
    #[minifix(variant = "19")] SponsoringFirm,
    #[minifix(variant = "20")] UnderlyingContraFirm,
    #[minifix(variant = "21")] ClearingOrganization,
    #[minifix(variant = "22")] Exchange,
    #[minifix(variant = "24")] CustomerAccount,
    #[minifix(variant = "25")] CorrespondentClearingOrganization,
    #[minifix(variant = "26")] CorrespondentBroker,
    #[minifix(variant = "27")] BuyerSeller,
    #[minifix(variant = "28")] Custodian,
    #[minifix(variant = "29")] Intermediary,
    #[minifix(variant = "30")] Agent,
    #[minifix(variant = "31")] SubCustodian,
    #[minifix(variant = "32")] Beneficiary,
    #[minifix(variant = "33")] InterestedParty,
    #[minifix(variant = "34")] RegulatoryBody,
    #[minifix(variant = "35")] LiquidityProvider,
    #[minifix(variant = "36")] EnteringTrader,
    #[minifix(variant = "37")] ContraTrader,
    #[minifix(variant = "38")] PositionAccount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum SettlCurrFxRateCalc {
    #[minifix(variant = "M")] Multiply,
    #[minifix(variant = "D")] Divide,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum ListOrderStatus {
    #[minifix(variant = "1")] Inbiddingprocess,
    #[minifix(variant = "2")] Receivedforexecution,
    #[minifix(variant = "3")] Executing,
    #[minifix(variant = "4")] Canceling,
    #[minifix(variant = "5")] Alert,
    #[minifix(variant = "6")] AllDone,
    #[minifix(variant = "7")] Reject,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum TradSesStatusRejReason {
    #[minifix(variant = "1")] UnknownOrInvalidTradingsessionid,
    #[minifix(variant = "99")] Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PriorityIndicator {
    #[minifix(variant = "0")] PriorityUnchanged,
    #[minifix(variant = "1")] LostPriorityAsResultOfOrderChange,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosMaintAction {
    #[minifix(variant = "1")] NewUsedToIncrementTheOverallTransactionQuantity,
    #[minifix(variant = "2")] ReplaceUsedToOverrideTheOverallTransactionQuantityOrSpecificAddMessagesBasedOnTheReferenceId,
    #[minifix(variant = "3")] CancelUsedToRemoveTheOverallTransactionOrSpecificAddMessagesBasedOnReferenceId,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum CancellationRights {
    #[minifix(variant = "Y")] Yes,
    #[minifix(variant = "N")] NoExecutionOnly,
    #[minifix(variant = "M")] NoWaiverAgreement,
    #[minifix(variant = "O")] NoInstitutional,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FieldType)]
pub enum PosAmtType {
    #[minifix(variant = "FMTM")] FinalMarkToMarketAmount,
    #[minifix(variant = "IMTM")] IncrementalMarkToMarketAmount,
    #[minifix(variant = "TVAR")] TradeVariationAmount,
    #[minifix(variant = "SMTM")] StartOfDayMarkToMarketAmount,
    #[minifix(variant = "PREM")] PremiumAmount,
    #[minifix(variant = "CRES")] CashResidualAmount,
    #[minifix(variant = "CASH")] CashAmount,
    #[minifix(variant = "VADJ")] ValueAdjustedAmount,
}

const FIELD_DEFINITIONS: &[(&str, u32, FixDatatype, FieldLocation)] = &[
    ("ThresholdAmount", 834, FixDatatype::PriceOffset, FieldLocation::Body),
    ("NestedPartyRole", 538, FixDatatype::Int, FieldLocation::Body),
    ("UnsolicitedIndicator", 325, FixDatatype::Boolean, FieldLocation::Body),
    ("NoNested2PartyIDs", 756, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SideValue1", 396, FixDatatype::Amt, FieldLocation::Body),
    ("MaturityDate", 541, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("EmailThreadID", 164, FixDatatype::String, FieldLocation::Body),
    ("OrderInputDevice", 821, FixDatatype::String, FieldLocation::Body),
    ("TradeReportRejectReason", 751, FixDatatype::Int, FieldLocation::Body),
    ("CollAsgnTransType", 903, FixDatatype::Int, FieldLocation::Body),
    ("ApplQueueMax", 812, FixDatatype::Int, FieldLocation::Body),
    ("RawDataLength", 95, FixDatatype::Length, FieldLocation::Body),
    ("ContraBroker", 375, FixDatatype::String, FieldLocation::Body),
    ("EncodedListStatusText", 446, FixDatatype::Data, FieldLocation::Body),
    ("UnderlyingCPRegType", 878, FixDatatype::String, FieldLocation::Body),
    ("NoContAmts", 518, FixDatatype::NumInGroup, FieldLocation::Body),
    ("NoOrders", 73, FixDatatype::NumInGroup, FieldLocation::Body),
    ("EncodedListExecInst", 353, FixDatatype::Data, FieldLocation::Body),
    ("SettlCurrBidFxRate", 656, FixDatatype::Float, FieldLocation::Body),
    ("DayOrderQty", 424, FixDatatype::Qty, FieldLocation::Body),
    ("TradeRequestType", 569, FixDatatype::Int, FieldLocation::Body),
    ("SettlDate", 64, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("UnderlyingLastPx", 651, FixDatatype::Price, FieldLocation::Body),
    ("RefSubID", 931, FixDatatype::String, FieldLocation::Body),
    ("PosMaintStatus", 722, FixDatatype::Int, FieldLocation::Body),
    ("GapFillFlag", 123, FixDatatype::Boolean, FieldLocation::Body),
    ("MDMkt", 275, FixDatatype::Exchange, FieldLocation::Body),
    ("NumberOfOrders", 346, FixDatatype::Int, FieldLocation::Body),
    ("ResponseTransportType", 725, FixDatatype::Int, FieldLocation::Body),
    ("LocateReqd", 114, FixDatatype::Boolean, FieldLocation::Body),
    ("NoSettlInst", 778, FixDatatype::NumInGroup, FieldLocation::Body),
    ("Scope", 546, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("QuoteID", 117, FixDatatype::String, FieldLocation::Body),
    ("MDEntryType", 269, FixDatatype::Char, FieldLocation::Body),
    ("ReversalIndicator", 700, FixDatatype::Boolean, FieldLocation::Body),
    ("TradSesStatus", 340, FixDatatype::Int, FieldLocation::Body),
    ("CrossType", 549, FixDatatype::Int, FieldLocation::Body),
    ("HeartBtInt", 108, FixDatatype::Int, FieldLocation::Body),
    ("QuoteQualifier", 695, FixDatatype::Char, FieldLocation::Body),
    ("TargetStrategy", 847, FixDatatype::Int, FieldLocation::Body),
    ("AllowableOneSidednessValue", 766, FixDatatype::Amt, FieldLocation::Body),
    ("OrdRejReason", 103, FixDatatype::Int, FieldLocation::Body),
    ("NoRegistDtls", 473, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SecurityTradingStatus", 326, FixDatatype::Int, FieldLocation::Body),
    ("NoPosAmt", 753, FixDatatype::NumInGroup, FieldLocation::Body),
    ("LastMkt", 30, FixDatatype::Exchange, FieldLocation::Header),
    ("CollAsgnRespType", 905, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingTradingSessionID", 822, FixDatatype::String, FieldLocation::Body),
    ("ExecID", 17, FixDatatype::String, FieldLocation::Header),
    ("CollInquiryQualifier", 896, FixDatatype::Int, FieldLocation::Body),
    ("QuoteStatus", 297, FixDatatype::Int, FieldLocation::Body),
    ("LegSymbol", 600, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingSecurityAltIDSource", 459, FixDatatype::String, FieldLocation::Body),
    ("QuoteEntryRejectReason", 368, FixDatatype::Int, FieldLocation::Body),
    ("BeginString", 8, FixDatatype::String, FieldLocation::Header),
    ("UnderlyingInstrRegistry", 595, FixDatatype::String, FieldLocation::Body),
    ("CPProgram", 875, FixDatatype::Int, FieldLocation::Body),
    ("ConfirmTransType", 666, FixDatatype::Int, FieldLocation::Body),
    ("AdvRefID", 3, FixDatatype::String, FieldLocation::Header),
    ("NoPartyIDs", 453, FixDatatype::NumInGroup, FieldLocation::Body),
    ("EncodedTextLen", 354, FixDatatype::Length, FieldLocation::Body),
    ("LastParPx", 669, FixDatatype::Price, FieldLocation::Body),
    ("Text", 58, FixDatatype::String, FieldLocation::Body),
    ("Nested3PartyID", 949, FixDatatype::String, FieldLocation::Body),
    ("Urgency", 61, FixDatatype::Char, FieldLocation::Body),
    ("AffirmStatus", 940, FixDatatype::Int, FieldLocation::Body),
    ("CouponRate", 223, FixDatatype::Percentage, FieldLocation::Body),
    ("NoDates", 580, FixDatatype::NumInGroup, FieldLocation::Body),
    ("CardStartDate", 503, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("DeskID", 284, FixDatatype::String, FieldLocation::Body),
    ("SendingTime", 52, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("SettlInstRefID", 214, FixDatatype::String, FieldLocation::Body),
    ("PriceImprovement", 639, FixDatatype::PriceOffset, FieldLocation::Body),
    ("SettlInstReqID", 791, FixDatatype::String, FieldLocation::Body),
    ("MktOfferPx", 646, FixDatatype::Price, FieldLocation::Body),
    ("PutOrCall", 201, FixDatatype::Int, FieldLocation::Body),
    ("MoneyLaunderingStatus", 481, FixDatatype::Char, FieldLocation::Body),
    ("MDEntryPx", 270, FixDatatype::Price, FieldLocation::Body),
    ("YieldRedemptionPrice", 697, FixDatatype::Price, FieldLocation::Body),
    ("OrderQty", 38, FixDatatype::Qty, FieldLocation::Header),
    ("OrderQty2", 192, FixDatatype::Qty, FieldLocation::Body),
    ("ParticipationRate", 849, FixDatatype::Percentage, FieldLocation::Body),
    ("PegScope", 840, FixDatatype::Int, FieldLocation::Body),
    ("LegRepurchaseTerm", 251, FixDatatype::Int, FieldLocation::Body),
    ("LegCFICode", 608, FixDatatype::String, FieldLocation::Body),
    ("LiquidityPctHigh", 403, FixDatatype::Percentage, FieldLocation::Body),
    ("UnderlyingSymbolSfx", 312, FixDatatype::String, FieldLocation::Body),
    ("MailingDtls", 474, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingIssueDate", 242, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("NoNestedPartyIDs", 539, FixDatatype::NumInGroup, FieldLocation::Body),
    ("CollAsgnRejectReason", 906, FixDatatype::Int, FieldLocation::Body),
    ("AvgPxIndicator", 819, FixDatatype::Int, FieldLocation::Body),
    ("LegAllocAcctIDSource", 674, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingRepurchaseRate", 245, FixDatatype::Percentage, FieldLocation::Body),
    ("SideValue2", 397, FixDatatype::Amt, FieldLocation::Body),
    ("QuoteCancelType", 298, FixDatatype::Int, FieldLocation::Body),
    ("RoundingDirection", 468, FixDatatype::Char, FieldLocation::Body),
    ("LegBenchmarkCurveName", 677, FixDatatype::String, FieldLocation::Body),
    ("Yield", 236, FixDatatype::Percentage, FieldLocation::Body),
    ("LastFragment", 893, FixDatatype::Boolean, FieldLocation::Body),
    ("UnderlyingStartValue", 884, FixDatatype::Amt, FieldLocation::Body),
    ("ContractMultiplier", 231, FixDatatype::Float, FieldLocation::Body),
    ("NestedPartyID", 524, FixDatatype::String, FieldLocation::Body),
    ("PartyIDSource", 447, FixDatatype::Char, FieldLocation::Body),
    ("CorporateAction", 292, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("NoSecurityAltID", 454, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AccruedInterestRate", 158, FixDatatype::Percentage, FieldLocation::Body),
    ("ContAmtType", 519, FixDatatype::Int, FieldLocation::Body),
    ("Nested3PartyIDSource", 950, FixDatatype::Char, FieldLocation::Body),
    ("BookingUnit", 590, FixDatatype::Char, FieldLocation::Body),
    ("DeliverToLocationID", 145, FixDatatype::String, FieldLocation::Body),
    ("DayCumQty", 425, FixDatatype::Qty, FieldLocation::Body),
    ("PosReqResult", 728, FixDatatype::Int, FieldLocation::Body),
    ("RegistRejReasonText", 496, FixDatatype::String, FieldLocation::Body),
    ("ClearingInstruction", 577, FixDatatype::Int, FieldLocation::Body),
    ("NoMiscFees", 136, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SecondaryAllocID", 793, FixDatatype::String, FieldLocation::Body),
    ("PosMaintResult", 723, FixDatatype::Int, FieldLocation::Body),
    ("SettlPartyRole", 784, FixDatatype::Int, FieldLocation::Body),
    ("QuoteReqID", 131, FixDatatype::String, FieldLocation::Body),
    ("NoSides", 552, FixDatatype::NumInGroup, FieldLocation::Body),
    ("MessageEncoding", 347, FixDatatype::String, FieldLocation::Body),
    ("MailingInst", 482, FixDatatype::String, FieldLocation::Body),
    ("MDImplicitDelete", 547, FixDatatype::Boolean, FieldLocation::Body),
    ("TargetStrategyPerformance", 850, FixDatatype::Float, FieldLocation::Body),
    ("EncodedLegIssuerLen", 618, FixDatatype::Length, FieldLocation::Body),
    ("BidForwardPoints", 189, FixDatatype::PriceOffset, FieldLocation::Body),
    ("TradSesStartTime", 341, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("PosAmt", 708, FixDatatype::Amt, FieldLocation::Body),
    ("OutMainCntryUIndex", 412, FixDatatype::Amt, FieldLocation::Body),
    ("EncodedLegSecurityDescLen", 621, FixDatatype::Length, FieldLocation::Body),
    ("ConfirmType", 773, FixDatatype::Int, FieldLocation::Body),
    ("AllowableOneSidednessCurr", 767, FixDatatype::Currency, FieldLocation::Body),
    ("DeliveryType", 919, FixDatatype::Int, FieldLocation::Body),
    ("TrdType", 828, FixDatatype::Int, FieldLocation::Body),
    ("HaltReasonChar", 327, FixDatatype::Char, FieldLocation::Body),
    ("NoBidDescriptors", 398, FixDatatype::NumInGroup, FieldLocation::Body),
    ("CollReqID", 894, FixDatatype::String, FieldLocation::Body),
    ("NoAffectedOrders", 534, FixDatatype::NumInGroup, FieldLocation::Body),
    ("Signature", 89, FixDatatype::Data, FieldLocation::Trailer),
    ("LastMsgSeqNumProcessed", 369, FixDatatype::SeqNum, FieldLocation::Body),
    ("AllocSettlCurrency", 736, FixDatatype::Currency, FieldLocation::Body),
    ("ContAmtCurr", 521, FixDatatype::Currency, FieldLocation::Body),
    ("AllocQty", 80, FixDatatype::Qty, FieldLocation::Body),
    ("NoSettlPartySubIDs", 801, FixDatatype::NumInGroup, FieldLocation::Body),
    ("ContractSettlMonth", 667, FixDatatype::MonthYear, FieldLocation::Body),
    ("StrikeCurrency", 947, FixDatatype::Currency, FieldLocation::Body),
    ("TradeDate", 75, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("EncodedText", 355, FixDatatype::Data, FieldLocation::Body),
    ("DayAvgPx", 426, FixDatatype::Price, FieldLocation::Body),
    ("ListID", 66, FixDatatype::String, FieldLocation::Body),
    ("AllocReportType", 794, FixDatatype::Int, FieldLocation::Body),
    ("MinTradeVol", 562, FixDatatype::Qty, FieldLocation::Body),
    ("ListExecInst", 69, FixDatatype::String, FieldLocation::Body),
    ("DeleteReason", 285, FixDatatype::Char, FieldLocation::Body),
    ("UnderlyingLastQty", 652, FixDatatype::Qty, FieldLocation::Body),
    ("NoBidComponents", 420, FixDatatype::NumInGroup, FieldLocation::Body),
    ("LegCoveredOrUncovered", 565, FixDatatype::Int, FieldLocation::Body),
    ("NoExecs", 124, FixDatatype::NumInGroup, FieldLocation::Body),
    ("MinBidSize", 647, FixDatatype::Qty, FieldLocation::Body),
    ("OrderCapacityQty", 863, FixDatatype::Qty, FieldLocation::Body),
    ("AdjustmentType", 718, FixDatatype::Int, FieldLocation::Body),
    ("SettlCurrAmt", 119, FixDatatype::Amt, FieldLocation::Body),
    ("MDEntrySize", 271, FixDatatype::Qty, FieldLocation::Body),
    ("TradSesOpenTime", 342, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("ShortQty", 705, FixDatatype::Qty, FieldLocation::Body),
    ("MinQty", 110, FixDatatype::Qty, FieldLocation::Body),
    ("StartCash", 921, FixDatatype::Amt, FieldLocation::Body),
    ("ConfirmRejReason", 774, FixDatatype::Int, FieldLocation::Body),
    ("PossResend", 97, FixDatatype::Boolean, FieldLocation::Body),
    ("LastRptRequested", 912, FixDatatype::Boolean, FieldLocation::Body),
    ("UnderlyingMaturityMonthYear", 313, FixDatatype::MonthYear, FieldLocation::Body),
    ("LegBenchmarkPriceType", 680, FixDatatype::Int, FieldLocation::Body),
    ("InvestorCountryOfResidence", 475, FixDatatype::Country, FieldLocation::Body),
    ("SecurityReqID", 320, FixDatatype::String, FieldLocation::Body),
    ("LegSettlCurrency", 675, FixDatatype::Currency, FieldLocation::Body),
    ("MiscFeeBasis", 891, FixDatatype::Int, FieldLocation::Body),
    ("OpenInterest", 746, FixDatatype::Amt, FieldLocation::Body),
    ("ExecRefID", 19, FixDatatype::String, FieldLocation::Header),
    ("QuoteEntryID", 299, FixDatatype::String, FieldLocation::Body),
    ("RoundingModulus", 469, FixDatatype::Float, FieldLocation::Body),
    ("TradeRequestResult", 749, FixDatatype::Int, FieldLocation::Body),
    ("CheckSum", 10, FixDatatype::String, FieldLocation::Header),
    ("CashOutstanding", 901, FixDatatype::Amt, FieldLocation::Body),
    ("NoPartySubIDs", 802, FixDatatype::NumInGroup, FieldLocation::Body),
    ("CommType", 13, FixDatatype::Char, FieldLocation::Header),
    ("LegInterestAccrualDate", 956, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("DefBidSize", 293, FixDatatype::Qty, FieldLocation::Body),
    ("LegCountryOfIssue", 596, FixDatatype::Country, FieldLocation::Body),
    ("SecurityAltID", 455, FixDatatype::String, FieldLocation::Body),
    ("EncodedUnderlyingSecurityDescLen", 364, FixDatatype::Length, FieldLocation::Body),
    ("AdvSide", 4, FixDatatype::Char, FieldLocation::Header),
    ("PreallocMethod", 591, FixDatatype::Char, FieldLocation::Body),
    ("InstrAttribType", 871, FixDatatype::Int, FieldLocation::Body),
    ("BenchmarkPrice", 662, FixDatatype::Price, FieldLocation::Body),
    ("SettlType", 63, FixDatatype::Char, FieldLocation::Body),
    ("RoutingID", 217, FixDatatype::String, FieldLocation::Body),
    ("FundRenewWaiv", 497, FixDatatype::Char, FieldLocation::Body),
    ("OpenCloseSettlFlag", 286, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("QuoteStatusReqID", 649, FixDatatype::String, FieldLocation::Body),
    ("Side", 54, FixDatatype::Char, FieldLocation::Body),
    ("NotifyBrokerOfCredit", 208, FixDatatype::Boolean, FieldLocation::Body),
    ("StatusText", 929, FixDatatype::String, FieldLocation::Body),
    ("OrigClOrdID", 41, FixDatatype::String, FieldLocation::Header),
    ("TradeReportType", 856, FixDatatype::Int, FieldLocation::Body),
    ("CoveredOrUncovered", 203, FixDatatype::Int, FieldLocation::Body),
    ("LegSide", 624, FixDatatype::Char, FieldLocation::Body),
    ("TransBkdTime", 483, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("MarketDepth", 264, FixDatatype::Int, FieldLocation::Body),
    ("LastQty", 32, FixDatatype::Qty, FieldLocation::Header),
    ("LastSpotRate", 194, FixDatatype::Price, FieldLocation::Body),
    ("EncodedLegIssuer", 619, FixDatatype::Data, FieldLocation::Body),
    ("EndCash", 922, FixDatatype::Amt, FieldLocation::Body),
    ("TrdRegTimestampOrigin", 771, FixDatatype::String, FieldLocation::Body),
    ("LegSwapType", 690, FixDatatype::Int, FieldLocation::Body),
    ("AllocLinkType", 197, FixDatatype::Int, FieldLocation::Body),
    ("CrossPercent", 413, FixDatatype::Percentage, FieldLocation::Body),
    ("QuoteRespID", 693, FixDatatype::String, FieldLocation::Body),
    ("LegRepurchaseRate", 252, FixDatatype::Percentage, FieldLocation::Body),
    ("DiscretionPrice", 845, FixDatatype::Price, FieldLocation::Body),
    ("PegOffsetType", 836, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingRedemptionDate", 247, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("TotalAccruedInterestAmt", 540, FixDatatype::Amt, FieldLocation::Body),
    ("BidDescriptorType", 399, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingSecurityExchange", 308, FixDatatype::Exchange, FieldLocation::Body),
    ("CountryOfIssue", 470, FixDatatype::Country, FieldLocation::Body),
    ("Concession", 238, FixDatatype::Amt, FieldLocation::Body),
    ("AffectedOrderID", 535, FixDatatype::String, FieldLocation::Body),
    ("CollAsgnID", 902, FixDatatype::String, FieldLocation::Body),
    ("ApplQueueAction", 815, FixDatatype::Int, FieldLocation::Body),
    ("LegSecurityAltIDSource", 606, FixDatatype::String, FieldLocation::Body),
    ("IssueDate", 225, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("LiquidityNumSecurities", 441, FixDatatype::Int, FieldLocation::Body),
    ("DefOfferSize", 294, FixDatatype::Qty, FieldLocation::Body),
    ("PartyID", 448, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingStateOrProvinceOfIssue", 593, FixDatatype::String, FieldLocation::Body),
    ("CashOrderQty", 152, FixDatatype::Qty, FieldLocation::Body),
    ("DatedDate", 873, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("NoEvents", 864, FixDatatype::NumInGroup, FieldLocation::Body),
    ("Subject", 147, FixDatatype::String, FieldLocation::Body),
    ("TradeRequestID", 568, FixDatatype::String, FieldLocation::Body),
    ("GTBookingInst", 427, FixDatatype::Int, FieldLocation::Body),
    ("CashDistribAgentName", 498, FixDatatype::String, FieldLocation::Body),
    ("MiscFeeCurr", 138, FixDatatype::Currency, FieldLocation::Body),
    ("MultiLegRptTypeReq", 563, FixDatatype::Int, FieldLocation::Body),
    ("RefCompID", 930, FixDatatype::String, FieldLocation::Body),
    ("OfferYield", 634, FixDatatype::Percentage, FieldLocation::Body),
    ("ResetSeqNumFlag", 141, FixDatatype::Boolean, FieldLocation::Body),
    ("Country", 421, FixDatatype::Country, FieldLocation::Body),
    ("PosReqType", 724, FixDatatype::Int, FieldLocation::Body),
    ("PaymentMethod", 492, FixDatatype::Int, FieldLocation::Body),
    ("LegLastPx", 637, FixDatatype::Price, FieldLocation::Body),
    ("BidPx", 132, FixDatatype::Price, FieldLocation::Body),
    ("NextExpectedMsgSeqNum", 789, FixDatatype::SeqNum, FieldLocation::Body),
    ("ContraryInstructionIndicator", 719, FixDatatype::Boolean, FieldLocation::Body),
    ("AllocSettlInstType", 780, FixDatatype::Int, FieldLocation::Body),
    ("OfferForwardPoints", 191, FixDatatype::PriceOffset, FieldLocation::Body),
    ("CrossID", 548, FixDatatype::String, FieldLocation::Body),
    ("TradSesPreCloseTime", 343, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("ProgRptReqs", 414, FixDatatype::Int, FieldLocation::Body),
    ("DiscretionScope", 846, FixDatatype::Int, FieldLocation::Body),
    ("LegContractMultiplier", 614, FixDatatype::Float, FieldLocation::Body),
    ("StandInstDbType", 169, FixDatatype::Int, FieldLocation::Body),
    ("SecurityRequestType", 321, FixDatatype::Int, FieldLocation::Body),
    ("SideMultiLegReportingType", 752, FixDatatype::Int, FieldLocation::Body),
    ("ListName", 392, FixDatatype::String, FieldLocation::Body),
    ("QuoteType", 537, FixDatatype::Int, FieldLocation::Body),
    ("SettlInstMode", 160, FixDatatype::Char, FieldLocation::Body),
    ("AltMDSourceID", 817, FixDatatype::String, FieldLocation::Body),
    ("ExerciseMethod", 747, FixDatatype::Char, FieldLocation::Body),
    ("MarginExcess", 899, FixDatatype::Amt, FieldLocation::Body),
    ("AllocIntermedReqType", 808, FixDatatype::Int, FieldLocation::Body),
    ("SecureData", 91, FixDatatype::Data, FieldLocation::Trailer),
    ("RefTagID", 371, FixDatatype::Int, FieldLocation::Body),
    ("MultiLegReportingType", 442, FixDatatype::Char, FieldLocation::Body),
    ("NoRpts", 82, FixDatatype::Int, FieldLocation::Body),
    ("InterestAccrualDate", 874, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("RegistTransType", 514, FixDatatype::Char, FieldLocation::Body),
    ("NoDlvyInst", 85, FixDatatype::NumInGroup, FieldLocation::Body),
    ("EncodedUnderlyingSecurityDesc", 365, FixDatatype::Data, FieldLocation::Body),
    ("DeliveryForm", 668, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingContractMultiplier", 436, FixDatatype::Float, FieldLocation::Body),
    ("OwnershipType", 517, FixDatatype::Char, FieldLocation::Body),
    ("BenchmarkPriceType", 663, FixDatatype::Int, FieldLocation::Body),
    ("TimeBracket", 943, FixDatatype::String, FieldLocation::Body),
    ("PriorSettlPrice", 734, FixDatatype::Price, FieldLocation::Body),
    ("AllocTransType", 71, FixDatatype::Char, FieldLocation::Body),
    ("SellerDays", 287, FixDatatype::Int, FieldLocation::Body),
    ("TotNoStrikes", 422, FixDatatype::Int, FieldLocation::Body),
    ("PosMaintRptID", 721, FixDatatype::String, FieldLocation::Body),
    ("ExpireTime", 126, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("OrdStatusReqID", 790, FixDatatype::String, FieldLocation::Body),
    ("NoSecurityTypes", 558, FixDatatype::NumInGroup, FieldLocation::Body),
    ("ReportToExch", 113, FixDatatype::Boolean, FieldLocation::Body),
    ("MDUpdateType", 265, FixDatatype::Int, FieldLocation::Body),
    ("YieldRedemptionDate", 696, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("TradingSessionID", 336, FixDatatype::String, FieldLocation::Body),
    ("NestedPartySubID", 545, FixDatatype::String, FieldLocation::Body),
    ("IOIQualifier", 104, FixDatatype::Char, FieldLocation::Body),
    ("Pool", 691, FixDatatype::String, FieldLocation::Body),
    ("DiscretionLimitType", 843, FixDatatype::Int, FieldLocation::Body),
    ("SecuritySubType", 762, FixDatatype::String, FieldLocation::Body),
    ("StopPx", 99, FixDatatype::Price, FieldLocation::Body),
    ("UnderlyingPutOrCall", 315, FixDatatype::Int, FieldLocation::Body),
    ("SecurityResponseID", 322, FixDatatype::String, FieldLocation::Body),
    ("AllowableOneSidednessPct", 765, FixDatatype::Percentage, FieldLocation::Body),
    ("IOIRefID", 26, FixDatatype::String, FieldLocation::Header),
    ("EndDate", 917, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("SecondaryTradeReportID", 818, FixDatatype::String, FieldLocation::Body),
    ("LastCapacity", 29, FixDatatype::Char, FieldLocation::Header),
    ("CollRptID", 908, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingSecurityID", 309, FixDatatype::String, FieldLocation::Body),
    ("LegBenchmarkCurveCurrency", 676, FixDatatype::Currency, FieldLocation::Body),
    ("StateOrProvinceOfIssue", 471, FixDatatype::String, FieldLocation::Body),
    ("BusinessRejectReason", 380, FixDatatype::Int, FieldLocation::Body),
    ("LegProduct", 607, FixDatatype::Int, FieldLocation::Body),
    ("NoUnderlyingStips", 887, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AllocAccruedInterestAmt", 742, FixDatatype::Amt, FieldLocation::Body),
    ("Currency", 15, FixDatatype::Currency, FieldLocation::Header),
    ("NoQuoteEntries", 295, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AllocPrice", 366, FixDatatype::Price, FieldLocation::Body),
    ("ConfirmStatus", 665, FixDatatype::Int, FieldLocation::Body),
    ("AvgPx", 6, FixDatatype::Price, FieldLocation::Header),
    ("CollInquiryStatus", 945, FixDatatype::Int, FieldLocation::Body),
    ("TargetSubID", 57, FixDatatype::String, FieldLocation::Body),
    ("NoCompIDs", 936, FixDatatype::NumInGroup, FieldLocation::Body),
    ("NoClearingInstructions", 576, FixDatatype::NumInGroup, FieldLocation::Body),
    ("CashDistribAgentCode", 499, FixDatatype::String, FieldLocation::Body),
    ("MDEntryRefID", 280, FixDatatype::String, FieldLocation::Body),
    ("SecurityID", 48, FixDatatype::String, FieldLocation::Header),
    ("MaxShow", 210, FixDatatype::Qty, FieldLocation::Body),
    ("ClearingFeeIndicator", 635, FixDatatype::String, FieldLocation::Body),
    ("DlvyInstType", 787, FixDatatype::Char, FieldLocation::Body),
    ("BidForwardPoints2", 642, FixDatatype::PriceOffset, FieldLocation::Body),
    ("PossDupFlag", 43, FixDatatype::Boolean, FieldLocation::Header),
    ("XmlData", 213, FixDatatype::Data, FieldLocation::Body),
    ("RegistAcctType", 493, FixDatatype::String, FieldLocation::Body),
    ("AggregatedBook", 266, FixDatatype::Boolean, FieldLocation::Body),
    ("MktBidPx", 645, FixDatatype::Price, FieldLocation::Body),
    ("MsgSeqNum", 34, FixDatatype::SeqNum, FieldLocation::Header),
    ("ReportedPx", 861, FixDatatype::Price, FieldLocation::Body),
    ("OrderID", 37, FixDatatype::String, FieldLocation::Header),
    ("PublishTrdIndicator", 852, FixDatatype::Boolean, FieldLocation::Body),
    ("NoIOIQualifiers", 199, FixDatatype::NumInGroup, FieldLocation::Body),
    ("LegSecurityDesc", 620, FixDatatype::String, FieldLocation::Body),
    ("ProgPeriodInterval", 415, FixDatatype::Int, FieldLocation::Body),
    ("BasisFeaturePrice", 260, FixDatatype::Price, FieldLocation::Body),
    ("LegRedemptionDate", 254, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("LegCouponRate", 615, FixDatatype::Percentage, FieldLocation::Body),
    ("AgreementCurrency", 918, FixDatatype::Currency, FieldLocation::Body),
    ("LegPriceType", 686, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingCouponPaymentDate", 241, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("TotNoRelatedSym", 393, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingSecurityType", 310, FixDatatype::String, FieldLocation::Body),
    ("TestMessageIndicator", 464, FixDatatype::Boolean, FieldLocation::Body),
    ("LegAllocQty", 673, FixDatatype::Qty, FieldLocation::Body),
    ("NoStipulations", 232, FixDatatype::NumInGroup, FieldLocation::Body),
    ("UnderlyingStipValue", 889, FixDatatype::String, FieldLocation::Body),
    ("TrdMatchID", 880, FixDatatype::String, FieldLocation::Body),
    ("RepurchaseRate", 227, FixDatatype::Percentage, FieldLocation::Body),
    ("ContAmtValue", 520, FixDatatype::Float, FieldLocation::Body),
    ("StrikeTime", 443, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("MDEntryBuyer", 288, FixDatatype::String, FieldLocation::Body),
    ("AllocNetMoney", 154, FixDatatype::Amt, FieldLocation::Body),
    ("ExecValuationPoint", 515, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("CollInquiryResult", 946, FixDatatype::Int, FieldLocation::Body),
    ("OrigOrdModTime", 586, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("NumDaysInterest", 157, FixDatatype::Int, FieldLocation::Body),
    ("ContraTradeQty", 437, FixDatatype::Qty, FieldLocation::Body),
    ("RegistRefID", 508, FixDatatype::String, FieldLocation::Body),
    ("DayBookingInst", 589, FixDatatype::Char, FieldLocation::Body),
    ("Headline", 148, FixDatatype::String, FieldLocation::Body),
    ("PctAtRisk", 869, FixDatatype::Percentage, FieldLocation::Body),
    ("NoQuoteQualifiers", 735, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AllocCancReplaceReason", 796, FixDatatype::Int, FieldLocation::Body),
    ("TargetLocationID", 143, FixDatatype::String, FieldLocation::Body),
    ("LegPositionEffect", 564, FixDatatype::Char, FieldLocation::Body),
    ("PriceType", 423, FixDatatype::Int, FieldLocation::Body),
    ("Designation", 494, FixDatatype::String, FieldLocation::Body),
    ("BidSize", 134, FixDatatype::Qty, FieldLocation::Body),
    ("SecurityListRequestType", 559, FixDatatype::Int, FieldLocation::Body),
    ("NoCapacities", 862, FixDatatype::NumInGroup, FieldLocation::Body),
    ("HopRefID", 630, FixDatatype::SeqNum, FieldLocation::Body),
    ("ContraTrader", 337, FixDatatype::String, FieldLocation::Body),
    ("LongQty", 704, FixDatatype::Qty, FieldLocation::Body),
    ("ValueOfFutures", 408, FixDatatype::Amt, FieldLocation::Body),
    ("LegIssuer", 617, FixDatatype::String, FieldLocation::Body),
    ("TrdRegTimestamp", 769, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("UnderlyingSecuritySubType", 763, FixDatatype::String, FieldLocation::Body),
    ("AgreementDate", 915, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("TradeLegRefID", 824, FixDatatype::String, FieldLocation::Body),
    ("StandInstDbID", 171, FixDatatype::String, FieldLocation::Body),
    ("SecurityResponseType", 323, FixDatatype::Int, FieldLocation::Body),
    ("BidType", 394, FixDatatype::Int, FieldLocation::Body),
    ("SettlInstID", 162, FixDatatype::String, FieldLocation::Body),
    ("MaturityNetMoney", 890, FixDatatype::Amt, FieldLocation::Body),
    ("MassCancelRequestType", 530, FixDatatype::Char, FieldLocation::Body),
    ("SettlInstSource", 165, FixDatatype::Char, FieldLocation::Body),
    ("GrossTradeAmt", 381, FixDatatype::Amt, FieldLocation::Body),
    ("TotNumTradeReports", 748, FixDatatype::Int, FieldLocation::Body),
    ("DiscretionInst", 388, FixDatatype::Char, FieldLocation::Body),
    ("TotalAffectedOrders", 533, FixDatatype::Int, FieldLocation::Body),
    ("ApplQueueDepth", 813, FixDatatype::Int, FieldLocation::Body),
    ("DeliveryDate", 743, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("NoNestedPartySubIDs", 804, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AllocStatus", 87, FixDatatype::Int, FieldLocation::Body),
    ("QuoteSetValidUntilTime", 367, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("ContraTradeTime", 438, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("NoAllocs", 78, FixDatatype::NumInGroup, FieldLocation::Body),
    ("NoInstrAttrib", 870, FixDatatype::NumInGroup, FieldLocation::Body),
    ("MatchType", 574, FixDatatype::String, FieldLocation::Body),
    ("SymbolSfx", 65, FixDatatype::String, FieldLocation::Body),
    ("MDReqRejReason", 281, FixDatatype::Char, FieldLocation::Body),
    ("MinOfferSize", 648, FixDatatype::Qty, FieldLocation::Body),
    ("IncTaxInd", 416, FixDatatype::Int, FieldLocation::Body),
    ("RoundLot", 561, FixDatatype::Qty, FieldLocation::Body),
    ("SettlCurrency", 120, FixDatatype::Currency, FieldLocation::Body),
    ("OfferForwardPoints2", 643, FixDatatype::PriceOffset, FieldLocation::Body),
    ("ConfirmReqID", 859, FixDatatype::String, FieldLocation::Body),
    ("PosMaintRptRefID", 714, FixDatatype::String, FieldLocation::Body),
    ("OnBehalfOfCompID", 115, FixDatatype::String, FieldLocation::Body),
    ("NoMDEntryTypes", 267, FixDatatype::NumInGroup, FieldLocation::Body),
    ("TradSesMethod", 338, FixDatatype::Int, FieldLocation::Body),
    ("SettlSessSubID", 717, FixDatatype::String, FieldLocation::Body),
    ("Issuer", 106, FixDatatype::String, FieldLocation::Body),
    ("TrdRegTimestampType", 770, FixDatatype::Int, FieldLocation::Body),
    ("UserRequestType", 924, FixDatatype::Int, FieldLocation::Body),
    ("QuotePriceType", 692, FixDatatype::Int, FieldLocation::Body),
    ("HighPx", 332, FixDatatype::Price, FieldLocation::Body),
    ("ExDestination", 100, FixDatatype::Exchange, FieldLocation::Body),
    ("LegQty", 687, FixDatatype::Qty, FieldLocation::Body),
    ("PeggedPrice", 839, FixDatatype::Price, FieldLocation::Body),
    ("Nested2PartyIDSource", 758, FixDatatype::Char, FieldLocation::Body),
    ("LastPx", 31, FixDatatype::Price, FieldLocation::Header),
    ("UnderlyingSymbol", 311, FixDatatype::String, FieldLocation::Body),
    ("NoContraBrokers", 382, FixDatatype::NumInGroup, FieldLocation::Body),
    ("AssignmentUnit", 745, FixDatatype::Qty, FieldLocation::Body),
    ("SecurityIDSource", 22, FixDatatype::String, FieldLocation::Header),
    ("NoTrades", 897, FixDatatype::NumInGroup, FieldLocation::Body),
    ("ApplQueueResolution", 814, FixDatatype::Int, FieldLocation::Body),
    ("BodyLength", 9, FixDatatype::Length, FieldLocation::Header),
    ("NoNested3PartySubIDs", 952, FixDatatype::NumInGroup, FieldLocation::Body),
    ("MDEntrySeller", 289, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingCountryOfIssue", 592, FixDatatype::Country, FieldLocation::Body),
    ("NetChgPrevDay", 451, FixDatatype::PriceOffset, FieldLocation::Body),
    ("EncodedAllocTextLen", 360, FixDatatype::Length, FieldLocation::Body),
    ("LegSettlType", 587, FixDatatype::Char, FieldLocation::Body),
    ("EventPx", 867, FixDatatype::Price, FieldLocation::Body),
    ("QuoteRequestRejectReason", 658, FixDatatype::Int, FieldLocation::Body),
    ("TimeInForce", 59, FixDatatype::Char, FieldLocation::Body),
    ("RegistDtls", 509, FixDatatype::String, FieldLocation::Body),
    ("MDEntryOriginator", 282, FixDatatype::String, FieldLocation::Body),
    ("AllocAcctIDSource", 661, FixDatatype::Int, FieldLocation::Body),
    ("SenderSubID", 50, FixDatatype::String, FieldLocation::Header),
    ("BenchmarkCurveCurrency", 220, FixDatatype::Currency, FieldLocation::Body),
    ("UnderlyingStrikeCurrency", 941, FixDatatype::Currency, FieldLocation::Body),
    ("Quantity", 53, FixDatatype::Qty, FieldLocation::Body),
    ("NetworkResponseID", 932, FixDatatype::String, FieldLocation::Body),
    ("NoRoutingIDs", 215, FixDatatype::NumInGroup, FieldLocation::Body),
    ("WorkingIndicator", 636, FixDatatype::Boolean, FieldLocation::Body),
    ("TaxAdvantageType", 495, FixDatatype::Int, FieldLocation::Body),
    ("QuoteCondition", 276, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("Price", 44, FixDatatype::Price, FieldLocation::Header),
    ("OptAttribute", 206, FixDatatype::Char, FieldLocation::Body),
    ("MidPx", 631, FixDatatype::Price, FieldLocation::Body),
    ("SettlPartyIDSource", 783, FixDatatype::Char, FieldLocation::Body),
    ("NoPositions", 702, FixDatatype::NumInGroup, FieldLocation::Body),
    ("OrdStatus", 39, FixDatatype::Char, FieldLocation::Header),
    ("SettlDate2", 193, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("LiquidityIndType", 409, FixDatatype::Int, FieldLocation::Body),
    ("MDReqID", 262, FixDatatype::String, FieldLocation::Body),
    ("LegStipulationValue", 689, FixDatatype::String, FieldLocation::Body),
    ("LegCouponPaymentDate", 248, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("DiscretionMoveType", 841, FixDatatype::Int, FieldLocation::Body),
    ("TotNumAssignmentReports", 832, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingRepoCollateralSecurityType", 243, FixDatatype::String, FieldLocation::Body),
    ("AffectedSecondaryOrderID", 536, FixDatatype::String, FieldLocation::Body),
    ("NumTickets", 395, FixDatatype::Int, FieldLocation::Body),
    ("TotNoQuoteEntries", 304, FixDatatype::Int, FieldLocation::Body),
    ("BookingRefID", 466, FixDatatype::String, FieldLocation::Body),
    ("StipulationValue", 234, FixDatatype::String, FieldLocation::Body),
    ("MassCancelResponse", 531, FixDatatype::Char, FieldLocation::Body),
    ("MarginRatio", 898, FixDatatype::Percentage, FieldLocation::Body),
    ("PriceDelta", 811, FixDatatype::Float, FieldLocation::Body),
    ("LegSecurityID", 602, FixDatatype::String, FieldLocation::Body),
    ("TotalTakedown", 237, FixDatatype::Amt, FieldLocation::Body),
    ("DiscretionOffsetValue", 389, FixDatatype::Float, FieldLocation::Body),
    ("MDEntryPositionNo", 290, FixDatatype::Int, FieldLocation::Body),
    ("Product", 460, FixDatatype::Int, FieldLocation::Body),
    ("LegSecurityAltID", 605, FixDatatype::String, FieldLocation::Body),
    ("Factor", 228, FixDatatype::Float, FieldLocation::Body),
    ("UnderlyingCurrentValue", 885, FixDatatype::Amt, FieldLocation::Body),
    ("CPRegType", 876, FixDatatype::String, FieldLocation::Body),
    ("AccruedInterestAmt", 159, FixDatatype::Amt, FieldLocation::Body),
    ("OrderPercent", 516, FixDatatype::Percentage, FieldLocation::Body),
    ("NoDistribInsts", 510, FixDatatype::NumInGroup, FieldLocation::Body),
    ("ExecType", 150, FixDatatype::Char, FieldLocation::Body),
    ("OddLot", 575, FixDatatype::Boolean, FieldLocation::Body),
    ("LegStrikeCurrency", 942, FixDatatype::Currency, FieldLocation::Body),
    ("CustOrderCapacity", 582, FixDatatype::Int, FieldLocation::Body),
    ("MiscFeeAmt", 137, FixDatatype::Amt, FieldLocation::Body),
    ("NumBidders", 417, FixDatatype::Int, FieldLocation::Body),
    ("PriorSpreadIndicator", 720, FixDatatype::Boolean, FieldLocation::Body),
    ("CardHolderName", 488, FixDatatype::String, FieldLocation::Body),
    ("MidYield", 633, FixDatatype::Percentage, FieldLocation::Body),
    ("DeliverToCompID", 128, FixDatatype::String, FieldLocation::Body),
    ("SettlPartySubID", 785, FixDatatype::String, FieldLocation::Body),
    ("ClearingBusinessDate", 715, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("IndividualAllocRejCode", 776, FixDatatype::Int, FieldLocation::Body),
    ("CashMargin", 544, FixDatatype::Char, FieldLocation::Body),
    ("TradSesMode", 339, FixDatatype::Int, FieldLocation::Body),
    ("WtAverageLiquidity", 410, FixDatatype::Percentage, FieldLocation::Body),
    ("DiscretionOffsetType", 842, FixDatatype::Int, FieldLocation::Body),
    ("LegMaturityMonthYear", 610, FixDatatype::MonthYear, FieldLocation::Body),
    ("LowPx", 333, FixDatatype::Price, FieldLocation::Body),
    ("LegSecuritySubType", 764, FixDatatype::String, FieldLocation::Body),
    ("LiquidityValue", 404, FixDatatype::Amt, FieldLocation::Body),
    ("LegOptAttribute", 613, FixDatatype::Char, FieldLocation::Body),
    ("SettlDeliveryType", 172, FixDatatype::Int, FieldLocation::Body),
    ("TrdSubType", 829, FixDatatype::Int, FieldLocation::Body),
    ("Nested2PartyRole", 759, FixDatatype::Int, FieldLocation::Body),
    ("TotNumReports", 911, FixDatatype::Int, FieldLocation::Body),
    ("TradeLinkID", 820, FixDatatype::String, FieldLocation::Body),
    ("SecurityType", 167, FixDatatype::String, FieldLocation::Body),
    ("MaxMessageSize", 383, FixDatatype::Length, FieldLocation::Body),
    ("BidID", 390, FixDatatype::String, FieldLocation::Body),
    ("EmailType", 94, FixDatatype::Char, FieldLocation::Body),
    ("UnderlyingEndValue", 886, FixDatatype::Amt, FieldLocation::Body),
    ("SecondaryClOrdID", 526, FixDatatype::String, FieldLocation::Body),
    ("ProcessCode", 81, FixDatatype::Char, FieldLocation::Body),
    ("EncodedAllocText", 361, FixDatatype::Data, FieldLocation::Body),
    ("ConfirmID", 664, FixDatatype::String, FieldLocation::Body),
    ("ExpireDate", 432, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("RegistID", 513, FixDatatype::String, FieldLocation::Body),
    ("RefAllocID", 72, FixDatatype::String, FieldLocation::Body),
    ("SideComplianceID", 659, FixDatatype::String, FieldLocation::Body),
    ("TrdRptStatus", 939, FixDatatype::Int, FieldLocation::Body),
    ("SettlPrice", 730, FixDatatype::Price, FieldLocation::Body),
    ("ListSeqNo", 67, FixDatatype::Int, FieldLocation::Body),
    ("LocationID", 283, FixDatatype::String, FieldLocation::Body),
    ("BidTradeType", 418, FixDatatype::Char, FieldLocation::Body),
    ("UnderlyingSettlPriceType", 733, FixDatatype::Int, FieldLocation::Body),
    ("OrigSendingTime", 122, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("SettlPartySubIDType", 786, FixDatatype::Int, FieldLocation::Body),
    ("Password", 554, FixDatatype::String, FieldLocation::Body),
    ("TradeCondition", 277, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("RFQReqID", 644, FixDatatype::String, FieldLocation::Body),
    ("EncodedIssuerLen", 348, FixDatatype::Length, FieldLocation::Body),
    ("TotNoSecurityTypes", 557, FixDatatype::Int, FieldLocation::Body),
    ("OnBehalfOfSubID", 116, FixDatatype::String, FieldLocation::Body),
    ("PosType", 703, FixDatatype::String, FieldLocation::Body),
    ("SecondaryTrdType", 855, FixDatatype::Int, FieldLocation::Body),
    ("PosReqID", 710, FixDatatype::String, FieldLocation::Body),
    ("MaxFloor", 111, FixDatatype::Qty, FieldLocation::Body),
    ("SubscriptionRequestType", 263, FixDatatype::Char, FieldLocation::Body),
    ("Adjustment", 334, FixDatatype::Int, FieldLocation::Body),
    ("BenchmarkSecurityIDSource", 761, FixDatatype::String, FieldLocation::Body),
    ("CxlRejReason", 102, FixDatatype::Int, FieldLocation::Body),
    ("AgreementDesc", 913, FixDatatype::String, FieldLocation::Body),
    ("TransferReason", 830, FixDatatype::String, FieldLocation::Body),
    ("IOIQltyInd", 25, FixDatatype::Char, FieldLocation::Header),
    ("CollRespID", 904, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingSecurityIDSource", 305, FixDatatype::String, FieldLocation::Body),
    ("LegIndividualAllocID", 672, FixDatatype::String, FieldLocation::Body),
    ("IndividualAllocID", 467, FixDatatype::String, FieldLocation::Body),
    ("ComplianceID", 376, FixDatatype::String, FieldLocation::Body),
    ("EndSeqNo", 16, FixDatatype::SeqNum, FieldLocation::Header),
    ("LegSecurityIDSource", 603, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingEndPrice", 883, FixDatatype::Price, FieldLocation::Body),
    ("InterestAtMaturity", 738, FixDatatype::Amt, FieldLocation::Body),
    ("ClOrdID", 11, FixDatatype::String, FieldLocation::Header),
    ("FinancialStatus", 291, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("CFICode", 461, FixDatatype::String, FieldLocation::Body),
    ("EncodedUnderlyingIssuerLen", 362, FixDatatype::Length, FieldLocation::Body),
    ("AllocInterestAtMaturity", 741, FixDatatype::Amt, FieldLocation::Body),
    ("AdvId", 2, FixDatatype::String, FieldLocation::Header),
    ("AdvTransType", 5, FixDatatype::String, FieldLocation::Header),
    ("NoNested3PartyIDs", 948, FixDatatype::NumInGroup, FieldLocation::Body),
    ("LegSettlDate", 588, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("RegistEmail", 511, FixDatatype::String, FieldLocation::Body),
    ("EncodedSubjectLen", 356, FixDatatype::Length, FieldLocation::Body),
    ("TransactTime", 60, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("BenchmarkCurvePoint", 222, FixDatatype::String, FieldLocation::Body),
    ("ClOrdLinkID", 583, FixDatatype::String, FieldLocation::Body),
    ("OrderAvgPx", 799, FixDatatype::Price, FieldLocation::Body),
    ("LegRefID", 654, FixDatatype::String, FieldLocation::Body),
    ("Symbol", 55, FixDatatype::String, FieldLocation::Body),
    ("AllocHandlInst", 209, FixDatatype::Int, FieldLocation::Body),
    ("CardNumber", 489, FixDatatype::String, FieldLocation::Body),
    ("MDEntryID", 278, FixDatatype::String, FieldLocation::Body),
    ("LastForwardPoints2", 641, FixDatatype::PriceOffset, FieldLocation::Body),
    ("MaturityMonthYear", 200, FixDatatype::MonthYear, FieldLocation::Body),
    ("AllocNoOrdersType", 857, FixDatatype::Int, FieldLocation::Body),
    ("NoLinesOfText", 33, FixDatatype::NumInGroup, FieldLocation::Header),
    ("TargetStrategyParameters", 848, FixDatatype::String, FieldLocation::Body),
    ("LastForwardPoints", 195, FixDatatype::PriceOffset, FieldLocation::Body),
    ("LegSecurityExchange", 616, FixDatatype::Exchange, FieldLocation::Body),
    ("ExchangeForPhysical", 411, FixDatatype::Boolean, FieldLocation::Body),
    ("UnderlyingCreditRating", 256, FixDatatype::String, FieldLocation::Body),
    ("LegRepoCollateralSecurityType", 250, FixDatatype::String, FieldLocation::Body),
    ("LegMaturityDate", 611, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("AgreementID", 914, FixDatatype::String, FieldLocation::Body),
    ("ExpirationCycle", 827, FixDatatype::Int, FieldLocation::Body),
    ("LegIOIQty", 682, FixDatatype::String, FieldLocation::Body),
    ("LegFactor", 253, FixDatatype::Float, FieldLocation::Body),
    ("EFPTrackingError", 405, FixDatatype::Percentage, FieldLocation::Body),
    ("UnderlyingIssuer", 306, FixDatatype::String, FieldLocation::Body),
    ("PaymentRef", 476, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingRepurchaseTerm", 244, FixDatatype::Int, FieldLocation::Body),
    ("PegLimitType", 837, FixDatatype::Int, FieldLocation::Body),
    ("TotNoAllocs", 892, FixDatatype::Int, FieldLocation::Body),
    ("RepoCollateralSecurityType", 239, FixDatatype::String, FieldLocation::Body),
    ("MassCancelRejectReason", 532, FixDatatype::Char, FieldLocation::Body),
    ("ClientBidID", 391, FixDatatype::String, FieldLocation::Body),
    ("QuoteRejectReason", 300, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingProduct", 462, FixDatatype::Int, FieldLocation::Body),
    ("ExDate", 230, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("SecondaryExecID", 527, FixDatatype::String, FieldLocation::Body),
    ("Nested2PartySubIDType", 807, FixDatatype::Int, FieldLocation::Body),
    ("LegLocaleOfIssue", 598, FixDatatype::String, FieldLocation::Body),
    ("AllocAvgPx", 153, FixDatatype::Price, FieldLocation::Body),
    ("ListExecInstType", 433, FixDatatype::Char, FieldLocation::Body),
    ("PaymentDate", 504, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("MassStatusReqType", 585, FixDatatype::Int, FieldLocation::Body),
    ("OnBehalfOfLocationID", 144, FixDatatype::String, FieldLocation::Body),
    ("EventType", 865, FixDatatype::Int, FieldLocation::Body),
    ("SettlPriceType", 731, FixDatatype::Int, FieldLocation::Body),
    ("SettlInstReqRejCode", 792, FixDatatype::Int, FieldLocation::Body),
    ("MiscFeeType", 139, FixDatatype::Char, FieldLocation::Body),
    ("SecurityRequestResult", 560, FixDatatype::Int, FieldLocation::Body),
    ("BasisPxType", 419, FixDatatype::Char, FieldLocation::Body),
    ("CardExpDate", 490, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("IOINaturalFlag", 130, FixDatatype::Boolean, FieldLocation::Body),
    ("NoLegs", 555, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SharedCommission", 858, FixDatatype::Amt, FieldLocation::Body),
    ("AllocType", 626, FixDatatype::Int, FieldLocation::Body),
    ("OfferPx", 133, FixDatatype::Price, FieldLocation::Body),
    ("EncodedIssuer", 349, FixDatatype::Data, FieldLocation::Body),
    ("SettlSessID", 716, FixDatatype::String, FieldLocation::Body),
    ("ExecPriceType", 484, FixDatatype::Char, FieldLocation::Body),
    ("HopSendingTime", 629, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("BidSpotRate", 188, FixDatatype::Price, FieldLocation::Body),
    ("NoSettlPartyIDs", 781, FixDatatype::NumInGroup, FieldLocation::Body),
    ("NoUnderlyings", 711, FixDatatype::NumInGroup, FieldLocation::Body),
    ("UserStatusText", 927, FixDatatype::String, FieldLocation::Body),
    ("ConfirmRefID", 772, FixDatatype::String, FieldLocation::Body),
    ("TradSesReqID", 335, FixDatatype::String, FieldLocation::Body),
    ("FairValue", 406, FixDatatype::Amt, FieldLocation::Body),
    ("PegRoundDirection", 838, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingMaturityDate", 542, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("AllocText", 161, FixDatatype::String, FieldLocation::Body),
    ("SolicitedFlag", 377, FixDatatype::Boolean, FieldLocation::Body),
    ("AssignmentMethod", 744, FixDatatype::Char, FieldLocation::Body),
    ("NoMsgTypes", 384, FixDatatype::NumInGroup, FieldLocation::Body),
    ("OrderRestrictions", 529, FixDatatype::MultipleCharValue, FieldLocation::Body),
    ("AllocRejCode", 88, FixDatatype::Int, FieldLocation::Body),
    ("LegDatedDate", 739, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("LegContractSettlMonth", 955, FixDatatype::MonthYear, FieldLocation::Body),
    ("OrderBookingQty", 800, FixDatatype::Qty, FieldLocation::Body),
    ("RptSeq", 83, FixDatatype::Int, FieldLocation::Body),
    ("EncodedUnderlyingIssuer", 363, FixDatatype::Data, FieldLocation::Body),
    ("CxlRejResponseTo", 434, FixDatatype::Char, FieldLocation::Body),
    ("AvgPxPrecision", 74, FixDatatype::Int, FieldLocation::Body),
    ("EventDate", 866, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("PreviouslyReported", 570, FixDatatype::Boolean, FieldLocation::Body),
    ("PositionEffect", 77, FixDatatype::Char, FieldLocation::Body),
    ("EncodedSubject", 357, FixDatatype::Data, FieldLocation::Body),
    ("AcctIDSource", 660, FixDatatype::Int, FieldLocation::Body),
    ("NoStrikes", 428, FixDatatype::NumInGroup, FieldLocation::Body),
    ("MatchStatus", 573, FixDatatype::Char, FieldLocation::Body),
    ("TotNoOrders", 68, FixDatatype::Int, FieldLocation::Body),
    ("ContraLegRefID", 655, FixDatatype::String, FieldLocation::Body),
    ("NetworkRequestType", 935, FixDatatype::Int, FieldLocation::Body),
    ("ResponseDestination", 726, FixDatatype::String, FieldLocation::Body),
    ("DKReason", 127, FixDatatype::Char, FieldLocation::Body),
    ("MDUpdateAction", 279, FixDatatype::Char, FieldLocation::Body),
    ("EncodedSecurityDescLen", 350, FixDatatype::Length, FieldLocation::Body),
    ("OrigPosReqRefID", 713, FixDatatype::String, FieldLocation::Body),
    ("NetMoney", 118, FixDatatype::Amt, FieldLocation::Body),
    ("SettlPartyID", 782, FixDatatype::String, FieldLocation::Body),
    ("CrossPrioritization", 550, FixDatatype::Int, FieldLocation::Body),
    ("EndAccruedInterestAmt", 920, FixDatatype::Amt, FieldLocation::Body),
    ("LegCreditRating", 257, FixDatatype::String, FieldLocation::Body),
    ("LegStipulationType", 688, FixDatatype::String, FieldLocation::Body),
    ("InViewOfCommon", 328, FixDatatype::Boolean, FieldLocation::Body),
    ("RawData", 96, FixDatatype::Data, FieldLocation::Body),
    ("NoLegStipulations", 683, FixDatatype::NumInGroup, FieldLocation::Body),
    ("PegMoveType", 835, FixDatatype::Int, FieldLocation::Body),
    ("AutoAcceptIndicator", 754, FixDatatype::Boolean, FieldLocation::Body),
    ("IOIQty", 27, FixDatatype::String, FieldLocation::Header),
    ("UnderlyingSecurityDesc", 307, FixDatatype::String, FieldLocation::Body),
    ("DistribPaymentMethod", 477, FixDatatype::Int, FieldLocation::Body),
    ("ExecRestatementReason", 378, FixDatatype::Int, FieldLocation::Body),
    ("Nested2PartyID", 757, FixDatatype::String, FieldLocation::Body),
    ("ExecInst", 18, FixDatatype::MultipleCharValue, FieldLocation::Header),
    ("CollInquiryID", 909, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingPx", 810, FixDatatype::Price, FieldLocation::Body),
    ("HandlInst", 21, FixDatatype::Char, FieldLocation::Header),
    ("TotalNetValue", 900, FixDatatype::Amt, FieldLocation::Body),
    ("QuoteResponseLevel", 301, FixDatatype::Int, FieldLocation::Body),
    ("NoLegSecurityAltID", 604, FixDatatype::NumInGroup, FieldLocation::Body),
    ("UnderlyingCFICode", 463, FixDatatype::String, FieldLocation::Body),
    ("RefMsgType", 372, FixDatatype::String, FieldLocation::Body),
    ("Commission", 12, FixDatatype::Amt, FieldLocation::Header),
    ("LegInstrRegistry", 599, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingQty", 879, FixDatatype::Qty, FieldLocation::Body),
    ("NoLegAllocs", 670, FixDatatype::NumInGroup, FieldLocation::Body),
    ("BeginSeqNo", 7, FixDatatype::SeqNum, FieldLocation::Header),
    ("PaymentRemitterID", 505, FixDatatype::String, FieldLocation::Body),
    ("EncodedHeadlineLen", 358, FixDatatype::Length, FieldLocation::Body),
    ("SettlCurrOfferFxRate", 657, FixDatatype::Float, FieldLocation::Body),
    ("ValidUntilTime", 62, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("RoutingType", 216, FixDatatype::Int, FieldLocation::Body),
    ("NetworkStatusResponseType", 937, FixDatatype::Int, FieldLocation::Body),
    ("SenderCompID", 49, FixDatatype::String, FieldLocation::Header),
    ("StatusValue", 928, FixDatatype::Int, FieldLocation::Body),
    ("PegOffsetValue", 211, FixDatatype::Float, FieldLocation::Body),
    ("BidYield", 632, FixDatatype::Percentage, FieldLocation::Body),
    ("CardIssNum", 491, FixDatatype::String, FieldLocation::Body),
    ("MDEntryDate", 272, FixDatatype::UtcDateOnly, FieldLocation::Body),
    ("OrdType", 40, FixDatatype::Char, FieldLocation::Header),
    ("StrikePrice", 202, FixDatatype::Price, FieldLocation::Body),
    ("NoHops", 627, FixDatatype::NumInGroup, FieldLocation::Body),
    ("LastUpdateTime", 779, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("YieldRedemptionPriceType", 698, FixDatatype::Int, FieldLocation::Body),
    ("MsgType", 35, FixDatatype::String, FieldLocation::Header),
    ("ExecPriceAdjustment", 485, FixDatatype::Float, FieldLocation::Body),
    ("TradedFlatSwitch", 258, FixDatatype::Boolean, FieldLocation::Body),
    ("YieldCalcDate", 701, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("AllocLinkID", 196, FixDatatype::String, FieldLocation::Body),
    ("ShortSaleReason", 853, FixDatatype::Int, FieldLocation::Body),
    ("DiscretionRoundDirection", 844, FixDatatype::Int, FieldLocation::Body),
    ("CreditRating", 255, FixDatatype::String, FieldLocation::Body),
    ("LegStrikePrice", 612, FixDatatype::Price, FieldLocation::Body),
    ("OutsideIndexPct", 407, FixDatatype::Percentage, FieldLocation::Body),
    ("UnderlyingStrikePrice", 316, FixDatatype::Price, FieldLocation::Body),
    ("CashDistribCurr", 478, FixDatatype::Currency, FieldLocation::Body),
    ("UnderlyingFactor", 246, FixDatatype::Float, FieldLocation::Body),
    ("InstrRegistry", 543, FixDatatype::String, FieldLocation::Body),
    ("CollStatus", 910, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingTradingSessionSubID", 823, FixDatatype::String, FieldLocation::Body),
    ("LegBenchmarkCurvePoint", 678, FixDatatype::String, FieldLocation::Body),
    ("StipulationType", 233, FixDatatype::String, FieldLocation::Body),
    ("MsgDirection", 385, FixDatatype::Char, FieldLocation::Body),
    ("QuoteSetID", 302, FixDatatype::String, FieldLocation::Body),
    ("SecurityAltIDSource", 456, FixDatatype::String, FieldLocation::Body),
    ("LegSymbolSfx", 601, FixDatatype::String, FieldLocation::Body),
    ("CouponPaymentDate", 224, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("SecondaryTradeReportRefID", 881, FixDatatype::String, FieldLocation::Body),
    ("InstrAttribValue", 872, FixDatatype::String, FieldLocation::Body),
    ("SettlCurrFxRate", 155, FixDatatype::Float, FieldLocation::Body),
    ("DistribPercentage", 512, FixDatatype::Percentage, FieldLocation::Body),
    ("UnderlyingCouponRate", 435, FixDatatype::Percentage, FieldLocation::Body),
    ("RegistStatus", 506, FixDatatype::Char, FieldLocation::Body),
    ("NoRelatedSym", 146, FixDatatype::NumInGroup, FieldLocation::Body),
    ("TradeReportID", 571, FixDatatype::String, FieldLocation::Body),
    ("NoCollInquiryQualifier", 938, FixDatatype::NumInGroup, FieldLocation::Body),
    ("TradeInputSource", 578, FixDatatype::String, FieldLocation::Body),
    ("URLLink", 149, FixDatatype::String, FieldLocation::Body),
    ("ListStatusType", 429, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingSettlPrice", 732, FixDatatype::Price, FieldLocation::Body),
    ("CashDistribAgentAcctNumber", 500, FixDatatype::String, FieldLocation::Body),
    ("AccountType", 581, FixDatatype::Int, FieldLocation::Body),
    ("PrevClosePx", 140, FixDatatype::Price, FieldLocation::Body),
    ("CopyMsgIndicator", 797, FixDatatype::Boolean, FieldLocation::Body),
    ("TotalNumPosReports", 727, FixDatatype::Int, FieldLocation::Body),
    ("TerminationType", 788, FixDatatype::Int, FieldLocation::Body),
    ("OfferSize", 135, FixDatatype::Qty, FieldLocation::Body),
    ("LegCurrency", 556, FixDatatype::Currency, FieldLocation::Body),
    ("EncodedSecurityDesc", 351, FixDatatype::Data, FieldLocation::Body),
    ("DateOfBirth", 486, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("OfferSpotRate", 190, FixDatatype::Price, FieldLocation::Body),
    ("OrigCrossID", 551, FixDatatype::String, FieldLocation::Body),
    ("QtyType", 854, FixDatatype::Int, FieldLocation::Body),
    ("EncodedLegSecurityDesc", 622, FixDatatype::Data, FieldLocation::Body),
    ("DueToRelated", 329, FixDatatype::Boolean, FieldLocation::Body),
    ("Nested2PartySubID", 760, FixDatatype::String, FieldLocation::Body),
    ("BidDescriptor", 400, FixDatatype::String, FieldLocation::Body),
    ("LegSecurityType", 609, FixDatatype::String, FieldLocation::Body),
    ("EffectiveTime", 168, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("ExchangeRule", 825, FixDatatype::String, FieldLocation::Body),
    ("AllocReportID", 755, FixDatatype::String, FieldLocation::Body),
    ("CollAsgnRefID", 907, FixDatatype::String, FieldLocation::Body),
    ("NoAltMDSource", 816, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SettlInstTransType", 163, FixDatatype::Char, FieldLocation::Body),
    ("BusinessRejectRefID", 379, FixDatatype::String, FieldLocation::Body),
    ("NoTradingSessions", 386, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SecureDataLen", 90, FixDatatype::Length, FieldLocation::Trailer),
    ("UnderlyingDirtyPrice", 882, FixDatatype::Price, FieldLocation::Body),
    ("OwnerType", 522, FixDatatype::Int, FieldLocation::Body),
    ("SignatureLength", 93, FixDatatype::Length, FieldLocation::Trailer),
    ("SessionRejectReason", 373, FixDatatype::Int, FieldLocation::Body),
    ("LegPool", 740, FixDatatype::String, FieldLocation::Body),
    ("ListStatusText", 444, FixDatatype::String, FieldLocation::Body),
    ("NestedPartyIDSource", 525, FixDatatype::Char, FieldLocation::Body),
    ("CxlQty", 84, FixDatatype::Qty, FieldLocation::Body),
    ("NestedPartySubIDType", 805, FixDatatype::Int, FieldLocation::Body),
    ("LegAllocAccount", 671, FixDatatype::String, FieldLocation::Body),
    ("Nested3PartyRole", 951, FixDatatype::Int, FieldLocation::Body),
    ("AllocAccount", 79, FixDatatype::String, FieldLocation::Body),
    ("EncodedHeadline", 359, FixDatatype::Data, FieldLocation::Body),
    ("NetGrossInd", 430, FixDatatype::Int, FieldLocation::Body),
    ("PosReqStatus", 729, FixDatatype::Int, FieldLocation::Body),
    ("AllocID", 70, FixDatatype::String, FieldLocation::Body),
    ("AllocAccountType", 798, FixDatatype::Int, FieldLocation::Body),
    ("LegPrice", 566, FixDatatype::Price, FieldLocation::Body),
    ("ForexReq", 121, FixDatatype::Boolean, FieldLocation::Body),
    ("MDEntryTime", 273, FixDatatype::UtcTimeOnly, FieldLocation::Body),
    ("Price2", 640, FixDatatype::Price, FieldLocation::Body),
    ("TradSesCloseTime", 344, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("Username", 553, FixDatatype::String, FieldLocation::Body),
    ("TestReqID", 112, FixDatatype::String, FieldLocation::Body),
    ("BenchmarkSecurityID", 699, FixDatatype::String, FieldLocation::Body),
    ("LastLiquidityInd", 851, FixDatatype::Int, FieldLocation::Body),
    ("PosQtyStatus", 706, FixDatatype::Int, FieldLocation::Body),
    ("SecurityDesc", 107, FixDatatype::String, FieldLocation::Body),
    ("BasisFeatureDate", 259, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("BuyVolume", 330, FixDatatype::Qty, FieldLocation::Body),
    ("PosTransType", 709, FixDatatype::Int, FieldLocation::Body),
    ("EncryptMethod", 98, FixDatatype::Int, FieldLocation::Body),
    ("NewPassword", 925, FixDatatype::String, FieldLocation::Body),
    ("TradeAllocIndicator", 826, FixDatatype::Int, FieldLocation::Body),
    ("StartDate", 916, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("UnderlyingOptAttribute", 317, FixDatatype::Char, FieldLocation::Body),
    ("LegOfferPx", 684, FixDatatype::Price, FieldLocation::Body),
    ("CommCurrency", 479, FixDatatype::Currency, FieldLocation::Body),
    ("SecurityStatusReqID", 324, FixDatatype::String, FieldLocation::Body),
    ("IOITransType", 28, FixDatatype::Char, FieldLocation::Header),
    ("LegBenchmarkPrice", 679, FixDatatype::Price, FieldLocation::Body),
    ("CollAsgnReason", 895, FixDatatype::Int, FieldLocation::Body),
    ("TradeRequestStatus", 750, FixDatatype::Int, FieldLocation::Body),
    ("IOIID", 23, FixDatatype::String, FieldLocation::Header),
    ("QuoteRequestType", 303, FixDatatype::Int, FieldLocation::Body),
    ("NoUnderlyingSecurityAltID", 457, FixDatatype::NumInGroup, FieldLocation::Body),
    ("BidRequestTransType", 374, FixDatatype::Char, FieldLocation::Body),
    ("AllocSettlCurrAmt", 737, FixDatatype::Amt, FieldLocation::Body),
    ("CumQty", 14, FixDatatype::Qty, FieldLocation::Header),
    ("Nested3PartySubID", 953, FixDatatype::String, FieldLocation::Body),
    ("NoNested2PartySubIDs", 806, FixDatatype::NumInGroup, FieldLocation::Body),
    ("Account", 1, FixDatatype::String, FieldLocation::Header),
    ("CollAction", 944, FixDatatype::Int, FieldLocation::Body),
    ("MassStatusReqID", 584, FixDatatype::String, FieldLocation::Body),
    ("RegistRejReasonCode", 507, FixDatatype::Int, FieldLocation::Body),
    ("EncodedListExecInstLen", 352, FixDatatype::Length, FieldLocation::Body),
    ("TargetCompID", 56, FixDatatype::String, FieldLocation::Body),
    ("Spread", 218, FixDatatype::PriceOffset, FieldLocation::Body),
    ("TradeInputDevice", 579, FixDatatype::String, FieldLocation::Body),
    ("AllocReportRefID", 795, FixDatatype::String, FieldLocation::Body),
    ("LegalConfirm", 650, FixDatatype::Boolean, FieldLocation::Body),
    ("BenchmarkCurveName", 221, FixDatatype::String, FieldLocation::Body),
    ("CashDistribPayRef", 501, FixDatatype::String, FieldLocation::Body),
    ("TickDirection", 274, FixDatatype::Char, FieldLocation::Body),
    ("OrigTime", 42, FixDatatype::UtcTimestamp, FieldLocation::Header),
    ("XmlDataLen", 212, FixDatatype::Length, FieldLocation::Body),
    ("NetworkRequestID", 933, FixDatatype::String, FieldLocation::Body),
    ("RefSeqNum", 45, FixDatatype::SeqNum, FieldLocation::Header),
    ("AvgParPx", 860, FixDatatype::Price, FieldLocation::Body),
    ("SecurityExchange", 207, FixDatatype::Exchange, FieldLocation::Body),
    ("HopCompID", 628, FixDatatype::String, FieldLocation::Body),
    ("TradeReportTransType", 487, FixDatatype::Int, FieldLocation::Body),
    ("NoMDEntries", 268, FixDatatype::NumInGroup, FieldLocation::Body),
    ("NewSeqNo", 36, FixDatatype::SeqNum, FieldLocation::Header),
    ("SecondaryOrderID", 198, FixDatatype::String, FieldLocation::Body),
    ("LegRatioQty", 623, FixDatatype::Float, FieldLocation::Body),
    ("UserStatus", 926, FixDatatype::Int, FieldLocation::Body),
    ("BookingType", 775, FixDatatype::Int, FieldLocation::Body),
    ("QuoteRespType", 694, FixDatatype::Int, FieldLocation::Body),
    ("LegIssueDate", 249, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("SideValueInd", 401, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingCurrency", 318, FixDatatype::Currency, FieldLocation::Body),
    ("LocaleOfIssue", 472, FixDatatype::String, FieldLocation::Body),
    ("LegBidPx", 681, FixDatatype::Price, FieldLocation::Body),
    ("RedemptionDate", 240, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("AsgnRptID", 833, FixDatatype::String, FieldLocation::Body),
    ("UnderlyingStipType", 888, FixDatatype::String, FieldLocation::Body),
    ("YieldType", 235, FixDatatype::String, FieldLocation::Body),
    ("OrderCapacity", 528, FixDatatype::Char, FieldLocation::Body),
    ("TotalVolumeTraded", 387, FixDatatype::Qty, FieldLocation::Body),
    ("NoQuoteSets", 296, FixDatatype::NumInGroup, FieldLocation::Body),
    ("UnderlyingSecurityAltID", 458, FixDatatype::String, FieldLocation::Body),
    ("RepurchaseTerm", 226, FixDatatype::Int, FieldLocation::Body),
    ("PartySubID", 523, FixDatatype::String, FieldLocation::Body),
    ("Nested3PartySubIDType", 954, FixDatatype::Int, FieldLocation::Body),
    ("PartySubIDType", 803, FixDatatype::Int, FieldLocation::Body),
    ("UnderlyingLocaleOfIssue", 594, FixDatatype::String, FieldLocation::Body),
    ("TradeOriginationDate", 229, FixDatatype::LocalMktDate, FieldLocation::Body),
    ("EncodedListStatusTextLen", 445, FixDatatype::Length, FieldLocation::Body),
    ("PartyRole", 452, FixDatatype::Int, FieldLocation::Body),
    ("LegStateOrProvinceOfIssue", 597, FixDatatype::String, FieldLocation::Body),
    ("SettlCurrFxRateCalc", 156, FixDatatype::Char, FieldLocation::Body),
    ("UnderlyingCPProgram", 877, FixDatatype::String, FieldLocation::Body),
    ("EventText", 868, FixDatatype::String, FieldLocation::Body),
    ("LeavesQty", 151, FixDatatype::Qty, FieldLocation::Body),
    ("TradeReportRefID", 572, FixDatatype::String, FieldLocation::Body),
    ("ListOrderStatus", 431, FixDatatype::Int, FieldLocation::Body),
    ("CashDistribAgentAcctName", 502, FixDatatype::String, FieldLocation::Body),
    ("SenderLocationID", 142, FixDatatype::String, FieldLocation::Body),
    ("TradSesStatusRejReason", 567, FixDatatype::Int, FieldLocation::Body),
    ("LastNetworkResponseID", 934, FixDatatype::String, FieldLocation::Body),
    ("PriorityIndicator", 638, FixDatatype::Int, FieldLocation::Body),
    ("DeliverToSubID", 129, FixDatatype::String, FieldLocation::Body),
    ("TradSesEndTime", 345, FixDatatype::UtcTimestamp, FieldLocation::Body),
    ("PosMaintAction", 712, FixDatatype::Int, FieldLocation::Body),
    ("CancellationRights", 480, FixDatatype::Char, FieldLocation::Body),
    ("TradingSessionSubID", 625, FixDatatype::String, FieldLocation::Body),
    ("SettlInstMsgID", 777, FixDatatype::String, FieldLocation::Body),
    ("PosAmtType", 707, FixDatatype::String, FieldLocation::Body),
    ("UserRequestID", 923, FixDatatype::String, FieldLocation::Body),
    ("NoTrdRegTimestamps", 768, FixDatatype::NumInGroup, FieldLocation::Body),
    ("SellVolume", 331, FixDatatype::Qty, FieldLocation::Body),
    ("LiquidityPctLow", 402, FixDatatype::Percentage, FieldLocation::Body),
    ("StandInstDbName", 170, FixDatatype::String, FieldLocation::Body)
];

pub const THRESHOLD_AMOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[0].0, tag: FIELD_DEFINITIONS[0].1, data_type: FIELD_DEFINITIONS[0].2, location: FIELD_DEFINITIONS[0].3 };
pub const NESTED_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[1].0, tag: FIELD_DEFINITIONS[1].1, data_type: FIELD_DEFINITIONS[1].2, location: FIELD_DEFINITIONS[1].3 };
pub const UNSOLICITED_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[2].0, tag: FIELD_DEFINITIONS[2].1, data_type: FIELD_DEFINITIONS[2].2, location: FIELD_DEFINITIONS[2].3 };
pub const NO_NESTED2_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[3].0, tag: FIELD_DEFINITIONS[3].1, data_type: FIELD_DEFINITIONS[3].2, location: FIELD_DEFINITIONS[3].3 };
pub const SIDE_VALUE1: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[4].0, tag: FIELD_DEFINITIONS[4].1, data_type: FIELD_DEFINITIONS[4].2, location: FIELD_DEFINITIONS[4].3 };
pub const MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[5].0, tag: FIELD_DEFINITIONS[5].1, data_type: FIELD_DEFINITIONS[5].2, location: FIELD_DEFINITIONS[5].3 };
pub const EMAIL_THREAD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[6].0, tag: FIELD_DEFINITIONS[6].1, data_type: FIELD_DEFINITIONS[6].2, location: FIELD_DEFINITIONS[6].3 };
pub const ORDER_INPUT_DEVICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[7].0, tag: FIELD_DEFINITIONS[7].1, data_type: FIELD_DEFINITIONS[7].2, location: FIELD_DEFINITIONS[7].3 };
pub const TRADE_REPORT_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[8].0, tag: FIELD_DEFINITIONS[8].1, data_type: FIELD_DEFINITIONS[8].2, location: FIELD_DEFINITIONS[8].3 };
pub const COLL_ASGN_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[9].0, tag: FIELD_DEFINITIONS[9].1, data_type: FIELD_DEFINITIONS[9].2, location: FIELD_DEFINITIONS[9].3 };
pub const APPL_QUEUE_MAX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[10].0, tag: FIELD_DEFINITIONS[10].1, data_type: FIELD_DEFINITIONS[10].2, location: FIELD_DEFINITIONS[10].3 };
pub const RAW_DATA_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[11].0, tag: FIELD_DEFINITIONS[11].1, data_type: FIELD_DEFINITIONS[11].2, location: FIELD_DEFINITIONS[11].3 };
pub const CONTRA_BROKER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[12].0, tag: FIELD_DEFINITIONS[12].1, data_type: FIELD_DEFINITIONS[12].2, location: FIELD_DEFINITIONS[12].3 };
pub const ENCODED_LIST_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[13].0, tag: FIELD_DEFINITIONS[13].1, data_type: FIELD_DEFINITIONS[13].2, location: FIELD_DEFINITIONS[13].3 };
pub const UNDERLYING_CP_REG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[14].0, tag: FIELD_DEFINITIONS[14].1, data_type: FIELD_DEFINITIONS[14].2, location: FIELD_DEFINITIONS[14].3 };
pub const NO_CONT_AMTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[15].0, tag: FIELD_DEFINITIONS[15].1, data_type: FIELD_DEFINITIONS[15].2, location: FIELD_DEFINITIONS[15].3 };
pub const NO_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[16].0, tag: FIELD_DEFINITIONS[16].1, data_type: FIELD_DEFINITIONS[16].2, location: FIELD_DEFINITIONS[16].3 };
pub const ENCODED_LIST_EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[17].0, tag: FIELD_DEFINITIONS[17].1, data_type: FIELD_DEFINITIONS[17].2, location: FIELD_DEFINITIONS[17].3 };
pub const SETTL_CURR_BID_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[18].0, tag: FIELD_DEFINITIONS[18].1, data_type: FIELD_DEFINITIONS[18].2, location: FIELD_DEFINITIONS[18].3 };
pub const DAY_ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[19].0, tag: FIELD_DEFINITIONS[19].1, data_type: FIELD_DEFINITIONS[19].2, location: FIELD_DEFINITIONS[19].3 };
pub const TRADE_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[20].0, tag: FIELD_DEFINITIONS[20].1, data_type: FIELD_DEFINITIONS[20].2, location: FIELD_DEFINITIONS[20].3 };
pub const SETTL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[21].0, tag: FIELD_DEFINITIONS[21].1, data_type: FIELD_DEFINITIONS[21].2, location: FIELD_DEFINITIONS[21].3 };
pub const UNDERLYING_LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[22].0, tag: FIELD_DEFINITIONS[22].1, data_type: FIELD_DEFINITIONS[22].2, location: FIELD_DEFINITIONS[22].3 };
pub const REF_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[23].0, tag: FIELD_DEFINITIONS[23].1, data_type: FIELD_DEFINITIONS[23].2, location: FIELD_DEFINITIONS[23].3 };
pub const POS_MAINT_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[24].0, tag: FIELD_DEFINITIONS[24].1, data_type: FIELD_DEFINITIONS[24].2, location: FIELD_DEFINITIONS[24].3 };
pub const GAP_FILL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[25].0, tag: FIELD_DEFINITIONS[25].1, data_type: FIELD_DEFINITIONS[25].2, location: FIELD_DEFINITIONS[25].3 };
pub const MD_MKT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[26].0, tag: FIELD_DEFINITIONS[26].1, data_type: FIELD_DEFINITIONS[26].2, location: FIELD_DEFINITIONS[26].3 };
pub const NUMBER_OF_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[27].0, tag: FIELD_DEFINITIONS[27].1, data_type: FIELD_DEFINITIONS[27].2, location: FIELD_DEFINITIONS[27].3 };
pub const RESPONSE_TRANSPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[28].0, tag: FIELD_DEFINITIONS[28].1, data_type: FIELD_DEFINITIONS[28].2, location: FIELD_DEFINITIONS[28].3 };
pub const LOCATE_REQD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[29].0, tag: FIELD_DEFINITIONS[29].1, data_type: FIELD_DEFINITIONS[29].2, location: FIELD_DEFINITIONS[29].3 };
pub const NO_SETTL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[30].0, tag: FIELD_DEFINITIONS[30].1, data_type: FIELD_DEFINITIONS[30].2, location: FIELD_DEFINITIONS[30].3 };
pub const SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[31].0, tag: FIELD_DEFINITIONS[31].1, data_type: FIELD_DEFINITIONS[31].2, location: FIELD_DEFINITIONS[31].3 };
pub const QUOTE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[32].0, tag: FIELD_DEFINITIONS[32].1, data_type: FIELD_DEFINITIONS[32].2, location: FIELD_DEFINITIONS[32].3 };
pub const MD_ENTRY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[33].0, tag: FIELD_DEFINITIONS[33].1, data_type: FIELD_DEFINITIONS[33].2, location: FIELD_DEFINITIONS[33].3 };
pub const REVERSAL_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[34].0, tag: FIELD_DEFINITIONS[34].1, data_type: FIELD_DEFINITIONS[34].2, location: FIELD_DEFINITIONS[34].3 };
pub const TRAD_SES_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[35].0, tag: FIELD_DEFINITIONS[35].1, data_type: FIELD_DEFINITIONS[35].2, location: FIELD_DEFINITIONS[35].3 };
pub const CROSS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[36].0, tag: FIELD_DEFINITIONS[36].1, data_type: FIELD_DEFINITIONS[36].2, location: FIELD_DEFINITIONS[36].3 };
pub const HEART_BT_INT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[37].0, tag: FIELD_DEFINITIONS[37].1, data_type: FIELD_DEFINITIONS[37].2, location: FIELD_DEFINITIONS[37].3 };
pub const QUOTE_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[38].0, tag: FIELD_DEFINITIONS[38].1, data_type: FIELD_DEFINITIONS[38].2, location: FIELD_DEFINITIONS[38].3 };
pub const TARGET_STRATEGY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[39].0, tag: FIELD_DEFINITIONS[39].1, data_type: FIELD_DEFINITIONS[39].2, location: FIELD_DEFINITIONS[39].3 };
pub const ALLOWABLE_ONE_SIDEDNESS_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[40].0, tag: FIELD_DEFINITIONS[40].1, data_type: FIELD_DEFINITIONS[40].2, location: FIELD_DEFINITIONS[40].3 };
pub const ORD_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[41].0, tag: FIELD_DEFINITIONS[41].1, data_type: FIELD_DEFINITIONS[41].2, location: FIELD_DEFINITIONS[41].3 };
pub const NO_REGIST_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[42].0, tag: FIELD_DEFINITIONS[42].1, data_type: FIELD_DEFINITIONS[42].2, location: FIELD_DEFINITIONS[42].3 };
pub const SECURITY_TRADING_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[43].0, tag: FIELD_DEFINITIONS[43].1, data_type: FIELD_DEFINITIONS[43].2, location: FIELD_DEFINITIONS[43].3 };
pub const NO_POS_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[44].0, tag: FIELD_DEFINITIONS[44].1, data_type: FIELD_DEFINITIONS[44].2, location: FIELD_DEFINITIONS[44].3 };
pub const LAST_MKT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[45].0, tag: FIELD_DEFINITIONS[45].1, data_type: FIELD_DEFINITIONS[45].2, location: FIELD_DEFINITIONS[45].3 };
pub const COLL_ASGN_RESP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[46].0, tag: FIELD_DEFINITIONS[46].1, data_type: FIELD_DEFINITIONS[46].2, location: FIELD_DEFINITIONS[46].3 };
pub const UNDERLYING_TRADING_SESSION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[47].0, tag: FIELD_DEFINITIONS[47].1, data_type: FIELD_DEFINITIONS[47].2, location: FIELD_DEFINITIONS[47].3 };
pub const EXEC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[48].0, tag: FIELD_DEFINITIONS[48].1, data_type: FIELD_DEFINITIONS[48].2, location: FIELD_DEFINITIONS[48].3 };
pub const COLL_INQUIRY_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[49].0, tag: FIELD_DEFINITIONS[49].1, data_type: FIELD_DEFINITIONS[49].2, location: FIELD_DEFINITIONS[49].3 };
pub const QUOTE_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[50].0, tag: FIELD_DEFINITIONS[50].1, data_type: FIELD_DEFINITIONS[50].2, location: FIELD_DEFINITIONS[50].3 };
pub const LEG_SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[51].0, tag: FIELD_DEFINITIONS[51].1, data_type: FIELD_DEFINITIONS[51].2, location: FIELD_DEFINITIONS[51].3 };
pub const UNDERLYING_SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[52].0, tag: FIELD_DEFINITIONS[52].1, data_type: FIELD_DEFINITIONS[52].2, location: FIELD_DEFINITIONS[52].3 };
pub const QUOTE_ENTRY_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[53].0, tag: FIELD_DEFINITIONS[53].1, data_type: FIELD_DEFINITIONS[53].2, location: FIELD_DEFINITIONS[53].3 };
pub const BEGIN_STRING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[54].0, tag: FIELD_DEFINITIONS[54].1, data_type: FIELD_DEFINITIONS[54].2, location: FIELD_DEFINITIONS[54].3 };
pub const UNDERLYING_INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[55].0, tag: FIELD_DEFINITIONS[55].1, data_type: FIELD_DEFINITIONS[55].2, location: FIELD_DEFINITIONS[55].3 };
pub const CP_PROGRAM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[56].0, tag: FIELD_DEFINITIONS[56].1, data_type: FIELD_DEFINITIONS[56].2, location: FIELD_DEFINITIONS[56].3 };
pub const CONFIRM_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[57].0, tag: FIELD_DEFINITIONS[57].1, data_type: FIELD_DEFINITIONS[57].2, location: FIELD_DEFINITIONS[57].3 };
pub const ADV_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[58].0, tag: FIELD_DEFINITIONS[58].1, data_type: FIELD_DEFINITIONS[58].2, location: FIELD_DEFINITIONS[58].3 };
pub const NO_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[59].0, tag: FIELD_DEFINITIONS[59].1, data_type: FIELD_DEFINITIONS[59].2, location: FIELD_DEFINITIONS[59].3 };
pub const ENCODED_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[60].0, tag: FIELD_DEFINITIONS[60].1, data_type: FIELD_DEFINITIONS[60].2, location: FIELD_DEFINITIONS[60].3 };
pub const LAST_PAR_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[61].0, tag: FIELD_DEFINITIONS[61].1, data_type: FIELD_DEFINITIONS[61].2, location: FIELD_DEFINITIONS[61].3 };
pub const TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[62].0, tag: FIELD_DEFINITIONS[62].1, data_type: FIELD_DEFINITIONS[62].2, location: FIELD_DEFINITIONS[62].3 };
pub const NESTED3_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[63].0, tag: FIELD_DEFINITIONS[63].1, data_type: FIELD_DEFINITIONS[63].2, location: FIELD_DEFINITIONS[63].3 };
pub const URGENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[64].0, tag: FIELD_DEFINITIONS[64].1, data_type: FIELD_DEFINITIONS[64].2, location: FIELD_DEFINITIONS[64].3 };
pub const AFFIRM_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[65].0, tag: FIELD_DEFINITIONS[65].1, data_type: FIELD_DEFINITIONS[65].2, location: FIELD_DEFINITIONS[65].3 };
pub const COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[66].0, tag: FIELD_DEFINITIONS[66].1, data_type: FIELD_DEFINITIONS[66].2, location: FIELD_DEFINITIONS[66].3 };
pub const NO_DATES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[67].0, tag: FIELD_DEFINITIONS[67].1, data_type: FIELD_DEFINITIONS[67].2, location: FIELD_DEFINITIONS[67].3 };
pub const CARD_START_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[68].0, tag: FIELD_DEFINITIONS[68].1, data_type: FIELD_DEFINITIONS[68].2, location: FIELD_DEFINITIONS[68].3 };
pub const DESK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[69].0, tag: FIELD_DEFINITIONS[69].1, data_type: FIELD_DEFINITIONS[69].2, location: FIELD_DEFINITIONS[69].3 };
pub const SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[70].0, tag: FIELD_DEFINITIONS[70].1, data_type: FIELD_DEFINITIONS[70].2, location: FIELD_DEFINITIONS[70].3 };
pub const SETTL_INST_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[71].0, tag: FIELD_DEFINITIONS[71].1, data_type: FIELD_DEFINITIONS[71].2, location: FIELD_DEFINITIONS[71].3 };
pub const PRICE_IMPROVEMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[72].0, tag: FIELD_DEFINITIONS[72].1, data_type: FIELD_DEFINITIONS[72].2, location: FIELD_DEFINITIONS[72].3 };
pub const SETTL_INST_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[73].0, tag: FIELD_DEFINITIONS[73].1, data_type: FIELD_DEFINITIONS[73].2, location: FIELD_DEFINITIONS[73].3 };
pub const MKT_OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[74].0, tag: FIELD_DEFINITIONS[74].1, data_type: FIELD_DEFINITIONS[74].2, location: FIELD_DEFINITIONS[74].3 };
pub const PUT_OR_CALL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[75].0, tag: FIELD_DEFINITIONS[75].1, data_type: FIELD_DEFINITIONS[75].2, location: FIELD_DEFINITIONS[75].3 };
pub const MONEY_LAUNDERING_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[76].0, tag: FIELD_DEFINITIONS[76].1, data_type: FIELD_DEFINITIONS[76].2, location: FIELD_DEFINITIONS[76].3 };
pub const MD_ENTRY_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[77].0, tag: FIELD_DEFINITIONS[77].1, data_type: FIELD_DEFINITIONS[77].2, location: FIELD_DEFINITIONS[77].3 };
pub const YIELD_REDEMPTION_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[78].0, tag: FIELD_DEFINITIONS[78].1, data_type: FIELD_DEFINITIONS[78].2, location: FIELD_DEFINITIONS[78].3 };
pub const ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[79].0, tag: FIELD_DEFINITIONS[79].1, data_type: FIELD_DEFINITIONS[79].2, location: FIELD_DEFINITIONS[79].3 };
pub const ORDER_QTY2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[80].0, tag: FIELD_DEFINITIONS[80].1, data_type: FIELD_DEFINITIONS[80].2, location: FIELD_DEFINITIONS[80].3 };
pub const PARTICIPATION_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[81].0, tag: FIELD_DEFINITIONS[81].1, data_type: FIELD_DEFINITIONS[81].2, location: FIELD_DEFINITIONS[81].3 };
pub const PEG_SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[82].0, tag: FIELD_DEFINITIONS[82].1, data_type: FIELD_DEFINITIONS[82].2, location: FIELD_DEFINITIONS[82].3 };
pub const LEG_REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[83].0, tag: FIELD_DEFINITIONS[83].1, data_type: FIELD_DEFINITIONS[83].2, location: FIELD_DEFINITIONS[83].3 };
pub const LEG_CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[84].0, tag: FIELD_DEFINITIONS[84].1, data_type: FIELD_DEFINITIONS[84].2, location: FIELD_DEFINITIONS[84].3 };
pub const LIQUIDITY_PCT_HIGH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[85].0, tag: FIELD_DEFINITIONS[85].1, data_type: FIELD_DEFINITIONS[85].2, location: FIELD_DEFINITIONS[85].3 };
pub const UNDERLYING_SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[86].0, tag: FIELD_DEFINITIONS[86].1, data_type: FIELD_DEFINITIONS[86].2, location: FIELD_DEFINITIONS[86].3 };
pub const MAILING_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[87].0, tag: FIELD_DEFINITIONS[87].1, data_type: FIELD_DEFINITIONS[87].2, location: FIELD_DEFINITIONS[87].3 };
pub const UNDERLYING_ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[88].0, tag: FIELD_DEFINITIONS[88].1, data_type: FIELD_DEFINITIONS[88].2, location: FIELD_DEFINITIONS[88].3 };
pub const NO_NESTED_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[89].0, tag: FIELD_DEFINITIONS[89].1, data_type: FIELD_DEFINITIONS[89].2, location: FIELD_DEFINITIONS[89].3 };
pub const COLL_ASGN_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[90].0, tag: FIELD_DEFINITIONS[90].1, data_type: FIELD_DEFINITIONS[90].2, location: FIELD_DEFINITIONS[90].3 };
pub const AVG_PX_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[91].0, tag: FIELD_DEFINITIONS[91].1, data_type: FIELD_DEFINITIONS[91].2, location: FIELD_DEFINITIONS[91].3 };
pub const LEG_ALLOC_ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[92].0, tag: FIELD_DEFINITIONS[92].1, data_type: FIELD_DEFINITIONS[92].2, location: FIELD_DEFINITIONS[92].3 };
pub const UNDERLYING_REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[93].0, tag: FIELD_DEFINITIONS[93].1, data_type: FIELD_DEFINITIONS[93].2, location: FIELD_DEFINITIONS[93].3 };
pub const SIDE_VALUE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[94].0, tag: FIELD_DEFINITIONS[94].1, data_type: FIELD_DEFINITIONS[94].2, location: FIELD_DEFINITIONS[94].3 };
pub const QUOTE_CANCEL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[95].0, tag: FIELD_DEFINITIONS[95].1, data_type: FIELD_DEFINITIONS[95].2, location: FIELD_DEFINITIONS[95].3 };
pub const ROUNDING_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[96].0, tag: FIELD_DEFINITIONS[96].1, data_type: FIELD_DEFINITIONS[96].2, location: FIELD_DEFINITIONS[96].3 };
pub const LEG_BENCHMARK_CURVE_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[97].0, tag: FIELD_DEFINITIONS[97].1, data_type: FIELD_DEFINITIONS[97].2, location: FIELD_DEFINITIONS[97].3 };
pub const YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[98].0, tag: FIELD_DEFINITIONS[98].1, data_type: FIELD_DEFINITIONS[98].2, location: FIELD_DEFINITIONS[98].3 };
pub const LAST_FRAGMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[99].0, tag: FIELD_DEFINITIONS[99].1, data_type: FIELD_DEFINITIONS[99].2, location: FIELD_DEFINITIONS[99].3 };
pub const UNDERLYING_START_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[100].0, tag: FIELD_DEFINITIONS[100].1, data_type: FIELD_DEFINITIONS[100].2, location: FIELD_DEFINITIONS[100].3 };
pub const CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[101].0, tag: FIELD_DEFINITIONS[101].1, data_type: FIELD_DEFINITIONS[101].2, location: FIELD_DEFINITIONS[101].3 };
pub const NESTED_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[102].0, tag: FIELD_DEFINITIONS[102].1, data_type: FIELD_DEFINITIONS[102].2, location: FIELD_DEFINITIONS[102].3 };
pub const PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[103].0, tag: FIELD_DEFINITIONS[103].1, data_type: FIELD_DEFINITIONS[103].2, location: FIELD_DEFINITIONS[103].3 };
pub const CORPORATE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[104].0, tag: FIELD_DEFINITIONS[104].1, data_type: FIELD_DEFINITIONS[104].2, location: FIELD_DEFINITIONS[104].3 };
pub const NO_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[105].0, tag: FIELD_DEFINITIONS[105].1, data_type: FIELD_DEFINITIONS[105].2, location: FIELD_DEFINITIONS[105].3 };
pub const ACCRUED_INTEREST_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[106].0, tag: FIELD_DEFINITIONS[106].1, data_type: FIELD_DEFINITIONS[106].2, location: FIELD_DEFINITIONS[106].3 };
pub const CONT_AMT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[107].0, tag: FIELD_DEFINITIONS[107].1, data_type: FIELD_DEFINITIONS[107].2, location: FIELD_DEFINITIONS[107].3 };
pub const NESTED3_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[108].0, tag: FIELD_DEFINITIONS[108].1, data_type: FIELD_DEFINITIONS[108].2, location: FIELD_DEFINITIONS[108].3 };
pub const BOOKING_UNIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[109].0, tag: FIELD_DEFINITIONS[109].1, data_type: FIELD_DEFINITIONS[109].2, location: FIELD_DEFINITIONS[109].3 };
pub const DELIVER_TO_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[110].0, tag: FIELD_DEFINITIONS[110].1, data_type: FIELD_DEFINITIONS[110].2, location: FIELD_DEFINITIONS[110].3 };
pub const DAY_CUM_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[111].0, tag: FIELD_DEFINITIONS[111].1, data_type: FIELD_DEFINITIONS[111].2, location: FIELD_DEFINITIONS[111].3 };
pub const POS_REQ_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[112].0, tag: FIELD_DEFINITIONS[112].1, data_type: FIELD_DEFINITIONS[112].2, location: FIELD_DEFINITIONS[112].3 };
pub const REGIST_REJ_REASON_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[113].0, tag: FIELD_DEFINITIONS[113].1, data_type: FIELD_DEFINITIONS[113].2, location: FIELD_DEFINITIONS[113].3 };
pub const CLEARING_INSTRUCTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[114].0, tag: FIELD_DEFINITIONS[114].1, data_type: FIELD_DEFINITIONS[114].2, location: FIELD_DEFINITIONS[114].3 };
pub const NO_MISC_FEES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[115].0, tag: FIELD_DEFINITIONS[115].1, data_type: FIELD_DEFINITIONS[115].2, location: FIELD_DEFINITIONS[115].3 };
pub const SECONDARY_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[116].0, tag: FIELD_DEFINITIONS[116].1, data_type: FIELD_DEFINITIONS[116].2, location: FIELD_DEFINITIONS[116].3 };
pub const POS_MAINT_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[117].0, tag: FIELD_DEFINITIONS[117].1, data_type: FIELD_DEFINITIONS[117].2, location: FIELD_DEFINITIONS[117].3 };
pub const SETTL_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[118].0, tag: FIELD_DEFINITIONS[118].1, data_type: FIELD_DEFINITIONS[118].2, location: FIELD_DEFINITIONS[118].3 };
pub const QUOTE_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[119].0, tag: FIELD_DEFINITIONS[119].1, data_type: FIELD_DEFINITIONS[119].2, location: FIELD_DEFINITIONS[119].3 };
pub const NO_SIDES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[120].0, tag: FIELD_DEFINITIONS[120].1, data_type: FIELD_DEFINITIONS[120].2, location: FIELD_DEFINITIONS[120].3 };
pub const MESSAGE_ENCODING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[121].0, tag: FIELD_DEFINITIONS[121].1, data_type: FIELD_DEFINITIONS[121].2, location: FIELD_DEFINITIONS[121].3 };
pub const MAILING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[122].0, tag: FIELD_DEFINITIONS[122].1, data_type: FIELD_DEFINITIONS[122].2, location: FIELD_DEFINITIONS[122].3 };
pub const MD_IMPLICIT_DELETE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[123].0, tag: FIELD_DEFINITIONS[123].1, data_type: FIELD_DEFINITIONS[123].2, location: FIELD_DEFINITIONS[123].3 };
pub const TARGET_STRATEGY_PERFORMANCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[124].0, tag: FIELD_DEFINITIONS[124].1, data_type: FIELD_DEFINITIONS[124].2, location: FIELD_DEFINITIONS[124].3 };
pub const ENCODED_LEG_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[125].0, tag: FIELD_DEFINITIONS[125].1, data_type: FIELD_DEFINITIONS[125].2, location: FIELD_DEFINITIONS[125].3 };
pub const BID_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[126].0, tag: FIELD_DEFINITIONS[126].1, data_type: FIELD_DEFINITIONS[126].2, location: FIELD_DEFINITIONS[126].3 };
pub const TRAD_SES_START_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[127].0, tag: FIELD_DEFINITIONS[127].1, data_type: FIELD_DEFINITIONS[127].2, location: FIELD_DEFINITIONS[127].3 };
pub const POS_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[128].0, tag: FIELD_DEFINITIONS[128].1, data_type: FIELD_DEFINITIONS[128].2, location: FIELD_DEFINITIONS[128].3 };
pub const OUT_MAIN_CNTRY_U_INDEX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[129].0, tag: FIELD_DEFINITIONS[129].1, data_type: FIELD_DEFINITIONS[129].2, location: FIELD_DEFINITIONS[129].3 };
pub const ENCODED_LEG_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[130].0, tag: FIELD_DEFINITIONS[130].1, data_type: FIELD_DEFINITIONS[130].2, location: FIELD_DEFINITIONS[130].3 };
pub const CONFIRM_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[131].0, tag: FIELD_DEFINITIONS[131].1, data_type: FIELD_DEFINITIONS[131].2, location: FIELD_DEFINITIONS[131].3 };
pub const ALLOWABLE_ONE_SIDEDNESS_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[132].0, tag: FIELD_DEFINITIONS[132].1, data_type: FIELD_DEFINITIONS[132].2, location: FIELD_DEFINITIONS[132].3 };
pub const DELIVERY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[133].0, tag: FIELD_DEFINITIONS[133].1, data_type: FIELD_DEFINITIONS[133].2, location: FIELD_DEFINITIONS[133].3 };
pub const TRD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[134].0, tag: FIELD_DEFINITIONS[134].1, data_type: FIELD_DEFINITIONS[134].2, location: FIELD_DEFINITIONS[134].3 };
pub const HALT_REASON_CHAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[135].0, tag: FIELD_DEFINITIONS[135].1, data_type: FIELD_DEFINITIONS[135].2, location: FIELD_DEFINITIONS[135].3 };
pub const NO_BID_DESCRIPTORS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[136].0, tag: FIELD_DEFINITIONS[136].1, data_type: FIELD_DEFINITIONS[136].2, location: FIELD_DEFINITIONS[136].3 };
pub const COLL_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[137].0, tag: FIELD_DEFINITIONS[137].1, data_type: FIELD_DEFINITIONS[137].2, location: FIELD_DEFINITIONS[137].3 };
pub const NO_AFFECTED_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[138].0, tag: FIELD_DEFINITIONS[138].1, data_type: FIELD_DEFINITIONS[138].2, location: FIELD_DEFINITIONS[138].3 };
pub const SIGNATURE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[139].0, tag: FIELD_DEFINITIONS[139].1, data_type: FIELD_DEFINITIONS[139].2, location: FIELD_DEFINITIONS[139].3 };
pub const LAST_MSG_SEQ_NUM_PROCESSED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[140].0, tag: FIELD_DEFINITIONS[140].1, data_type: FIELD_DEFINITIONS[140].2, location: FIELD_DEFINITIONS[140].3 };
pub const ALLOC_SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[141].0, tag: FIELD_DEFINITIONS[141].1, data_type: FIELD_DEFINITIONS[141].2, location: FIELD_DEFINITIONS[141].3 };
pub const CONT_AMT_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[142].0, tag: FIELD_DEFINITIONS[142].1, data_type: FIELD_DEFINITIONS[142].2, location: FIELD_DEFINITIONS[142].3 };
pub const ALLOC_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[143].0, tag: FIELD_DEFINITIONS[143].1, data_type: FIELD_DEFINITIONS[143].2, location: FIELD_DEFINITIONS[143].3 };
pub const NO_SETTL_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[144].0, tag: FIELD_DEFINITIONS[144].1, data_type: FIELD_DEFINITIONS[144].2, location: FIELD_DEFINITIONS[144].3 };
pub const CONTRACT_SETTL_MONTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[145].0, tag: FIELD_DEFINITIONS[145].1, data_type: FIELD_DEFINITIONS[145].2, location: FIELD_DEFINITIONS[145].3 };
pub const STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[146].0, tag: FIELD_DEFINITIONS[146].1, data_type: FIELD_DEFINITIONS[146].2, location: FIELD_DEFINITIONS[146].3 };
pub const TRADE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[147].0, tag: FIELD_DEFINITIONS[147].1, data_type: FIELD_DEFINITIONS[147].2, location: FIELD_DEFINITIONS[147].3 };
pub const ENCODED_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[148].0, tag: FIELD_DEFINITIONS[148].1, data_type: FIELD_DEFINITIONS[148].2, location: FIELD_DEFINITIONS[148].3 };
pub const DAY_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[149].0, tag: FIELD_DEFINITIONS[149].1, data_type: FIELD_DEFINITIONS[149].2, location: FIELD_DEFINITIONS[149].3 };
pub const LIST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[150].0, tag: FIELD_DEFINITIONS[150].1, data_type: FIELD_DEFINITIONS[150].2, location: FIELD_DEFINITIONS[150].3 };
pub const ALLOC_REPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[151].0, tag: FIELD_DEFINITIONS[151].1, data_type: FIELD_DEFINITIONS[151].2, location: FIELD_DEFINITIONS[151].3 };
pub const MIN_TRADE_VOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[152].0, tag: FIELD_DEFINITIONS[152].1, data_type: FIELD_DEFINITIONS[152].2, location: FIELD_DEFINITIONS[152].3 };
pub const LIST_EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[153].0, tag: FIELD_DEFINITIONS[153].1, data_type: FIELD_DEFINITIONS[153].2, location: FIELD_DEFINITIONS[153].3 };
pub const DELETE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[154].0, tag: FIELD_DEFINITIONS[154].1, data_type: FIELD_DEFINITIONS[154].2, location: FIELD_DEFINITIONS[154].3 };
pub const UNDERLYING_LAST_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[155].0, tag: FIELD_DEFINITIONS[155].1, data_type: FIELD_DEFINITIONS[155].2, location: FIELD_DEFINITIONS[155].3 };
pub const NO_BID_COMPONENTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[156].0, tag: FIELD_DEFINITIONS[156].1, data_type: FIELD_DEFINITIONS[156].2, location: FIELD_DEFINITIONS[156].3 };
pub const LEG_COVERED_OR_UNCOVERED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[157].0, tag: FIELD_DEFINITIONS[157].1, data_type: FIELD_DEFINITIONS[157].2, location: FIELD_DEFINITIONS[157].3 };
pub const NO_EXECS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[158].0, tag: FIELD_DEFINITIONS[158].1, data_type: FIELD_DEFINITIONS[158].2, location: FIELD_DEFINITIONS[158].3 };
pub const MIN_BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[159].0, tag: FIELD_DEFINITIONS[159].1, data_type: FIELD_DEFINITIONS[159].2, location: FIELD_DEFINITIONS[159].3 };
pub const ORDER_CAPACITY_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[160].0, tag: FIELD_DEFINITIONS[160].1, data_type: FIELD_DEFINITIONS[160].2, location: FIELD_DEFINITIONS[160].3 };
pub const ADJUSTMENT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[161].0, tag: FIELD_DEFINITIONS[161].1, data_type: FIELD_DEFINITIONS[161].2, location: FIELD_DEFINITIONS[161].3 };
pub const SETTL_CURR_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[162].0, tag: FIELD_DEFINITIONS[162].1, data_type: FIELD_DEFINITIONS[162].2, location: FIELD_DEFINITIONS[162].3 };
pub const MD_ENTRY_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[163].0, tag: FIELD_DEFINITIONS[163].1, data_type: FIELD_DEFINITIONS[163].2, location: FIELD_DEFINITIONS[163].3 };
pub const TRAD_SES_OPEN_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[164].0, tag: FIELD_DEFINITIONS[164].1, data_type: FIELD_DEFINITIONS[164].2, location: FIELD_DEFINITIONS[164].3 };
pub const SHORT_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[165].0, tag: FIELD_DEFINITIONS[165].1, data_type: FIELD_DEFINITIONS[165].2, location: FIELD_DEFINITIONS[165].3 };
pub const MIN_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[166].0, tag: FIELD_DEFINITIONS[166].1, data_type: FIELD_DEFINITIONS[166].2, location: FIELD_DEFINITIONS[166].3 };
pub const START_CASH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[167].0, tag: FIELD_DEFINITIONS[167].1, data_type: FIELD_DEFINITIONS[167].2, location: FIELD_DEFINITIONS[167].3 };
pub const CONFIRM_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[168].0, tag: FIELD_DEFINITIONS[168].1, data_type: FIELD_DEFINITIONS[168].2, location: FIELD_DEFINITIONS[168].3 };
pub const POSS_RESEND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[169].0, tag: FIELD_DEFINITIONS[169].1, data_type: FIELD_DEFINITIONS[169].2, location: FIELD_DEFINITIONS[169].3 };
pub const LAST_RPT_REQUESTED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[170].0, tag: FIELD_DEFINITIONS[170].1, data_type: FIELD_DEFINITIONS[170].2, location: FIELD_DEFINITIONS[170].3 };
pub const UNDERLYING_MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[171].0, tag: FIELD_DEFINITIONS[171].1, data_type: FIELD_DEFINITIONS[171].2, location: FIELD_DEFINITIONS[171].3 };
pub const LEG_BENCHMARK_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[172].0, tag: FIELD_DEFINITIONS[172].1, data_type: FIELD_DEFINITIONS[172].2, location: FIELD_DEFINITIONS[172].3 };
pub const INVESTOR_COUNTRY_OF_RESIDENCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[173].0, tag: FIELD_DEFINITIONS[173].1, data_type: FIELD_DEFINITIONS[173].2, location: FIELD_DEFINITIONS[173].3 };
pub const SECURITY_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[174].0, tag: FIELD_DEFINITIONS[174].1, data_type: FIELD_DEFINITIONS[174].2, location: FIELD_DEFINITIONS[174].3 };
pub const LEG_SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[175].0, tag: FIELD_DEFINITIONS[175].1, data_type: FIELD_DEFINITIONS[175].2, location: FIELD_DEFINITIONS[175].3 };
pub const MISC_FEE_BASIS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[176].0, tag: FIELD_DEFINITIONS[176].1, data_type: FIELD_DEFINITIONS[176].2, location: FIELD_DEFINITIONS[176].3 };
pub const OPEN_INTEREST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[177].0, tag: FIELD_DEFINITIONS[177].1, data_type: FIELD_DEFINITIONS[177].2, location: FIELD_DEFINITIONS[177].3 };
pub const EXEC_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[178].0, tag: FIELD_DEFINITIONS[178].1, data_type: FIELD_DEFINITIONS[178].2, location: FIELD_DEFINITIONS[178].3 };
pub const QUOTE_ENTRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[179].0, tag: FIELD_DEFINITIONS[179].1, data_type: FIELD_DEFINITIONS[179].2, location: FIELD_DEFINITIONS[179].3 };
pub const ROUNDING_MODULUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[180].0, tag: FIELD_DEFINITIONS[180].1, data_type: FIELD_DEFINITIONS[180].2, location: FIELD_DEFINITIONS[180].3 };
pub const TRADE_REQUEST_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[181].0, tag: FIELD_DEFINITIONS[181].1, data_type: FIELD_DEFINITIONS[181].2, location: FIELD_DEFINITIONS[181].3 };
pub const CHECK_SUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[182].0, tag: FIELD_DEFINITIONS[182].1, data_type: FIELD_DEFINITIONS[182].2, location: FIELD_DEFINITIONS[182].3 };
pub const CASH_OUTSTANDING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[183].0, tag: FIELD_DEFINITIONS[183].1, data_type: FIELD_DEFINITIONS[183].2, location: FIELD_DEFINITIONS[183].3 };
pub const NO_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[184].0, tag: FIELD_DEFINITIONS[184].1, data_type: FIELD_DEFINITIONS[184].2, location: FIELD_DEFINITIONS[184].3 };
pub const COMM_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[185].0, tag: FIELD_DEFINITIONS[185].1, data_type: FIELD_DEFINITIONS[185].2, location: FIELD_DEFINITIONS[185].3 };
pub const LEG_INTEREST_ACCRUAL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[186].0, tag: FIELD_DEFINITIONS[186].1, data_type: FIELD_DEFINITIONS[186].2, location: FIELD_DEFINITIONS[186].3 };
pub const DEF_BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[187].0, tag: FIELD_DEFINITIONS[187].1, data_type: FIELD_DEFINITIONS[187].2, location: FIELD_DEFINITIONS[187].3 };
pub const LEG_COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[188].0, tag: FIELD_DEFINITIONS[188].1, data_type: FIELD_DEFINITIONS[188].2, location: FIELD_DEFINITIONS[188].3 };
pub const SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[189].0, tag: FIELD_DEFINITIONS[189].1, data_type: FIELD_DEFINITIONS[189].2, location: FIELD_DEFINITIONS[189].3 };
pub const ENCODED_UNDERLYING_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[190].0, tag: FIELD_DEFINITIONS[190].1, data_type: FIELD_DEFINITIONS[190].2, location: FIELD_DEFINITIONS[190].3 };
pub const ADV_SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[191].0, tag: FIELD_DEFINITIONS[191].1, data_type: FIELD_DEFINITIONS[191].2, location: FIELD_DEFINITIONS[191].3 };
pub const PREALLOC_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[192].0, tag: FIELD_DEFINITIONS[192].1, data_type: FIELD_DEFINITIONS[192].2, location: FIELD_DEFINITIONS[192].3 };
pub const INSTR_ATTRIB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[193].0, tag: FIELD_DEFINITIONS[193].1, data_type: FIELD_DEFINITIONS[193].2, location: FIELD_DEFINITIONS[193].3 };
pub const BENCHMARK_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[194].0, tag: FIELD_DEFINITIONS[194].1, data_type: FIELD_DEFINITIONS[194].2, location: FIELD_DEFINITIONS[194].3 };
pub const SETTL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[195].0, tag: FIELD_DEFINITIONS[195].1, data_type: FIELD_DEFINITIONS[195].2, location: FIELD_DEFINITIONS[195].3 };
pub const ROUTING_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[196].0, tag: FIELD_DEFINITIONS[196].1, data_type: FIELD_DEFINITIONS[196].2, location: FIELD_DEFINITIONS[196].3 };
pub const FUND_RENEW_WAIV: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[197].0, tag: FIELD_DEFINITIONS[197].1, data_type: FIELD_DEFINITIONS[197].2, location: FIELD_DEFINITIONS[197].3 };
pub const OPEN_CLOSE_SETTL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[198].0, tag: FIELD_DEFINITIONS[198].1, data_type: FIELD_DEFINITIONS[198].2, location: FIELD_DEFINITIONS[198].3 };
pub const QUOTE_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[199].0, tag: FIELD_DEFINITIONS[199].1, data_type: FIELD_DEFINITIONS[199].2, location: FIELD_DEFINITIONS[199].3 };
pub const SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[200].0, tag: FIELD_DEFINITIONS[200].1, data_type: FIELD_DEFINITIONS[200].2, location: FIELD_DEFINITIONS[200].3 };
pub const NOTIFY_BROKER_OF_CREDIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[201].0, tag: FIELD_DEFINITIONS[201].1, data_type: FIELD_DEFINITIONS[201].2, location: FIELD_DEFINITIONS[201].3 };
pub const STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[202].0, tag: FIELD_DEFINITIONS[202].1, data_type: FIELD_DEFINITIONS[202].2, location: FIELD_DEFINITIONS[202].3 };
pub const ORIG_CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[203].0, tag: FIELD_DEFINITIONS[203].1, data_type: FIELD_DEFINITIONS[203].2, location: FIELD_DEFINITIONS[203].3 };
pub const TRADE_REPORT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[204].0, tag: FIELD_DEFINITIONS[204].1, data_type: FIELD_DEFINITIONS[204].2, location: FIELD_DEFINITIONS[204].3 };
pub const COVERED_OR_UNCOVERED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[205].0, tag: FIELD_DEFINITIONS[205].1, data_type: FIELD_DEFINITIONS[205].2, location: FIELD_DEFINITIONS[205].3 };
pub const LEG_SIDE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[206].0, tag: FIELD_DEFINITIONS[206].1, data_type: FIELD_DEFINITIONS[206].2, location: FIELD_DEFINITIONS[206].3 };
pub const TRANS_BKD_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[207].0, tag: FIELD_DEFINITIONS[207].1, data_type: FIELD_DEFINITIONS[207].2, location: FIELD_DEFINITIONS[207].3 };
pub const MARKET_DEPTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[208].0, tag: FIELD_DEFINITIONS[208].1, data_type: FIELD_DEFINITIONS[208].2, location: FIELD_DEFINITIONS[208].3 };
pub const LAST_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[209].0, tag: FIELD_DEFINITIONS[209].1, data_type: FIELD_DEFINITIONS[209].2, location: FIELD_DEFINITIONS[209].3 };
pub const LAST_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[210].0, tag: FIELD_DEFINITIONS[210].1, data_type: FIELD_DEFINITIONS[210].2, location: FIELD_DEFINITIONS[210].3 };
pub const ENCODED_LEG_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[211].0, tag: FIELD_DEFINITIONS[211].1, data_type: FIELD_DEFINITIONS[211].2, location: FIELD_DEFINITIONS[211].3 };
pub const END_CASH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[212].0, tag: FIELD_DEFINITIONS[212].1, data_type: FIELD_DEFINITIONS[212].2, location: FIELD_DEFINITIONS[212].3 };
pub const TRD_REG_TIMESTAMP_ORIGIN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[213].0, tag: FIELD_DEFINITIONS[213].1, data_type: FIELD_DEFINITIONS[213].2, location: FIELD_DEFINITIONS[213].3 };
pub const LEG_SWAP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[214].0, tag: FIELD_DEFINITIONS[214].1, data_type: FIELD_DEFINITIONS[214].2, location: FIELD_DEFINITIONS[214].3 };
pub const ALLOC_LINK_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[215].0, tag: FIELD_DEFINITIONS[215].1, data_type: FIELD_DEFINITIONS[215].2, location: FIELD_DEFINITIONS[215].3 };
pub const CROSS_PERCENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[216].0, tag: FIELD_DEFINITIONS[216].1, data_type: FIELD_DEFINITIONS[216].2, location: FIELD_DEFINITIONS[216].3 };
pub const QUOTE_RESP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[217].0, tag: FIELD_DEFINITIONS[217].1, data_type: FIELD_DEFINITIONS[217].2, location: FIELD_DEFINITIONS[217].3 };
pub const LEG_REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[218].0, tag: FIELD_DEFINITIONS[218].1, data_type: FIELD_DEFINITIONS[218].2, location: FIELD_DEFINITIONS[218].3 };
pub const DISCRETION_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[219].0, tag: FIELD_DEFINITIONS[219].1, data_type: FIELD_DEFINITIONS[219].2, location: FIELD_DEFINITIONS[219].3 };
pub const PEG_OFFSET_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[220].0, tag: FIELD_DEFINITIONS[220].1, data_type: FIELD_DEFINITIONS[220].2, location: FIELD_DEFINITIONS[220].3 };
pub const UNDERLYING_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[221].0, tag: FIELD_DEFINITIONS[221].1, data_type: FIELD_DEFINITIONS[221].2, location: FIELD_DEFINITIONS[221].3 };
pub const TOTAL_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[222].0, tag: FIELD_DEFINITIONS[222].1, data_type: FIELD_DEFINITIONS[222].2, location: FIELD_DEFINITIONS[222].3 };
pub const BID_DESCRIPTOR_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[223].0, tag: FIELD_DEFINITIONS[223].1, data_type: FIELD_DEFINITIONS[223].2, location: FIELD_DEFINITIONS[223].3 };
pub const UNDERLYING_SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[224].0, tag: FIELD_DEFINITIONS[224].1, data_type: FIELD_DEFINITIONS[224].2, location: FIELD_DEFINITIONS[224].3 };
pub const COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[225].0, tag: FIELD_DEFINITIONS[225].1, data_type: FIELD_DEFINITIONS[225].2, location: FIELD_DEFINITIONS[225].3 };
pub const CONCESSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[226].0, tag: FIELD_DEFINITIONS[226].1, data_type: FIELD_DEFINITIONS[226].2, location: FIELD_DEFINITIONS[226].3 };
pub const AFFECTED_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[227].0, tag: FIELD_DEFINITIONS[227].1, data_type: FIELD_DEFINITIONS[227].2, location: FIELD_DEFINITIONS[227].3 };
pub const COLL_ASGN_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[228].0, tag: FIELD_DEFINITIONS[228].1, data_type: FIELD_DEFINITIONS[228].2, location: FIELD_DEFINITIONS[228].3 };
pub const APPL_QUEUE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[229].0, tag: FIELD_DEFINITIONS[229].1, data_type: FIELD_DEFINITIONS[229].2, location: FIELD_DEFINITIONS[229].3 };
pub const LEG_SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[230].0, tag: FIELD_DEFINITIONS[230].1, data_type: FIELD_DEFINITIONS[230].2, location: FIELD_DEFINITIONS[230].3 };
pub const ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[231].0, tag: FIELD_DEFINITIONS[231].1, data_type: FIELD_DEFINITIONS[231].2, location: FIELD_DEFINITIONS[231].3 };
pub const LIQUIDITY_NUM_SECURITIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[232].0, tag: FIELD_DEFINITIONS[232].1, data_type: FIELD_DEFINITIONS[232].2, location: FIELD_DEFINITIONS[232].3 };
pub const DEF_OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[233].0, tag: FIELD_DEFINITIONS[233].1, data_type: FIELD_DEFINITIONS[233].2, location: FIELD_DEFINITIONS[233].3 };
pub const PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[234].0, tag: FIELD_DEFINITIONS[234].1, data_type: FIELD_DEFINITIONS[234].2, location: FIELD_DEFINITIONS[234].3 };
pub const UNDERLYING_STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[235].0, tag: FIELD_DEFINITIONS[235].1, data_type: FIELD_DEFINITIONS[235].2, location: FIELD_DEFINITIONS[235].3 };
pub const CASH_ORDER_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[236].0, tag: FIELD_DEFINITIONS[236].1, data_type: FIELD_DEFINITIONS[236].2, location: FIELD_DEFINITIONS[236].3 };
pub const DATED_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[237].0, tag: FIELD_DEFINITIONS[237].1, data_type: FIELD_DEFINITIONS[237].2, location: FIELD_DEFINITIONS[237].3 };
pub const NO_EVENTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[238].0, tag: FIELD_DEFINITIONS[238].1, data_type: FIELD_DEFINITIONS[238].2, location: FIELD_DEFINITIONS[238].3 };
pub const SUBJECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[239].0, tag: FIELD_DEFINITIONS[239].1, data_type: FIELD_DEFINITIONS[239].2, location: FIELD_DEFINITIONS[239].3 };
pub const TRADE_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[240].0, tag: FIELD_DEFINITIONS[240].1, data_type: FIELD_DEFINITIONS[240].2, location: FIELD_DEFINITIONS[240].3 };
pub const GT_BOOKING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[241].0, tag: FIELD_DEFINITIONS[241].1, data_type: FIELD_DEFINITIONS[241].2, location: FIELD_DEFINITIONS[241].3 };
pub const CASH_DISTRIB_AGENT_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[242].0, tag: FIELD_DEFINITIONS[242].1, data_type: FIELD_DEFINITIONS[242].2, location: FIELD_DEFINITIONS[242].3 };
pub const MISC_FEE_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[243].0, tag: FIELD_DEFINITIONS[243].1, data_type: FIELD_DEFINITIONS[243].2, location: FIELD_DEFINITIONS[243].3 };
pub const MULTI_LEG_RPT_TYPE_REQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[244].0, tag: FIELD_DEFINITIONS[244].1, data_type: FIELD_DEFINITIONS[244].2, location: FIELD_DEFINITIONS[244].3 };
pub const REF_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[245].0, tag: FIELD_DEFINITIONS[245].1, data_type: FIELD_DEFINITIONS[245].2, location: FIELD_DEFINITIONS[245].3 };
pub const OFFER_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[246].0, tag: FIELD_DEFINITIONS[246].1, data_type: FIELD_DEFINITIONS[246].2, location: FIELD_DEFINITIONS[246].3 };
pub const RESET_SEQ_NUM_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[247].0, tag: FIELD_DEFINITIONS[247].1, data_type: FIELD_DEFINITIONS[247].2, location: FIELD_DEFINITIONS[247].3 };
pub const COUNTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[248].0, tag: FIELD_DEFINITIONS[248].1, data_type: FIELD_DEFINITIONS[248].2, location: FIELD_DEFINITIONS[248].3 };
pub const POS_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[249].0, tag: FIELD_DEFINITIONS[249].1, data_type: FIELD_DEFINITIONS[249].2, location: FIELD_DEFINITIONS[249].3 };
pub const PAYMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[250].0, tag: FIELD_DEFINITIONS[250].1, data_type: FIELD_DEFINITIONS[250].2, location: FIELD_DEFINITIONS[250].3 };
pub const LEG_LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[251].0, tag: FIELD_DEFINITIONS[251].1, data_type: FIELD_DEFINITIONS[251].2, location: FIELD_DEFINITIONS[251].3 };
pub const BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[252].0, tag: FIELD_DEFINITIONS[252].1, data_type: FIELD_DEFINITIONS[252].2, location: FIELD_DEFINITIONS[252].3 };
pub const NEXT_EXPECTED_MSG_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[253].0, tag: FIELD_DEFINITIONS[253].1, data_type: FIELD_DEFINITIONS[253].2, location: FIELD_DEFINITIONS[253].3 };
pub const CONTRARY_INSTRUCTION_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[254].0, tag: FIELD_DEFINITIONS[254].1, data_type: FIELD_DEFINITIONS[254].2, location: FIELD_DEFINITIONS[254].3 };
pub const ALLOC_SETTL_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[255].0, tag: FIELD_DEFINITIONS[255].1, data_type: FIELD_DEFINITIONS[255].2, location: FIELD_DEFINITIONS[255].3 };
pub const OFFER_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[256].0, tag: FIELD_DEFINITIONS[256].1, data_type: FIELD_DEFINITIONS[256].2, location: FIELD_DEFINITIONS[256].3 };
pub const CROSS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[257].0, tag: FIELD_DEFINITIONS[257].1, data_type: FIELD_DEFINITIONS[257].2, location: FIELD_DEFINITIONS[257].3 };
pub const TRAD_SES_PRE_CLOSE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[258].0, tag: FIELD_DEFINITIONS[258].1, data_type: FIELD_DEFINITIONS[258].2, location: FIELD_DEFINITIONS[258].3 };
pub const PROG_RPT_REQS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[259].0, tag: FIELD_DEFINITIONS[259].1, data_type: FIELD_DEFINITIONS[259].2, location: FIELD_DEFINITIONS[259].3 };
pub const DISCRETION_SCOPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[260].0, tag: FIELD_DEFINITIONS[260].1, data_type: FIELD_DEFINITIONS[260].2, location: FIELD_DEFINITIONS[260].3 };
pub const LEG_CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[261].0, tag: FIELD_DEFINITIONS[261].1, data_type: FIELD_DEFINITIONS[261].2, location: FIELD_DEFINITIONS[261].3 };
pub const STAND_INST_DB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[262].0, tag: FIELD_DEFINITIONS[262].1, data_type: FIELD_DEFINITIONS[262].2, location: FIELD_DEFINITIONS[262].3 };
pub const SECURITY_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[263].0, tag: FIELD_DEFINITIONS[263].1, data_type: FIELD_DEFINITIONS[263].2, location: FIELD_DEFINITIONS[263].3 };
pub const SIDE_MULTI_LEG_REPORTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[264].0, tag: FIELD_DEFINITIONS[264].1, data_type: FIELD_DEFINITIONS[264].2, location: FIELD_DEFINITIONS[264].3 };
pub const LIST_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[265].0, tag: FIELD_DEFINITIONS[265].1, data_type: FIELD_DEFINITIONS[265].2, location: FIELD_DEFINITIONS[265].3 };
pub const QUOTE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[266].0, tag: FIELD_DEFINITIONS[266].1, data_type: FIELD_DEFINITIONS[266].2, location: FIELD_DEFINITIONS[266].3 };
pub const SETTL_INST_MODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[267].0, tag: FIELD_DEFINITIONS[267].1, data_type: FIELD_DEFINITIONS[267].2, location: FIELD_DEFINITIONS[267].3 };
pub const ALT_MD_SOURCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[268].0, tag: FIELD_DEFINITIONS[268].1, data_type: FIELD_DEFINITIONS[268].2, location: FIELD_DEFINITIONS[268].3 };
pub const EXERCISE_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[269].0, tag: FIELD_DEFINITIONS[269].1, data_type: FIELD_DEFINITIONS[269].2, location: FIELD_DEFINITIONS[269].3 };
pub const MARGIN_EXCESS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[270].0, tag: FIELD_DEFINITIONS[270].1, data_type: FIELD_DEFINITIONS[270].2, location: FIELD_DEFINITIONS[270].3 };
pub const ALLOC_INTERMED_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[271].0, tag: FIELD_DEFINITIONS[271].1, data_type: FIELD_DEFINITIONS[271].2, location: FIELD_DEFINITIONS[271].3 };
pub const SECURE_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[272].0, tag: FIELD_DEFINITIONS[272].1, data_type: FIELD_DEFINITIONS[272].2, location: FIELD_DEFINITIONS[272].3 };
pub const REF_TAG_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[273].0, tag: FIELD_DEFINITIONS[273].1, data_type: FIELD_DEFINITIONS[273].2, location: FIELD_DEFINITIONS[273].3 };
pub const MULTI_LEG_REPORTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[274].0, tag: FIELD_DEFINITIONS[274].1, data_type: FIELD_DEFINITIONS[274].2, location: FIELD_DEFINITIONS[274].3 };
pub const NO_RPTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[275].0, tag: FIELD_DEFINITIONS[275].1, data_type: FIELD_DEFINITIONS[275].2, location: FIELD_DEFINITIONS[275].3 };
pub const INTEREST_ACCRUAL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[276].0, tag: FIELD_DEFINITIONS[276].1, data_type: FIELD_DEFINITIONS[276].2, location: FIELD_DEFINITIONS[276].3 };
pub const REGIST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[277].0, tag: FIELD_DEFINITIONS[277].1, data_type: FIELD_DEFINITIONS[277].2, location: FIELD_DEFINITIONS[277].3 };
pub const NO_DLVY_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[278].0, tag: FIELD_DEFINITIONS[278].1, data_type: FIELD_DEFINITIONS[278].2, location: FIELD_DEFINITIONS[278].3 };
pub const ENCODED_UNDERLYING_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[279].0, tag: FIELD_DEFINITIONS[279].1, data_type: FIELD_DEFINITIONS[279].2, location: FIELD_DEFINITIONS[279].3 };
pub const DELIVERY_FORM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[280].0, tag: FIELD_DEFINITIONS[280].1, data_type: FIELD_DEFINITIONS[280].2, location: FIELD_DEFINITIONS[280].3 };
pub const UNDERLYING_CONTRACT_MULTIPLIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[281].0, tag: FIELD_DEFINITIONS[281].1, data_type: FIELD_DEFINITIONS[281].2, location: FIELD_DEFINITIONS[281].3 };
pub const OWNERSHIP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[282].0, tag: FIELD_DEFINITIONS[282].1, data_type: FIELD_DEFINITIONS[282].2, location: FIELD_DEFINITIONS[282].3 };
pub const BENCHMARK_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[283].0, tag: FIELD_DEFINITIONS[283].1, data_type: FIELD_DEFINITIONS[283].2, location: FIELD_DEFINITIONS[283].3 };
pub const TIME_BRACKET: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[284].0, tag: FIELD_DEFINITIONS[284].1, data_type: FIELD_DEFINITIONS[284].2, location: FIELD_DEFINITIONS[284].3 };
pub const PRIOR_SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[285].0, tag: FIELD_DEFINITIONS[285].1, data_type: FIELD_DEFINITIONS[285].2, location: FIELD_DEFINITIONS[285].3 };
pub const ALLOC_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[286].0, tag: FIELD_DEFINITIONS[286].1, data_type: FIELD_DEFINITIONS[286].2, location: FIELD_DEFINITIONS[286].3 };
pub const SELLER_DAYS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[287].0, tag: FIELD_DEFINITIONS[287].1, data_type: FIELD_DEFINITIONS[287].2, location: FIELD_DEFINITIONS[287].3 };
pub const TOT_NO_STRIKES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[288].0, tag: FIELD_DEFINITIONS[288].1, data_type: FIELD_DEFINITIONS[288].2, location: FIELD_DEFINITIONS[288].3 };
pub const POS_MAINT_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[289].0, tag: FIELD_DEFINITIONS[289].1, data_type: FIELD_DEFINITIONS[289].2, location: FIELD_DEFINITIONS[289].3 };
pub const EXPIRE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[290].0, tag: FIELD_DEFINITIONS[290].1, data_type: FIELD_DEFINITIONS[290].2, location: FIELD_DEFINITIONS[290].3 };
pub const ORD_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[291].0, tag: FIELD_DEFINITIONS[291].1, data_type: FIELD_DEFINITIONS[291].2, location: FIELD_DEFINITIONS[291].3 };
pub const NO_SECURITY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[292].0, tag: FIELD_DEFINITIONS[292].1, data_type: FIELD_DEFINITIONS[292].2, location: FIELD_DEFINITIONS[292].3 };
pub const REPORT_TO_EXCH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[293].0, tag: FIELD_DEFINITIONS[293].1, data_type: FIELD_DEFINITIONS[293].2, location: FIELD_DEFINITIONS[293].3 };
pub const MD_UPDATE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[294].0, tag: FIELD_DEFINITIONS[294].1, data_type: FIELD_DEFINITIONS[294].2, location: FIELD_DEFINITIONS[294].3 };
pub const YIELD_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[295].0, tag: FIELD_DEFINITIONS[295].1, data_type: FIELD_DEFINITIONS[295].2, location: FIELD_DEFINITIONS[295].3 };
pub const TRADING_SESSION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[296].0, tag: FIELD_DEFINITIONS[296].1, data_type: FIELD_DEFINITIONS[296].2, location: FIELD_DEFINITIONS[296].3 };
pub const NESTED_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[297].0, tag: FIELD_DEFINITIONS[297].1, data_type: FIELD_DEFINITIONS[297].2, location: FIELD_DEFINITIONS[297].3 };
pub const IOI_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[298].0, tag: FIELD_DEFINITIONS[298].1, data_type: FIELD_DEFINITIONS[298].2, location: FIELD_DEFINITIONS[298].3 };
pub const POOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[299].0, tag: FIELD_DEFINITIONS[299].1, data_type: FIELD_DEFINITIONS[299].2, location: FIELD_DEFINITIONS[299].3 };
pub const DISCRETION_LIMIT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[300].0, tag: FIELD_DEFINITIONS[300].1, data_type: FIELD_DEFINITIONS[300].2, location: FIELD_DEFINITIONS[300].3 };
pub const SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[301].0, tag: FIELD_DEFINITIONS[301].1, data_type: FIELD_DEFINITIONS[301].2, location: FIELD_DEFINITIONS[301].3 };
pub const STOP_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[302].0, tag: FIELD_DEFINITIONS[302].1, data_type: FIELD_DEFINITIONS[302].2, location: FIELD_DEFINITIONS[302].3 };
pub const UNDERLYING_PUT_OR_CALL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[303].0, tag: FIELD_DEFINITIONS[303].1, data_type: FIELD_DEFINITIONS[303].2, location: FIELD_DEFINITIONS[303].3 };
pub const SECURITY_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[304].0, tag: FIELD_DEFINITIONS[304].1, data_type: FIELD_DEFINITIONS[304].2, location: FIELD_DEFINITIONS[304].3 };
pub const ALLOWABLE_ONE_SIDEDNESS_PCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[305].0, tag: FIELD_DEFINITIONS[305].1, data_type: FIELD_DEFINITIONS[305].2, location: FIELD_DEFINITIONS[305].3 };
pub const IOI_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[306].0, tag: FIELD_DEFINITIONS[306].1, data_type: FIELD_DEFINITIONS[306].2, location: FIELD_DEFINITIONS[306].3 };
pub const END_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[307].0, tag: FIELD_DEFINITIONS[307].1, data_type: FIELD_DEFINITIONS[307].2, location: FIELD_DEFINITIONS[307].3 };
pub const SECONDARY_TRADE_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[308].0, tag: FIELD_DEFINITIONS[308].1, data_type: FIELD_DEFINITIONS[308].2, location: FIELD_DEFINITIONS[308].3 };
pub const LAST_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[309].0, tag: FIELD_DEFINITIONS[309].1, data_type: FIELD_DEFINITIONS[309].2, location: FIELD_DEFINITIONS[309].3 };
pub const COLL_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[310].0, tag: FIELD_DEFINITIONS[310].1, data_type: FIELD_DEFINITIONS[310].2, location: FIELD_DEFINITIONS[310].3 };
pub const UNDERLYING_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[311].0, tag: FIELD_DEFINITIONS[311].1, data_type: FIELD_DEFINITIONS[311].2, location: FIELD_DEFINITIONS[311].3 };
pub const LEG_BENCHMARK_CURVE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[312].0, tag: FIELD_DEFINITIONS[312].1, data_type: FIELD_DEFINITIONS[312].2, location: FIELD_DEFINITIONS[312].3 };
pub const STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[313].0, tag: FIELD_DEFINITIONS[313].1, data_type: FIELD_DEFINITIONS[313].2, location: FIELD_DEFINITIONS[313].3 };
pub const BUSINESS_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[314].0, tag: FIELD_DEFINITIONS[314].1, data_type: FIELD_DEFINITIONS[314].2, location: FIELD_DEFINITIONS[314].3 };
pub const LEG_PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[315].0, tag: FIELD_DEFINITIONS[315].1, data_type: FIELD_DEFINITIONS[315].2, location: FIELD_DEFINITIONS[315].3 };
pub const NO_UNDERLYING_STIPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[316].0, tag: FIELD_DEFINITIONS[316].1, data_type: FIELD_DEFINITIONS[316].2, location: FIELD_DEFINITIONS[316].3 };
pub const ALLOC_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[317].0, tag: FIELD_DEFINITIONS[317].1, data_type: FIELD_DEFINITIONS[317].2, location: FIELD_DEFINITIONS[317].3 };
pub const CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[318].0, tag: FIELD_DEFINITIONS[318].1, data_type: FIELD_DEFINITIONS[318].2, location: FIELD_DEFINITIONS[318].3 };
pub const NO_QUOTE_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[319].0, tag: FIELD_DEFINITIONS[319].1, data_type: FIELD_DEFINITIONS[319].2, location: FIELD_DEFINITIONS[319].3 };
pub const ALLOC_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[320].0, tag: FIELD_DEFINITIONS[320].1, data_type: FIELD_DEFINITIONS[320].2, location: FIELD_DEFINITIONS[320].3 };
pub const CONFIRM_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[321].0, tag: FIELD_DEFINITIONS[321].1, data_type: FIELD_DEFINITIONS[321].2, location: FIELD_DEFINITIONS[321].3 };
pub const AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[322].0, tag: FIELD_DEFINITIONS[322].1, data_type: FIELD_DEFINITIONS[322].2, location: FIELD_DEFINITIONS[322].3 };
pub const COLL_INQUIRY_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[323].0, tag: FIELD_DEFINITIONS[323].1, data_type: FIELD_DEFINITIONS[323].2, location: FIELD_DEFINITIONS[323].3 };
pub const TARGET_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[324].0, tag: FIELD_DEFINITIONS[324].1, data_type: FIELD_DEFINITIONS[324].2, location: FIELD_DEFINITIONS[324].3 };
pub const NO_COMP_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[325].0, tag: FIELD_DEFINITIONS[325].1, data_type: FIELD_DEFINITIONS[325].2, location: FIELD_DEFINITIONS[325].3 };
pub const NO_CLEARING_INSTRUCTIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[326].0, tag: FIELD_DEFINITIONS[326].1, data_type: FIELD_DEFINITIONS[326].2, location: FIELD_DEFINITIONS[326].3 };
pub const CASH_DISTRIB_AGENT_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[327].0, tag: FIELD_DEFINITIONS[327].1, data_type: FIELD_DEFINITIONS[327].2, location: FIELD_DEFINITIONS[327].3 };
pub const MD_ENTRY_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[328].0, tag: FIELD_DEFINITIONS[328].1, data_type: FIELD_DEFINITIONS[328].2, location: FIELD_DEFINITIONS[328].3 };
pub const SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[329].0, tag: FIELD_DEFINITIONS[329].1, data_type: FIELD_DEFINITIONS[329].2, location: FIELD_DEFINITIONS[329].3 };
pub const MAX_SHOW: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[330].0, tag: FIELD_DEFINITIONS[330].1, data_type: FIELD_DEFINITIONS[330].2, location: FIELD_DEFINITIONS[330].3 };
pub const CLEARING_FEE_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[331].0, tag: FIELD_DEFINITIONS[331].1, data_type: FIELD_DEFINITIONS[331].2, location: FIELD_DEFINITIONS[331].3 };
pub const DLVY_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[332].0, tag: FIELD_DEFINITIONS[332].1, data_type: FIELD_DEFINITIONS[332].2, location: FIELD_DEFINITIONS[332].3 };
pub const BID_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[333].0, tag: FIELD_DEFINITIONS[333].1, data_type: FIELD_DEFINITIONS[333].2, location: FIELD_DEFINITIONS[333].3 };
pub const POSS_DUP_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[334].0, tag: FIELD_DEFINITIONS[334].1, data_type: FIELD_DEFINITIONS[334].2, location: FIELD_DEFINITIONS[334].3 };
pub const XML_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[335].0, tag: FIELD_DEFINITIONS[335].1, data_type: FIELD_DEFINITIONS[335].2, location: FIELD_DEFINITIONS[335].3 };
pub const REGIST_ACCT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[336].0, tag: FIELD_DEFINITIONS[336].1, data_type: FIELD_DEFINITIONS[336].2, location: FIELD_DEFINITIONS[336].3 };
pub const AGGREGATED_BOOK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[337].0, tag: FIELD_DEFINITIONS[337].1, data_type: FIELD_DEFINITIONS[337].2, location: FIELD_DEFINITIONS[337].3 };
pub const MKT_BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[338].0, tag: FIELD_DEFINITIONS[338].1, data_type: FIELD_DEFINITIONS[338].2, location: FIELD_DEFINITIONS[338].3 };
pub const MSG_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[339].0, tag: FIELD_DEFINITIONS[339].1, data_type: FIELD_DEFINITIONS[339].2, location: FIELD_DEFINITIONS[339].3 };
pub const REPORTED_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[340].0, tag: FIELD_DEFINITIONS[340].1, data_type: FIELD_DEFINITIONS[340].2, location: FIELD_DEFINITIONS[340].3 };
pub const ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[341].0, tag: FIELD_DEFINITIONS[341].1, data_type: FIELD_DEFINITIONS[341].2, location: FIELD_DEFINITIONS[341].3 };
pub const PUBLISH_TRD_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[342].0, tag: FIELD_DEFINITIONS[342].1, data_type: FIELD_DEFINITIONS[342].2, location: FIELD_DEFINITIONS[342].3 };
pub const NO_IOI_QUALIFIERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[343].0, tag: FIELD_DEFINITIONS[343].1, data_type: FIELD_DEFINITIONS[343].2, location: FIELD_DEFINITIONS[343].3 };
pub const LEG_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[344].0, tag: FIELD_DEFINITIONS[344].1, data_type: FIELD_DEFINITIONS[344].2, location: FIELD_DEFINITIONS[344].3 };
pub const PROG_PERIOD_INTERVAL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[345].0, tag: FIELD_DEFINITIONS[345].1, data_type: FIELD_DEFINITIONS[345].2, location: FIELD_DEFINITIONS[345].3 };
pub const BASIS_FEATURE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[346].0, tag: FIELD_DEFINITIONS[346].1, data_type: FIELD_DEFINITIONS[346].2, location: FIELD_DEFINITIONS[346].3 };
pub const LEG_REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[347].0, tag: FIELD_DEFINITIONS[347].1, data_type: FIELD_DEFINITIONS[347].2, location: FIELD_DEFINITIONS[347].3 };
pub const LEG_COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[348].0, tag: FIELD_DEFINITIONS[348].1, data_type: FIELD_DEFINITIONS[348].2, location: FIELD_DEFINITIONS[348].3 };
pub const AGREEMENT_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[349].0, tag: FIELD_DEFINITIONS[349].1, data_type: FIELD_DEFINITIONS[349].2, location: FIELD_DEFINITIONS[349].3 };
pub const LEG_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[350].0, tag: FIELD_DEFINITIONS[350].1, data_type: FIELD_DEFINITIONS[350].2, location: FIELD_DEFINITIONS[350].3 };
pub const UNDERLYING_COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[351].0, tag: FIELD_DEFINITIONS[351].1, data_type: FIELD_DEFINITIONS[351].2, location: FIELD_DEFINITIONS[351].3 };
pub const TOT_NO_RELATED_SYM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[352].0, tag: FIELD_DEFINITIONS[352].1, data_type: FIELD_DEFINITIONS[352].2, location: FIELD_DEFINITIONS[352].3 };
pub const UNDERLYING_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[353].0, tag: FIELD_DEFINITIONS[353].1, data_type: FIELD_DEFINITIONS[353].2, location: FIELD_DEFINITIONS[353].3 };
pub const TEST_MESSAGE_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[354].0, tag: FIELD_DEFINITIONS[354].1, data_type: FIELD_DEFINITIONS[354].2, location: FIELD_DEFINITIONS[354].3 };
pub const LEG_ALLOC_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[355].0, tag: FIELD_DEFINITIONS[355].1, data_type: FIELD_DEFINITIONS[355].2, location: FIELD_DEFINITIONS[355].3 };
pub const NO_STIPULATIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[356].0, tag: FIELD_DEFINITIONS[356].1, data_type: FIELD_DEFINITIONS[356].2, location: FIELD_DEFINITIONS[356].3 };
pub const UNDERLYING_STIP_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[357].0, tag: FIELD_DEFINITIONS[357].1, data_type: FIELD_DEFINITIONS[357].2, location: FIELD_DEFINITIONS[357].3 };
pub const TRD_MATCH_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[358].0, tag: FIELD_DEFINITIONS[358].1, data_type: FIELD_DEFINITIONS[358].2, location: FIELD_DEFINITIONS[358].3 };
pub const REPURCHASE_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[359].0, tag: FIELD_DEFINITIONS[359].1, data_type: FIELD_DEFINITIONS[359].2, location: FIELD_DEFINITIONS[359].3 };
pub const CONT_AMT_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[360].0, tag: FIELD_DEFINITIONS[360].1, data_type: FIELD_DEFINITIONS[360].2, location: FIELD_DEFINITIONS[360].3 };
pub const STRIKE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[361].0, tag: FIELD_DEFINITIONS[361].1, data_type: FIELD_DEFINITIONS[361].2, location: FIELD_DEFINITIONS[361].3 };
pub const MD_ENTRY_BUYER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[362].0, tag: FIELD_DEFINITIONS[362].1, data_type: FIELD_DEFINITIONS[362].2, location: FIELD_DEFINITIONS[362].3 };
pub const ALLOC_NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[363].0, tag: FIELD_DEFINITIONS[363].1, data_type: FIELD_DEFINITIONS[363].2, location: FIELD_DEFINITIONS[363].3 };
pub const EXEC_VALUATION_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[364].0, tag: FIELD_DEFINITIONS[364].1, data_type: FIELD_DEFINITIONS[364].2, location: FIELD_DEFINITIONS[364].3 };
pub const COLL_INQUIRY_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[365].0, tag: FIELD_DEFINITIONS[365].1, data_type: FIELD_DEFINITIONS[365].2, location: FIELD_DEFINITIONS[365].3 };
pub const ORIG_ORD_MOD_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[366].0, tag: FIELD_DEFINITIONS[366].1, data_type: FIELD_DEFINITIONS[366].2, location: FIELD_DEFINITIONS[366].3 };
pub const NUM_DAYS_INTEREST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[367].0, tag: FIELD_DEFINITIONS[367].1, data_type: FIELD_DEFINITIONS[367].2, location: FIELD_DEFINITIONS[367].3 };
pub const CONTRA_TRADE_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[368].0, tag: FIELD_DEFINITIONS[368].1, data_type: FIELD_DEFINITIONS[368].2, location: FIELD_DEFINITIONS[368].3 };
pub const REGIST_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[369].0, tag: FIELD_DEFINITIONS[369].1, data_type: FIELD_DEFINITIONS[369].2, location: FIELD_DEFINITIONS[369].3 };
pub const DAY_BOOKING_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[370].0, tag: FIELD_DEFINITIONS[370].1, data_type: FIELD_DEFINITIONS[370].2, location: FIELD_DEFINITIONS[370].3 };
pub const HEADLINE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[371].0, tag: FIELD_DEFINITIONS[371].1, data_type: FIELD_DEFINITIONS[371].2, location: FIELD_DEFINITIONS[371].3 };
pub const PCT_AT_RISK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[372].0, tag: FIELD_DEFINITIONS[372].1, data_type: FIELD_DEFINITIONS[372].2, location: FIELD_DEFINITIONS[372].3 };
pub const NO_QUOTE_QUALIFIERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[373].0, tag: FIELD_DEFINITIONS[373].1, data_type: FIELD_DEFINITIONS[373].2, location: FIELD_DEFINITIONS[373].3 };
pub const ALLOC_CANC_REPLACE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[374].0, tag: FIELD_DEFINITIONS[374].1, data_type: FIELD_DEFINITIONS[374].2, location: FIELD_DEFINITIONS[374].3 };
pub const TARGET_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[375].0, tag: FIELD_DEFINITIONS[375].1, data_type: FIELD_DEFINITIONS[375].2, location: FIELD_DEFINITIONS[375].3 };
pub const LEG_POSITION_EFFECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[376].0, tag: FIELD_DEFINITIONS[376].1, data_type: FIELD_DEFINITIONS[376].2, location: FIELD_DEFINITIONS[376].3 };
pub const PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[377].0, tag: FIELD_DEFINITIONS[377].1, data_type: FIELD_DEFINITIONS[377].2, location: FIELD_DEFINITIONS[377].3 };
pub const DESIGNATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[378].0, tag: FIELD_DEFINITIONS[378].1, data_type: FIELD_DEFINITIONS[378].2, location: FIELD_DEFINITIONS[378].3 };
pub const BID_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[379].0, tag: FIELD_DEFINITIONS[379].1, data_type: FIELD_DEFINITIONS[379].2, location: FIELD_DEFINITIONS[379].3 };
pub const SECURITY_LIST_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[380].0, tag: FIELD_DEFINITIONS[380].1, data_type: FIELD_DEFINITIONS[380].2, location: FIELD_DEFINITIONS[380].3 };
pub const NO_CAPACITIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[381].0, tag: FIELD_DEFINITIONS[381].1, data_type: FIELD_DEFINITIONS[381].2, location: FIELD_DEFINITIONS[381].3 };
pub const HOP_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[382].0, tag: FIELD_DEFINITIONS[382].1, data_type: FIELD_DEFINITIONS[382].2, location: FIELD_DEFINITIONS[382].3 };
pub const CONTRA_TRADER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[383].0, tag: FIELD_DEFINITIONS[383].1, data_type: FIELD_DEFINITIONS[383].2, location: FIELD_DEFINITIONS[383].3 };
pub const LONG_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[384].0, tag: FIELD_DEFINITIONS[384].1, data_type: FIELD_DEFINITIONS[384].2, location: FIELD_DEFINITIONS[384].3 };
pub const VALUE_OF_FUTURES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[385].0, tag: FIELD_DEFINITIONS[385].1, data_type: FIELD_DEFINITIONS[385].2, location: FIELD_DEFINITIONS[385].3 };
pub const LEG_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[386].0, tag: FIELD_DEFINITIONS[386].1, data_type: FIELD_DEFINITIONS[386].2, location: FIELD_DEFINITIONS[386].3 };
pub const TRD_REG_TIMESTAMP: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[387].0, tag: FIELD_DEFINITIONS[387].1, data_type: FIELD_DEFINITIONS[387].2, location: FIELD_DEFINITIONS[387].3 };
pub const UNDERLYING_SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[388].0, tag: FIELD_DEFINITIONS[388].1, data_type: FIELD_DEFINITIONS[388].2, location: FIELD_DEFINITIONS[388].3 };
pub const AGREEMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[389].0, tag: FIELD_DEFINITIONS[389].1, data_type: FIELD_DEFINITIONS[389].2, location: FIELD_DEFINITIONS[389].3 };
pub const TRADE_LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[390].0, tag: FIELD_DEFINITIONS[390].1, data_type: FIELD_DEFINITIONS[390].2, location: FIELD_DEFINITIONS[390].3 };
pub const STAND_INST_DB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[391].0, tag: FIELD_DEFINITIONS[391].1, data_type: FIELD_DEFINITIONS[391].2, location: FIELD_DEFINITIONS[391].3 };
pub const SECURITY_RESPONSE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[392].0, tag: FIELD_DEFINITIONS[392].1, data_type: FIELD_DEFINITIONS[392].2, location: FIELD_DEFINITIONS[392].3 };
pub const BID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[393].0, tag: FIELD_DEFINITIONS[393].1, data_type: FIELD_DEFINITIONS[393].2, location: FIELD_DEFINITIONS[393].3 };
pub const SETTL_INST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[394].0, tag: FIELD_DEFINITIONS[394].1, data_type: FIELD_DEFINITIONS[394].2, location: FIELD_DEFINITIONS[394].3 };
pub const MATURITY_NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[395].0, tag: FIELD_DEFINITIONS[395].1, data_type: FIELD_DEFINITIONS[395].2, location: FIELD_DEFINITIONS[395].3 };
pub const MASS_CANCEL_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[396].0, tag: FIELD_DEFINITIONS[396].1, data_type: FIELD_DEFINITIONS[396].2, location: FIELD_DEFINITIONS[396].3 };
pub const SETTL_INST_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[397].0, tag: FIELD_DEFINITIONS[397].1, data_type: FIELD_DEFINITIONS[397].2, location: FIELD_DEFINITIONS[397].3 };
pub const GROSS_TRADE_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[398].0, tag: FIELD_DEFINITIONS[398].1, data_type: FIELD_DEFINITIONS[398].2, location: FIELD_DEFINITIONS[398].3 };
pub const TOT_NUM_TRADE_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[399].0, tag: FIELD_DEFINITIONS[399].1, data_type: FIELD_DEFINITIONS[399].2, location: FIELD_DEFINITIONS[399].3 };
pub const DISCRETION_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[400].0, tag: FIELD_DEFINITIONS[400].1, data_type: FIELD_DEFINITIONS[400].2, location: FIELD_DEFINITIONS[400].3 };
pub const TOTAL_AFFECTED_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[401].0, tag: FIELD_DEFINITIONS[401].1, data_type: FIELD_DEFINITIONS[401].2, location: FIELD_DEFINITIONS[401].3 };
pub const APPL_QUEUE_DEPTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[402].0, tag: FIELD_DEFINITIONS[402].1, data_type: FIELD_DEFINITIONS[402].2, location: FIELD_DEFINITIONS[402].3 };
pub const DELIVERY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[403].0, tag: FIELD_DEFINITIONS[403].1, data_type: FIELD_DEFINITIONS[403].2, location: FIELD_DEFINITIONS[403].3 };
pub const NO_NESTED_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[404].0, tag: FIELD_DEFINITIONS[404].1, data_type: FIELD_DEFINITIONS[404].2, location: FIELD_DEFINITIONS[404].3 };
pub const ALLOC_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[405].0, tag: FIELD_DEFINITIONS[405].1, data_type: FIELD_DEFINITIONS[405].2, location: FIELD_DEFINITIONS[405].3 };
pub const QUOTE_SET_VALID_UNTIL_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[406].0, tag: FIELD_DEFINITIONS[406].1, data_type: FIELD_DEFINITIONS[406].2, location: FIELD_DEFINITIONS[406].3 };
pub const CONTRA_TRADE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[407].0, tag: FIELD_DEFINITIONS[407].1, data_type: FIELD_DEFINITIONS[407].2, location: FIELD_DEFINITIONS[407].3 };
pub const NO_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[408].0, tag: FIELD_DEFINITIONS[408].1, data_type: FIELD_DEFINITIONS[408].2, location: FIELD_DEFINITIONS[408].3 };
pub const NO_INSTR_ATTRIB: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[409].0, tag: FIELD_DEFINITIONS[409].1, data_type: FIELD_DEFINITIONS[409].2, location: FIELD_DEFINITIONS[409].3 };
pub const MATCH_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[410].0, tag: FIELD_DEFINITIONS[410].1, data_type: FIELD_DEFINITIONS[410].2, location: FIELD_DEFINITIONS[410].3 };
pub const SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[411].0, tag: FIELD_DEFINITIONS[411].1, data_type: FIELD_DEFINITIONS[411].2, location: FIELD_DEFINITIONS[411].3 };
pub const MD_REQ_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[412].0, tag: FIELD_DEFINITIONS[412].1, data_type: FIELD_DEFINITIONS[412].2, location: FIELD_DEFINITIONS[412].3 };
pub const MIN_OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[413].0, tag: FIELD_DEFINITIONS[413].1, data_type: FIELD_DEFINITIONS[413].2, location: FIELD_DEFINITIONS[413].3 };
pub const INC_TAX_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[414].0, tag: FIELD_DEFINITIONS[414].1, data_type: FIELD_DEFINITIONS[414].2, location: FIELD_DEFINITIONS[414].3 };
pub const ROUND_LOT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[415].0, tag: FIELD_DEFINITIONS[415].1, data_type: FIELD_DEFINITIONS[415].2, location: FIELD_DEFINITIONS[415].3 };
pub const SETTL_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[416].0, tag: FIELD_DEFINITIONS[416].1, data_type: FIELD_DEFINITIONS[416].2, location: FIELD_DEFINITIONS[416].3 };
pub const OFFER_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[417].0, tag: FIELD_DEFINITIONS[417].1, data_type: FIELD_DEFINITIONS[417].2, location: FIELD_DEFINITIONS[417].3 };
pub const CONFIRM_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[418].0, tag: FIELD_DEFINITIONS[418].1, data_type: FIELD_DEFINITIONS[418].2, location: FIELD_DEFINITIONS[418].3 };
pub const POS_MAINT_RPT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[419].0, tag: FIELD_DEFINITIONS[419].1, data_type: FIELD_DEFINITIONS[419].2, location: FIELD_DEFINITIONS[419].3 };
pub const ON_BEHALF_OF_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[420].0, tag: FIELD_DEFINITIONS[420].1, data_type: FIELD_DEFINITIONS[420].2, location: FIELD_DEFINITIONS[420].3 };
pub const NO_MD_ENTRY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[421].0, tag: FIELD_DEFINITIONS[421].1, data_type: FIELD_DEFINITIONS[421].2, location: FIELD_DEFINITIONS[421].3 };
pub const TRAD_SES_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[422].0, tag: FIELD_DEFINITIONS[422].1, data_type: FIELD_DEFINITIONS[422].2, location: FIELD_DEFINITIONS[422].3 };
pub const SETTL_SESS_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[423].0, tag: FIELD_DEFINITIONS[423].1, data_type: FIELD_DEFINITIONS[423].2, location: FIELD_DEFINITIONS[423].3 };
pub const ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[424].0, tag: FIELD_DEFINITIONS[424].1, data_type: FIELD_DEFINITIONS[424].2, location: FIELD_DEFINITIONS[424].3 };
pub const TRD_REG_TIMESTAMP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[425].0, tag: FIELD_DEFINITIONS[425].1, data_type: FIELD_DEFINITIONS[425].2, location: FIELD_DEFINITIONS[425].3 };
pub const USER_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[426].0, tag: FIELD_DEFINITIONS[426].1, data_type: FIELD_DEFINITIONS[426].2, location: FIELD_DEFINITIONS[426].3 };
pub const QUOTE_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[427].0, tag: FIELD_DEFINITIONS[427].1, data_type: FIELD_DEFINITIONS[427].2, location: FIELD_DEFINITIONS[427].3 };
pub const HIGH_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[428].0, tag: FIELD_DEFINITIONS[428].1, data_type: FIELD_DEFINITIONS[428].2, location: FIELD_DEFINITIONS[428].3 };
pub const EX_DESTINATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[429].0, tag: FIELD_DEFINITIONS[429].1, data_type: FIELD_DEFINITIONS[429].2, location: FIELD_DEFINITIONS[429].3 };
pub const LEG_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[430].0, tag: FIELD_DEFINITIONS[430].1, data_type: FIELD_DEFINITIONS[430].2, location: FIELD_DEFINITIONS[430].3 };
pub const PEGGED_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[431].0, tag: FIELD_DEFINITIONS[431].1, data_type: FIELD_DEFINITIONS[431].2, location: FIELD_DEFINITIONS[431].3 };
pub const NESTED2_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[432].0, tag: FIELD_DEFINITIONS[432].1, data_type: FIELD_DEFINITIONS[432].2, location: FIELD_DEFINITIONS[432].3 };
pub const LAST_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[433].0, tag: FIELD_DEFINITIONS[433].1, data_type: FIELD_DEFINITIONS[433].2, location: FIELD_DEFINITIONS[433].3 };
pub const UNDERLYING_SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[434].0, tag: FIELD_DEFINITIONS[434].1, data_type: FIELD_DEFINITIONS[434].2, location: FIELD_DEFINITIONS[434].3 };
pub const NO_CONTRA_BROKERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[435].0, tag: FIELD_DEFINITIONS[435].1, data_type: FIELD_DEFINITIONS[435].2, location: FIELD_DEFINITIONS[435].3 };
pub const ASSIGNMENT_UNIT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[436].0, tag: FIELD_DEFINITIONS[436].1, data_type: FIELD_DEFINITIONS[436].2, location: FIELD_DEFINITIONS[436].3 };
pub const SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[437].0, tag: FIELD_DEFINITIONS[437].1, data_type: FIELD_DEFINITIONS[437].2, location: FIELD_DEFINITIONS[437].3 };
pub const NO_TRADES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[438].0, tag: FIELD_DEFINITIONS[438].1, data_type: FIELD_DEFINITIONS[438].2, location: FIELD_DEFINITIONS[438].3 };
pub const APPL_QUEUE_RESOLUTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[439].0, tag: FIELD_DEFINITIONS[439].1, data_type: FIELD_DEFINITIONS[439].2, location: FIELD_DEFINITIONS[439].3 };
pub const BODY_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[440].0, tag: FIELD_DEFINITIONS[440].1, data_type: FIELD_DEFINITIONS[440].2, location: FIELD_DEFINITIONS[440].3 };
pub const NO_NESTED3_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[441].0, tag: FIELD_DEFINITIONS[441].1, data_type: FIELD_DEFINITIONS[441].2, location: FIELD_DEFINITIONS[441].3 };
pub const MD_ENTRY_SELLER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[442].0, tag: FIELD_DEFINITIONS[442].1, data_type: FIELD_DEFINITIONS[442].2, location: FIELD_DEFINITIONS[442].3 };
pub const UNDERLYING_COUNTRY_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[443].0, tag: FIELD_DEFINITIONS[443].1, data_type: FIELD_DEFINITIONS[443].2, location: FIELD_DEFINITIONS[443].3 };
pub const NET_CHG_PREV_DAY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[444].0, tag: FIELD_DEFINITIONS[444].1, data_type: FIELD_DEFINITIONS[444].2, location: FIELD_DEFINITIONS[444].3 };
pub const ENCODED_ALLOC_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[445].0, tag: FIELD_DEFINITIONS[445].1, data_type: FIELD_DEFINITIONS[445].2, location: FIELD_DEFINITIONS[445].3 };
pub const LEG_SETTL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[446].0, tag: FIELD_DEFINITIONS[446].1, data_type: FIELD_DEFINITIONS[446].2, location: FIELD_DEFINITIONS[446].3 };
pub const EVENT_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[447].0, tag: FIELD_DEFINITIONS[447].1, data_type: FIELD_DEFINITIONS[447].2, location: FIELD_DEFINITIONS[447].3 };
pub const QUOTE_REQUEST_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[448].0, tag: FIELD_DEFINITIONS[448].1, data_type: FIELD_DEFINITIONS[448].2, location: FIELD_DEFINITIONS[448].3 };
pub const TIME_IN_FORCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[449].0, tag: FIELD_DEFINITIONS[449].1, data_type: FIELD_DEFINITIONS[449].2, location: FIELD_DEFINITIONS[449].3 };
pub const REGIST_DTLS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[450].0, tag: FIELD_DEFINITIONS[450].1, data_type: FIELD_DEFINITIONS[450].2, location: FIELD_DEFINITIONS[450].3 };
pub const MD_ENTRY_ORIGINATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[451].0, tag: FIELD_DEFINITIONS[451].1, data_type: FIELD_DEFINITIONS[451].2, location: FIELD_DEFINITIONS[451].3 };
pub const ALLOC_ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[452].0, tag: FIELD_DEFINITIONS[452].1, data_type: FIELD_DEFINITIONS[452].2, location: FIELD_DEFINITIONS[452].3 };
pub const SENDER_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[453].0, tag: FIELD_DEFINITIONS[453].1, data_type: FIELD_DEFINITIONS[453].2, location: FIELD_DEFINITIONS[453].3 };
pub const BENCHMARK_CURVE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[454].0, tag: FIELD_DEFINITIONS[454].1, data_type: FIELD_DEFINITIONS[454].2, location: FIELD_DEFINITIONS[454].3 };
pub const UNDERLYING_STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[455].0, tag: FIELD_DEFINITIONS[455].1, data_type: FIELD_DEFINITIONS[455].2, location: FIELD_DEFINITIONS[455].3 };
pub const QUANTITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[456].0, tag: FIELD_DEFINITIONS[456].1, data_type: FIELD_DEFINITIONS[456].2, location: FIELD_DEFINITIONS[456].3 };
pub const NETWORK_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[457].0, tag: FIELD_DEFINITIONS[457].1, data_type: FIELD_DEFINITIONS[457].2, location: FIELD_DEFINITIONS[457].3 };
pub const NO_ROUTING_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[458].0, tag: FIELD_DEFINITIONS[458].1, data_type: FIELD_DEFINITIONS[458].2, location: FIELD_DEFINITIONS[458].3 };
pub const WORKING_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[459].0, tag: FIELD_DEFINITIONS[459].1, data_type: FIELD_DEFINITIONS[459].2, location: FIELD_DEFINITIONS[459].3 };
pub const TAX_ADVANTAGE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[460].0, tag: FIELD_DEFINITIONS[460].1, data_type: FIELD_DEFINITIONS[460].2, location: FIELD_DEFINITIONS[460].3 };
pub const QUOTE_CONDITION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[461].0, tag: FIELD_DEFINITIONS[461].1, data_type: FIELD_DEFINITIONS[461].2, location: FIELD_DEFINITIONS[461].3 };
pub const PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[462].0, tag: FIELD_DEFINITIONS[462].1, data_type: FIELD_DEFINITIONS[462].2, location: FIELD_DEFINITIONS[462].3 };
pub const OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[463].0, tag: FIELD_DEFINITIONS[463].1, data_type: FIELD_DEFINITIONS[463].2, location: FIELD_DEFINITIONS[463].3 };
pub const MID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[464].0, tag: FIELD_DEFINITIONS[464].1, data_type: FIELD_DEFINITIONS[464].2, location: FIELD_DEFINITIONS[464].3 };
pub const SETTL_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[465].0, tag: FIELD_DEFINITIONS[465].1, data_type: FIELD_DEFINITIONS[465].2, location: FIELD_DEFINITIONS[465].3 };
pub const NO_POSITIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[466].0, tag: FIELD_DEFINITIONS[466].1, data_type: FIELD_DEFINITIONS[466].2, location: FIELD_DEFINITIONS[466].3 };
pub const ORD_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[467].0, tag: FIELD_DEFINITIONS[467].1, data_type: FIELD_DEFINITIONS[467].2, location: FIELD_DEFINITIONS[467].3 };
pub const SETTL_DATE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[468].0, tag: FIELD_DEFINITIONS[468].1, data_type: FIELD_DEFINITIONS[468].2, location: FIELD_DEFINITIONS[468].3 };
pub const LIQUIDITY_IND_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[469].0, tag: FIELD_DEFINITIONS[469].1, data_type: FIELD_DEFINITIONS[469].2, location: FIELD_DEFINITIONS[469].3 };
pub const MD_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[470].0, tag: FIELD_DEFINITIONS[470].1, data_type: FIELD_DEFINITIONS[470].2, location: FIELD_DEFINITIONS[470].3 };
pub const LEG_STIPULATION_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[471].0, tag: FIELD_DEFINITIONS[471].1, data_type: FIELD_DEFINITIONS[471].2, location: FIELD_DEFINITIONS[471].3 };
pub const LEG_COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[472].0, tag: FIELD_DEFINITIONS[472].1, data_type: FIELD_DEFINITIONS[472].2, location: FIELD_DEFINITIONS[472].3 };
pub const DISCRETION_MOVE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[473].0, tag: FIELD_DEFINITIONS[473].1, data_type: FIELD_DEFINITIONS[473].2, location: FIELD_DEFINITIONS[473].3 };
pub const TOT_NUM_ASSIGNMENT_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[474].0, tag: FIELD_DEFINITIONS[474].1, data_type: FIELD_DEFINITIONS[474].2, location: FIELD_DEFINITIONS[474].3 };
pub const UNDERLYING_REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[475].0, tag: FIELD_DEFINITIONS[475].1, data_type: FIELD_DEFINITIONS[475].2, location: FIELD_DEFINITIONS[475].3 };
pub const AFFECTED_SECONDARY_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[476].0, tag: FIELD_DEFINITIONS[476].1, data_type: FIELD_DEFINITIONS[476].2, location: FIELD_DEFINITIONS[476].3 };
pub const NUM_TICKETS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[477].0, tag: FIELD_DEFINITIONS[477].1, data_type: FIELD_DEFINITIONS[477].2, location: FIELD_DEFINITIONS[477].3 };
pub const TOT_NO_QUOTE_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[478].0, tag: FIELD_DEFINITIONS[478].1, data_type: FIELD_DEFINITIONS[478].2, location: FIELD_DEFINITIONS[478].3 };
pub const BOOKING_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[479].0, tag: FIELD_DEFINITIONS[479].1, data_type: FIELD_DEFINITIONS[479].2, location: FIELD_DEFINITIONS[479].3 };
pub const STIPULATION_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[480].0, tag: FIELD_DEFINITIONS[480].1, data_type: FIELD_DEFINITIONS[480].2, location: FIELD_DEFINITIONS[480].3 };
pub const MASS_CANCEL_RESPONSE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[481].0, tag: FIELD_DEFINITIONS[481].1, data_type: FIELD_DEFINITIONS[481].2, location: FIELD_DEFINITIONS[481].3 };
pub const MARGIN_RATIO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[482].0, tag: FIELD_DEFINITIONS[482].1, data_type: FIELD_DEFINITIONS[482].2, location: FIELD_DEFINITIONS[482].3 };
pub const PRICE_DELTA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[483].0, tag: FIELD_DEFINITIONS[483].1, data_type: FIELD_DEFINITIONS[483].2, location: FIELD_DEFINITIONS[483].3 };
pub const LEG_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[484].0, tag: FIELD_DEFINITIONS[484].1, data_type: FIELD_DEFINITIONS[484].2, location: FIELD_DEFINITIONS[484].3 };
pub const TOTAL_TAKEDOWN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[485].0, tag: FIELD_DEFINITIONS[485].1, data_type: FIELD_DEFINITIONS[485].2, location: FIELD_DEFINITIONS[485].3 };
pub const DISCRETION_OFFSET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[486].0, tag: FIELD_DEFINITIONS[486].1, data_type: FIELD_DEFINITIONS[486].2, location: FIELD_DEFINITIONS[486].3 };
pub const MD_ENTRY_POSITION_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[487].0, tag: FIELD_DEFINITIONS[487].1, data_type: FIELD_DEFINITIONS[487].2, location: FIELD_DEFINITIONS[487].3 };
pub const PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[488].0, tag: FIELD_DEFINITIONS[488].1, data_type: FIELD_DEFINITIONS[488].2, location: FIELD_DEFINITIONS[488].3 };
pub const LEG_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[489].0, tag: FIELD_DEFINITIONS[489].1, data_type: FIELD_DEFINITIONS[489].2, location: FIELD_DEFINITIONS[489].3 };
pub const FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[490].0, tag: FIELD_DEFINITIONS[490].1, data_type: FIELD_DEFINITIONS[490].2, location: FIELD_DEFINITIONS[490].3 };
pub const UNDERLYING_CURRENT_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[491].0, tag: FIELD_DEFINITIONS[491].1, data_type: FIELD_DEFINITIONS[491].2, location: FIELD_DEFINITIONS[491].3 };
pub const CP_REG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[492].0, tag: FIELD_DEFINITIONS[492].1, data_type: FIELD_DEFINITIONS[492].2, location: FIELD_DEFINITIONS[492].3 };
pub const ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[493].0, tag: FIELD_DEFINITIONS[493].1, data_type: FIELD_DEFINITIONS[493].2, location: FIELD_DEFINITIONS[493].3 };
pub const ORDER_PERCENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[494].0, tag: FIELD_DEFINITIONS[494].1, data_type: FIELD_DEFINITIONS[494].2, location: FIELD_DEFINITIONS[494].3 };
pub const NO_DISTRIB_INSTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[495].0, tag: FIELD_DEFINITIONS[495].1, data_type: FIELD_DEFINITIONS[495].2, location: FIELD_DEFINITIONS[495].3 };
pub const EXEC_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[496].0, tag: FIELD_DEFINITIONS[496].1, data_type: FIELD_DEFINITIONS[496].2, location: FIELD_DEFINITIONS[496].3 };
pub const ODD_LOT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[497].0, tag: FIELD_DEFINITIONS[497].1, data_type: FIELD_DEFINITIONS[497].2, location: FIELD_DEFINITIONS[497].3 };
pub const LEG_STRIKE_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[498].0, tag: FIELD_DEFINITIONS[498].1, data_type: FIELD_DEFINITIONS[498].2, location: FIELD_DEFINITIONS[498].3 };
pub const CUST_ORDER_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[499].0, tag: FIELD_DEFINITIONS[499].1, data_type: FIELD_DEFINITIONS[499].2, location: FIELD_DEFINITIONS[499].3 };
pub const MISC_FEE_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[500].0, tag: FIELD_DEFINITIONS[500].1, data_type: FIELD_DEFINITIONS[500].2, location: FIELD_DEFINITIONS[500].3 };
pub const NUM_BIDDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[501].0, tag: FIELD_DEFINITIONS[501].1, data_type: FIELD_DEFINITIONS[501].2, location: FIELD_DEFINITIONS[501].3 };
pub const PRIOR_SPREAD_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[502].0, tag: FIELD_DEFINITIONS[502].1, data_type: FIELD_DEFINITIONS[502].2, location: FIELD_DEFINITIONS[502].3 };
pub const CARD_HOLDER_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[503].0, tag: FIELD_DEFINITIONS[503].1, data_type: FIELD_DEFINITIONS[503].2, location: FIELD_DEFINITIONS[503].3 };
pub const MID_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[504].0, tag: FIELD_DEFINITIONS[504].1, data_type: FIELD_DEFINITIONS[504].2, location: FIELD_DEFINITIONS[504].3 };
pub const DELIVER_TO_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[505].0, tag: FIELD_DEFINITIONS[505].1, data_type: FIELD_DEFINITIONS[505].2, location: FIELD_DEFINITIONS[505].3 };
pub const SETTL_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[506].0, tag: FIELD_DEFINITIONS[506].1, data_type: FIELD_DEFINITIONS[506].2, location: FIELD_DEFINITIONS[506].3 };
pub const CLEARING_BUSINESS_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[507].0, tag: FIELD_DEFINITIONS[507].1, data_type: FIELD_DEFINITIONS[507].2, location: FIELD_DEFINITIONS[507].3 };
pub const INDIVIDUAL_ALLOC_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[508].0, tag: FIELD_DEFINITIONS[508].1, data_type: FIELD_DEFINITIONS[508].2, location: FIELD_DEFINITIONS[508].3 };
pub const CASH_MARGIN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[509].0, tag: FIELD_DEFINITIONS[509].1, data_type: FIELD_DEFINITIONS[509].2, location: FIELD_DEFINITIONS[509].3 };
pub const TRAD_SES_MODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[510].0, tag: FIELD_DEFINITIONS[510].1, data_type: FIELD_DEFINITIONS[510].2, location: FIELD_DEFINITIONS[510].3 };
pub const WT_AVERAGE_LIQUIDITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[511].0, tag: FIELD_DEFINITIONS[511].1, data_type: FIELD_DEFINITIONS[511].2, location: FIELD_DEFINITIONS[511].3 };
pub const DISCRETION_OFFSET_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[512].0, tag: FIELD_DEFINITIONS[512].1, data_type: FIELD_DEFINITIONS[512].2, location: FIELD_DEFINITIONS[512].3 };
pub const LEG_MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[513].0, tag: FIELD_DEFINITIONS[513].1, data_type: FIELD_DEFINITIONS[513].2, location: FIELD_DEFINITIONS[513].3 };
pub const LOW_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[514].0, tag: FIELD_DEFINITIONS[514].1, data_type: FIELD_DEFINITIONS[514].2, location: FIELD_DEFINITIONS[514].3 };
pub const LEG_SECURITY_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[515].0, tag: FIELD_DEFINITIONS[515].1, data_type: FIELD_DEFINITIONS[515].2, location: FIELD_DEFINITIONS[515].3 };
pub const LIQUIDITY_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[516].0, tag: FIELD_DEFINITIONS[516].1, data_type: FIELD_DEFINITIONS[516].2, location: FIELD_DEFINITIONS[516].3 };
pub const LEG_OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[517].0, tag: FIELD_DEFINITIONS[517].1, data_type: FIELD_DEFINITIONS[517].2, location: FIELD_DEFINITIONS[517].3 };
pub const SETTL_DELIVERY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[518].0, tag: FIELD_DEFINITIONS[518].1, data_type: FIELD_DEFINITIONS[518].2, location: FIELD_DEFINITIONS[518].3 };
pub const TRD_SUB_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[519].0, tag: FIELD_DEFINITIONS[519].1, data_type: FIELD_DEFINITIONS[519].2, location: FIELD_DEFINITIONS[519].3 };
pub const NESTED2_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[520].0, tag: FIELD_DEFINITIONS[520].1, data_type: FIELD_DEFINITIONS[520].2, location: FIELD_DEFINITIONS[520].3 };
pub const TOT_NUM_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[521].0, tag: FIELD_DEFINITIONS[521].1, data_type: FIELD_DEFINITIONS[521].2, location: FIELD_DEFINITIONS[521].3 };
pub const TRADE_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[522].0, tag: FIELD_DEFINITIONS[522].1, data_type: FIELD_DEFINITIONS[522].2, location: FIELD_DEFINITIONS[522].3 };
pub const SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[523].0, tag: FIELD_DEFINITIONS[523].1, data_type: FIELD_DEFINITIONS[523].2, location: FIELD_DEFINITIONS[523].3 };
pub const MAX_MESSAGE_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[524].0, tag: FIELD_DEFINITIONS[524].1, data_type: FIELD_DEFINITIONS[524].2, location: FIELD_DEFINITIONS[524].3 };
pub const BID_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[525].0, tag: FIELD_DEFINITIONS[525].1, data_type: FIELD_DEFINITIONS[525].2, location: FIELD_DEFINITIONS[525].3 };
pub const EMAIL_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[526].0, tag: FIELD_DEFINITIONS[526].1, data_type: FIELD_DEFINITIONS[526].2, location: FIELD_DEFINITIONS[526].3 };
pub const UNDERLYING_END_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[527].0, tag: FIELD_DEFINITIONS[527].1, data_type: FIELD_DEFINITIONS[527].2, location: FIELD_DEFINITIONS[527].3 };
pub const SECONDARY_CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[528].0, tag: FIELD_DEFINITIONS[528].1, data_type: FIELD_DEFINITIONS[528].2, location: FIELD_DEFINITIONS[528].3 };
pub const PROCESS_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[529].0, tag: FIELD_DEFINITIONS[529].1, data_type: FIELD_DEFINITIONS[529].2, location: FIELD_DEFINITIONS[529].3 };
pub const ENCODED_ALLOC_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[530].0, tag: FIELD_DEFINITIONS[530].1, data_type: FIELD_DEFINITIONS[530].2, location: FIELD_DEFINITIONS[530].3 };
pub const CONFIRM_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[531].0, tag: FIELD_DEFINITIONS[531].1, data_type: FIELD_DEFINITIONS[531].2, location: FIELD_DEFINITIONS[531].3 };
pub const EXPIRE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[532].0, tag: FIELD_DEFINITIONS[532].1, data_type: FIELD_DEFINITIONS[532].2, location: FIELD_DEFINITIONS[532].3 };
pub const REGIST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[533].0, tag: FIELD_DEFINITIONS[533].1, data_type: FIELD_DEFINITIONS[533].2, location: FIELD_DEFINITIONS[533].3 };
pub const REF_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[534].0, tag: FIELD_DEFINITIONS[534].1, data_type: FIELD_DEFINITIONS[534].2, location: FIELD_DEFINITIONS[534].3 };
pub const SIDE_COMPLIANCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[535].0, tag: FIELD_DEFINITIONS[535].1, data_type: FIELD_DEFINITIONS[535].2, location: FIELD_DEFINITIONS[535].3 };
pub const TRD_RPT_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[536].0, tag: FIELD_DEFINITIONS[536].1, data_type: FIELD_DEFINITIONS[536].2, location: FIELD_DEFINITIONS[536].3 };
pub const SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[537].0, tag: FIELD_DEFINITIONS[537].1, data_type: FIELD_DEFINITIONS[537].2, location: FIELD_DEFINITIONS[537].3 };
pub const LIST_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[538].0, tag: FIELD_DEFINITIONS[538].1, data_type: FIELD_DEFINITIONS[538].2, location: FIELD_DEFINITIONS[538].3 };
pub const LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[539].0, tag: FIELD_DEFINITIONS[539].1, data_type: FIELD_DEFINITIONS[539].2, location: FIELD_DEFINITIONS[539].3 };
pub const BID_TRADE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[540].0, tag: FIELD_DEFINITIONS[540].1, data_type: FIELD_DEFINITIONS[540].2, location: FIELD_DEFINITIONS[540].3 };
pub const UNDERLYING_SETTL_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[541].0, tag: FIELD_DEFINITIONS[541].1, data_type: FIELD_DEFINITIONS[541].2, location: FIELD_DEFINITIONS[541].3 };
pub const ORIG_SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[542].0, tag: FIELD_DEFINITIONS[542].1, data_type: FIELD_DEFINITIONS[542].2, location: FIELD_DEFINITIONS[542].3 };
pub const SETTL_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[543].0, tag: FIELD_DEFINITIONS[543].1, data_type: FIELD_DEFINITIONS[543].2, location: FIELD_DEFINITIONS[543].3 };
pub const PASSWORD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[544].0, tag: FIELD_DEFINITIONS[544].1, data_type: FIELD_DEFINITIONS[544].2, location: FIELD_DEFINITIONS[544].3 };
pub const TRADE_CONDITION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[545].0, tag: FIELD_DEFINITIONS[545].1, data_type: FIELD_DEFINITIONS[545].2, location: FIELD_DEFINITIONS[545].3 };
pub const RFQ_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[546].0, tag: FIELD_DEFINITIONS[546].1, data_type: FIELD_DEFINITIONS[546].2, location: FIELD_DEFINITIONS[546].3 };
pub const ENCODED_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[547].0, tag: FIELD_DEFINITIONS[547].1, data_type: FIELD_DEFINITIONS[547].2, location: FIELD_DEFINITIONS[547].3 };
pub const TOT_NO_SECURITY_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[548].0, tag: FIELD_DEFINITIONS[548].1, data_type: FIELD_DEFINITIONS[548].2, location: FIELD_DEFINITIONS[548].3 };
pub const ON_BEHALF_OF_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[549].0, tag: FIELD_DEFINITIONS[549].1, data_type: FIELD_DEFINITIONS[549].2, location: FIELD_DEFINITIONS[549].3 };
pub const POS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[550].0, tag: FIELD_DEFINITIONS[550].1, data_type: FIELD_DEFINITIONS[550].2, location: FIELD_DEFINITIONS[550].3 };
pub const SECONDARY_TRD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[551].0, tag: FIELD_DEFINITIONS[551].1, data_type: FIELD_DEFINITIONS[551].2, location: FIELD_DEFINITIONS[551].3 };
pub const POS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[552].0, tag: FIELD_DEFINITIONS[552].1, data_type: FIELD_DEFINITIONS[552].2, location: FIELD_DEFINITIONS[552].3 };
pub const MAX_FLOOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[553].0, tag: FIELD_DEFINITIONS[553].1, data_type: FIELD_DEFINITIONS[553].2, location: FIELD_DEFINITIONS[553].3 };
pub const SUBSCRIPTION_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[554].0, tag: FIELD_DEFINITIONS[554].1, data_type: FIELD_DEFINITIONS[554].2, location: FIELD_DEFINITIONS[554].3 };
pub const ADJUSTMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[555].0, tag: FIELD_DEFINITIONS[555].1, data_type: FIELD_DEFINITIONS[555].2, location: FIELD_DEFINITIONS[555].3 };
pub const BENCHMARK_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[556].0, tag: FIELD_DEFINITIONS[556].1, data_type: FIELD_DEFINITIONS[556].2, location: FIELD_DEFINITIONS[556].3 };
pub const CXL_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[557].0, tag: FIELD_DEFINITIONS[557].1, data_type: FIELD_DEFINITIONS[557].2, location: FIELD_DEFINITIONS[557].3 };
pub const AGREEMENT_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[558].0, tag: FIELD_DEFINITIONS[558].1, data_type: FIELD_DEFINITIONS[558].2, location: FIELD_DEFINITIONS[558].3 };
pub const TRANSFER_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[559].0, tag: FIELD_DEFINITIONS[559].1, data_type: FIELD_DEFINITIONS[559].2, location: FIELD_DEFINITIONS[559].3 };
pub const IOI_QLTY_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[560].0, tag: FIELD_DEFINITIONS[560].1, data_type: FIELD_DEFINITIONS[560].2, location: FIELD_DEFINITIONS[560].3 };
pub const COLL_RESP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[561].0, tag: FIELD_DEFINITIONS[561].1, data_type: FIELD_DEFINITIONS[561].2, location: FIELD_DEFINITIONS[561].3 };
pub const UNDERLYING_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[562].0, tag: FIELD_DEFINITIONS[562].1, data_type: FIELD_DEFINITIONS[562].2, location: FIELD_DEFINITIONS[562].3 };
pub const LEG_INDIVIDUAL_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[563].0, tag: FIELD_DEFINITIONS[563].1, data_type: FIELD_DEFINITIONS[563].2, location: FIELD_DEFINITIONS[563].3 };
pub const INDIVIDUAL_ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[564].0, tag: FIELD_DEFINITIONS[564].1, data_type: FIELD_DEFINITIONS[564].2, location: FIELD_DEFINITIONS[564].3 };
pub const COMPLIANCE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[565].0, tag: FIELD_DEFINITIONS[565].1, data_type: FIELD_DEFINITIONS[565].2, location: FIELD_DEFINITIONS[565].3 };
pub const END_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[566].0, tag: FIELD_DEFINITIONS[566].1, data_type: FIELD_DEFINITIONS[566].2, location: FIELD_DEFINITIONS[566].3 };
pub const LEG_SECURITY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[567].0, tag: FIELD_DEFINITIONS[567].1, data_type: FIELD_DEFINITIONS[567].2, location: FIELD_DEFINITIONS[567].3 };
pub const UNDERLYING_END_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[568].0, tag: FIELD_DEFINITIONS[568].1, data_type: FIELD_DEFINITIONS[568].2, location: FIELD_DEFINITIONS[568].3 };
pub const INTEREST_AT_MATURITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[569].0, tag: FIELD_DEFINITIONS[569].1, data_type: FIELD_DEFINITIONS[569].2, location: FIELD_DEFINITIONS[569].3 };
pub const CL_ORD_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[570].0, tag: FIELD_DEFINITIONS[570].1, data_type: FIELD_DEFINITIONS[570].2, location: FIELD_DEFINITIONS[570].3 };
pub const FINANCIAL_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[571].0, tag: FIELD_DEFINITIONS[571].1, data_type: FIELD_DEFINITIONS[571].2, location: FIELD_DEFINITIONS[571].3 };
pub const CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[572].0, tag: FIELD_DEFINITIONS[572].1, data_type: FIELD_DEFINITIONS[572].2, location: FIELD_DEFINITIONS[572].3 };
pub const ENCODED_UNDERLYING_ISSUER_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[573].0, tag: FIELD_DEFINITIONS[573].1, data_type: FIELD_DEFINITIONS[573].2, location: FIELD_DEFINITIONS[573].3 };
pub const ALLOC_INTEREST_AT_MATURITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[574].0, tag: FIELD_DEFINITIONS[574].1, data_type: FIELD_DEFINITIONS[574].2, location: FIELD_DEFINITIONS[574].3 };
pub const ADV_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[575].0, tag: FIELD_DEFINITIONS[575].1, data_type: FIELD_DEFINITIONS[575].2, location: FIELD_DEFINITIONS[575].3 };
pub const ADV_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[576].0, tag: FIELD_DEFINITIONS[576].1, data_type: FIELD_DEFINITIONS[576].2, location: FIELD_DEFINITIONS[576].3 };
pub const NO_NESTED3_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[577].0, tag: FIELD_DEFINITIONS[577].1, data_type: FIELD_DEFINITIONS[577].2, location: FIELD_DEFINITIONS[577].3 };
pub const LEG_SETTL_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[578].0, tag: FIELD_DEFINITIONS[578].1, data_type: FIELD_DEFINITIONS[578].2, location: FIELD_DEFINITIONS[578].3 };
pub const REGIST_EMAIL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[579].0, tag: FIELD_DEFINITIONS[579].1, data_type: FIELD_DEFINITIONS[579].2, location: FIELD_DEFINITIONS[579].3 };
pub const ENCODED_SUBJECT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[580].0, tag: FIELD_DEFINITIONS[580].1, data_type: FIELD_DEFINITIONS[580].2, location: FIELD_DEFINITIONS[580].3 };
pub const TRANSACT_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[581].0, tag: FIELD_DEFINITIONS[581].1, data_type: FIELD_DEFINITIONS[581].2, location: FIELD_DEFINITIONS[581].3 };
pub const BENCHMARK_CURVE_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[582].0, tag: FIELD_DEFINITIONS[582].1, data_type: FIELD_DEFINITIONS[582].2, location: FIELD_DEFINITIONS[582].3 };
pub const CL_ORD_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[583].0, tag: FIELD_DEFINITIONS[583].1, data_type: FIELD_DEFINITIONS[583].2, location: FIELD_DEFINITIONS[583].3 };
pub const ORDER_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[584].0, tag: FIELD_DEFINITIONS[584].1, data_type: FIELD_DEFINITIONS[584].2, location: FIELD_DEFINITIONS[584].3 };
pub const LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[585].0, tag: FIELD_DEFINITIONS[585].1, data_type: FIELD_DEFINITIONS[585].2, location: FIELD_DEFINITIONS[585].3 };
pub const SYMBOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[586].0, tag: FIELD_DEFINITIONS[586].1, data_type: FIELD_DEFINITIONS[586].2, location: FIELD_DEFINITIONS[586].3 };
pub const ALLOC_HANDL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[587].0, tag: FIELD_DEFINITIONS[587].1, data_type: FIELD_DEFINITIONS[587].2, location: FIELD_DEFINITIONS[587].3 };
pub const CARD_NUMBER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[588].0, tag: FIELD_DEFINITIONS[588].1, data_type: FIELD_DEFINITIONS[588].2, location: FIELD_DEFINITIONS[588].3 };
pub const MD_ENTRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[589].0, tag: FIELD_DEFINITIONS[589].1, data_type: FIELD_DEFINITIONS[589].2, location: FIELD_DEFINITIONS[589].3 };
pub const LAST_FORWARD_POINTS2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[590].0, tag: FIELD_DEFINITIONS[590].1, data_type: FIELD_DEFINITIONS[590].2, location: FIELD_DEFINITIONS[590].3 };
pub const MATURITY_MONTH_YEAR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[591].0, tag: FIELD_DEFINITIONS[591].1, data_type: FIELD_DEFINITIONS[591].2, location: FIELD_DEFINITIONS[591].3 };
pub const ALLOC_NO_ORDERS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[592].0, tag: FIELD_DEFINITIONS[592].1, data_type: FIELD_DEFINITIONS[592].2, location: FIELD_DEFINITIONS[592].3 };
pub const NO_LINES_OF_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[593].0, tag: FIELD_DEFINITIONS[593].1, data_type: FIELD_DEFINITIONS[593].2, location: FIELD_DEFINITIONS[593].3 };
pub const TARGET_STRATEGY_PARAMETERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[594].0, tag: FIELD_DEFINITIONS[594].1, data_type: FIELD_DEFINITIONS[594].2, location: FIELD_DEFINITIONS[594].3 };
pub const LAST_FORWARD_POINTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[595].0, tag: FIELD_DEFINITIONS[595].1, data_type: FIELD_DEFINITIONS[595].2, location: FIELD_DEFINITIONS[595].3 };
pub const LEG_SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[596].0, tag: FIELD_DEFINITIONS[596].1, data_type: FIELD_DEFINITIONS[596].2, location: FIELD_DEFINITIONS[596].3 };
pub const EXCHANGE_FOR_PHYSICAL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[597].0, tag: FIELD_DEFINITIONS[597].1, data_type: FIELD_DEFINITIONS[597].2, location: FIELD_DEFINITIONS[597].3 };
pub const UNDERLYING_CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[598].0, tag: FIELD_DEFINITIONS[598].1, data_type: FIELD_DEFINITIONS[598].2, location: FIELD_DEFINITIONS[598].3 };
pub const LEG_REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[599].0, tag: FIELD_DEFINITIONS[599].1, data_type: FIELD_DEFINITIONS[599].2, location: FIELD_DEFINITIONS[599].3 };
pub const LEG_MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[600].0, tag: FIELD_DEFINITIONS[600].1, data_type: FIELD_DEFINITIONS[600].2, location: FIELD_DEFINITIONS[600].3 };
pub const AGREEMENT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[601].0, tag: FIELD_DEFINITIONS[601].1, data_type: FIELD_DEFINITIONS[601].2, location: FIELD_DEFINITIONS[601].3 };
pub const EXPIRATION_CYCLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[602].0, tag: FIELD_DEFINITIONS[602].1, data_type: FIELD_DEFINITIONS[602].2, location: FIELD_DEFINITIONS[602].3 };
pub const LEG_IOI_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[603].0, tag: FIELD_DEFINITIONS[603].1, data_type: FIELD_DEFINITIONS[603].2, location: FIELD_DEFINITIONS[603].3 };
pub const LEG_FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[604].0, tag: FIELD_DEFINITIONS[604].1, data_type: FIELD_DEFINITIONS[604].2, location: FIELD_DEFINITIONS[604].3 };
pub const EFP_TRACKING_ERROR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[605].0, tag: FIELD_DEFINITIONS[605].1, data_type: FIELD_DEFINITIONS[605].2, location: FIELD_DEFINITIONS[605].3 };
pub const UNDERLYING_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[606].0, tag: FIELD_DEFINITIONS[606].1, data_type: FIELD_DEFINITIONS[606].2, location: FIELD_DEFINITIONS[606].3 };
pub const PAYMENT_REF: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[607].0, tag: FIELD_DEFINITIONS[607].1, data_type: FIELD_DEFINITIONS[607].2, location: FIELD_DEFINITIONS[607].3 };
pub const UNDERLYING_REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[608].0, tag: FIELD_DEFINITIONS[608].1, data_type: FIELD_DEFINITIONS[608].2, location: FIELD_DEFINITIONS[608].3 };
pub const PEG_LIMIT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[609].0, tag: FIELD_DEFINITIONS[609].1, data_type: FIELD_DEFINITIONS[609].2, location: FIELD_DEFINITIONS[609].3 };
pub const TOT_NO_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[610].0, tag: FIELD_DEFINITIONS[610].1, data_type: FIELD_DEFINITIONS[610].2, location: FIELD_DEFINITIONS[610].3 };
pub const REPO_COLLATERAL_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[611].0, tag: FIELD_DEFINITIONS[611].1, data_type: FIELD_DEFINITIONS[611].2, location: FIELD_DEFINITIONS[611].3 };
pub const MASS_CANCEL_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[612].0, tag: FIELD_DEFINITIONS[612].1, data_type: FIELD_DEFINITIONS[612].2, location: FIELD_DEFINITIONS[612].3 };
pub const CLIENT_BID_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[613].0, tag: FIELD_DEFINITIONS[613].1, data_type: FIELD_DEFINITIONS[613].2, location: FIELD_DEFINITIONS[613].3 };
pub const QUOTE_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[614].0, tag: FIELD_DEFINITIONS[614].1, data_type: FIELD_DEFINITIONS[614].2, location: FIELD_DEFINITIONS[614].3 };
pub const UNDERLYING_PRODUCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[615].0, tag: FIELD_DEFINITIONS[615].1, data_type: FIELD_DEFINITIONS[615].2, location: FIELD_DEFINITIONS[615].3 };
pub const EX_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[616].0, tag: FIELD_DEFINITIONS[616].1, data_type: FIELD_DEFINITIONS[616].2, location: FIELD_DEFINITIONS[616].3 };
pub const SECONDARY_EXEC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[617].0, tag: FIELD_DEFINITIONS[617].1, data_type: FIELD_DEFINITIONS[617].2, location: FIELD_DEFINITIONS[617].3 };
pub const NESTED2_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[618].0, tag: FIELD_DEFINITIONS[618].1, data_type: FIELD_DEFINITIONS[618].2, location: FIELD_DEFINITIONS[618].3 };
pub const LEG_LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[619].0, tag: FIELD_DEFINITIONS[619].1, data_type: FIELD_DEFINITIONS[619].2, location: FIELD_DEFINITIONS[619].3 };
pub const ALLOC_AVG_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[620].0, tag: FIELD_DEFINITIONS[620].1, data_type: FIELD_DEFINITIONS[620].2, location: FIELD_DEFINITIONS[620].3 };
pub const LIST_EXEC_INST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[621].0, tag: FIELD_DEFINITIONS[621].1, data_type: FIELD_DEFINITIONS[621].2, location: FIELD_DEFINITIONS[621].3 };
pub const PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[622].0, tag: FIELD_DEFINITIONS[622].1, data_type: FIELD_DEFINITIONS[622].2, location: FIELD_DEFINITIONS[622].3 };
pub const MASS_STATUS_REQ_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[623].0, tag: FIELD_DEFINITIONS[623].1, data_type: FIELD_DEFINITIONS[623].2, location: FIELD_DEFINITIONS[623].3 };
pub const ON_BEHALF_OF_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[624].0, tag: FIELD_DEFINITIONS[624].1, data_type: FIELD_DEFINITIONS[624].2, location: FIELD_DEFINITIONS[624].3 };
pub const EVENT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[625].0, tag: FIELD_DEFINITIONS[625].1, data_type: FIELD_DEFINITIONS[625].2, location: FIELD_DEFINITIONS[625].3 };
pub const SETTL_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[626].0, tag: FIELD_DEFINITIONS[626].1, data_type: FIELD_DEFINITIONS[626].2, location: FIELD_DEFINITIONS[626].3 };
pub const SETTL_INST_REQ_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[627].0, tag: FIELD_DEFINITIONS[627].1, data_type: FIELD_DEFINITIONS[627].2, location: FIELD_DEFINITIONS[627].3 };
pub const MISC_FEE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[628].0, tag: FIELD_DEFINITIONS[628].1, data_type: FIELD_DEFINITIONS[628].2, location: FIELD_DEFINITIONS[628].3 };
pub const SECURITY_REQUEST_RESULT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[629].0, tag: FIELD_DEFINITIONS[629].1, data_type: FIELD_DEFINITIONS[629].2, location: FIELD_DEFINITIONS[629].3 };
pub const BASIS_PX_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[630].0, tag: FIELD_DEFINITIONS[630].1, data_type: FIELD_DEFINITIONS[630].2, location: FIELD_DEFINITIONS[630].3 };
pub const CARD_EXP_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[631].0, tag: FIELD_DEFINITIONS[631].1, data_type: FIELD_DEFINITIONS[631].2, location: FIELD_DEFINITIONS[631].3 };
pub const IOI_NATURAL_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[632].0, tag: FIELD_DEFINITIONS[632].1, data_type: FIELD_DEFINITIONS[632].2, location: FIELD_DEFINITIONS[632].3 };
pub const NO_LEGS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[633].0, tag: FIELD_DEFINITIONS[633].1, data_type: FIELD_DEFINITIONS[633].2, location: FIELD_DEFINITIONS[633].3 };
pub const SHARED_COMMISSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[634].0, tag: FIELD_DEFINITIONS[634].1, data_type: FIELD_DEFINITIONS[634].2, location: FIELD_DEFINITIONS[634].3 };
pub const ALLOC_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[635].0, tag: FIELD_DEFINITIONS[635].1, data_type: FIELD_DEFINITIONS[635].2, location: FIELD_DEFINITIONS[635].3 };
pub const OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[636].0, tag: FIELD_DEFINITIONS[636].1, data_type: FIELD_DEFINITIONS[636].2, location: FIELD_DEFINITIONS[636].3 };
pub const ENCODED_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[637].0, tag: FIELD_DEFINITIONS[637].1, data_type: FIELD_DEFINITIONS[637].2, location: FIELD_DEFINITIONS[637].3 };
pub const SETTL_SESS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[638].0, tag: FIELD_DEFINITIONS[638].1, data_type: FIELD_DEFINITIONS[638].2, location: FIELD_DEFINITIONS[638].3 };
pub const EXEC_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[639].0, tag: FIELD_DEFINITIONS[639].1, data_type: FIELD_DEFINITIONS[639].2, location: FIELD_DEFINITIONS[639].3 };
pub const HOP_SENDING_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[640].0, tag: FIELD_DEFINITIONS[640].1, data_type: FIELD_DEFINITIONS[640].2, location: FIELD_DEFINITIONS[640].3 };
pub const BID_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[641].0, tag: FIELD_DEFINITIONS[641].1, data_type: FIELD_DEFINITIONS[641].2, location: FIELD_DEFINITIONS[641].3 };
pub const NO_SETTL_PARTY_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[642].0, tag: FIELD_DEFINITIONS[642].1, data_type: FIELD_DEFINITIONS[642].2, location: FIELD_DEFINITIONS[642].3 };
pub const NO_UNDERLYINGS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[643].0, tag: FIELD_DEFINITIONS[643].1, data_type: FIELD_DEFINITIONS[643].2, location: FIELD_DEFINITIONS[643].3 };
pub const USER_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[644].0, tag: FIELD_DEFINITIONS[644].1, data_type: FIELD_DEFINITIONS[644].2, location: FIELD_DEFINITIONS[644].3 };
pub const CONFIRM_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[645].0, tag: FIELD_DEFINITIONS[645].1, data_type: FIELD_DEFINITIONS[645].2, location: FIELD_DEFINITIONS[645].3 };
pub const TRAD_SES_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[646].0, tag: FIELD_DEFINITIONS[646].1, data_type: FIELD_DEFINITIONS[646].2, location: FIELD_DEFINITIONS[646].3 };
pub const FAIR_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[647].0, tag: FIELD_DEFINITIONS[647].1, data_type: FIELD_DEFINITIONS[647].2, location: FIELD_DEFINITIONS[647].3 };
pub const PEG_ROUND_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[648].0, tag: FIELD_DEFINITIONS[648].1, data_type: FIELD_DEFINITIONS[648].2, location: FIELD_DEFINITIONS[648].3 };
pub const UNDERLYING_MATURITY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[649].0, tag: FIELD_DEFINITIONS[649].1, data_type: FIELD_DEFINITIONS[649].2, location: FIELD_DEFINITIONS[649].3 };
pub const ALLOC_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[650].0, tag: FIELD_DEFINITIONS[650].1, data_type: FIELD_DEFINITIONS[650].2, location: FIELD_DEFINITIONS[650].3 };
pub const SOLICITED_FLAG: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[651].0, tag: FIELD_DEFINITIONS[651].1, data_type: FIELD_DEFINITIONS[651].2, location: FIELD_DEFINITIONS[651].3 };
pub const ASSIGNMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[652].0, tag: FIELD_DEFINITIONS[652].1, data_type: FIELD_DEFINITIONS[652].2, location: FIELD_DEFINITIONS[652].3 };
pub const NO_MSG_TYPES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[653].0, tag: FIELD_DEFINITIONS[653].1, data_type: FIELD_DEFINITIONS[653].2, location: FIELD_DEFINITIONS[653].3 };
pub const ORDER_RESTRICTIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[654].0, tag: FIELD_DEFINITIONS[654].1, data_type: FIELD_DEFINITIONS[654].2, location: FIELD_DEFINITIONS[654].3 };
pub const ALLOC_REJ_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[655].0, tag: FIELD_DEFINITIONS[655].1, data_type: FIELD_DEFINITIONS[655].2, location: FIELD_DEFINITIONS[655].3 };
pub const LEG_DATED_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[656].0, tag: FIELD_DEFINITIONS[656].1, data_type: FIELD_DEFINITIONS[656].2, location: FIELD_DEFINITIONS[656].3 };
pub const LEG_CONTRACT_SETTL_MONTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[657].0, tag: FIELD_DEFINITIONS[657].1, data_type: FIELD_DEFINITIONS[657].2, location: FIELD_DEFINITIONS[657].3 };
pub const ORDER_BOOKING_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[658].0, tag: FIELD_DEFINITIONS[658].1, data_type: FIELD_DEFINITIONS[658].2, location: FIELD_DEFINITIONS[658].3 };
pub const RPT_SEQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[659].0, tag: FIELD_DEFINITIONS[659].1, data_type: FIELD_DEFINITIONS[659].2, location: FIELD_DEFINITIONS[659].3 };
pub const ENCODED_UNDERLYING_ISSUER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[660].0, tag: FIELD_DEFINITIONS[660].1, data_type: FIELD_DEFINITIONS[660].2, location: FIELD_DEFINITIONS[660].3 };
pub const CXL_REJ_RESPONSE_TO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[661].0, tag: FIELD_DEFINITIONS[661].1, data_type: FIELD_DEFINITIONS[661].2, location: FIELD_DEFINITIONS[661].3 };
pub const AVG_PX_PRECISION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[662].0, tag: FIELD_DEFINITIONS[662].1, data_type: FIELD_DEFINITIONS[662].2, location: FIELD_DEFINITIONS[662].3 };
pub const EVENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[663].0, tag: FIELD_DEFINITIONS[663].1, data_type: FIELD_DEFINITIONS[663].2, location: FIELD_DEFINITIONS[663].3 };
pub const PREVIOUSLY_REPORTED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[664].0, tag: FIELD_DEFINITIONS[664].1, data_type: FIELD_DEFINITIONS[664].2, location: FIELD_DEFINITIONS[664].3 };
pub const POSITION_EFFECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[665].0, tag: FIELD_DEFINITIONS[665].1, data_type: FIELD_DEFINITIONS[665].2, location: FIELD_DEFINITIONS[665].3 };
pub const ENCODED_SUBJECT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[666].0, tag: FIELD_DEFINITIONS[666].1, data_type: FIELD_DEFINITIONS[666].2, location: FIELD_DEFINITIONS[666].3 };
pub const ACCT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[667].0, tag: FIELD_DEFINITIONS[667].1, data_type: FIELD_DEFINITIONS[667].2, location: FIELD_DEFINITIONS[667].3 };
pub const NO_STRIKES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[668].0, tag: FIELD_DEFINITIONS[668].1, data_type: FIELD_DEFINITIONS[668].2, location: FIELD_DEFINITIONS[668].3 };
pub const MATCH_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[669].0, tag: FIELD_DEFINITIONS[669].1, data_type: FIELD_DEFINITIONS[669].2, location: FIELD_DEFINITIONS[669].3 };
pub const TOT_NO_ORDERS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[670].0, tag: FIELD_DEFINITIONS[670].1, data_type: FIELD_DEFINITIONS[670].2, location: FIELD_DEFINITIONS[670].3 };
pub const CONTRA_LEG_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[671].0, tag: FIELD_DEFINITIONS[671].1, data_type: FIELD_DEFINITIONS[671].2, location: FIELD_DEFINITIONS[671].3 };
pub const NETWORK_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[672].0, tag: FIELD_DEFINITIONS[672].1, data_type: FIELD_DEFINITIONS[672].2, location: FIELD_DEFINITIONS[672].3 };
pub const RESPONSE_DESTINATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[673].0, tag: FIELD_DEFINITIONS[673].1, data_type: FIELD_DEFINITIONS[673].2, location: FIELD_DEFINITIONS[673].3 };
pub const DK_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[674].0, tag: FIELD_DEFINITIONS[674].1, data_type: FIELD_DEFINITIONS[674].2, location: FIELD_DEFINITIONS[674].3 };
pub const MD_UPDATE_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[675].0, tag: FIELD_DEFINITIONS[675].1, data_type: FIELD_DEFINITIONS[675].2, location: FIELD_DEFINITIONS[675].3 };
pub const ENCODED_SECURITY_DESC_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[676].0, tag: FIELD_DEFINITIONS[676].1, data_type: FIELD_DEFINITIONS[676].2, location: FIELD_DEFINITIONS[676].3 };
pub const ORIG_POS_REQ_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[677].0, tag: FIELD_DEFINITIONS[677].1, data_type: FIELD_DEFINITIONS[677].2, location: FIELD_DEFINITIONS[677].3 };
pub const NET_MONEY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[678].0, tag: FIELD_DEFINITIONS[678].1, data_type: FIELD_DEFINITIONS[678].2, location: FIELD_DEFINITIONS[678].3 };
pub const SETTL_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[679].0, tag: FIELD_DEFINITIONS[679].1, data_type: FIELD_DEFINITIONS[679].2, location: FIELD_DEFINITIONS[679].3 };
pub const CROSS_PRIORITIZATION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[680].0, tag: FIELD_DEFINITIONS[680].1, data_type: FIELD_DEFINITIONS[680].2, location: FIELD_DEFINITIONS[680].3 };
pub const END_ACCRUED_INTEREST_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[681].0, tag: FIELD_DEFINITIONS[681].1, data_type: FIELD_DEFINITIONS[681].2, location: FIELD_DEFINITIONS[681].3 };
pub const LEG_CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[682].0, tag: FIELD_DEFINITIONS[682].1, data_type: FIELD_DEFINITIONS[682].2, location: FIELD_DEFINITIONS[682].3 };
pub const LEG_STIPULATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[683].0, tag: FIELD_DEFINITIONS[683].1, data_type: FIELD_DEFINITIONS[683].2, location: FIELD_DEFINITIONS[683].3 };
pub const IN_VIEW_OF_COMMON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[684].0, tag: FIELD_DEFINITIONS[684].1, data_type: FIELD_DEFINITIONS[684].2, location: FIELD_DEFINITIONS[684].3 };
pub const RAW_DATA: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[685].0, tag: FIELD_DEFINITIONS[685].1, data_type: FIELD_DEFINITIONS[685].2, location: FIELD_DEFINITIONS[685].3 };
pub const NO_LEG_STIPULATIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[686].0, tag: FIELD_DEFINITIONS[686].1, data_type: FIELD_DEFINITIONS[686].2, location: FIELD_DEFINITIONS[686].3 };
pub const PEG_MOVE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[687].0, tag: FIELD_DEFINITIONS[687].1, data_type: FIELD_DEFINITIONS[687].2, location: FIELD_DEFINITIONS[687].3 };
pub const AUTO_ACCEPT_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[688].0, tag: FIELD_DEFINITIONS[688].1, data_type: FIELD_DEFINITIONS[688].2, location: FIELD_DEFINITIONS[688].3 };
pub const IOI_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[689].0, tag: FIELD_DEFINITIONS[689].1, data_type: FIELD_DEFINITIONS[689].2, location: FIELD_DEFINITIONS[689].3 };
pub const UNDERLYING_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[690].0, tag: FIELD_DEFINITIONS[690].1, data_type: FIELD_DEFINITIONS[690].2, location: FIELD_DEFINITIONS[690].3 };
pub const DISTRIB_PAYMENT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[691].0, tag: FIELD_DEFINITIONS[691].1, data_type: FIELD_DEFINITIONS[691].2, location: FIELD_DEFINITIONS[691].3 };
pub const EXEC_RESTATEMENT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[692].0, tag: FIELD_DEFINITIONS[692].1, data_type: FIELD_DEFINITIONS[692].2, location: FIELD_DEFINITIONS[692].3 };
pub const NESTED2_PARTY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[693].0, tag: FIELD_DEFINITIONS[693].1, data_type: FIELD_DEFINITIONS[693].2, location: FIELD_DEFINITIONS[693].3 };
pub const EXEC_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[694].0, tag: FIELD_DEFINITIONS[694].1, data_type: FIELD_DEFINITIONS[694].2, location: FIELD_DEFINITIONS[694].3 };
pub const COLL_INQUIRY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[695].0, tag: FIELD_DEFINITIONS[695].1, data_type: FIELD_DEFINITIONS[695].2, location: FIELD_DEFINITIONS[695].3 };
pub const UNDERLYING_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[696].0, tag: FIELD_DEFINITIONS[696].1, data_type: FIELD_DEFINITIONS[696].2, location: FIELD_DEFINITIONS[696].3 };
pub const HANDL_INST: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[697].0, tag: FIELD_DEFINITIONS[697].1, data_type: FIELD_DEFINITIONS[697].2, location: FIELD_DEFINITIONS[697].3 };
pub const TOTAL_NET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[698].0, tag: FIELD_DEFINITIONS[698].1, data_type: FIELD_DEFINITIONS[698].2, location: FIELD_DEFINITIONS[698].3 };
pub const QUOTE_RESPONSE_LEVEL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[699].0, tag: FIELD_DEFINITIONS[699].1, data_type: FIELD_DEFINITIONS[699].2, location: FIELD_DEFINITIONS[699].3 };
pub const NO_LEG_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[700].0, tag: FIELD_DEFINITIONS[700].1, data_type: FIELD_DEFINITIONS[700].2, location: FIELD_DEFINITIONS[700].3 };
pub const UNDERLYING_CFI_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[701].0, tag: FIELD_DEFINITIONS[701].1, data_type: FIELD_DEFINITIONS[701].2, location: FIELD_DEFINITIONS[701].3 };
pub const REF_MSG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[702].0, tag: FIELD_DEFINITIONS[702].1, data_type: FIELD_DEFINITIONS[702].2, location: FIELD_DEFINITIONS[702].3 };
pub const COMMISSION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[703].0, tag: FIELD_DEFINITIONS[703].1, data_type: FIELD_DEFINITIONS[703].2, location: FIELD_DEFINITIONS[703].3 };
pub const LEG_INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[704].0, tag: FIELD_DEFINITIONS[704].1, data_type: FIELD_DEFINITIONS[704].2, location: FIELD_DEFINITIONS[704].3 };
pub const UNDERLYING_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[705].0, tag: FIELD_DEFINITIONS[705].1, data_type: FIELD_DEFINITIONS[705].2, location: FIELD_DEFINITIONS[705].3 };
pub const NO_LEG_ALLOCS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[706].0, tag: FIELD_DEFINITIONS[706].1, data_type: FIELD_DEFINITIONS[706].2, location: FIELD_DEFINITIONS[706].3 };
pub const BEGIN_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[707].0, tag: FIELD_DEFINITIONS[707].1, data_type: FIELD_DEFINITIONS[707].2, location: FIELD_DEFINITIONS[707].3 };
pub const PAYMENT_REMITTER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[708].0, tag: FIELD_DEFINITIONS[708].1, data_type: FIELD_DEFINITIONS[708].2, location: FIELD_DEFINITIONS[708].3 };
pub const ENCODED_HEADLINE_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[709].0, tag: FIELD_DEFINITIONS[709].1, data_type: FIELD_DEFINITIONS[709].2, location: FIELD_DEFINITIONS[709].3 };
pub const SETTL_CURR_OFFER_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[710].0, tag: FIELD_DEFINITIONS[710].1, data_type: FIELD_DEFINITIONS[710].2, location: FIELD_DEFINITIONS[710].3 };
pub const VALID_UNTIL_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[711].0, tag: FIELD_DEFINITIONS[711].1, data_type: FIELD_DEFINITIONS[711].2, location: FIELD_DEFINITIONS[711].3 };
pub const ROUTING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[712].0, tag: FIELD_DEFINITIONS[712].1, data_type: FIELD_DEFINITIONS[712].2, location: FIELD_DEFINITIONS[712].3 };
pub const NETWORK_STATUS_RESPONSE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[713].0, tag: FIELD_DEFINITIONS[713].1, data_type: FIELD_DEFINITIONS[713].2, location: FIELD_DEFINITIONS[713].3 };
pub const SENDER_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[714].0, tag: FIELD_DEFINITIONS[714].1, data_type: FIELD_DEFINITIONS[714].2, location: FIELD_DEFINITIONS[714].3 };
pub const STATUS_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[715].0, tag: FIELD_DEFINITIONS[715].1, data_type: FIELD_DEFINITIONS[715].2, location: FIELD_DEFINITIONS[715].3 };
pub const PEG_OFFSET_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[716].0, tag: FIELD_DEFINITIONS[716].1, data_type: FIELD_DEFINITIONS[716].2, location: FIELD_DEFINITIONS[716].3 };
pub const BID_YIELD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[717].0, tag: FIELD_DEFINITIONS[717].1, data_type: FIELD_DEFINITIONS[717].2, location: FIELD_DEFINITIONS[717].3 };
pub const CARD_ISS_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[718].0, tag: FIELD_DEFINITIONS[718].1, data_type: FIELD_DEFINITIONS[718].2, location: FIELD_DEFINITIONS[718].3 };
pub const MD_ENTRY_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[719].0, tag: FIELD_DEFINITIONS[719].1, data_type: FIELD_DEFINITIONS[719].2, location: FIELD_DEFINITIONS[719].3 };
pub const ORD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[720].0, tag: FIELD_DEFINITIONS[720].1, data_type: FIELD_DEFINITIONS[720].2, location: FIELD_DEFINITIONS[720].3 };
pub const STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[721].0, tag: FIELD_DEFINITIONS[721].1, data_type: FIELD_DEFINITIONS[721].2, location: FIELD_DEFINITIONS[721].3 };
pub const NO_HOPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[722].0, tag: FIELD_DEFINITIONS[722].1, data_type: FIELD_DEFINITIONS[722].2, location: FIELD_DEFINITIONS[722].3 };
pub const LAST_UPDATE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[723].0, tag: FIELD_DEFINITIONS[723].1, data_type: FIELD_DEFINITIONS[723].2, location: FIELD_DEFINITIONS[723].3 };
pub const YIELD_REDEMPTION_PRICE_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[724].0, tag: FIELD_DEFINITIONS[724].1, data_type: FIELD_DEFINITIONS[724].2, location: FIELD_DEFINITIONS[724].3 };
pub const MSG_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[725].0, tag: FIELD_DEFINITIONS[725].1, data_type: FIELD_DEFINITIONS[725].2, location: FIELD_DEFINITIONS[725].3 };
pub const EXEC_PRICE_ADJUSTMENT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[726].0, tag: FIELD_DEFINITIONS[726].1, data_type: FIELD_DEFINITIONS[726].2, location: FIELD_DEFINITIONS[726].3 };
pub const TRADED_FLAT_SWITCH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[727].0, tag: FIELD_DEFINITIONS[727].1, data_type: FIELD_DEFINITIONS[727].2, location: FIELD_DEFINITIONS[727].3 };
pub const YIELD_CALC_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[728].0, tag: FIELD_DEFINITIONS[728].1, data_type: FIELD_DEFINITIONS[728].2, location: FIELD_DEFINITIONS[728].3 };
pub const ALLOC_LINK_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[729].0, tag: FIELD_DEFINITIONS[729].1, data_type: FIELD_DEFINITIONS[729].2, location: FIELD_DEFINITIONS[729].3 };
pub const SHORT_SALE_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[730].0, tag: FIELD_DEFINITIONS[730].1, data_type: FIELD_DEFINITIONS[730].2, location: FIELD_DEFINITIONS[730].3 };
pub const DISCRETION_ROUND_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[731].0, tag: FIELD_DEFINITIONS[731].1, data_type: FIELD_DEFINITIONS[731].2, location: FIELD_DEFINITIONS[731].3 };
pub const CREDIT_RATING: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[732].0, tag: FIELD_DEFINITIONS[732].1, data_type: FIELD_DEFINITIONS[732].2, location: FIELD_DEFINITIONS[732].3 };
pub const LEG_STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[733].0, tag: FIELD_DEFINITIONS[733].1, data_type: FIELD_DEFINITIONS[733].2, location: FIELD_DEFINITIONS[733].3 };
pub const OUTSIDE_INDEX_PCT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[734].0, tag: FIELD_DEFINITIONS[734].1, data_type: FIELD_DEFINITIONS[734].2, location: FIELD_DEFINITIONS[734].3 };
pub const UNDERLYING_STRIKE_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[735].0, tag: FIELD_DEFINITIONS[735].1, data_type: FIELD_DEFINITIONS[735].2, location: FIELD_DEFINITIONS[735].3 };
pub const CASH_DISTRIB_CURR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[736].0, tag: FIELD_DEFINITIONS[736].1, data_type: FIELD_DEFINITIONS[736].2, location: FIELD_DEFINITIONS[736].3 };
pub const UNDERLYING_FACTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[737].0, tag: FIELD_DEFINITIONS[737].1, data_type: FIELD_DEFINITIONS[737].2, location: FIELD_DEFINITIONS[737].3 };
pub const INSTR_REGISTRY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[738].0, tag: FIELD_DEFINITIONS[738].1, data_type: FIELD_DEFINITIONS[738].2, location: FIELD_DEFINITIONS[738].3 };
pub const COLL_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[739].0, tag: FIELD_DEFINITIONS[739].1, data_type: FIELD_DEFINITIONS[739].2, location: FIELD_DEFINITIONS[739].3 };
pub const UNDERLYING_TRADING_SESSION_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[740].0, tag: FIELD_DEFINITIONS[740].1, data_type: FIELD_DEFINITIONS[740].2, location: FIELD_DEFINITIONS[740].3 };
pub const LEG_BENCHMARK_CURVE_POINT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[741].0, tag: FIELD_DEFINITIONS[741].1, data_type: FIELD_DEFINITIONS[741].2, location: FIELD_DEFINITIONS[741].3 };
pub const STIPULATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[742].0, tag: FIELD_DEFINITIONS[742].1, data_type: FIELD_DEFINITIONS[742].2, location: FIELD_DEFINITIONS[742].3 };
pub const MSG_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[743].0, tag: FIELD_DEFINITIONS[743].1, data_type: FIELD_DEFINITIONS[743].2, location: FIELD_DEFINITIONS[743].3 };
pub const QUOTE_SET_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[744].0, tag: FIELD_DEFINITIONS[744].1, data_type: FIELD_DEFINITIONS[744].2, location: FIELD_DEFINITIONS[744].3 };
pub const SECURITY_ALT_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[745].0, tag: FIELD_DEFINITIONS[745].1, data_type: FIELD_DEFINITIONS[745].2, location: FIELD_DEFINITIONS[745].3 };
pub const LEG_SYMBOL_SFX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[746].0, tag: FIELD_DEFINITIONS[746].1, data_type: FIELD_DEFINITIONS[746].2, location: FIELD_DEFINITIONS[746].3 };
pub const COUPON_PAYMENT_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[747].0, tag: FIELD_DEFINITIONS[747].1, data_type: FIELD_DEFINITIONS[747].2, location: FIELD_DEFINITIONS[747].3 };
pub const SECONDARY_TRADE_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[748].0, tag: FIELD_DEFINITIONS[748].1, data_type: FIELD_DEFINITIONS[748].2, location: FIELD_DEFINITIONS[748].3 };
pub const INSTR_ATTRIB_VALUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[749].0, tag: FIELD_DEFINITIONS[749].1, data_type: FIELD_DEFINITIONS[749].2, location: FIELD_DEFINITIONS[749].3 };
pub const SETTL_CURR_FX_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[750].0, tag: FIELD_DEFINITIONS[750].1, data_type: FIELD_DEFINITIONS[750].2, location: FIELD_DEFINITIONS[750].3 };
pub const DISTRIB_PERCENTAGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[751].0, tag: FIELD_DEFINITIONS[751].1, data_type: FIELD_DEFINITIONS[751].2, location: FIELD_DEFINITIONS[751].3 };
pub const UNDERLYING_COUPON_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[752].0, tag: FIELD_DEFINITIONS[752].1, data_type: FIELD_DEFINITIONS[752].2, location: FIELD_DEFINITIONS[752].3 };
pub const REGIST_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[753].0, tag: FIELD_DEFINITIONS[753].1, data_type: FIELD_DEFINITIONS[753].2, location: FIELD_DEFINITIONS[753].3 };
pub const NO_RELATED_SYM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[754].0, tag: FIELD_DEFINITIONS[754].1, data_type: FIELD_DEFINITIONS[754].2, location: FIELD_DEFINITIONS[754].3 };
pub const TRADE_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[755].0, tag: FIELD_DEFINITIONS[755].1, data_type: FIELD_DEFINITIONS[755].2, location: FIELD_DEFINITIONS[755].3 };
pub const NO_COLL_INQUIRY_QUALIFIER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[756].0, tag: FIELD_DEFINITIONS[756].1, data_type: FIELD_DEFINITIONS[756].2, location: FIELD_DEFINITIONS[756].3 };
pub const TRADE_INPUT_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[757].0, tag: FIELD_DEFINITIONS[757].1, data_type: FIELD_DEFINITIONS[757].2, location: FIELD_DEFINITIONS[757].3 };
pub const URL_LINK: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[758].0, tag: FIELD_DEFINITIONS[758].1, data_type: FIELD_DEFINITIONS[758].2, location: FIELD_DEFINITIONS[758].3 };
pub const LIST_STATUS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[759].0, tag: FIELD_DEFINITIONS[759].1, data_type: FIELD_DEFINITIONS[759].2, location: FIELD_DEFINITIONS[759].3 };
pub const UNDERLYING_SETTL_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[760].0, tag: FIELD_DEFINITIONS[760].1, data_type: FIELD_DEFINITIONS[760].2, location: FIELD_DEFINITIONS[760].3 };
pub const CASH_DISTRIB_AGENT_ACCT_NUMBER: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[761].0, tag: FIELD_DEFINITIONS[761].1, data_type: FIELD_DEFINITIONS[761].2, location: FIELD_DEFINITIONS[761].3 };
pub const ACCOUNT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[762].0, tag: FIELD_DEFINITIONS[762].1, data_type: FIELD_DEFINITIONS[762].2, location: FIELD_DEFINITIONS[762].3 };
pub const PREV_CLOSE_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[763].0, tag: FIELD_DEFINITIONS[763].1, data_type: FIELD_DEFINITIONS[763].2, location: FIELD_DEFINITIONS[763].3 };
pub const COPY_MSG_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[764].0, tag: FIELD_DEFINITIONS[764].1, data_type: FIELD_DEFINITIONS[764].2, location: FIELD_DEFINITIONS[764].3 };
pub const TOTAL_NUM_POS_REPORTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[765].0, tag: FIELD_DEFINITIONS[765].1, data_type: FIELD_DEFINITIONS[765].2, location: FIELD_DEFINITIONS[765].3 };
pub const TERMINATION_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[766].0, tag: FIELD_DEFINITIONS[766].1, data_type: FIELD_DEFINITIONS[766].2, location: FIELD_DEFINITIONS[766].3 };
pub const OFFER_SIZE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[767].0, tag: FIELD_DEFINITIONS[767].1, data_type: FIELD_DEFINITIONS[767].2, location: FIELD_DEFINITIONS[767].3 };
pub const LEG_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[768].0, tag: FIELD_DEFINITIONS[768].1, data_type: FIELD_DEFINITIONS[768].2, location: FIELD_DEFINITIONS[768].3 };
pub const ENCODED_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[769].0, tag: FIELD_DEFINITIONS[769].1, data_type: FIELD_DEFINITIONS[769].2, location: FIELD_DEFINITIONS[769].3 };
pub const DATE_OF_BIRTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[770].0, tag: FIELD_DEFINITIONS[770].1, data_type: FIELD_DEFINITIONS[770].2, location: FIELD_DEFINITIONS[770].3 };
pub const OFFER_SPOT_RATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[771].0, tag: FIELD_DEFINITIONS[771].1, data_type: FIELD_DEFINITIONS[771].2, location: FIELD_DEFINITIONS[771].3 };
pub const ORIG_CROSS_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[772].0, tag: FIELD_DEFINITIONS[772].1, data_type: FIELD_DEFINITIONS[772].2, location: FIELD_DEFINITIONS[772].3 };
pub const QTY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[773].0, tag: FIELD_DEFINITIONS[773].1, data_type: FIELD_DEFINITIONS[773].2, location: FIELD_DEFINITIONS[773].3 };
pub const ENCODED_LEG_SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[774].0, tag: FIELD_DEFINITIONS[774].1, data_type: FIELD_DEFINITIONS[774].2, location: FIELD_DEFINITIONS[774].3 };
pub const DUE_TO_RELATED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[775].0, tag: FIELD_DEFINITIONS[775].1, data_type: FIELD_DEFINITIONS[775].2, location: FIELD_DEFINITIONS[775].3 };
pub const NESTED2_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[776].0, tag: FIELD_DEFINITIONS[776].1, data_type: FIELD_DEFINITIONS[776].2, location: FIELD_DEFINITIONS[776].3 };
pub const BID_DESCRIPTOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[777].0, tag: FIELD_DEFINITIONS[777].1, data_type: FIELD_DEFINITIONS[777].2, location: FIELD_DEFINITIONS[777].3 };
pub const LEG_SECURITY_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[778].0, tag: FIELD_DEFINITIONS[778].1, data_type: FIELD_DEFINITIONS[778].2, location: FIELD_DEFINITIONS[778].3 };
pub const EFFECTIVE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[779].0, tag: FIELD_DEFINITIONS[779].1, data_type: FIELD_DEFINITIONS[779].2, location: FIELD_DEFINITIONS[779].3 };
pub const EXCHANGE_RULE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[780].0, tag: FIELD_DEFINITIONS[780].1, data_type: FIELD_DEFINITIONS[780].2, location: FIELD_DEFINITIONS[780].3 };
pub const ALLOC_REPORT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[781].0, tag: FIELD_DEFINITIONS[781].1, data_type: FIELD_DEFINITIONS[781].2, location: FIELD_DEFINITIONS[781].3 };
pub const COLL_ASGN_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[782].0, tag: FIELD_DEFINITIONS[782].1, data_type: FIELD_DEFINITIONS[782].2, location: FIELD_DEFINITIONS[782].3 };
pub const NO_ALT_MD_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[783].0, tag: FIELD_DEFINITIONS[783].1, data_type: FIELD_DEFINITIONS[783].2, location: FIELD_DEFINITIONS[783].3 };
pub const SETTL_INST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[784].0, tag: FIELD_DEFINITIONS[784].1, data_type: FIELD_DEFINITIONS[784].2, location: FIELD_DEFINITIONS[784].3 };
pub const BUSINESS_REJECT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[785].0, tag: FIELD_DEFINITIONS[785].1, data_type: FIELD_DEFINITIONS[785].2, location: FIELD_DEFINITIONS[785].3 };
pub const NO_TRADING_SESSIONS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[786].0, tag: FIELD_DEFINITIONS[786].1, data_type: FIELD_DEFINITIONS[786].2, location: FIELD_DEFINITIONS[786].3 };
pub const SECURE_DATA_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[787].0, tag: FIELD_DEFINITIONS[787].1, data_type: FIELD_DEFINITIONS[787].2, location: FIELD_DEFINITIONS[787].3 };
pub const UNDERLYING_DIRTY_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[788].0, tag: FIELD_DEFINITIONS[788].1, data_type: FIELD_DEFINITIONS[788].2, location: FIELD_DEFINITIONS[788].3 };
pub const OWNER_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[789].0, tag: FIELD_DEFINITIONS[789].1, data_type: FIELD_DEFINITIONS[789].2, location: FIELD_DEFINITIONS[789].3 };
pub const SIGNATURE_LENGTH: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[790].0, tag: FIELD_DEFINITIONS[790].1, data_type: FIELD_DEFINITIONS[790].2, location: FIELD_DEFINITIONS[790].3 };
pub const SESSION_REJECT_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[791].0, tag: FIELD_DEFINITIONS[791].1, data_type: FIELD_DEFINITIONS[791].2, location: FIELD_DEFINITIONS[791].3 };
pub const LEG_POOL: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[792].0, tag: FIELD_DEFINITIONS[792].1, data_type: FIELD_DEFINITIONS[792].2, location: FIELD_DEFINITIONS[792].3 };
pub const LIST_STATUS_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[793].0, tag: FIELD_DEFINITIONS[793].1, data_type: FIELD_DEFINITIONS[793].2, location: FIELD_DEFINITIONS[793].3 };
pub const NESTED_PARTY_ID_SOURCE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[794].0, tag: FIELD_DEFINITIONS[794].1, data_type: FIELD_DEFINITIONS[794].2, location: FIELD_DEFINITIONS[794].3 };
pub const CXL_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[795].0, tag: FIELD_DEFINITIONS[795].1, data_type: FIELD_DEFINITIONS[795].2, location: FIELD_DEFINITIONS[795].3 };
pub const NESTED_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[796].0, tag: FIELD_DEFINITIONS[796].1, data_type: FIELD_DEFINITIONS[796].2, location: FIELD_DEFINITIONS[796].3 };
pub const LEG_ALLOC_ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[797].0, tag: FIELD_DEFINITIONS[797].1, data_type: FIELD_DEFINITIONS[797].2, location: FIELD_DEFINITIONS[797].3 };
pub const NESTED3_PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[798].0, tag: FIELD_DEFINITIONS[798].1, data_type: FIELD_DEFINITIONS[798].2, location: FIELD_DEFINITIONS[798].3 };
pub const ALLOC_ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[799].0, tag: FIELD_DEFINITIONS[799].1, data_type: FIELD_DEFINITIONS[799].2, location: FIELD_DEFINITIONS[799].3 };
pub const ENCODED_HEADLINE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[800].0, tag: FIELD_DEFINITIONS[800].1, data_type: FIELD_DEFINITIONS[800].2, location: FIELD_DEFINITIONS[800].3 };
pub const NET_GROSS_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[801].0, tag: FIELD_DEFINITIONS[801].1, data_type: FIELD_DEFINITIONS[801].2, location: FIELD_DEFINITIONS[801].3 };
pub const POS_REQ_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[802].0, tag: FIELD_DEFINITIONS[802].1, data_type: FIELD_DEFINITIONS[802].2, location: FIELD_DEFINITIONS[802].3 };
pub const ALLOC_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[803].0, tag: FIELD_DEFINITIONS[803].1, data_type: FIELD_DEFINITIONS[803].2, location: FIELD_DEFINITIONS[803].3 };
pub const ALLOC_ACCOUNT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[804].0, tag: FIELD_DEFINITIONS[804].1, data_type: FIELD_DEFINITIONS[804].2, location: FIELD_DEFINITIONS[804].3 };
pub const LEG_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[805].0, tag: FIELD_DEFINITIONS[805].1, data_type: FIELD_DEFINITIONS[805].2, location: FIELD_DEFINITIONS[805].3 };
pub const FOREX_REQ: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[806].0, tag: FIELD_DEFINITIONS[806].1, data_type: FIELD_DEFINITIONS[806].2, location: FIELD_DEFINITIONS[806].3 };
pub const MD_ENTRY_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[807].0, tag: FIELD_DEFINITIONS[807].1, data_type: FIELD_DEFINITIONS[807].2, location: FIELD_DEFINITIONS[807].3 };
pub const PRICE2: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[808].0, tag: FIELD_DEFINITIONS[808].1, data_type: FIELD_DEFINITIONS[808].2, location: FIELD_DEFINITIONS[808].3 };
pub const TRAD_SES_CLOSE_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[809].0, tag: FIELD_DEFINITIONS[809].1, data_type: FIELD_DEFINITIONS[809].2, location: FIELD_DEFINITIONS[809].3 };
pub const USERNAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[810].0, tag: FIELD_DEFINITIONS[810].1, data_type: FIELD_DEFINITIONS[810].2, location: FIELD_DEFINITIONS[810].3 };
pub const TEST_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[811].0, tag: FIELD_DEFINITIONS[811].1, data_type: FIELD_DEFINITIONS[811].2, location: FIELD_DEFINITIONS[811].3 };
pub const BENCHMARK_SECURITY_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[812].0, tag: FIELD_DEFINITIONS[812].1, data_type: FIELD_DEFINITIONS[812].2, location: FIELD_DEFINITIONS[812].3 };
pub const LAST_LIQUIDITY_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[813].0, tag: FIELD_DEFINITIONS[813].1, data_type: FIELD_DEFINITIONS[813].2, location: FIELD_DEFINITIONS[813].3 };
pub const POS_QTY_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[814].0, tag: FIELD_DEFINITIONS[814].1, data_type: FIELD_DEFINITIONS[814].2, location: FIELD_DEFINITIONS[814].3 };
pub const SECURITY_DESC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[815].0, tag: FIELD_DEFINITIONS[815].1, data_type: FIELD_DEFINITIONS[815].2, location: FIELD_DEFINITIONS[815].3 };
pub const BASIS_FEATURE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[816].0, tag: FIELD_DEFINITIONS[816].1, data_type: FIELD_DEFINITIONS[816].2, location: FIELD_DEFINITIONS[816].3 };
pub const BUY_VOLUME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[817].0, tag: FIELD_DEFINITIONS[817].1, data_type: FIELD_DEFINITIONS[817].2, location: FIELD_DEFINITIONS[817].3 };
pub const POS_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[818].0, tag: FIELD_DEFINITIONS[818].1, data_type: FIELD_DEFINITIONS[818].2, location: FIELD_DEFINITIONS[818].3 };
pub const ENCRYPT_METHOD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[819].0, tag: FIELD_DEFINITIONS[819].1, data_type: FIELD_DEFINITIONS[819].2, location: FIELD_DEFINITIONS[819].3 };
pub const NEW_PASSWORD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[820].0, tag: FIELD_DEFINITIONS[820].1, data_type: FIELD_DEFINITIONS[820].2, location: FIELD_DEFINITIONS[820].3 };
pub const TRADE_ALLOC_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[821].0, tag: FIELD_DEFINITIONS[821].1, data_type: FIELD_DEFINITIONS[821].2, location: FIELD_DEFINITIONS[821].3 };
pub const START_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[822].0, tag: FIELD_DEFINITIONS[822].1, data_type: FIELD_DEFINITIONS[822].2, location: FIELD_DEFINITIONS[822].3 };
pub const UNDERLYING_OPT_ATTRIBUTE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[823].0, tag: FIELD_DEFINITIONS[823].1, data_type: FIELD_DEFINITIONS[823].2, location: FIELD_DEFINITIONS[823].3 };
pub const LEG_OFFER_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[824].0, tag: FIELD_DEFINITIONS[824].1, data_type: FIELD_DEFINITIONS[824].2, location: FIELD_DEFINITIONS[824].3 };
pub const COMM_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[825].0, tag: FIELD_DEFINITIONS[825].1, data_type: FIELD_DEFINITIONS[825].2, location: FIELD_DEFINITIONS[825].3 };
pub const SECURITY_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[826].0, tag: FIELD_DEFINITIONS[826].1, data_type: FIELD_DEFINITIONS[826].2, location: FIELD_DEFINITIONS[826].3 };
pub const IOI_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[827].0, tag: FIELD_DEFINITIONS[827].1, data_type: FIELD_DEFINITIONS[827].2, location: FIELD_DEFINITIONS[827].3 };
pub const LEG_BENCHMARK_PRICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[828].0, tag: FIELD_DEFINITIONS[828].1, data_type: FIELD_DEFINITIONS[828].2, location: FIELD_DEFINITIONS[828].3 };
pub const COLL_ASGN_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[829].0, tag: FIELD_DEFINITIONS[829].1, data_type: FIELD_DEFINITIONS[829].2, location: FIELD_DEFINITIONS[829].3 };
pub const TRADE_REQUEST_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[830].0, tag: FIELD_DEFINITIONS[830].1, data_type: FIELD_DEFINITIONS[830].2, location: FIELD_DEFINITIONS[830].3 };
pub const IOIID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[831].0, tag: FIELD_DEFINITIONS[831].1, data_type: FIELD_DEFINITIONS[831].2, location: FIELD_DEFINITIONS[831].3 };
pub const QUOTE_REQUEST_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[832].0, tag: FIELD_DEFINITIONS[832].1, data_type: FIELD_DEFINITIONS[832].2, location: FIELD_DEFINITIONS[832].3 };
pub const NO_UNDERLYING_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[833].0, tag: FIELD_DEFINITIONS[833].1, data_type: FIELD_DEFINITIONS[833].2, location: FIELD_DEFINITIONS[833].3 };
pub const BID_REQUEST_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[834].0, tag: FIELD_DEFINITIONS[834].1, data_type: FIELD_DEFINITIONS[834].2, location: FIELD_DEFINITIONS[834].3 };
pub const ALLOC_SETTL_CURR_AMT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[835].0, tag: FIELD_DEFINITIONS[835].1, data_type: FIELD_DEFINITIONS[835].2, location: FIELD_DEFINITIONS[835].3 };
pub const CUM_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[836].0, tag: FIELD_DEFINITIONS[836].1, data_type: FIELD_DEFINITIONS[836].2, location: FIELD_DEFINITIONS[836].3 };
pub const NESTED3_PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[837].0, tag: FIELD_DEFINITIONS[837].1, data_type: FIELD_DEFINITIONS[837].2, location: FIELD_DEFINITIONS[837].3 };
pub const NO_NESTED2_PARTY_SUB_I_DS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[838].0, tag: FIELD_DEFINITIONS[838].1, data_type: FIELD_DEFINITIONS[838].2, location: FIELD_DEFINITIONS[838].3 };
pub const ACCOUNT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[839].0, tag: FIELD_DEFINITIONS[839].1, data_type: FIELD_DEFINITIONS[839].2, location: FIELD_DEFINITIONS[839].3 };
pub const COLL_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[840].0, tag: FIELD_DEFINITIONS[840].1, data_type: FIELD_DEFINITIONS[840].2, location: FIELD_DEFINITIONS[840].3 };
pub const MASS_STATUS_REQ_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[841].0, tag: FIELD_DEFINITIONS[841].1, data_type: FIELD_DEFINITIONS[841].2, location: FIELD_DEFINITIONS[841].3 };
pub const REGIST_REJ_REASON_CODE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[842].0, tag: FIELD_DEFINITIONS[842].1, data_type: FIELD_DEFINITIONS[842].2, location: FIELD_DEFINITIONS[842].3 };
pub const ENCODED_LIST_EXEC_INST_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[843].0, tag: FIELD_DEFINITIONS[843].1, data_type: FIELD_DEFINITIONS[843].2, location: FIELD_DEFINITIONS[843].3 };
pub const TARGET_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[844].0, tag: FIELD_DEFINITIONS[844].1, data_type: FIELD_DEFINITIONS[844].2, location: FIELD_DEFINITIONS[844].3 };
pub const SPREAD: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[845].0, tag: FIELD_DEFINITIONS[845].1, data_type: FIELD_DEFINITIONS[845].2, location: FIELD_DEFINITIONS[845].3 };
pub const TRADE_INPUT_DEVICE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[846].0, tag: FIELD_DEFINITIONS[846].1, data_type: FIELD_DEFINITIONS[846].2, location: FIELD_DEFINITIONS[846].3 };
pub const ALLOC_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[847].0, tag: FIELD_DEFINITIONS[847].1, data_type: FIELD_DEFINITIONS[847].2, location: FIELD_DEFINITIONS[847].3 };
pub const LEGAL_CONFIRM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[848].0, tag: FIELD_DEFINITIONS[848].1, data_type: FIELD_DEFINITIONS[848].2, location: FIELD_DEFINITIONS[848].3 };
pub const BENCHMARK_CURVE_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[849].0, tag: FIELD_DEFINITIONS[849].1, data_type: FIELD_DEFINITIONS[849].2, location: FIELD_DEFINITIONS[849].3 };
pub const CASH_DISTRIB_PAY_REF: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[850].0, tag: FIELD_DEFINITIONS[850].1, data_type: FIELD_DEFINITIONS[850].2, location: FIELD_DEFINITIONS[850].3 };
pub const TICK_DIRECTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[851].0, tag: FIELD_DEFINITIONS[851].1, data_type: FIELD_DEFINITIONS[851].2, location: FIELD_DEFINITIONS[851].3 };
pub const ORIG_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[852].0, tag: FIELD_DEFINITIONS[852].1, data_type: FIELD_DEFINITIONS[852].2, location: FIELD_DEFINITIONS[852].3 };
pub const XML_DATA_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[853].0, tag: FIELD_DEFINITIONS[853].1, data_type: FIELD_DEFINITIONS[853].2, location: FIELD_DEFINITIONS[853].3 };
pub const NETWORK_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[854].0, tag: FIELD_DEFINITIONS[854].1, data_type: FIELD_DEFINITIONS[854].2, location: FIELD_DEFINITIONS[854].3 };
pub const REF_SEQ_NUM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[855].0, tag: FIELD_DEFINITIONS[855].1, data_type: FIELD_DEFINITIONS[855].2, location: FIELD_DEFINITIONS[855].3 };
pub const AVG_PAR_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[856].0, tag: FIELD_DEFINITIONS[856].1, data_type: FIELD_DEFINITIONS[856].2, location: FIELD_DEFINITIONS[856].3 };
pub const SECURITY_EXCHANGE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[857].0, tag: FIELD_DEFINITIONS[857].1, data_type: FIELD_DEFINITIONS[857].2, location: FIELD_DEFINITIONS[857].3 };
pub const HOP_COMP_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[858].0, tag: FIELD_DEFINITIONS[858].1, data_type: FIELD_DEFINITIONS[858].2, location: FIELD_DEFINITIONS[858].3 };
pub const TRADE_REPORT_TRANS_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[859].0, tag: FIELD_DEFINITIONS[859].1, data_type: FIELD_DEFINITIONS[859].2, location: FIELD_DEFINITIONS[859].3 };
pub const NO_MD_ENTRIES: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[860].0, tag: FIELD_DEFINITIONS[860].1, data_type: FIELD_DEFINITIONS[860].2, location: FIELD_DEFINITIONS[860].3 };
pub const NEW_SEQ_NO: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[861].0, tag: FIELD_DEFINITIONS[861].1, data_type: FIELD_DEFINITIONS[861].2, location: FIELD_DEFINITIONS[861].3 };
pub const SECONDARY_ORDER_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[862].0, tag: FIELD_DEFINITIONS[862].1, data_type: FIELD_DEFINITIONS[862].2, location: FIELD_DEFINITIONS[862].3 };
pub const LEG_RATIO_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[863].0, tag: FIELD_DEFINITIONS[863].1, data_type: FIELD_DEFINITIONS[863].2, location: FIELD_DEFINITIONS[863].3 };
pub const USER_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[864].0, tag: FIELD_DEFINITIONS[864].1, data_type: FIELD_DEFINITIONS[864].2, location: FIELD_DEFINITIONS[864].3 };
pub const BOOKING_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[865].0, tag: FIELD_DEFINITIONS[865].1, data_type: FIELD_DEFINITIONS[865].2, location: FIELD_DEFINITIONS[865].3 };
pub const QUOTE_RESP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[866].0, tag: FIELD_DEFINITIONS[866].1, data_type: FIELD_DEFINITIONS[866].2, location: FIELD_DEFINITIONS[866].3 };
pub const LEG_ISSUE_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[867].0, tag: FIELD_DEFINITIONS[867].1, data_type: FIELD_DEFINITIONS[867].2, location: FIELD_DEFINITIONS[867].3 };
pub const SIDE_VALUE_IND: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[868].0, tag: FIELD_DEFINITIONS[868].1, data_type: FIELD_DEFINITIONS[868].2, location: FIELD_DEFINITIONS[868].3 };
pub const UNDERLYING_CURRENCY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[869].0, tag: FIELD_DEFINITIONS[869].1, data_type: FIELD_DEFINITIONS[869].2, location: FIELD_DEFINITIONS[869].3 };
pub const LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[870].0, tag: FIELD_DEFINITIONS[870].1, data_type: FIELD_DEFINITIONS[870].2, location: FIELD_DEFINITIONS[870].3 };
pub const LEG_BID_PX: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[871].0, tag: FIELD_DEFINITIONS[871].1, data_type: FIELD_DEFINITIONS[871].2, location: FIELD_DEFINITIONS[871].3 };
pub const REDEMPTION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[872].0, tag: FIELD_DEFINITIONS[872].1, data_type: FIELD_DEFINITIONS[872].2, location: FIELD_DEFINITIONS[872].3 };
pub const ASGN_RPT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[873].0, tag: FIELD_DEFINITIONS[873].1, data_type: FIELD_DEFINITIONS[873].2, location: FIELD_DEFINITIONS[873].3 };
pub const UNDERLYING_STIP_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[874].0, tag: FIELD_DEFINITIONS[874].1, data_type: FIELD_DEFINITIONS[874].2, location: FIELD_DEFINITIONS[874].3 };
pub const YIELD_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[875].0, tag: FIELD_DEFINITIONS[875].1, data_type: FIELD_DEFINITIONS[875].2, location: FIELD_DEFINITIONS[875].3 };
pub const ORDER_CAPACITY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[876].0, tag: FIELD_DEFINITIONS[876].1, data_type: FIELD_DEFINITIONS[876].2, location: FIELD_DEFINITIONS[876].3 };
pub const TOTAL_VOLUME_TRADED: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[877].0, tag: FIELD_DEFINITIONS[877].1, data_type: FIELD_DEFINITIONS[877].2, location: FIELD_DEFINITIONS[877].3 };
pub const NO_QUOTE_SETS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[878].0, tag: FIELD_DEFINITIONS[878].1, data_type: FIELD_DEFINITIONS[878].2, location: FIELD_DEFINITIONS[878].3 };
pub const UNDERLYING_SECURITY_ALT_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[879].0, tag: FIELD_DEFINITIONS[879].1, data_type: FIELD_DEFINITIONS[879].2, location: FIELD_DEFINITIONS[879].3 };
pub const REPURCHASE_TERM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[880].0, tag: FIELD_DEFINITIONS[880].1, data_type: FIELD_DEFINITIONS[880].2, location: FIELD_DEFINITIONS[880].3 };
pub const PARTY_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[881].0, tag: FIELD_DEFINITIONS[881].1, data_type: FIELD_DEFINITIONS[881].2, location: FIELD_DEFINITIONS[881].3 };
pub const NESTED3_PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[882].0, tag: FIELD_DEFINITIONS[882].1, data_type: FIELD_DEFINITIONS[882].2, location: FIELD_DEFINITIONS[882].3 };
pub const PARTY_SUB_ID_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[883].0, tag: FIELD_DEFINITIONS[883].1, data_type: FIELD_DEFINITIONS[883].2, location: FIELD_DEFINITIONS[883].3 };
pub const UNDERLYING_LOCALE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[884].0, tag: FIELD_DEFINITIONS[884].1, data_type: FIELD_DEFINITIONS[884].2, location: FIELD_DEFINITIONS[884].3 };
pub const TRADE_ORIGINATION_DATE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[885].0, tag: FIELD_DEFINITIONS[885].1, data_type: FIELD_DEFINITIONS[885].2, location: FIELD_DEFINITIONS[885].3 };
pub const ENCODED_LIST_STATUS_TEXT_LEN: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[886].0, tag: FIELD_DEFINITIONS[886].1, data_type: FIELD_DEFINITIONS[886].2, location: FIELD_DEFINITIONS[886].3 };
pub const PARTY_ROLE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[887].0, tag: FIELD_DEFINITIONS[887].1, data_type: FIELD_DEFINITIONS[887].2, location: FIELD_DEFINITIONS[887].3 };
pub const LEG_STATE_OR_PROVINCE_OF_ISSUE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[888].0, tag: FIELD_DEFINITIONS[888].1, data_type: FIELD_DEFINITIONS[888].2, location: FIELD_DEFINITIONS[888].3 };
pub const SETTL_CURR_FX_RATE_CALC: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[889].0, tag: FIELD_DEFINITIONS[889].1, data_type: FIELD_DEFINITIONS[889].2, location: FIELD_DEFINITIONS[889].3 };
pub const UNDERLYING_CP_PROGRAM: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[890].0, tag: FIELD_DEFINITIONS[890].1, data_type: FIELD_DEFINITIONS[890].2, location: FIELD_DEFINITIONS[890].3 };
pub const EVENT_TEXT: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[891].0, tag: FIELD_DEFINITIONS[891].1, data_type: FIELD_DEFINITIONS[891].2, location: FIELD_DEFINITIONS[891].3 };
pub const LEAVES_QTY: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[892].0, tag: FIELD_DEFINITIONS[892].1, data_type: FIELD_DEFINITIONS[892].2, location: FIELD_DEFINITIONS[892].3 };
pub const TRADE_REPORT_REF_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[893].0, tag: FIELD_DEFINITIONS[893].1, data_type: FIELD_DEFINITIONS[893].2, location: FIELD_DEFINITIONS[893].3 };
pub const LIST_ORDER_STATUS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[894].0, tag: FIELD_DEFINITIONS[894].1, data_type: FIELD_DEFINITIONS[894].2, location: FIELD_DEFINITIONS[894].3 };
pub const CASH_DISTRIB_AGENT_ACCT_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[895].0, tag: FIELD_DEFINITIONS[895].1, data_type: FIELD_DEFINITIONS[895].2, location: FIELD_DEFINITIONS[895].3 };
pub const SENDER_LOCATION_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[896].0, tag: FIELD_DEFINITIONS[896].1, data_type: FIELD_DEFINITIONS[896].2, location: FIELD_DEFINITIONS[896].3 };
pub const TRAD_SES_STATUS_REJ_REASON: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[897].0, tag: FIELD_DEFINITIONS[897].1, data_type: FIELD_DEFINITIONS[897].2, location: FIELD_DEFINITIONS[897].3 };
pub const LAST_NETWORK_RESPONSE_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[898].0, tag: FIELD_DEFINITIONS[898].1, data_type: FIELD_DEFINITIONS[898].2, location: FIELD_DEFINITIONS[898].3 };
pub const PRIORITY_INDICATOR: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[899].0, tag: FIELD_DEFINITIONS[899].1, data_type: FIELD_DEFINITIONS[899].2, location: FIELD_DEFINITIONS[899].3 };
pub const DELIVER_TO_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[900].0, tag: FIELD_DEFINITIONS[900].1, data_type: FIELD_DEFINITIONS[900].2, location: FIELD_DEFINITIONS[900].3 };
pub const TRAD_SES_END_TIME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[901].0, tag: FIELD_DEFINITIONS[901].1, data_type: FIELD_DEFINITIONS[901].2, location: FIELD_DEFINITIONS[901].3 };
pub const POS_MAINT_ACTION: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[902].0, tag: FIELD_DEFINITIONS[902].1, data_type: FIELD_DEFINITIONS[902].2, location: FIELD_DEFINITIONS[902].3 };
pub const CANCELLATION_RIGHTS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[903].0, tag: FIELD_DEFINITIONS[903].1, data_type: FIELD_DEFINITIONS[903].2, location: FIELD_DEFINITIONS[903].3 };
pub const TRADING_SESSION_SUB_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[904].0, tag: FIELD_DEFINITIONS[904].1, data_type: FIELD_DEFINITIONS[904].2, location: FIELD_DEFINITIONS[904].3 };
pub const SETTL_INST_MSG_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[905].0, tag: FIELD_DEFINITIONS[905].1, data_type: FIELD_DEFINITIONS[905].2, location: FIELD_DEFINITIONS[905].3 };
pub const POS_AMT_TYPE: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[906].0, tag: FIELD_DEFINITIONS[906].1, data_type: FIELD_DEFINITIONS[906].2, location: FIELD_DEFINITIONS[906].3 };
pub const USER_REQUEST_ID: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[907].0, tag: FIELD_DEFINITIONS[907].1, data_type: FIELD_DEFINITIONS[907].2, location: FIELD_DEFINITIONS[907].3 };
pub const NO_TRD_REG_TIMESTAMPS: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[908].0, tag: FIELD_DEFINITIONS[908].1, data_type: FIELD_DEFINITIONS[908].2, location: FIELD_DEFINITIONS[908].3 };
pub const SELL_VOLUME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[909].0, tag: FIELD_DEFINITIONS[909].1, data_type: FIELD_DEFINITIONS[909].2, location: FIELD_DEFINITIONS[909].3 };
pub const LIQUIDITY_PCT_LOW: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[910].0, tag: FIELD_DEFINITIONS[910].1, data_type: FIELD_DEFINITIONS[910].2, location: FIELD_DEFINITIONS[910].3 };
pub const STAND_INST_DB_NAME: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition { name: FIELD_DEFINITIONS[911].0, tag: FIELD_DEFINITIONS[911].1, data_type: FIELD_DEFINITIONS[911].2, location: FIELD_DEFINITIONS[911].3 };