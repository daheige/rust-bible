use redis::{self, AsyncCommands};

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Hello, world!");
    let client = RedisService::new("redis://:@127.0.0.1:6379/0").unwrap();
    let mut conn = client.get_async_connection().await?;

    conn.set("my_name", 1).await?;
    conn.set("key1", b"foo").await?;

    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut conn)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut conn)
        .await;
    assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    Ok(())
}

struct RedisService {}
impl RedisService {
    fn new(dsn: &str) -> redis::RedisResult<redis::Client> {
        let client = redis::Client::open(dsn)?;
        Ok(client)
    }
}
