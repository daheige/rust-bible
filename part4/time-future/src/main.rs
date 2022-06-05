use std::time::Duration;
use time_future::TimerFuture;

mod time_future;

// 由于 async 会返回一个 Future，因此我们还需要配合使用 .await 来让该 Future 运行起来
async fn say_hello() {
    println!("hello,rust");
}

// 通过 tokio 运行时执行time_future future
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let shared_state = TimerFuture::new(Duration::from_secs(2)); // 返回一个future trait
    println!("abc");

    shared_state.await; // .await不会阻塞当前线程，用.await 等待future执行完毕
    println!("exec say_hello func");
    let f = say_hello();
    f.await;
    println!("exec end");
}
