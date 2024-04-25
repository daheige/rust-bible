// build.rs
// 将foo.c编译为foo二进制文件
fn main() {
    cc::Build::new().file("foo.c").compile("foo");
}
