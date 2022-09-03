// 导出的宏定义，作为包给其他地方使用
#[macro_export]
macro_rules! map {
    // key=>value表达式，零个或多个
    ($($k:expr => $v:expr),*) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($k,$v); // 执行map insert操作
            )*
            // 返回m
            m
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn map_test() {
        let m = map! {
            "1" => 1,
            "2" => 2,
            "3" => 3
        };
        println!("m:{:?}", m); // m:{"1": 1, "3": 3, "2": 2}
        assert_eq!(m["1"], 1);
        assert_eq!(m["2"], 2);
    }
}
