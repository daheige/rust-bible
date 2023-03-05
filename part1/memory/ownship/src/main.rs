#[derive(Debug)]
struct Foo(u32);

#[derive(Copy, Clone, Debug)]
struct Dummy;

// 在 derive 属性中添加了一个 Clone 特征。
// 有了它，我们就可以在 a 上调用 clone方法来获得它的新副本
#[derive(Copy, Clone, Debug)]
struct DummyInfo {
    items: u32,
}

fn main() {
    let foo = Foo(12); // foo 是 Foo实例的所有者
    let bar = foo; // Rust 会默认移动变量指向的值

    // 当我们把foo赋值给bar后，foo所有权move到了bar上面去了，因此下面的代码运行失败
    // println!("foo is :{:?}", foo); // ^^^ value borrowed here after move
    println!("bar is:{:?}", bar);

    // 作用域
    let s = String::from("daheige"); // s的作用域离开是main函数执行结束的地方
                                     // 块作用域
    {
        let num = 9;
        {
            let mut v = vec![1, 2, 3];
            v.push(123);
        } // v在这里离开作用域

        // v.push(3); // 这里不再有效 // ^ help: a local variable with a similar name exists: `s`
    }
    println!("s = {}", s);

    // 复制语义
    let a = Dummy;
    let b = a;
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // 显式调用clone
    let i = DummyInfo { items: 123 };
    let m = i.clone();
    println!("current dummy info:{:?}", i);
    println!("current m dummy info:{:?}", m);
}
