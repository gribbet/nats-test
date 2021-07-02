use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    println!("Publisher");

    let system = System::new()?;

    loop {
        let measurement = Measurement { temperature: 1.0 };
        system.publish(&"temperature", &measurement)?;
        println!("Published {:?}", measurement);
        std::thread::sleep(std::time::Duration::from_millis(200))
    }
}
