// 用于序列化和反序列化
use serde::{Serialize, Deserialize};
use serde_json;

use futures::TryStreamExt;
use pulsar::{
    Authentication, Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor,
    Error as PulsarError,
};

use std::env;
use env_logger;
use log::{info, error};

// 定义消息格式
#[derive(Serialize, Deserialize)]
struct Message {
    data: String,
}

// 实现反序列化
impl DeserializeMessage for Message {
    type Output = Result<Message, serde_json::Error>;
    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    println!("Hello, world!");
    env_logger::init();

    let address = env::var("PULSAR_ADDRESS").ok().unwrap_or_else(|| {
        "pulsar://127.0.0.1:6650".to_string()
    });
    let topic = env::var("PULSAR_TOPIC").ok().unwrap_or_else(|| {
        "my-topic".to_string()
    });

    let mut builder = Pulsar::builder(address, TokioExecutor);
    if let Ok(token) = env::var("PULSAR_TOKEN") {
        let authentication = Authentication {
            name: "token".to_string(),
            data: token.into_bytes(),
        };

        builder = builder.with_auth(authentication);
    }

    // 通过build的方式创建pulsar object
    let pulsar_obj: Pulsar<_> = builder.build().await?;

    // create consumer
    let mut consumer: Consumer<Message, _> = pulsar_obj.consumer()
        .with_topic(topic)
        .with_consumer_name("group-2") // 设置消费组名字
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("my_topic test")
        .build()
        .await?;

    println!("consumer has run...");
    let mut counter: usize = 0;
    while let Some(msg) = consumer.try_next().await? {
        info!("metadata:{:?}",msg.message_id());
        info!("id:{:?}",msg.message_id());
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(err) => {
                error!("could not deserialize message:{:?}",err);
                break;
            }
        };

        // 消费消息逻辑
        println!("data:{}", data.data.as_str());

        // 消息ack确认
        consumer.ack(&msg).await?;
        counter += 1;
        info!("got {} messages",counter);
    }

    Ok(())
}

/*
Finished dev [unoptimized + debuginfo] target(s) in 0.12s
 Running `target/debug/pulsar-consumer`
Hello, world!
consumer has run...
data:hello
data:hello
data:hello
data:hello
 */