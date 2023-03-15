use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("test.md").await?;
    let mut buf = [0; 10]; // 定义数组buf缓冲区,每次读取10字节长度的内容

    // 1. 读取到字节数组中，最多读取10个字节
    let n = file.read(&mut buf[..]).await?;

    println!("read bytes:{:?}", &buf[..n]);

    // 2. 读取所有的字节
    let mut f = File::open("test.md").await?;
    // 这里定义一个vec
    let mut buf = Vec::new();
    // 将内容全部读取到buf，当遇到EOF就停止读取
    // AsyncReadExt::read_to_end 方法会从字节流中读取所有的字节，直到遇到 EOF
    f.read_to_end(&mut buf).await?;

    println!("read bytes2:{:?}", &buf);

    // 3. read to string 将内容读取到字符串中
    let mut f = File::open("test.md").await?;
    let mut s = String::new(); // 这里定义一个String
    let _ = f.read_to_string(&mut s).await;
    println!("read string:{}", &s);

    // 4. async fn write 异步写内容
    // AsyncWriteExt::write 异步方法会尝试将缓冲区的内容写入到写入器( writer )中
    // 同时返回写入的字节数
    let mut file = File::create("test2.md").await?;
    // b"xxx" 写法可以将一个 &str 字符串转变成一个字节数组
    let n = file.write(b"hello,rust").await?; // 这里使用b"xxx" 方式将字符串转换为bytes
    println!("write {} bytes", n);

    // AsyncWriteExt::write_all 将缓冲区的内容全部写入到写入文件中
    // 5. async fn write_all
    let mut file = File::create("test3.md").await?;
    // 将缓冲区的内容全部一次性写入文件中
    file.write_all(b"hello,rust").await?; // 这里使用b"xxx" 方式将字符串转换为bytes

    // 6. 和标准库一样 tokio::io 模块包含了多个实用的函数或API，可以用于处理标准输入/输出/错误等
    // 比如说，tokio::io::copy 异步的将读取器( reader )中的内容拷贝到写入器(writer)中
    // reader读取器存放的是一个bytes ，然后通过io::copy方式写入文件中
    // reader 字节数组 &[u8] 实现了 AsyncRead，所以这里可以直接将 &u8 用作读取器
    // 可以将reader中的内容写入file
    let mut reader: &[u8] = b"hello,rust";
    let mut file = File::create("test4.md").await?;
    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
