use crate::Food::Cake;

// 定义枚举类型，这里的Debug表示该类型可以调试输出
#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad
}

#[derive(Debug)]
struct Bag {
    food: Food
}

fn main() {
   let mut n = 3;
    take_the_n(&mut n); // 注意这里，参数需要使用&mut 修饰，表示传递的是一个可变引用的参数
    println!("n = {}",n);
    let s = String::from("rust");
    // 将不可变引用（也就是借用）
    say_hello(&s);
    println!("s = {}",s); // s还可以继续使用
    take_ownership(s); // 将s的所有权传递给函数之后，就不能再继续使用s
    // println!("s = {}",s); // 这一句会直接报错^ value borrowed here after move

    let s = String::from("hello");
    // 多个不可变引用可以同时存在
    let s1 = &s;
    let s2 = &s;
    println!("s1:{},s2:{}",s1,s2);

    // 可变引用和不可变引用在同一时刻不能同时存在
    // let s3 = &s;
    // let s4 = &mut s;

    // 在模式匹配中使用借用
    let bag = Bag{
        food: Food::Cake
    };

    match bag.food{
        Food::Cake => println!("I got cake"),
        // 这里的ref关键字，表示可以通过引用来匹配元素，而不是根据值来匹配它们
        ref a => println!("I got {:?} by ref keywords",a)
    }

    println!("{:?}",bag);

}

// 函数参数可变借用（可变引用）
fn take_the_n(n: &mut u8){
    // 通过*n解引用获得起值，然后改变n的值
    *n +=2;
}

fn say_hello(s:&str){
    println!("hello,{}",s);
}

fn take_ownership(s : String){
    println!("call take_ownership");
    println!("s = {}",s);
}