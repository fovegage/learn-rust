// use std::arch::asm;

// extern crate test_asm;

// use test_asm::asm::{test_add, nop_func};

fn main() {
    // unsafe { nop_func() };
    println!("hello")
}

/////////////////////// 运行 c 无问题
// ffi/rust-call-c/src/main.rs
// 标准库<stdlib.h>内置的abs函数
// extern "C" {
//     #[link_name = "abs"]
//     fn abs_in_rust(input: i32) -> i32;
// }
//
// fn main() {
//     unsafe {
//         println!("abs(-1) is {}", abs_in_rust(-1));
//     }
// }

