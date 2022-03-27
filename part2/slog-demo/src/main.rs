use slog::{self, Drain, error, info, o};
use slog_json;
use slog_async;
use std::io;

fn main() {
    println!("slog demo...");
    let drain = slog_json::Json::new(io::stdout())
        // .set_pretty(true) // 是否需要美化处理
        .set_pretty(false)
        .add_default_keys()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain,o!("format"=>"pretty"));

    info!(logger,"an exam log message";"foo"=> "bar");
    info!(logger,"test";"fizz"=>"buzz");
    error!(logger,"test";"abc"=>"a");
}

/*
{
  "msg": "an exam log message",
  "level": "INFO",
  "ts": "2022-03-27T13:11:48.58628Z",
  "format": "pretty",
  "foo": "bar"
}
{
  "msg": "test",
  "level": "INFO",
  "ts": "2022-03-27T13:11:48.586452Z",
  "format": "pretty",
  "fizz": "buzz"
}
{
  "msg": "test",
  "level": "ERRO",
  "ts": "2022-03-27T13:11:48.586477Z",
  "format": "pretty",
  "abc": "a"
}
*/
