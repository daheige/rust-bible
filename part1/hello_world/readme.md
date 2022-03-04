# release发布构建

当项目最终准备好发布时，可以使用 cargo build --release 来优化编译项目。
这会在 target/release 而不是 target/debug 下生成可执行文件。这些优化可以让
Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。
这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；
另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。
如果你在测试代码的运行时间，请确保运行 cargo build --release 并使用
target/release 下的可执行文件进行测试。

# 开发阶段运行
    % cargo run
# build

    % cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s

# cargo new 
    cargo new xxx 新建项目
    cargo new --lib xxx 新建一个组件库
# cargo check
    cargo check 检查是否可以正常编译
