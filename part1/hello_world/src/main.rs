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

    println!("hello");
}

fn add(a : i64,b : i64) -> i64{
    a+b
}

#[test]
fn test_add(){
    assert_eq!(12,add(1,11));
    assert_eq!(10,add(1,9));
    println!("add(1,2) = {}",add(1,2));

}

