/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2006  Ralf Baechle (ralf@linux-mips.org)
 * Copyright (c) 2018  Jim Wilson (jimw@sifive.com)
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/futex.h, linux/uaccess.h, linux/errno.h, asm/asm.h,
// and asm/asm-extable.h.

/* We don't even really need the extable code, but for now keep it simple */
// When CONFIG_MMU is not enabled, these operations are empty.
#[cfg(not(CONFIG_MMU))]
#[inline(always)]
unsafe fn __enable_user_access() {}

#[cfg(not(CONFIG_MMU))]
#[inline(always)]
unsafe fn __disable_user_access() {}

// C macro __futex_atomic_op(insn, ret, oldval, uaddr, oparg).
// The instruction text and exception-table operands are retained literally;
// the surrounding translation supplies the target-specific assembly helpers.
macro_rules! __futex_atomic_op {
    ($insn:expr, $ret:expr, $oldval:expr, $uaddr:expr, $oparg:expr) => {{
        unsafe { __enable_user_access(); }
        unsafe {
            core::arch::asm!(
                "1: {insn}\n2:",
                insn = const $insn,
                inout("r") $ret,
                lateout("r") $oldval,
                inout("r") $uaddr,
                in("r") $oparg,
                options(nostack)
            );
        }
        unsafe { __disable_user_access(); }
    }};
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32 = 0;

    if !access_ok(uaddr as *const core::ffi::c_void, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    match op {
        FUTEX_OP_SET => __futex_atomic_op!(
            "amoswap.w.aqrl %[ov],%z[op],%[u]", ret, oldval, uaddr, oparg
        ),
        FUTEX_OP_ADD => __futex_atomic_op!(
            "amoadd.w.aqrl %[ov],%z[op],%[u]", ret, oldval, uaddr, oparg
        ),
        FUTEX_OP_OR => __futex_atomic_op!(
            "amoor.w.aqrl %[ov],%z[op],%[u]", ret, oldval, uaddr, oparg
        ),
        FUTEX_OP_ANDN => __futex_atomic_op!(
            "amoand.w.aqrl %[ov],%z[op],%[u]", ret, oldval, uaddr, !oparg
        ),
        FUTEX_OP_XOR => __futex_atomic_op!(
            "amoxor.w.aqrl %[ov],%z[op],%[u]", ret, oldval, uaddr, oparg
        ),
        _ => ret = -ENOSYS,
    }

    if ret == 0 {
        *oval = oldval;
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
    let mut val: u32 = 0;
    let mut tmp: usize;

    if !access_ok(uaddr as *const core::ffi::c_void, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    unsafe { __enable_user_access(); }
    unsafe {
        core::arch::asm!(
            "1: lr.w {v}, {u}\n\tbne {v}, {ov}, 3f\n2: sc.w.aqrl {t}, {nv}, {u}\n\tbnez {t}, 1b\n3:",
            v = lateout(reg) val,
            u = inout(reg) uaddr,
            ov = in(reg) oldval as i32,
            nv = in(reg) newval,
            t = lateout(reg) tmp,
            inout(reg) ret,
            options(nostack)
        );
    }
    unsafe { __disable_user_access(); }

    *uval = val;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
