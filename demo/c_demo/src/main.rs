// src/main.rs

// 注意，这里没有再使用 `#[link]` 属性。我们把选择使用哪个 link 的责任交给了构建脚本，而不是在这里进行硬编码
extern {
    fn multiply(x: i32, y: i32) -> i32;
}

// 标准库<stdlib.h>内置的abs函数
extern "C" {
    #[link_name = "abs"]
    fn abs_in_rust(input: i32) -> i32;
}

fn main() {
    unsafe {
        // 使用外部的包
        println!("{}", multiply(5, 7));
        // 使用内部的包
        println!("abs(-1) is {}", abs_in_rust(-1));
    }
}

