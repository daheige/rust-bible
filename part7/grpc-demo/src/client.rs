use hello::greeter_service_client::GreeterServiceClient;
use hello::HelloReq;

// tonic request
use tonic::Request;

mod hello {
    tonic::include_proto!("App.Grpc.Hello"); // 必须和hello.proto package一样
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(HelloReq {
        id: 1,
        name: "daheige".into(),
    });

    let mut client = GreeterServiceClient::connect("http://127.0.0.1:8081").await?;
    println!("client:{:?}", client);

    let response = client.say_hello(request).await?;
    println!("res:{:?}", response);

    // 由于response 对外是私有的，这里通过 into_inner 转换为公开字段
    let res = response.into_inner();
    println!("name:{}", res.name);
    println!("message:{}", res.message);
    Ok(())
}
