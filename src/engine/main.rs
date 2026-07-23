pub mod engine;
pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut engine = engine::Engine::new();

    engine.spawn_terminal_server();

    engine.spawn_broadcaster();

    engine.run_engine().await?;

    Ok(())
}
