fn main() {
    use cc::Build;
    // 编译寻找到的.s文件
    Build::new()
        // .no_default_flags(true) // 不使用默认的编译参数
        // .files(&entry) // 传递寻找的汇编文件
        .file("src/asm.s")
        // .pic(true) // 配置编译器是否将发出调试信息,默认为false
        // .static_flag(true) // 设置-static 编译参数
        // .shared_flag(false) // 不设置-shared 编译参数
        // .out_dir("")
        .compile("test_asm"); // 指定编译的名称test_asm，该名称必须要与#[link(name = "test_asm", kind = "static")]中的name一致
    println!("cargo:rustc-link-lib=static=test_asm");
}
