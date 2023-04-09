// 允许没有使用过的代码，不警告
#![allow(dead_code)] // this line prevents compiler warnings

// 定义枚举
enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct SeaCreature {
    species: Species, // 这里将枚举类型作为结构体的成员
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

// 带数据的枚举
// enum 的元素可以有一个或多个数据类型，从而使其表现得像c语言的联合
/*
enum 的内存细节：
    - 枚举数据的内存大小等于它最大元素的大小。此举是为了让所有可能的值都能存入相同的内存空间。
    - 除了元素数据类型（如果有）之外，每个元素还有一个数字值，用于表示它是哪个标签
其他细节：
    - Rust 的 enum 也被称为标签联合 （tagged-union）
    - 把类型组合成一种新的类型，这就是人们所说的 Rust 具有 代数类型 的含义。
 */
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}

// 下面定义的 Weapon 枚举的每一个成员都是不同的类型，带有数据类型
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };
    match ferris.species {
        // 匹配顺序先外面后里面
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_class, size) => {
                let size_desc = match size {
                    Size::Small => "small",
                    Size::Big => "big",
                };
                println!("ferris is a crab with{} {} class", num_class, size_desc);
            }
            _ => println!("ferris is a craw with some other weapon"),
        },
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }
}
