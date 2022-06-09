# rust grpc
    grpc使用 采用tokio,tonic,tonic-build 和 prost 进行构建

# centos7 protoc工具安装

    1、下载https://github.com/protocolbuffers/protobuf/archive/v3.15.8.tar.gz
        cd /usr/local/src
        sudo wget https://github.com/protocolbuffers/protobuf/archive/v3.15.8.tar.gz
    
    2、开始安装
        sudo mv v3.15.8.tar.gz protobuf-3.15.8.tar.gz
        sudo tar zxvf protobuf-3.15.8.tar.gz
        cd protobuf-3.15.8
        sudo yum install gcc-c++ cmake libtool
        # 对于ubuntu系统 sudo apt install gcc cmake make libtool
        $ sudo mkdir /usr/local/protobuf

        需要编译, 在新版的 PB 源码中，是不包含 .configure 文件的，需要生成
        此时先执行 sudo ./autogen.sh 
        脚本说明如下:
        # Run this script to generate the configure script and other files that will
        # be included in the distribution. These files are not checked in because they
        # are automatically generated.

        此时生成了 .configure 文件，可以开始编译了
        sudo ./configure --prefix=/usr/local/protobuf
        sudo make && make install

        安装完成后,查看版本:
        $ cd /usr/local/protobuf/bin
        $ ./protoc --version
        libprotoc 3.15.8
        
        建立软链接
        $ sudo ln -s /usr/local/protobuf/bin/protoc /usr/bin/protoc
        $ sudo chmod +x /usr/bin/protoc

# 开始rust grpc

cargo new grpc-demo

1.新建src/client.rs
```rust
fn main() {}
```

2.在src同级目录新建build.rs文件，添加如下内容：
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    Ok(())
}
```

3.添加依赖
```toml
[[bin]]
name = "grpc-demo-client"
path = "src/client.rs"

[dependencies]
tonic = "0.7.2"
prost = "0.10.4"
tokio = {version = "1",features = ["full"]}

[build-dependencies]
tonic-build = "0.7.2"
```
4. cargo run --bin grpc-demo
这一步就会安装好所有的依赖，并构建proto/hello.proto

5. 在src/main.go中添加rust grpc server代码
```rust
// 生成的 greeter_service_server mod
// 然后生成 GreeterService trait 和 GreeterServiceServer
use hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use hello::{HelloReq,HelloReply};

use tonic::{transport::Server, Request, Response, Status};

// 指定protobuf文件的rust package
// 这些内容放在 grpc-demo/target/debug/build/grpc-demo-991bac770132394e/out/app.grpc.hello.rs
// 可以自己查看生成的grpc rust代码
mod hello {
    tonic::include_proto!("App.Grpc.Hello"); // 必须和hello.proto package一样
}

// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl GreeterService for GreeterImpl {
    // 实现async_hello方法
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloReply>, Status> {
        println!("got a request:{:?}", request);
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
    println!("Hello, world!");
    let address = "[::1]:8081".parse()?;
    let greeter = GreeterImpl::default();

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(address)
        .await?;

    Ok(())
}

```
6.添加client.rs代码
```rust
use hello::greeter_service_client::GreeterServiceClient;
use hello::{HelloReply, HelloReq};

// tonic request
use tonic::{Request, Response};

mod hello {
    tonic::include_proto!("App.Grpc.Hello"); // 必须和hello.proto package一样
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(HelloReq {
        id: 1,
        name: "daheige".into(),
    });

    let mut client = GreeterServiceClient::connect("http://[::1]:8081").await?;
    println!("client:{:?}", client);

    let response = client.say_hello(request).await?;
    println!("res:{:?}", response);

    // 由于response 对外是私有的，这里通过 into_inner 转换为公开字段
    let res = response.into_inner();
    println!("name:{}", res.name);
    println!("message:{}", res.message);
    Ok(())
}

```

# grpc go grpc demo
https://github.com/daheige/gmicro-demo

# rust grpc参考
https://cloud.tencent.com/developer/article/1669569
