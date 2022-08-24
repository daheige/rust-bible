fn give_me<T>(value: T) {
    let _ = value;
    println!("generic_func define");
}

// 泛型结构体定义
#[derive(Debug)]
struct Container<T> {
    item: T,
}

// 为泛型结构体添加方法（泛型实现impl)
// 先要用impl<T>模式声明泛型参数，后使用T
impl<T: std::fmt::Display> Container<T> {
    fn new(item: T) -> Self {
        Self { item }
    }

    fn output(&self) {
        println!("item:{}", self.item);
    }
}

// 泛型枚举
#[derive(Debug)]
enum Transmission<T> {
    Signal(T),
    Nosignal,
}

fn main() {
    println!("Hello, world!");
    println!("generic func call");
    give_me(1);
    give_me("rust generic demo");

    // 泛型结构体初始化
    // c:Container { item: 1 }
    // enum t:Signal("ctrl+c")
    let c = Container { item: 1 };

    println!("c:{:?}", c);

    let t = Transmission::Signal("ctrl+c");
    println!("enum t:{:?}", t);

    let c2 = Container::new("daheige");
    c2.output();
}
