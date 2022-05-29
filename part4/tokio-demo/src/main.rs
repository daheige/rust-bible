use mini_redis::{client, Result};

// 只用mini_redis 这里是练习，对于线上不要用这个crate
#[tokio::main]
async fn main() -> Result<()> {
    // 使用异步编程，无法立即完成的操作会被切到后台去等待
    // 因此当前线程不会被阻塞，它会接着执行其它的操作。
    // 一旦之前的操作准备好可以继续执行后，它会通知执行器，
    // 然后执行器会调度它并从上次离开的点继续执行
    //
    // 如果没有使用 await，而是按照这个异步的流程
    // 使用通知 -> 回调的方式实现，代码该多么的难写和难读
    // 好在 Rust 为我们提供了 async/await 的异步编程特性，
    // 让我们可以像写同步代码那样去写异步的代码
    // 下面的client::connect
    // async fn 异步函数并不会直接返回值，而是返回一个 Future
    // 顾名思义，该 Future 会在未来某个时间点被执行，然后最终获取到真实的返回值
    // 这里不会真正建立连接，只有在未来某个时候使用的时候才会建立连接
    let mut client = client::connect("127.0.0.1:6379").await?;
    // 设置key
    let key = "hello";
    client.set(key, "rust".into()).await?; // 异步执行set
    let result = client.get(key).await?; // 获取结果
    println!("get name:{} = {:?}", key, result);
    Ok(())
}
