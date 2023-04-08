# iter迭代
- iter()通过引用获取元素。
- iter_mut()用于获取元素的可变引用。
- into_iter()用于获取值的所有权，并在完全迭代后使用实际类型，原始集合将无法再访问

# 关于for循环
```rust
fn main() {
    // .. 运算符号，生成的序列迭代器，不包含结尾元素
    for x in 0..5 {
        println!("current x = {}", x);
    }

    // ..= 运算符生成的序列，包含开始和结束的元素
    for x in 0..=5 {
        println!("x = {}", x);
    }
}

// output:
// current x = 0
// current x = 1
// current x = 2
// current x = 3
// current x = 4
// x = 0
// x = 1
// x = 2
// x = 3
// x = 4
// x = 5

```