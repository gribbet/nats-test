use data::Position;
use system::System;

fn main() -> std::io::Result<()> {
    let system = System::new()?;
    let positions = system.subscribe::<Position>(&"position")?;

    for position in positions {
        println!("Received {:?}", position);
    }

    Ok(())
}
