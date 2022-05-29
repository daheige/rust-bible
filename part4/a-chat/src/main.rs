mod client;
mod server;

// 自定义Result 使用Box机制，主要是存放实现了 error trait的对象
// 标准库中实现了 From<&'_ str> for Box<dyn Error> 的将允许你使用 ? 操作符来处理错误
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut args = std::env::args();
    match (args.nth(1).as_ref().map(String::as_str), args.next()) {
        (Some("client"), None) => client::main(),
        (Some("server"), None) => server::main(),
        _ => Err("Usage: a-chat [client|server]".into()),
    }
}
