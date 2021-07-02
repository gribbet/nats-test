use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    println!("Publisher");

    let nats_url = &std::env::var("NATS").unwrap_or("localhost".to_string());
    let system = System::new(nats_url)?;

    loop {
        let measurement = Measurement { temperature: 1.0 };
        system.send(&"temperature", &measurement)?;
        println!("Published {:?}", measurement);
        std::thread::sleep(std::time::Duration::from_millis(200))
    }
}
