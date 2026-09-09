// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/lib/kasprintf.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the surrounding kernel translation.
pub type gfp_t = c_uint;
pub type va_list = *mut c_void;

extern "C" {
    fn va_copy(dst: va_list, src: va_list);
    fn va_end(ap: va_list);
    fn va_arg(ap: va_list, ty: *const c_void) -> *const c_char;
    fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ap: va_list) -> c_uint;
    fn kmalloc_track_caller(size: usize, gfp: gfp_t) -> *mut c_char;
    fn kstrdup_const(s: *const c_char, gfp: gfp_t) -> *const c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn warn(condition: bool, fmt: *const c_char, ...);
}

/* Simplified asprintf. */
pub unsafe extern "C" fn kvasprintf(
    gfp: gfp_t,
    fmt: *const c_char,
    ap: va_list,
) -> *mut c_char {
    let first: c_uint;
    let second: c_uint;
    let p: *mut c_char;
    let aq: va_list;

    va_copy(aq, ap);
    first = vsnprintf(core::ptr::null_mut(), 0, fmt, aq);
    va_end(aq);

    p = kmalloc_track_caller(first.wrapping_add(1) as usize, gfp);
    if p.is_null() {
        return core::ptr::null_mut();
    }

    second = vsnprintf(p, first.wrapping_add(1) as usize, fmt, ap);
    warn(
        first != second,
        c"different return values (%u and %u) from vsnprintf(\"%s\", ...)".as_ptr(),
        first,
        second,
        fmt,
    );

    p
}

/*
 * If fmt contains no % (or is exactly %s), use kstrdup_const. If fmt
 * (or the sole vararg) points to rodata, we will then save a memory
 * allocation and string copy. In any case, the return value should be
 * freed using kfree_const().
 */
pub unsafe extern "C" fn kvasprintf_const(
    gfp: gfp_t,
    fmt: *const c_char,
    ap: va_list,
) -> *const c_char {
    if strchr(fmt, '%' as c_int).is_null() {
        return kstrdup_const(fmt, gfp);
    }
    if strcmp(fmt, c"%s".as_ptr()) == 0 {
        return kstrdup_const(va_arg(ap, core::ptr::null()), gfp);
    }
    kvasprintf(gfp, fmt, ap)
}

pub unsafe extern "C" fn kasprintf(
    gfp: gfp_t,
    fmt: *const c_char,
    mut args: ...,
) -> *mut c_char {
    let ap: va_list = &mut args as *mut _ as va_list;
    kvasprintf(gfp, fmt, ap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
