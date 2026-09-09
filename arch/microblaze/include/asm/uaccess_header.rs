/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Translated from the MicroBlaze Linux uaccess header.
// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const __FIXUP_SECTION: &str = ".section .fixup,\"ax\"\n";
pub const __EX_TABLE_SECTION: &str = ".section __ex_table,\"a\"\n";

extern "C" {
    pub fn __copy_tofrom_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        size: usize,
    ) -> usize;
    pub fn __user_bad() -> isize;
}

/// Return: number of not copied bytes, i.e. 0 if OK or non-zero if fail.
#[inline]
pub unsafe fn __clear_user(mut to: *mut core::ffi::c_void, mut n: usize) -> usize {
    // Normal memset with two words to __ex_table.
    // The original is MicroBlaze inline assembly; preserve its exact template here.
    core::arch::asm!(
        "1: sb r0, {to}, r0;",
        "   addik {n}, {n}, -1;",
        "   bneid {n}, 1b;",
        "   addik {to}, {to}, 1;",
        "2:",
        ".section __ex_table,\"a\"",
        ".word 1b,2b;",
        ".previous;",
        to = inout(reg) to,
        n = inout(reg) n,
        options(nostack),
    );
    n
}

#[inline]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    // might_fault();
    // access_ok(to, n) is provided by the surrounding kernel translation.
    if !access_ok(to, n) {
        return n;
    }
    __clear_user(to, n)
}

#[macro_export]
macro_rules! __get_user_asm {
    ($insn:literal, $ptr:expr, $val:expr, $err:expr) => {{
        let mut __gu_err: isize;
        unsafe {
            core::arch::asm!(
                concat!("1:", $insn, " {val}, {ptr}, r0;"),
                "addk {err}, r0, r0;",
                "2:",
                ".section .fixup,\"ax\"",
                "3: brid 2b;",
                "   addik {err}, r0, {fault};",
                ".previous;",
                ".section __ex_table,\"a\"",
                ".word 1b,3b;",
                ".previous;",
                err = lateout(reg) __gu_err,
                val = lateout(reg) $val,
                ptr = in(reg) $ptr,
                fault = const -(14i32),
                options(nostack),
            );
        }
        $err = __gu_err;
        __gu_err
    }};
}

/// Get a simple variable from user space.
#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        let __gu_ptr = $ptr;
        if access_ok(__gu_ptr, core::mem::size_of_val(&*$ptr)) {
            __get_user!($x, __gu_ptr)
        } else {
            -(14isize)
        }
    }};
}

#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {{
        let mut __gu_err: isize = 0;
        match core::mem::size_of_val(&*$ptr) {
            1 => { __get_user_asm!("lbu", $ptr, $x, __gu_err); }
            2 => { __get_user_asm!("lhu", $ptr, $x, __gu_err); }
            4 => { __get_user_asm!("lw", $ptr, $x, __gu_err); }
            8 => {
                let mut __x: u64 = 0;
                __gu_err = if raw_copy_from_user(&mut __x as *mut _ as *mut _, $ptr, 8) != 0 { -(14isize) } else { 0 };
                $x = __x as _;
            }
            _ => { __gu_err = unsafe { __user_bad() }; }
        }
        __gu_err
    }};
}

#[macro_export]
macro_rules! __put_user_asm {
    ($insn:literal, $ptr:expr, $val:expr, $err:expr) => {{
        let mut __pu_err: isize;
        unsafe {
            core::arch::asm!(
                concat!("1:", $insn, " {val}, {ptr}, r0;"),
                "addk {err}, r0, r0;",
                "2:", ".section .fixup,\"ax\"", "3: brid 2b;",
                "addik {err}, r0, {fault};", ".previous;",
                ".section __ex_table,\"a\"", ".word 1b,3b;", ".previous;",
                err = lateout(reg) __pu_err, val = in(reg) $val,
                ptr = in(reg) $ptr, fault = const -(14i32), options(nostack),
            );
        }
        $err = __pu_err;
        __pu_err
    }};
}

#[macro_export]
macro_rules! __put_user_asm_8 {
    ($ptr:expr, $val:expr, $err:expr) => {{
        let mut __pu_err: isize;
        unsafe {
            core::arch::asm!(
                "lwi {err}, {val}, 0;", "1: swi {err}, {ptr}, 0;",
                "lwi {err}, {val}, 4;", "2: swi {err}, {ptr}, 4;",
                "addk {err}, r0, r0;", "3:",
                ".section .fixup,\"ax\"", "4: brid 3b;",
                "addik {err}, r0, {fault};", ".previous;",
                ".section __ex_table,\"a\"", ".word 1b,4b,2b,4b;", ".previous;",
                err = lateout(reg) __pu_err, val = in(reg) &$val,
                ptr = in(reg) $ptr, fault = const -(14i32), options(nostack),
            );
        }
        $err = __pu_err;
        __pu_err
    }};
}

#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => { __put_user_check!($x, $ptr, core::mem::size_of_val(&*$ptr)) };
}

#[macro_export]
macro_rules! __put_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_val = $x;
        let __pu_addr = $ptr;
        let mut __pu_err: isize = 0;
        if access_ok(__pu_addr, $size) {
            match $size {
                1 => { __put_user_asm!("sb", __pu_addr, __pu_val, __pu_err); }
                2 => { __put_user_asm!("sh", __pu_addr, __pu_val, __pu_err); }
                4 => { __put_user_asm!("sw", __pu_addr, __pu_val, __pu_err); }
                8 => { __put_user_asm_8!(__pu_addr, __pu_val, __pu_err); }
                _ => { __pu_err = unsafe { __user_bad() }; }
            }
        } else { __pu_err = -(14isize); }
        __pu_err
    }};
}

#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {{
        let __gu_val = $x;
        let mut __gu_err: isize = 0;
        match core::mem::size_of_val(&__gu_val) {
            1 => { __put_user_asm!("sb", $ptr, __gu_val, __gu_err); }
            2 => { __put_user_asm!("sh", $ptr, __gu_val, __gu_err); }
            4 => { __put_user_asm!("sw", $ptr, __gu_val, __gu_err); }
            8 => { __put_user_asm_8!($ptr, __gu_val, __gu_err); }
            _ => { __gu_err = unsafe { __user_bad() }; }
        }
        __gu_err
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
