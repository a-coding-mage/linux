/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux futex, uaccess, errno, and
// configuration-specific architecture headers are intentionally external.
// The C header selects futex-irq, futex-cas, or futex-llsc according to the
// build configuration; SMP configurations without a supported CPU are an
// error.

#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    if !access_ok(uaddr, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    atomic_futex_op_cmpxchg_inatomic(uval, uaddr, oldval, newval)
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: u32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: u32 = 0;
    let mut newval: u32 = 0;
    let mut prev: u32 = 0;
    let mut ret: i32;

    loop {
        ret = get_user(&mut oldval, uaddr);

        if ret != 0 {
            break;
        }

        match op {
            FUTEX_OP_SET => {
                newval = oparg;
            }
            FUTEX_OP_ADD => {
                newval = oldval.wrapping_add(oparg);
            }
            FUTEX_OP_OR => {
                newval = oldval | oparg;
            }
            FUTEX_OP_ANDN => {
                newval = oldval & !oparg;
            }
            FUTEX_OP_XOR => {
                newval = oldval ^ oparg;
            }
            _ => {
                ret = -ENOSYS;
            }
        }

        if ret != 0 {
            break;
        }

        ret = futex_atomic_cmpxchg_inatomic(&mut prev, uaddr, oldval, newval);
        if ret != 0 || prev == oldval {
            break;
        }
    }

    if ret == 0 {
        *oval = oldval as i32;
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
