# 借用和可变引用
rust引用规则如下：
- Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在。
- 一个引用永远也不会比它的所有者存活得更久

内存细节
- 上面的第一点规避了数据竞争。什么是数据竞争呢？在对数据进行读取的时候，数据争用可能会因为同时存在对数据的写入而产生不同步，这一点往往会出现在多线程编程中
- 第二点引用规则则避免了通过引用而错误的访问到不存在的数据，比如说c语言的悬挂指针
具体代码如下：
```rust
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // 可变引用 f 在这里被 dropped 自动释放
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &foo; //  ---- immutable borrow occurs here
    // cannot borrow `foo` as mutable because it is also borrowed as immutable
    let f2 = &mut foo; // 一次可变借用 ^^^^^^^^ mutable borrow occurs here
    // let f3 = &mut foo; // 又一次可变借用

    // 下面的运行println! 使得可变引用和借用同时存在
    // Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在
    println!("f = {:?},f2 = {:?} ", f, f2); // 这里直接报错 - immutable borrow later used here

    // 为什么下面的代码又可以呢？
    // 因为所有的可变引用都在 do_something 函数内部被释放了
    do_something(&mut foo);
    // 此时我们便可以再创建一个
    do_something(&mut foo);
    // foo 在这里被 dropped 释放
}

```
这段代码运行时会抛出错误,也就是上面提到的借用规则生效了
```rust
 Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `foo` as mutable because it is also borrowed as immutable
  --> src/main.rs:15:14
   |
13 |     let f = &foo; //  ---- immutable borrow occurs here
   |             ---- immutable borrow occurs here
14 |     // cannot borrow `foo` as mutable because it is also borrowed as immutable
15 |     let f2 = &mut foo; // 一次可变借用 ^^^^^^^^ mutable borrow occurs here
   |              ^^^^^^^^ mutable borrow occurs here
...
19 |     println!("f = {:?},f2 = {:?} ", f, f2); // 这里直接报错 - immutable borrow later used here
   |                                     - immutable borrow later used here
```


