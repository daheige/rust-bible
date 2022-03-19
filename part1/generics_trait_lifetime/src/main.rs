use std::cmp::PartialOrd;
use std::fmt::Display;
use std::ops::Deref;

// 结构体泛型 T是任意类型
// <T>表示可以任意类型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 为泛型结构体添加方法
impl<T> Point<T> {
    // 关联方法（静态方法）不需要self作为参数
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    // where约定trait
    fn cmp_display(&self) where T: Display + PartialOrd {
        if self.x >= self.y {
            println!("the max is x = {}", self.x);
            return;
        }

        println!("the mas is y = {}", self.y);
    }
}


// 不同类型的T,U泛型结构体
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// 给泛型结构体定义方法
impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn new(x: T, y: U) -> Self {
        Self {
            x,
            y,
        }
    }

    // 混入类型，泛型结构体中的方法也是可以支持泛型函数的
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// 泛型枚举
// 对于标注库中的Option<T>实际上就是一个枚举类型 有值就是Some(T),没有值就是None
// 对于Result<T,E>也是一个泛型枚举结构
#[derive(Debug)]
enum Color<T> {
    Some(T),
    Nome,
}

// trait 定义,定义一组方法行为
// 也可以给出默认实现的某个方法
pub trait Summarizable {
    fn summary(&self) -> String;
    fn display(&self) {
        println!("[object] no impl display method");
    }
}

struct Art {
    title: String,
    author: String,
    content: String,
}

// impl trait名字 for 具体类型
// 实现Summarizable trait
impl Summarizable for Art {
    fn summary(&self) -> String {
        format!("author: {},title:{},content:{}", self.author, self.title, self.content)
    }

    // 覆盖默认实现
    fn display(&self) {
        println!("{}", self.summary());
    }
}

// art 构造函数new 是一个静态方法，可以直接调用，生成一个Art实例
impl Art {
    fn new(title: String, author: String, content: String) -> Self {
        Self {
            title, // 这里不需要写title: title，可以省略前面的title:
            author,
            content,
        }
    }
}

struct Weibo {
    title: String,
    content: String,
    read_count: i64,
}

impl Summarizable for Weibo {
    fn summary(&self) -> String {
        format!("{}-{}-{}", self.title, self.content, self.read_count)
    }
}

impl Weibo {
    fn new(title: String, content: String, read_count: i64) -> Self {
        Self {
            title,
            content,
            read_count,
        }
    }
}

// trait bounds 表示trait 约束
// 参数是T 它是一个Summarizable trait 就可以调用summary方法
pub fn notify<T: Summarizable>(item: T) {
    println!("news: {}", item.summary());
}

// 特征对象通过&dyn trait name 或&mut dyn trait来进行特征分发
// 可以创建多态api，这种方式被声明为实现了某个特征api,就是特征对象
// 特征对象实现实际上一个胖指针，并且是不定长类型，需要在引用符号后面使用
fn show_me(item: &dyn Summarizable) {
    println!("summary: {}", item.summary());
}

// 在函数中定义泛型
// 下面是一个i32 slice取得最大值
fn max(s: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < s.len() {
        if s[i] > s[max_index] {
            max_index = i;
        }
        i += 1;
    }

    s[max_index]
}

// T 指定trait
// PartialOrd trait 特征可以比较大小
fn max2<T: PartialOrd>(s: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < s.len() {
        if s[i] > s[max_index] {
            max_index = i;
        }

        i += 1;
    }

    &s[max_index]
}

// 比较操作trait
trait Comparable {
    fn cmp(&self, obj: &Self) -> i8;
}

// 为浮点类型实现Comparable trait
impl Comparable for f64 {
    fn cmp(&self, obj: &Self) -> i8 {
        if &self > &obj {
            1
        } else if &self == &obj {
            0
        } else {
            -1
        }
    }
}

fn max3<T: Comparable>(s: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < s.len() {
        // 调用cmp 特征方法
        if s[i].cmp(&s[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }

    &s[max_index]
}


// 特征作为返回值
trait Descriptive {
    fn describe(&self) -> String {
        String::from("[object]")
    }

    fn output(&self);
}

struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("name:{} age:{}", self.name, self.age)
    }

    fn output(&self) {
        println!("{}", self.describe());
    }
}

// 当特征作为返回值，同一个函数中所有可能的返回值类型必须是同一个类型实现的trait才可以
fn create_person(name: String, age: u8) -> impl Descriptive {
    Person {
        name,
        age,
    }
}

fn main() {
    println!("Hello, world!");
    let p = Point { x: 1, y: 2 };
    let f = Point { x: 3.1, y: 1.3 };
    println!("p:{:?}", p);
    println!("f:{:?}", f);
    p.cmp_display();
    f.cmp_display();

    let p = Point2 { x: 1, y: 1.2 };
    println!("p.x = {},p.y = {}", p.x, p.y);

    let c = Color::Some(1);
    println!("{:?}", c);

    let c = Point2::new(1, 2.3);
    println!("x = {}", c.x());
    println!("y = {}", c.y);
    let c2 = Point2::new(2.1, 3);
    let c3 = c2.mixup(c);
    println!("c3.x = {}", c3.x());
    println!("c3.y = {}", c3.y);
    /**
    c3.x = 2.1
    c3.y = 2.3
     */

    let a = Art::new("hello,rust".to_string(), "heige".to_string(), "wolrd".to_string());
    a.display(); // 调用trait特征上面的方法

    let w = Weibo::new("hi,rust".to_string(), "from weibo".to_string(), 100);
    w.display();
    println!("summary: {}", w.summary());
    notify(w); // 调用notify方法

    let s = [1, 2, 3, 4];
    println!("max2 {}", max2(&s[..]));

    let s2 = [1.1, 1.3, 1.5, 1.2, 1.9, 1.8];
    println!("max2 float {}", max2(&s2[..]));

    // 通过cmp trait来比较
    let s3 = [1.1, 1.3, 1.5, 1.2, 1.9, 1.8];
    println!("max3 float {}", max3(&s3[..]));

    let p = create_person("heige".to_string(), 32);
    p.output();

    // trait object 特征对象动态分发规则
    // trait特征动态分发 通过关键字dyn来处理
    let a1 = Art::new("post1".to_string(), "heige".to_string(), "rust study".to_string());
    let a2 = Art::new("post2".to_string(), "daheige".to_string(), "rust study".to_string());
    let w = Weibo::new("weibo".to_string(), "hi rust".to_string(), 100);
    let s: Vec<&dyn Summarizable> = vec![
        &a1,
        &a2,
        &w,
    ];

    for v in s {
        show_me(v); // 调用动态分发的函数
    }
    /*
    summary:author: heige,title:post1,content:rust study
    summary:author: daheige,title:post2,content:rust study
    summary:weibo-hi rust-100
     */
}

// 下面这个函数是编译不通过，提示生命周期错误
// 提示返回值，需要一个泛型生命周期
// rust编译器，不知道是否要返回x,y其中的哪一个
// borrow checker 编译器的借用检查器，它比较作用域来确定所有的借用都是有效的
// 需要通过泛型生命周期参数来确定引用之间的作用域关系
// fn longest(x:&str,y:&str) -> &str{ // ^ expected named lifetime parameter
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }

// 生命周期注解的作用就是将多个引用的生命周期联系起来，方便借用检查器能确定引用的具体生命周期，也就是作用域的范围大小
// 生命周期参数名称通过'a 这种语法来标记，告诉rust多个引用的泛型生命周期参数如何相互联系
// 生命周期参数注解位于引用的&之后，并有一个空格来将引用类型与生命周期注解分割开
//
// &i32 表示一个借用
// &'a i32 // a reference with an explicit lifetime 引用生命周期注解
// &'a mut i32 // a mutable reference with an explicit lifetime 可变引用生命周期注解
//
// 在函数后面<'a>声明泛型生命周期，同时函数也返回与生命周期'a存在一样长的字符串slice
// 被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的一部分
// 下面的生命周期参数'a标注来返回的引用值,所以x,y中生命周期较短的那一个在结束之前保持有效
// 借用检查器认可这些代码；它能够编译和运行
// 返回值的生命周期与参数的生命周期需要关联
//
// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
// 而返回值的生命周期被称 为 输出生命周期（output lifetimes）。
// 输入型生命周期范围<=输出型生命周期范围
//
// 是否需要生命周期的规则：
// 1.每个引用的参数都有自己的生命周期，如果一个函数存在两个不同的生命周期，就需要声明'a,'b两个生命周期注解
// 2.如果只有一个输入生命周期参数，那么它就可以省略生命周期
// 3.如果方法有多个生命周期参数，方法的&self,&mut self，那么就可以省略
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中存在引用的话，需要给每个引用添加生命周期注解
// part存放了字符串slice,这是一个引用，类似于泛型参数类型，必须在结构体后面Tant后面添加<'a>声明泛型生命周期参数
// 这样就可以在结构体定义中使用生命周期参数
// 生命周期注解其实是一个泛型类型
// 每一个引用都有一个生命周期，需要为使用了引用的函数或结构体指定生命周期，这样rust编译器就可以成功编译并运行
struct Tant<'a> {
    part: &'a str,
}

// 方法定义中的生命周期注解'a
// impl 后面的<'a>生命周期参数必须要有的
impl<'a> Tant<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce(&self, announce: &str) -> &str {
        println!("attention:{}", announce);
        self.part
    }
}

// 静态生命周期 'static 生命周期存活在整个程序期间，直接存储在程序的二进制文件中
// 所有的字符串字面量都拥有 'static 生命周期
// 将一个引用声明为 'static 之前需要考虑清楚，是否要这么做
fn static_lifetime() {
    let s: &'static str = "hell rust"; // 这个字符量字符串，生命周期可以省略
    println!("s = {}", s);
}

// 结合泛型类型参数，trait bounds 和生命周期
// ann是一个泛型参数T，它需要实现Display trait 特征，通过where来实现泛型特征的绑定
// 由于生命周期'a也是泛型，所以和T都同时放在尖括号中，用来定义参数的作用域范围
fn longest_with_announce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("ann:{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 省略生命周期的场景,借用检查器可以自行推断生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 通过#[cfg(test)] 来指定下面的都是测试代码
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let s = super::longest("heige", "abc");
        println!("s = {}", s);
    }

    #[test]
    fn struct_lifetimes() {
        let s = super::Tant {
            part: "heige",
        };
        println!("part: {}", s.part);

        let a = s.announce("abc");
        println!("a = {}", a);
    }

    #[test]
    fn words() {
        println!("s = {}", super::first_word("hello rust")); // s = hello
    }

    #[test]
    fn test_longest_with_ann() {
        let s = super::longest_with_announce("hello", "rust", 12);
        println!("s = {}", s);
    }
}

