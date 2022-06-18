use std::sync::Arc;

// 手动处理依赖问题
trait Logger {
    fn write(&self, content: String);
}

trait DateLogger {
    fn write_date(&self);
}

struct LoggerImpl;

// 实现 Logger
impl Logger for LoggerImpl {
    fn write(&self, content: String) {
        println!("content:{}", content);
    }
}

struct DateLoggerImpl {
    output: Arc<dyn Logger>, // 这里是将 Logger trait object动态分发，通过arc保证线程安全操作
    today: String,
    year: usize,
}

// 实现 DateLogger
impl DateLogger for DateLoggerImpl {
    fn write_date(&self) {
        self.output
            .write(format!("year:{},{}", self.year, self.today))
    }
}

fn main() {
    let today_logger = DateLoggerImpl {
        output: Arc::new(LoggerImpl), // 先初始化
        today: "Jan 26".to_string(),
        year: 2022,
    };

    today_logger.write_date();
}
