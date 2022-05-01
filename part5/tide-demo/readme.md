# tide
    https://github.com/http-rs/tide

# tide 更多用法
https://github.com/http-rs/tide/tree/main/examples

# tide log middleware
    log输出格式
    tide::log::middleware <-- Request received
        method GET
        path /query
    name:daheige
    legs:1
    tide::log::middleware --> Response sent
        method GET
        path /query
        status 200 - OK
        duration 76.594µs
