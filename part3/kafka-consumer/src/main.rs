use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
// use kafka::producer::AsBytes;
use log::{error, info};
use std::env;
use std::thread;
use std::time::Duration;
fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let broker = "localhost:9092".to_owned();
    let topic = "my-topic";
    let group = "my-group-2";

    info!("consumer message begin...");
    // consumer message
    if let Err(err) = consumer_message(group, topic, vec![broker]) {
        error!("consumer message err:{}", err);
    }
}

fn consumer_message(group: &str, topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    // create consumer connection
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic.to_string())
        .with_group(group.to_string())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;
    loop {
        let message_sets = con.poll()?;
        if message_sets.is_empty() {
            println!("no message available right now");
            thread::sleep(Duration::from_secs(2));
            continue;
            // return Ok(());
        }

        // 为了方便查看value，我这里转换为了string格式
        for ms in message_sets.iter() {
            for m in ms.messages() {
                println!(
                    "topic:{} group:{} partition:{}@offset:{}: value:{:?}",
                    ms.topic(),
                    group,
                    ms.partition(),
                    m.offset,
                    String::from_utf8(m.value.to_owned()).unwrap_or("".to_string()),
                );
            }

            let _ = con.consume_messageset(ms);
        }

        // commit consumed
        con.commit_consumed()?;
    }
}
