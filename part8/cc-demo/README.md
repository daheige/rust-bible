# 通过cc运行在rust中调用c语言提供的函数

ffi调用参考： https://doc.rust-lang.org/nomicon/ffi.html

构建依赖如下：

```toml
[build-dependencies]
cc = "1.0.95"
```

执行cargo check 和 cargo run运行效果如下图所示：
![](cc-demo.jpg)
