// build.rs
// 将foo.c编译为foo二进制文件
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
        .compile("foo");
}
