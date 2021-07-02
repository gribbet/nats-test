use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    println!("Subscriber");

    let nats_url = &std::env::var("NATS").unwrap_or("localhost".to_string());
    let system = System::new(nats_url)?;

    system
        .subscribe::<Measurement>(&"temperature")?
        .for_each(|measurement| println!("Received {:?}", measurement));

    Ok(())
}
