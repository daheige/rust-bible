// 常规用法
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

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

// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Adds files to myapp
//     Add {
//         #[clap(value_parser)]
//         name: Option<String>, // 可选
//     },
// }

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds files to myapp
    Add {
        #[clap(value_parser)]
        name: Option<String>, // 可选
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    println!("cli param: {:?}", cli);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
    }
}
