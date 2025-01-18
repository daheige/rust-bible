# rust学习笔记

    基于rust edition = "2021" 
    需要安装rust v1.58.0 以上版本

# rust从入门到实战

- part1 rust基础,thread concurrent编程初体验
- part2 serde json序列化处理，yaml文件读取，log日志库基础操作
- part3 redis,mysql,kafka和pulsar mq基本使用
- part4 rust异步编程基础--tokio和async-std两种不同的异步操作库使用
- part5 axum,actix-web,tide框架使用
- part6 rust 依赖注入和clap命令终端使用
- part7 rust实战(rust web/job/grpc）综合使用
- part8 rust ffi调用

# rust安装
参考文档：
- https://www.rust-lang.org/zh-CN/tools/install
- https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

安装步骤：
```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

如果上述安装过程中比较慢，还可以使用如下方式安装rust。也就是说，第一次安装 rustup 的时候，如果按照官网教程 https://sh.rustup.rs 链接可能无法下载，可以先执行如下命令下载sh文件：
```shell
wget https://mirrors.ustc.edu.cn/misc/rustup-install.sh
```

然后把脚本中的 `RUSTUP_UPDATE_ROOT` 变量改为 `https://mirrors.ustc.edu.cn/rust-static/rustup`
接着再执行sh rustup-install.sh即可快速安装rust
```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
sh rustup-install.sh
```

# rustup升级rust
```shell
rustup update
```
rust版本查看
```shell
rustc --version
cargo --version
```

# rust 1.70.x以上版本的crates发布
需要添加--registry参数
```shell
cargo publish --registry crates-io
```

# 设置rust国内镜像
参考文档
- https://mirrors.ustc.edu.cn/help/rust-static.html
- https://mirrors.ustc.edu.cn/help/crates.io-index.html

国内提高访问速度，建议设置环境变量: vim ~/.bash_profile或者vim ~/.bashrc
```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
export PATH="$HOME/.cargo/bin:$PATH"
```
执行source ~/.bash_profile生效

在用户目录.cargo目录目录中创建config.toml
```shell
cd ~/.cargo/
touch config.toml
```
添加如下内容：
```toml
# 源码地址
[source.crates-io]
#registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像，可以根据实际情况选择不同的镜像
replace-with = 'ustc'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"

[source.aliyun]
registry = "https://code.aliyun.com/rustcc/crates.io-index"
[net]
git-fetch-with-cli=true
[http]
check-revoke = false
```
或者直接使用如下配置：
```toml
[source.crates-io]
replace-with = 'mirror' # 直接替换为mirror为tuna镜像

[source.mirror]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[net]
git-fetch-with-cli=true
[http]
check-revoke = false
```

# 使用rsproxy代理安装rust更快
步骤一：设置 Rustup 镜像， 修改配置 ~/.zshrc or ~/.bashrc
```shell
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```
步骤二：安装 Rust（请先完成步骤一的环境变量导入并 source rc 文件或重启终端生效）
```shell
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```

步骤三：设置 crates.io 镜像， 修改配置 ~/.cargo/config.toml，已支持git协议和sparse协议，Rust >=1.68 版本建议使用 sparse-index，速度更快。
```toml
# 使用rsproxy
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli=true
[http]
check-revoke = false
```

# docker环境搭建参考
- https://github.com/rust-lang/docker-rust/tree/master/1.77.2
- https://github.com/daheige/pyo3-in-action/blob/main/Dockerfile

# rust编辑器选择

可以使用vscode,clion都可以

# vscode配置

需要安装好 rust-analyzer 插件，然后配置setting.json

``` json
{
    "files.eol": "\n",
    "editor.formatOnSave": true,
    "editor.fontSize": 13,
    "workbench.colorTheme": "Monokai",
    "rust.all_features": true,
    "editor.formatOnPaste": true,
    "editor.multiCursorModifier": "ctrlCmd",
    "editor.snippetSuggestions": "top",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

# rust语言程序设计

- rust程序设计第二版 https://kaisery.github.io/trpl-zh-cn/title-page.html
- 配套代码 https://github.com/daheige/myrust

# rust package发布方法

https://zhuanlan.zhihu.com/p/477390034

# rust crates官网

https://crates.io/
可以搜到很多有用的第三方包，比如axum,actix-web,tide,serde等等

# rust std库

- https://rustwiki.org/zh-CN/std/
- https://doc.rust-lang.org/std
