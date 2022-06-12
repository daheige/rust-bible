use futures::executor;
use grpc::ClientStubExt;
use rust_grpc::hello::*;
use rust_grpc::hello_grpc::*;

mod rust_grpc;

fn main() {
    let client = GreeterServiceClient::new_plain("127.0.0.1", 8081, Default::default()).unwrap();
    let mut req = HelloReq::new();
    req.id = 1;
    req.name = "daheige".to_string();
    let resp = client
        .say_hello(grpc::RequestOptions::new(), req)
        .join_metadata_result();
    let resp = executor::block_on(resp);
    match resp {
        Err(err) => println!("get error: {:?}", err),
        Ok((_, r, _)) => println!("{:?}", r),
    }
}
