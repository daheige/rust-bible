fn main() {
    // 通过构建器方法cpp(true)指定 C++编译器
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo"); // 编译后名字libfoo.a
}
