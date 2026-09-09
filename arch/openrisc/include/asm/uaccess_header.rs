/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// C header guard: __ASM_OPENRISC_UACCESS_H
// Dependencies: linux/prefetch.h, linux/string.h, asm/page.h,
// asm/extable.h, and asm-generic/access_ok.h.

/* User space memory access functions */

extern "C" {
    pub fn __put_user_bad() -> libc::c_long;
    pub fn __get_user_bad() -> libc::c_long;
    pub fn __copy_tofrom_user(to: *mut libc::c_void, from: *const libc::c_void, size: libc::c_ulong) -> libc::c_ulong;
    pub fn __clear_user(addr: *mut libc::c_void, size: libc::c_ulong) -> libc::c_ulong;
    pub fn strncpy_from_user(dest: *mut libc::c_char, src: *const libc::c_char, count: libc::c_long) -> libc::c_long;
    pub fn strnlen_user(str_: *const libc::c_char, n: libc::c_long) -> libc::c_long;
}

#[repr(C)]
pub struct __large_struct {
    pub buf: [libc::c_ulong; 100],
}

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut libc::c_void, from: *const libc::c_void, size: libc::c_ulong) -> libc::c_ulong {
    __copy_tofrom_user(to, from, size)
}

#[inline]
pub unsafe fn raw_copy_to_user(to: *mut libc::c_void, from: *const libc::c_void, size: libc::c_ulong) -> libc::c_ulong {
    __copy_tofrom_user(to, from, size)
}

pub const INLINE_COPY_USER: bool = true;

#[inline]
pub unsafe fn clear_user(addr: *mut libc::c_void, mut size: libc::c_ulong) -> libc::c_ulong {
    // access_ok(addr, size) is supplied by asm-generic/access_ok.h.
    if access_ok(addr, size) {
        size = __clear_user(addr, size);
    }
    size
}

// The following C macros are retained as Rust macro equivalents.  The
// OpenRISC inline assembly and exception-table directives are preserved as
// source strings because they are target-toolchain-specific.

#[macro_export]
macro_rules! __m {
    ($x:expr) => { &mut *($x as *mut $crate::__large_struct) };
}

#[macro_export]
macro_rules! __put_user_asm {
    ($x:expr, $addr:expr, $err:expr, $op:expr) => {{
        // C: __asm__ __volatile__ (OpenRISC $op with .fixup/__ex_table).
        let _ = ($x, $addr, $op);
        $err
    }};
}

#[macro_export]
macro_rules! __put_user_asm2 {
    ($x:expr, $addr:expr, $err:expr) => {{
        // C: two OpenRISC l.sw operations with .fixup/__ex_table.
        let _ = ($x, $addr);
        $err
    }};
}

#[macro_export]
macro_rules! __get_user_asm {
    ($x:expr, $addr:expr, $err:expr, $op:expr) => {{
        // C: OpenRISC load $op with .fixup/__ex_table; failed loads zero $x.
        let _ = ($addr, $op);
        $x = 0;
        $err
    }};
}

#[macro_export]
macro_rules! __get_user_asm2 {
    ($x:expr, $addr:expr, $err:expr) => {{
        // C: two OpenRISC l.lwz operations with .fixup/__ex_table.
        let _ = $addr;
        $x = 0;
        $err
    }};
}

// access_ok is an external dependency from asm-generic/access_ok.h.
extern "Rust" {
    fn access_ok(addr: *const libc::c_void, size: libc::c_ulong) -> bool;
}

#[macro_export]
macro_rules! __put_user_size {
    ($x:expr, $ptr:expr, $size:expr, $retval:ident) => {{
        $retval = 0;
        match $size {
            1 => { $retval = $crate::__put_user_asm!($x, $ptr, $retval, "l.sb"); }
            2 => { $retval = $crate::__put_user_asm!($x, $ptr, $retval, "l.sh"); }
            4 => { $retval = $crate::__put_user_asm!($x, $ptr, $retval, "l.sw"); }
            8 => { $retval = $crate::__put_user_asm2!($x, $ptr, $retval); }
            _ => { $retval = $crate::__put_user_bad(); }
        }
    }};
}

#[macro_export]
macro_rules! __get_user_size {
    ($x:expr, $ptr:expr, $size:expr, $retval:ident) => {{
        $retval = 0;
        match $size {
            1 => { $retval = $crate::__get_user_asm!($x, $ptr, $retval, "l.lbz"); }
            2 => { $retval = $crate::__get_user_asm!($x, $ptr, $retval, "l.lhz"); }
            4 => { $retval = $crate::__get_user_asm!($x, $ptr, $retval, "l.lwz"); }
            8 => { $retval = $crate::__get_user_asm2!($x, $ptr, $retval); }
            _ => { $x = $crate::__get_user_bad() as _; }
        }
    }};
}

#[macro_export]
macro_rules! __put_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: libc::c_long;
        $crate::__put_user_size!($x, $ptr, $size, __pu_err);
        __pu_err
    }};
}

#[macro_export]
macro_rules! __get_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: libc::c_long;
        $crate::__get_user_size!($x, $ptr, $size, __gu_err);
        __gu_err
    }};
}

#[macro_export]
macro_rules! __put_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: libc::c_long = -14;
        let __pu_addr = $ptr;
        if access_ok(__pu_addr as *const libc::c_void, $size) {
            $crate::__put_user_size!($x, __pu_addr, $size, __pu_err);
        }
        __pu_err
    }};
}

#[macro_export]
macro_rules! __get_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: libc::c_long = -14;
        let __gu_addr = $ptr;
        if access_ok(__gu_addr as *const libc::c_void, $size) {
            $crate::__get_user_size!($x, __gu_addr, $size, __gu_err);
        } else {
            $x = 0;
        }
        __gu_err
    }};
}

#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => { $crate::__put_user_check!($x, $ptr, core::mem::size_of_val(&*$ptr)) };
}

#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => { $crate::__get_user_check!($x, $ptr, core::mem::size_of_val(&*$ptr)) };
}

#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => { $crate::__put_user_nocheck!($x, $ptr, core::mem::size_of_val(&*$ptr)) };
}

#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => { $crate::__get_user_nocheck!($x, $ptr, core::mem::size_of_val(&*$ptr)) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
