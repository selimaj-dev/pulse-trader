use tokio::{
    io::AsyncReadExt,
    net::{
        UnixListener,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct TerminalServer {
    clients: Vec<OwnedWriteHalf>,
}

impl TerminalServer {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    pub async fn run(&mut self) -> tokio::io::Result<()> {
        let path = pulse_wire::server_path();

        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }

        let listener = UnixListener::bind(&path)?;

        println!("Terminal server listening on {:?}", path);

        loop {
            let (stream, _) = listener.accept().await?;

            let (reader, writer) = stream.into_split();

            self.clients.push(writer);

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

    pub async fn broadcast(&mut self) -> tokio::io::Result<()> {
        // for client in &mut self.clients {}
        Ok(())
    }
}
