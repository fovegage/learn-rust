// This is a comment
// hello.rs

// main function
fn main() {
    // rustc hello.rs 将生成二进制文件
    // ./hello 执行二进制文件
    println!("Hello World!");
}

#[cfg(test)]
fn tests() {
    #[test]
    main()
}