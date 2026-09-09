// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Dependency supplied by lib/vdso/getrandom.c.
unsafe extern "C" {
    fn __cvdso_getrandom(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;
}

pub unsafe fn __vdso_getrandom(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut core::ffi::c_void,
    opaque_len: usize,
) -> isize {
    unsafe { __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len) }
}

// C weak alias: getrandom aliases __vdso_getrandom.
#[inline]
pub unsafe fn getrandom(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut core::ffi::c_void,
    opaque_len: usize,
) -> isize {
    unsafe { __vdso_getrandom(buffer, len, flags, opaque_state, opaque_len) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
