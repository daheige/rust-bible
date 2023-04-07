use env_logger;
use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
fn main() {
    env_logger::init(); // 初始化logger配置
    println!("kafka demo....");
    let broker = "localhost:9092";
    let topic = "my-topic";
    let data = "hello,rust".as_bytes();
    let mut i = 0;
    while i < 1000 {
        if let Err(e) = message_publish(data, topic, vec![broker.to_owned()]) {
            println!("failed producing message error:{}", e);
        }
        i += 1;
    }
}

// send message
fn message_publish<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("publish message at {:?} to {}", brokers, topic);

    // create producer
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    // send message
    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    // you can send message by this way
    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}
