/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm-generic/futex.h. */

/* Dependencies supplied by linux/futex.h, linux/uaccess.h, and asm/errno.h. */

/*
 * The following aliases apply only when CONFIG_SMP is not enabled and
 * futex_atomic_cmpxchg_inatomic has not already been defined.  They rely on
 * preempt_disable() ensuring mutual exclusion.
 */
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    futex_atomic_cmpxchg_inatomic_local(uval, uaddr, oldval, newval)
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: u32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    futex_atomic_op_inuser_local(op, oparg, oval, uaddr)
}

/**
 * futex_atomic_op_inuser_local() - Atomic arithmetic operation with constant
 * argument and comparison of the previous futex value with another constant.
 *
 * Return:
 * 0 - On success
 * -EFAULT - User access resulted in a page fault
 * -EAGAIN - Atomic operation was unable to complete due to contention
 * -ENOSYS - Operation not supported
 */
#[inline]
pub unsafe fn futex_atomic_op_inuser_local(
    op: i32,
    oparg: u32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;
    let mut tmp: u32;

    preempt_disable();

    ret = -EFAULT;
    if unlikely(get_user(&mut oldval, uaddr) != 0) {
        preempt_enable();
        if ret == 0 {
            *oval = oldval;
        }
        return ret;
    }

    ret = 0;
    tmp = oldval as u32;

    match op {
        FUTEX_OP_SET => tmp = oparg,
        FUTEX_OP_ADD => tmp = tmp.wrapping_add(oparg),
        FUTEX_OP_OR => tmp |= oparg,
        FUTEX_OP_ANDN => tmp &= !oparg,
        FUTEX_OP_XOR => tmp ^= oparg,
        _ => ret = -ENOSYS,
    }

    if ret == 0 && unlikely(put_user(tmp, uaddr) != 0) {
        ret = -EFAULT;
    }

    preempt_enable();

    if ret == 0 {
        *oval = oldval;
    }

    ret
}

/**
 * futex_atomic_cmpxchg_inatomic_local() - Compare and exchange the content of
 * uaddr with newval if the current value is oldval.
 *
 * Return:
 * 0 - On success
 * -EFAULT - User access resulted in a page fault
 * -EAGAIN - Atomic operation was unable to complete due to contention
 */
#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic_local(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut val: u32 = 0;

    preempt_disable();
    if unlikely(get_user(&mut val, uaddr) != 0) {
        preempt_enable();
        return -EFAULT;
    }

    if val == oldval && unlikely(put_user(newval, uaddr) != 0) {
        preempt_enable();
        return -EFAULT;
    }

    *uval = val;
    preempt_enable();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
