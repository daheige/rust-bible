# rust学习笔记
    基于rust edition = "2021" 
    需要安装rust v1.58.0 以上版本

# rust从入门到实战--7大块
- part1 rust基础,thread concurrent编程初体验
- part2 serde json序列化处理，yaml文件读取，log日志库基础操作
- part3 redis,mysql,kafka和pulsar mq基本使用
- part4 rust异步编程基础--tokio和async-std两种不同的异步操作库使用
- part5 axum,actix-web,tide框架使用
- part6 rust 依赖注入和clap命令终端使用
- part7 rust实战(rust web/job/grpc）综合使用

# rust安装
- https://www.rust-lang.org/zh-CN/tools/install
- https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html

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

# rust 1.70.x以上版本的crates发布
```shell
cargo publish --registry crates-io
```

# 设置rust国内镜像
    https://mirrors.tuna.tsinghua.edu.cn/help/rustup/

	国内提高访问速度，建议设置环境变量 
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    export PATH="$HOME/.cargo/bin:$PATH"

	在用户目录.cargo文件夹或在与Cargo.toml同级目录.cargo文件夹下创建config文件
	$cd ~/.cargo/
	$touch config
	添加如下内容：
	# 指定镜像
    #replace-with = 'sjtu'
    replace-with = 'ustc'
    
    # 源码地址
    [source.crates-io]
    registry = "https://github.com/rust-lang/crates.io-index"
    
    # 清华大学
    [source.tuna]
    registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
    
    # 中国科学技术大学
    [source.ustc]
    registry = "https://mirrors.ustc.edu.cn/crates.io-index"
    #registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    
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
