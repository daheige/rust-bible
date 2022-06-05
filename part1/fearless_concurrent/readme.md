# rust并发模型
    起初，Rust 团队认为确保内存安全和防止并发问题是两个分别需要不同方法应对的挑战。
    随着时间的推移，团队发现所有权和类型系统是一系列解决内存安全和并发问题的强有力的工具！
    通过利用所有权和类型检查，在 Rust 中很多并发错误都是 编译时 错误，而非运行时错误。
    因此，相比花费大量时间尝试重现运行时并发 bug 出现的特定情况，
    Rust 会拒绝编译不正确的代码并提供解释问题的错误信息。
    因此，你可以在开发时修复代码，而不是在部署到生产环境后修复代码。
    我们给 Rust 的这一部分起了一个绰号 无畏并发（fearless concurrency）。
    无畏并发令你的代码免于出现诡异的 bug 并可以轻松重构且无需担心会引入新的 bug。

    很多语言所提供的处理并发问题的解决方法都非常有特色。例如，Erlang 有着优雅的消息传递并发功能，
    但只有模糊不清的在线程间共享状态的方法。对于高级语言来说，只实现可能解决方案的子集是一个合理的策略，
    因为高级语言所许诺的价值来源于牺牲一些控制来换取抽象。然而对于底层语言则期望提供在任何给定的情况下
    有着最高的性能且对硬件有更少的抽象。
    因此，Rust 提供了多种工具，以符合实际情况和需求的方式来为问题建模。

    Rust 标准库只提供了 1:1 线程模型实现。由于 Rust 是较为底层的语言，
    如果你愿意牺牲性能来换取的抽象，以获得对线程运行更精细的控制及更低的
    上下文切换成本，你可以使用实现了 M:N 线程模型的 crate。

    在大部分现代操作系统中，执行中程序的代码运行于一个 进程（process）中，
    操作系统则负责管理多个进程。在程序内部，也可以拥有多个同时运行的独立部分。
    这个运行这些独立部分的功能被称为 线程（threads）。
    将程序中的计算拆分进多个线程可以改善性能，因为程序可以同时进行多个任务，
    不过这也会增加复杂性。因为线程是同时运行的，所以无法预先保证不同线程中的
    代码的执行顺序。这会导致诸如此类的问题：
    1.竞争状态（Race conditions），多个线程以不一致的顺序访问数据或资源
    2.死锁（Deadlocks），两个线程相互等待对方停止使用其所拥有的资源，
    这会阻止它们继续运行
    3.只会发生在特定情况且难以稳定重现和修复的 bug

    Rust 尝试缓和使用线程的负面影响。不过在多线程上下文中编程仍需格外小心，
    同时其所要求的代码结构也不同于运行于单线程的程序。

    编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。
    这种由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程。
    很多编程语言提供了自己特殊的线程实现。
    编程语言提供的线程被称为 绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 
    线程的上下文中执行它们。
    为此，绿色线程模式被称为 M:N 模型：M 个绿色线程对应 N 个 OS 线程，这里 M 和 N 不必相同。
    每一个模型都有其优势和取舍。
    
    对于 Rust 来说最重要的取舍是运行时支持。运行时（Runtime）是一个令人迷惑的概念，其在不同上下文中可能有不同的含义。
    在当前上下文中，运行时 代表二进制文件中包含的由语言自身提供的代码。
    这些代码根据语言的不同可大可小，不过任何非汇编语言都会有一定数量的运行时代码。
    为此，通常人们说一个语言 “没有运行时”，一般意味着 “小运行时”。
    更小的运行时拥有更少的功能不过其优势在于更小的二进制输出，这使其易于在更多上下文中与其他语言相结合。
    虽然很多语言觉得增加运行时来换取更多功能没有什么问题，但是 Rust 需要做到几乎没有运行时，
    同时为了保持高性能必需能够调用 C 语言，这点也是不能妥协的。

    绿色线程的 M:N 模型需要更大的语言运行时来管理这些线程。
    因此，Rust 标准库只提供了 1:1 线程模型实现。
    由于 Rust 是较为底层的语言，如果你愿意牺牲性能来换取的抽象，
    以获得对线程运行更精细的控制及更低的上下文切换成本，
    你可以使用实现了 M:N 线程模型的 crate。
# rust Send 和 Sync trait 深入剖析

    幸运的是，支持并发的 Rust 已经有了两个广为人知的有效的概念，
    它们对并发部分的共享程序进行了抽象：Send 和 Sync 。
    值得注意的是，Send 和 Sync 的特征抽象自并发处理策略，它们结构规整，且不指定实现。
    
    简短概述：
    Send 将计算中的传递数据抽象到另一个并发计算 (我们称其为接收方)，而对于发送方，将无法再次访问它。
    在许多程序设计语言中，通常都实施此策略，但是缺少语言层面的支持，
    而寄希望于你自己执行 “丢掉访问” 的操作。
    bugs 的常规来源：发送方保留发送内容的句柄，甚至在发送后也可以操作它们。
    Rust 通过使这种行为摆到明面上来减轻此问题。类型可以是 Send ，也可以不是 (通过适当的特征实现标记)，
    允许或不允许发送它们，并且借助所有权和借用规则阻止后续的访问。

    Sync 是指在程序的两个并发部分之间共享数据。这是另一种常见的模式：由于向内存位置写入或在另一方正在
    写入时进行读取本来就不安全，因此需要通过同步来协调。协调双方有许多常见的方式来达成共识，
    即不同时使用位于内存中的同一部分，例如互斥锁和自旋锁。同样，Rust 提供了 (安全！) 无需担心的选择项。
    Rust 可以让人自由地表达哪些需要同步，而无需对具体的实现连篇累牍。
    
    Rust 使我们能够安全地抽象出并发程序的重要属性，以及其数据的共享方式。
    它以一种非常轻量级的办法达成目标；语言本身标识的是 Send 和 Sync trait，并在可能的条件下，
    通过派生这两个标记帮助我们达成目的。剩下的就是库要处理的问题了。

# 通过spawn创建线程
```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    // 1.使用 spawn 创建新线程
    thread::spawn(||{
        for i in 1..10{
            println!("spawn thread: current i = {}",i);
            thread::sleep(Duration::from_millis(1)); // 在独立线程中停顿1ms
        }
    });

    for i in 1..5{
        println!("main thread: i = {}",i);
        thread::sleep(Duration::from_millis(1));
    }
}
/*当主线程退出了，子线程就会跟着退出
    Hello, world!
    main thread: i = 1
    spawn thread: current i = 1
    main thread: i = 2
    spawn thread: current i = 2
    spawn thread: current i = 3
    main thread: i = 3
    main thread: i = 4
    spawn thread: current i = 4
    spawn thread: current i = 5*/
```

# 通过join等待所有线程结束
    thread::spawn 的返回值类型是 JoinHandle。
    JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束
```rust
let handler = thread::spawn(||{
        for i in 1..10{
            println!("spawn thread: current i = {}",i);
            thread::sleep(Duration::from_millis(1)); // 在独立线程中停顿1ms
        }
    });

    for i in 1..5{
        println!("main thread: i = {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
    println!("main thread will exit...");
// 执行结果如下：
/*
Hello, world!
main thread: i = 1
spawn thread: current i = 1
spawn thread: current i = 2
main thread: i = 2
spawn thread: current i = 3
main thread: i = 3
spawn thread: current i = 4
main thread: i = 4
spawn thread: current i = 5
spawn thread: current i = 6
spawn thread: current i = 7
spawn thread: current i = 8
spawn thread: current i = 9
main thread will exit...
*/
```

# 线程与 move 闭包
    move其经常与 thread::spawn 一起使用，
    因为它允许我们在一个线程中使用另一个线程的数据
```rust
let v = vec![1,2,3,4,5];
let handler = thread::spawn(||{
        for i in &v {
            println!("i = {}",i);
        }
    });
handler.join().unwrap();
    println!("main thread will exit...");

//上面的代码就会报错：
|     thread::spawn(||{
   |                   ^^ may outlive borrowed value `v`
48 |         for i in &v {
   |                   - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:47:5
   |
47 | /     thread::spawn(||{
48 | |         for i in &v {
49 | |             println!("i = {}",i);
50 | |         }
51 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
47 |     thread::spawn(move ||{
   |                   ++++ 
```

```rust
// 通过下面的move来解决
   let handler = thread::spawn(move ||{
for i in &v {
println!("i = {}",i);
}

println!("v = {:?}",v);
});
```

# 使用消息传递在线程间传送数据
```rust
#[cfg(test)]
mod tests{
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn message_pass(){
        let (tx,rx) = mpsc::channel(); // 创建一个无限缓冲的通道channel
        // 发送消息
        thread::spawn(move ||{ // 这里需要move 将tx所有权移动到闭包中
            let v = String::from("abc");
            tx.send(v).unwrap();
        });

        // 接收消息
        let msg = rx.recv().unwrap();
        println!("msg:{}",msg);
    }
}
```

# 发送多个值并观察接收者的等待
```rust
// 发送多个值
    #[test]
    fn message_pass2(){
        let (tx,rx) = mpsc::channel();
        thread::spawn(move ||{
            let v = vec![1,2,3,4];
            for val in v {
                tx.send(val).unwrap();
            }
        });

        // 下面就会阻塞，等待发送者发送完毕后就会消费
        for msg in rx {
            println!("msg: {}",msg);
        }
    }
```

# 多个生产者模式
    通过克隆发送者来创建多个生产者
```rust
#[test]
    fn message_mp(){
        let (tx,rx) = mpsc::channel();

        // 通过tx来克隆一个生产者
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let s = String::from("abc");
            tx1.send(s).unwrap();
        });

        thread::spawn(move || {
            let s = vec![
                String::from("hello"),
                String::from("rust"),
            ];
            for v in s {
                tx.send(v).unwrap();
            }
        });

        for msg in rx{
            println!("recv msg: {}",msg);
        }
    }
```
# 共享状态
    通过mutex互斥锁来实现
```rust
#[test]
    fn mutex_test(){
        let mut m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num += 6;
        }
        println!("m = {:?}",m);
        println!("m = {:?}",m.get_mut().unwrap());
        // m = Mutex { data: 11, poisoned: false, .. }
        // m = 11
    }
```

# 在线程中共享Mutex<T>
```rust
// 下面的代码不能编译成功
#[test]
    fn mutex_share_data(){
        let counter = Mutex::new(0);
        // 上面一行不能编译，抛出下面的错误
        // ------- move occurs because `counter` has type `Mutex<i32>`,
        // which does not implement the `Copy` trait
        let mut handlers = vec![];
        for i in 0..10{
            // 创建多个线程
            let handler = thread::spawn(move ||{
                // 上面一行报错
                // ^^^^^^^ value moved into closure here, in previous iteration of loop
                let mut num = counter.lock().unwrap();
                *num +=i;
            });

            // 将handle join句柄加入到handlers
            handlers.push(handler);
        }

        for handler in handlers{
            handler.join().unwrap();
        }

        println!("result: {}",*counter.lock().unwrap());
    }
// 上面的代码需要通过多所有权来修复这个问题
// Arc 多个线程中安全操作的原子引用计数器Arc<T>
// 像 Mutex<T> 和 Arc<T> 这样可以安全的用于并发上下文的智能指针
#[test]
fn mutex_share_data(){
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for i in 0..10{
        // 创建多个线程
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num +=i;
        });

        // 将handle join句柄加入到handlers
        handlers.push(handler);
    }

    for handler in handlers{
        handler.join().unwrap();
    }

    // result: 45
    println!("result: {}",*counter.lock().unwrap());
}
```

# 无畏并发
    Rust 提供了用于消息传递的通道，和像 Mutex<T> 和 Arc<T> 
    这样可以安全的用于并发上下文的智能指针。类型系统和借 用检查器会确保这些场景中的代码，
    不会出现数据竞争和无效的引用。一旦代码可以编译了，我们就可以坚信这些代码
    可以正确的运行于多线程环境，而不会出现其他语言中经常出现的那些难以追踪的 bug。
    并发编程不再是什么可怕的概念：无所畏惧地并发吧！
