use into_map::IntoMapTrait;
use into_map_derive::IntoMap;

// 指定IntoMap trait 来自into_map_derive包的派生宏
#[derive(IntoMap)]
struct User {
    name: String,
    id: usize,
    active: bool,
}

#[derive(IntoMap)]
struct Person {
    id: i64,
    name: String,
    sex: i64,
}

fn main() {
    let user = User {
        name: "daheige".to_string(),
        id: 12,
        active: true,
    };
    let map = user.into_map();
    println!("m:{:?}", map);

    let p = Person {
        id: 123,
        name: "xiaoming".to_string(),
        sex: 1,
    };
    let m = p.into_map();
    println!("m:{:?}", m);
}
