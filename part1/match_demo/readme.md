# match 模式匹配
- Rust 有一个非常有用的关键字，用于匹配值的所有可能条件， 并在匹配为真时执行相应代码
- match 是穷尽的，意为所有可能的值都必须被考虑到

下面简单的例子：
```rust
fn main() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // 我们可以匹配多个值
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 我们可以匹配迭代器，包含3,4,5,6,7,8,9
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // 我们可以将匹配数值绑定到变量，通过@操作符和..=迭代匹配
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
        _ => {
            println!("found something else!");
        }
    }
}
```
