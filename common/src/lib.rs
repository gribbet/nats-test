use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {
    pub temperature: f64,
}

pub struct System {
    connection: nats::Connection,
}

impl System {
    pub fn new(nats_url: &str) -> std::io::Result<System> {
        Ok(System {
            connection: nats::connect(nats_url)?,
        })
    }

    pub fn send<T: Serialize>(&self, subject: &str, message: &T) -> std::io::Result<()> {
        self.connection
            .publish(subject, bincode::serialize(message).unwrap())?; // TODO: Unwrap
        Ok(())
    }

    pub fn receive<'a, T: DeserializeOwned>(&self, subject: &str) -> std::io::Result<T> {
        Ok(bincode::deserialize::<T>(
            &self
                .connection
                .subscribe(subject)?
                .next_timeout(std::time::Duration::from_secs(5))?
                .data,
        )
        .unwrap()) // TODO: Unwrap
    }
}
