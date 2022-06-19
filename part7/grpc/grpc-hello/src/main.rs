use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use rust_grpc::hello::*;
use rust_grpc::hello_grpc::*;
use std::thread;

mod rust_grpc;

// 实现grpc service
struct GreeterServiceImpl;

impl GreeterService for GreeterServiceImpl {
    fn say_hello(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<HelloReq>,
        resp: ServerResponseUnarySink<HelloReply>,
    ) -> grpc::Result<()> {
        let name = req.message.name;
        let id = req.message.id;
        println!("id:{} name: {}", id, name);

        // 响应结果
        let mut rsp = HelloReply::new();
        rsp.set_message("hello,rust grpc demo".to_string());
        rsp.set_name(name);
        resp.finish(rsp)
    }
}

fn main() {
    let port = 8081;
    let mut server = grpc::ServerBuilder::new_plain();
    // let _ = server.http.set_addr("0.0.0.0:8081");
    let _ = server.http.set_port(port);

    // 注入grpc server服务
    server.add_service(GreeterServiceServer::new_service_def(GreeterServiceImpl));
    let _ = server.build().expect("build service failed");
    println!("greeter server started on port {} without tls", port,);

    // 防止主线程退出
    // 这里好像启动不起来，估计是grpc rust bug.
    // 推荐使用tonic
    loop {
        thread::park();
    }
}
