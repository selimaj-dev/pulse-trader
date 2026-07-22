pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut terminal_server = terminal::TerminalServer::new();

    terminal_server.run().await?;

    Ok(())
}
