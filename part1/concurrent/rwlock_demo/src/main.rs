use std::sync::RwLock;
use std::thread;

// rwlock读写锁使用
// 互斥锁适用于大多数应用场景，但对于某些多线程环境，读取的发生频率高于写入的。
// 在这种情况下，我们可以采用 RwLock 类型，它提供共享可变性，但可以在更细粒度上执行操作
// 读写锁同时支持多个读操作，但是在给定的作用域内只允许一个写操作
//
// rwlock上有两个公开的方法：
// read 读取操作，可以存在多个读操作
// write 提供对线程的独占访问，以便数据写入包装类型
fn main() {
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        {
            *m.write().unwrap() += 1;
        }

        let updated_value = *m.read().unwrap();
        updated_value
    });

    let updated = c.join().unwrap();
    println!("updated:{:?}", updated); // updated:6
}
