use r2d2::Pool;
use redis::{self, Commands};
use std::time::Duration;

fn main() -> redis::RedisResult<()> {
    println!("Hello, world!");
    // redis pool连接池模式
    let pool = RedisService::new_pool("redis://:@127.0.0.1:6379/0");
    let mut conn = pool.get().unwrap(); // 默认超时是 connection_timeout 参数

    // 设置单个pool timeout
    // let mut conn = pool.get_timeout(Duration::from_secs(2)).unwrap();
    let res: redis::RedisResult<String> = conn.set("my_user", "daheige");
    if res.is_err() {
        println!("redis set error:{}", res.err().unwrap().to_string());
        return Err(redis::RedisError::from((
            redis::ErrorKind::ResponseError,
            "set redis data fail",
            // "server inner error".to_string(),
        )));
    }

    Ok(())
}

// redis connection pool模式
struct RedisService {}
impl RedisService {
    fn new_pool(dsn: &str) -> Pool<redis::Client> {
        let client = redis::Client::open(dsn).unwrap();
        let pool: Pool<redis::Client> = Pool::builder()
            .max_size(200)
            .max_lifetime(Some(Duration::from_secs(1800)))
            .idle_timeout(Some(Duration::from_secs(300)))
            .min_idle(Some(30))
            .connection_timeout(Duration::from_secs(3))
            .build(client)
            .unwrap();
        pool
    }
}
