/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_FUTEX_H
// When CONFIG_SMP is not enabled, this header includes asm-generic/futex.h.
// The declarations below correspond to the CONFIG_SMP implementation.

use core::sync::atomic::{fence, AtomicU32, Ordering};

// Supplied by the kernel's uaccess and futex headers.
extern "C" {
    fn access_ok(addr: *const core::ffi::c_void, size: usize) -> bool;
}

// Supplied by linux/errno.h and linux/futex.h.
const EFAULT: i32 = 14;
const ENOSYS: i32 = 38;
const FUTEX_OP_SET: i32 = 0;
const FUTEX_OP_ADD: i32 = 1;
const FUTEX_OP_OR: i32 = 2;
const FUTEX_OP_ANDN: i32 = 3;
const FUTEX_OP_XOR: i32 = 4;

#[inline]
unsafe fn __futex_atomic_op<F>(ret: &mut i32, oldval: &mut u32, uaddr: *mut u32, operation: F)
where
    F: Fn(u32) -> u32,
{
    // Corresponds to __atomic_pre_full_fence().
    fence(Ordering::SeqCst);

    // The C implementation uses C-SKY ldex.w/stex.w with exception-table
    // recovery to -EFAULT.  This CAS loop preserves the same atomic update,
    // retry, and ordering semantics for the translated interface.
    let atomic = &*(uaddr as *const AtomicU32);
    let mut observed = atomic.load(Ordering::Acquire);
    loop {
        *oldval = observed;
        let newval = operation(observed);
        match atomic.compare_exchange_weak(
            observed,
            newval,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => break,
            Err(value) => observed = value,
        }
    }

    *ret = 0;
    // Corresponds to __atomic_post_full_fence().
    fence(Ordering::SeqCst);
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

    if !access_ok(uaddr as *const core::ffi::c_void, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    match op {
        FUTEX_OP_SET => __futex_atomic_op(&mut ret, &mut oldval, uaddr, |_| oparg as u32),
        FUTEX_OP_ADD => __futex_atomic_op(&mut ret, &mut oldval, uaddr, |v| {
            v.wrapping_add(oparg as u32)
        }),
        FUTEX_OP_OR => __futex_atomic_op(&mut ret, &mut oldval, uaddr, |v| v | oparg as u32),
        FUTEX_OP_ANDN => __futex_atomic_op(&mut ret, &mut oldval, uaddr, |v| {
            v & !(oparg as u32)
        }),
        FUTEX_OP_XOR => __futex_atomic_op(&mut ret, &mut oldval, uaddr, |v| v ^ oparg as u32),
        _ => ret = -ENOSYS,
    }

    if ret == 0 {
        core::ptr::write(oval, oldval as i32);
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

    if !access_ok(uaddr as *const core::ffi::c_void, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    fence(Ordering::SeqCst);
    let atomic = &*(uaddr as *const AtomicU32);
    let observed = atomic.load(Ordering::Acquire);
    if observed == oldval {
        let _ = atomic.compare_exchange(oldval, newval, Ordering::AcqRel, Ordering::Acquire);
    }
    val = observed;
    fence(Ordering::SeqCst);

    core::ptr::write(uval, val);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
