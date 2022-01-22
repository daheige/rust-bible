fn main() {
    println!("Hello, world!");
    let a = true;
    if a {
        println!("a = {}", a);
    } else {
        println!("a is false");
    }

    // let..if用法
    let num = if a { 5 } else { 6 };

    println!("num = {}", num);

    // for ..in循环
    for i in 1..5 {
        // 这里不包含5
        println!("index = {}", i);
    }

    let a = [1, 2, 3, 4];
    // 遍历数组,在循环中获取元素的索引
    for (key, value) in a.iter().enumerate() {
        println!("key = {},value = {}", key, value);
    }

    // while循环
    let mut n = 0;
    while n < 5 {
        println!("current num = {}", n);
        n += 1;
    }

    // loop循环
    let mut n = 0;
    loop {
        if n >= 5 {
            break;
        }

        println!("loop current num = {}", n);
        n += 1;
    }
}
