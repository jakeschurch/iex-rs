#[macro_use]
extern crate serde_derive;
extern crate byteorder;
extern crate chrono;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use failure::Error;
use serde_json::Value;
use std::borrow::Borrow;
use std::result;
use url::Url;

pub mod iextp;
pub mod types;

pub use self::types::*;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    FiveYears,
    TwoYears,
    OneYear,
    YearToDate,
    SixMonths,
    ThreeMonths,
    OneMonth,
    OneDay,
    // TODO: Date returns a different JSON structure to the rest of the duration parameters. We'll
    // need to make a different function to support it.
    // REVIEW(Duration::Date): This should suffice - but as of now, no way to validate input format.
    Date(&'static str),
    Dynamic,
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        match self {
            Duration::FiveYears => String::from("5y"),
            Duration::TwoYears => String::from("2y"),
            Duration::OneYear => String::from("1y"),
            Duration::YearToDate => String::from("ytd"),
            Duration::SixMonths => String::from("6m"),
            Duration::ThreeMonths => String::from("3m"),
            Duration::OneMonth => String::from("1m"),
            Duration::OneDay => String::from("1d"),
            Duration::Date(date) => format!("date/{}", date),
            Duration::Dynamic => String::from("dynamic"),
        }
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::OneMonth
    }
}

#[derive(PartialEq, Eq)]
/// The `Request` enum type allows for HTTP request matching to the IEX API.
// TODO(Request): Add documentation from IEX website.
pub enum Request<'a> {
    Book {
        symbol: &'a str,
    },
    // REVIEW(CHART): Should `time-series` be implemented even though it forwards
    // towards the same endpoint as `Chart`?
    Chart {
        symbol: &'a str,
        duration: Duration,
        params: Option<Vec<ChartParam>>,
    },
    Company {
        symbol: &'a str,
    },
    DelayedQuote {
        symbol: &'a str,
    },
    Dividends {
        symbol: &'a str,
        duration: Duration,
    },
    Earnings {
        symbol: &'a str,
    },
    EffectiveSpread {
        symbol: &'a str,
    },
    Financials {
        symbol: &'a str,
    },
    // REVIEW|TODO(List): add in default displayParameters(?)
    List {
        param: ListParam,
    },
    Logo {
        symbol: &'a str,
    },
    News {
        symbol: &'a str,
        range: Option<i32>,
    },
    Ohlc {
        symbol: &'a str,
    },
    Peers {
        symbol: &'a str,
    },
    Previous {
        symbol: &'a str,
    },
    Price {
        symbol: &'a str,
    },
    Quote {
        symbol: &'a str,
    },
    Relevant {
        symbol: &'a str,
    },
    Splits {
        symbol: &'a str,
        duration: Duration,
    },
    Stats {
        symbol: &'a str,
    },
    Symbols,
    /// IEX Regulation SHO Threshold Securities List
    ThresholdSecurities {
        // REVIEW: may be a good idea to implement a date struct to use here and
        // wrap in the Duration::Date enum variant, or eq - a match statement.
        date: Option<Duration>,
    },
    // TODO(ShortInterest): implement variant.
    VolumeByVenue {
        symbol: &'a str,
    },
}

// IDEA: return vec<str> where index 0 is uri endpoint, rest are params.
impl<'a> ToString for Request<'a> {
    fn to_string(&self) -> String {
        match self {
            Request::Book { symbol } => format!("stock/{}/book", symbol),
            Request::Chart {
                symbol, duration, ..
            } => format!(
                "stock/{}/chart/{}",
                symbol,
                duration.to_string(),
                // parse_params(params)
            ),
            Request::Company { symbol } => format!("stock/{}/company", symbol),
            Request::DelayedQuote { symbol } => format!("stock/{}/delayed-quote", symbol),
            Request::Dividends { symbol, duration } => {
                format!("stock/{}/dividends/{}", symbol, duration.to_string())
            }
            Request::Earnings { symbol } => format!("stock/{}/earnings", symbol),
            Request::EffectiveSpread { symbol } => format!("stock/{}/effective-spread", symbol),
            Request::Financials { symbol } => format!("stock/{}/financials", symbol),
            Request::List { param } => format!("stock/market/list/{}", param.to_string()),
            Request::Logo { symbol } => format!("stock/{}/logo", symbol),
            Request::News { symbol, range } => format!(
                "stock/{}/news/last/{}",
                symbol,
                range.map(|r| r.to_string()).unwrap_or("".to_string())
            ),
            Request::Ohlc { symbol } => format!("stock/{}/ohlc", symbol),
            Request::Peers { symbol } => format!("stock/{}/peers", symbol),
            // TODO(Request::Previous) It's possible to pass in "market" as an argument here
            // and get one entry for each symbol. We need to handle that
            // scenario.
            // REVIEW(Request::Previous): Regarding ^: should be taken care of
            // due to change in Client's request method signature to serve_json::Value. Let me know if otherwise.
            Request::Previous { symbol } => format!("stock/{}/previous", symbol),
            Request::Price { symbol } => format!("stock/{}/price", symbol),
            Request::Quote { symbol } => format!("stock/{}/quote", symbol),
            Request::Relevant { symbol } => format!("stock/{}/relevant", symbol),
            Request::Stats { symbol } => format!("stock/{}/stats", symbol),
            Request::Splits { symbol, duration } => {
                format!("stock/{}/splits/{}", symbol, duration.to_string())
            }
            Request::Symbols => String::from("/ref-data/symbols"),
            Request::ThresholdSecurities { date } => format!(
                "stock/market/threshold-securities/{}",
                date.unwrap_or(Duration::Date("")).to_string()
            ),
            Request::VolumeByVenue { symbol } => format!("stock/{}/volume-by-venue", symbol),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response(pub Value);

impl Response {
    pub fn try_into<T>(self) -> Result<T>
    // TEMP(Response): Keep until try_from trait becomes stable rust feature.
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        Ok(serde_json::from_value(self.0)?)
    }
}

/// `Client` acts as a Handler for the `Response` enum.
pub struct Client {
    _types: Vec<String>,
    _symbols: Vec<&'static str>,
    // _params:
}

impl Client {
    pub fn new() -> Self {
        Client {
            _types: vec![],
            _symbols: vec![],
        }
    }

    #[must_use]
    pub fn types(&mut self, types: Vec<Request<'static>>) -> () {
        self._types = types.iter().map(|val| val.to_string()).collect()
    }

    pub fn symbols(&mut self, symbols: Vec<&'static str>) -> () {
        self._symbols = symbols;
    }

    fn build_url(&self) {
        // let symbols = &self._symbols;

        let symbol_path: String = if self._symbols.iter().count() == 1 {
            self._symbols.join(",")
        } else {
            String::from("market")
        };

        /// `IEX_STOCK` is the URL base of IEX Stocks API.
        const IEX_STOCK: &'static str = "https://api.iextrading.com/1.0/stock";

        let url_str = format!(
            "{base_url}/stock/market/batch?symbols={symbols}&types={types}",
            base_url = IEX_STOCK,
            symbols = symbol_path,
            types = self._types.join(",")
        );

        let options = Url::options();

        let api = Url::parse(&url_str).unwrap();
    }

    // #[must_use]
    // pub fn request(&self) {
    //     // unimplemented!();

    //     let types_tup = &self._types;
    //     let symbols_tup = match &self._symbols {
    //         Some(symbols_tup) => symbols_tup.clone(),
    //         None => ("types", vec![]),
    //     };

    //     let n_symbols = symbols_tup.1.iter().count();
    //     // let is_batch: bool = types_tup.1.iter().count() > 1 || symbols_tup.1.iter().count() > 1;

    //     let symbol_req = |n| -> &str {
    //         if n == 1 {
    //             return symbols_tup.1[0];
    //         } else {
    //             return "market";
    //         }
    //     };

    //     let request_url = format!(
    //         "https://api.iextrading.com/1.0/stock/{symbol}/batch",
    //         symbol = symbol_req(n_symbols)
    //     );

    //     let url: Url;

    //     let url_str: &str = if n_symbols > 1 {
    //         Url::parse_with_params(&request_url, symbols_tup)
    //             .unwrap()
    //             .as_str()
    //     } else {
    //         request_url
    //     };

    //     // let url = format!(
    //     //     "{base}/{endpoint}",
    //     //     base = IEX_BASE,
    //     //     endpoint = req.to_string()
    //     // );

    //     // Ok(reqwest::get(&url)?.json()?);
    // }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ListParam {
    MostActive,
    Gainers,
    Losers,
    IexVolume,
    IexPercent,
}

impl ListParam {
    fn to_string(&self) -> String {
        match self {
            ListParam::MostActive => String::from("mostactive"),
            ListParam::Gainers => String::from("gainers"),
            ListParam::Losers => String::from("losers"),
            ListParam::IexVolume => String::from("iexvolume"),
            ListParam::IexPercent => String::from("iexpercent"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ChartParam {
    /// If true, 1d chart will reset at midnight instead of the default behavior of 9:30am ET.
    Reset(bool),
    /// If true, runs a polyline simplification using the Douglas-Peucker algorithm. This is useful if plotting sparkline charts.
    Simplify(bool),
    /// If passed, chart data will return every Nth element as defined by `Interval`.
    Interval(usize),
    /// If true, changeOverTime and marketChangeOverTime will be relative to previous day close instead of the first value.
    ChangeFromClose(bool),
    /// If passed, chart data will return the last N elements.
    Last(usize),
}

impl ChartParam {
    fn to_str(&self) -> String {
        match self {
            ChartParam::Reset(val)
            | ChartParam::Simplify(val)
            | ChartParam::ChangeFromClose(val) => {
                if *val == true {
                    return String::from("true");
                } else {
                    return String::from("false");
                }
            }
            ChartParam::Interval(val) | ChartParam::Last(val) => val.to_string(),
        }
    }

    fn to_pair(&self) -> (&str, String) {
        let first_arg: &str = match self {
            ChartParam::Reset(_res) => "chartReset",
            ChartParam::Simplify(_res) => "chartSimplify",
            ChartParam::Interval(_res) => "chartInterval",
            ChartParam::ChangeFromClose(_res) => "changeFromClose",
            ChartParam::Last(_res) => "chartLast",
        };
        (first_arg.borrow(), self.to_str())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use url::Url;

//     // TODO: implement/update logic in match statement.
//     #[test]
//     fn test_parse_params() {
//         let client = Client;
//         let req = "stock";
//         let duration = Duration::OneDay;
//         let params: Vec<ChartParam> = vec![ChartParam::Last(5), ChartParam::Reset(true)];

//         let mut url = Url::parse("https://api.iextrading.com/1.0").unwrap();

//         url.path_segments_mut()
//             .unwrap()
//             .push("stock")
//             .push("aapl")
//             .push("chart")
//             .push(&duration.to_string());

//         let m_params = params.iter().map(|p| p.to_pair());

//         url = Url::parse_with_params(url.as_str(), m_params).unwrap();
//         println!("{}", url);
//     }

//     // NOTE(test): Unfortunately rust has no implementation of sub-testing, or at least not aware of as of yet.
//     #[test]
//     fn test_client_request_book() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Book { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_chart() {
//         let client = Client;
//         let symbol = "aapl";
//         let duration = Duration::OneDay;

//         assert!(
//             client
//                 .request(&Request::Chart {
//                     symbol,
//                     duration,
//                     params: None
//                 })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_company() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Company { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_delayed_quote() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::DelayedQuote { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_dividends() {
//         let client = Client;
//         let symbol = "aapl";
//         let duration = Duration::OneDay;
//         assert!(
//             client
//                 .request(&Request::Dividends { symbol, duration })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_earnings() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Earnings { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_effective_spread() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::EffectiveSpread { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_financials() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Financials { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_list() {
//         let client = Client;

//         assert!(
//             client
//                 .request(&Request::List {
//                     param: ListParam::Gainers
//                 })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_logo() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Logo { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_news() {
//         let client = Client;
//         let symbol = "aapl";
//         assert!(
//             client
//                 .request(&Request::News {
//                     symbol,
//                     range: None
//                 })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_ohlc() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Ohlc { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_peers() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Peers { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_previous() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Previous { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_price() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Price { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_quote() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Quote { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_relevant() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Relevant { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_splits() {
//         let client = Client;
//         let symbol = "aapl";
//         let duration = Duration::OneDay;

//         assert!(
//             client
//                 .request(&Request::Splits { symbol, duration })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_stats() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::Stats { symbol }).is_ok());
//     }

//     #[test]
//     fn test_client_request_symbols() {
//         let client = Client;

//         assert!(client.request(&Request::Symbols).is_ok());
//     }

//     #[test]
//     fn test_client_request_threshold_securiteis() {
//         let client = Client;

//         assert!(
//             client
//                 .request(&Request::ThresholdSecurities { date: None })
//                 .is_ok()
//         );
//     }

//     #[test]
//     fn test_client_request_volume_by_venue() {
//         let client = Client;
//         let symbol = "aapl";

//         assert!(client.request(&Request::VolumeByVenue { symbol }).is_ok());
//     }
// }
