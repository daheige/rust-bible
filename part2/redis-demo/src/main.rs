use redis::{self, Commands};

fn main() {
    println!("redis-demo...");
    let mut redis_obj = RedisService::new("redis://:@127.0.0.1:6379/0");
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
