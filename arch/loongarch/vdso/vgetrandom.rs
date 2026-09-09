// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Xi Ruoyao <xry111@xry111.site>. All Rights Reserved.
 */

use core::ffi::c_void;

unsafe extern "C" {
    fn __cvdso_getrandom(
        buffer: *mut c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut c_void,
        opaque_len: usize,
    ) -> isize;
}

pub unsafe extern "C" fn __vdso_getrandom(
    buffer: *mut c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut c_void,
    opaque_len: usize,
) -> isize {
    unsafe { __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
