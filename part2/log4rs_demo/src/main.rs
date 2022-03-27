use log::{debug, error, info, warn};
use log4rs;

fn main() {
    println!("Hello, world!");
    log4rs::init_file("log.yaml",Default::default()).unwrap();
    info!("hello {}","daheige");
    warn!("runtime error:{}","token invalid");
    debug!("debug mysql");
}
