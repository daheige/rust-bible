# tcp-http-demo
    一个多线程并行和并发处理请求的http demo
    使用futures和async-std 运行时crate实现的简单http 并发服务器

# 运行效果
在浏览器中输入：http://127.0.0.1:8080/
![](tcp-demo.jpg)

# async-std spawn 并发函数的底层签名
下面是 async_std::task::spawn的定义：
```rust
pub fn spawn<F, T>(future: F) -> JoinHandle<T>
where
F: Future<Output = T> + Send + 'static,
T: Send + 'static,
{
    Builder::new().spawn(future).expect("cannot spawn task")
}
// 和标准库的spawn签名相似
#[stable(feature = "rust1", since = "1.0.0")]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```
看的出标准库spawn，它是一个看上去非常烦琐的类型签名。让我们对其中的内容进行逐一分析。
- spawn 是一个包含 F 和 T 的泛型函数，并且会接收一个参数 f，返回的泛型是
  JoinHandle<T>。随后的 where 子句指定了多个特征边界。
- F:FnOnce() -> T：这表示 F 实现了一个只能被调用一次的闭包。换句话说，f 是一
  个闭包，通过值获取所有内容并移动从环境中引用的项。
- F:Send + 'static：这表示闭包必须是发送型（Send），并且必须具有'static 的生命周
  期，同时执行环境中闭包内引用的任何类型必须是发送型，必须在程序的整个生命
  周期内存活。
- T:Send + 'static：来自闭包的返回类型 T 必须实现 Send+'static 特征。
  Send 是一种标记性特征。它只用于类型级标记，意味着可以安全地跨线程发送值；并
  且大多数类型都是发送型。未实现 Send 特征的类型是指针、引用等。此外，Send 是自动
  型特征或自动派生的特征。复合型数据类型，例如结构体，如果其中的所有字段都是 Send
  型，那么该结构体实现了 Send 特征。

对于async-std来说，仅仅是F变成了 F: Future<Output = T> + Send + 'static,
- 它是一个trait Future 类型，并且可以安全地发送到多个线程，这表明该类型是一种移动类型。
- 另外F具有'static 静态的生命周期，表明在整个程序中都是存活的，避免了future在执行poll的时候，生命周期发生改变。
- T是也是和F具有相同的参数约束，是可以安全返回的，并且也是static生命周期类型，从而保证了并发安全性和内存安全。

# mock TcpStream和unit test
```rust
// unit test 单元测试
// 为了保证单元测试的个理性和程序运行的确定性，我就使用MockTcpStream来模拟tcp请求
// 这里实现了impl Read + Write + Unpin 等特征
#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};
    use std::cmp::min;
    use std::pin::Pin;

    // 模拟tcp stream
    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    // 实现read trait 里面的poll_read方法
    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            let size: usize = min(self.read_data.len(), buf.len());
            // 将数据拷贝到buf缓存中
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size)) // 表明read请求已处理成功
        }
    }

    // 为 MockTcpStream 实现Write特征，需要实现三个方法 poll_write，poll_flush，poll_close
    impl Write for MockTcpStream {
        // 拷贝输入数据到mock的TcpStream中，完成后返回Poll::Ready
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            Poll::Ready(Ok(buf.len()))
        }

        // 由于TcpStream无需flush和close，所以下面的方法直接返回了Poll::Ready
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    // 实现 Unpin 移动特征，表示它可以在内存中安全的移动
    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}

    // 下面的 #[async_std::test] 作用和 #[async_std::main] 作用一样，将test函数转换为异步函数执行
    use std::fs;
    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        // 将数据写入contents中
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        // 初始化MockTcpStream用来创建tcp mock stream
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        // 处理请求
        handle_connection(&mut stream).await; // 等待请求执行完毕

        // 读取数据到buf中
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = fs::read_to_string("hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        // 判断请求的数据和期望的数据是否一致
        assert!(stream.write_data.starts_with(expected_response.as_bytes()))
    }
}
```
