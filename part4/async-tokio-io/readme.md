# tokio 异步io操作
    Tokio 中的 I/O 操作和 std 在使用方式上几无区别，最大的区别就是前者是异步的，
    例如 Tokio 的读写特征分别是 AsyncRead 和 AsyncWrite:
    有部分类型按照自己的所需实现了它们: 
        TcpStream，File，Stdout
    还有数据结构也实现了它们：
        Vec<u8>、&[u8]
    这样就可以直接使用这些数据结构作为读写器( reader / writer)

# AsyncRead 和 AsyncWrite
    这两个特征为字节流的异步读写提供了便利的使用方法，这些方法都使用 async 声明，
    且需要通过 .await 进行调用。

# 参考
https://zhuanlan.zhihu.com/p/462387088
