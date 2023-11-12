// 2023/11/09
// lipeng

mod cxx_call_cpp;

use cxx_call_cpp::ffi;

extern {fn hello();}

fn main() {

    // Use cc Called C Code
    unsafe {
        hello();
    }

    // Rust Called C++ Code
    ffi::hello_world();
}
