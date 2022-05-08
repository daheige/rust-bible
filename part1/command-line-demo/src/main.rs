use std::str::FromStr;
use std::env;

fn main() {
    println!("Hello, world!");
    let mut numbers = Vec::new(); // numbers 是一个可变类型的vec
    for arg in env::args().skip(1) {
        // 将参数解析到i64 然后添加到numbers中
        numbers.push(i64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0{
        eprintln!("Usage: input number...");
        std::process::exit(1);
    }

    println!("numbers:{:?}",numbers);
    let mut sum:i64 = 0;
    for m in &numbers {
        println!("m = {}",m);
        sum += m;
    }

    println!("sum = {}",sum);

}

/*
 *
cargo run 12 13
*/

