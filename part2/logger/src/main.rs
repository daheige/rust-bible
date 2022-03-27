use log::{info, error, warn, debug, trace};
// 可以按需加载不同的级别
use env_logger;
use std::env;

fn main() {
    println!("Hello, world!");
    // 这里的logger是对应rust crate包名
    // 设置最低日志级别为debug
    // 日志level 优先级  error > warn > info > debug > trace
    //
    // 可以在运行时候指定
    // RUST_LOG=path::to_module=log_leve 多个用逗号隔开
    // 比如 RUST_LOG=log_dem=info,abc=debug
    //
    // 如果是运行指定，下面的代码需要注释掉
    env::set_var("RUST_LOG", "logger=debug"); // 手动设置环境变量 通过调用函数设置环境变量
    env_logger::init();
    // 初始化操作
    info!("{}","abc");
    error!("1111");
    warn!("warn 222");
    debug!("debug 222");
    trace!("debug 222");
}

#[cfg(test)]
mod tests {
    use env_logger::{Builder, Target};
    use std::env;
    use log::{info, error, warn, debug, trace};

    #[test]
    fn config_log_output() {
        env::set_var("RUST_LOG", "logger=debug");
        let mut builder = Builder::from_default_env();
        builder.target(Target::Stdout);
        builder.init();
        info!("{}","abc");
        error!("1111");
        warn!("warn 222");
        debug!("debug 222");
        trace!("debug 222");
    }
}
