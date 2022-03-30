use std::thread;
use std::time;
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

    // FnOnce 从执行环境获取数据的所有权的闭包，实现了FnOnce特征，一般在线程生成
    let mut a = Box::new(23);
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
    let x = |num|{
        if num > 1{
            1
        }else{
            2
        }
    };

    let y = x(1);
    println!("y = {}",y);
}
