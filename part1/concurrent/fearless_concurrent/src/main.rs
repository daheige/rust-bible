use std::thread;
use std::time::Duration;

// Rust 的并发原语依赖于本机的操作系统线程，它在标准库中的 std::thread 模块中提供了线程 API。
fn main() {
    println!("Hello, world!");
    // 1.使用 spawn 创建新线程
    // thread::spawn(||{
    //     for i in 1..10{
    //         println!("spawn thread: current i = {}",i);
    //         thread::sleep(Duration::from_millis(1)); // 在独立线程中停顿1ms
    //     }
    // });
    //
    // for i in 1..5{
    //     println!("main thread: i = {}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    /*当主线程退出了，子线程就会跟着退出
    Hello, world!
    main thread: i = 1
    spawn thread: current i = 1
    main thread: i = 2
    spawn thread: current i = 2
    spawn thread: current i = 3
    main thread: i = 3
    main thread: i = 4
    spawn thread: current i = 4
    spawn thread: current i = 5*/

    // thread::spawn 的返回值类型是 JoinHandle。
    // JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn thread: current i = {}", i);
            thread::sleep(Duration::from_millis(1)); // 在独立线程中停顿1ms
        }
    });

    for i in 1..5 {
        println!("main thread: i = {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();

    let v = vec![1, 2, 3, 4, 5];
    // thread::spawn(||{
    //     for i in &v {
    //         println!("i = {}",i);
    //     }
    // });

    // 通过move将v的所有权移动到来线程里面
    let handler = thread::spawn(move || {
        for i in &v {
            println!("i = {}", i);
        }

        println!("v = {:?}", v);
    });

    handler.join().unwrap(); // 这个是等待子线程执行完毕，它放在这里会阻塞，等待上面的线程执行完毕
    for i in 1..5 {
        println!("main thread: i = {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("main thread will exit...");
}

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    #[test]
    fn message_pass() {
        let (tx, rx) = mpsc::channel(); // 创建一个无限缓冲的通道channel
                                        // 发送消息
        thread::spawn(move || {
            // 这里需要move 将tx所有权移动到闭包中
            let v = String::from("abc");
            tx.send(v).unwrap();
            // 一旦v被发送到通道中，v就会变成未初始化，不能再使用
            // println!("v = {}",v); // ^ value borrowed here after move
        });

        // 接收消息,单个消息接收通过recv方法进行接收,它的返回值是Result<T, RecvError>
        let msg = rx.recv().unwrap();
        println!("msg:{}", msg);
    }

    // 发送多个值
    #[test]
    fn message_pass2() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let v = vec![1, 2, 3, 4];
            for val in v {
                tx.send(val).unwrap();
            }
        });

        // 下面就会阻塞，等待发送者发送完毕后就会消费
        for msg in rx {
            println!("msg: {}", msg);
        }
    }

    #[test]
    fn message_mp() {
        let (tx, rx) = mpsc::channel();

        // 通过tx来克隆一个生产者
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let s = String::from("abc");
            tx1.send(s).unwrap();
        });

        thread::spawn(move || {
            let s = vec![String::from("hello"), String::from("rust")];
            for v in s {
                tx.send(v).unwrap();
            }
        });

        for msg in rx {
            println!("recv msg: {}", msg);
        }
    }

    #[test]
    fn mutex_test() {
        let mut m = Mutex::new(5);
        {
            // 使用 lock 方法获取锁，以访问互斥器中的数据。
            // 这个调用会阻塞当前线程，直到我们拥有锁为止
            //
            // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。
            // 在这种情况下，没人能够再获取锁，所以这里选择 unwrap
            // 并在遇到这种情况时使线程 panic。
            // 当调用lock返回一个叫做MutexGuard的智能指针
            // 这个智能指针实现了Deref 来指向其内部数据，同时也提供Drop实现
            // 当MutexGuard离开作用域自动释放lock
            let mut num = m.lock().unwrap();
            *num += 6;
        }

        // 当mutex lock离开作用域，自动释放锁
        println!("m = {:?}", m);
        // 获取可变引用数据
        println!("m = {:?}", m.get_mut().unwrap());
        // m = Mutex { data: 11, poisoned: false, .. }
        // m = 11
    }

    // #[test]
    // fn mutex_share_data(){
    //     let counter = Mutex::new(0);
    //     // 上面一行不能编译，抛出下面的错误
    //     // ------- move occurs because `counter` has type `Mutex<i32>`,
    //     // which does not implement the `Copy` trait
    //     let mut handlers = vec![];
    //     for i in 0..10{
    //         // 创建多个线程
    //         let handler = thread::spawn(move ||{
    //             // 上面一行报错
    //             // ^^^^^^^ value moved into closure here, in previous iteration of loop
    //             let mut num = counter.lock().unwrap();
    //             *num +=i;
    //         });
    //
    //         // 将handle join句柄加入到handlers
    //         handlers.push(handler);
    //     }
    //
    //     for handler in handlers{
    //         handler.join().unwrap();
    //     }
    //
    //     println!("result: {}",*counter.lock().unwrap());
    // }

    // Rc机制仅在单线程中是安全的，下面的程序依旧不能编译成功
    //#[test]
    // fn mutex_share_data(){
    //     let counter = Rc::new(Mutex::new(0));
    //     // 上面一行不能编译，抛出下面的错误
    //     // ------- move occurs because `counter` has type `Mutex<i32>`,
    //     // which does not implement the `Copy` trait
    //     let mut handlers = vec![];
    //     for i in 0..10{
    //         // 创建多个线程
    //         // 抛出下面的错误信息，因为 Rc<T> 并不能安全的在线程间共享
    //         // `Rc<Mutex<i32>>` cannot be sent between threads safely
    //         let counter = Rc::clone(&counter);
    //         // `Rc<Mutex<i32>>` cannot be sent between threads safely
    //         let handler = thread::spawn(move ||{
    //             // 上面一行报错
    //             // ^^^^^^^ value moved into closure here, in previous iteration of loop
    //             let mut num = counter.lock().unwrap();
    //             *num +=i;
    //         });
    //
    //         // 将handle join句柄加入到handlers
    //         handlers.push(handler);
    //     }
    //
    //     for handler in handlers{
    //         handler.join().unwrap();
    //     }
    //
    //     println!("result: {}",*counter.lock().unwrap());
    // }

    // Arc 多个线程中安全操作的原子引用计数器Arc<T>
    #[test]
    fn mutex_share_data() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];
        for i in 0..10 {
            // 创建多个线程
            let counter = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += i;
            });

            // 将handle join句柄加入到handlers
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        // result: 45
        println!("result: {}", *counter.lock().unwrap());
    }
}
