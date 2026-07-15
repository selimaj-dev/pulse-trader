#[derive(Debug, Clone)]
pub struct WatchListItem {
    pub symbol: String,
    pub price: f64,
    pub trend: f64,
}

#[derive(Debug, Clone)]
pub struct ActivePosition {
    pub symbol: String,
    pub profit: f64,
    pub amount: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum MarketTrend {
    Bullish,
    Bearish,
    Neutral,
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

#[derive(Debug, Clone, Copy)]
pub enum Volatility {
    Low,
    Medium,
    High,
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

#[derive(Debug, Clone)]
pub struct MarketOverview {
    pub trend: MarketTrend,
    pub volatility: Volatility,
    pub pressure: f64,

    pub alerts: Vec<Alert>,
}

#[derive(Debug, Clone, Copy)]
pub enum Feed {
    Connected,
    Disconnected,
    Connecting,
    Failed,
}

pub struct Status {
    pub feed: Feed,
    pub exchange: String,
    pub dex: String,
    pub latency: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum SignalKind {
    Buy,
    Sell,
}

impl std::fmt::Display for SignalKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignalParameter {
    Lim,
    Stl,
    Tap,
    Chk,
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

#[derive(Debug, Clone)]
pub struct Signal {
    pub kind: SignalKind,
    pub symbol: String,
    pub param: SignalParameter,
    pub price: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum LogKind {
    Info,
    Warn,
    Err,
    Debug,
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

#[derive(Debug, Clone)]
pub struct EventLog {
    pub kind: LogKind,
    pub name: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum AlertLevel {
    High,
    Medium,
    Low,
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

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum InspectTarget {
    None,
    Symbol(Box<(WatchListItem, MarketOverview)>),
    Position(ActivePosition),
    Signal(Signal),
    Alert(Alert),
}
