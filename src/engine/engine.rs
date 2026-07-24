use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{config::Config, terminal::TerminalServer};

#[derive(Debug, Clone)]
pub struct Engine {
    pub terminal_server: Arc<TerminalServer>,
    pub config: Arc<Mutex<Config>>,
}

impl Engine {
    pub async fn new() -> tokio::io::Result<Arc<Self>> {
        let config = Arc::new(Mutex::new(Config::new().await?));

        Ok(Arc::new_cyclic(|engine| Self {
            terminal_server: TerminalServer::new(engine.clone()),
            config,
        }))
    }

    pub async fn spawn_terminal_server(&self) -> JoinHandle<tokio::io::Result<()>> {
        let terminal_server = self.terminal_server.clone();

        tokio::spawn(async move { terminal_server.run().await })
    }

    pub async fn spawn_broadcaster(&self) -> JoinHandle<tokio::io::Result<()>> {
        let s = self.clone();

        tokio::spawn(async move { s.run_broadcaster().await })
    }

    pub async fn run_broadcaster(&self) -> tokio::io::Result<()> {
        let mut refresh = tokio::time::interval(tokio::time::Duration::from_secs(5));

        loop {
            refresh.tick().await;

            let watch_list = &self.config.lock().await.watchlist.symbols;

            match crate::fetch::fetch_watch_list(watch_list).await {
                Ok(watch_list) => {
                    if let Err(error) = self
                        .terminal_server
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
    }

    pub async fn run_engine(&self) -> tokio::io::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        }
    }

    pub async fn execute_command(&self, command: &str, _args: Vec<&str>) -> tokio::io::Result<()> {
        match command {
            _ => {
                self.terminal_server
                    .error(
                        "Command executor",
                        &format!("Command '{}' not found", command),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
