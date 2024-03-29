use env_logger;
use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use log::{error, info};
use std::env;
use std::time::Duration;
fn main() {
    // 日志level 优先级  error > warn > info > debug > trace
    // 设置日志级别环境变量
    env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 初始化logger配置
    println!("kafka demo....");
    let broker = "localhost:9092";
    let topic = "my-topic";

    info!("publish message...");
    let mut i = 0;
    while i < 10000 {
        // let data = "hello world,rust";
        // if let Err(e) = message_publish(data.as_bytes(), topic, vec![broker.to_owned()]) {
        //     println!("failed producing message error:{}", e);
        // }
        println!("current index:{}", i);
        if let Err(e) = message_publish(
            format!("hello world: {}", i).as_bytes(),
            topic,
            vec![broker.to_owned()],
        ) {
            error!("failed producing message error:{}", e);
        }

        i += 1;
    }
}

// send message
fn message_publish(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
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
