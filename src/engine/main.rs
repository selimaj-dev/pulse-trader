pub mod config;
pub mod engine;
pub mod fetch;
pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let engine = engine::Engine::new().await?;

    engine.spawn_terminal_server().await?;

    engine.spawn_broadcaster().await?;

    engine.run_engine().await?;

    Ok(())
}
