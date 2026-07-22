pub mod terminal;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let terminal_server = terminal::TerminalServer::new();

    {
        let terminal_server = terminal_server.clone();

        tokio::spawn(async move {
            terminal_server
                .run()
                .await
                .expect("Failed to run terminal server");
        });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        terminal_server
            .broadcast(pulse_wire::terminal::TerminalServerMessage::AddLog(
                pulse_wire::terminal::EventLog {
                    kind: pulse_wire::terminal::LogKind::Debug,
                    name: "Engine".to_string(),
                    message: "Hello, world!".to_string(),
                },
            ))
            .await?;
        println!("Ok");
    }
}
