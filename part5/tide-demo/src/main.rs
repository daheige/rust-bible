use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::prelude::*;
use tide::{Body, Request, Response};

#[derive(Debug, Deserialize, Serialize)]
struct Animal {
    name: String,
    legs: i64,
}
use async_std::task;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let address = "127.0.0.1:8000";
    println!("server has on: {}", address);
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.at("/").get(home);
    app.at("/index").get(|_| async { Ok("hello rust") });
    app.at("/submit").post(submit);
    app.at("/animals").get(|_| async {
        Ok(json!({
            "code":0,
            "message":"ok",
            "data":{
                "id":1,
                "name":"cat",
            }
        }))
    });

    app.at("/foo").post(|mut req: Request<()>| async move {
        // 接收body 字符串 这里用move 关键字将req移动到闭包中获取req所有权
        let body: String = req.body_string().await.unwrap();
        println!("body:{}", body);
        Ok("success")
    });
    app.at("/foo2").post(foo);
    // app.at("/api/:name").get(|mut req: Request<()>| async move {
    //     let name = req.param("name").unwrap().to_string();
    //     println!("name:{}", name);
    //     let mut res = Response::new(200);
    //     res.set_body(Body::from_string(name));
    //     Ok(res)
    // });
    app.at("/api/:name").get(get_name);

    app.listen(address).await?;
    Ok(())
}

// 接收body json数据
async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    println!("name:{}", name);
    Ok(format!("hello,{} i have put in an order for {} shoes", name, legs).into())
}

async fn home(mut _req: Request<()>) -> tide::Result {
    Ok("hello tide".into())
}

/*
% curl --location --request POST 'localhost:8000/submit' \
--header 'Content-Type: application/json' \
--data-raw '{"name":"heige","legs":10}'
 */
async fn submit(mut req: Request<()>) -> tide::Result {
    let cat: Animal = req.body_json().await?;

    // 响应
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&cat)?);
    Ok(res)
}

async fn foo(mut req: Request<()>) -> tide::Result {
    // 接收body 字符串
    let body: String = req.body_string().await.unwrap();
    println!("body:{}", body);
    Ok("success".into())
}

// 从path获取参数
async fn get_name(mut req: Request<()>) -> tide::Result {
    let name = req.param("name").unwrap().to_string();
    println!("name:{}", name);
    let mut res = Response::new(200);
    res.set_body(Body::from_string(name));
    Ok(res)
}
