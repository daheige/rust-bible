// 使用async-std 第三方async运行时库来实现异步功能
// 它跟标准库的api相似，相对来说比较容易
use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::fs;
use std::marker::Unpin;

// 通过async_std::main 包裹main函数使得它可以修饰为async的方式运行
// 它可以将main函数修改为异步的
#[async_std::main]
async fn main() {
    // 监听本地端口，等待 tcp 连接的建立
    let address = "127.0.0.1:8080";
    println!("async-std tcp http demo run on:{}", address);
    // 异步版本的TcpListener 为 listener.incoming() 实现了 Stream 特征
    // 1. 这里它就不会阻塞
    // 2. 使用 for_each_concurrent 并发地处理从 Stream 获取的元素
    let listener = TcpListener::bind(address).await.unwrap();

    // 非阻塞等待请求进入
    // for each循环处理每个请求
    // 这里listener.incoming() 它是一个阻塞的迭代器，当listener在等待连接时，执行器是无法执行其他的future的
    // 只有当处理完已有的连接后才可以，所以在接收到请求后这里采用了async-std spawn来处理每个请求
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            println!("handler request...");

            // 通过async-std spawn将异步函数放入其中执行
            spawn(handle_connection(stream));
        })
        .await;
}

// 访问地址：http://localhost:8080/
// 异步的方式处理每个请求
//
// 由于 handle_connection 实现了 Send 特征，并且不会阻塞，因此它使用async-std spawn 多线程并行的处理请求是可行的
// 这里要提到的一点：
// async-std包允许我们使用多个线程的方式处理请求，这样我们就同时使用并行（多线程）和并发 async 两个
// 同时来处理多个请求
async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buf = [0; 1024];
    // 等待stream读取完毕，它是异步执行的
    stream.read(&mut buf).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // 读取文件内容
    let contents = fs::read_to_string(filename).unwrap();

    // 将回复内容写入连接缓冲中
    let response = format!("{}{}", status_line, contents);
    // 响应内容response
    stream.write(response.as_bytes()).await.unwrap();
    // 使用flush将缓冲中的内存发送到客户端
    stream.flush().await.unwrap();
}

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
