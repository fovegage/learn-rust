// src/main.rs

// 注意，这里没有再使用 `#[link]` 属性。我们把选择使用哪个 link 的责任交给了构建脚本，而不是在这里进行硬编码
extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}
