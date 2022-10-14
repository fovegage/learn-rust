#[link(name = "test_asm", kind = "static")] // 定义链接名称，使用的是静态链接方式
extern "C" {
    // target_env = "gnu"
    #[cfg(link_name = "nop_func")]
    // link_name必须与asm.s文件中.global后导出名称一致，
    // target_env = "gnu"，target_env表示使用gun环境如果不添加则会出现
    // unresolved import `asm_rust::asm::nop_func`
    // 这段代码只能在linux中使用
    pub fn nop_func();
}
