/*
 * User space memory access functions for Nios II
 *
 * Copyright (C) 2010-2011, Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009, Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies: linux/string.h, asm/page.h, asm/extable.h,
// asm-generic/access_ok.h

pub const __EX_TABLE_SECTION: &str = ".section __ex_table,\"a\"\n";

/* Zero Userspace */

#[inline]
pub unsafe fn __clear_user(mut to: *mut core::ffi::c_void, mut n: usize) -> usize {
    // Original implementation is Nios II inline assembly with an exception-table entry.
    core::arch::asm!(
        "1: stb zero, 0({to})",
        "addi {n}, {n}, -1",
        "addi {to}, {to}, 1",
        "bne {n}, zero, 1b",
        "2:",
        ".section __ex_table,\"a\"",
        ".word 1b, 2b",
        ".previous",
        to = inout(reg) to,
        n = inout(reg) n,
        options(nostack)
    );
    n
}

#[inline]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    if !access_ok(to, n) { n } else { __clear_user(to, n) }
}

unsafe extern "C" {
    pub fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn strncpy_from_user(to: *mut i8, from: *const i8, len: isize) -> isize;
    pub fn strnlen_user(s: *const i8, n: isize) -> isize;
    pub fn __get_user_unknown();
}

pub const INLINE_COPY_USER: bool = true;

#[inline(always)]
pub unsafe fn __get_user_asm<T>(val: &mut T, insn: &str, addr: *const T, err: &mut isize) {
    let mut gu_val: usize;
    core::arch::asm!(
        "movi {err}, {fault}",
        "1:",
        "{insn} {value}, 0({addr})",
        "movi {err}, 0",
        "2:",
        ".section __ex_table,\"a\"",
        ".word 1b, 2b",
        ".previous",
        err = inout(reg) *err,
        value = lateout(reg) gu_val,
        addr = in(reg) addr,
        fault = const -14isize,
        insn = const insn,
        options(nostack)
    );
    core::ptr::write(val, gu_val as T);
}

#[inline(always)]
pub unsafe fn __get_user_8<T>(val: &mut T, ptr: *const T, err: &mut isize) {
    let mut value: u64 = 0;
    *err = 0;
    if raw_copy_from_user((&mut value as *mut u64).cast(), ptr.cast(), core::mem::size_of::<T>()) != 0 {
        *err = -14;
    } else {
        core::ptr::write(val, value as T);
    }
}

#[inline(always)]
pub unsafe fn __get_user_common<T>(val: &mut T, size: usize, ptr: *const T, err: &mut isize) {
    match size {
        1 => __get_user_asm(val, "ldbu", ptr, err),
        2 => __get_user_asm(val, "ldhu", ptr, err),
        4 => __get_user_asm(val, "ldw", ptr, err),
        8 => __get_user_8(val, ptr, err),
        _ => __get_user_unknown(),
    }
}

#[inline(always)]
pub unsafe fn __put_user_asm<T>(val: T, insn: &str, ptr: *mut T, err: &mut isize) {
    core::arch::asm!(
        "movi {err}, {fault}",
        "1:",
        "{insn} {val}, 0({ptr})",
        "movi {err}, 0",
        "2:",
        ".section __ex_table,\"a\"",
        ".word 1b, 2b",
        ".previous",
        err = inout(reg) *err,
        val = in(reg) val,
        ptr = in(reg) ptr,
        fault = const -14isize,
        insn = const insn,
        options(nostack)
    );
}

#[inline(always)]
pub unsafe fn __put_user_common<T: Copy>(val: T, ptr: *mut T) -> isize {
    let mut err: isize = -14;
    match core::mem::size_of::<T>() {
        1 => __put_user_asm(val, "stb", ptr, &mut err),
        2 => __put_user_asm(val, "sth", ptr, &mut err),
        4 => __put_user_asm(val, "stw", ptr, &mut err),
        _ => {
            /* XXX: This looks wrong... */
            err = 0;
            if __copy_to_user(ptr.cast(), (&val as *const T).cast(), core::mem::size_of::<T>()) != 0 { err = -14; }
        }
    }
    err
}

// Supplied by asm-generic/access_ok.h and the user-copy implementation.
extern "Rust" {
    pub fn access_ok<T>(ptr: *const T, size: usize) -> bool;
    pub fn __copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
