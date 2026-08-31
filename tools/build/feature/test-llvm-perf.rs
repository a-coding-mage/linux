// SPDX-License-Identifier: GPL-2.0
// Depends on LLVM Support ManagedStatic and raw_ostream.

// C++ build-time condition:
// #if LLVM_VERSION_MAJOR < 13
// # error "Perf requires llvm-devel/llvm-dev version 13 or greater"
// #endif

extern "C" {
    #[link_name = "_ZN4llvm4errsEv"]
    fn llvm_errs() -> *mut llvm_raw_ostream;

    #[link_name = "_ZN4llvm13llvm_shutdownEv"]
    fn llvm_shutdown();

    #[link_name = "_ZN4llvm11raw_ostreamlsEPKc"]
    fn llvm_raw_ostream_shl_cstr(os: *mut llvm_raw_ostream, s: *const ::std::os::raw::c_char)
        -> *mut llvm_raw_ostream;
}

#[repr(C)]
pub struct llvm_raw_ostream {
    _private: [u8; 0],
}

fn main() -> i32 {
    unsafe {
        llvm_raw_ostream_shl_cstr(llvm_errs(), b"Hello World!\n\0".as_ptr() as *const _);
        llvm_shutdown();
    }

    0
}
