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
