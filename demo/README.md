## 理解

```
# 生成 dll
cargo new dll-demo --lib

# 编译
cargo build

# 生成dll
cargo build --release

# 编译windows dll
rustup target list # 支持的平台
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc

# 运行
cargo run --target x86_64-apple-darwin

# apple 
rustup target add --toolchain stable i686-pc-windows-msvc
rustup target add --toolchain stable x86_64-pc-windows-msvc

env CROSS_COMPILE=x86_64-pc-windows-msvc cargo build --release --target x86_64-pc-windows-msvc

# 安装的交叉编译工具
rustup show
--------------------------------------
installed targets for active toolchain
--------------------------------------

aarch64-apple-darwin
i686-pc-windows-msvc
x86_64-apple-darwin
x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-aarch64-apple-darwin (default)
rustc 1.64.0 (a55dd71d5 2022-09-19)

# 更新
rustup +nightly component add rust-src
```

## rustup

```
# https://blog.csdn.net/inthat/article/details/106742193
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 工具链版本
stable beta nightly
# 默认工具链位置
/Users/gaozhe/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust

# 添加工具链
rustup target add --toolchain stable i686-pc-windows-msvc

```

## 其他

```
# dll说明
Windows系统的动态库是DLL文件
Linux系统是so文件
macOS系统的动态库则使用dylib文件作为动态库
```

## 逆向

```
# 植入websocket 发送接收命令

# rust 做client  golang做server

```