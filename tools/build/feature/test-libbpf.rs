// SPDX-License-Identifier: GPL-2.0
//
// C dependency intent:
// #include <bpf/libbpf.h>
//
// C build-time requirement:
// #if !defined(LIBBPF_MAJOR_VERSION) || (LIBBPF_MAJOR_VERSION < 1)
// #error At least libbpf 1.0 is required for Linux tools.
// #endif

use std::os::raw::c_char;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
}

fn main() -> i32 {
    unsafe {
        if !bpf_object__open(b"test\0".as_ptr() as *const c_char).is_null() {
            0
        } else {
            -1
        }
    }
}
