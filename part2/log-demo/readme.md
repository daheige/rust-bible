# log库使用
日志level 优先级  error > warn > info > debug > trace

# 指定日志级别运行
```shell
 % RUST_LOG=info ./target/debug/log-demo 
Hello, world!
[2023-03-19T09:45:41Z ERROR log_demo] this page not found
[2023-03-19T09:45:41Z INFO  log_demo] this a demo

// 指定为error级别的输出
% RUST_LOG=error ./target/debug/log-demo
Hello, world!
[2023-03-19T09:46:21Z ERROR log_demo] this page not found

 % RUST_LOG=debug ./target/debug/log-demo
Hello, world!
[2023-03-19T09:49:08Z DEBUG log_demo] this is a debug:message
[2023-03-19T09:49:08Z ERROR log_demo] this page not found
[2023-03-19T09:49:08Z INFO  log_demo] this a demo

% RUST_LOG=trace ./target/debug/log-demo
Hello, world!
[2023-03-19T09:49:33Z DEBUG log_demo] this is a debug:message
[2023-03-19T09:49:33Z ERROR log_demo] this page not found
[2023-03-19T09:49:33Z TRACE log_demo] this is trace
[2023-03-19T09:49:33Z INFO  log_demo] this a demo
```

# 自定义日志对象
默认情况下，env_logger 会输出到标准错误 stderr，如果你想要输出到标准输出 stdout，可以使用 Builder 来改变日志对象( target ):
```rust
use std::env;
use env_logger::{Builder, Target};
let mut builder = Builder::from_default_env();
builder.target(Target::Stdout);

builder.init();

// 默认

if cfg!(debug_assertions) {
    eprintln!("debug: {:?} -> {:?}", record, fields);
}
```

# env_logger用法
https://crates.io/crates/env_logger



