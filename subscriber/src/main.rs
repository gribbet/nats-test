use bincode;
use common::Measurement;
use nats;

fn main() -> std::io::Result<()> {
    println!("Subscriber");

    let host = &std::env::var("NATS").unwrap_or("localhost".to_string());
    let nats = nats::connect(host)?;
    let subscription = nats.subscribe(&"temperature")?;

    subscription.iter().for_each(move |msg| {
        let measurement = bincode::deserialize::<Measurement>(&msg.data);
        println!("Received {:?}", measurement.unwrap());
    });

    Ok(())
}
