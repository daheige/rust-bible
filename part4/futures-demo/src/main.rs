use futures::executor::block_on;

// async fn 返回一个future
// 需要在一个执行者上面才可以运行
async fn hello_world(name: String) {
    println!("exec async task:{}", name);
}

fn main() {
    println!("Hello, world!");
    let f = hello_world("daheige".to_string());

    // 1.block_on阻塞当前线程，直到提供的Future运行完成
    // 2. 其他执行者提供更加复杂的行为，例如将多个future安排到同一个线程上执行
    block_on(f);
}
