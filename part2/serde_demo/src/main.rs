use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::{Result};

// 在结构体上面添加Serialize, Deserialize trait就可以自动做json序列化和反序列化
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i64,
    z: String,
}

// 序列化point
// 通过serde_json::Result返回结果
fn ser_point() -> Result<String> {
    let p = Point {
        x: 1,
        y: 2,
        z: "abc".to_string(),
    };

    // 下面的?表示如果出现错误就立即返回错误，这是?传播错误的简写
    let j = serde_json::to_string(&p)?; // 返回值是serde_json::Result<String>
    println!("{}", j);
    Ok(j)
}

fn main() {
    let p = Point {
        x: 1,
        y: 13,
        z: String::from("hello,rust"),
    };

    //==========序列化 json encode=================
    // 将类型转换为json字符串
    // convert to json string
    let b = serde_json::to_string(&p).unwrap(); // to_string()返回的是一个Result<T>
    println!("b = {}", b); // b = {"x":1,"y":13,"z":"hello,rust"}

    //==========反序列化 json decode=================
    // 将json 字符串转换为具体的类型
    let p: Point = serde_json::from_str(&b).unwrap(); // from_str返回也是一个Result<T>
    println!("p.x = {},p.y = {},p.z = {}", p.x, p.y, p.z);

    let p_str = ser_point().unwrap();
    println!("p_str = {}", p_str);
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use crate::Point;

    // 未指定序列化的类型
    #[test]
    fn untyped_example() {
        // 通过r#" "#来包裹原始json字符串
        let data = r#"
            {"x":1,"y":13,"z":"hello,rust"}
        "#;

        // parse string into serde_json::Value
        // serde_json::Value底层是一个枚举类型
        /*enum Value {
                            Null,
                            Bool(bool),
                            Number(Number),
                            String(String),
                            Array(Vec<Value>),
                            Object(Map<String, Value>),
                        }*/
        let v: Value = serde_json::from_str(data).unwrap();
        println!("x = {}", v["x"]); // 可以通过属性直接访问
        let y = v["y"].as_i64().unwrap(); // 这里通过属性访问后，对y做了强制转换为i64
        println!("y = {}", y);
    }

    #[test]
    fn typed_example() {
        // 通过r#" "#来包裹原始json字符串
        let data = r#"
            {"x":1,"y":13,"z":"hello,rust"}
        "#;

        // 解析到具体类型，在变量声明后面需要指定类型
        let p: Point = serde_json::from_str(&data).unwrap();
        println!("x = {}", p.x); // p_str = {"x":1,"y":2,"z":"abc"}
    }

    // 利用json!宏来构建serde_json::Value
    #[test]
    fn test_json_macro() {
        // 返回一个Value
        let j = json!({
            "name":"heige",
            "age":23,
        });
        println!("name:{}", j["name"]);

        // 将Value转换为字符串String
        // convert to a string of json
        let s = j.to_string();
        println!("s = {}", s);
    }
}

