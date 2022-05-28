mod back_of_house {
    // 如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，
    // 但是这个结构体的字段仍然是私有的。我们可以根据情况决定每个字段是否公有。
    pub struct BreakFast {
        pub toast: String, // 表示这个字段是公开的
        fruit: String,     // 私有字段
    }

    impl BreakFast {
        pub fn new(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                fruit: String::from("heige"),
            }
        }

        pub fn get_fruit(&self) -> String {
            // 通过借用的方式访问fruit字段，然后转换为String
            // 否则就会抛出错误 // ^ value used here after move
            self.fruit.to_string()
        }
    }

    // 枚举enum定义了pub话，整个enum都是公开可访问的
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 模块嵌套
mod front_of_house {
    pub mod hosting {
        pub fn say(name: &str) {
            println!("{}", name);
        }
    }
}

// 通过use方式引入模块
// 通过在 crate 根的方式引入是当前模块根路径
// use crate::front_of_house::hosting;
// 也可直接通过use相对路径的方式引入模块mod
use front_of_house::hosting;

// 使用 pub use 重导出名称
// 当你的代码的内部结构与调用你的代码的开发者的思考领域不同时，
// 重导出会很有用。例如，在这个餐馆的比喻中，经营餐馆的人会想到“前台”和“后台”。
// 但顾客在光顾一家餐馆时，可能不会以这些术语来考虑餐馆的各个部分。使用 pub use，
// 我们可以使用一种结构编写代码，却将不同的结构形式暴露出来。这样做使我们的库井井有条，
// 方便开发这个库的开发者和调用这个库的开发者之间组织起来。
pub use front_of_house::hosting::say;

// 引入标准库
use std::collections::HashMap;

// 使用外部crate
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let b = back_of_house::BreakFast::new("hello"); // 调用模块上的结构体方法
    println!("{}", b.toast);
    // println!("{}", b.fruit); // ^^^^^ private field
    println!("fruit: {}", b.get_fruit());
    println!("fruit: {}", b.get_fruit());
    println!("toast: {}", b.toast);
    println!("enum:{:?}", back_of_house::Appetizer::Salad);

    // 调用通过use 方式引入的crate mod
    println!("========call hosting mod func======");
    hosting::say("daheige");
    hosting::say("rust");
    say("rust123");

    // 调用标准库
    // 注意标准库（std）对于你的包来说也是外部 crate。
    // 因为标准库随 Rust 语言一同分发，无需修改 Cargo.toml 来引入 std，
    // 不过需要通过 use 将标准库中定义的项引入项目包的作用域中来引用它们
    let mut m = HashMap::new();
    m.insert(1, 2);
    println!("{}", m.get(&1).unwrap());

    // 调用外部crate mod
    // crates.io 上有很多 Rust 社区成员发布的包，将其引入你自己的项目都需要一道相同的步骤：
    // 在 Cargo.toml 列出它们并通过 use 将其中定义的项引入项目包的作用域中。
    let rnd: i64 = rand::thread_rng().gen_range(1..101);
    println!("rnd:{}", rnd);
}
