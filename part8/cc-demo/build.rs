// build.rs
// 将foo.c编译为foo二进制文件
// 由于绑定的 C 非常简单，因此只需要将一个源文件，传递给cc::Build。
// 对于更复杂的构建需求，cc::Build为指定的include的路径和额外的编译器flag标志们，提供了一整套构建器方法。
fn main() {
    // 编译之后的名字为foo
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("foo.c")
        .compile("foo"); // // 输出 `libfoo.a`
}
