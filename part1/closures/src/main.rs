use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let a = String::from("rust hello");
    // 定义闭包
    let fn_closure = || {
        println!("a = {}", a);
    };
    fn_closure();
    fn_closure(); // 即使在闭包之后依然可以访问变量a，因为闭包默认是使用了引用

    // FnMut闭包trait
    // 当编译器发现闭包改变了执行环境中引用的某个值时候，它就实现了FnMut特征
    let mut a = String::from("hey!");
    let mut fn_mut_closure = || {
        a.push_str(",wolrd");
        println!("a:{}", a);
    };
    fn_mut_closure();
    fn_mut_closure(); // a:hey!,wolrd,wolrd 这里闭包保存了a的上下文环境，这里a是一个可变引用

    // FnOnce 从执行环境获取数据的所有权的闭包，实现了FnOnce特征，一般用于线程生成
    let a = Box::new(23);
    let call_me = || {
        // closure cannot be invoked more than once because it moves the variable `a` out of its environment
        let c = a;
        println!("c = {}", c);
    };
    call_me();
    // this value implements `FnOnce`, which causes it to be moved when called
    // call_me();  // 这里不能再调用了
    let a = 1;
    // thread::spawn(||{
    //     // to force the closure to take ownership of `a` (and any other referenced variables), use the `move` keyword
    //     println!("a = {}",a); // 默认是引用，不获取所有权
    // });

    // move 闭包 将所有权移动到了闭包中了
    thread::spawn(move || {
        println!("a = {}", a);
    });
    println!("a = {}", a); // a 是基础类型，具有copy特征，可以继续使用

    thread::sleep(Duration::from_secs(1));

    println!("ok");

    let v = vec![1, 2, 3, 4];
    // 下面的需要通过move 将v的所有权移动到闭包中
    // let handle = thread::spawn(||{
    //     println!("v = {:?}",v);
    // });
    /*
        note: function requires argument type to outlive `'static`
      --> src/main.rs:52:18
       |
    52 |       let handle = thread::spawn(||{
       |  __________________^
    53 | |         println!("v = {:?}",v);
    54 | |     });
       | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables),
    use the `move` keyword
       |
    52 |     let handle = thread::spawn(move ||{
     */

    // Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v。
    // 然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。
    // 通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。
    // 通过告诉 Rust 将 v 的所有权移动到新建线程，我们向 Rust 保证主线程不会再使用 v
    let handle = thread::spawn(move || {
        println!("v = {:?}", v);
    });
    // 下面不能再执行，因为所有权已经移动到了闭包中了，使得v在主线程中变成了未初始化了，就不能再使用了
    // println!("v = {:?}", v); //  ^ value borrowed here after move
    handle.join().unwrap();

    // 将闭包的执行结果返回给x,这里x是一个闭包函数
    let x = |num| {
        if num > 1 {
            1
        } else {
            2
        }
    };

    let y = x(1);
    println!("y = {}", y);
}

// 函数调用后闭包获得了被释放的内存环境
// 通过 Box 装箱返回Fn，我们提供了一个已知大小的返回值，并允许它离开我们 的栈帧
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包定义
    let expensive_closure = |num| {
        println!("calculating slowly....");
        // 停顿2s
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today,do {} pushups!", expensive_closure(intensity));
        println!("next do {} situps!", expensive_closure(intensity));
        return;
    }

    if random_number == 3 {
        println!("task a break today! remember to stay hydrated!");
    } else {
        println!("today run for {} minutes!", expensive_closure(intensity));
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    // 闭包定义
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly....");
        // 停顿2s
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today,do {} pushups!", expensive_result.value(intensity));
        println!("next do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("task a break today! remember to stay hydrated!");
        } else {
            println!(
                "today run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// 定义泛型结构体Cacher calculation 类型是一个函数闭包
// value 是一个option 存放函数T闭包执行的结果
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 创建一个实例对象
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 如果 self.value 是 None，则会调用 self.calculation 中储存的闭包
    // 将结果保存到 self.value 以便将来使用，并同时返回结果值。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // 当函数没有调用就调用，然后将结果放入option 中缓存起来
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn closure_demo() {
        //=============闭包深入研究=============
        // 闭包定义 参数放在管道||里面 {}是一个表达式
        // 闭包的 || {} 语法
        let plus_one = |x: i32| x + 1;
        assert_eq!(2, plus_one(1));

        let plus_two = |x| {
            let mut res: i32 = x;
            res += 1;
            res += 1;
            res
        };
        assert_eq!(4, plus_two(2));
        let f = super::factory();
        let answer = f(1);
        println!("answer = 6 is {}", answer == 6);
        assert_eq!(6, answer);
        assert_eq!(6, f(1));

        super::generate_workout(12, 5);

        // 定义的时候指定x类型
        let add_one = |x: u32| -> u32 { x * 2 + 1 };

        println!("add_one(2) = {}", add_one(2));

        // 通过自行推导x参数类型
        let ex = |x| x;
        let s = ex(String::from("hello")); // 这里调用后，闭包里面的参数类型就是string了
        println!("{}", s);
        /*
         *第一次使用 String 值调用 ex 时，编译器推断 x 和此闭包返回值的类型为 String。
         *接着这些类型被锁定进闭包 ex 中，如果尝试对同一闭包使用不同类型则会得到类型错误
         */
        // let m = ex(1); // expected struct `String`, found integer
        let m = ex(1.to_string()); // 调用to_string方法进行转化
        println!("{}", m);

        // ====================调用闭包方法============
        super::generate_workout2(12, 6);

        // 闭包中的move
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x; // 编译器自动推导z为 Vec<i32>类型

        // x所有权被移动到闭包中了，后续就不能再使用x
        // - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
        // println!("can not use x here {:?}", x); // - variable moved due to use in closure
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
/*
answer = 6 is true
calculating slowly....
Today,do 12 pushups!
calculating slowly....
next do 12 situps!
add_one(2) = 5
hello
1
calculating slowly....
Today,do 12 pushups!
next do 12 situps!
 */
