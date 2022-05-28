// 将模块划分到不同的文件中，以package模式进行管理
// 定义模块名称，这个是以文件模式组织模块
mod front_of_house;

// 通过目录来组织模块,需要在front目录下新建mod.rs 然后进行模块声明定义
mod front;

// use crate::front::hosting as front_hosting;
// 通过相对路径引入
use front::hosting as front_hosting;

// 引入模块
// pub use crate::front_of_house::hosting;
// 通过相对路径引入
use front_of_house::hosting;

fn main() {
    println!("Hello, world!");
    hosting::say("daheige");
}
