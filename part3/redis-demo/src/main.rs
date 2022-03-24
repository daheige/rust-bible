use std::collections::HashMap;
use std::ptr::hash;
use redis::{self, Commands};

// redis cmd 基本操作
fn main() {
    println!("redis-demo...");
    let mut redis_obj = RedisService::new("redis://:@127.0.0.1:6379/0");

    // 大部分命令返回值是一个 redis::RedisResult<()>，具体可以点击看源码
    // 执行set_ex
    let _: redis::RedisResult<()> = redis_obj.conn.set_ex("user", "heige", 120);

    println!("get value");
    // let res : redis::RedisResult<String> = redis_obj.conn.get("user");
    // println!("res: {}",res.unwrap());

    // 返回值是 pub type RedisResult<T> = Result<T, RedisError>
    let user: String = redis_obj.conn.get("user").unwrap();
    println!("user: {}", user);

    // let _ : redis::RedisResult<i64> = redis_obj.conn.incr("my_number",1);
    let num: i64 = redis_obj.conn.incr("my_number", 1).unwrap();
    println!("num:{}", num);

    // hash set
    let _: redis::RedisResult<()> = redis_obj.conn.hset("my_hash", "a", "abc");
    let _: redis::RedisResult<()> = redis_obj.conn.hset("my_hash", "b", "efg");

    // hash get
    let a: String = redis_obj.conn.hget("my_hash", "a").unwrap_or("".to_string());
    println!("a: {}", a);

    // hash hgetall 哈希获得所有数据 map
    let hash: HashMap<String, String> = redis_obj.conn.hgetall("my_hash").unwrap();
    println!("hash: {:?}", hash);
    println!("hash.a = {}", hash.get("a").unwrap());

    let len: i64 = redis_obj.conn.hlen("my_hash").unwrap_or(0); // 如果不存在就初始化长度为0
    println!("len = {}", len);

    // set myname dahege ex 100 nx
    let _: redis::RedisResult<()> = redis::cmd("set").arg("my_name").arg("daheige").
        arg("ex").arg(100).arg("nx").query(&mut redis_obj.conn);

    // 一次设置多个值
    let _: redis::RedisResult<()> = redis_obj.conn.set_multiple(&[("num1", 1), ("num2", 2)]);

    // 一次设置多个hash key field value
    let s = vec![("user1", "heige"), ("user2", "daheige")];
    let _: redis::RedisResult<()> = redis_obj.conn.hset_multiple("my_hash", &s);

    // hincy my_hash_number art:1:read_count 1 哈希计数器
    // hget my_hash_number art:1:read_count
    let _: redis::RedisResult<()> = redis_obj.conn.hincr("my_hash_number", "art:1:read_count", 1);

    // 设置集合元素
    let _: redis::RedisResult<()> = redis_obj.conn.sadd("my_set", 1);
    let _: redis::RedisResult<()> = redis_obj.conn.sadd("my_set", 2);
}

struct RedisService {
    conn: redis::Connection,
}

impl RedisService {
    fn new(dsn: &str) -> Self {
        let conn = Self::redis_connection(dsn).unwrap(); // 如果连接失败就抛出异常
        Self { conn: conn }
    }

    /**
    % redis-cli
    127.0.0.1:6379> get my_number
    "12"
     */
    fn fetch_an_integer(&mut self) -> redis::RedisResult<isize> {
        let _: () = self.conn.set("my_number", 12)?;
        // read back the key and return it.  Because the return value
        // from the function is a result for integer this will automatically
        // convert into one.
        self.conn.get("my_number")
    }

    fn fetch_cmd_res(&mut self, name: String) -> redis::RedisResult<String> {
        // 通过命令的方式执行redis cmd
        // 先获取是否有值，没有就设置一个
        let res: redis::RedisResult<String> = redis::cmd("get").arg(&name).query(&mut self.conn);
        if let Ok(v) = res {
            return Ok(v);
        }

        let s = "heige";
        let _ = redis::cmd("set").arg(name).arg(s).query(&mut self.conn)?;
        Ok(s.to_string())
    }

    fn redis_connection(dsn: &str) -> redis::RedisResult<redis::Connection> {
        let client = redis::Client::open(dsn)?;
        let conn = client.get_connection()?;
        Ok(conn)
    }

    // 通过redis cmd模式调用相关的method
    fn do_some_cmd(&mut self) -> redis::RedisResult<()> {
        self.conn.set_ex("my_user", "heige", 120)?;
        self.conn.set_ex("user1", "daheige", 100)?;
        Ok(())
    }
}

// redis 测试用例
#[cfg(test)]
mod tests {
    #[test]
    fn redis_demo() {
        let mut redis_obj = super::RedisService::new("redis://:@127.0.0.1:6379/0");
        let my_value = redis_obj.fetch_an_integer().unwrap();
        println!("my_value:{}", my_value);

        let my_name = "my_name1";
        let name = redis_obj.fetch_cmd_res(my_name.to_string()).unwrap();
        println!("name:{}", name);

        // 另一种方式
        let name = redis_obj.fetch_cmd_res(my_name.to_string());
        if let Ok(name) = name {
            println!("other name:{}", name);
        } else {
            println!("not found key:{}", my_name);
        }

        let _ = redis_obj.do_some_cmd();
    }
}