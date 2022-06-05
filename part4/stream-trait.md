# Stream trait
    Stream trait和标准库的Future trait 类似，但它可以在完成前产生多个值，这个和标准库Iterator trait相似
```rust
trait Stream {
    type Item;
    
    // attempt to resolve the next item in the stream
    // returns `Poll::Pending` if not ready,`Poll::Ready(Some(x)) if a value
    // is ready,and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

```
+ 与同步的Iterator类似，有多种方法迭代和处理Stream中的值： 
    - 组合器风格的： map,filter,fold
    - 相应的 early-exit-on-error 版本： try_map,try_filter,try_fold

+ for循环无法和Stream 一起使用
+ 命令式的while let 和 next/try_next 函数可以与Stream 一起使用
