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
            loop {
                let mut len_buf = [0u8; size_of::<usize>()];
                let size = reader.read_exact(&mut len_buf).await?;

                let len = usize::from_le_bytes(len_buf);

                if size == 0 || len == 0 {
                    break;
                }

                let mut buffer = vec![0u8; len];

                reader.read_exact(&mut buffer).await?;

                println!("Received {} bytes", len);
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
}
