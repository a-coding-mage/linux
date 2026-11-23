// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/lib/kasprintf.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Dependencies: linux/stdarg.h, linux/slab.h, linux/types.h, linux/string.h.
//
// C-variadic kasprintf() and the va_list kvasprintf*() use the compiler's
// VaList ABI, admitted for this object by lib/Makefile (RUST_KASPRINTF
// requires the inspected rustc 1.85.1, as RUST_PANIC does). The
// kmalloc_track_caller() site receives kvasprintf()'s own return address,
// the C unit's _RET_IP_.

use core::ffi::VaList;
use core::ptr::null_mut;
use kernel::bindings::{gfp_t, kstrdup_const, strchr, strcmp};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    #[link_name = "vsnprintf"]
    fn native_vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: VaList<'_, '_>) -> c_int;
    #[link_name = "llvm.returnaddress"]
    fn return_address(level: c_int) -> *mut c_void;

    fn lupos_kasprintf_kmalloc_track_caller(size: usize, gfp: gfp_t, caller: c_ulong) -> *mut c_char;
    fn lupos_kasprintf_warn_mismatch(first: c_uint, second: c_uint, fmt: *const c_char);
}

/* Simplified asprintf. */
/// Returns a newly allocated string formatted from `fmt` and `ap`.
///
/// # Safety
/// `fmt` and `ap` form a valid printf format and argument list.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kvasprintf(gfp: gfp_t, fmt: *const c_char, ap: VaList<'_, '_>) -> *mut c_char {
    // SAFETY: va_copy() brackets the sizing pass; the second pass writes at
    // most first + 1 bytes into the allocation.
    unsafe {
        let caller = return_address(0) as c_ulong;

        let first = ap.with_copy(|aq| native_vsnprintf(null_mut(), 0, fmt, aq)) as c_uint;

        let p = lupos_kasprintf_kmalloc_track_caller(first.wrapping_add(1) as usize, gfp, caller);
        if p.is_null() {
            return null_mut();
        }

        let second = native_vsnprintf(p, first.wrapping_add(1) as usize, fmt, ap) as c_uint;
        if first != second {
            lupos_kasprintf_warn_mismatch(first, second, fmt);
        }

        p
    }
}
// EXPORT_SYMBOL(kvasprintf);

/*
 * If fmt contains no % (or is exactly %s), use kstrdup_const. If fmt
 * (or the sole vararg) points to rodata, we will then save a memory
 * allocation and string copy. In any case, the return value should be
 * freed using kfree_const().
 */
/// # Safety
/// `fmt` and `ap` form a valid printf format and argument list.
#[no_mangle]
pub unsafe extern "C" fn kvasprintf_const(gfp: gfp_t, fmt: *const c_char, mut ap: VaList<'_, '_>) -> *const c_char {
    // SAFETY: fmt is NUL-terminated; a "%s" format carries one string.
    unsafe {
        if strchr(fmt, c_int::from(b'%')).is_null() {
            return kstrdup_const(fmt, gfp);
        }
        if strcmp(fmt, c"%s".as_ptr().cast()) == 0 {
            return kstrdup_const(ap.arg::<*const c_char>(), gfp);
        }
        kvasprintf(gfp, fmt, ap)
    }
}
// EXPORT_SYMBOL(kvasprintf_const);

/// Returns a newly allocated string formatted from `fmt` and its arguments.
///
/// # Safety
/// `fmt` is a printf format matching the variadic arguments.
#[no_mangle]
pub unsafe extern "C" fn kasprintf(gfp: gfp_t, fmt: *const c_char, mut args: ...) -> *mut c_char {
    // SAFETY: va_start/va_end bracket this call, as in the C body.
    unsafe { kvasprintf(gfp, fmt, args.as_va_list()) }
}
// EXPORT_SYMBOL(kasprintf);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
