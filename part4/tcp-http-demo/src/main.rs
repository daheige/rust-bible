use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::fs;
use std::marker::Unpin;

#[async_std::main]
async fn main() {
    let address = "127.0.0.1:8080";
    println!("async-std tcp http demo run on:{}", address);
    let listener = TcpListener::bind(address).await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}

// 访问地址：http://localhost:8080/
async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    // 响应内容response
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

// unit test
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
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}

    use std::fs;
    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        // 创建tcp mock stream
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        // 处理请求
        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = fs::read_to_string("hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()))
    }
}
