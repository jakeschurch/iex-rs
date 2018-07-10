use Endpoint;

#[derive(PartialEq, Eq)]
/// The `StocksEndpoint` enum allows for HTTP requests matching to a IEX Stocks Endpoint API.
// TODO:(Request): Add documentation from IEX website.
// TODO: use display_percent
pub enum StocksEndpoint<'a> {
    Book,
    Chart {
        duration: Duration<'a>,
        params: Option<Vec<ChartParam>>,
    },
    Company,
    DelayedQuote,
    Dividends {
        duration: Duration<'a>,
    },
    Earnings,
    EffectiveSpread,
    Financials,
    List {
        param: ListParam,
    },
    Logo,
    News {
        range: Option<i32>,
    },
    Ohlc,
    Peers,
    Previous,
    Price,
    Quote,
    Relevant,
    Splits {
        duration: Duration<'a>,
    },
    Stats,

    // TODO: IEX Short Interest List
    /// IEX Regulation SHO Threshold Securities List
    TimeSeries,
    ThresholdSecurities {
        date: Option<Duration<'a>>,
    },
    // TODO(ShortInterest): implement variant.
    VolumeByVenue,
}

impl<'a> Endpoint for StocksEndpoint<'a> {
    fn to_endpoint(self) -> String {
        use self::StocksEndpoint::*;

        match self {
            Book => String::from("book"),

            Chart {
                ref duration,
                ref params,
            } => format!(
                "chart/{}?{chart_params}",
                duration.to_string(),
                chart_params = match params {
                    Some(parameters) => {
                        let chart_params: String = parameters
                            .iter()
                            .map(|param| param.to_string() + "/")
                            .collect();
                        chart_params
                    }
                    None => String::from(""),
                }
            ),

            Company => String::from("company"),

            DelayedQuote => String::from("delayed-quote"),

            Dividends { ref duration } => format!("dividends/{}", duration.to_string()),

            Earnings => String::from("earnings"),

            EffectiveSpread => String::from("effective-spread"),

            Financials => String::from("financials"),

            List { ref param } => format!("list/{}", param.to_string()),

            Logo => String::from("logo"),

            News { ref range } => format!(
                "news/last/{}",
                range.map(|r| r.to_string()).unwrap_or("".to_string()),
            ),

            Ohlc => String::from("ohlc"),

            Peers => String::from("peers"),

            Previous => String::from("previous"),

            Price => String::from("price"),

            Quote => String::from("quote"),

            Relevant => String::from("relevant"),

            Stats => String::from("stats"),

            Splits { ref duration } => format!("splits/{}", duration.to_string()),

            TimeSeries => String::from("time-series"),

            ThresholdSecurities { ref date } => format!(
                "threshold-securities/{}",
                date.unwrap_or(Duration::None).to_string(),
            ),

            VolumeByVenue => String::from("volume-by-venue"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration<'a> {
    FiveYears,
    TwoYears,
    OneYear,
    YearToDate,
    SixMonths,
    ThreeMonths,
    OneMonth,
    OneDay,
    Date(&'a str),
    Dynamic,
    None,
}

impl<'a> ToString for Duration<'a> {
    fn to_string(&self) -> String {
        use self::Duration::*;

        match self {
            FiveYears => String::from("5y"),
            TwoYears => String::from("2y"),
            OneYear => String::from("1y"),
            YearToDate => String::from("ytd"),
            SixMonths => String::from("6m"),
            ThreeMonths => String::from("3m"),
            OneMonth => String::from("1m"),
            OneDay => String::from("1d"),
            Date(ref date) => format!("date/{}", date),
            Dynamic => String::from("dynamic"),
            None => String::from(""),
        }
    }
}

impl<'a> Default for Duration<'a> {
    fn default() -> Duration<'a> {
        Duration::OneMonth
    }
}

#[derive(PartialEq, Eq)]
pub enum ListParam {
    MostActive,
    Gainers,
    Losers,
    IexVolume,
    IexPercent,
}

impl ToString for ListParam {
    fn to_string(&self) -> String {
        use self::ListParam::*;

        match self {
            MostActive => String::from("mostactive"),
            Gainers => String::from("gainers"),
            Losers => String::from("losers"),
            IexVolume => String::from("iexvolume"),
            IexPercent => String::from("iexpercent"),
        }
    }
}

#[derive(PartialEq, Eq)]
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

impl ToString for ChartParam {
    fn to_string(&self) -> String {
        use self::ChartParam::*;

        match self {
            Reset(ref res) => format!("chartReset={}", res),
            Simplify(ref res) => format!("chartSimplify={}", res),
            Interval(ref res) => format!("chartInterval={}", res),
            ChangeFromClose(ref res) => format!("changeFromClose={}", res),
            Last(ref res) => format!("chartLast={}", res),
        }
    }
}

// pub enum ReferenceEndpoint<'a> {
//     Symbols,
//     CorporateActions { date: Option<&'a str> },
// }

// impl<'a> ReferenceEndpoint<'a> {
//     pub fn to_endpoint(self) -> String {
//         match self {
//             ReferenceEndpoint::Symbols => String::from("symbols"),
//             ReferenceEndpoint::CorporateActions => format!("{}", )
//         }
//     }
// }
