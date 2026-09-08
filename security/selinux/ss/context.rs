// SPDX-License-Identifier: GPL-2.0
/*
 * Implementations of the security context functions.
 *
 * Author: Ondrej Mosnacek <omosnacek@gmail.com>
 * Copyright (C) 2020 Red Hat, Inc.
 */

// Dependencies from the original C includes:
// #include <linux/jhash.h>
// #include "context.h"
// #include "mls.h"

use core::ffi::c_void;
use core::ptr;

extern "C" {
    fn full_name_hash(salt: *const c_void, name: *const core::ffi::c_char, len: u32) -> u32;
    fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32;
    fn mls_range_hash(range: *const mls_range, hash: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn context_compute_hash(c: *const context) -> u32 {
    let mut hash: u32 = 0;

    /*
     * If a context is invalid, it will always be represented by a
     * context struct with only the len & str set (and vice versa)
     * under a given policy. Since context structs from different
     * policies should never meet, it is safe to hash valid and
     * invalid contexts differently. The context_equal() function
     * already operates under the same assumption.
     */
    if unsafe { (*c).len } != 0 {
        return unsafe { full_name_hash(ptr::null(), (*c).str, (*c).len) };
    }

    hash = unsafe { jhash_3words((*c).user, (*c).role, (*c).type_, hash) };
    hash = unsafe { mls_range_hash(&raw const (*c).range, hash) };
    hash
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
