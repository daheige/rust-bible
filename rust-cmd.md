# rust 常用命令

    查看版本
    rustc --version
    
    升级
    rustup update
    
    查看cargo版本
    cargo --version
    
    创建项目
    cargo new hello_cargo
    
    构建项目
    cargo build
    cargo build --release // 打包生产环境
    
    运行项目
    cargo run

    检测是否能正常编译
    cargo check

# cargo build配置

    Cargo 有两个主要的配置：运行 cargo build 时采用的 dev 配置
    和运行 cargo build --release 的 release 配置。
    dev 配置被定义为开发时的好的默认配置，release 配置则有着良好的
    发布构建的默认配置.

    当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，
    Cargo 会对每一个配置都采用默认设置。
    通过增加任何希望定制的配置对应的 [profile.*] 部分，
    我们可以选择覆盖任意默认设置的子集。
    例如，如下是 dev 和 release 配置的 opt-level 设置的默认值：
    文件名: Cargo.toml
```toml
 [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3
```
    opt-level 设置控制 Rust 会对代码进行何种程度的优化。这个配置的值从 0 到 3。
    越高的优化级别需要更多的时间编译，所以如果你在进行开发并经常编译，
    可能会希望在牺牲一些代码性能的情况下编译得快一些。
    这就是为什么 dev 的 opt-level 默认为 0。当你准备发布时，花费更多时间在编译上则更好。
    只需要在发布模式编译一次，而编译出来的程序则会运行很多次，所以发布模式

# rust交叉编译

    绝大部分的Rust程序员都会有跟我我一样的需求，写代码用的是Windows或者Mac，部署平台是Linux，
    这种情况下就需要使用Cross-Compiler交叉编译，意思是可以在当前平台Host下编译出目标平台target的可执行文件，
    尤其是做ARM平台开发的同学对这个更为熟悉。
    
    Rust交叉编译在Github上有一个文档Rust核心员工Jorge Aparicio提供的一份文档https://github.com/japaric/rust-cross，
    推荐大家仔细的读一读。
    
    对我而言，我的要求比较简单，都是X86_64架构，从Mac上编译出unknow-linux就好

    musl工具链

    musl实现了Linux libc，质量可靠，适配所有Linux环境，使用静态连接替换动态链接，这样就能打出一个完整的二进制文件，
    可以丢到任何Linux环境里运行。
    
    当然，关于静态链接与动态链接各有优缺点，这里不细说。

    安装一下
    
    rustup target add x86_64-unknown-linux-musl
    vim  main.rs
``` rust
    fn main() {
        println!("Hello, world!");
    }
```

    cargo build --release --target=x86_64-unknown-linux-musl
    编译好的结果会放入 target/x86_64-unknown-linux-musl/release中

    把结果丢到Linux下执行，没问题
    
    $ ./hello
    Hello, world!

    常见问题

    要是提示/bin/sh: musl-gcc: command not found，解决方法是安装musl-cross
    
    brew install filosottile/musl-cross/musl-cross

    配置config
    
    $ vim .cargo/config
``` toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

    要是你的程序依赖原生库，需要设置一个环境变量CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc，所以完整的编译命令如下

    CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl

    要是你的程序使用了OpenSSL类库，这是一个麻烦的事情，目前普遍做法是在Cargo.toml文件中添加依赖
    Cargo.toml
``` toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

# Docker编译

    我更愿意推荐使用这种方式，因为使用第一种方式的话，需要安装一些工具链，还得看程序实际依赖的各种类库而做各种调整。
    其次是部署的时候也可以选择使用Docker部署，这样一来就几乎没有交叉编译的需求了。
    
    Rust官方提供了Docker镜像在hub.docker.com上，根据官方的文档，可以使用以下命令编译出Linux平台的可执行文件
``` shell
    docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp \
    rust:1.54 cargo build --release
```

    Docker环境下编译也有问题，每次都得重新拉取资源，其次是容易卡，其它都还好。
