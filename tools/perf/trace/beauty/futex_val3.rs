// SPDX-License-Identifier: LGPL-2.1
// Depends on trace/beauty/beauty.h and linux/futex.h declarations.

use core::ffi::{c_char, c_uint};

const FUTEX_BITSET_MATCH_ANY: c_uint = 0xffffffff;

extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

#[repr(C)]
pub struct syscall_arg {
    pub val: u64,
    pub show_string_prefix: bool,
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_futex_val3(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let prefix = b"FUTEX_BITSET_\0";
    let bitset = (*arg).val as c_uint;

    if bitset == FUTEX_BITSET_MATCH_ANY {
        return scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if (*arg).show_string_prefix {
                prefix.as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"MATCH_ANY\0".as_ptr() as *const c_char,
        );
    }

    scnprintf(bf, size, b"%#xd\0".as_ptr() as *const c_char, bitset)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
