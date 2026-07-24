pub mod config;
pub mod engine;
pub mod fetch;
pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let engine = engine::Engine::new().await?;

    engine.spawn_terminal_server();

    engine.spawn_broadcaster();

    engine.run_engine().await?;

    Ok(())
}
