// SPDX-License-Identifier: BSD-3-Clause
/*
 * linux/net/sunrpc/auth_gss/auth_gss_internal.h
 *
 * Internal definitions for RPCSEC_GSS client authentication
 *
 * Copyright (c) 2000 The Regents of the University of Michigan.
 * All rights reserved.
 *
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel/Rust translation.
extern "C" {
    fn kmemdup_noprof(src: *const c_void, len: usize, gfp: u32) -> *mut c_void;
}

const GFP_KERNEL: u32 = 0;
const EFAULT: isize = 14;
const ENOMEM: isize = 12;

#[inline]
pub unsafe fn simple_get_bytes(
    p: *const c_void,
    end: *const c_void,
    res: *mut c_void,
    len: usize,
) -> *const c_void {
    let q = (p as *const u8).add(len) as *const c_void;
    if (q as usize) > (end as usize) || (q as usize) < (p as usize) {
        return (-EFAULT) as isize as *const c_void;
    }
    core::ptr::copy_nonoverlapping(p as *const u8, res as *mut u8, len);
    q
}

#[inline]
pub unsafe fn simple_get_netobj_noprof(
    mut p: *const c_void,
    end: *const c_void,
    dest: *mut crate::xdr_netobj,
) -> *const c_void {
    let mut len: u32 = 0;

    p = simple_get_bytes(
        p,
        end,
        (&mut len as *mut u32).cast::<c_void>(),
        core::mem::size_of::<u32>(),
    );
    if (p as usize) >= (isize::MIN as usize) {
        return p;
    }
    let q = (p as *const u8).add(len as usize) as *const c_void;
    if (q as usize) > (end as usize) || (q as usize) < (p as usize) {
        return (-EFAULT) as isize as *const c_void;
    }
    if len != 0 {
        (*dest).data = kmemdup_noprof(p, len as usize, GFP_KERNEL);
        if (*dest).data.is_null() {
            return (-ENOMEM) as isize as *const c_void;
        }
    } else {
        (*dest).data = core::ptr::null_mut();
    }
    (*dest).len = len;
    q
}

// The C macro applies the allocator instrumentation hook at the call site.
macro_rules! simple_get_netobj {
    ($($arg:expr),* $(,)?) => {
        alloc_hooks!(simple_get_netobj_noprof($($arg),*))
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
