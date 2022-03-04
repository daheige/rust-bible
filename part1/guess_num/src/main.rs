use rand::Rng;
use std::cmp::Ordering;
use std::io;
// 通过use导入本地包，标准包，第三方包

fn main() {
    println!("guess the number!");
    println!("please input your num");
    // 变量只是默认不可变，通过mut关键字指定变量的特性，可变化的变量
    let mut guess = String::new(); // 创建一个可变的字符串String，底层实际上是一个Vec<u8>
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    // 转化为i32数字类型
    // 下面的guess采用了rust shadow 变量遮盖原则，可以用同一个变量遮蔽前面声明的guess
    // 它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量
    // 字符串的 parse 方法 将字符串解析成数字。因为这个方法可以解析多种数字类型，
    // 因此需要告诉 Rust 具体的数字类型,在:后面声明具体的类型
    let guess: i32 = guess.trim().parse().expect("please type a number");
    println!("you guessed:{}", guess);

    // 调用rand的静态方法thread_rng创建一个rand实例，然后生成一个随机数
    // Rust 默认使用 i32，所以它是 secret_num 的类型
    let secret_num = rand::thread_rng().gen_range(1..101); // 随机生成一个数字

    // match模式匹配是否符合条件
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win!"),
    }

    println!("the secret number is:{}", secret_num);
}

/*
guess the number!
please input your num
12
you guessed:12
too small
the secret number is:92
 */
