// build.rs
// 将foo.c编译为foo二进制文件
// 由于绑定的 C 非常简单，因此只需要将一个源文件，传递给cc::Build。
// 对于更复杂的构建需求，cc::Build为指定的include的路径和额外的编译器flag标志们，提供了一整套构建器方法。
fn main() {
    // 编译之后的名字为foo
    // 通过define方法来定义c语言的define宏
    cc::Build::new()
        .define(
            "APP_NAME",
            format!("\"{}\"", env!("CARGO_PKG_NAME")).as_str(), // 获取rust当前项目的名字
        )
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(), // 获取rust当前项目的版本号
        )
        .define("WELCOME", "\"YES\"") //  定义WELCOME宏
        .file("src/foo.c")
        .compile("foo"); // // 编译输出的文件会有lib的前缀，也就是会在debug/build/cc-demo-xxx目录中生成libfoo.a文件
}
