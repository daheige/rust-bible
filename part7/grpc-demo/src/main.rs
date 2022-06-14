use rust_grpc::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use rust_grpc::hello::{HelloReply, HelloReq};

use tonic::{transport::Server, Request, Response, Status};

// 定义grpc代码生成的包名
mod rust_grpc;

// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl GreeterService for GreeterImpl {
    // 实现async_hello方法
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloReply>, Status> {
        // println!("got a request:{:?}", &request);
        let reply = HelloReply {
            message: format!("hello,rust grpc"),
            // 由于gRPC请求和响应中的字段都是私有的，所以需要使用 .into_inner()
            name: format!("{}", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

// 采用tokio 运行时来跑grpc server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:8081".parse()?;
    println!("grpc server run:{}", address);

    let greeter = GreeterImpl::default();
    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(address)
        .await?;

    Ok(())
}
