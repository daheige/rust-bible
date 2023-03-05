fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    println!("s = {s}");
    s.push_str(",world");
    s.push_str(",daheige");

    println!("s = {s}");

    // ================变量和数据交互方式1： 移动move==============
    // Rust基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，
    // 都被存在栈中，完全无需在堆上分配内存。
    // Rust基本类型都是通过自动拷贝的方式来赋值的
    // 像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。
    // 这意味着没有理由在创建变量 y 后使 x 无效（x、 y都仍然有效）
    let x = 5;
    let y = x; // 整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
    println!("x = {},y = {}", x, y);

    // 对于string底层是一个存放字符串内容的内存指针，一个长度，一个容量
    let s1 = String::from("hello");
    let s2 = s1; // s1不再有效了，s1离开作用域后就自动清理了s1

    // println!("s1 :{},s2:{}", s1, s2); // ^^ value borrowed here after move
    // s1移动到了s2 因为只有 s2 是有效的，当其离开作用域，s1就释放自己的内存，完毕
    println!("s1:{}", s2);

    // 变量和数据交互方式2： 克隆
    let s1 = String::from("abc");
    let s2 = s1.clone(); // 深复制

    // 当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。
    // 你很容易察觉到一些不寻常的事情正在发生
    println!("s1:{},s2:{}", s1, s2);

    let i = 12;
    makes_copy(i);
    println!("i = {}", i);
    let s = String::from("abc");
    // takes_and_give_back(s); // - value moved here 这里s的所有权已经转移到了函数内部
    // println!("s = {}", s); // ^ value borrowed here after move 这里不能再使用了

    let s2 = takes_and_give_back(s);
    println!("s2 ={}", s2);

    let mut s = String::from("hello");
    println!("s len = {}", cal_len(&s));
    change(&mut s);
    // 对于可变引用 在同一时间只能有一个对某一特定数据的可变引用。
    // 这些尝试创建两个 s 的可变引用的代码会失败
    // let s2 = &mut s; // ------ first mutable borrow occurs here
    // let s3 = &mut s; //  ^^^^^^ second mutable borrow occurs here
    // println!("s2 = {},s3 = {}", s2, s3);

    // 在拥有不可变引用的同时拥有可变引用。不可变引用的用户可不希望在他们的眼皮底
    // 下值就被意外的改变了！然而，多个不可变引用是可以的，
    // 因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
    let s2 = &s;
    println!("s2 = {}", s2);
    println!("s = {}", s);
}

// i是i32属于标准的copy类型，将i传递给函数后，原来的变量依旧可以继续使用
fn makes_copy(i: i32) {
    println!("current i: {}", i);
}

// s的所有权会移动到函数内部中，所以这里将s重新进行了返回处理
// 将s的所有权转移到了函数里面，然后又返回了s 新变量
fn takes_and_give_back(s: String) -> String {
    s
}

// 引用，是只读引用，拿来用下，但是不转移所有权
// & 符号就是 引用，它们允许你使用值但不获取其所有权
// 因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
// 创建一个引用的行为称为 借用（borrowing）。
// 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。我们并不拥有它。
fn cal_len(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权

// 创建可变引用通过 &mut方式创建
fn change(s: &mut String) {
    s.push_str(",world");
}
