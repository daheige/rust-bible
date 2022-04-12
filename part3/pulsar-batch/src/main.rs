use serde::{Deserialize, Serialize};
use serde_json;

// 引入pulsar
use pulsar::{
    message::proto, message::proto::command_subscribe::SubType, message::Payload, producer,
    Authentication, Consumer, DeserializeMessage, Error as PulsarError, Pulsar, SerializeMessage,
    TokioExecutor,
};

// 日志组件设置
use env_logger;
use log::{error, info};
use std::env;

// 引入futures
use futures::{future::join_all, TryStreamExt};

// 定义消息格式
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    data: String,
}

// 实现 Message SerializeMessage trait
impl SerializeMessage for Message {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default() // 其他字段采用默认设置
        })
    }
}

// 实现反序列化
impl DeserializeMessage for Message {
    // 执行输出的返回结果
    type Output = Result<Message, serde_json::Error>;
    fn deserialize_message(payload: &Payload) -> Self::Output {
        println!("data:{:?}", payload);
        serde_json::from_slice(&payload.data)
    }
}

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    env_logger::init();
    let address = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "my-topic".to_string());

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
    let mut producer_obj = pulsar_obj
        .producer()
        .with_topic("my-topic")
        .with_name("my_producer2")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    // let mut producer_obj = pulsar_obj.producer()
    //     .with_topic("my-topic")
    //     .with_name("my_producer2")
    //     .with_options(producer::ProducerOptions {
    //         // batch message size
    //         batch_size: Some(4),
    //         // compression: Some(proto::CompressionType::Snappy),
    //         ..Default::default()
    //     }).build().await?;

    // check producer connection
    producer_obj
        .check_connection()
        .await
        .map(|_| println!("producer connection ok"))?;

    // 开启独立线程发送消息
    tokio::task::spawn(async move {
        let mut counter: usize = 0;
        let mut v = Vec::new();
        loop {
            let s = counter.to_string();
            println!("will send msg");
            let receipt_rx = producer_obj
                .send(Message {
                    data: "hello: ".to_string() + &s,
                })
                .await
                .unwrap();

            v.push(receipt_rx);
            println!("sent msg");
            counter += 1;
            if counter % 100 == 0 {
                println!("send {} messages", counter);
                break;
            }
        }

        println!("receipts:{:?}", join_all(v).await);
    });

    // 创建消费者
    // create consumer
    let mut consumer: Consumer<Message, _> = pulsar_obj
        .consumer()
        .with_topic(topic)
        .with_consumer_name("group-2") // 设置消费组名字
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("my_topic test")
        .build()
        .await?;

    println!("consumer has run...");
    let mut counter: usize = 0;
    while let Some(msg) = consumer.try_next().await? {
        info!("metadata:{:?}", msg.message_id());
        info!("id:{:?}", msg.message_id());
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(err) => {
                error!("could not deserialize message:{:?}", err);
                continue;
            }
        };

        // 消费消息逻辑
        println!("data:{}", data.data.as_str());

        // 消息ack确认
        consumer.ack(&msg).await?;
        counter += 1;
        info!("got {} messages", counter);
        if counter % 100 == 0 {
            break;
        }
    }

    Ok(())
}
