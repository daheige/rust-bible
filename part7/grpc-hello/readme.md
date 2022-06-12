# grpc-hello
+ 安装protoc 参考grpc-demo
+ 安装protoc-gen-rust插件
```shell

$ cargo install protobuf
$ cargo install protobuf-codegen
```
+ 安装protoc-gen-rust-grpc插件
```shell
$ cargo install grpc-compiler
Installing /Users/heige/.cargo/bin/protoc-gen-rust-grpc
Installed package `grpc-compiler v0.8.3` (executable `protoc-gen-rust-grpc`)
```

# 运行server
```shell
cd ../grpc-demo
    % cargo run --bin grpc-demo
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/grpc-demo`
    grpc server run:127.0.0.1:8081
```

# 运行client
```shell
    cargo run --bin grpc-hello-client
    name: "daheige" message: "hello,rust grpc"
```
