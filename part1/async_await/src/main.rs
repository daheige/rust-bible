use futures;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    let f = hello(); // 返回值是一个Future
    // block_on会阻塞直到Future执行完成
    block_on(f); // 阻塞执行

    println!("=======block on exec=====");
    // 在执行每一个任务时候阻塞
    let song = block_on(learn_song());
    block_on((sing_song(song)));
    block_on(dance());

    // 异步执行
    println!("=========async/await exec=======");
    block_on(async_action_exec());

    thread::sleep(Duration::from_secs(1));
}

// 异步函数返回的是一个future 等待某个时候被执行
async fn hello() {
    println!("hello async demo");
}

struct Song;

async fn learn_song() -> Song {
    //dom something
    println!("learn song");
    Song
}

async fn sing_song(song: Song) {
    //sing the song
    println!("sing song");
}

async fn dance() {
    // dance
    println!("dance");
}

// .await是在代码块中按顺序执行，会阻塞后面的代码，但是此时会让出线程；
// block_on会阻塞直到Future执行完成。
async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

// Rust异步编程，通过join执行Future
async fn async_action_exec() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    // 允许其他任务并发执行
    // `join!` 类似于 `.await` ，但是可以等待多个 future 并发完成
    futures::join!(f1,f2);
}

async fn print_async() {
    println!("hello from print_async");
}

// 测试异步执行的函数
#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    #[test]
    fn async_test() {
        let f = super::print_async();
        println!("hello test main");
        block_on(f);
    }

    #[test]
    fn async_song() {
        block_on(super::async_action_exec());
    }
}