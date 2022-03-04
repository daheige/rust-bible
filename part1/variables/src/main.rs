fn main() {
    println!("Hello, world!");
    let mut x = 5; // mut表示x是可以修改的变量，自行推导位i32
    x = 12;
    println!("x = {}", x); // 语句是;结尾的

    // 变量解构
    // a 是一个不可变的变量，b是一个可变的String
    let (a, mut b): (bool, String) = (true, String::from("daheige,hello"));
    println!("a = {},b = {}", a, b);
    // a = false; 这是不允许更改的，因为a不可变
    b.push_str(",world");
    println!("new a = {a},b = {b}"); // new a = true,b = daheige,hello,world
    assert_eq!(a, true); // 比较a == true的宏assert_eq!

    // 定义常量
    // 常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效
    // rust语言中，常量全部大写，下划线分割
    const MAX_POINTS: u32 = 10_000; // 常量默认大写通过下划线_来分割
    println!("max_points = {MAX_POINTS}"); // max_points = 10000

    // 变量遮蔽功能
    // 变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量
    // (在被遮蔽后，无法再访问到之前的同名变量)，
    // 就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    let x = 1;
    println!("x = {x}");

    let x = true; // 后面的这个x将前面的变量给遮蔽了
    println!("new x = {x}");

    // 数据类型，rust在编译的时候就知道变量的类型
    let x = 1.2; // rust默认自行推导为f64
    println!("float x = {x}");

    // y 接着: 指定具体的类型
    let y: f32 = 1.3;
    println!("y = {y}");

    let t = true; // 布尔值true,false
    if t {
        println!("1111");
    }

    // 字符类型
    let c = 'a';
    println!("{}", c);

    // 复合类型 tuple元组,array 数组
    let tup = (1, 2, 3);
    println!("tup: {:?}", tup);

    // 数组类型
    let arr = [1, 2, 3, 4]; // i32类型的数组
    println!("arr:{:?}", arr);

    // 数字运算
    let sum = 5 + 10;
    println!("sum = {sum}");

    let diff = 89.5 - 45 as f64; // 这里的as是强制类型转换
    println!("diff = {diff}");

    let quotient = 56.7 / 32.1;
    println!("quotient = {quotient}");

    let remainder = 34 % 3; // 取模
    println!("remainder = {remainder}");

    // NaN = not a number不是一个数字
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("x is not number");
    }

    // for in遍历序列
    for i in 1..5 {
        // 这里不包含5
        println!("current i = {i}");
    }

    // 遍历字符a-z不包含z
    for i in 'a'..'z' {
        println!("char {i}");
    }

    println!("add(1,2) = {}", add(1, 2));

    another_fn(123);
    // dead_end();
}

// 定义函数,->表示返回的标志，返回结果是一个i32类型
// x，y后面的: 接着指定类型
fn add(x: i32, y: i32) -> i32 {
    x + y // 这里rust中可以省略return直接将表达式作为结果返回
}

// 无返回值的函数定义
fn another_fn(x: i32) {
    println!("x = {x}");
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

// 一个用不退出的loop循环
fn forever() -> ! {
    loop {
        //...
    }
}
