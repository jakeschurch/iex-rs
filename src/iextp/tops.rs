use chrono::{DateTime, Utc};

const ChannelID: u32 = 1;
const V_1_5_MessageProtocolID: u16 = 0x8002;
const V_1_6_MessageProtocolID: u16 = 0x8003;
const FeedName: &str = "TOPS";

struct SEM_Builder;

struct SystemEventMessage {
    Message: Message,
    SystemEvent: SystemEvent,
    Timestamp: DateTime<Utc>,
}

enum Message {
    // Administrative message formats
    SystemEvent = 0x53,
    SecurityDirectory = 0x44,
    TradingStatus = 0x48,
    OperationalHaltStatus = 0x4f,
    ShortSalePriceTestStatus = 0x50,

    // Trading message formats.
    QuoteUpdate = 0x51,
    TradeReport = 0x54,
    TradeBreak = 0x42,
    OfficialPrice = 0x58,

    // Auction message formats.
    AuctionInformation = 0x41,
}

enum SystemEvent {
    // Outside of heartbeat messages on the lower level protocol,
    // the start of day message is the first message in any trading session.
    StartOfMessages = 0x4f,
    // This message indicates that IEX is open and ready to start accepting
    // orders.
    StartOfSystemHours = 0x53,
    // This message indicates that DAY and GTX orders, as well as
    // market orders and pegged orders, are available for execution on IEX.
    StartOfRegularMarketHours = 0x52,
    // This message indicates that DAY orders, market orders, and pegged
    // orders are no longer accepted by IEX.
    EndOfRegularMarketHours = 0x4d,
    // This message indicates that IEX is now closed and will not accept
    // any new orders during this trading session. It is still possible to
    // receive messages after the end of day.
    EndOfSystemHours = 0x45,
    // This is always the last message sent in any trading session.
    EndOfMessages = 0x43,
}
