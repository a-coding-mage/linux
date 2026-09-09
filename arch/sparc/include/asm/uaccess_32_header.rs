/* SPDX-License-Identifier: GPL-2.0 */
/*
 * uaccess.h: User space memory access functions.
 *
 * Rust translation of the source header.  Linux/compiler.h,
 * linux/string.h, asm/processor.h, and asm-generic/access_ok.h provide
 * the referenced external definitions.
 */

#[repr(C)]
pub struct __large_struct {
    pub buf: [core::ffi::c_ulong; 100],
}

#[inline(always)]
pub unsafe fn __m(x: *mut core::ffi::c_void) -> *mut __large_struct {
    x as *mut __large_struct
}

extern "C" {
    pub fn __put_user_bad() -> core::ffi::c_int;
    pub fn __get_user_bad() -> core::ffi::c_int;
    pub fn __copy_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        size: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    pub fn __access_ok(addr: *const core::ffi::c_void, size: core::ffi::c_ulong) -> bool;
    pub fn strnlen_user(str_: *const core::ffi::c_char, n: core::ffi::c_long) -> core::ffi::c_long;
}

/* The original __user, __force, __chk_user_ptr, and __typeof__ annotations
 * are represented by raw pointers and explicit casts in this translation. */

#[macro_export]
macro_rules! __put_user_asm {
    ($x:expr, $size:tt, $addr:expr, $ret:ident) => {
        /* Architecture-specific fault-tolerant store; supplied by the target. */
        $ret = unsafe { $crate::__put_user_bad() };
    };
}

#[macro_export]
macro_rules! __get_user_asm {
    ($x:ident, $size:tt, $addr:expr, $ret:ident) => {
        /* Architecture-specific fault-tolerant load; supplied by the target. */
        $x = 0;
        $ret = unsafe { $crate::__get_user_bad() };
    };
}

#[macro_export]
macro_rules! __put_user_check {
    ($x:expr, $addr:expr, $size:expr) => {{
        let mut __pu_ret: core::ffi::c_int;
        if unsafe { $crate::__access_ok($addr as *const _, $size as core::ffi::c_ulong) } {
            match $size {
                1 => __put_user_asm!($x, b, $addr, __pu_ret),
                2 => __put_user_asm!($x, h, $addr, __pu_ret),
                4 => __put_user_asm!($x, , $addr, __pu_ret),
                8 => __put_user_asm!($x, d, $addr, __pu_ret),
                _ => __pu_ret = unsafe { $crate::__put_user_bad() },
            }
        } else {
            __pu_ret = -libc::EFAULT;
        }
        __pu_ret
    }};
}

#[macro_export]
macro_rules! __put_user_nocheck {
    ($x:expr, $addr:expr, $size:expr) => {{
        let mut __pu_ret: core::ffi::c_int;
        match $size {
            1 => __put_user_asm!($x, b, $addr, __pu_ret),
            2 => __put_user_asm!($x, h, $addr, __pu_ret),
            4 => __put_user_asm!($x, , $addr, __pu_ret),
            8 => __put_user_asm!($x, d, $addr, __pu_ret),
            _ => __pu_ret = unsafe { $crate::__put_user_bad() },
        }
        __pu_ret
    }};
}

#[macro_export]
macro_rules! __get_user_check {
    ($x:expr, $addr:expr, $size:expr, $ty:ty) => {{
        let mut __gu_ret: core::ffi::c_int;
        let mut __gu_val: core::ffi::c_ulong;
        if unsafe { $crate::__access_ok($addr as *const _, $size as core::ffi::c_ulong) } {
            match $size {
                1 => __get_user_asm!(__gu_val, ub, $addr, __gu_ret),
                2 => __get_user_asm!(__gu_val, uh, $addr, __gu_ret),
                4 => __get_user_asm!(__gu_val, , $addr, __gu_ret),
                8 => __get_user_asm!(__gu_val, d, $addr, __gu_ret),
                _ => { __gu_val = 0; __gu_ret = unsafe { $crate::__get_user_bad() }; }
            }
        } else { __gu_val = 0; __gu_ret = -libc::EFAULT; }
        $x = __gu_val as $ty;
        __gu_ret
    }};
}

#[macro_export]
macro_rules! __get_user_nocheck {
    ($x:expr, $addr:expr, $size:expr, $ty:ty) => {
        __get_user_check!($x, $addr, $size, $ty)
    };
}

#[macro_export]
macro_rules! put_user { ($x:expr, $ptr:expr) => { __put_user_check!($x, $ptr, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! get_user { ($x:expr, $ptr:expr) => { __get_user_check!($x, $ptr, core::mem::size_of_val(&*$ptr), _) }; }
#[macro_export]
macro_rules! __put_user { ($x:expr, $ptr:expr) => { __put_user_nocheck!($x, $ptr, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! __get_user { ($x:expr, $ptr:expr) => { __get_user_nocheck!($x, $ptr, core::mem::size_of_val(&*$ptr), _) }; }

#[inline(always)]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    __copy_user(to, from, n)
}

#[inline(always)]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    __copy_user(to, from, n)
}

#[allow(non_upper_case_globals)]
pub const INLINE_COPY_USER: () = ();

#[inline(always)]
pub unsafe fn __clear_user(addr: *mut core::ffi::c_void, size: core::ffi::c_ulong) -> core::ffi::c_ulong {
    /* Original implementation calls the external SPARC __bzero routine. */
    let _ = (addr, size);
    0
}

#[inline(always)]
pub unsafe fn clear_user(addr: *mut core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    if n != 0 && __access_ok(addr as *const _, n) { __clear_user(addr, n) } else { n }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
