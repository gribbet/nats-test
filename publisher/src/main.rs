use bincode;
use common::Measurement;
use nats;

fn main() -> std::io::Result<()> {
    println!("Publisher");

    let host = &std::env::var("NATS").unwrap_or("localhost".to_string());
    let nats = nats::connect(host)?;

    loop {
        let measurement = Measurement { temperature: 1.0 };
        nats.publish(
            &"temperature",
            bincode::serialize::<Measurement>(&measurement).unwrap(),
        )?;
        println!("Published {:?}", measurement);
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
}
