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

fn main() {
    println!("Hello, world!");
    let b = back_of_house::BreakFast::new("hello"); // 调用模块上的结构体方法
    println!("{}", b.toast);
    // println!("{}", b.fruit); // ^^^^^ private field
    println!("{}", b.get_fruit());
    println!("{}", b.get_fruit());
    println!("{}", b.toast);
    println!("enum:{:?}", back_of_house::Appetizer::Salad);
}
