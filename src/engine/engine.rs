use crate::terminal::TerminalServer;

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
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
            }
        });
    }

    pub async fn run_engine(&mut self) -> tokio::io::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        }

        Ok(())
    }
}
