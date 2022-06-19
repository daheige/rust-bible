# clap
- clap是一个简单易用，功能强大的命令行参数解析库
- clap 允许多中方式指定我们的命令行。支持常规的 Rust 方法调用、宏或者YAML配置

# 使用方式
    % cargo build
    % ./target/debug/clap-demo -h
    clap-demo 0.1.0
    
    USAGE:
    clap-demo [OPTIONS] --port <PORT> --name <NAME>
    
    OPTIONS:
    -c, --count <COUNT>    [default: 1]
    -h, --help             Print help information
    -n, --name <NAME>      name of app
    -p, --port <PORT>      port of app
    -V, --version          Print version information
    % ./target/debug/clap-demo -p=123 -n=daheige
    AppService { port: 123, name: "daheige", count: 1 }
