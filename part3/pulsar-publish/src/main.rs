use serde::{Serialize, Deserialize};
use serde_json;

// 引入pulsar
use pulsar::{
    message::proto, producer, Authentication, Error as PulsarError, Pulsar, SerializeMessage,
    TokioExecutor,
};

// 日志组件设置
use std::env;
use env_logger;

// 定义消息格式
#[derive(Serialize, Deserialize)]
struct Message {
    data: String,
}

impl SerializeMessage for Message {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input)
            .map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default() // 其他字段采用默认设置
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    // env::set_var("RUST_LOG", "pulsar-publish=debug");
    env_logger::init();
    let address = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());

    // 对于topic需要提前kafka建立好
    // 可以参考kafka sh命令使用
    // 也可以参考 go-god/broker/gpulsar 启动一个创建topic
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

    // 创建producer
    let mut producer = pulsar_obj
        .producer()
        .with_topic(topic)
        .with_name("my_producer")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    let mut counter: usize = 0;
    loop {
        let s = counter.to_string();

        // 发送消息
        producer
            .send(Message {
                data: "hello: ".to_string() + &s, // 发送的message内容是 {"data":"hello"}
            }).await?;

        counter += 1;
        println!("{} messages", counter);
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        if counter >= 100 {
            break;
        }
    }

    Ok(())
}