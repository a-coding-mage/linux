// SPDX-License-Identifier: GPL-2.0
// C++ source included:
//   llvm/Support/ManagedStatic.h
//   llvm/Support/raw_ostream.h
//
// Original version macro:
//   NUM_VERSION = (LLVM_VERSION_MAJOR << 16) + (LLVM_VERSION_MINOR << 8) + LLVM_VERSION_PATCH
// Original build-time condition:
//   #if NUM_VERSION < 0x030900
//   # error "LLVM version too low"
//   #endif

extern "C" {
    #[link_name = "_ZN4llvm4errsEv"]
    fn llvm_errs() -> *mut llvm_raw_ostream;

    #[link_name = "_ZN4llvm13llvm_shutdownEv"]
    fn llvm_shutdown();

    #[link_name = "_ZlsRN4llvm11raw_ostreamEPKc"]
    fn llvm_raw_ostream_shl_cstr(
        stream: *mut llvm_raw_ostream,
        value: *const ::std::os::raw::c_char,
    ) -> *mut llvm_raw_ostream;
}

#[repr(C)]
pub struct llvm_raw_ostream {
    _private: [u8; 0],
}

pub unsafe fn main() -> i32 {
    llvm_raw_ostream_shl_cstr(llvm_errs(), b"Hello World!\n\0".as_ptr().cast());
    llvm_shutdown();
    0
}
