# clap 子命令--subcommand
    % ./target/debug/clap-regular-use -p=1234 -n=daheige
    clap-regular-use 0.1.0
    
    USAGE:
    clap-regular-use [OPTIONS] --port <PORT> --name <NAME> <SUBCOMMAND>
    
    OPTIONS:
    -c, --count <COUNT>    [default: 1]
    -h, --help             Print help information
    -n, --name <NAME>      name of app
    -p, --port <PORT>      port of app
    -V, --version          Print version information
    
    SUBCOMMANDS:
    add     Adds files to myapp
    help    Print this message or the help of the given subcommand(s)

# 开始运行
    % ./target/debug/clap-regular-use -p=1234 -n=daheige add test
    cli param: Cli { command: Add { name: Some("test") }, port: 1234, name: "daheige", count: 1 }
    'myapp add' was used, name is: Some("test")
