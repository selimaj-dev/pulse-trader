use crate::PulseTradeApp;

impl PulseTradeApp {
    pub async fn execute_command(&mut self, ctx: &pulse_ui::state::Context, command: &str) {
        if command.is_empty() {
            return;
        }

        match command.trim() {
            "exit" | "quit" | "leave" => {
                ctx.close().await;
            }

            _ => {
                if let Err(e) = self
                    .sock
                    .as_mut()
                    .unwrap()
                    .send(pulse_wire::terminal::TerminalClientMessage::ExecuteCommand(
                        command.to_string(),
                    ))
                    .await
                {
                    eprintln!("{e}");
                }
            }
        }
    }
}
