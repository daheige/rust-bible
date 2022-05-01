use http_types::Mime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use tide::prelude::*;
use tide::{log, Redirect};
use tide::{Body, Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
struct Animal {
    name: String,
    legs: i64,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

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

    app.at("/query").get(get_req);
    app.at("/query2").get(get_req2);

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
    // 静态目录设置 http://localhost:8000/static/test.html
    app.at("/static").serve_dir("static/")?;
    app.at("/page").get(redirect);

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
async fn get_name(req: Request<()>) -> tide::Result {
    let name = req.param("name").unwrap().to_string();
    println!("name:{}", name);
    let mut res = Response::new(200);
    res.set_body(Body::from_string(name));
    Ok(res)
}

// 从query中获取参数
// localhost:8000/query?name=daheige&legs=1
async fn get_req(req: Request<()>) -> tide::Result {
    // let q = req.query();
    // if q.is_ok() {
    //     let query: Animal = q.unwrap();
    //     println!("name:{}", query.name);
    //     println!("legs:{}", query.legs);
    //
    //     let mut res = Response::new(200); // 创建一个response
    //     res.set_body(Body::from_json(&query)?);
    //     Ok(res)
    // } else {
    //     Ok("param invalid".into())
    // }
    let query: Animal = req.query().unwrap();
    println!("name:{}", query.name);
    println!("legs:{}", query.legs);

    // let mut res = Response::new(200); // 创建一个response
    let mut res = Response::new(StatusCode::Ok); // 创建一个response
    res.set_body(Body::from_json(&query)?);
    Ok(res)
}

// 返回内容type设置
// localhost:8000/query2?name=daheige&legs=1
async fn get_req2(req: Request<()>) -> tide::Result {
    // 当用户参数没有传递正确，它会抛出panic，仅仅是当前的线程中
    // 只在当前请求中执行失败，不会影响别的请求
    // thread 'async-std/runtime' panicked at 'called `Result::unwrap()` 
    // on an `Err` value: failed with reason: missing field `legs`',
    //  src/main.rs:132:37
    // let query: Animal = req.query().expect("request param invalid");
    let query: Animal = req.query().unwrap();
    println!("name:{}", query.name);
    println!("legs:{}", query.legs);

    log::info!("legs:{}", query.legs);

    let mime = Mime::from_str("application/json; charset=utf-8").unwrap();
    // let mut res = Response::new(200); // 创建一个response
    let mut res = Response::new(StatusCode::Ok); // 创建一个response
    res.set_content_type(mime);
    res.set_body(Body::from_json(&query)?);
    Ok(res)
}

async fn redirect(_req: Request<()>) -> tide::Result {
    Ok(Redirect::new("/animals").into())
}
