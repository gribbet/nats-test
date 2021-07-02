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

    pub fn publish<T: Serialize>(&self, subject: &str, message: &T) -> std::io::Result<()> {
        self.connection
            .publish(subject, bincode::serialize(message).unwrap())?; // TODO: Unwrap
        Ok(())
    }

    pub fn subscribe<'a, T: DeserializeOwned>(
        &self,
        subject: &str,
    ) -> std::io::Result<impl Iterator<Item = T>> {
        Ok(self
            .connection
            .subscribe(subject)?
            .into_iter()
            .map(move |message| bincode::deserialize::<T>(&message.data).unwrap()))
        // TODO: Unwrap
    }
}
