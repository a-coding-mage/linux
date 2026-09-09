// SPDX-License-Identifier: GPL-2.0
/*
 * Powerpc userspace implementation of getrandom()
 *
 * Copyright (C) 2024 Christophe Leroy <christophe.leroy@csgroup.eu>, CS GROUP France
 */

// Dependencies corresponding to <linux/time.h> and <linux/types.h> are
// supplied by the surrounding translation unit/build environment.

extern "C" {
    fn __cvdso_getrandom(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;
}

pub unsafe fn __c_kernel_getrandom(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut core::ffi::c_void,
    opaque_len: usize,
) -> isize {
    __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
