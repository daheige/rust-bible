// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheudling
// multiple futures onto the same thread.
//
// block_on 会阻塞当前线程执行，直到提供的future执行完毕为止
// 其他的执行器 executors 提供更加复杂的行为，比如说多个futures调度在同一个线程上
use futures;
use futures::executor::block_on;

// async fn 返回一个Future ，需要在执行的时候才能起到作用
async fn do_task() {
    println!("do async task");
}

async fn learn_song() {
    println!("learn song");
}

async fn sing_song(name: String) {
    println!("sing song:{}", name);
}

async fn dance() {
    println!("dance...")
}

async fn learn_and_sing() {
    // 这里用.await 不是用block_on ,这样可以让其他future执行
    let _ = learn_song().await;
    sing_song("hello".to_string()).await;
}

// 返回返回一个执行器
async fn do_more_tasks() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // 下面的join! 类似于.await ,但他可以等待多个 future 并发完成
    // 如果说f1有了短暂的阻塞，这个时候f2 就会接管当前的线程
    // 如果f2执行时候发生了阻塞，f1就会接管执行
    futures::join!(f1, f2);
}

fn main() {
    println!("start exec async future");
    let future = do_task();

    println!("task start");
    block_on(future);
    println!("task has done");

    // async/.await 执行操作
    block_on(do_more_tasks());
}
