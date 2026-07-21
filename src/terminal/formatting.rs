use crate::types::{
    ActivePosition, Alert, AlertLevel, EventLog, InspectTarget, LogKind, MarketOverview, Signal,
    SignalKind, Status, WatchListItem,
};

pub trait Formatted {
    fn get_formatted(&self) -> Vec<String>;
}

impl Formatted for InspectTarget {
    fn get_formatted(&self) -> Vec<String> {
        match self {
            Self::None => vec!["\x1b[2mnothing to inspect\x1b[0m".to_string()],

            Self::Symbol(boxed) => {
                let (watch, overview) = &**boxed;
                vec![
                    Property("Symbol", format!("\x1b[35m{}\x1b[0m", watch.symbol)),
                    Property("Price", format!("\x1b[96m{}\x1b[0m", format_f64(watch.price))),
                    Property("Trend", format!("{}", format_f64(watch.trend))),
                    Property("Market", format!("{}", overview.trend)),
                    Property("Volatility", format!("{}", overview.volatility)),
                    Property(
                        "Pressure",
                        format!("{:+.2}%", overview.pressure * 100.0),
                    ),
                    Property("Alerts", format!("{}", overview.alerts.len())),
                ]
                .get_formatted()
            }

            Self::Position(pos) => vec![
                Property("Symbol", format!("\x1b[35m{}\x1b[0m", pos.symbol)),
                Property("Profit", format!("{:+.2}", pos.profit)),
                Property("Amount", format!("{}", pos.amount)),
            ]
            .get_formatted(),

            Self::Signal(sig) => vec![
                Property("Type", format!("{}", sig.kind)),
                Property("Symbol", format!("\x1b[35m{}\x1b[0m", sig.symbol)),
                Property("Parameter", format!("{}", sig.param)),
                Property("Price", format!("\x1b[96m{}\x1b[0m", format_f64(sig.price))),
            ]
            .get_formatted(),

            Self::Alert(alert) => vec![
                Property("Level", format!("{}", alert.level)),
                Property("Message", alert.message.clone()),
            ]
            .get_formatted(),
        }
    }
}

impl Formatted for Alert {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            format!(
                "{}{}\x1b[0m",
                match &self.level {
                    AlertLevel::Low => "\x1b[34m",
                    AlertLevel::Medium => "\x1b[33m",
                    AlertLevel::High => "\x1b[31m",
                },
                self.level
            ),
            self.message.clone(),
        ]
    }
}

impl Formatted for EventLog {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            format!(
                "{}{}\x1b[0m",
                match &self.kind {
                    LogKind::Info => "\x1b[34m",
                    LogKind::Debug => "\x1b[36m",
                    LogKind::Err => "\x1b[31m",
                    LogKind::Warn => "\x1b[33m",
                },
                self.kind
            ),
            format!("(\x1b[37m{}\x1b[0m)", self.name),
            self.message.clone(),
        ]
    }
}

impl Formatted for Signal {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            if matches!(self.kind, SignalKind::Buy) {
                format!("\x1b[32m{}\x1b[0m", self.kind)
            } else {
                format!("\x1b[31m{}\x1b[0m", self.kind)
            },
            format!("\x1b[35m{}\x1b[0m", self.symbol),
            format!("\x1b[34m{}\x1b[0m", self.param),
            format_f64(self.price),
        ]
    }
}

impl Formatted for WatchListItem {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            format!("\x1b[35m{}\x1b[0m", self.symbol),
            format_f64(self.price),
            format!(
                "{} {}",
                if self.trend.is_sign_positive() {
                    "\x1b[32m▲\x1b[0m"
                } else {
                    "\x1b[31m▼\x1b[0m"
                },
                format_f64(self.trend.abs())
            ),
        ]
    }
}

impl Formatted for ActivePosition {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            format!("\x1b[35m{}\x1b[0m", self.symbol),
            format_f64(self.amount),
            format!(
                "{} {}",
                if self.profit.is_sign_positive() {
                    "\x1b[32m▲\x1b[0m"
                } else {
                    "\x1b[31m▼\x1b[0m"
                },
                format_f64(self.profit.abs())
            ),
        ]
    }
}

struct Property(&'static str, String);

impl Formatted for MarketOverview {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            Property("TREND", format!("{}", self.trend)),
            Property("VOLATILITY", format!("{}", self.volatility)),
            Property(
                "PRESSURE",
                if self.pressure.is_sign_positive() {
                    format!("\x1b[32m{:.3}\x1b[0m", self.pressure)
                } else {
                    format!("\x1b[31m{:.3}\x1b[0m", self.pressure)
                },
            ),
        ]
        .get_formatted()
    }
}

impl Formatted for Status {
    fn get_formatted(&self) -> Vec<String> {
        vec![
            Property("Feed", format!("{:?}", self.feed)),
            Property("Exchange", self.exchange.clone()),
            Property("DEX", self.dex.clone()),
            Property("Latency", format!("{} ms", self.latency)),
        ]
        .get_formatted()
    }
}

impl Formatted for Property {
    fn get_formatted(&self) -> Vec<String> {
        vec![format!("\x1b[91m{}\x1b[0m", self.0), self.1.clone()]
    }
}

impl<T: Formatted> Formatted for Vec<T> {
    fn get_formatted(&self) -> Vec<String> {
        let mut output = Vec::new();

        let rows: Vec<Vec<String>> = self.iter().map(|v| v.get_formatted()).collect();

        if rows.is_empty() {
            return output;
        }

        let column_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        let mut widths = vec![0; column_count];

        for row in &rows {
            for (i, col) in row.iter().enumerate() {
                widths[i] = widths[i].max(col.len());
            }
        }

        for row in rows {
            let mut row_out = String::new();

            for (col_idx, col) in row.iter().enumerate() {
                let width = widths[col_idx];

                row_out.push_str(&format!("{:<width$}", col, width = width));

                if col_idx + 1 != row.len() {
                    row_out.push_str("  ");
                }
            }

            output.push(row_out);
        }

        output
    }
}

pub fn apply_padding(mut items: Vec<String>) -> Vec<String> {
    for item in &mut items {
        item.insert(0, ' ');
    }

    items
}

pub fn format_f64(value: f64) -> String {
    let val = value.to_string();
    let parts: Vec<&str> = val.split('.').collect();

    let int = parts[0].to_string();
    let negative = int.starts_with('-');

    let start = if negative { 1 } else { 0 };
    let mut result = String::new();

    for (i, c) in int[start..].chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    let mut formatted: String = result.chars().rev().collect();

    if negative {
        formatted.insert(0, '-');
    }

    if parts.len() > 1 {
        formatted.push('.');
        formatted.push_str(parts[1]);
    }

    formatted
}
