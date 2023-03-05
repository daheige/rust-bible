use intro::{add, display_me, eat, lazy_adder, show_me, Apple, Bob, Eatable, Food, Programmer};
use intro::{Enemy, Game, Hero};

mod intro;

fn main() {
    println!("Hello, world!");
    let g = Game;

    // 调用 Game 上面的load方法
    g.load(Enemy);
    g.load(Hero);
    let x = add(1, 2);
    println!("x:{}", x);
    show_me(x);

    let apple = Food::new(Apple);
    apple.eat();
    eat(apple); // 接收一个特征trait参数

    let bob = Bob;
    bob.animate();

    let add_later = lazy_adder(1, 2); // 延迟计算
    let x = add_later();
    println!("result:{}", x);
    display_me(x);
}
