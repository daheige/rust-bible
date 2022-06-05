# async fn
    异步函数定义 
    async fn do_task(){ // ....}
    async fn 返回的是一个Future状态机，需要一个执行者来运行

# futures crate
async fn 运行时的future crate包
futures::executor::block_on
  + block_on阻塞当前线程，直到提供的Future运行完成
  + 其他执行者提供更加复杂的行为，例如将多个future安排到同一个线程上执行

# await
   + 在async fn函数中，可以用.await来等待另一个实现Future trait的完成
   + 与block_on不同，.await不会阻塞当前线程，而是将异步的等待Future的完成
    （如果该Future目前无法完成，将允许其他任务执行）

# demo
    参考async-demo
