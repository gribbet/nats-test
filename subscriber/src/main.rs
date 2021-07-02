use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    println!("Subscriber");

    let nats_url = &std::env::var("NATS").unwrap_or("localhost".to_string());
    let system = System::new(nats_url)?;

    loop {
        let measurement = system.receive::<Measurement>(&"temperature")?;
        println!("Received {:?}", measurement);
    }
}
