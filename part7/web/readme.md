# rust web使用
    基于axum库 web应用

# axum
https://github.com/tokio-rs/axum

# axum-example
https://github.com/tokio-rs/axum/tree/main/examples

# 访问
post请求
```shell
curl --location --request POST 'http://localhost:3000/users' \
--header 'Content-Type: application/json' \
--data-raw '{"username":"daheige"}'

"id":1,"name":"daheige"}
```

浏览器访问
http://127.0.0.1:3000/
