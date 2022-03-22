use log::{info, error, warn, debug, trace};
// 可以按需加载不同的级别
use env_logger;
use std::env;

fn main() {
    println!("Hello, world!");
    // 这里的logger是对应rust crate包名
    // 设置最低日志级别为debug
    // 日志level 优先级  error > warn > info > debug > trace
    // 手动设置环境变量
    env::set_var("RUST_LOG", "logger=debug"); // 通过调用函数设置环境变量
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
        let mut buidler = Builder::from_default_env();
        buidler.target(Target::Stdout);
        buidler.init();
        info!("{}","abc");
        error!("1111");
        warn!("warn 222");
        debug!("debug 222");
        trace!("debug 222");
    }
}
