use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{config::Config, terminal::TerminalServer};

pub struct Engine {
    pub terminal_server: Arc<TerminalServer>,
    pub config: Arc<Mutex<Config>>,
}

impl Engine {
    pub async fn new() -> tokio::io::Result<Self> {
        Ok(Self {
            terminal_server: TerminalServer::new(),
            config: Arc::new(Mutex::new(Config::new().await?)),
        })
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

    pub fn spawn_broadcaster(&self) {
        let terminal_server = self.terminal_server.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut refresh = tokio::time::interval(tokio::time::Duration::from_secs(5));

            loop {
                refresh.tick().await;

                let watch_list = &config.lock().await.watchlist.symbols;

                match crate::fetch::fetch_watch_list(watch_list).await {
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

    pub async fn run_engine(&self) -> tokio::io::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        }
    }
}
