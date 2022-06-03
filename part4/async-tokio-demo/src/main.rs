use std::rc::Rc;
use std::sync::Mutex;
use std::thread::yield_now;
use tokio::{spawn, sync::mpsc, sync::oneshot, task};
// 实现自定义的Future需要的mod
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

async fn say_to_world() -> String {
    println!("exec say_to_world");
    String::from("hello,rust")
}

struct Increment {
    mutex: Mutex<i64>,
}

impl Increment {
    fn incr(&self) {
        let mut value = self.mutex.lock().unwrap();
        *value += 1;
    }
}

async fn incr_task(i: &Increment) {
    i.incr();
}

// #[tokio::main] 将最外层 Future 提交给 Tokio 的执行器。该执行器负责调用 poll 函数，
// 然后推动 Future 的执行，最终直至完成

#[tokio::main]
async fn main() {
    let op = say_to_world();
    println!("hello");
    // 由于 async 会返回一个 Future，因此我们还需要配合使用 .await 来让该 Future 运行起来，
    // 最终获得返回值:
    println!("op :{}", op.await);

    // spawn 返回JoinHandle
    // 任务是调度器管理的执行单元。spawn生成的任务会首先提交给调度器，
    // 然后由它负责调度执行。需要注意的是，执行任务的线程未必是创建任务的线程，
    // 任务完全有可能运行在另一个不同的线程上，而且任务在生成后，
    // 它还可能会在线程间被移动。
    //
    // 任务在 Tokio 中远比看上去要更轻量，例如创建一个任务仅仅需要一次64字节大小的内存分配。
    // 因此应用程序在生成任务上，完全不应该有任何心理负担，
    // 除非你在一台没那么好的机器上疯狂生成了几百万个任务
    // let handle = tokio::spawn(async { 10086 });
    let handle = spawn(async { 10086 });

    // .await 会返回一个Result
    // 如果上面的spawn 创建的任务正常结束，则返回一个 Ok(T)，否则返回错误error
    // match handle.await {
    //     Ok(value) => println!("value:{}", value),
    //     Err(err) => println!("get value err:{}", err),
    // };

    // 下面的写法比较简洁
    let out = handle.await.unwrap();
    println!("get value:{}", out);

    // 当使用 Tokio 创建一个任务时，该任务类型的生命周期必须时 'static。
    // 意味着，在任务中不能使用外部数据的引用
    // 下面的v就是例子
    let v = vec![1, 2, 3];
    // 创建异步任务
    // 这里需要使用move关键字将v 所有权移动到闭包函数中
    let handler = task::spawn(async move {
        println!("hello");
        println!("vec:{:?}", v);
    });

    let _ = handler.await;

    let handler = spawn(async {
        // 语句块的使用强制来 rc 会在.await被调用前就被释放
        // 因此rc并不会影响到.await的安全性
        // rc 是否会保存到任务状态中，取决于.await 的调用是否处于它的作用域中
        {
            let rc = Rc::new("hello");
            println!("rc: {}", rc);
        }

        // rc的作用范围已经失效来，当上面的任务让出所有权给当前线程时，它就不需要作为状态被保存起来
        yield_now();
    });
    let _ = handler.await;

    let m = Mutex::new(1);
    let i = Increment { mutex: m };

    let i2 = incr_task(&i).await;
    println!("{:?}", i2);
    println!("mutex: {:?}", i.mutex);
    println!("mutex value: {}", i.mutex.lock().unwrap());

    println!("create msg channel");
    // 创建消息通道
    // 通过tokio::sync::mpsc，多个生产者-单个消费者模式创建tx,rx
    // 并且指定缓冲队列长度为32
    // 一旦存满了 32 条消息，使用send(...).await的发送者会进入睡眠，
    // 直到缓冲队列可以放入信息的消息
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone(); // 这里tx是允许多次clone，并发送消息到通道中
    let tx3 = tx.clone();
    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });
    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });
    tokio::spawn(async move {
        tx3.send("sending from three handle").await;
    });

    // 接收消息
    // 当所有的发送者都被 Drop 掉后(超出作用域或被 drop(...) 函数主动释放)，
    // 就不再会有任何消息发送给该通道，此时 recv 方法将返回 None，
    // 也意味着该通道已经被关闭。
    while let Some(msg) = rx.recv().await {
        println!("rx msg:{}", msg);
    }

    // 使用 oneshot 消息通道
    let t1 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel(); // 无缓冲通道

        // 往 oneshot 中发送消息时，并没有使用 .await，原因是该发送操作要么直接成功、要么失败，并不需要等待
        // tx.send("abc").unwrap();
        let _ = tx.send("abc");

        let msg = rx.await;
        println!("get msg:{}", msg.unwrap());
    });

    // 调用自定义的Delay
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };
    let out = future.await; // 执行

    // .await 只能用于 async fn 函数中，因此我们将 main 函数声明成 async fn main
    // 同时使用 #[tokio::main] 进行了标注
    println!("out:{}", out);
    assert_eq!(out, "done");
}

// 自定义一个Future实现
// 1. 等待某个特定时间点的到来
// 2. 在标准输出打印文本
// 3. 生成一个字符串
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            // 时间到了
            println!("hello,world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
