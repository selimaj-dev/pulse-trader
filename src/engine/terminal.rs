use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

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

use crate::engine::Engine;

#[derive(Debug)]
pub struct TerminalServer {
    clients: Mutex<HashMap<usize, OwnedWriteHalf>>,
    logs: Mutex<Vec<EventLog>>,
    engine: Weak<Engine>,
}

impl TerminalServer {
    pub fn new(engine: Weak<Engine>) -> Arc<Self> {
        Arc::new(Self {
            clients: Mutex::new(HashMap::new()),
            logs: Mutex::new(Vec::new()),
            engine,
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

            let id = rand::random();

            self.clients.lock().await.insert(id, writer);

            let s = self.clone();

            tokio::spawn(async move {
                if let Err(err) = s.handle_client(&id, reader).await {
                    eprintln!("Terminal connection error: {err}");
                }
            });
        }
    }

    async fn handle_client(
        self: &Arc<Self>,
        id: &usize,
        mut reader: OwnedReadHalf,
    ) -> tokio::io::Result<()> {
        self.send_to(
            id,
            pulse_wire::terminal::TerminalServerMessage::SetLogs(self.logs.lock().await.clone()),
        )
        .await?;

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

                    let (command, args) = if let Some((command, args)) = command.split_once(" ") {
                        (command, args.split(" ").collect())
                    } else {
                        (command, Vec::new())
                    };

                    self.get_engine().execute_command(command, args).await?;
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

        let mut remove_clients = Vec::new();

        for (id, client) in clients.iter_mut() {
            if let Err(e) = Self::send_to_client(client, &msg).await {
                remove_clients.push(*id);
                println!("{e:?}");
            }
        }

        for id in remove_clients {
            clients.remove(&id);
        }

        Ok(())
    }

    pub async fn send_to(
        self: &Arc<Self>,
        id: &usize,
        message: pulse_wire::terminal::TerminalServerMessage,
    ) -> tokio::io::Result<()> {
        Self::send_to_client(
            self.clients.lock().await.get_mut(id).ok_or_else(|| {
                tokio::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Client({id}) does not exist"),
                )
            })?,
            &message.to_com(),
        )
        .await
    }

    pub async fn send_to_client(client: &mut OwnedWriteHalf, msg: &[u8]) -> tokio::io::Result<()> {
        client.write(&msg.len().to_le_bytes()).await?;
        client.write(msg).await?;
        client.flush().await?;

        Ok(())
    }

    pub async fn log(
        self: &Arc<Self>,
        kind: LogKind,
        name: &str,
        message: &str,
    ) -> tokio::io::Result<()> {
        let log = EventLog {
            kind,
            name: name.to_string(),
            message: message.to_string(),
        };

        self.logs.lock().await.push(log.clone());

        self.broadcast(pulse_wire::terminal::TerminalServerMessage::AddLog(log))
            .await
    }

    pub async fn info(self: &Arc<Self>, name: &str, message: &str) -> tokio::io::Result<()> {
        self.log(LogKind::Info, name, message).await
    }

    pub async fn warn(self: &Arc<Self>, name: &str, message: &str) -> tokio::io::Result<()> {
        self.log(LogKind::Warn, name, message).await
    }

    pub async fn error(self: &Arc<Self>, name: &str, message: &str) -> tokio::io::Result<()> {
        self.log(LogKind::Err, name, message).await
    }

    pub async fn debug(self: &Arc<Self>, name: &str, message: &str) -> tokio::io::Result<()> {
        self.log(LogKind::Debug, name, message).await
    }

    pub fn get_engine(&self) -> Arc<Engine> {
        self.engine
            .upgrade()
            .expect("Failed to upgrade engine(Weak) to Arc")
    }
}
