use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// 自定义time-future trait实现
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool, // 是否完成
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // 已经完成了
            Poll::Ready(())
        } else {
            // 将waker给了当前任务
            // TimerFuture 可在执行者的任务间移动，所以这里克隆了一份
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        println!("[{:?}] 开始创建 TimerFuture...", thread::current().id());
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 生成新的线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!(
                "[{:?}] TimerFuture 生成新的线程并睡眠:{}s...",
                thread::current().id(),
                duration.as_secs()
            );

            thread::sleep(duration);

            let mut shared_state = thread_shared_state.lock().unwrap();
            // 发出信号：计时器已经停止，并唤醒 Future 被 pool 的最后一个任务
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!(
                    "[{:?}] TimerFuture 新线程获得waker，并执行wake()...",
                    thread::current().id(),
                );
                waker.wake()
            } else {
                println!(
                    "[{:?}] TimerFuture 新线程没有获得waker...",
                    thread::current().id(),
                );
            }
        });

        println!("[{:?}] 返回新的 TimerFuture ...", thread::current().id(),);

        Self { shared_state }
    }
}
