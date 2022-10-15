// build.rs

fn main() {
    cc::Build::new()
        .file("src/hello.cpp")
        .compile("hello");
    println!("cargo:rerun-if-changed=src/hello.cpp");
}
