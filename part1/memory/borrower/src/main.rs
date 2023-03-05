fn main() {
    let a = String::from("daheige");
    let a_ref = &a; // 不可变引用，称为不可变借用
    println!("a = {}", a);
    println!("a_ref = {}", a_ref);

    // 可变引用
    let mut s = String::from("owned string"); // 可变借用在声明的时候，需要原有的变量自身使用关键字mut进行修饰
    let s_ref = &mut s;
    // let b = &s; // ^^ immutable borrow occurs here
    s_ref.push('!'); // 一旦值被可变借用，我们就不能再对它进行其他借用，即使是进行不可变借用。
    println!("s = {}", s);
}
