extern "C" {
    fn foo();
}

pub fn call() {
    unsafe {
        foo();
    }
}
fn main() {
    println!("cc call begin...");
    call();
}
