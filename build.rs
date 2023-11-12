
// build.rs

//extern crate cc;

fn main() {

    // Compile C++ Code
    // let source_files = vec!["src/main.rs", "src/cxx_call_cpp.rs"];
    cxx_build::bridge("src/cxx_call_cpp.rs")
        .file("src/hello_world.cpp")
        .compile("cpp_rust_demo");

    
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}