// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the included C headers:
// asm/facility.h, uapi/asm-generic/errno.h, and vdso.h.

use core::ffi::c_void;

extern "C" {
    fn test_facility(facility: i32) -> bool;
    fn __cvdso_getrandom(
        buffer: *mut c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut c_void,
        opaque_len: usize,
    ) -> isize;
    fn getrandom_syscall(buffer: *mut c_void, len: usize, flags: u32) -> isize;
}

// ENOSYS from uapi/asm-generic/errno.h.
const ENOSYS: isize = 38;

pub unsafe extern "C" fn __kernel_getrandom(
    buffer: *mut c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut c_void,
    opaque_len: usize,
) -> isize {
    if test_facility(129) {
        return __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len);
    }
    if opaque_len == usize::MAX && buffer.is_null() && len == 0 && flags == 0 {
        return -ENOSYS;
    }
    getrandom_syscall(buffer, len, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
