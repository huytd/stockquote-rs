#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
    pub quote_summary: QuoteSummary,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSummary {
    pub result: Vec<Result>,
    pub error: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub asset_profile: Option<AssetProfile>,
    pub fund_profile: Option<FundProfile>,
    pub summary_detail: Option<SummaryDetail>,
    pub price: Option<Price>,
    pub default_key_statistics: Option<DefaultKeyStatistics>,
    pub summary_profile: Option<SummaryProfile>,
    pub top_holdings: Option<TopHoldings>,
    pub fund_performance: Option<FundPerformance>,
    pub quote_type: Option<QuoteType>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetProfile {
    pub phone: Option<String>,
    pub long_business_summary: Option<String>,
    pub company_officers: Vec<::serde_json::Value>,
    pub max_age: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundProfile {
    pub max_age: Option<i64>,
    pub style_box_url: Option<String>,
    pub family: Option<String>,
    pub category_name: Option<String>,
    pub legal_type: Option<String>,
    pub management_info: ManagementInfo,
    pub fees_expenses_investment: FeesExpensesInvestment,
    pub fees_expenses_investment_cat: FeesExpensesInvestmentCat,
    pub init_investment: InitInvestment,
    pub init_ira_investment: InitIraInvestment,
    pub init_aip_investment: InitAipInvestment,
    pub subseq_investment: SubseqInvestment,
    pub subseq_ira_investment: SubseqIraInvestment,
    pub subseq_aip_investment: SubseqAipInvestment,
    pub brokerages: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementInfo {
    pub manager_name: ::serde_json::Value,
    pub manager_bio: ::serde_json::Value,
    pub startdate: Startdate,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Startdate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeesExpensesInvestment {
    pub annual_report_expense_ratio: AnnualReportExpenseRatio,
    pub front_end_sales_load: FrontEndSalesLoad,
    pub deferred_sales_load: DeferredSalesLoad,
    #[serde(rename = "twelveBOne")]
    pub twelve_bone: TwelveBone,
    pub net_exp_ratio: NetExpRatio,
    pub gross_exp_ratio: GrossExpRatio,
    pub annual_holdings_turnover: AnnualHoldingsTurnover,
    pub total_net_assets: TotalNetAssets,
    pub projection_values: ProjectionValues,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReportExpenseRatio {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndSalesLoad {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeferredSalesLoad {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwelveBone {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetExpRatio {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrossExpRatio {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualHoldingsTurnover {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalNetAssets {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionValues {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeesExpensesInvestmentCat {
    pub annual_report_expense_ratio: AnnualReportExpenseRatio2,
    pub front_end_sales_load: FrontEndSalesLoad2,
    pub deferred_sales_load: DeferredSalesLoad2,
    #[serde(rename = "twelveBOne")]
    pub twelve_bone: TwelveBone2,
    pub annual_holdings_turnover: AnnualHoldingsTurnover2,
    pub total_net_assets: TotalNetAssets2,
    pub projection_values_cat: ProjectionValuesCat,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReportExpenseRatio2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndSalesLoad2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeferredSalesLoad2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwelveBone2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualHoldingsTurnover2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalNetAssets2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionValuesCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitIraInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAipInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubseqInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubseqIraInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubseqAipInvestment {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryDetail {
    pub max_age: Option<i64>,
    pub price_hint: PriceHint,
    pub previous_close: PreviousClose,
    pub open: Open,
    pub day_low: DayLow,
    pub day_high: DayHigh,
    pub regular_market_previous_close: RegularMarketPreviousClose,
    pub regular_market_open: RegularMarketOpen,
    pub regular_market_day_low: RegularMarketDayLow,
    pub regular_market_day_high: RegularMarketDayHigh,
    pub dividend_rate: DividendRate,
    pub dividend_yield: DividendYield,
    pub ex_dividend_date: ExDividendDate,
    pub payout_ratio: PayoutRatio,
    pub five_year_avg_dividend_yield: FiveYearAvgDividendYield,
    pub beta: Beta,
    #[serde(rename = "forwardPE")]
    pub forward_pe: ForwardPe,
    pub volume: Volume,
    pub regular_market_volume: RegularMarketVolume,
    pub average_volume: AverageVolume,
    #[serde(rename = "averageVolume10days")]
    pub average_volume10_days: AverageVolume10Days,
    pub average_daily_volume10_day: AverageDailyVolume10Day,
    pub bid: Bid,
    pub ask: Ask,
    pub bid_size: BidSize,
    pub ask_size: AskSize,
    pub market_cap: MarketCap,
    #[serde(rename = "yield")]
    pub yield_field: Yield,
    pub ytd_return: YtdReturn,
    pub total_assets: TotalAssets,
    pub expire_date: ExpireDate,
    pub strike_price: StrikePrice,
    pub open_interest: OpenInterest,
    pub fifty_two_week_low: FiftyTwoWeekLow,
    pub fifty_two_week_high: FiftyTwoWeekHigh,
    pub price_to_sales_trailing12_months: PriceToSalesTrailing12Months,
    pub fifty_day_average: FiftyDayAverage,
    pub two_hundred_day_average: TwoHundredDayAverage,
    pub trailing_annual_dividend_rate: TrailingAnnualDividendRate,
    pub trailing_annual_dividend_yield: TrailingAnnualDividendYield,
    pub nav_price: NavPrice,
    pub currency: Option<String>,
    pub from_currency: ::serde_json::Value,
    pub to_currency: ::serde_json::Value,
    pub last_market: ::serde_json::Value,
    pub volume24_hr: Volume24Hr,
    pub volume_all_currencies: VolumeAllCurrencies,
    pub circulating_supply: CirculatingSupply,
    pub algorithm: ::serde_json::Value,
    pub max_supply: MaxSupply,
    pub start_date: StartDate,
    pub tradeable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHint {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousClose {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Open {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayLow {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayHigh {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketPreviousClose {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketOpen {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketDayLow {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketDayHigh {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendRate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendYield {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExDividendDate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRatio {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYearAvgDividendYield {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardPe {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketVolume {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageVolume {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageVolume10Days {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageDailyVolume10Day {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidSize {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskSize {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCap {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yield {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdReturn {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalAssets {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpireDate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrikePrice {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiftyTwoWeekLow {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiftyTwoWeekHigh {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToSalesTrailing12Months {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiftyDayAverage {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoHundredDayAverage {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingAnnualDividendRate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingAnnualDividendYield {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavPrice {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume24Hr {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAllCurrencies {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CirculatingSupply {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxSupply {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub max_age: Option<i64>,
    pub pre_market_change: PreMarketChange,
    pub pre_market_price: PreMarketPrice,
    pub pre_market_source: Option<String>,
    pub post_market_change: PostMarketChange,
    pub post_market_price: PostMarketPrice,
    pub regular_market_change_percent: RegularMarketChangePercent,
    pub regular_market_change: RegularMarketChange,
    pub regular_market_time: Option<i64>,
    pub price_hint: PriceHint2,
    pub regular_market_price: RegularMarketPrice,
    pub regular_market_day_high: RegularMarketDayHigh2,
    pub regular_market_day_low: RegularMarketDayLow2,
    pub regular_market_volume: RegularMarketVolume2,
    pub average_daily_volume10_day: AverageDailyVolume10Day2,
    pub average_daily_volume3_month: AverageDailyVolume3Month,
    pub regular_market_previous_close: RegularMarketPreviousClose2,
    pub regular_market_source: Option<String>,
    pub regular_market_open: RegularMarketOpen2,
    pub strike_price: StrikePrice2,
    pub open_interest: OpenInterest2,
    pub exchange: Option<String>,
    pub exchange_name: Option<String>,
    pub exchange_data_delayed_by: Option<i64>,
    pub market_state: Option<String>,
    pub quote_type: Option<String>,
    pub symbol: Option<String>,
    pub underlying_symbol: ::serde_json::Value,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub currency: Option<String>,
    pub quote_source_name: Option<String>,
    pub currency_symbol: Option<String>,
    pub from_currency: ::serde_json::Value,
    pub to_currency: ::serde_json::Value,
    pub last_market: ::serde_json::Value,
    pub volume24_hr: Volume24Hr2,
    pub volume_all_currencies: VolumeAllCurrencies2,
    pub circulating_supply: CirculatingSupply2,
    pub market_cap: MarketCap2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreMarketChange {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreMarketPrice {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMarketChange {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMarketPrice {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketChangePercent {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketChange {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHint2 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketPrice {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketDayHigh2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketDayLow2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketVolume2 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageDailyVolume10Day2 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AverageDailyVolume3Month {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketPreviousClose2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarketOpen2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrikePrice2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume24Hr2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAllCurrencies2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CirculatingSupply2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCap2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultKeyStatistics {
    pub max_age: Option<i64>,
    pub price_hint: PriceHint3,
    pub enterprise_value: EnterpriseValue,
    #[serde(rename = "forwardPE")]
    pub forward_pe: ForwardPe2,
    pub profit_margins: ProfitMargins,
    pub morning_star_overall_rating: MorningStarOverallRating,
    pub morning_star_risk_rating: MorningStarRiskRating,
    pub category: Option<String>,
    pub book_value: BookValue,
    pub price_to_book: PriceToBook,
    pub annual_report_expense_ratio: AnnualReportExpenseRatio3,
    pub ytd_return: YtdReturn2,
    pub beta3_year: Beta3Year,
    pub total_assets: TotalAssets2,
    #[serde(rename = "yield")]
    pub yield_field: Yield2,
    pub fund_family: Option<String>,
    pub fund_inception_date: FundInceptionDate,
    pub legal_type: Option<String>,
    pub three_year_average_return: ThreeYearAverageReturn,
    pub five_year_average_return: FiveYearAverageReturn,
    pub price_to_sales_trailing12_months: PriceToSalesTrailing12Months2,
    pub last_fiscal_year_end: LastFiscalYearEnd,
    pub next_fiscal_year_end: NextFiscalYearEnd,
    pub most_recent_quarter: MostRecentQuarter,
    pub earnings_quarterly_growth: EarningsQuarterlyGrowth,
    pub revenue_quarterly_growth: RevenueQuarterlyGrowth,
    pub net_income_to_common: NetIncomeToCommon,
    pub trailing_eps: TrailingEps,
    pub forward_eps: ForwardEps,
    pub peg_ratio: PegRatio,
    pub last_split_factor: ::serde_json::Value,
    pub last_split_date: LastSplitDate,
    pub enterprise_to_revenue: EnterpriseToRevenue,
    pub enterprise_to_ebitda: EnterpriseToEbitda,
    #[serde(rename = "52WeekChange")]
    pub n52_week_change: N52WeekChange,
    #[serde(rename = "SandP52WeekChange")]
    pub sand_p52_week_change: SandP52WeekChange,
    pub last_dividend_value: LastDividendValue,
    pub last_dividend_date: LastDividendDate,
    pub last_cap_gain: LastCapGain,
    pub annual_holdings_turnover: AnnualHoldingsTurnover3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHint3 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseValue {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardPe2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitMargins {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorningStarOverallRating {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorningStarRiskRating {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookValue {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToBook {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReportExpenseRatio3 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdReturn2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta3Year {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalAssets2 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yield2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundInceptionDate {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYearAverageReturn {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYearAverageReturn {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToSalesTrailing12Months2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastFiscalYearEnd {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextFiscalYearEnd {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostRecentQuarter {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsQuarterlyGrowth {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueQuarterlyGrowth {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetIncomeToCommon {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingEps {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardEps {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PegRatio {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastSplitDate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseToRevenue {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseToEbitda {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N52WeekChange {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandP52WeekChange {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastDividendValue {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastDividendDate {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastCapGain {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualHoldingsTurnover3 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryProfile {
    pub phone: Option<String>,
    pub long_business_summary: Option<String>,
    pub company_officers: Vec<::serde_json::Value>,
    pub max_age: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopHoldings {
    pub max_age: Option<i64>,
    pub cash_position: CashPosition,
    pub stock_position: StockPosition,
    pub bond_position: BondPosition,
    pub other_position: OtherPosition,
    pub preferred_position: PreferredPosition,
    pub convertible_position: ConvertiblePosition,
    pub holdings: Vec<Holding>,
    pub equity_holdings: EquityHoldings,
    pub bond_holdings: BondHoldings,
    pub bond_ratings: Vec<BondRating>,
    pub sector_weightings: Vec<SectorWeighting>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashPosition {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPosition {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondPosition {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherPosition {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferredPosition {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertiblePosition {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holding {
    pub symbol: Option<String>,
    pub holding_name: Option<String>,
    pub holding_percent: HoldingPercent,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingPercent {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityHoldings {
    pub price_to_earnings: PriceToEarnings,
    pub price_to_book: PriceToBook2,
    pub price_to_sales: PriceToSales,
    pub price_to_cashflow: PriceToCashflow,
    pub median_market_cap: MedianMarketCap,
    pub three_year_earnings_growth: ThreeYearEarningsGrowth,
    pub price_to_earnings_cat: PriceToEarningsCat,
    pub price_to_book_cat: PriceToBookCat,
    pub price_to_sales_cat: PriceToSalesCat,
    pub price_to_cashflow_cat: PriceToCashflowCat,
    pub median_market_cap_cat: MedianMarketCapCat,
    pub three_year_earnings_growth_cat: ThreeYearEarningsGrowthCat,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToEarnings {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToBook2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToSales {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToCashflow {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedianMarketCap {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYearEarningsGrowth {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToEarningsCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToBookCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToSalesCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceToCashflowCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedianMarketCapCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYearEarningsGrowthCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondHoldings {
    pub maturity: Maturity,
    pub duration: Duration,
    pub credit_quality: CreditQuality,
    pub maturity_cat: MaturityCat,
    pub duration_cat: DurationCat,
    pub credit_quality_cat: CreditQualityCat,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maturity {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditQuality {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaturityCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurationCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditQualityCat {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondRating {
    pub bb: Option<Bb>,
    pub aa: Option<Aa>,
    pub aaa: Option<Aaa>,
    pub a: Option<A>,
    pub other: Option<Other>,
    pub b: Option<B>,
    pub bbb: Option<Bbb>,
    #[serde(rename = "below_b")]
    pub below_b: Option<BelowB>,
    #[serde(rename = "us_government")]
    pub us_government: Option<UsGovernment>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bb {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aa {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aaa {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct B {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bbb {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BelowB {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsGovernment {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorWeighting {
    pub realestate: Option<Realestate>,
    #[serde(rename = "consumer_cyclical")]
    pub consumer_cyclical: Option<ConsumerCyclical>,
    #[serde(rename = "basic_materials")]
    pub basic_materials: Option<BasicMaterials>,
    #[serde(rename = "consumer_defensive")]
    pub consumer_defensive: Option<ConsumerDefensive>,
    pub technology: Option<Technology>,
    #[serde(rename = "communication_services")]
    pub communication_services: Option<CommunicationServices>,
    #[serde(rename = "financial_services")]
    pub financial_services: Option<FinancialServices>,
    pub utilities: Option<Utilities>,
    pub industrials: Option<Industrials>,
    pub energy: Option<Energy>,
    pub healthcare: Option<Healthcare>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realestate {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerCyclical {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicMaterials {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerDefensive {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationServices {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialServices {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Utilities {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Industrials {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Energy {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healthcare {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundPerformance {
    pub max_age: Option<i64>,
    pub performance_overview: PerformanceOverview,
    pub performance_overview_cat: PerformanceOverviewCat,
    pub trailing_returns: TrailingReturns,
    pub trailing_returns_nav: TrailingReturnsNav,
    pub trailing_returns_cat: TrailingReturnsCat,
    pub annual_total_returns: AnnualTotalReturns,
    pub past_quarterly_returns: PastQuarterlyReturns,
    pub risk_overview_statistics: RiskOverviewStatistics,
    pub risk_overview_statistics_cat: RiskOverviewStatisticsCat,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceOverview {
    pub as_of_date: AsOfDate,
    pub morning_star_return_rating: MorningStarReturnRating,
    pub ytd_return_pct: YtdReturnPct,
    pub five_yr_avg_return_pct: FiveYrAvgReturnPct,
    pub num_years_up: NumYearsUp,
    pub num_years_down: NumYearsDown,
    pub best_one_yr_total_return: BestOneYrTotalReturn,
    pub worst_one_yr_total_return: WorstOneYrTotalReturn,
    pub best_three_yr_total_return: BestThreeYrTotalReturn,
    pub worst_three_yr_total_return: WorstThreeYrTotalReturn,
    pub one_year_total_return: OneYearTotalReturn,
    pub three_year_total_return: ThreeYearTotalReturn,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsOfDate {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorningStarReturnRating {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdReturnPct {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYrAvgReturnPct {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumYearsUp {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumYearsDown {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestOneYrTotalReturn {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorstOneYrTotalReturn {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestThreeYrTotalReturn {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorstThreeYrTotalReturn {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneYearTotalReturn {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYearTotalReturn {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceOverviewCat {
    pub as_of_date: AsOfDate2,
    pub morning_star_return_rating: MorningStarReturnRating2,
    pub ytd_return_pct: YtdReturnPct2,
    pub five_yr_avg_return_pct: FiveYrAvgReturnPct2,
    pub num_years_up: NumYearsUp2,
    pub num_years_down: NumYearsDown2,
    pub best_one_yr_total_return: BestOneYrTotalReturn2,
    pub worst_one_yr_total_return: WorstOneYrTotalReturn2,
    pub best_three_yr_total_return: BestThreeYrTotalReturn2,
    pub worst_three_yr_total_return: WorstThreeYrTotalReturn2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsOfDate2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorningStarReturnRating2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdReturnPct2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYrAvgReturnPct2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumYearsUp2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumYearsDown2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestOneYrTotalReturn2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorstOneYrTotalReturn2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestThreeYrTotalReturn2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorstThreeYrTotalReturn2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingReturns {
    pub as_of_date: AsOfDate3,
    pub ytd: Ytd,
    pub one_month: OneMonth,
    pub three_month: ThreeMonth,
    pub one_year: OneYear,
    pub three_year: ThreeYear,
    pub five_year: FiveYear,
    pub ten_year: TenYear,
    pub last_bull_mkt: LastBullMkt,
    pub last_bear_mkt: LastBearMkt,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsOfDate3 {
    pub raw: Option<i64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ytd {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneMonth {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeMonth {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneYear {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYear {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYear {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenYear {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastBullMkt {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastBearMkt {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingReturnsNav {
    pub ytd: Ytd2,
    pub one_month: OneMonth2,
    pub three_month: ThreeMonth2,
    pub one_year: OneYear2,
    pub three_year: ThreeYear2,
    pub five_year: FiveYear2,
    pub ten_year: TenYear2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ytd2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneMonth2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeMonth2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneYear2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYear2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYear2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenYear2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingReturnsCat {
    pub ytd: Ytd3,
    pub one_month: OneMonth3,
    pub three_month: ThreeMonth3,
    pub one_year: OneYear3,
    pub three_year: ThreeYear3,
    pub five_year: FiveYear3,
    pub ten_year: TenYear3,
    pub last_bull_mkt: LastBullMkt2,
    pub last_bear_mkt: LastBearMkt2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ytd3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneMonth3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeMonth3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneYear3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeYear3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiveYear3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenYear3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastBullMkt2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastBearMkt2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualTotalReturns {
    pub returns: Vec<Return>,
    pub returns_cat: Vec<ReturnsCat>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub year: Option<String>,
    pub annual_value: AnnualValue,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualValue {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnsCat {
    pub year: Option<String>,
    pub annual_value: AnnualValue2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualValue2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastQuarterlyReturns {
    pub returns: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskOverviewStatistics {
    pub risk_rating: RiskRating,
    pub risk_statistics: Vec<RiskStatistic>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRating {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskStatistic {
    pub year: Option<String>,
    pub alpha: Alpha,
    pub beta: Beta2,
    pub mean_annual_return: MeanAnnualReturn,
    pub r_squared: RSquared,
    pub std_dev: StdDev,
    pub sharpe_ratio: SharpeRatio,
    pub treynor_ratio: TreynorRatio,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alpha {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeanAnnualReturn {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RSquared {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StdDev {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharpeRatio {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreynorRatio {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskOverviewStatisticsCat {
    pub risk_statistics_cat: Vec<RiskStatisticsCat>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskStatisticsCat {
    pub year: Option<String>,
    pub alpha: Alpha2,
    pub beta: Beta3,
    pub mean_annual_return: MeanAnnualReturn2,
    pub r_squared: RSquared2,
    pub std_dev: StdDev2,
    pub sharpe_ratio: SharpeRatio2,
    pub treynor_ratio: TreynorRatio2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alpha2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta3 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeanAnnualReturn2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RSquared2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StdDev2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharpeRatio2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreynorRatio2 {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteType {
    pub exchange: Option<String>,
    pub quote_type: Option<String>,
    pub symbol: Option<String>,
    pub underlying_symbol: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub first_trade_date_epoch_utc: Option<i64>,
    pub time_zone_full_name: Option<String>,
    pub time_zone_short_name: Option<String>,
    pub uuid: Option<String>,
    pub message_board_id: Option<String>,
    pub gmt_off_set_milliseconds: Option<i64>,
    pub max_age: Option<i64>,
}