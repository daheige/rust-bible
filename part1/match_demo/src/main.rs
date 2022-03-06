#[derive(Debug)] // 可以进行调试的trait声明
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match模式匹配枚举
    match coin {
        // 模式=>代码
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => {
            println!("dime enum match");
            3
        }
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 匹配option
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");
    let c = Coin::Dime;
    println!("c = {}", value_in_cents(c));

    let q = Coin::Quarter(UsState::Alaska);
    println!("q = {:?}", value_in_cents(q)); // state quarter from Alaska 25

    // 匹配Option<T>
    let five = Some(5);
    println!("plus_one(5) = {:?}", plus_one(five)); // plus_one(5) = Some(6)
                                                    // 通配模式和_忽略
    let dice_roll = 9;
    match dice_roll {
        1 | 2 | 3 => println!("123"), // 多个匹配
        7 => println!("777"),
        other => println!("other value"), // 其他情况
    }

    match dice_roll {
        1 | 2 | 3 => println!("123"), // 多个匹配
        7 => println!("777"),
        _ => println!("ignore value"), // 忽略没匹配到的情况
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Penny;
    // if let只匹配单个值的情况
    if let Coin::Quarter(state) = coin {
        println!("state:{:?}", state);
        count += 2;
    } else {
        count += 1;
    }

    println!("count = {}", count);
}
