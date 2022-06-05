# rust async 异步编程基础
    
    1.futures crate 和 async-std crate 运行机制研究
    2.tokio crate 运行时研究
    
    rust 目前只提供了编写async 代码的基本要素，标准库中尚未提供执行器 executor,
    任务task,wake反应器、组合器以及低级 I/O future 和 trait
    社区提供的async 运行时生态主要是async-std,tokio

    tokio: 一个流行的async 生态系统，包含HTTP,gRPC和trace跟踪框架等等
    async-std: 提供标准库的async 运行时副本

# async runtime 运行时剖析
+ async 运行时是用于执行async 应用程序的库
+ 运行时通常将一个反应器与一个或多个执行器bind在一起
+ 反应器为异步i/o、进程间通信和计时器等外部事件提供订阅机制
+ 在 async 运行时中，订阅者通常是代表低级别的i/o操作的future
+ 执行者负责安排和执行任务
    - 它们跟踪正在运行和暂停的任务，对future进行poll直到完成，并在任务能够取得进展时候唤醒
    - executor "执行者" 一词经常与"运行时" runtime 相互使用
+ 我们使用"生态系统"来描述一个与兼容trait 和特性 bind 捆绑在一起的运行时

# 什么是async/await
+ async/.await是rust语言用于编写同步代码像异步代码一样的内置工具 
+ async将一个代码块转换为一个实现了 Future trait 的状态机
+ 虽然在同步的方法中调用阻塞函数会阻塞整个线程，但阻塞的 Futures 将让出线程所有权, 允许其他的 Futures 运行。

深入理解：
+ async/.await 是rust特殊语法，在发生阻塞时候，它让放弃当前线程的控制权成为可能，
这就允许在等待操作完成的时候，允许线程运行其他代码。
+ .await 会驱动future执行，如果执行时候发生了阻塞，就会让出控制权，让线程执行其他代码
也就是说.await 会等待，直到future 变成 Poll::Ready(T) 同时await最终会解析出future值，放入T
+ 获得Future 的所有权，并对其进行poll 
+ 如果Future Ready ，其最终值就是 await 表达式的值，这时执行就可以继续了
    否则就返回 Pending 给调用者

# async的生命周期
+ 与传统的函数不同：async fn，如果它的参数是引用或是其它的非'static的，那么它返回的Future
  就会绑定到参数的生命周期上
+ 这就意味着 async fn 返回的future,在.await的同时，fn 的非'static 的参数必须保持有效

# async move
  - async 块和闭包都支持move
  - async move 块会获得其引用变量的所有权
    + 允许其比当前所在的作用域活得长
    + 但同时也放弃了与其它代码共享这些变量的能力

# 在多线程执行者上运行.await
+ 当使用多线程 future 执行者时候，future 就可以线程间移动
    - 所以async 体里面用的变量必须能够在线程间移动
    - 因为任何的.await 都可能导致切换到一个新的线程

+ 这意味着使用以下类型是不安全的：
    - Rc，&RefCell 和任何其它没有实现 Send trait 的类型，包含没实现 Sync trait 的引用
    - 注意： 调用 .await 时候，只要这些类型不在作用域内，就可以使用他们，不安全

+ 在跨越一个 .await 期间，持有传统的、对future无感知的锁，也不是好主意
    - 可导致线程池锁定
    - 为此，可使用futures::lock 里面的Mutex 而不是 std::sync里面的Mutex

# rust Future原理--简单抽象理解
```rust
trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// Poll 是一个状态机，要么完成，要么尚未完成
// 通过调用该 poll 函数可以推进 Futures，这将推动 future 尽可能地完成。
// 如果 Future 完成，它将返回 Poll::Ready(result)。
// 如果 Future 尚未完成，它将返回 Poll::Pending 并安排在 Future 
// 准备好进行更多进展时调用 wake() 函数。当 wake() 被调用时，
// executor(执行者) 驱动 Future 将再次调用 poll，以便 Future 能够取得更多进展。
enum Poll<T> {
    Ready(T),
    Pending,
}
```

# rust 底层真正的Futures trait 抽象初步认识
```rust
use std::{pin::Pin, task::{Context, Poll}};
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```
仔细阅读，观察到的有以下内容：

  + 通用的 Output，就是执行完毕后返回值
  + 提供 poll 函数，函数功能是允许我们查看当前计算的状态
  （暂时忽略 Pin 和 Context，先不对它们作深入理解）

  + 关联类型 Output 是 Future 执行完成后返回的值的类型
  + Pin 类型是在异步函数中进行借用的关键，它允许我们创建不可移动的future
  + 不可移动的对象可以在他们的字段间存储指针
  + 需要启动async/await,pin 就是必须的
  + 跟上面的 simpleFuture比较，wake: fn() 变成了 &mut Context
  + Context 类型提供了访问Waker类型的值的方式，这些值可以被用来wake up特定的任务（比如说web 连接处理）
  + 和其它语言不同，Rust中的 Future 不代表一个发生在后台的计算，
    而是 Future 就代表了计算本身，因此 Future 的所有者有责任去推进该计算过程的执行，
例如通过 Future::poll 函数：
      - 在 Rust 中，async 是惰性的，直到执行器 poll 它们时，才会开始执行
      - Waker 是 Future 被执行的关键，它可以链接起 Future 任务和执行器
      - 当资源没有准备时，会返回一个 Poll::Pending
      - 当资源准备好时，会通过 waker.wake 发出通知
      - 执行器会收到通知，然后调度该任务继续执行，此时由于资源已经准备好，
        因此任务可以顺利往前推进了
      
   每次调用 poll() 都会导致以下两种情况的其中一种：
   + 计算已经完成， poll 会返回 Poll::Ready
   + 计算尚未完成执行，poll 会返回 Poll::Pending

    上述机制使我们可以从外部检查 Future 是否仍有未完成的工作，或最终是否完成并给出返回值。
    最简单 (但不是最有效) 的方法是不断循环执行 poll 函数。
    这种办法当然是有优化的可能，而这正是一个好的运行时要做的。
    请注意，在第 1 种情况发生后再次调用 poll 可能会导致混乱。
    
    自定义future trait实现，参考 time-future

# async-std
    async-std 及其支持库，可以使您的异步编程更为轻松。它主要用于编写上层库和相关的应用程序。
    正如该库的名字所述，async-std 通过将替换原部件为异步部件，尽可能地构建了一个支持异步的 Rust 标准库。 
    async-std 的界面提供了所有的重要原语：文件系统操作，网络操作和实现并发性的基础组件（如计时器）。
    它同样公开了一个类似于 Rust 标准库中 thread 模块的 task 模块。但 task 模块不止包括了 I/O 原语，
    还容纳了像是 Mutex 这种可兼容 async/await 的原语
    book.async.rs/

# std::future 与 futures-rs
    Rust 中 Future 可能指两种不同的类型：
        第一种是指 Rust 标准库中的 std::future::Future。
        第二种是指 futures-rs crate 中的 futures::future::Future。

    futures-rs 中定义的 future 类型是标准库中 future 的原始实现。
    Rust 将核心的 Future trait 移入标准库中并改为 std::future::Future 以实现 async/await 语法。
    在某种意义上讲，std::future::Future 可以看作是 futures::future::Future 的最小子集。
    与 futures 相同，async-std 也重新导出了核心 std::future::Future 类型。
    您可以通过将 futures 加入您的 Cargo.toml 中并导入 FuturesExt 来使用它的扩展。

# async-std 运行时--异步概念
    Futures 是对代码运行方式的抽象。Futures 本身并没有什么作用，
    虽然这对于按步执行的命令式语言来说这是一个奇怪的概念。
    Futures 的运行由你决定。Futures 仅在对它们调用 executing 时才会执行。
    这一部分被称为执行者 (excutor)。执行者 决定你执行 futures 的时间 和方式。
    async-std::task 提供了这些执行者的接口。

# async-std Futures

    fearless concurrency 是 Rust 值得大书特书的一点。它提出了一个想法，
    授予你并行处理的权限而不舍弃安全性。
    而且，Rust 作为底层程序设计语言，无需担忧它对并发的处理，不用自己去选择特定的并发实现策略。
    这也意味着，如果我们想不同策略的用户可以共享代码，我们必须对策略抽象，以便将来可以做选择。
    Futures 对计算抽象。Futures 描述 “what”，并使其独立于 “where” 和 “when” 。
    因此，Futures 旨在将代码拆解为小的可组合的操作，然后由自己的系统的一部分执行这些操作。
    让人们了解计算事务的本质，找到可以抽象之处。

# async-std接口与稳定性

    async-std 旨在成为像 Rust 标准库一样稳定可靠的库。这也意味着我们的接口并不依赖于 futures 库。
    同时，async-std 的类型也实现了所有 futures 库中的 traits，以提供许多用户喜爱的 futures-rs 库
    带来的便利。

    幸运的是，上述的方法为您提供了充分的灵活性。如果您注重稳定性，您可以只使用 async-std 库。
    如果您更喜欢 futures 库的接口，您可以引入它。这两种用法都没有问题。
    async_std::future
    您可以在 async_std::future 模块中找到与处理各种 futures 
    相关的重要支持函数，我们同样保证它们的稳定性。
    
    Streams 和 Read/Write/Seek/BufRead traits
    由于 Rust 编译器的限制，它们只能在 async_std 中实现，而不可以由用户自己实现。

# async-std 未来展望
    async-std 实现了以下 traits：
        Read
        Write
        Seek
        BufRead
        Stream
    为了与生态交互，所有实现了这些 traits 的类型都在 futures-rs 库中实现了相应的接口。
    我们的 SemVer 保证并不包含这些接口，这些接口将会保守地同步更新。

    async-std1.0.z 所需的最低 Rust 版本都为 1.37.0，而使用 async-std 1.y (y>0) 
    所需的最低 Rust 版本可能会大于 1.37.0。
    一般来说，本 crate 对于最低支持 Rust 版本的改动比较保守。
    但 async/await 本身就是一个新功能，我们将在开发初期逐步地改动最低支持 Rust 版本。

# async-std Async

尽管 Future trait 在 Rust 中已经存在不短的时间，但构建和描述它们并不方便。为此，Rust 有了特殊的语法：async。请看一个使用 async-std 实现 Future 的例子：
```rust
use async_std;
use async_std::{fs::File, io, io::prelude::*};
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
```

当同时执行两个或多个函数时，我们的运行时系统会处理当前正在进行的所有其他事件，以此来填充等待时间
Future 是一种不代表任何值的数据类型，但在将来的某个时间点它有能力产生一个值。
针对不同的用例，实现的方式千差万别，但对外暴露的接口会非常简洁。

# async_std 中的 Task

	Task 是 async_std 的核心抽象之一。 和 Rust 的 thread 一样，Task 提供了一些原始概念上的实用功能。 
	Task 与运行时有关， 同时它们本身又是独立的。 async_std 的 task 有许多理想的属性：
	    1.所有分配一次完成
	    2.所有任务都有 backchannel（回发通道），通过 JoinHandle 将结果和错误回传到生成任务
	    3.带有用于调试的元数据
	    4.支持本地存储任务
	async_std 的 task API 可以处理后台运行时的设置和拆除，不用依赖于显式启动的运行时。

    async_std 附带了一个有用的 Task 类型，它与类似 std::thread 的 API 配合使用。
    它以结构化和定义的方式涵盖了错误行为和紧急情况。

# 关于async-std task 阻塞

假定 Task 是并发运行，那么可能是通过共享执行线程来处理并发的。这意味着阻塞进行中的系统线程的操作，例如
```rust
std::thread::sleep
```

或调用 Rust 的 std 类库的 io 函数，都将停止执行共享此线程的所有任务。
其他的库（例如数据库驱动程序）就有类似的行为。
需注意，阻塞当前线程本身并不是不好的行为，只是其不能与 async-std 的并发执行模型不能很好地混合使用。
本质上，永远不要有以下操作：

```rust
use async_std::task;
fn main() {
    task::block_on(async {
    // this is std::fs, which blocks
    std::fs::read_to_string("test_file");
})
}
```
如果要多种混合操作，请考虑将此类阻塞操作放在单独的 thread 上。

# async-std errors and panics
Tasks report errors through normal patterns: If they are fallible, 
their Output should be of kind Result<T,E>.
In case of panic, behaviour differs depending on whether 
there's a reasonable part that addresses the panic. 
If not, the program aborts.
In practice, that means that block_on propagates panics to the blocking component

task任务在正常情况下，上报错误；如果不能正常上报，应该是Result<T,E> 模式。 
在出现panic的情况下，行为的不同取决于panic是否能合理解决出现的问题，如果不能，整个程序就会终止； 
在实践过程中，应该将panic传递给阻塞组件处理：
```rust
use async_std::task;
fn main() {
    task::block_on(async {
    panic!("test");
    });
}
// thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

当出现panic恐慌时候，程序就会终止(下面的程序不会正常执行)
```rust
use std::time::Duration;
task::spawn(async {
    panic!("test");
});

task::block_on(async {
    task::sleep(Duration::from_millis(10000)).await;
})
// thread 'async-task-driver' panicked at 'test', examples/panic.rs:8:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
// Aborted (core dumped)
```

# async-std examples
https://github.com/async-rs/async-std/tree/master/examples/a-chat

# async异步参考
+ https://github.com/rust-lang/async-book
+ https://book.async.rs/

# async-book example
https://github.com/rust-lang/async-book/tree/master/examples

# async 生态兼容性如何确定
+ 与async I/O、计时器、进程间通信或任务交互的async代码通常取决于特定的异步执行器或反应器
+ 所有其他 async 代码，如异步表达式、组合器、同步类型和流，通常与生态系统无关，
     前提是任何嵌套的future 也与生态系统无关
+ 在开始一个项目前，建议先研究相关的 async 框架和库，以确保与你选择的运行时runtime以及彼此之间的兼容性

# 单线程 VS 多线程执行器 executor 比较
+ async 执行器可以是单线程或多线程的
+ 多线程执行器可以在多个任务上同时取得进展，对于有许多任务的工作负载，它可以大大加快执行速度，
     但在任务之间同步数据通常比较昂贵
+ 在单线程运行时和多线程运行时之间进行选择时候，建议先测量应用程序的性能，做好压力测试
+ 任务可以在创建他们的线程上运行，也可以单独的线程上执行
+ 任务运行时通常提供将任务生成到单独的线程的功能
    - 即使任务在不同的线程上执行，他们也应该是非阻塞的
+ 为了能够在多线程执行器上调度任务task,他们必须是具有Send trait的
+ 一些运行时提供了生成非Send 的任务函数，确保每个任务都在生成它的线程上执行
+ 他们还可以提供将阻塞任务生成到专用线程的函数，这对于运行来自其他库的阻塞同步代码非常有用
