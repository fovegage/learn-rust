// in call_test/build.rs
fn main() {
    // use std::ffi::OsString;
    // use std::fs;
    use cc::Build;
    // 在src/asm文件夹中寻找所有以.s结尾的文件
    // let entry = fs::read_dir("src/asm").unwrap().filter_map(|f| {
    //     f.ok().and_then(|e| {
    //         let path = e.path();
    //         match path.extension() {
    //             Some(ext) if ext.eq(&OsString::from("s")) => Some(path),
    //             _ => None
    //         }
    //     })
    // }).collect::<Vec<_>>();

    // 编译寻找到的.s文件
    Build::new()

        .no_default_flags(true) // 不使用默认的编译参数
        // .files(&entry) // 传递寻找的汇编文件
        .file("src/asm.s")
        .pic(true) // 配置编译器是否将发出调试信息,默认为false
        .static_flag(true) // 设置-static 编译参数
        .shared_flag(false) // 不设置-shared 编译参数
        // .out_dir("")
        .compile("asm"); // 指定编译的名称test_asm，该名称必须要与#[link(name = "test_asm", kind = "static")]中的name一致
    // println!("cargo:rustc-link-lib=static=test_asm");
}
