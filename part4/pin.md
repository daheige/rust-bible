# 什么是Pin
  - Pin 和Unpin 标记一起工作
  - Pin 会保证实现了!Unpin的对象永远不会被移动
  - Pin 类型会包裹指针类型，保证指针指向的值不会被移动
        比如： Pin<&mut T>,Pin<&T>,Pin<Box<T>>,即使T:!Unpin，也能保证T不会被移动

# 为什么要Pin?
```rust
let fut_one = /** ... */
let fut_two = /** ... */
// 1. 这会创建一个 Future trait 的匿名类型
// 2. 提供一个和Future trait poll 方法
use std::{pin::Pin, task::{Context, Poll}};
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

// Pin 可以把 Future Pin到内存中的特定位置防止该内存地址移动发生问题
// 可以在 async 块里安全的创建到值的引用，固定在内存的某个位置
```
