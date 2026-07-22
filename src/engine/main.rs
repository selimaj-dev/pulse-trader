use pulse_wire::terminal::{ActivePosition, Signal, WatchListItem};

pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let terminal_server = terminal::TerminalServer::new();

    {
        let terminal_server = terminal_server.clone();

        tokio::spawn(async move {
            terminal_server
                .run()
                .await
                .expect("Failed to run terminal server");
        });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

        terminal_server
            .broadcast(
                pulse_wire::terminal::TerminalServerMessage::WatchListUpdated(vec![
                    WatchListItem {
                        symbol: "BTC".to_string(),
                        price: 118_402.12,
                        trend: 0.82,
                    },
                    WatchListItem {
                        symbol: "ETH".to_string(),
                        price: 3_912.48,
                        trend: -0.41,
                    },
                    WatchListItem {
                        symbol: "SOL".to_string(),
                        price: 182.91,
                        trend: 2.18,
                    },
                    WatchListItem {
                        symbol: "XRP".to_string(),
                        price: 2.84,
                        trend: 1.22,
                    },
                ]),
            )
            .await?;

        terminal_server
            .broadcast(
                pulse_wire::terminal::TerminalServerMessage::PositionsUpdated(vec![
                    ActivePosition {
                        symbol: "BTC".to_string(),
                        profit: 125.50,
                        amount: 0.25,
                    },
                    ActivePosition {
                        symbol: "SOL".to_string(),
                        profit: 84.20,
                        amount: 5.0,
                    },
                    ActivePosition {
                        symbol: "ETH".to_string(),
                        profit: -32.75,
                        amount: 1.0,
                    },
                    ActivePosition {
                        symbol: "XRP".to_string(),
                        profit: 12.30,
                        amount: 0.75,
                    },
                ]),
            )
            .await?;

        terminal_server
            .broadcast(pulse_wire::terminal::TerminalServerMessage::SignalsUpdated(
                vec![
                    Signal {
                        kind: pulse_wire::terminal::SignalKind::Buy,
                        symbol: "BTC".to_string(),
                        param: pulse_wire::terminal::SignalParameter::Lim,
                        price: 118_800.0,
                    },
                    Signal {
                        kind: pulse_wire::terminal::SignalKind::Buy,
                        symbol: "BTC".to_string(),
                        param: pulse_wire::terminal::SignalParameter::Tap,
                        price: 120_000.0,
                    },
                    Signal {
                        kind: pulse_wire::terminal::SignalKind::Buy,
                        symbol: "BTC".to_string(),
                        param: pulse_wire::terminal::SignalParameter::Stl,
                        price: 118_000.0,
                    },
                ],
            ))
            .await?;

        terminal_server
            .broadcast(pulse_wire::terminal::TerminalServerMessage::AddLog(
                pulse_wire::terminal::EventLog {
                    kind: pulse_wire::terminal::LogKind::Debug,
                    name: "Engine".to_string(),
                    message: "Hello, world!".to_string(),
                },
            ))
            .await?;

        println!("Ok");
    }
}
