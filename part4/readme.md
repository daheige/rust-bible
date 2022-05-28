# async 异步编程
    futures 和 async-std运行机制研究

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

# async/await
    async/.await是rust语言用于编写同步代码像异步代码一样的内置工具
    async将一个代码块转换为一个实现了 Future trait 的状态机。
    虽然在同步的方法中调用阻塞函数会阻塞整个线程，但阻塞的 Futures 将让出线程所有权,
    允许其他的 Futures 运行。

# Future 原理
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

# rust Futures trait 初步认识
```rust
use std::{pin::Pin, task::{Context, Poll}};
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```
仔细阅读，观察到的有以下内容：

    通用的 Output.
    提供 poll 函数，函数功能是允许我们查看当前计算的状态
    （暂时忽略 Pin 和 Context，先不对它们作深入理解）

每次调用 poll() 都会导致以下两种情况的其中一种：

    1.计算已经完成， poll 会返回 Poll::Ready
    2.计算尚未完成执行，poll 会返回 Poll::Pending
上述机制使我们可以从外部检查 Future 是否仍有未完成的工作，或最终是否完成并给出返回值。
最简单 (但不是最有效) 的方法是不断循环执行 poll 函数。
这种办法当然是有优化的可能，而这正是一个好的运行时要做的。
请注意，在第 1 种情况发生后再次调用 poll 可能会导致混乱。

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

# async-std 异步概念
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

task任务在正常情况下，报告错误；如果不能正常上报，应该是Result<T,E> 模式
在出现panic的情况下，行为的不同取决于panic是否能合理解决出现的问题，如果不能整个程序就会终止；
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
https://learnku.com/docs/async-book/2018
https://book.async.rs/
