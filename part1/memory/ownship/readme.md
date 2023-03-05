# 所有权
Rust 试图为程序中值的所有权设定适当的语义。Rust 的所有权规则遵循以下原则。
- 使用 let 语句创建值或资源，并将其分配给变量时，该变量将成为资源的所有者。
- 当值从一个变量重新分配给另一个变量时，值的所有权将转移至另一个变量，原来的变量将失效以便另作他用。
- 值和变量在其作用域的末尾会被清理、释放。需要注意的是，Rust 中的值只有一个所有者，即创建它们的变量。

```rust
#[derive(Debug)]
struct Foo(u32);

fn main() {
    let foo = Foo(12);
    let bar = foo;
    // 当我们把foo赋值给bar后，foo所有权move到了bar上面去了，因此下面的代码运行失败
    // bar 成为Foo 实例的新所有者
    println!("foo is :{:?}", foo); // ^^^ value borrowed here after move
    println!("bar is:{:?}", bar);
}
// 运行出错
error[E0382]: borrow of moved value: `foo`
--> src/main.rs:7:30
|
5 |     let foo = Foo(12);
|         --- move occurs because `foo` has type `Foo`, which does not implement the `Copy` trait
6 |     let bar = foo;
|               --- value moved here
7 |     println!("foo is :{:?}", foo);
|                              ^^^ value borrowed here after move
```
- 每当我们将变量分配给某个其他变量或从变量读取数据时，Rust 会默认移动变量指向的值。
所有权规则可以防止你通过多个访问点来修改值，这可能导致访问已被释放的变量，即使在单线程上下文中，
使用允许多个值的可变别名的语言也是如此。

- 所有权与作用域协同工作。因此，作用域只不过是变量和值存在的环境。
你声明的每个变量都与作用域有关。代码中的作用域是由一对花括号表示的。无论何时使用块表达式都会创建一个作用
域，即任何以花括号开头和结尾的表达式。此外，作用域支持互相嵌套，并且可以在子作
用域中访问父作用域的元素，但反过来不行。

- 请注意，在推断所有权规则时，作用域是一个非常重要的属性。
它也会被用来推断借用和生命周期。当作用域结束时，拥有值的任何变量都会运行相关代码以取消
分配该值，并且其自身在作用域之外是无效的。特别是对在堆上分配的值，drop 方法会被
放在作用域结束标记}之前调用。这类似于在 C 语言中调用 free 函数，但这里是隐式的，
并且可以避免程序员忘记释放值。drop 方法来自 Drop 特征，它是为 Rust 中大部分堆分配
类型实现的，可以轻松地自动释放资源。

## 移动和复制语义
在 Rust 中，变量绑定默认具有移动语义。
在静态类型语言中，这些语义大致分为移动语义和复制语义。
### 移动语义：
通过变量访问或重新分配给变量时移动到接收项的值表示移动语义。
由于Rust 的仿射类型系统，它默认会采用移动语义。
仿射类型系统的一个突出特点是值或资源只能使用一次，而 Rust 通过所有权规则展示此属性。
### 复制语义：
默认情况下，通过变量分配或访问，以及从函数返回时复制的值（例如按位复制）具有复制语义。
这意味着该值可以使用任意次数，每个值都是全新的。

Rust中的移动语义有时会受到限制。幸运的是，通过实现 Copy 特征可以更改类型的行为以遵循复制语义。
基础类型和其他仅适用于堆栈的数据类型在默认情况下实现了上述特征。
```rust
#[derive(Copy, Clone, Debug)]
struct Dummy;
// 复制语义
let a = Dummy;
let b = a;
println!("a = {:?}", a);
println!("b = {:?}", b);
```
上面的代码执行结果：
```
a = Dummy
b = Dummy
```
有趣的是，Copy 特征似乎依赖于 Clone 特征。这是因为 Copy 特征在标准库的定义
如下：
```rust
pub trait Copy: Clone { }
```
`Clone是 Copy 的父级特征，任何实现 Copy 特征的类型必须实现 Clone。我们可以在派生注释中的 Copy 旁边添加 Clone 特征。`

## copy和clone
### Copy
- Copy 特征通常用于可以在堆栈上完全表示的类型，也就是说它们自身没有任何部分位
于堆上。如果出现了这种情况，那么 Copy 将是开销很大的操作，因为它必须从堆中复制
值。这直接影响到赋值运算符的工作方式。如果类型实现了 Copy，则从一个变量到另一个
变量的赋值操作将隐式复制数据。
- Copy 是一种自动化特征，大多数堆栈上的数据类型都自动实现了它，例如基元类型和
不可变引用，即&T。Copy 特征复制类型的方式与 C 语言中的 memcpy 函数类似，后者用
于按位复制值。默认情况下不会为自定义类型实现 Copy 特征，因为 Rust 希望显式指定复
制操作，并且要求开发人员必须选择实现该特征。当任何人都想在自定义类型上实现 Copy
特征时，Copy 还取决于 Clone 特征。
- 没有实现 Copy 特征的类型包括 Vec<T>、String 和可变引用

### Clone
Clone 特征用于显式复制，并附带 clone 方法，类型可以实现该方法以获取自身的副本。
Clone 特征的定义如下:
```rust
pub trait Clone { 
 fn clone(&self) -> Self; 
}
```
- Clone 有一个名为 clone 的方法，用于获取接收者的不可变引用，即&self，并返回相同
类型的新值。用户自定义类型或任何需要提供能够复制自身的包装器类型，应通过实现
clone 方法来实现 Clone 特征。
- 但是 Clone 与 Copy 特征的不同之处在于，其中的赋值操作是隐式复制值，要复制 Clone值，我们必须显式调用 clone 方法。
- clone 方法是一种更通用的复制机制，Copy 是它的一个特例，即总是按位复制。
- String 和 Vec 这类元素很难进行复制，只实现了 Clone 特征。
- `智能指针类型也实现了 Clone 特征，它只是在指向堆上相同数据的同时复制指针和额外的元数据（例如引用计数）`

### 显式clone
```rust
// 在 derive 属性中添加了一个 Clone 特征。
// 有了它，我们就可以在 a 上调用 clone方法来获得它的新副本
#[derive(Copy, Clone, Debug)]
struct DummyInfo {
items: u32,
}
// 显式调用clone
let i = DummyInfo { items: 123 };
let m = i.clone();
println!("current dummy info:{:?}", i);
println!("current m dummy info:{:?}", m);

// 执行结果：
// current dummy info:DummyInfo { items: 123 }
// current m dummy info:DummyInfo { items: 123 }
```

### clone使用的代价
- 复制类型看起来似乎很容易绕过所有权规则，但是由于 Clone 总是复制类型，可能会
调用内存分配器 API，这是一种涉及系统调用，并且开销高昂的操作，因此它无法满足零
成本抽象承诺的所有要点。
- 随着移动语义和所有权规则的实施，在 Rust 中编写程序很快就会变得困难重重。幸运
的是，我们引入了借用和引用类型的概念，它们放宽了规则所施加的限制，但仍然能够在
编译期确保兼容所有权规则。

### 何时在类型上实现 Copy
- 如果类型仅依赖于在其上实现了 Copy 特征的其他类型，则 Copy 特征是为其隐式实现的。
- Copy 特征隐式影响赋值运算符的工作方式。使用 Copy 特征构建自定义外部可见类型需要考虑它是否会对赋值运算符产生影响。如果在开发的早期阶段，你的类型是Copy，后续将它移除之后则会影响使用该类型进行赋值的所有环节。你可以通过这种方式轻松地破坏 API。

### 何时在类型上实现 Clone
- Clone 特征只是声明一个 clone 方法，需要被显式调用
- 如果你的类型在堆上还包含一个值作为其表示的一部分，那么可选择实现 Clone 特征，这也需要向复制堆数据的用户明确表示
- 如果要实现智能指针类型（例如引用计数类型），那么应该在类型上实现 Clone 特征，以便仅复制堆栈上的指针

### ownership实际案例
可以看part1/owner_ship https://github.com/daheige/rust-bible/tree/main/part1/owner_ship
