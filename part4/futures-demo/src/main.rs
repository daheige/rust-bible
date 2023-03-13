use futures::executor::block_on;

// async fn 返回一个future 需要在一个执行者上面才可以运行
async fn hello_world(name: String) {
    println!("hello:{}", name);
}

async fn hello_cat() {
    println!("hello,kitty");
}

// 在async函数中执行另一个异步调用
// 通过同步代码的顺序实现了异步的执行效果
async fn say() {
    // 这里必须使用.await等待另一个异步调用future执行完毕
    hello_cat().await;
    println!("exec success");
}

fn main() {
    println!("Hello, world!");
    let f = hello_world("daheige".to_string());

    // 1.block_on阻塞当前线程，直到提供的Future运行完成
    // 2. 其他执行者提供更加复杂的行为，例如将多个future安排到同一个线程上执行
    block_on(f);

    // 在async fn函数中使用.await可以等待另一个异步调用的完成。
    // 但是与block_on不同，.await并不会阻塞当前的线程，而是异步的等待Future A的完成，在等待的过程中，
    // 该线程还可以继续执行其它的Future B，最终实现了并发处理的效果
    let f = say();
    block_on(f);
}
