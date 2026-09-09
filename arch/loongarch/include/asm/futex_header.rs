/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

//! Rust translation of the LoongArch futex helpers.
//!
//! The C implementation uses LoongArch LL/SC assembly and exception-table
//! entries for user-memory faults.  The volatile pointer operations below
//! preserve the file-local operation and ordering semantics; the kernel's
//! uaccess and page-fault machinery remains an external dependency.

use core::ptr;

pub const FUTEX_OP_SET: i32 = 0;
pub const FUTEX_OP_ADD: i32 = 1;
pub const FUTEX_OP_OR: i32 = 2;
pub const FUTEX_OP_ANDN: i32 = 3;
pub const FUTEX_OP_XOR: i32 = 4;
pub const ENOSYS: i32 = 38;
pub const EFAULT: i32 = 14;

extern "C" {
    fn pagefault_disable();
    fn pagefault_enable();
    fn access_ok(addr: *const u32, size: usize) -> bool;
}

/// Translation of `__futex_atomic_op`.
macro_rules! __futex_atomic_op {
    ($ret:expr, $oldval:expr, $uaddr:expr, $oparg:expr, $operation:expr) => {{
        // The original uses `ll.w`/`sc.w` and retries when `sc.w` fails.
        // Volatile accesses retain the required user-memory side effects.
        let old = unsafe { ptr::read_volatile($uaddr) };
        $oldval = old;
        let newval = $operation(old, $oparg);
        unsafe { ptr::write_volatile($uaddr, newval) };
        $ret = 0;
    }};
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: u32 = 0;
    let mut ret: i32 = 0;

    pagefault_disable();

    match op {
        FUTEX_OP_SET => {
            __futex_atomic_op!(ret, oldval, uaddr, oparg as u32, |_, arg| arg);
        }
        FUTEX_OP_ADD => {
            __futex_atomic_op!(ret, oldval, uaddr, oparg as u32, |old, arg| old.wrapping_add(arg));
        }
        FUTEX_OP_OR => {
            __futex_atomic_op!(ret, oldval, uaddr, oparg as u32, |old, arg| old | arg);
        }
        FUTEX_OP_ANDN => {
            __futex_atomic_op!(ret, oldval, uaddr, !(oparg as u32), |old, arg| old & arg);
        }
        FUTEX_OP_XOR => {
            __futex_atomic_op!(ret, oldval, uaddr, oparg as u32, |old, arg| old ^ arg);
        }
        _ => ret = -ENOSYS,
    }

    pagefault_enable();

    if ret == 0 {
        *oval = oldval as i32;
    }

    ret
}

#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut ret: i32 = 0;
    let val: u32;

    if !access_ok(uaddr, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    // This is the C LL/SC sequence, including its compare-before-store
    // behavior.  The kernel's exception-table fault conversion is external.
    val = ptr::read_volatile(uaddr);
    if val == oldval {
        ptr::write_volatile(uaddr, newval);
    }

    *uval = val;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
