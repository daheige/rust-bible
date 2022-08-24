fn give_me<T>(value: T) {
    let _ = value;
    println!("generic_func define");
}

fn main() {
    println!("Hello, world!");
    println!("generic func call");
    give_me(1);
    give_me("rust generic demo");
}
