## 基础

```
# 包
cargo new demo # 新建项目
cargo new demo --lib # 新建 dll 项目
cargo install playwright # 安装包
cargo run # 运行
cargo build # 打包

# 文件
rustc *.rs # 编译二进制，编译好运行

-------------------------------
# 依赖放入dependencies会自动下载
[dependencies]
playwright = "*
# 仓库类似于 pypi.python.org
https://crates.io/
```

## windows

```
# error: linker `link.exe` not found
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu # 设置默认toolchain
```

## 命令

```
# 安装及其命令
brew install rust
rustc --version
cargo --version

brew install rustup-init
brew cleanup rustup-init
rustup-init 会生产 rust-up
rustup

## 主要命令
rustc --version、cargo --version、rustup --version
```

## 概念

```
Cargo 是 Rust 的构建系统和包管理器。
rustc是 Rust 编程语言的编译器(http://llever.com/rustc-zh/)
rust-gdb     
rust-gdbgui  
rust-lldb    
rustc  # 编译器  可以直接使用cargo build 
rustdoc 
rust-src rust标准库
rustup 交叉编译
clippy 优化代码用

```

## 资料

### 基础资料

- [Rust Crates 实践指南](https://mirrors.gitcode.host/zzy/rust-crate-guide)
- [Rust 基础](https://learnku.com/rust/wikis/29018)
- [Rust 参考手册](https://rustwiki.org/zh-CN/reference/names/preludes.html)

### Rust浏览器

- [Servo](https://github.com/servo/servo)

## 规划

```
1. 建立rust中文论坛（rust-china.cn）
```

## 配置

```
## http
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

## git
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

## 常用

```
---
gaozhe@gaozhedeMacBook-Pro learn-rust % ls $HOME/.cargo/bin 
cargo           cargo-fmt       clippy-driver   rust-gdb        rust-lldb       rustdoc         rustup
cargo-clippy    cargo-miri      rls             rust-gdbgui     rustc           rustfmt
---
du -sh ./* 
---
```

## 其他

```
# rust浏览器
https://github.com/cisen/blog/issues/615
https://github.com/mbrubeck/robinson
# rust WebAssembly
https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm
# rust yew前端框架
https://github.com/yewstack/yew
https://yew.rs/zh-Hant/
# rust grpc demo
https://www.cnblogs.com/actra/p/14880013.html
# rust v8 
https://morioh.com/p/164c75aa64f3
# rust window dll hook
https://blog.csdn.net/kunyus/article/details/108884016
```
