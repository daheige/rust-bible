use std::fmt::{Debug, Display};
use std::ops::Add;

pub struct Game;
pub struct Enemy;
pub struct Hero;

// 给Game添加方法load
impl Game {
    // 实现load方法,它是一个泛型方法
    // entity 上面必须实现init方法才可以调用
    // 这里通过特征trait来约束参数类型T，必须是一个trait
    //
    // “:Loadable”部分表明了我们指定特征范围的方式。特征区间允许我们限制
    // 泛型 API 可以接收的参数范围。指定泛型元素上的绑定的特征类似于我们为变量指定类型
    // 的方式，但是此处的变量是泛型 T，类型是某些特征
    pub fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

// 定义trait
pub trait Loadable {
    fn init(&self);
}

// Enemy 实现 Loadable trait
impl Loadable for Enemy {
    fn init(&self) {
        println!("enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("hero loaded");
    }
}

// 泛型函数的参数类型上面绑定某种trait
// 由于函数返回了T,但是这个属于输出类型，所以继续在T上绑定了输出Add<Output = T>
pub fn add<T: Add + Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 区间内泛型
// 这是在泛型元素上指定特征区间的最常用语法。
// 下面代码中的 show_me 函数是指定特
// 征区间的一种方法，它会接收任何实现了 Display 特征的类型。
// 这是在泛型函数的类型签名的长度较短时声明特征区间的常见语法。
pub fn show_me<T: Display>(val: T) {
    println!("val:{}", val);
}

// 泛型函数和impl代码块上的特征区间
// 定义eat trait特征
pub trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
pub struct Food<T>(T); // 这是一个泛型结构体，是一个元祖类型的结构体

impl<T> Food<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}

#[derive(Debug)]
pub struct Apple;

// impl实现特征trait，如果某个类型是泛型，必须在impl后面通过<T>的方式声明T再使用
// T通过where来实现参数绑定
impl<T> Eatable for Food<T>
where
    T: Debug,
{
    fn eat(&self) {
        println!("eating:{:?}", self);
    }
}

// 泛型函数的参数是一个trait类型
// 下面的泛型函数的参数类型T通过where语句来实现trait绑定
pub fn eat<T>(val: T)
where
    T: Eatable,
{
    val.eat();
}

// == 使用“+”将特征组合为区间==
pub trait Eat {
    fn eat(&self) {
        println!("eat...");
    }
}

pub trait Code {
    fn code(&self) {
        println!("coding....");
    }
}

pub trait Sleep {
    fn sleep(&self) {
        println!("sleep....");
    }
}

// 通过+将特征组合为区间
pub trait Programmer: Eat + Code + Sleep {
    // 这里trait可默认实现方法
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
    }
}

pub struct Bob;
// 分别为Bob实现Code,Eat,Sleep trait
impl Code for Bob {}
impl Eat for Bob {}
impl Sleep for Bob {}

// 组合的Programmer实现
// 如果类型 T 实现了 Programmer，
// 那么它必须实现上述所有特征
impl Programmer for Bob {}

// ===特征区间与 impl 特征语法===
// impl 特征语法
// 直接使用了 impl Display，而不是指定 T：Display。这是 impl 特征语法
// 这为我们返回复杂或不方便表示的类型（例如函数的闭包）提供了便利。如果没有这种语法，则
// 必须使用 Box 智能指针类型将其放在指针后面返回，这涉及堆分配。闭包的底层结构由实
// 现了一系列特征的结构体组成。Fn(T) -> U 特征就是其中之一
pub fn display_me(val: impl Display) {
    println!("val:{} through impl Display", val);
}

// 这里返回值是一个impl trait类型
pub fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}
