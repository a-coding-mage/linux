/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Translated from asm/hexagon/futex.h.
 *
 * The original header is enabled only for the kernel build and depends on
 * linux/futex.h, linux/uaccess.h, and asm/errno.h.
 */

/* XXX TODO-- need to add sync barriers! */

/*
 * Original __futex_atomic_op inline Hexagon assembly:
 *
 * 1: oldval = memw_locked(uaddr)
 *    perform the requested operation
 * 2: memw_locked(uaddr,p2) = ret
 *    if (!p2) jump 1b
 *    ret = 0
 * 3:
 * .section .fixup,"ax"
 * 4: ret = -EFAULT
 *    jump 3b
 * .section __ex_table,"a"
 * .long 1b,4b,2b,4b
 *
 * The Hexagon instruction-template parameter is represented by the Rust
 * operation performed by arch_futex_atomic_op_inuser below.
 */

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let ret: i32;

    if !access_ok(uaddr as *const u32, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    match op {
        FUTEX_OP_SET => {
            oldval = core::ptr::read_volatile(uaddr) as i32;
            core::ptr::write_volatile(uaddr, oparg as u32);
            ret = 0;
        }
        FUTEX_OP_ADD => {
            oldval = core::ptr::read_volatile(uaddr) as i32;
            core::ptr::write_volatile(uaddr, oldval.wrapping_add(oparg) as u32);
            ret = 0;
        }
        FUTEX_OP_OR => {
            oldval = core::ptr::read_volatile(uaddr) as i32;
            core::ptr::write_volatile(uaddr, (oldval | oparg) as u32);
            ret = 0;
        }
        FUTEX_OP_ANDN => {
            oldval = core::ptr::read_volatile(uaddr) as i32;
            core::ptr::write_volatile(uaddr, (oldval & !oparg) as u32);
            ret = 0;
        }
        FUTEX_OP_XOR => {
            oldval = core::ptr::read_volatile(uaddr) as i32;
            core::ptr::write_volatile(uaddr, (oldval ^ oparg) as u32);
            ret = 0;
        }
        _ => ret = -ENOSYS,
    }

    if ret == 0 {
        *oval = oldval;
    }

    ret
}

/* Original operation:
 * 1: prev = memw_locked(uaddr)
 *    p2 = (prev == oldval)
 *    if (!p2) jump 3f
 * 2: memw_locked(uaddr,p2) = newval
 *    if (!p2) jump 1b
 * 3: return
 * A fault handler stores -EFAULT in ret.
 */
#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let prev: u32;
    let ret: i32;

    if !access_ok(uaddr as *const u32, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    prev = core::ptr::read_volatile(uaddr);
    if prev == oldval {
        core::ptr::write_volatile(uaddr, newval);
    }
    ret = 0;

    *uval = prev;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
