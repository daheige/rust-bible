fn main() {
    // 通过构建器方法cpp(true)指定 C++编译器
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .warnings(false) // 忽略警告
        .flag("-std=c++11") // 通过c11进行编译
        .compile("foo"); // 编译后名字libfoo.a
}
