# 关于生命周期lifetimes

官方权威的lifetimes说明：

- https://github.com/rust-lang/rfcs/blob/master/text/2115-argument-lifetimes.md
- https://github.com/rust-lang/rfcs/blob/master/text/0141-lifetime-elision.md
- https://github.com/rust-lang/rfcs/blob/master/text/0556-raw-lifetime.md

lifetimes进一步解释：

- 生命周期纯粹是一个编译期构造，它可以帮助编译器确定某个引用有效的作用域，并
  确保它遵循借用规则。
- 它可以跟踪诸如引用的来源，以及它们是否比借用值生命周期更长这类事情。
- Rust 中的生命周期能够确保引用的存续时间不超过它指向的值。
- 生命周期并不是你作为开发人员将要用到的，而是编译器使用和推断引用的有效性时会用到的。
- 对于rust编译器在编译的时候，如果不能推断变量的作用范围，这个时候就必须显式的标注其生命周期的范围，也就是采用'a,'
  b这样的生命周期注解来辅助编译器

# 显式标注生命周期的场景

当 Rust 无法为我们代劳时，有很多地方需要用户指定生命周期，主要体现这些地方：

- 函数签名
- 结构体和结构体中包含引用任何类型的字段
- impl 代码块

```rust
struct FooRef<'a, T> {
    value: &'a T, // 生命周期用'a的方式来标注
}

// 省略生命周期标注
fn get_one(x: &u8) -> &u8 {
    let y = x + 1;
    println!("y = {}", y);
    &x
}

/*
输入型生命周期：函数参数上的生命周期注释当作引用时被称为输入型生命周期。
输出型生命周期：函数返回值上的生命周期参数当作引用时被称为输出型生命周期。
任何输出型生命周期都源自输入型生命周期，我们不能拥有独立于输
入型生命周期的输出型生命周期。
它只能是一个小于或等于输出型生命周期的生命周期
 */

/* 下面的有2个输入型生命周期，但是输出型生命周期只有1个，无法编译，抛出错误
在这种情况下，返回值的生命周期并不明显，因为涉及两个输入引用
Rust 无法确定返回值的生命周期，它需要我们的帮助
当 Rust 无法为我们代劳时，有很多地方需要用户指定生命周期。
• 函数签名
• 结构体和结构体字段
• impl 代码块
fn foo(x: &str, y: &str) -> &str {
    x
}
fn foo(x: &str, y: &str) -> &str {
   |           ----     ----     ^ expected named lifetime parameter
*/

// 结构体中包含引用任何类型的字段时候，需要明确指定这些引用的生命周期
#[derive(Debug)]
struct NumberInfo<'a> {
    n: &'a i64,
}

// impl代码块中的生命周期标注
// 先在impl声明，后使用
impl<'a> NumberInfo<'a> {
    fn new(n: &'a i64) -> Self {
        Self { n }
    }

    fn get_num(&self) -> &'a i64 {
        self.n
    }
    fn set_num(&mut self, number: &'a i64) {
        self.n = number
    }
}

// 多个生命周期
// 假设我们在使用一个解码器，它根据模式和给定的已编码字节流来解析二进制文件
struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R,
}

// 在实现impl的时候也要指定'a,'b，S,R这些泛型参数，先声明后使用
impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
    where
        S: std::fmt::Display,
        R: std::fmt::Display,
        'a: 'b, // 这里是指定'a的生命周期比'b长
{
    fn new(schema: &'a S, reader: &'b R) -> Self {
        Self { schema, reader }
    }

    fn get_schema(&self) -> &'a S {
        self.schema
    }

    fn do_reader(&self) {
        println!("current schema:{}", self.schema);
        println!("reader:{}", self.reader);
    }
}

// 在泛型上声明生命周期区间通过where来处理
#[derive(Debug)]
enum Level {
    Error,
    Info,
}

#[derive(Debug)]
struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(t: T)
    where
        T: Send + 'static + std::fmt::Debug,
{
    println!("t:{:?}", t);
}

fn main() {
    println!("Hello, world!");
    let foo = FooRef { value: &23 };
    println!("foo.value = {}", &foo.value);

    let x = get_one(&1);
    println!("x = {}", x);

    // 结构体中包含引用时候的生命周期使用
    let number_info = NumberInfo { n: &1234 };
    println!("n = {}", &number_info.n);

    let mut n2 = NumberInfo::new(&12345); // 这里必须给n2指定mut可变引用，不然下面无法调用set_num修改n
    n2.set_num(&12);
    println!("n2.n = {}", &n2.get_num());

    let schema = String::from("http");
    let d = Decoder::new(&schema, &2);
    println!("schema:{}", d.get_schema());
    d.do_reader();

    let name = "common";
    let log = Logger(name, Level::Info);
    configure_logger(log); // t:Logger("common", Info)
}

```

# 结构体包含引用任何类型的字段

```rust
// 结构体中包含引用任何类型的字段时候，需要明确指定这些引用的生命周期
struct Foo<'a> {
    i: &'a i32
}

fn main() {
    let x = 42;
    let foo = Foo {
        i: &x
    };
    println!("{}", foo.i);
}

```

# 'static 静态生命周期

- 一个静态变量是一个在编译期间即被创建并存在于整个程序始末的内存资源。他们必须被明确指定类型。
- 一个静态生命周期是指一段内存资源无限期地延续到程序结束。需要注意的一点是，在此定义之下，一些静态生命周期的资源也可以在运行时被创建。
- 拥有静态生命周期的资源会拥有一个特殊的生命周期注解 `'static`。 `'static` 资源永远也不会被 drop 释放。
- 如果静态生命周期资源包含了引用，那么这些引用的生命周期也一定是 'static 的。（任何缺少了此注解的引用都不会达到同样长的存活时间）

内存细节：

- 因为静态变量可以全局性地被任何人访问读取而潜在地引入数据争用，所以修改它具有内在的危险性
- Rust 允许使用 unsafe { ... } 代码块来进行一些无法被编译器担保的内存操作

```rust
// 定义静态生命周期的额变量，在定义的时候必须指定类型
static PI: f64 = 3.1415;

fn main() {
    // 静态变量的范围也可以被限制在一个函数内
    static mut SECRET: &'static str = "swordfish";

    // 字符串字面值拥有 'static 生命周期
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // 你可以打破一些规则，但是必须是显式地使用unsafe来改变static静态变量的值
    unsafe {
        // 我们可以修改 SECRET 到一个字符串字面值因为其同样是 'static 的
        SECRET = "abcddd";
        println!("{}", SECRET);
    }
}
```
