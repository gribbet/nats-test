use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    let system = System::new()?;
    let measurements = system.subscribe::<Measurement>(&"temperature")?;

    for measurement in measurements {
        println!("Received {:?}", measurement);
    }

    Ok(())
}
