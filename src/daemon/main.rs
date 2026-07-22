pub mod ptc;

use std::time::Instant;

use ptc::PulseCom;

fn main() {
    let input = ptc::EventLog {
        kind: ptc::LogKind::Warn,
        name: "Test".to_string(),
        message: "Hello, WOrld".to_string(),
    };

    let start = Instant::now();

    let mut val = input.to_com();
    let out = ptc::EventLog::from_com(&mut val);

    let elapsed = start.elapsed();

    println!("{:?} {:?}", out, elapsed);
}
