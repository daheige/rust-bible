use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);

    let value = m.get("a"); // get方法返回值是一个option
    if value.is_none() {
        println!("a not found");
        return;
    }

    println!("a = {}", value.unwrap());

    // 通过match模式匹配来处理
    let b = match m.get("b") {
        Some(val) => val + 1,
        None => 0,
    };

    println!("b = {}", b);

    // 另外一种简单的写法 if let some语法
    // 这种写法，适合只有一种情况的match比较推荐
    let i = if let Some(val) = m.get("b") {
        val + 1
    } else {
        0
    };
    println!("i = {}", i);

    /*
    另一种不太安全的方法是在 Option 上调用解压缩方法，即 unwrap()和
    expect()方法。如果返回的结果是 Some，那么调用这些方法后将提取内部的值；如果返回
    的结果是 None，则会发生异常。仅当我们确定 Option 值确实包含某个值时，才推荐使用
    这些方法
    */

    let c = m.get("c").unwrap();
    let c = m.get("c").expect("c not found"); // thread 'main' panicked at 'c not found', src/main.rs:41:24
    println!("c = {}", c);
    /* 当不存在就直接抛错，程序终止运行
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     */
}
