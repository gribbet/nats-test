use data::Position;
use system::System;

fn main() -> std::io::Result<()> {
    let system = System::new()?;

    loop {
        let position = Position {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
        };
        system.publish(&"position", &position)?;
        println!("Published {:?}", position);
        std::thread::sleep(std::time::Duration::from_millis(200))
    }
}
