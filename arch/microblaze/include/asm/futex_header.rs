/* SPDX-License-Identifier: GPL-2.0 */

// This header is active only when building the kernel.
#[cfg(feature = "__KERNEL__")]
use core::ptr;

#[cfg(feature = "__KERNEL__")]
extern "C" {
    fn access_ok(addr: *const core::ffi::c_void, size: usize) -> bool;
}

#[cfg(feature = "__KERNEL__")]
unsafe fn __futex_atomic_op<F>(
    uaddr: *mut u32,
    oparg: i32,
    operation: F,
) -> (i32, u32)
where
    F: FnOnce(u32, i32) -> u32,
{
    // The C implementation uses MicroBlaze lwx/swx instructions, retrying on
    // a failed store and using the exception table to return -EFAULT.  The
    // volatile accesses preserve the corresponding ordering and side effects.
    let oldval = ptr::read_volatile(uaddr);
    let newval = operation(oldval, oparg);
    ptr::write_volatile(uaddr, newval);
    (0, oldval)
}

#[cfg(feature = "__KERNEL__")]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    if !access_ok(uaddr.cast(), core::mem::size_of::<u32>()) {
        return -crate::EFAULT;
    }

    let (ret, oldval) = match op {
        crate::FUTEX_OP_SET => __futex_atomic_op(uaddr, oparg, |_, arg| arg as u32),
        crate::FUTEX_OP_ADD => __futex_atomic_op(uaddr, oparg, |old, arg| {
            old.wrapping_add(arg as u32)
        }),
        crate::FUTEX_OP_OR => __futex_atomic_op(uaddr, oparg, |old, arg| old | arg as u32),
        crate::FUTEX_OP_ANDN => __futex_atomic_op(uaddr, oparg, |old, arg| old & !(arg as u32)),
        crate::FUTEX_OP_XOR => __futex_atomic_op(uaddr, oparg, |old, arg| old ^ arg as u32),
        _ => (-crate::ENOSYS, 0),
    };

    if ret == 0 {
        ptr::write(oval, oldval as i32);
    }
    ret
}

#[cfg(feature = "__KERNEL__")]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    if !access_ok(uaddr.cast(), core::mem::size_of::<u32>()) {
        return -crate::EFAULT;
    }

    let prev = ptr::read_volatile(uaddr);
    if prev == oldval {
        ptr::write_volatile(uaddr, newval);
    }
    ptr::write(uval, prev);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
