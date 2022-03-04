use std::io;

fn main() {
    println!("Hello, world!");
    let a = [1, 2, 3, 4, 5];
    println!("please enter an array index.");
    let mut index = String::new();
    // read_line返回值是一个Result<T,E>
    // 要么有值，要么发生错误，当发生来错误就通过expect抛出异常，停止运行
    io::stdin().read_line(&mut index).expect("index invalid");

    // 字符串通过parse转换为具体的类型
    // let index: usize = index.trim().parse().expect("index was not a number");
    // println!("index: {}", index);
    // println!("current ele:{}", a[index]);

    // 通过match模式匹配方式控制逻辑
    let index: Result<usize, _> = index.trim().parse();
    // parse方法返回的是一个Result<T,E>
    // T表示成功的值，E表示发生错误的err
    match index {
        Ok(index) => {
            println!("num:{}", index);
            if index >= a.len() {
                println!("index invalid");
                return;
            }

            println!("current ele:{}", a[index]);
        }
        Err(err) => println!("err:{}", err),
    }
}
