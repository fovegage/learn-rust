## 说明

```
# 教程
https://www.cnblogs.com/actra/p/14880013.html
cargo build 会生成rust proto 会执行 Cargo.toml ---> package ---> build
Cargo.lock 类似 yarn.lock

# 生成 protoc-gen-rust-grpc 
cargo install protobuf
https://docs.rs/protoc-rust/latest/protoc_rust/struct.Codegen.html#method.include
```

## 命令

```
# rust 包约定管理结构
cargo new grpc-demo
# target/debug/ 下生成 server 和 client 執行文件
cargo build
# 运行项目
cargo run
```