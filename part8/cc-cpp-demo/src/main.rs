// 声明这是一个外部函数
extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
    fn interop_sort(arr: *mut i32, n: u32);
}

pub fn sort_from_cpp(arr: &mut [i32]) {
    unsafe {
        // 将arr转换为i32切片指针对象，接着再转换为i32 vec指针，也就是cpp vector类型
        interop_sort(arr as *mut [i32] as *mut i32, arr.len() as u32);
    }
}

fn main() {
    println!("Hello, cc-cpp-demo");
    // 调用cpp提供的外部函数multiply
    unsafe {
        println!("multiply(5,7) = {}", multiply(5, 7));
    }

    println!("call cpp sort function...");
    // 定义一个i32的数组
    let mut my_arr: [i32; 10] = [1, 2, 3, 23, 12, 90, -12, 1, 13, 15];
    println!("before sorting...");
    println!("{:?}", my_arr);

    // 将切作为参数传递
    sort_from_cpp(&mut my_arr);
    println!("after sorting...");
    println!("{:?}", my_arr);
}
