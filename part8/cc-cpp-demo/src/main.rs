// 声明这是一个外部函数
extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    println!("Hello, cc-cpp-demo");
    // 调用cpp提供的外部函数multiply
    unsafe {
        println!("multiply(5,7) = {}", multiply(5, 7));
    }
}
