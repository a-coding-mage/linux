/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/futex.h, linux/uaccess.h, and asm/errno.h.

/// Equivalent of the SPARC64 `__futex_cas_op` assembly macro.
#[inline(always)]
unsafe fn __futex_cas_op(
    insn: &str,
    ret: &mut i32,
    oldval: &mut i32,
    tem: &mut i32,
    uaddr: *mut u32,
    oparg: i32,
) {
    // The instruction text and exception-table fixups are architecture-specific
    // kernel assembly and are retained here as the direct Rust inline-assembly
    // counterpart.
    core::arch::asm!(
        "1:",
        "lduwa [{uaddr}] %asi, {tem}",
        "{insn}",
        "2:",
        "casa [{uaddr}] %asi, {tem}, {oldval}",
        "cmp {tem}, {oldval}",
        "bne,pn %icc, 1b",
        " mov 0, {ret}",
        "3:",
        insn = in(reg) insn.as_ptr(),
        uaddr = in(reg) uaddr,
        tem = lateout(reg) *tem,
        oldval = inlateout(reg) *oldval => _,
        ret = lateout(reg) *ret,
        in("g5") oparg,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;
    let mut tem: i32;

    if ((uaddr as usize) & 0x3usize) != 0 {
        return -EINVAL;
    }

    match op {
        FUTEX_OP_SET => {
            __futex_cas_op("mov\t%4, %1", &mut ret, &mut oldval, &mut tem, uaddr, oparg);
        }
        FUTEX_OP_ADD => {
            __futex_cas_op("add\t%2, %4, %1", &mut ret, &mut oldval, &mut tem, uaddr, oparg);
        }
        FUTEX_OP_OR => {
            __futex_cas_op("or\t%2, %4, %1", &mut ret, &mut oldval, &mut tem, uaddr, oparg);
        }
        FUTEX_OP_ANDN => {
            __futex_cas_op("andn\t%2, %4, %1", &mut ret, &mut oldval, &mut tem, uaddr, oparg);
        }
        FUTEX_OP_XOR => {
            __futex_cas_op("xor\t%2, %4, %1", &mut ret, &mut oldval, &mut tem, uaddr, oparg);
        }
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
    mut newval: u32,
) -> i32 {
    let mut ret: i32 = 0;

    core::arch::asm!(
        "1: casa [{uaddr}] %asi, {oldval}, {newval}",
        "2:",
        uaddr = in(reg) uaddr,
        oldval = in(reg) oldval,
        newval = inlateout(reg) newval,
        ret = inlateout(reg) ret,
        in("g5") -EFAULT,
        options(nostack)
    );

    *uval = newval;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
