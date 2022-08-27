/*
生命周期纯粹是一个编译期构造，它可以帮助编译器确定某个引用有效的作用域，并
确保它遵循借用规则。它可以跟踪诸如引用的来源，以及它们是否比借用值生命周期更长
这类事情。Rust 中的生命周期能够确保引用的存续时间不超过它指向的值。生命周期并不
是你作为开发人员将要用到的，而是编译器使用和推断引用的有效性时会用到的。
*/
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
