use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("reqwest demo");
    // get 请求
    let rsp = reqwest::get("http://localhost:8000/animals")
        .await?
        .text()
        .await?;
    println!("resp: {}", rsp);
    Ok(())
}

// Blocking Client 阻塞的方式请求
fn request_block_v1() -> Result<(), Box<dyn Error>> {
    let rsp = reqwest::blocking::get("http://localhost:8000/animals")?.text()?;
    println!("resp: {}", rsp);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Animal {
    id: i64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    code: i64,
    message: String,
    data: Animal,
}

#[cfg(test)]
mod tests {
    use super::Resp;
    use reqwest;

    #[test]
    fn request_block() {
        let rsp = reqwest::blocking::get("http://localhost:8000/animals")
            .unwrap()
            .text()
            .unwrap();
        println!("resp: {}", rsp);
    }

    #[test]
    fn req_block_v1() {
        let resp = super::request_block_v1();
        match resp {
            Ok(_) => println!("request success"),
            Err(err) => println!("err:{}", err),
        }
    }

    #[test]
    fn request_json_parse() {
        let res = reqwest::blocking::get("http://localhost:8000/animals")
            .unwrap()
            .json::<Resp>()
            .unwrap();
        println!("{:#?}", res);
    }
}
