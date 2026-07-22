use pulse_wire::{PulseWire, server_path, terminal::TerminalServerMessage};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::PulseTradeApp;

pub struct TerminalClient {
    writer: OwnedWriteHalf,
    reader: Option<OwnedReadHalf>,
}

impl TerminalClient {
    pub async fn new() -> tokio::io::Result<Self> {
        let (reader, writer) = UnixStream::connect(server_path()).await?.into_split();

        Ok(Self {
            reader: Some(reader),
            writer,
        })
    }

    pub async fn send(
        &mut self,
        message: pulse_wire::terminal::TerminalClientMessage,
    ) -> tokio::io::Result<()> {
        self.writer.write(&message.to_com()).await?;

        Ok(())
    }

    pub fn use_app(&mut self, app: PulseTradeApp) -> PulseTradeApp {
        let mut reader = None;

        std::mem::swap(&mut self.reader, &mut reader);

        let mut reader = reader.expect("Reader failed to swap");

        let watch_list = app.watch_list.clone();
        let active_positions = app.active_positions.clone();
        let logs = app.logs.clone();
        let signals = app.signals.clone();
        let market_overview = app.market_overview.clone();
        let status = app.status.clone();
        let inspect = app.inspect.clone();

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0u8; 4096];

                let len = reader
                    .read(&mut buffer)
                    .await
                    .expect("Failed to read socket");

                if len == 0 {
                    break;
                }

                buffer.truncate(len);

                match TerminalServerMessage::from_com(&mut buffer) {
                    TerminalServerMessage::WatchListUpdated(v) => {
                        *watch_list.lock().await = v;
                    }

                    TerminalServerMessage::PositionsUpdated(v) => {
                        *active_positions.lock().await = v;
                    }

                    TerminalServerMessage::OverviewUpdated(v) => {
                        *market_overview.lock().await = v;
                    }

                    TerminalServerMessage::SignalsUpdated(v) => {
                        *signals.lock().await = v;
                    }

                    TerminalServerMessage::Inspect(v) => {
                        *inspect.lock().await = v;
                    }

                    TerminalServerMessage::SetStatus(v) => {
                        *status.lock().await = v;
                    }

                    TerminalServerMessage::SetLogs(v) => {
                        *logs.lock().await = v;
                    }

                    TerminalServerMessage::AddLog(v) => {
                        logs.lock().await.push(v);
                    }
                }
            }
        });

        app
    }
}
