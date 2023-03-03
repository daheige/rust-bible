// 特征继承
// 和面向对象语言相比，特征及其实现类似接口和实现这些接口的类
pub trait Vehicle {
    fn get_price(&self) -> u64;
}

// 对于Car trait的具体实现者必须要实现Vehicle上的get_price方法
pub trait Car: Vehicle {
    fn model(&self) -> String;
}
