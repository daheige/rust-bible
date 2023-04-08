# immutable borrow 和 mut borrow
不可变借用：当我们在类型之前使用运算符&时，就会创建一个不可变借用。
可变借用：可以使用&mut 运算符对某个值进行可变借用。通过可变借用，你可以改变该值。

# rust 借用规则
- 一个引用的生命周期可能不会超过其被引用的时间。这是显而易见的，因为如果它
  的生命周期超过其被借用的时间，那么它将指向一个垃圾值（被销毁的值）。
- 如果存在一个值的可变借用，那么不允许其他引用（可变借用或不可变借用）在该
  作用域下指向相同的值。可变借用是一种独占性借用，可变借用和不可变引用不能同时出现。
- 如果不存在指向某些东西的可变借用，那么在该作用域下允许出现对同一值的任意
  数量的不可变借用。

Rust 中的借用规则由编译器中被称为借用检查器的组件进行分析。Rust 社区把处理借用错误戏称为和借用检查器作斗争。
# match匹配中的借用
在 match 表达式中，默认情况下会对匹配的值进行移动，除非它 是 Copy 类型。

# 基于借用规则的方法类型
借用规则还规定了如何定义类型的固有方法和特征的实例方法
- &self 方法：这些方法只对其成员具有不可变的访问权限。
- &mut self 方法：这些方法能够可变地借用 self 实例。
- self 方法：这些方法拥有调用它的实例的所有权，并且类型在后续调用时将失效。
对于自定义类型，相同的借用规则也适用于其作用域成员。

# 方法的参数借用规则
对于方法的参数借用，除非你有意编写一个应该在结束时移动或删除 self 的
函数，否则总是应该使用不可变的借用方法，即将&self作为第 1 个参数。

# 关于借用和可变引用的规则
rust引用规则如下：
- Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在。
- 一个引用永远也不会比它的所有者存活得更久

```rust
// 借用和可变引用使用

// 定义结构体Foo
#[derive(Debug)]
struct Foo {
    x: i32,
}

// 返回所有权
fn get_foo_ownship() -> Foo {
    Foo { x: 42 }
    // 所有权被移出
}

// 借用
fn say(f: &Foo) {
    println!("hello,{}", f.x);
}

// 传递可变引用，改变x的值
fn change(f: &mut Foo) {
    f.x = 12;
}

fn main() {
    let mut foo = get_foo_ownship();
    // foo 成为了所有者
    // foo 在函数域结尾被 dropped 释放
    println!("foo.x = {}", foo.x);
    say(&foo); // 借用的方式传递参数
    change(&mut foo); // 改变foo
    say(&foo);
    let f = &mut foo;
    let f3 = &mut foo;
    println!("{:?}",f3);

    // 借用规则如下：
    // 1、同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    // 2、引用必须总是有效的
    // 可变引用与不可变引用不能同时存在
    // let mut s = String::from("daheige");
    // let s1 = &s; //  -- immutable borrow occurs here
    // let s2 = &s;
    // let s3 = &mut s; // ^^^^^^ mutable borrow occurs here
    // println!("s1 = {},s2 = {},s = {}", s1, s2, s); // s1 -- immutable borrow later used here
}
```

# 借用和可变引用
https://github.com/daheige/rust-bible/tree/main/part1/ref_var
