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

pub struct Account {
    pub equity: f64,
    pub liquid: f64,
    pub unreal: f64,
    pub realized: f64,
    pub margin: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Feed {
    Connected,
    Disconnected,
    Connecting,
    Failed,
}

pub struct System {
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
