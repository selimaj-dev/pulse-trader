use std::time::Instant;

use pulse_wire::PulseWire;

fn main() {
    let input = pulse_wire::terminal::EventLog {
        kind: pulse_wire::terminal::LogKind::Warn,
        name: "Test".to_string(),
        message: "Hello, WOrld".to_string(),
    };

    let start = Instant::now();

    let mut val = input.to_com();
    let out = pulse_wire::terminal::EventLog::from_com(&mut val);

    let elapsed = start.elapsed();

    println!("{:?} {:?}", out, elapsed);
}
