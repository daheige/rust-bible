use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

// 定义接口  其实 pub Interface = Any + Send + Sync
trait Logger: Interface {
    fn log(&self, content: &str);
}

// 定义日期格式logger
trait DateLogger: Interface {
    fn log_date(&self);
}

// 声明这是一个组件并且是为了实现logger
#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl;

// 实现logger
impl Logger for LoggerImpl {
    fn log(&self, content: &str) {
        println!("{}", content);
    }
}

// 动态接口，通过inject的方式注入进来
#[derive(Component)]
#[shaku(interface = DateLogger)]
struct DateLoggerImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    today: String,
    year: usize,
}

// 实现 DateLogger
impl DateLogger for DateLoggerImpl {
    fn log_date(&self) {
        self.logger
            .log(&format!("Today is {}, {}", self.today, self.year));
    }
}

// Component represents a single instance of a service, aka a singleton.
// Provider is more like a factory for instances.
// Each time a component is resolved you will get the same instance.
// Each time a provider is resolved you will get a new instance.
module! {
    MyModule {
        components = [LoggerImpl, DateLoggerImpl],
        providers = []
    }
}

fn main() {
    // 定义DateLoggerImpl 参数
    let di = MyModule::builder()
        .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
            today: "Jan 26".to_string(),
            year: 2022,
        })
        .build();

    // 解析components
    let date_logger: &dyn DateLogger = di.resolve_ref();
    date_logger.log_date();
}
