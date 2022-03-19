fn main() {
    println!("Hello, world!");
}

pub fn add(x:i32,y:i32) -> i32{
    x+y
}

fn add2(x:i32,y:i32) -> i32{
    if x < 0{
        panic!("x < 0")
    }

    x +y
}
// 单元测试写法
#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        println!("ok");
        assert_eq!(2+1,3); // 判断是否相等的宏
    }

    #[test]
    fn test_add(){
        // 调用上一层的方法
        assert_eq!(super::add(1,2),3);
        println!("test ok");
    }

    // 必须抛出panic
    #[test]
    #[should_panic]
    fn test_less_add(){
        // 一些操作必须panic才算通过
        super::add2(-1,2);
    }
}
