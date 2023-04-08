# struct
- 一个 struct 就是一些字段的集合
- 结构体的字段是一个与数据结构相关联的数据值。它的值可以是基本类型或结构体类型。
- 它的定义就像给编译器的蓝图，告诉编译器如何在内存中布局彼此相邻的字段
```rust
// 定义结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 通过impl给结构体增加方法
// impl 是 implementation 的缩写
impl Rectangle {
    // 静态方法
    // Self表示当前类型Rectangle
    fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    // 方法的第一个参数必须有一个名为 self 的Self 类型的参数
    // 所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写
    // 我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例
    // 借用实例，不拥有所有权，只读访问Self
    //
    // 我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
    // 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；
    // 这种技术通常用在当方法将 self 转换成别的实例的时候，
    // 这时我们想要防止调用者在转换之后使用原始的实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 通过可变引用改变实例对象的值
    fn change_height(&mut self, h: u32) {
        self.height = h;
    }

    fn show(&self) {
        println!(
            "current width:{},height:{},area:{}",
            self.width,
            self.height,
            self.area()
        );
    }
}

fn main() {
    println!("Hello, world!");
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 100,
    };
    dbg!(&rect1);

    rect1.show();

    // rect是一个可变的变量
    let mut rect = Rectangle::new(12, 13); // ::调用静态方法
    rect.show();
    rect.change_height(11); // 可变引用的方式调用方法
    rect.show();
}

/* dbg!是一个debug 宏，可以把调试信息打印到终端输出
Hello, world!
[src/main.rs:11] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 100,
}
 */

```

# 方法调用
- 与函数（function）不同，方法（method）是与特定数据类型关联的函数。
- 静态方法 — 属于某个类型，调用时使用 :: 运算符。
- 实例方法 — 属于某个类型的实例，调用时使用 . 运算符。

```rust
fn main() {
    // 使用静态方法来创建一个String实例
    let s = String::from("Hello world!");
    // 使用实例来调用方法
    println!("{} is {} characters long.", s, s.len());
}
```

# 元组结构体
```rust
// 元组结构体定义
struct Location(i32, i32);

fn main() {
    // 这仍然是一个在栈上的结构体
    let loc = Location(42, 32);
    // 通过下标进行访问
    println!("{}, {}", loc.0, loc.1);
}

```
