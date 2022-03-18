// 结构体泛型 T是任意类型
// <T>表示可以任意类型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
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

    fn display(&self) {
        println!("{}",self.summary());
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

struct Weibo{
    title: String,
    content: String,
    read_count: i64,
}

impl Summarizable for Weibo{
    fn summary(&self) -> String {
        format!("{}-{}-{}",self.title,self.content,self.read_count)
    }
}

impl Weibo {
    fn new(title :String,content:String,read_count: i64) -> Self{
        Self{
            title,
            content,
            read_count,
        }
    }
}

// trait bounds 表示trait 约束
// 参数是T 它是一个Summarizable trait 就可以调用summary方法
pub fn notify<T:Summarizable>(item:T){
    println!("news: {}",item.summary());
}

// 在函数中定义泛型
// 下面是一个i32 slice取得最大值
fn max(s:&[i32]) -> i32{
    let mut max_index = 0;
    let mut i = 1;
    while  i < s.len() {
        if s[i] > s[max_index]{
            max_index = i;
        }
        i+=1;
    }

    s[max_index]
}

// T 指定trait
fn max2<T:std::cmp::PartialOrd>(s :&[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while  i < s.len() {
        if s[i] > s[max_index]{
            max_index = i;
        }

        i+=1;
    }

    &s[max_index]
}

fn main() {
    println!("Hello, world!");
    let p = Point { x: 1, y: 2 };
    let f = Point { x: 1.2, y: 1.3 };
    println!("p:{:?}", p);
    println!("f:{:?}", f);

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

    let a = Art::new("hello,rust".to_string(),"heige".to_string(),"wolrd".to_string());
    a.display(); // 调用trait特征上面的方法

    let w = Weibo::new("hi,rust".to_string(),"from weibo".to_string(),100);
    w.display();
    println!("summary: {}",w.summary());
    notify(w); // 调用notify方法

    let s = [1,2,3,4];
    println!("max2 {}",max2(&s[..]));
}
