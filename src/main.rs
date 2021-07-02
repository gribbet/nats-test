use bincode;
use nats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Measurement {
    temperature: f64,
}

fn main() -> std::io::Result<()> {
    let host = &std::env::var("NATS").unwrap_or("localhost".to_string());
    println!("Test {:?}", host);
    let nats = nats::connect(host)?;
    let subject = nats.new_inbox();
    let subscription = nats.subscribe(&subject)?;

    let measurement = Measurement { temperature: 10.0 };
    nats.publish(
        &subject,
        bincode::serialize::<Measurement>(&measurement).unwrap(),
    )?;

    let mut result = subscription
        .iter()
        .map(move |msg| bincode::deserialize::<Measurement>(&msg.data));
    println!("received {:?}", result.next().unwrap());

    Ok(())
}
