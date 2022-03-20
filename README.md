# rust学习笔记
    从rust入门到实战
    基于rust edition = "2021" 
    需要安装rust v1.58.0 以上版本

# 学习笔记--7大块
    part1 rust基础,thread编程初体验
    part2 serde json序列化处理，yaml文件读取，log日志库基础操作
    part3 redis,mysql,kafka和pulsar mq基本使用
    part4 rust异步编程基础--tokio和async-std两种不同的异步操作库使用
    part5 axum,actix-web,tide框架使用
    part6 rust 依赖注入和ddd领域驱动设计
    part7 rust实战(rust web/job/grpc）综合使用

# rust安装

https://www.rust-lang.org/zh-CN/tools/install
https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

	建议安装到rust v1.58.0+版本
    shell安装
	curl https://sh.rustup.rs -sSf | sh
    对于centos7安装请看 rust-centos7-install.md
    
    rust升级执行如下操作：
    rustup update
    
    rust版本查看
    cargo --version
    cargo 1.58.0 (7f08ace4f 2021-11-24)
    升级到指定版本 rust update "1.58.0"

# 设置rust国内镜像

	国内提高访问速度，建议设置环境变量 
	export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
	export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

	在用户目录.cargo文件夹或在与Cargo.toml同级目录.cargo文件夹下创建config文件
	$cd ~/.cargo/
	$touch config
	添加如下内容：
	[source.crates-io]
	registry = "https://github.com/rust-lang/crates.io-index"
	# 指定镜像
	replace-with = 'ustc'

	# 清华大学
	[source.tuna]
	registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

	# 中国科学技术大学
	[source.ustc]
	registry = "git://mirrors.ustc.edu.cn/crates.io-index"

	# 上海交通大学
	[source.sjtu]
	registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

	# rustcc社区
	[source.rustcc]
	registry = "https://code.aliyun.com/rustcc/crates.io-index.git"

# rust编辑器

    可以使用vscode,clion都可以
    对于vscode配置
    rust vscode setting.json配置

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
    "rust-client.channel": "stable",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust"
    }
}
```

# rust语言程序设计

rust程序设计第二版 https://kaisery.github.io/trpl-zh-cn/title-page.html

配套代码 https://github.com/daheige/myrust
