use crate::terminal::TerminalServer;

const WATCH_LIST_SYMBOLS: &[&str] = &["BTC", "ETH", "SOL", "XRP"];

pub struct Engine {
    pub terminal_server: TerminalServer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            terminal_server: TerminalServer::new(),
        }
    }

    pub fn spawn_terminal_server(&self) {
        let terminal_server = self.terminal_server.clone();

        tokio::spawn(async move {
            terminal_server
                .run()
                .await
                .expect("Failed to run terminal server");
        });
    }

    pub fn spawn_broadcaster(&mut self) {
        let terminal_server = self.terminal_server.clone();

        tokio::spawn(async move {
            let mut refresh = tokio::time::interval(tokio::time::Duration::from_secs(5));

            loop {
                refresh.tick().await;

                match crate::fetch::fetch_watch_list(WATCH_LIST_SYMBOLS).await {
                    Ok(watch_list) => {
                        if let Err(error) = terminal_server
                            .broadcast(
                                pulse_wire::terminal::TerminalServerMessage::WatchListUpdated(
                                    watch_list,
                                ),
                            )
                            .await
                        {
                            eprintln!("Failed to broadcast Hyperliquid watch list: {error}");
                        }
                    }
                    Err(error) => eprintln!("Failed to refresh Hyperliquid watch list: {error}"),
                }
            }
        });
    }

    pub async fn run_engine(&mut self) -> tokio::io::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        }
    }
}
