use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // read_file();
    let _ = read_file2();

    let s = vec!["a", "b", "c"];
    let res = get_value(&s);
    if res.is_err() {
        println!("err: {:?}", res.err().unwrap()); // err: "value not found"
        return;
    }

    println!("success");
}

fn read_file() {
    let path = Path::new("data.txt");

    // 如果文件不存在就会抛出错误： thread 'main' panicked at 'open file error:No such file or directory
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("open file error:{}", err),
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s); // 忽略返回结果isize
    println!("content: {}", s);
}

// 通过?来简化错误处理
// 者通过使用运算符“?”将错误传递给程序调用方
// Rust 的错误处理方式是显式的：允许失败的操作通过 Result
// 或 Option 泛型返回两个部分的值
fn read_file2() -> Result<(), std::io::Error> {
    let mut s = String::new();
    let path = Path::new("data.txt");
    let mut file = File::open(&path)?;
    file.read_to_string(&mut s)?;

    // 一句话写完
    //  File::open(&path)?.read_to_string(&mut s)?;
    println!("content: {}", s);
    Ok(())
}

fn get_value(s: &Vec<&str>) -> Result<(), &'static str> {
    let four = s.get(4).ok_or("value not found")?;
    println!("four: {}", four);
    Ok(())
}
