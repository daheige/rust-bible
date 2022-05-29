use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("test.md").await?;
    let mut buf = [0; 10]; // 定义数组buf缓冲区,每次读取10字节长度的内容
                           // 读取到字节数组中
    let n = file.read(&mut buf[..]).await?;

    println!("read bytes:{:?}", &buf[..n]);

    // 读取所有的字节
    let mut f = File::open("test.md").await?;
    let mut buf = Vec::new(); // 这里定义一个vec
    f.read_to_end(&mut buf).await?;

    println!("read bytes2:{:?}", &buf);

    // read to string
    let mut f = File::open("test.md").await?;
    let mut s = String::new(); // 这里定义一个String
    let _ = f.read_to_string(&mut s).await;
    println!("read string:{}", &s);

    // async fn write
    let mut file = File::create("test2.md").await?;
    let n = file.write(b"hello,rust").await?; // 这里使用b"xxx" 方式将字符串转换为bytes
    println!("write {} bytes", n);

    // async fn write_all
    let mut file = File::create("test3.md").await?;
    // 将缓冲区的内容全部一次性写入文件中
    file.write_all(b"hello,rust").await?; // 这里使用b"xxx" 方式将字符串转换为bytes

    // tokio::io 模块包含了多个实用的函数或API，可以用于处理标准输入/输出/错误等
    // reader读取器存放的是一个bytes ，然后通过io::copy方式写入文件中
    // reader 字节数组 &[u8] 实现了 AsyncRead，所以可以将reader中的内容写入file
    let mut reader: &[u8] = b"hello,rust";
    let mut file = File::create("test4.md").await?;
    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
