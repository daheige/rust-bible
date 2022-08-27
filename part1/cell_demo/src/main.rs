/*
Cell 和 RefCell 的绑定的可变性并不是细粒度的，值既可以是不可变的，也可以是可变的，
并且如果它是结构体或枚举，那么它还将包括其所有字段。Cell 和 RefCell 可以将不可变的
内容转换成可变的，允许我们将不可变的结构体中的某个部分定义为可变的
 */

// Cell<T>类型是一种智能指针类型，可以为值提供可变性，甚至允许值位于不可引用之
// 后。它以极低的开销提供此功能，并具有最简洁的 API。 • Cell::new 方法允许你通过传递任意类型 T 来创建 Cell 类型的新实例。
// • get:get 方法允许你复制单元（cell）中的值。仅当包装类型 T 为 Copy 时，该方法
// 才有效。
// • set：允许用户修改内部的值，即使该值位于某个不可变引用的后面。

// RefCell<T>
// 如果你需要某个非 Copy 类型支持 Cell 的功能，那么可以使用 RefCell 类型。
// 它采用了和借用类似的读/写模式，但是将借用检查移动到了运行时，这很方便，但不
// 是零成本的。RefCell 分发值的引用不是像 Cell 类型那样按值返回。
// RefCell 类型为我们提供了以下两种借用方法。
// • 使用 borrow 方法会接收一个新的不可变引用。
// • 使用 borrow_mut 方法会接收一个新的可变引用。

// 注意： Cell 和 RefCell 类型不是线程安全（thread-safety）的。
// 这意味着 Rust 不允许用户在多线程环境中共享这些类型
use std::cell::{Cell, RefCell};

// Cell内部可变性应用
struct Point {
    x: i64,
    y: i64,
    cached_sum: Cell<Option<i64>>,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            cached_sum: Cell::new(None),
        }
    }

    fn sum(&self) -> i64 {
        match self.cached_sum.get() {
            Some(sum) => {
                println!("got cached sum:{}", sum);
                sum
            }
            None => {
                let new_sum = self.x + self.y;
                self.cached_sum.set(Some(new_sum));
                println!("cached sum:{}", new_sum);
                new_sum
            }
        }
    }
}

// RefCell<T> 使用
#[derive(Debug)]
struct BagInfo {
    item: Box<i64>,
}

impl BagInfo {
    fn new(i: i64) -> Self {
        Self { item: Box::new(i) }
    }

    fn ref_cell(self) -> RefCell<Self> {
        RefCell::new(self)
    }
}

fn main() {
    let p = Point::new(1, 2);
    println!("sum result:{}", p.sum());
    println!("sum result:{}", p.sum());
    for i in 0..10 {
        println!("get current index:{},sum result:{}", i, p.sum());
    }

    let bag = BagInfo::new(123).ref_cell();
    let bag1 = &bag;
    let bag2 = &bag;
    // borrow_mut 方法会接收一个新的可变引用
    *bag1.borrow_mut() = BagInfo::new(2);
    *bag2.borrow_mut() = BagInfo::new(12); // bag内部已经改成了12
    let borrowed = bag1.borrow();
    println!("{:?}", borrowed);

    let borrowed = bag1.borrow();
    println!("{:?}", borrowed);
    println!("item: {}", borrowed.item);
}

/*
cached sum:3
sum result:3
got cached sum:3
sum result:3
got cached sum:3
get current index:0,sum result:3
got cached sum:3
get current index:1,sum result:3
got cached sum:3
get current index:2,sum result:3
got cached sum:3
get current index:3,sum result:3
got cached sum:3
get current index:4,sum result:3
got cached sum:3
get current index:5,sum result:3
got cached sum:3
get current index:6,sum result:3
got cached sum:3
get current index:7,sum result:3
got cached sum:3
get current index:8,sum result:3
got cached sum:3
get current index:9,sum result:3
 */
