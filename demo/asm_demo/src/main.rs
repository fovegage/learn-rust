// use std::arch::asm;
//
// extern crate test_asm;
//
// use test_asm::asm::{test_add, nop_func};
// #[link(name = "asm", kind = "static")] // 定义链接名称，使用的是静态链接方式
// extern {
//     // #[cfg(link_name = "nop_func")]
//     // link_name必须与asm.s文件中.global后导出名称一致，
//     // target_env = "gnu"，target_env表示使用gun环境如果不添加则会出现
//     // unresolved import `asm_rust::asm::nop_func`
//     // 这段代码只能在linux中使用
//     // #[cfg(link_name = "nop_func")]
//     fn nop_func();
// }

use std::arch::asm;

fn main() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
        "mov {0}, {1}",
        "add {0}, {number}",
        out(reg) o,
        in(reg) i,
        number = const 5,
        );
    }
    assert_eq!(o, 8);
    // unsafe { nop_func() };
    // println!("hello")
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

