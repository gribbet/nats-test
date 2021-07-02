use common::Measurement;
use common::System;

fn main() -> std::io::Result<()> {
    println!("Subscriber");

    let system = System::new()?;

    system
        .subscribe::<Measurement>(&"temperature")?
        .for_each(|measurement| println!("Received {:?}", measurement));

    Ok(())
}
