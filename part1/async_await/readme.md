# 关于协程
    协程的引入其实就是为了缓解这一艰巨的挑战：它通过引入一个逻辑上抽象的概念来简化编程模型。
    某种程度上说，可以将一个协程看作是一段可以以非阻塞的方式高效执行的代码；
    而多个协程之间可以通过特定关键字或者语句的方式进行组合，以便程序员可以直接写出看起来
    是同步执行而实际上底层却是被异步调度执行的代码。

# 关于Future
    不同的编程语言和环境对于协程的具体实现可以是大相径庭的，只是大多编程语言
    多通过Future/Promise这样的机制来实现。一个Future代表一个现在可能还
    没有结果（或者在等待IO完毕或者还没有被调度到CPU上去实际执行），但是将来
    会返回某个结果的抽象过程。将来会返回的结果抽象为Promise，即承诺一定会返回的
    某个东西。这两个概念如果存在，一定是成对存在的。

# 关于Executor
    异步的Future最后必然要被底层的调度程序来驱动和执行，这个负责执行用户定义的
    抽象的Future的实体成为Executor，它底层的执行体可以是按需创建的线程池，也
    可以是共享固定大小的线程池甚至于单个线程等。最好的Executor抽象应该是
    从JDK5开始流行开来的，目前已经成为一个跨编程语言的抽象。

# async/await
    对协程的组合和等待则通过async/await这样的关键字来标识。async用于标记某个
    可以返回Future的语句或者调用，await用来异步的等待某个Future的执行结果，
    即await的逻辑看起来同步的，但是其实依赖于底层协程的调度结果，即实际执行
    完毕之后才能得到结果。
    如果有后续的逻辑处理依赖于await的结果，那么它们将被自动调度为顺序执行。

# Rust的Future
    Future本身是一个抽象的概念，对应的Rust语言里面的动态抽象机制是Trait，
    因此毫不意外地看到，Rust语言定义的Future就是一个Trait。
    逻辑上来说，Rust的Future可以简单的被认为是定义为如下的形式
```rust
// Warning: this is not the offical definition
trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}
```
该Trait的核心是定义了一个输出的类型，即实际执行结束之后的结果返回类型，
以及一个相关的poll函数。顾名思义这是一个底层的协程Executor提供的”轮询”执行结果的函数，
返回的结果是操作是否完毕，它是一个枚举类型
```rust
enum Poll<T> {
    Ready<T>,
    Pending,
}
```
Rust自身的类型安全机制很好地封装了其它语言中不得不需要的Promise:
这里我们不需要专门的Promise类型，仅仅根据枚举值来做模式匹配即可。

用户程序可以主动调用这个轮询函数，驱动底层的Executor来执行一次调用，
然后检查实际结果；只是绝大部分情况下，这并不是最佳的使用方式：
因为这一的执行方式和协程底层封装的思路想违背了。大部分情况下，
用户需要的是关注多个协程之间的互相组合，确定它们之间的逻辑依赖关系即可，
Executor的驱动甚至可以是自动封装驱动的。

# wake函数
如果没有这个函数，Executor在需要多次驱动用户提供的Future来持续运行的时候，
将没有办法知道如何继续下去。这个函数的作用就是一个唤醒回调，在Future被实际执行的时候，
如果一次没有执行完毕，那么下次被调用的时候，可以从上次中断的地方继续执行下去。

# rust标准库中的Future
```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut self>, ctx: &mut Context<'_>) -> Poll::(Self::Output);
}
```

# Rust的await和async
    这次加入1.36稳定版的功能体验在语言层面的两个关键字async和await

# async
async用于声明一个代码块为返回一个Future，通过在某个普通的函数声明前面加上async，
Rust可以自动完成返回类型到Future类型的封装和转换，如下面的代码的返回值会是一个Future.
```rust
async fn do_something() {
    //some heavy operation
}
```
Rust本身的Executor库也提供了阻塞执行的方法，允许我们在当前的调用线程里面阻塞执行
直到封装的异步执行块允许完毕，即如下的代码:
```rust
let fut = do_something();
block_on(fut);
// proceeds until wrapped something is executed
```

# await
await语句可以作用在Future上，用于非阻塞方式的同步逻辑，即异步地等待作用的Future对象的完成，
然后读取返回的结果，考虑如下的三个异步的Future执行块，前两个有先后依赖而第三个可以同时进行：
```rust
async fn learn_song() -> Song {
    //dom something
    Song
}
async fn sing_song(song: Song) {
    //sing the song
}


async fn dance() {
    //dance
}
```

