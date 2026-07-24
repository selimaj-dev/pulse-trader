use std::sync::Arc;

use pulse_wire::{
    PulseWire,
    terminal::{EventLog, LogKind, TerminalClientMessage},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        UnixListener,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
};

#[derive(Debug)]
pub struct TerminalServer {
    clients: Mutex<Vec<OwnedWriteHalf>>,
    logs: Mutex<Vec<EventLog>>,
}

impl TerminalServer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            clients: Mutex::new(Vec::new()),
            logs: Mutex::new(Vec::new()),
        })
    }

    pub async fn run(self: &Arc<Self>) -> tokio::io::Result<()> {
        let path = pulse_wire::server_path();

        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }

        let listener = UnixListener::bind(&path)?;

        println!("Terminal server listening on {:?}", path);

        loop {
            let (stream, _) = listener.accept().await?;

            let (reader, writer) = stream.into_split();

            self.clients.lock().await.push(writer);

            let s = self.clone();

            tokio::spawn(async move {
                if let Err(err) = s.handle_client(reader).await {
                    eprintln!("Terminal connection error: {err}");
                }
            });
        }
    }

    async fn handle_client(self: &Arc<Self>, mut reader: OwnedReadHalf) -> tokio::io::Result<()> {
        loop {
            let mut len_buf = [0u8; size_of::<usize>()];
            let size = reader.read_exact(&mut len_buf).await?;

            let len = usize::from_le_bytes(len_buf);

            if size == 0 || len == 0 {
                break;
            }

            let mut buffer = vec![0u8; len];

            reader.read_exact(&mut buffer).await?;

            match TerminalClientMessage::from_com(&mut buffer) {
                TerminalClientMessage::ExecuteCommand(command) => {
                    let command = command.as_str();

                    let (command, _args) = if let Some((command, args)) = command.split_once(" ") {
                        (command, args.split(" ").collect())
                    } else {
                        (command, Vec::new())
                    };

                    match command {
                        _ => {
                            self.broadcast(pulse_wire::terminal::TerminalServerMessage::AddLog(
                                pulse_wire::terminal::EventLog {
                                    kind: pulse_wire::terminal::LogKind::Err,
                                    name: "Command executor".to_string(),
                                    message: format!("Command '{}' not found", command),
                                },
                            ))
                            .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn broadcast(
        self: &Arc<Self>,
        message: pulse_wire::terminal::TerminalServerMessage,
    ) -> tokio::io::Result<()> {
        let msg = message.to_com();

        let mut clients = self.clients.lock().await;

        for i in (0..clients.len()).rev() {
            if let Err(e) = Self::send_to_client(&mut clients, i, &msg).await {
                clients.remove(i);
                println!("{e:?}");
            }
        }

        Ok(())
    }

    pub async fn send_to_client(
        clients: &mut tokio::sync::MutexGuard<'_, Vec<OwnedWriteHalf>>,
        i: usize,
        msg: &[u8],
    ) -> tokio::io::Result<()> {
        clients[i].write(&msg.len().to_le_bytes()).await?;
        clients[i].write(msg).await?;
        clients[i].flush().await?;

        Ok(())
    }

    pub async fn log(self: &Arc<Self>, kind: LogKind, name: &str, message: &str) {
        self.logs.lock().await.push(EventLog {
            kind,
            name: name.to_string(),
            message: message.to_string(),
        });
    }

    pub async fn info(self: &Arc<Self>, name: &str, message: &str) {
        self.log(LogKind::Info, name, message).await
    }

    pub async fn warn(self: &Arc<Self>, name: &str, message: &str) {
        self.log(LogKind::Warn, name, message).await
    }

    pub async fn error(self: &Arc<Self>, name: &str, message: &str) {
        self.log(LogKind::Err, name, message).await
    }

    pub async fn debug(self: &Arc<Self>, name: &str, message: &str) {
        self.log(LogKind::Debug, name, message).await
    }
}
