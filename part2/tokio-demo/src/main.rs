use redis::{self, Commands};

fn main() {
    println!("Hello, world!");
    let my_value = fetch_an_integer().unwrap();
    println!("my_value:{}", my_value);

    let my_name = "my_name";
    // let name = fetch_cmd_res(my_name.to_string()).unwrap();
    // println!("name:{}", name);

    // 另一种方式
    let name = fetch_cmd_res(my_name.to_string());
    if let Ok(name) = name {
        println!("name:{}", name);
    } else {
        println!("not found key:{}", my_name);
    }
}

/**
% redis-cli
127.0.0.1:6379> get my_number
"12"
 */
fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://:@127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_number", 12)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_number")
}

fn fetch_cmd_res(name: String) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://:@127.0.0.1:6379/")?;
    let mut conn = client.get_connection()?;

    // 通过命令的方式执行redis cmd
    let _ = redis::cmd("set")
        .arg("my_name")
        .arg("heige")
        .query(&mut conn)?;
    redis::cmd("get").arg(name).query(&mut conn)
}
