# async 异步编程
https://learnku.com/docs/async-book/2018

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
