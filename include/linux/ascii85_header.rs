/*
 * SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2008 Intel Corporation
 * Copyright (c) 2018 The Linux Foundation. All rights reserved.
 */

// Translated from ascii85.h. The original dependencies are linux/math.h and
// linux/types.h.

use core::ffi::{c_char, c_long};

pub const ASCII85_BUFSZ: usize = 6;

#[inline]
pub fn ascii85_encode_len(len: c_long) -> c_long {
    (len + 3) / 4
}

#[inline]
pub unsafe fn ascii85_encode(r#in: u32, out: *mut c_char) -> *const c_char {
    let mut input = r#in;

    if input == 0 {
        return b"z\0".as_ptr() as *const c_char;
    }

    *out.add(5) = 0;
    let mut i = 5;
    while i != 0 {
        i -= 1;
        *out.add(i as usize) = (b'!' as u32 + input % 85) as c_char;
        input /= 85;
    }

    out as *const c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
