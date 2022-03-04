#![allow(unused_variables)]
use std::fmt::Display;
// Rust在编译的时候会扫描代码，变量声明后未使用会以warning警告的形式进行提示），
// 引入#![allow(unused_variables)]属性标记，该标记会告诉编译器忽略未使用的变量，
// 不要抛出warning警告
type File = String; // 类型别名

#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    sign_in_count: i64,
}

fn build_user(name: String, active: bool, sign_in_count: i64) -> User {
    User {
        name,
        active,
        sign_in_count,
    }
}

// 给结构体定义方法
// 使用方法代替函数有以下好处：
//
//     不用在函数签名中重复书写 self 对应的类型
//     代码的组织性和内聚性更强，对于代码维护和阅读来说，好处巨大

impl User {
    // &self作为第一个参数，表示借用self,只读，不获取self的所有权
    fn print_name(&self) {
        println!("current name = {}", self.name);
    }

    // 关联函数，它没有 self，相当于静态方法，可以直接调用
    fn new(name: String, active: bool, sign_in_count: i64) -> Self {
        Self {
            // 这里Self实际上就是结构体本身User名字
            name,
            active,
            sign_in_count,
        }
    }
}

#[derive(Debug)] // 可以格式化打印
struct Color(i32, i32, i32); // 元组结构体

// 枚举类型
enum Poker {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn main() {
    println!("Hello, world!");

    println!("add(1) = {}", add(1));

    let mut f1 = File::from("abc");
    open(&mut f1);

    let mut v: Vec<u8> = vec![];
    // read(&mut f1, &mut v);

    close(&mut f1);

    // 字符串字面量是切片，实际上，s的类型是&str
    // &str是一个不可变引用
    let name = "daheige";
    println!("name = {}", name);

    // 字符串切片的类型标识是&str
    // 就字符串字面值来说，我们在编译时就知道其内容，
    // 最终字面值文本被直接硬编码进可执行文件中，
    // 这使得字符串字面值快速且高效，这主要得益于字符串的不可变性
    /* String 底层是一个u8类型的容器，包含cap,len,ptr指针
        pub struct String {
        vec: Vec<u8>,
    }

    对于 String 类型，为了支持一个可变、可增长的文本片段，
    需要在堆上分配一块在编译时未知大小的内存来存放内容，
    这些都是在程序运行时完成的：

    首先向操作系统请求内存来存放String对象
    在使用完成后，将内存释放，归还给操作系统

         */

    let s = String::from("hello rust");
    let s1 = &s[0..5]; // s1指向s切片的一部分
    println!("s1 = {}", s1); // s1 = hello
    println!("len(s) = {}", s.len()); // String的长度

    // Rust中的字符是Unicode类型，因此每个字符占据4个字节内存空间，
    // 但是在字符串中不一样，字符串是UTF8编码，也就是字符所占的字节数
    // 是变长的(1-4)，这样有助于大幅降低字符串所占用的内存空间

    // Rust在语言级别，只有一种字符串类型：str，它通常是以引用类型出现&str，
    // 也就是上文提到的字符串切片。虽然语言级别只有上述的str类型，但是在标准库里，
    // 还有多种不同用途的字符串类型，其中使用最广的即是String类型

    // str类型是硬编码进可执行文件，也无法被修改，但是String则是一个可增长、
    // 可改变且具有所有权的UTF8编码字符串，当Rust用户提到字符串时，
    // 往往指的就是String类型和&str字符串切片类型，这两个类型都是UTF8编码

    let mut s = String::new();
    s.push_str("hello,world");
    s.push('!');
    println!("s = {}", s);

    // 从现有的&str切片创建String类型
    let mut s = "hello".to_string();
    s.push_str(",abc");
    println!("s = {}", s);

    // 将String类型转为&str类型
    let s = String::from("hello,world");
    println!("current s = {}", &s);

    // rust中字符串是utf8格式
    let s = "好好学习rust";
    for c in s.chars() {
        println!("current char = {}", c);
    }

    // 字节
    for b in s.bytes() {
        println!("b = {}", b);
    }

    // 变量在离开作用域后，就自动释放其占用的内存
    {
        let local_s = String::from("abc");
        println!("local_s = {}", local_s);
    } // / 此作用域已结束，local_s 不再有效，内存被释放 Rust则在变量离开作用域时，自动调用drop函数
      // println!("local_s = {}", local_s); // ^^^^^^^ not found in this scope

    // 整数切片
    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3];
    println!("slice = {:?}", s); // slice = [2, 3] 调试模式打印

    // 元组 里面的元素可以是不同的类型
    let tup: (i32, f64, u8) = (500, 1.2, 1);
    println!("tup = {:?}", tup); // tup = (500, 1.2, 1)

    let (x, y, z) = tup; // 变量解构
    println!("x = {},y = {},z ={}", x, y, z);

    println!("访问tup.0 = {}", tup.0);
    println!("{:?}", cal("heige".to_string())); // ("heige", 5)

    let u = build_user("daehige".to_string(), true, 1);
    // user = User { name: "daehige", active: true, sign_in_count: 1 }
    println!("user = {:?}", u);

    let u2 = User {
        name: "heige".to_string(), // 除了这个字段外，其他字段用u的字段填充
        ..u
    };
    println!("u.active = {}", u.active);
    println!("user = {:?}", u);
    println!("user2 = {:?}", u2);

    let user1 = User {
        name: String::from("heige"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        name: user1.name,
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    // 下面这行会报错
    // println!("{:?}", user1) // ^^^^^ value borrowed here after partial move
    let u = User::new("heige".to_string(), true, 10);
    u.print_name();

    let c = Color(1, 23, 235);
    println!("c = {:?}", c);
    let p = Poker::Clubs; // 枚举通过match来枚举
    match p {
        Poker::Clubs => println!("match success"),
        _ => println!("not match"),
    }

    // Option枚举用于处理空值
    /**
    enum Option<T> {
    Some(T), // 表示有值,T是泛型参数，Some(T)表示该枚举成员的数据类型是T
    None, // 没有值
    }*/
    let some_number = Some(5);
    // 对于option采用match来匹配
    match some_number {
        Some(number) => println!("number = {}", number),
        None => println!("not found value"),
    }
    // 第二种匹配模式if let Some
    if let Some(number) = some_number {
        println!("number: {}", number);
    }

    // array
    let arr = [1, 2, 3, 4, 5]; // 数组的元素类型要一样
    println!("arr = {:?}", arr);

    let arr: [i32; 3] = [1, 2, 3]; // 声明的时候指定类型
    println!("arr2 = {:?}", arr);
    println!("arr[0]= {}", arr[0]);
    // 通过数组生成切片
    let s: &[i32] = &arr[1..3];
    println!("s = {:?}", s); // s = [2, 3] 切片cap,len,ptr指向数组底层的某个元素开始位置的内存地址
    println!("arr = {:?}", arr);

    let p = Point::new(1, 2);
    p.print();
}

// 函数定义关键字fn，返回值通过->来定义
fn add(mut i: i32) -> i32 {
    i += 1;
    i
}

// 元组在函数返回值场景很常用，例如下面的代码，可以使用元组返回多个值:
fn cal(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回了字符串所有权和长度usize
}

#[allow(dead_code)]
// 返回一个!，这个表明该函数是一个发散函数，不会返回任何值，包括()
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // unimplemented!()告诉编译器该函数尚未实现
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// 泛型
// 泛型的性能：
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// 定义泛型结构体，这里的T是任意类型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// impl<T>本质上就是为指定类型T为任意类型
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 通过trait特征约束来实现display
// 使用特征约束有条件的实现方法或特征
impl<T: Display> Point<T> {
    fn print(&self) {
        println!("point.x = {},point.y = {}", self.x, self.y);
    }
}

// 为特定类型的泛型结构体定义方法
impl Point<i32> {
    fn print2(&self) {
        println!("point.x = {},point.y = {}", self.x, self.y);
    }
}
