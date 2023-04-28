// trait object 特征对象是 Rust 执行动态分发的方式，我们没有实际的具体类型信息。通过跳转到
// vtable 并调用适当的方法完成方法解析。特征对象的另一个用例是，它们允许用户对可以
// 具有多种类型的集合进行操作，但是在运行时需要额外的间接指针引用开销

use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

// 这里定义的Area trait继承了Debug特征，然后接着定义了一个方法get_area
trait Area: Debug {
    fn get_area(&self) -> f32;
}

// 实现Area trait
impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

// 将trait object作为函数的参数使用，属于动态分发处理
// 这里将特征对象传递给函数
fn show_me(item: &dyn Display) {
    println!("{}", item);
}

// 将 impl trait作为参数，这种仅仅是实现了Display才可以传递给参数
// 通常建议将特征区间的 impl 特征语法用做函数的返回类型，比如说下面的lazy_add函数。
// 在参数位置使用它意味着我们不能使用 turbofish 运算符。
// 如果某些相关代码使用 turbofish 运算符来调用软件包中的
// 某个方法，那么可能导致 API 不兼容。
// 对于impl trait 这种，只有当我们没有可用的具体类型时才应该使用它，就像闭包那样。
fn display_me(item: impl Display) {
    println!("{}", item);
}

// 函数返回一个trait
fn lazy_add(a: i32, b: i32) -> impl Fn() -> i32 {
    move || a + b
}

fn main() {
    println!("Hello, world!");
    // trait dispatch 特征对象分发
    // 必须使用&dyn来进行修饰trait,否则运行时候抛出错误doesn't have a size known at compile-time
    // 特征对象是 Rust 执行动态分发的方式
    // 特征对象是由 dyn xxx 表示的，意味着它是指向 Area 特征某些实现的指针。特征对象形式的类型允许
    // 用户在集合类型（例如 Vec）中存储不同类型
    //
    // 请注意，我们只能创建在编译时知道类型尺寸的特征对象。
    // dyn Trait是一个不定长类型，只能作为引用创建。
    // 我们还可以通过将特征对象置于其他指针类型之后来创建特征对象，例如 Box、Rc、Arc 等
    let shapes: Vec<&dyn Area> = vec![&Square(1.2), &Rectangle(1.1, 1.2)];
    for s in shapes {
        println!("area:{}", s.get_area());
    }

    // show_me("abc"); // ^^^^^ doesn't have a size known at compile-time
    // 将特征对象传递给函数，必须通过引用的方式使用，因为它是动态分发机制
    show_me(&"abc");

    // 通过impl trait 参数传递的方式调用函数，跟上面的show_me是不一样的，display_me是impl trait
    display_me("abc");

    let c = lazy_add(1, 2);
    println!("result:{}", c());
}
