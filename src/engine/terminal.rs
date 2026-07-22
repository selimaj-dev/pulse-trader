use std::sync::Arc;

use pulse_wire::PulseWire;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        UnixListener,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
};

#[derive(Debug, Clone)]
pub struct TerminalServer {
    clients: Arc<Mutex<Vec<OwnedWriteHalf>>>,
}

impl TerminalServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&self) -> tokio::io::Result<()> {
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

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(reader).await {
                    eprintln!("Terminal connection error: {err}");
                }
            });
        }
    }

    async fn handle_client(mut reader: OwnedReadHalf) -> tokio::io::Result<()> {
        let input = tokio::spawn(async move {
            let mut buffer = [0u8; 2048];

            loop {
                let size = reader.read(&mut buffer).await?;

                if size == 0 {
                    break;
                }

                println!("Received {} bytes", size);
            }

            Ok::<(), tokio::io::Error>(())
        });

        let _ = tokio::try_join!(input)?;

        Ok(())
    }

    pub async fn broadcast(
        &self,
        message: pulse_wire::terminal::TerminalServerMessage,
    ) -> tokio::io::Result<()> {
        let msg = message.to_com();

        for i in (0..self.clients.lock().await.len()).rev() {
            if let Err(e) = self.clients.lock().await[i].write(&msg).await {
                self.clients.lock().await.remove(i);
                println!("{e:?}");
            }
        }

        Ok(())
    }
}
