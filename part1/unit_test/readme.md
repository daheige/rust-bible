# 单元测试
    cargo test

# 指定线程数量测试
    cargo test -- --test-threads=1
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
    Running unittests (target/debug/deps/unit_test-18dbf167890f523f)
    
    running 3 tests
    test tests::it_works ... ok
    test tests::less_add - should panic ... ok
    test tests::test_add ... ok
    
    test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

# 对于测试的打印输出
    如果你希望也能看到通过的测试中打印的值
    捕获输出的行为可以通过 --nocapture 参数来禁用
    cargo test -- --nocapture

    % cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/unit_test-18dbf167890f523f)

    running 3 tests
    thread 'ok
    tests::less_add' panicked at 'x < 0', src/main.rs:test ok
    11:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    test tests::it_works ... ok
    test tests::test_add ... ok
    test tests::less_add - should panic ... ok
    
    test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

# 通过名称来运行测试的子集
    给cargo test 后面紧接着要测试的函数名字就可以
    cargo test it_works  -- --nocapture
    running 1 test
    ok
    test tests::it_works ... ok
    
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

# test测试过滤
    % cargo test test  -- --nocapture
    这里可以测试以test_开头的函数

# tests集成测试
    在src 同一层新建tests目录,测试用例写法：
```rust
#[test]
fn test_print(){
    println!("{}",12);
}
```