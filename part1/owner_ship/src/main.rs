fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    println!("s = {s}");
    s.push_str(",world");

    println!("s = {s}");

    // Rust基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，
    // 都被存在栈中，完全无需在堆上分配内存。
    // Rust基本类型都是通过自动拷贝的方式来赋值的
    // 像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。
    // 这意味着没有理由在创建变量 y 后使 x 无效（x、 y都仍然有效）
    let x = 5;
    let y = x;
    println!("x = {},y = {}", x, y);
}
