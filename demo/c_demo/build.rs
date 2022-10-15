// build.rs

fn main() {
    cc::Build::new()
        .file("src/hello.cpp")
        .static_flag(true)
        .compile("hello");
    // println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rerun-if-changed=src/hello.cpp");
}


// build.rs

// use std::process::Command;
// use std::env;
// use std::path::Path;
//
// fn main() {
//     let out_dir = env::var("OUT_DIR").unwrap();
//
//     // Note that there are a number of downsides to this approach, the comments
//     // below detail how to improve the portability of these commands.
//     Command::new("gcc").args(&["src/hello.cpp", "-c", "-fPIC", "-o"])
//         .arg(&format!("{}/hello.o", out_dir))
//         .status().unwrap();
//     Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
//         .current_dir(&Path::new(&out_dir))
//         .status().unwrap();
//
//     println!("cargo:rustc-link-search=native={}", out_dir);
//     println!("cargo:rustc-link-lib=static=hello");
//     println!("cargo:rerun-if-changed=src/hello.cpp");
// }
