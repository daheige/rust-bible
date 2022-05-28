use async_std::{fs::File, io, prelude::*, task};

// 异步读取文件内容
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    println!("Hello, world!");
    // 创建一个Task程序，它返回了 JoinHandle 类似std thread spawn JoinHandle
    // 只不过这个async_std task创建的task 由程序调度，而不是操作系统内核线程调度
    // 并且在需要等待的地方，等待结束后程序本身负责再次唤醒它。
    // 与此同时async_std task也可以有名称和ID，就像线程一样
    // reader_task 在spawn后，该任务在后台继续执行
    // JoinHandle 本身就是一个future，一旦Task 结束，它就会结束，跟std threads join相似
    // 这里Task 是async_std的核心抽象之一，和rust thread 一样，task提供来一些原始概念上的
    // 实用功能；task 与运行时有关系，同时它又是独立的。
    let reader_task = task::spawn(async {
        // 异步读取文件，等待执行完毕
        let res = read_file("test.md").await;
        match res {
            Ok(s) => println!("read content:{}", s),
            Err(err) => println!("error:{:?}", err),
        }
    });

    println!("start task...");
    // 将future 传递给block_on来阻塞程序，然后等待它完成
    task::block_on(reader_task);
    println!("stop task...");
}
