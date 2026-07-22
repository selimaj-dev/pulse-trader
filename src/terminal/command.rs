use crate::{PulseTradeApp, ptc::EventLog};

impl PulseTradeApp {
    pub async fn execute_command(&mut self, ctx: &pulse_ui::state::Context, command: &str) {
        if command.is_empty() {
            return;
        }

        let (command, _args) = if let Some((command, args)) = command.split_once(" ") {
            (command, args.split(" ").collect())
        } else {
            (command, Vec::new())
        };

        match command {
            "exit" | "quit" | "leave" => {
                ctx.close().await;
            }

            _ => {
                self.logs.lock().await.push(EventLog {
                    kind: crate::ptc::LogKind::Err,
                    name: "cmd".to_string(),
                    message: format!("Command '{}' not found", command),
                });
            }
        }
    }
}
