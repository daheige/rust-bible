// let 变量解构
#[derive(Debug)]
enum Food {
    Pizza,
    Salad,
}

#[derive(Debug)]
enum PaymentMode {
    Bitcoin,
    Credit,
}

#[derive(Debug)]
struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode,
}

struct Person(String);

enum Container {
    Item(u64),
    Empty,
}

struct ContainerInfo {
    items_count: u32,
}

// 可变引用的解构
fn increment_item(ContainerInfo { mut items_count }: &mut ContainerInfo) {
    items_count += 1;
    println!("items_count:{}", items_count);
}

// 引用解构
fn calculate_cost(ContainerInfo { items_count }: &ContainerInfo) -> u32 {
    let rate = 67;
    rate * items_count
}

fn main() {
    // 1. let 解构操作
    let food_order = Order {
        count: 120,
        item: Food::Pizza,
        payment: PaymentMode::Credit,
    };

    // 变量被解构的方式取决于右边的值是不可变引用、可
    // 变引用，还是拥有所有权的值，或者我们如何使用 ref 或 mut 模式在左侧引用它
    let Order { count, item, .. } = food_order;
    println!("count:{},item:{:?}", count, item);

    // 如果想通过不可变引用来构造成员，那么可以在 food_order
    // 之前放置一个运算符“&”或者使用关键字 ref 或 mut 进行修饰
    // let Order { count, item, .. } = &food_order; // 不可变的引用解构方式

    let ref food_order2 = Order {
        count: 120,
        item: Food::Pizza,
        payment: PaymentMode::Credit,
    };
    // 不关心的字段可以使用“..”予以忽略
    // 解构的轻微限制
    // 在于我们不能自由选择单个字段的可变性。所有变量必须具有相同的可变性——要么都是
    // 不可变的，要么都是可变的。请注意，ref 通常不用于声明变量绑定，而主要用于 match 表达式中
    let Order { count, item, .. } = food_order2;
    println!("不可变引用 count:{},item:{:?}", count, item);

    // ref在match模式匹配中使用，解构引用
    let a = Person("daheige".to_string());
    match a {
        Person(ref name) => println!("your name:{}", name),
        _ => println!("not match"),
    }

    // let 解构枚举
    let maybe_item = Container::Item(0u64);
    // if let模式匹配
    let has_item = if let Container::Item(0) = maybe_item {
        true
    } else {
        false
    };
    println!("has_item:{}", has_item);

    // 2. 函数参数的解构操作
    let mut c = ContainerInfo { items_count: 10 };
    increment_item(&mut c);
    println!("c.items_count:{}", c.items_count);

    let total_cost = calculate_cost(&ContainerInfo { items_count: 12 });
    println!("total_cost:{}", total_cost);

    // 3. loop循环作为表达式
    let mut i = 0;
    let a = loop {
        i += 1;
        if i >= 10 {
            break i;
        }
    };
    println!("a = {}", a);

    // 4.类型推断
    let mut v = vec![]; // Vec<i32>
    v.push(2);
    println!("v = {:?}", v);

    // 5. 有所有权的String和借用&str
    // String在标准库中底层是Vec<u8>
    //
    // 调用 to_string 方法创建字符串，该方法来自 ToString 特征
    let s = "hello".to_string(); // String类型
    println!("s = {}", s);

    // 调用 from 方法创建一个字符串，它是String上面的关联方法，可以直接使用
    let b = String::from("abc"); // String类型

    // 通过 ToOwned 特征的to_owned 特征方法创建的，该特征是&str 类型——基于字面字符串而实现
    let c = "world".to_owned(); // String类型
    println!("b = {},c = {}", b, c);
    // 由于 String 是在堆上分配的，因此它可以被修改，并且能够在运行时根据需要增加长
    // 度。这意味着字符串在执行此操作时会产生相应的开销，因为它们可能会在你不断添加字
    // 节时重新分配内存。堆分配是一种开销相对昂贵的操作，但幸运的是，Vec 分配内存时（容
    // 量限制加倍）使该成本会按使用量平摊而降低

    // &str借用字符串
    //
    // 所有声明的变量在进入堆栈时都是
    // 以值本身或者指向堆分配类型的指针的形式存在的。所有堆栈分配的值都需要具有已知的
    // 适当大小，因此无法初始化 str

    // 字符串是有效的 UTF-8 编码字节序列，单个字节并不等同于单个字符。在 UTF-8 中，单个字符也
    // 可以由多个字节表示，所以字符串不能像别的语言那样通过index索引访问字符串
    let s = String::from("hello");
    // let first_char = hello[0];// ^^^^^ not found in this scope 这里直接就panic掉了
    // 可以通过字符串切片的方式访问
    let first_three = &s[0..3];
    println!("{:?}", first_three);
    // 对字符串的所有字符进行迭代使用chars方法
    for c in s.chars() {
        // 这里chars方法会以适当的unicode边界返回字符串中的字符
        println!("current char:{}", c);
    }

    // 6.在函数中使用字符串
    say_hello("daheige");
    let s = String::from("daheige");
    say_hello(&s); // 这里&String会自动转换为&str，是因为String为str类型实现了Deref所以自动解引用

    // 7.字符串拼接
    let a = "foo";
    let b = "abc";
    // 在rust中不鼓励隐式进行堆分配
    // let c = a + b; //  `+` cannot be used to concatenate two `&str` strings
    // 编译器建议将第一个字符串a转换为所有权的字符串来拼接
    // String和&str之间的区别：&str自身能够北编译器识别，而String是标准库中的自定义类型
    let c = a.to_string() + b;
    println!("c = {}", c);
}

// 对刚接触 Rust 的程序员来说，辨别&str 和 String 的应用场景会存在一些困惑。最佳的
// 做法是尽可能使用带有&str 类型的 API，因为当字符串已经分配到某处时，只需引用该字
// 符串就可以节省复制和分配的成本。在程序中传递&str 几乎是零成本的：它几乎不会产生
// 分配成本，也不会复制内存。
fn say_hello(s: &str) {
    println!("hello:{}", s);
}

// 'static表示静态的声明周期修饰，这里表示字符串在程序存续时间保持不变
// &表示它指向字符串文本的指针，而str表示不定长类型
// &str类型都是在堆上借用任何拥有String类型的字符串切片，它一旦创建就无法更改，默认情况下不可变的
fn get_str_literal() -> &'static str {
    "hello"
}
