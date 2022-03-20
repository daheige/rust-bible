fn main() {
    println!("Hello, world!");
    // rust iter 迭代器
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(12);
    for val in v.iter() {
        println!("current val = {}", val);
    }

    // 在iter基础上调用map方法，创建一个新的迭代器，接着将collect消费新的迭代创建一个vec
    // 这里v2的类型由于通过collect()返回的是一个vec,但是类型不确定,这里通过显式指定为i32
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v2);
    for v in v2.iter() {
        println!("v = {}", v);
    }

    // 采用filter方法快速过滤
    // // nth(x)表示第几个索引位置上的元素
    let item: Option<&i32> = v2.iter().filter(|&x| *x % 2 == 0).nth(2); // 返回的是一个Option<T>
    println!("item = {:?}", item); // item = Some(6)

    // 对于v2迭代求和
    // 这里需要指定类型，因为sum返回的是一个泛型S
    let sum: i32 = v2.iter().sum();
    println!("v2 sum = {}", sum);

    // 对数组进行迭代遍历
    println!("iter array");
    let v = [1, 2, 3, 4];
    for (key, val) in v.iter().enumerate() {
        println!("k = {}, v = {}", key, val);
    }

    // 对于Counter进行迭代
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum is: {}", sum);

    let mut c = Counter::new();
    for val in c.next() {
        println!("{}", val);
    }
}

// 实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    // 初始化counter实例
    fn new() -> Self {
        Self { count: 0 }
    }
}

// 实现 标准库中的Iterator trait
// 来创建自定义迭代器
// 需要实现next方法，返回值是一个Option<Item>
// 实现迭代器上的next方法
// 标准库中其他方法，默认已经实现了，不需要再次实现
// 标准库则提供了其它调用 next 的方法的默认实现
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn call_next_directly() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}

#[test]
fn using_other_iterator_counter() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
