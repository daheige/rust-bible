use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::io::Result;

// actix_web基本使用
#[actix_web::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1";
    let port = 8090;

    println!("server has run {}:{}", address, port);
    let server = HttpServer::new(|| {
        App::new()
            .service(greet)
            .route("/get_index", web::get().to(get_index))
            .route("/post_sum", web::post().to(post_sum))
    });
    server.bind((address, port))?.run().await
}

#[derive(Deserialize, Serialize)]
struct SumRequest {
    a: i64,
    b: i64,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello,{name}")
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

// 这里form是指定的web::Form类型
async fn post_sum(form: web::Form<SumRequest>) -> HttpResponse {
    if form.a == 0 || form.b == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("bad request,request param invalid");
    }

    let res = format!("the sum is {}", add(form.a, form.b));
    HttpResponse::Ok().content_type("text/html").body(res)
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <html>
                <title>sum</title>
                <form action="/post_sum" method="post">
                    a: <input type="text" name="a"/>
                    b: <input type="text" name="b"/>
                    <button type="submit">submit</button>
                </form>
            </html>
            "#,
    )
}
