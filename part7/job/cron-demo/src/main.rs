use rcron::{Job, JobScheduler};
use std::thread;
use std::time::Duration;

struct Param {
    x: i32,
    y: i32,
}

fn main() {
    let mut sched = JobScheduler::new();
    let x = 1;
    let p = Param { x: 1, y: 2 };
    sched.add(Job::new("1/3 * * * * *".parse().unwrap(), || {
        println!("x:{}", x);
        println!("p.x:{}", p.x);
        println!("p.y:{}", p.y);
        say_hello();
    }));

    // The `tick` method increments time for the JobScheduler and executes
    // any pending jobs. It is recommended to sleep for at least 500
    // milliseconds between invocations of this method.
    loop {
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }
}

fn say_hello() {
    println!("hello,rust job");
}
