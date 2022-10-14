#![allow(unused)]

fn main() {
    use std::arch::asm;
    // 使用 shifts 和 adds 实现 x 乘以 6
    let mut x: u64 = 4;
    unsafe {
        asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
}
