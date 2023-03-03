// 使用关键字 use 将特征导入需要调用特征的模块的作用域中
use crate::car::Vehicle;
use car::Car;
use media::Playable;

mod car;
mod media; // 这句是定义media模块

struct Tesl {
    model: String,
    release_date: u16,
}

impl Tesl {
    // 为Tesl实现关联方法，函数返回一个Tesl实例对象
    fn new(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}
impl Car for Tesl {
    fn model(&self) -> String {
        self.model.to_string()
    }
}

// Tesl 同时要实现Vehicle trait
impl Vehicle for Tesl {
    fn get_price(&self) -> u64 {
        200_000
    }
}

struct Audio(String);
struct Video(String);

// 分别为Audio,Video实现Playable
// 使用关键字 impl 后跟特征名称来声明特征实现，随后是关键字 for 和希望实现
// 的特征类型，其后的花括号用于编写特征实现
// 在花括号中，我们需要提供方法的实现，并根据需要覆盖特征中存在的任何默认实现
impl Playable for Audio {
    fn play(&self) {
        println!("now playing:{}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("now playing video:{}", self.0);
    }
}

fn main() {
    let a = Audio("audio test".to_string());
    let v = Video("mp3 test".to_string());
    a.play();
    v.play();

    // 直接调用关联方法
    Audio::pause();

    let tesl = Tesl::new("model 2", 2021);
    println!(
        "tesl model:{},date:{},price:{}",
        tesl.model(),
        tesl.release_date,
        tesl.get_price()
    );
}
