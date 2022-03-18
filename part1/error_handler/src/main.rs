use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3,4];
    println!("{:?}",v);

    // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 5'
    // 越界发生了panic
    // println!("v[5] = {}",v[5]);

    // Result与可恢复的错误
    // Reulst<T,E> 成功的话，会把值T放到Ok(t)里面，否则就是失败的时候，放入E
    let f = File::open("hello.txt");
    let f = match f{
        Ok(file) => file,
        Err(error) => {
            // 如果文件不存在
            // thread 'main' panicked at 'open file error:No such file or directory (os error 2)'
            panic!("open file error:{}",error);
        }
    };

    // 失败时候的panic简写，unwrap(),expect("xxx")
    // let f = File::open("hello.txt2").unwrap();
    // thread 'main' panicked at 'open file error:
    // Os { code: 2, kind: NotFound, message: "No such file or directory" }
    let f = File::open("hello.txt2").expect("open file error");
}

fn read_file() -> Result<String,io::Error>{
    // 错误传播
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 通过?简写来进行错误传播
// 当遇到错误的时候,?会直接返回错误，不会往下走
// 当执行成功，没错误，把值放到Ok(v)里面
fn read_file2() -> Result<String,io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 可以?链式调用处理，当遇到错误就会终止，立即返回错误error
// ?消除来大量样板代码，使得函数处理错误更加简单，它会自动给处理错误返回
// ? 只能被用于返回的Result的函数处理
fn read_file3() -> Result<String,io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 单元测试
#[cfg(test)]
mod tests{
    use crate::read_file2;

    // 指定函数可以测试
    #[test]
    fn read(){
        // super是调用父模块中的函数
        let res = super::read_file(); // 这里返回的是一个Result<T,E>
        // 通过match模式匹配
        match res {
            Ok(s) => {
                println!("s:{}",s);
            }
            Err(e) => {
                println!("read file error:{}",e);
            }
        };
    }

    #[test]
    fn read2(){
        // let res = read_file2();
        let res = super::read_file3();
        // if let Ok(v) 这种是仅仅关注成功的时候的值处理
        if let Ok(s) = res{
            println!("s :{}",s);
        }
    }
}

/**
关于panic！和 Result<T,E> 错误传播处理的选择
    对于一些启动或读取配置文件，这种可以panic!,程序必须终止的时候
    对于一些错误处理可交给上游业务方处理的话，就建议使用Result + ?错误传播的方式，让调用者自己来决定
    对于不会发生的错误，可以使用unwrap(),expect("xxx")来处理
panic!,unwrap(),expect这三个，如果在多个线程中处理，仅仅只会影响当前线程，不会发生跨线程的panic

Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。
panic! 宏代表一个程序无法处理的状态，并停止执行而不是 使用无效或不正确的值继续处理。
Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的情况下失败。
可以使 用 Result 来告诉代码调用者他需要处理潜在的成功或失败。
在适当的场景使用 panic! 和 Result 将会使你的代码在面 对无处不在的错误时显得更加可靠。
 */

