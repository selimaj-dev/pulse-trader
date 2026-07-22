use pulse_macros::p_com;

#[p_com]
pub struct WatchListItem {
    symbol: String,
    price: f64,
    trend: f64,
}

#[p_com]
pub struct ActivePosition {
    symbol: String,
    profit: f64,
    amount: f64,
}

#[p_com]
pub enum MarketTrend {
    Bullish,
    Bearish,
    Neutral,
}

#[p_com]
pub enum Volatility {
    Low,
    Medium,
    High,
}

#[p_com]
pub struct MarketOverview {
    trend: MarketTrend,
    volatility: Volatility,
    pressure: f64,

    alerts: Vec<Alert>,
}

#[p_com]
pub enum Feed {
    Connected,
    Disconnected,
    Connecting,
    Failed,
}

#[p_com]
pub struct Status {
    feed: Feed,
    exchange: String,
    dex: String,
    latency: u16,
}

#[p_com]
pub enum SignalKind {
    Buy,
    Sell,
}

#[p_com]
pub enum SignalParameter {
    Lim,
    Stl,
    Tap,
    Chk,
}

#[p_com]
pub struct Signal {
    kind: SignalKind,
    symbol: String,
    param: SignalParameter,
    price: f64,
}

#[p_com]
pub enum LogKind {
    Info,
    Warn,
    Err,
    Debug,
}

#[p_com]
pub struct EventLog {
    kind: LogKind,
    name: String,
    message: String,
}

#[p_com]
pub enum AlertLevel {
    High,
    Medium,
    Low,
}

#[p_com]
pub struct Alert {
    level: AlertLevel,
    message: String,
}

#[p_com]
pub enum InspectTarget {
    None,
    Symbol(WatchListItem),
    Position(ActivePosition),
    Signal(Signal),
    Alert(Alert),
}

impl std::fmt::Display for MarketTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bullish => write!(f, "Bullish"),
            Self::Bearish => write!(f, "Bearish"),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "H"),
            Self::Medium => write!(f, "M"),
            Self::Low => write!(f, "L"),
        }
    }
}

impl std::fmt::Display for Volatility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

impl std::fmt::Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Err => write!(f, "ERR"),
            Self::Debug => write!(f, "DEBUG"),
        }
    }
}

impl std::fmt::Display for SignalKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

impl std::fmt::Display for SignalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lim => write!(f, "LIM"),
            Self::Stl => write!(f, "STL"),
            Self::Tap => write!(f, "TAP"),
            Self::Chk => write!(f, "CHK"),
        }
    }
}
