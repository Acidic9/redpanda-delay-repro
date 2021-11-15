use std::error;

use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};

const REDPANDA_HOST: &str = "127.0.0.1:9092";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut config = ClientConfig::new();
    config
        .set("group.id", env!("CARGO_PKG_NAME"))
        .set("bootstrap.servers", REDPANDA_HOST)
        .set_log_level(RDKafkaLogLevel::Debug);

    let consumer: StreamConsumer = config.create()?;
    consumer.subscribe(&["hello"])?;

    loop {
        println!("Waiting for message...");
        let msg = consumer.recv().await?;
        let key = msg.key_view::<str>().transpose()?.unwrap_or_default();
        let payload = msg.payload_view::<str>().transpose()?.unwrap_or_default();
        println!("Received message: key = {}, payload = {}", key, payload);
    }
}
