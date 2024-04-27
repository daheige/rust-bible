// 关闭混淆功能以让c程序可以找到调用的函数
// extern 默认导出为C ABI
#[no_mangle]
pub extern "C" fn say_hello(){
    println!("call func from rust");
    println!("hello,{}","world");
}
