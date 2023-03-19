# log库使用
日志level 优先级  error > warn > info > debug > trace

# log门面定义
```rust
// 它定义了一个 Log 特征：
pub trait Log: Sync + Send {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool;
    fn log(&self, record: &Record<'_>);
    fn flush(&self);
}
```
- enabled 用于判断某条带有元数据的日志是否能被记录，它对于 log_enabled! 宏特别有用
- log 会记录 record 所代表的日志
- flush 会将缓存中的日志数据刷到输出中，例如标准输出或者文件中

目前来说，已经有了不少日志库实现，官方也推荐了一些 ，大家可以根据自己的需求来选择，不过 env_logger 是一个相当不错的选择。
log 还提供了 set_logger 函数用于设置日志库，set_max_level 用于设置最大日志级别，但是如果你选了具体的日志库，
它往往会提供更高级的 API，无需我们手动调用这两个函数。
例如下面的 env_logger 就是如此。

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



