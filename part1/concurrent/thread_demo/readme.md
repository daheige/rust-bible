# thread并发
Rust 内置了两种流行的并发模型：通过同步共享数据和通过消息传递共享数据。

# spawn使用

```rust
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
// use std::thread::Builder;

fn main() {
    // 开辟子线程
    // 返回值是一个JoinHandler是子线程的句柄，可用于连接线程
    let child = thread::spawn(|| {
        println!("hello thread");
    });

    // 调用join会阻塞当前线程，并在执行join调用后的任何代码之前等待子线程执行完毕
    let value = child.join().expect("failed joining child thread");
    // 上面的闭包函数执行完毕返回()
    println!("value :{:?}", value);

    // 自定义线程
    // 设置线程名称和设置堆栈大小
    /*下面的代码运行panic
    thread 'worker thread' has overflowed its stack
    fatal runtime error: stack overflow
    */
    // let my_thread = Builder::new()
    //     .name("worker thread".to_string())
    //     .stack_size(4 * 1024);
    // let handler = my_thread.spawn(|| {
    //     panic!("oops!");
    // });
    // let child_status = handler.unwrap().join();
    // println!("child status:{:?}", child_status);

    // 同时开辟多个子线程
    // Rust 内置了两种流行的并发模型：通过同步共享数据和通过消息传递共享数据。
    /*
    let nums = vec![1, 2, 3, 4, 5];
    let mut handlers = Vec::new();
    for n in 0..nums.len() {
        let mut handler = thread::spawn(|| {
            //  下面的nums被移动到线程中，但是用了一次后，就不能使用
            // 因此需要共享所有权

            // ---- use occurs due to use in closure
            println!("current value:{}", nums[n]);
        });

        handlers.push(handler);
    } */

    // 修复方式，通过Arc共享所有权在多个线程中通信
    let nums = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handlers = Vec::new();
    for n in 0..nums.len() {
        let ns = Arc::clone(&nums); // 这里复制了&nums的副本
        let handler = thread::spawn(move || {
            println!("current value:{}", ns[n]);
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    // 修改线程中的共享数据
    /* let mut nums = Arc::new(vec![]);
    for n in 0..5 {
        let mut ns = nums.clone(); //   ^^^^^^^^^^^^ value borrowed here after move

        // 通过 Arc 包装的相同 nums，但是无法改变它
        // 它不能正常运作是因为复制 Arc 分发了对内部值的不可变引用。要改变来自多线程的
        // 数据，我们需要使用一种提供共享可变性的类型，就像 RefCell 那样。但与 Rc 类似，RefCell
        // 不能跨多个线程使用
        // 我们需要使用它们的线程安全的变体，例如 Mutex 或 RwLock 包装器类型
        thread::spawn(move || {
            nums.push(n);
        });
    } */

    // 在单个子线程中使用Mutex互斥锁
    // 互斥锁是一个守护对象，线程获取该对象以保护要由多个线程共享
    // 或修改的数据。它的工作原理是通过锁定值来禁止一次访问多个线程中的值。如果其中一
    // 个线程对互斥锁类型执行了锁定，那么任何其他线程都不能运行相同的代码，直到持有该
    // 锁定的线程完成为止
    let m = Mutex::new(0); // 对数据的锁粒度
                           // 这里move是将m移动到闭包中
    let c = thread::spawn(move || {
        {
            *m.lock().unwrap() += 1;
        }

        // 读取m并返回
        let updated = *m.lock().unwrap();
        // 线程执行完毕后的返回值
        updated
    });

    let updated_value = c.join().unwrap(); // 等待子线程执行完毕，将返回值给updated_value
    println!("updated_value: {:?}", updated_value);

    // 但是当多个线程尝试访问该值时，这将无效，因为
    // Mutex 类型不提供共享可变性。为了允许 Mutex 类型中的值支持在多线程环境下被修改，
    // 我们需要将它包装成 Arc 类型。
    // 接下来让我们看看如何做到这一点：在互斥锁上执行锁定将阻止其他线程调用锁定，直到锁定消失为止
    // 通过Arc+Mutex将数据进行包裹，达到线程安全
    let nums = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0..5 {
        let ns = nums.clone();
        let handler = thread::spawn(move || {
            let mut v = ns.lock().unwrap();
            v.push(i);
        });
        childs.push(handler);
    }
    // 等待每个线程之心完毕
    for handler in childs {
        handler.join().unwrap();
    }

    println!("nums: {:?}", nums); // nums: Mutex { data: [0, 1, 2, 4, 3], poisoned: false, .. }
}

```
