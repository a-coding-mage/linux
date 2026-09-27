// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2020, Microsoft Corporation.
 *
 *   Author(s): Steve French <stfrench@microsoft.com>
 *              Suresh Jayaraman <sjayaraman@suse.de>
 *              Jeff Layton <jlayton@kernel.org>
 */

// Dependencies supplied by the surrounding kernel/CIFS implementation.

extern "C" {
    fn strlen(s: *const kernel::ffi::c_char) -> usize;
    fn strchr(s: *const kernel::ffi::c_char, c: kernel::ffi::c_int)
        -> *mut kernel::ffi::c_char;
    fn kmalloc(size: usize, flags: kernel::ffi::c_ulong) -> *mut kernel::ffi::c_void;
    fn memcpy(
        dest: *mut kernel::ffi::c_void,
        src: *const kernel::ffi::c_void,
        count: usize,
    ) -> *mut kernel::ffi::c_void;
    fn kstrdup(s: *const kernel::ffi::c_char, flags: kernel::ffi::c_ulong)
        -> *mut kernel::ffi::c_char;
    fn ERR_PTR(error: kernel::ffi::c_long) -> *mut kernel::ffi::c_char;
}

// GFP_KERNEL is supplied by the kernel headers.
const GFP_KERNEL: kernel::ffi::c_ulong = 0;
const EINVAL: kernel::ffi::c_long = 22;
const ENOMEM: kernel::ffi::c_long = 12;

/* extract the host portion of the UNC string */
pub unsafe extern "C" fn extract_hostname(
    unc: *const kernel::ffi::c_char,
) -> *mut kernel::ffi::c_char {
    let mut src: *const kernel::ffi::c_char;
    let dst: *mut kernel::ffi::c_char;
    let delim: *mut kernel::ffi::c_char;
    let len: kernel::ffi::c_uint;

    /* skip double chars at beginning of string */
    /* BB: check validity of these bytes? */
    if strlen(unc) < 3 {
        return ERR_PTR(-EINVAL);
    }
    src = unc;
    while *src != 0 && *src == b'\\' as kernel::ffi::c_char {
        src = src.add(1);
    }
    if *src == 0 {
        return ERR_PTR(-EINVAL);
    }

    /* delimiter between hostname and sharename is always '\\' now */
    delim = strchr(src, b'\\' as kernel::ffi::c_int);
    if delim.is_null() {
        return ERR_PTR(-EINVAL);
    }

    len = delim.offset_from(src) as kernel::ffi::c_uint;
    dst = kmalloc((len + 1) as usize, GFP_KERNEL) as *mut kernel::ffi::c_char;
    if dst.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    memcpy(dst as *mut kernel::ffi::c_void, src as *const kernel::ffi::c_void, len as usize);
    *dst.add(len as usize) = 0;

    dst
}

pub unsafe extern "C" fn extract_sharename(
    unc: *const kernel::ffi::c_char,
) -> *mut kernel::ffi::c_char {
    let src: *const kernel::ffi::c_char;
    let delim: *mut kernel::ffi::c_char;
    let dst: *mut kernel::ffi::c_char;

    /* skip double chars at the beginning */
    src = unc.add(2);

    /* share name is always preceded by '\\' now */
    delim = strchr(src, b'\\' as kernel::ffi::c_int);
    if delim.is_null() {
        return ERR_PTR(-EINVAL);
    }
    let delim = delim.add(1);

    /* caller has to free the memory */
    dst = kstrdup(delim, GFP_KERNEL);
    if dst.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    dst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
