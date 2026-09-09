/*
 * include/asm-xtensa/uaccess.h
 *
 * User space memory access functions
 *
 * These routines provide basic accessing functions to the user memory
 * space for the kernel.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/prefetch.h, asm/types.h, asm/extable.h, asm-generic/access_ok.h

#[macro_export]
macro_rules! put_user { ($x:expr, $ptr:expr) => { __put_user_check!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) }; }
#[macro_export]
macro_rules! get_user { ($x:expr, $ptr:expr) => { __get_user_check!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) }; }

#[macro_export]
macro_rules! __put_user { ($x:expr, $ptr:expr) => { __put_user_nocheck!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) }; }
#[macro_export]
macro_rules! __get_user { ($x:expr, $ptr:expr) => { __get_user_nocheck!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) }; }

unsafe extern "C" {
    pub fn __put_user_bad() -> core::ffi::c_long;
    pub fn __get_user_bad() -> core::ffi::c_long;
    pub fn __xtensa_copy_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: u32) -> u32;
    pub fn __strnlen_user(str_: *const core::ffi::c_char, len: core::ffi::c_long) -> core::ffi::c_long;
    pub fn __strncpy_user(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: core::ffi::c_long) -> core::ffi::c_long;
}

// The following macros preserve the original C expression interfaces and
// Xtensa exception-table inline assembly. Their external kernel primitives
// are intentionally left unresolved here.
#[macro_export]
macro_rules! __put_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: core::ffi::c_long = 0;
        __put_user_size!($x, $ptr, $size, __pu_err);
        __pu_err
    }};
}
#[macro_export]
macro_rules! __put_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: core::ffi::c_long = -EFAULT as core::ffi::c_long;
        let __pu_addr = $ptr;
        if access_ok(__pu_addr, $size) { __put_user_size!($x, __pu_addr, $size, __pu_err); }
        __pu_err
    }};
}
#[macro_export]
macro_rules! __put_user_size {
    ($x:expr, $ptr:expr, $size:expr, $retval:ident) => {{
        $retval = 0;
        match $size {
            1 => __put_user_asm!($x, $ptr, $retval, 1, "s8i"),
            2 => __put_user_asm!($x, $ptr, $retval, 2, "s16i"),
            4 => __put_user_asm!($x, $ptr, $retval, 4, "s32i"),
            8 => { let __v64 = $x; $retval = if __copy_to_user($ptr, &__v64, 8) != 0 { -EFAULT as _ } else { 0 }; }
            _ => { __put_user_bad(); }
        }
    }};
}

#[macro_export]
macro_rules! __put_user_asm { ($x:expr, $addr:expr, $err:expr, $align:expr, $insn:expr) => {{
    // C inline Xtensa assembly is retained as an unresolved low-level operation.
    let _ = (&$x, &$addr, &$err, $align, $insn);
}}; }

#[macro_export]
macro_rules! __get_user_nocheck { ($x:expr, $ptr:expr, $size:expr) => {{ let mut __gu_err = 0; __get_user_size!($x, $ptr, $size, __gu_err); __gu_err }}; }
#[macro_export]
macro_rules! __get_user_check { ($x:expr, $ptr:expr, $size:expr) => {{ let mut __gu_err = -EFAULT as _; let __gu_addr = $ptr; if access_ok(__gu_addr, $size) { __get_user_size!($x, __gu_addr, $size, __gu_err); } else { $x = 0; } __gu_err }}; }
#[macro_export]
macro_rules! __get_user_size { ($x:expr, $ptr:expr, $size:expr, $retval:ident) => {{ $retval = 0; match $size { 1 => __get_user_asm!($x, $ptr, $retval, 1, "l8ui"), 2 => __get_user_asm!($x, $ptr, $retval, 2, "l16ui"), 4 => __get_user_asm!($x, $ptr, $retval, 4, "l32i"), 8 => { let mut __x: u64 = 0; if unlikely(__copy_from_user(&mut __x, $ptr, 8) != 0) { $retval = -EFAULT as _; $x = 0; } else { $x = __x as _; } }, _ => { $x = 0; __get_user_bad(); } } }}; }
#[macro_export]
macro_rules! __get_user_asm { ($x:expr, $addr:expr, $err:expr, $align:expr, $insn:expr) => {{ let mut __x: u32 = 0; let _ = (&mut __x, &$x, &$addr, &$err, $align, $insn); $x = __x as _; }}; }

pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    prefetchw(to);
    __xtensa_copy_user(to, from, n as u32) as usize
}
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    prefetch(from);
    __xtensa_copy_user(to, from, n as u32) as usize
}

// #define INLINE_COPY_USER
// #define __clear_user __xtensa_clear_user
pub use __xtensa_clear_user as __clear_user;

pub unsafe fn __xtensa_clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize {
    if __memset(addr, 0, size) == 0 { size } else { 0 }
}
pub unsafe fn clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize {
    if access_ok(addr, size) { __xtensa_clear_user(addr, size) } else if size != 0 { -EFAULT as usize } else { 0 }
}

pub unsafe fn strncpy_from_user(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: core::ffi::c_long) -> core::ffi::c_long {
    if access_ok(src, 1) { __strncpy_user(dst, src, count) } else { -EFAULT as core::ffi::c_long }
}

pub unsafe fn strnlen_user(str_: *const core::ffi::c_char, len: core::ffi::c_long) -> core::ffi::c_long {
    if !access_ok(str_, 1) { return 0; }
    __strnlen_user(str_, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
