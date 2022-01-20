fn main() {
    println!("Hello, world!");
    // Rust原生支持UTF-8编码的字符串，这意味着你可以很容易的使用世界各国文字作为字符串内容
    greet_world()
}

fn greet_world() {
    let southern_ger = "Grüß Gott!";
    let chinese = "你好，rust!";
    let en = "hello,wolrd";
    let regions = [southern_ger, chinese, en];
    for region in regions.iter() {
        // 迭代遍历
        println!("region:{region}"); // 这个特性是v1.58.0加入的，低版本不会对变量进行替换处理
    }
}
