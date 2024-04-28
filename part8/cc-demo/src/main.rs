use std::error::Error;
// 导入c语言模块的的char
use std::ffi::CString;

// 引入标准输入和输出模块
use std::io::{stdin, stdout, Write};
use std::os::raw::c_char;

// 在 Rust 中想使用cc库，可以通过在extern块中，定义外部函数签名。
// 使用cc时候，一个静态库文件将被生成（在这情况下，就是libfoo.a），也即是foo.c会生成libfoo.a文件
// 链接的文件为foo
#[link(name = "foo")]
extern "C" {
    // 这个标识标识在rust语言中通过ffi方式引入C语言的代码
    // 也就是说通过extern块引入C语言定义的代码
    fn foo();
    fn greet(name: *const c_char);
    fn print_app_info();
}

// 调用c提供的foo函数
pub fn call() -> Result<(), Box<dyn Error>> {
    // 调用foo函数，它是相对来说安全的函数，所以这里需要加上unsafe关键字
    unsafe {
        foo();
    }

    let name = prompt("what's your name?")?;
    // 调用c语言的函数
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}

fn prompt(s: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", s); // 需要输入之前的提示语
    stdout().flush()?;
    // 从终端中读取输入的内容
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    // 调用函数，由于这个函数是第三方中定义（也就是foo.c文件中定义的函数)，
    // 它是相对来说不安全的，所以需要调用的地方加上unsafe关键字，明确告诉这非安全的调用
    unsafe {
        print_app_info();
    }

    println!("cc call begin...");
    let _ = call();
}
