pub struct WatchListItem {
    pub symbol: String,
    pub price: f64,
    pub trend: f64,
}

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

#[derive(Debug, Clone, Copy)]
pub enum Volatility {
    Low,
    Medium,
    High,
}

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
    BUY,
    SELL,
}

#[derive(Debug, Clone, Copy)]
pub enum SignalParameter {
    LIM,
    STL,
    TAP,
    CHK,
}

pub struct Signal {
    pub kind: SignalKind,
    pub symbol: String,
    pub param: SignalParameter,
    pub price: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum LogKind {
    INFO,
    WARN,
    ERR,
    DEBUG,
}

#[derive(Debug, Clone)]
pub struct EventLog {
    pub kind: LogKind,
    pub name: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum AlertLevel {
    H,
    M,
    L,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum InspectTarget {
    None,

    Symbol {
        symbol: String,
        price: f64,
        trend: f64,

        market_trend: MarketTrend,
        volatility: Volatility,
        pressure: f64,

        alerts: Vec<Alert>,
    },

    Position {
        symbol: String,
        profit: f64,
        amount: f64,
    },

    Signal {
        kind: SignalKind,
        symbol: String,
        param: SignalParameter,
        price: f64,
    },

    Alert {
        level: AlertLevel,
        message: String,
    },
}
