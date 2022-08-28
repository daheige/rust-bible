use std::sync::mpsc::channel;
use std::thread;

// 通过消息传递channel进行通信，避免了需要用户显式锁定的要求
// 在rust标准库中提供了std::sync::mpsc 多个生产者/单个消费者模型的消息通信的队列
// mpsc提供了2种方式的消息通信：channel 异步有缓冲通道 和 sync_channel 同步的有界缓冲通道
fn main() {
    // 异步通道
    // 首先为了创建 mpsc 队列，会调用 channel 函数，它会返回 Sender<T>
    // 和 Receiver<T>
    // Sender<T>是一种复制类型，这意味着它可以切换到多个线程中，允许它们将消息发送
    // 到共享队列。
    //
    // 使用默认的异步通道时，send 方法永远不会阻塞。这是因为通道缓冲区是无限的，所
    // 以总是会提供更多的空间。当然，它实际上并不是无限的，只是在概念上如此：如果你在
    // 没有收到任何数据的情况下向通道发送数千兆字节，那么系统可能会耗尽内存
    let (tx, rx) = channel(); // 其中tx表示发送者，rx表示接收者
    let handler = thread::spawn(move || {
        // 在循环中不断接收值，直到tx失效
        // while let Ok(n) = rx.recv() {
        //     println!("received data:{}", n);
        // }
        for msg in rx {
            println!("recv msg: {}", msg);
        }
    });

    // 执行发送操作
    for i in 0..10 {
        tx.send(i).unwrap();
    }
    drop(tx); // 关闭发送通道

    handler.join().unwrap(); // 等待子线程执行完毕
}

// 多个生产者单元测试
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    #[test]
    fn message_mp() {
        // 声明生产者和消费者tx,rx
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
}
