use log::{debug, error, info, log_enabled, trace, warn, Level};

use env_logger;

fn main() {
    println!("Hello, world!");
    // 初始化日志env logger
    env_logger::init(); // 由于log 库提供的是日志门面模式，具体实现交给了env_logger实现
    debug!("this is a debug:{}", "message");
    error!("this page not found");
    trace!("this is trace");
    if log_enabled!(Level::Info) {
        info!("this a demo");
    }
}
