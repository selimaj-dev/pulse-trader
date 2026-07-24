pub mod config;
pub mod engine;
pub mod fetch;
pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let engine = engine::Engine::new().await?;

    let terminal_server = engine.spawn_terminal_server().await;

    let broadcaster = engine.spawn_broadcaster().await;

    engine.run_engine().await?;

    let (terminal_server, broadcaster) = tokio::join!(terminal_server, broadcaster);

    terminal_server??;
    broadcaster??;

    Ok(())
}
