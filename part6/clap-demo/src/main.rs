use clap::Parser;

// 通过derive注解实现参数的解析
// This uses our Derive API which provides access
// to the Builder API as attributes on a struct
#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct AppService {
    /// port of app
    #[clap(short, long, value_parser)]
    port: i16,

    /// name of app
    // short表示单命名的方式，long是长命名方式,value_parser表示值解析
    // name of app表示注释说明
    #[clap(short, long, value_parser)]
    name: String,

    // count of app
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    let app = AppService::parse();
    println!("{:?}", app);
}
