mod model;
use model::StockQuote;
use std::{error::Error, fmt};

/// The Error that may occured when requesting a [StockQuote]
#[derive(Debug)]
pub enum StockQuoteError {
    /// Returned when something wrong happen during the HTTP request
    /// it's mostly things that are out of control, like network issue
    /// or the server did not response.
    NetworkError(String),
    /// Returned when something wrong happen during the parsing of the
    /// stock data, for example, missing or unknown data field appeared.
    DataError(String),
}

impl fmt::Display for StockQuoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockQuoteError::NetworkError(err) => write!(f, "NetworkError: {}", err),
            StockQuoteError::DataError(err) => write!(f, "DataError: {}", err),
        }
    }
}

impl From<reqwest::Error> for StockQuoteError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_decode() {
            StockQuoteError::DataError(error.to_string())
        } else {
            StockQuoteError::NetworkError(error.to_string())
        }
    }
}

impl Error for StockQuoteError { }

fn get_url(quote: &str) -> String {
    ["https://query2.finance.yahoo.com/v10/finance/quoteSummary/", quote, "?modules=assetProfile,balanceSheetHistory,balanceSheetHistoryQuarterly,calendarEvents,cashflowStatementHistory,cashflowStatementHistoryQuarterly,defaultKeyStatistics,earnings,earningsHistory,earningsTrend,financialData,fundOwnership,incomeStatementHistory,incomeStatementHistoryQuarterly,indexTrend,industryTrend,insiderHolders,insiderTransactions,institutionOwnership,majorDirectHolders,majorHoldersBreakdown,netSharePurchaseActivity,price,quoteType,recommendationTrend,secFilings,sectorTrend,summaryDetail,summaryProfile,symbol,upgradeDowngradeHistory,fundProfile,topHoldings,fundPerformance"].join("")
}

/// Get Stock Quote
///
/// Get the quote data of a stock. It returns a [StockQuote] object.
/// In case of error, a [StockQuoteError] object will be returned.
///
/// # Examples
///
/// ```
/// # async fn run() -> Result<(), reqwest::Error> {
///   let result = stockquote::get("IBM").await;
/// # Ok(())
/// # }
/// ```
pub async fn get(quote: &str) -> Result<StockQuote, StockQuoteError> {
    let result = reqwest::get(get_url(quote).as_str())
        .await?
        .json::<StockQuote>()
        .await?;
    Ok(result)
}
