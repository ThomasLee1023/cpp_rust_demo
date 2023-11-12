// src/cxx_call_cpp.rs

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cpp_rust_demo/include/hello_world.h");

        // Function implement in C++
        fn hello_world();

    }

}