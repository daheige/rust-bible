fn main() {
    println!("Hello, world!");
    let p = Post::new(
        "hello trait".to_string(),
        "heige".to_string(),
        "abc".to_string(),
    );
    println!("{}", p.get_title());
    println!("{}", p.summarize());

    println!("==========weibo post=========");
    let p2 = Weibo::new(
        "hello".to_string(),
        "daheige".to_string(),
        "rust".to_string(),
    );
    println!("p2 title: {}", p2.get_title());
    println!("{}", p2.summarize());
    println!("{}", p2.summarize_author());

    // 通过trait的方式调用方法output_content
    output_content(&p2);
}

// 定义trait特征
pub trait Summary {
    fn summarize(&self) -> String;
    fn get_title(&self) -> String {
        format!("current title unknown")
    }
    fn summarize_author(&self) -> String;
}

pub struct Post {
    title: String,
    author: String,
    content: String,
}

impl Post {
    fn new(title: String, author: String, content: String) -> Self {
        Self {
            title,
            author,
            content,
        }
    }

    // 覆盖默认实现
    fn get_title(&self) -> String {
        format!("post title:{}\n", self.title)
    }
}

// 为post实现Summary trait
// 为类型实现特征
impl Summary for Post {
    fn summarize(&self) -> String {
        format!(
            "title:{},author:{},content:{}",
            self.title, self.author, self.content
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Weibo {
    username: String,
    title: String,
    content: String,
}

impl Weibo {
    fn new(title: String, username: String, content: String) -> Self {
        Self {
            title,
            username,
            content,
        }
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!(
            "title:{},user:{},content:{}",
            self.title, self.username, self.content
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特征作为函数参数
// 实现了Summary特征 的 item 参数,是具体某个类型的借用
// 当某个类型实现了Summary trait，参数就可以传递&xxx模式作为参数传人到函数中
fn output_content(item: &impl Summary) {
    println!("breaking new! {}", item.summarize());
}

// impl Trait 的返回值类型并不支持多种不同的类型返回
// 下面的fn是行不通的
// fn summarizable_by_type(t: bool, title: String, username: String, content: String) -> Summary {
//     if t {
//         Post::new(title, username, content)
//     } else {
//         Weibo::new(title, username, content)
//         // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Post`, found struct `Weibo`
//     }
// }
