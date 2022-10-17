#![allow(unused)]

use std::arch::asm;


// fn main() {
//     let x: u64;
//     unsafe {
//         asm!("mov {}, 5", out(reg) x);
//     }
//     assert_eq!(x, 5);
// }

fn main() {
    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);
}







