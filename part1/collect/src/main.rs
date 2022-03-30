use std::collections::HashMap;

#[derive(Debug)] // 允许枚举可display debug
enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String)
}

fn main() {
    println!("Hello, world!");
    let mut v : Vec<i32> = Vec::new(); // 声明一个mut可变类型的i32 slice，存放的值都是i32
    v.push(1);
    v.push(2);

    println!("current v:{:?}",v);

    v.push(3);
    println!("current v:{:?}",v);

    let v2 = vec![1,2,3,4,5]; // 通过宏的方式创建一个vector 这个是一个不可的vector
    println!("v2 :{:?}",v2);
    let first:&i32 = &v[1]; // &和[]返回了一个引用
    println!("first:{}",first);

    let third : Option<&i32> = v.get(2); // 通过get方法，返回一个Option，可能有值，可能没有就是None
    println!("third:{:?}",third); // third:Some(3)
    match third {
        Some(num) => println!("num: {}",num),
        None => println!("not get value from v"),
    }

    let mut v = vec![1,2,3,4]; // 声明一个可变的i32 slice
    // let first = &v[0]; // 引用第一个元素,一个借用
    // v.push(1); // 这里又发生了可变引用，当first的引用指向了被释放的内存，^^^^^^^^^ mutable borrow occurs here
    // // 可变引用与不可变引用不能同时存在
    //
    // println!("first:{}",first);

    println!("first: {}",&v[0]); // 这里是把&v[0]借用，当使用这个println宏输出后不可变借用的内存就释放了
    v.push(111);

    // 遍历vector，这里用的是引用方式遍历，不获取v的所有权，不可变的方式遍历vector
    for i in &v{
        println!("i:{}",i);
    }

    // 通过可变的方式遍历
    for i in &mut v{ // 这里的i变量是一个&mut i32 可变引用
        *i += 1; // 为了修改可变引用指向的值，这里用*解引用的方式获得i
    }
    println!("v :{:?}",v); // v :[2, 3, 4, 5, 112]

    // 定义枚举类型的vector,存放不同的类型值
    // 对于rust来说，需要明确知道vector里面存放的类型，需要多少内存；防止类型造成错误
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text("abc".to_string()),
    ];

    println!("row:{:?}",row); // row:[Int(1), Float(1.2), Text("abc")]

    // string是一个vec集合
    // 在rust中只有一种字符串类型 str，是一个字符串slice,通常通过被借用的方式出现&str
    // 对于字符串字面量被存储在二进制中
    // String 是可增长的，可变化，有所有权的utf-8的字符串
    // 通常说字符串就是说,指的是String和字符串slice &str类型，不是单独的一个
    // 对于标准库中，还有其他类型的字符串，比如OsString,OsStr等
    // 对于String底层是一个struct ，元素是vec u8类型的切片
    /*
    pub struct String {
    vec: Vec<u8>,
}
     */
    let mut s = String::new();
    s.push_str("acb");
    s.push_str("hello");
    println!("s:{}",s);
    let s2 = String::from("hhhh");
    let s3 = s+&s2; // 当做了+操作后，s所有权就转给了s3了
    // println!("s: {}",s); //  ^ value borrowed here after move
    println!("s3:{}",s3);

    // 通过format!宏做字符串的拼接
    let s = String::from("abc");
    let s2 = String::from("def");
    let s = format!("{}-{}",s,s2);
    println!("new s:{}",s);

    // 索引字符串
    // let h = s[0]; //  ^^^^ `String` cannot be indexed by `{integer}`
    // 在rust中不支持string index索引访问
    let s2 = &s[0..4]; // 字符串本质上一个vec u8切片，通过range的方式访问
    println!("s2:{}",s2); // 这里s2是i个&str, s2:abc- 返回字符串的头4个资金，对于字母都是两个字节长
    // 上面的这个方式访问部分字符串，会存在问题，当存在一些特殊字符就会抛出panic

    // 遍历字符串
    for c in s.chars(){
        println!("{}",c);
    }

    // 通过字节的方式访问
    for b in s.bytes(){
        print!("b:{}",b); //  b:97b:98b:99b:45b:100b:101b:102
    }

    // rust中通过准确的方式处理string 类型，作为默认类型，并且是是utf-8合法的字符串，确实是复杂，但是是安全的

    // 对于hash map 使用，它是一个key/val键值对
    let mut m = HashMap::new();

    /*
     // insert是一个泛型函数,k,v是任意类型
     pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
     */
    m.insert("a",1);
    // m.insert(1,2); // 这里是不行的，因为上面一行已经确定了k的类型是&str
    m.insert("b",2);
    // 访问hash元素
    println!("access m");
    println!("m[a]= {}",m["a"]);

    // 通过get访问
    let v = m.get("a"); // 是一个Option<&i32> 可能有值的option
    println!("get m[0] {:?}",v);

    m.entry("c").or_insert(3); // 只有在键不存在就插入c
    m.insert("c",35);  // 以覆盖的方式更新原来的值

   for (key,value) in &m{
       println!("key:{} value:{}",key,value);
   }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_change(){
        let mut v = vec![100,120,110];
        for i in &mut v{
            *i += 10
        }

        println!("v:{:?}",v);
    }

    #[test]
    fn string_demo(){
        /*
        pub struct String {
            vec: Vec<u8>,
        }
         */
        let s = String::from("rust hello"); // 底层是一个String结构体，里面字段是vec:vec<u8>
        println!("s:{}",s);
    }
}
