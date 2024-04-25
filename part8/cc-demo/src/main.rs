extern "C" {
    fn foo();
}

// 调用c提供的foo函数
pub fn call() {
    unsafe {
        foo();
    }
}
fn main() {
    println!("cc call begin...");
    call();
}
