use std::io::stdin;
// 通过macro宏实现一个从终端获取输入的字符串，并打印
macro_rules! scanline {
    ($x:expr) => {{
        stdin().read_line(&mut $x).unwrap();
    }};

    // 另一情况scanline!()
    () => {{
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    }};
}

fn main() {
    let mut input = String::new();
    scanline!(input);
    println!("your input: {}", input);

    let a = scanline!();
    println!("{}", a)
}

/*
123
your input: 123

234
234
 */
