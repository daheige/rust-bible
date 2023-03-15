# tokio task
    一个 Tokio 任务是一个异步的绿色线程，它们通过 tokio::spawn 进行创建，
    该函数会返回一个 JoinHandle 类型的句柄，调用者可以使用该句柄跟创建的任务进行交互。
    spawn 函数的参数是一个 async 语句块，该语句块甚至可以返回一个值，
    然后调用者可以通过 JoinHandle 句柄获取该值

# 'static 约束
当使用 Tokio 创建一个任务时，该任务类型的生命周期必须时 'static
意味着，在任务中不能使用外部数据的引用
```rust
// 下面的v就是例子
    let v = vec![1, 2, 3];
    // 这里需要使用move关键字将v 所有权移动到闭包函数中
    let handler = task::spawn(async move {
        println!("hello");
        println!("vec:{:?}", v);
    });

    let _ = handler.await;
```

# Send 约束
    tokio::spawn 生成的任务必须实现 Send 特征，因为 Tokio 调度器会将任务在线程间进行移动，
    当这些任务在 .await 执行过程中发生阻塞时。

    一个任务要实现 Send 特征，那它在 .await 调用的过程中所持有的全部数据都必须实现 Send 特征。
    当 .await 调用发生阻塞时，任务会让出当前线程所有权给调度器，然后当任务准备好后，调度器会从
    上一次暂停的位置继续执行该任务。该流程能正确的工作，任务必须将.await之后使用的所有状态保存起来，
    这样才能在中断后恢复现场并继续执行。若这些状态实现了 Send 特征(可以在线程间安全地移动)，
    那任务自然也就可以在线程间安全地移动。

# 任务、线程和锁竞争

    当竞争不多的时候，使用阻塞性的锁去保护共享数据是一个正确的选择。
    当一个锁竞争触发后，当前正在执行任务(请求锁)的线程会被阻塞，并等待锁被前一个使用者释放。
    这里的关键就是：
        锁竞争不仅仅会导致当前的任务被阻塞，还会导致执行任务的线程被阻塞，
        因此该线程准备执行的其它任务也会因此被阻塞！
    
    默认情况下，Tokio 调度器使用了多线程模式，此时如果有大量的任务都需要访问同一个锁，
    那么锁竞争将变得激烈起来。当然，你也可以使用 current_thread 运行时设置，在该设置
    下会使用一个单线程的调度器(执行器)，所有的任务都会创建并执行在当前线程上，因此不再会有锁竞争。
    
        current_thread 是一个轻量级、单线程的运行时，当任务数不多或连接数不多时是一个很好的选择。
        例如你想在一个异步客户端库的基础上提供给用户同步的API访问时，该模式就很适用
    
    当同步锁的竞争变成一个问题时，使用 Tokio 提供的异步锁几乎并不能帮你解决问题，
    此时可以考虑如下选项：
    
        - 创建专门的任务并使用消息传递的方式来管理状态
        - 将锁进行分片
        - 重构代码以避免锁

# 在 .await 期间持有锁
在某些时候，你可能会不经意写下这种代码:
```rust
use std::sync::{Mutex, MutexGuard};

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
*lock += 1;

    do_something_async().await;
} // 锁在这里超出作用域
```

如果你要 spawn 一个任务来执行上面的函数话，会报错:
错误的原因在于 std::sync::MutexGuard 类型并没有实现 Send 特征，
这意味着你不能将一个 Mutex 锁发送到另一个线程，
因为 .await 可能会让任务转移到另一个线程上执行，这个之前也介绍过。

# 提前释放锁
要解决这个问题，就必须重构代码，让 Mutex 锁在 .await 被调用前就被释放掉。
```rust
// 下面的代码可以工作！
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock在这里超出作用域 (被释放)

    do_something_async().await;
}

// 我们可能已经发现，很多错误都是因为 .await 引起的，其实你只要记住，
// 在 .await 执行期间，任务可能会在线程间转移，那么这些错误将变得很好理解
```

# 关于tokio 引入并发和队列选择

    在 Tokio 中我们必须要显式地引入并发和队列:
    
        tokio::spawn
        select!
        join!
        mpsc::channel
    
    当这么做时，我们需要小心的控制并发度来确保系统的安全。
    例如，当使用一个循环去接收 TCP 连接时，你要确保当前打开的 socket 数量在可控范围内，
    而不是毫无原则的接收连接。 再比如，当使用 mpsc::channel 时，要设置一个缓冲值。

# async小结
- 在 Rust 中，async 是惰性的，直到执行器 poll 它们时，才会开始执行 
- Waker 是 Future 被执行的关键，它可以链接起 Future 任务和执行器
- 当资源没有准备时，会返回一个 Poll::Pending
- 当资源准备好时，会通过 waker.wake 发出通知
- 执行器会收到通知，然后调度该任务继续执行，此时由于资源已经准备好，因此任务可以顺利往前推进了

# tokio 神秘面纱参考
    https://zhuanlan.zhihu.com/p/460984955
    https://zhuanlan.zhihu.com/p/461044853
    https://zhuanlan.zhihu.com/p/461384827
    https://zhuanlan.zhihu.com/p/461874095
    https://zhuanlan.zhihu.com/p/462116823
    https://zhuanlan.zhihu.com/p/462387088
