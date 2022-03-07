fn main() {
    println!("Hello, world!");
    // 获取变量的引用，在rust中称之为借用
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // *y表示解引用

    // 不可变引用
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // 借用s1的值，但是不获取所有权
    println!("s = {} len = {}", s1, len); // s = hello len = 5

    let mut s = String::from("hello");
    change(&mut s);
    // let s2 = &mut s; // 可变引用同时只能存在一个
    // println!("change s = {}", s); // ^ immutable borrow occurs here
    println!("change s = {}", s);
    println!("change &s2 = {}", &s); // 不可变应用可以允许多个

    // 可变引用与不可变引用不能同时存在
    // let mut s = String::from("daheige");
    // let s1 = &s; //  -- immutable borrow occurs here
    // let s2 = &s;
    // let s3 = &mut s; // ^^^^^^ mutable borrow occurs here
    // println!("s1 = {},s2 = {},s = {}", s1, s2, s); // s1 -- immutable borrow later used here

    let mut s = String::from("daheige");
    let s1 = &s;
    let s2 = &s;
    println!("s1 = {},s2 = {}", s1, s2); // 引用作用域的结束位置从花括号变成最后一次使用的位置

    let s3 = &mut s; // 可变引用

    println!("s3 = {}", s3);
    // println!("s1 {},s2 {},s3 = {}", s1, s2, s3);

    println!("no_dangle() = {}", no_dangle())
}

// 这里s必须是一个可变引用才可以更改s
// fn change(s: &String) {
//     s.push_str("world");
// }

fn change(s: &mut String) {
    s.push_str(",world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回了s的不可变引用
// } // 这里s离开了作用域后，被丢弃，其内存被释放掉了，这个时候返回值就是一个悬垂引用
/*
 --> src/main.rs:54:16
   |
54 | fn dangle() -> &String {
   |                ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
54 | fn dangle() -> &'static String {
 */

// s 是一个不可变引用
fn calculate_len(s: &String) -> usize {
    s.len()
}
