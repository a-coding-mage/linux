/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Vineetg: August 2010: From Android kernel work
 */

// C includes and header guards are intentionally omitted; their symbols are
// supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_ARC_HAS_LLSC")]
macro_rules! __futex_atomic_op {
    ($insn:expr, $ret:ident, $oldval:ident, $uaddr:ident, $oparg:ident) => {{
        unsafe { smp_mb(); }
        unsafe {
            core::arch::asm!(
                "1: llock {oldval}, [{uaddr}]",
                $insn,
                "2: scond {ret}, [{uaddr}]",
                "bnz 1b",
                "mov {ret}, 0",
                "3:",
                ".section .fixup,\"ax\"",
                ".align 4",
                "4: mov {ret}, {fault}",
                "j 3b",
                ".previous",
                ".section __ex_table,\"a\"",
                ".align 4",
                ".word 1b, 4b",
                ".word 2b, 4b",
                ".previous",
                oldval = out(reg) $oldval,
                ret = out(reg) $ret,
                uaddr = in(reg) $uaddr,
                oparg = in(reg) $oparg,
                fault = const -EFAULT,
                options(preserves_flags),
            );
        }
        unsafe { smp_mb(); }
    }};
}

#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
macro_rules! __futex_atomic_op {
    ($insn:expr, $ret:ident, $oldval:ident, $uaddr:ident, $oparg:ident) => {{
        unsafe { smp_mb(); }
        unsafe {
            core::arch::asm!(
                "1: ld {oldval}, [{uaddr}]",
                $insn,
                "2: st {ret}, [{uaddr}]",
                "mov {ret}, 0",
                "3:",
                ".section .fixup,\"ax\"",
                ".align 4",
                "4: mov {ret}, {fault}",
                "j 3b",
                ".previous",
                ".section __ex_table,\"a\"",
                ".align 4",
                ".word 1b, 4b",
                ".word 2b, 4b",
                ".previous",
                oldval = out(reg) $oldval,
                ret = out(reg) $ret,
                uaddr = in(reg) $uaddr,
                oparg = in(reg) $oparg,
                fault = const -EFAULT,
                options(preserves_flags),
            );
        }
        unsafe { smp_mb(); }
    }};
}

pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;

    if !access_ok(uaddr as *const _, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    #[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
    preempt_disable(); // to guarantee atomic r-m-w of futex op

    match op {
        FUTEX_OP_SET => __futex_atomic_op!("mov {ret}, {oparg}", ret, oldval, uaddr, oparg),
        // oldval = *uaddr; *uaddr += oparg; ret = *uaddr
        FUTEX_OP_ADD => __futex_atomic_op!("add {ret}, {oldval}, {oparg}", ret, oldval, uaddr, oparg),
        FUTEX_OP_OR => __futex_atomic_op!("or {ret}, {oldval}, {oparg}", ret, oldval, uaddr, oparg),
        FUTEX_OP_ANDN => __futex_atomic_op!("bic {ret}, {oldval}, {oparg}", ret, oldval, uaddr, oparg),
        FUTEX_OP_XOR => __futex_atomic_op!("xor {ret}, {oldval}, {oparg}", ret, oldval, uaddr, oparg),
        _ => ret = -ENOSYS,
    }

    #[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
    preempt_enable();

    if ret == 0 {
        *oval = oldval;
    }
    ret
}

/*
 * cmpxchg of futex (pagefaults disabled by caller)
 * Return 0 for success, -EFAULT otherwise
 */
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    expval: u32,
    newval: u32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut existval: u32;

    if !access_ok(uaddr as *const _, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    #[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
    preempt_disable(); // to guarantee atomic r-m-w of futex op
    smp_mb();

    #[cfg(feature = "CONFIG_ARC_HAS_LLSC")]
    core::arch::asm!(
        "1: llock {existval}, [{uaddr}]",
        "brne {existval}, {expval}, 3f",
        "2: scond {newval}, [{uaddr}]",
        "bnz 1b",
        "3:",
        ".section .fixup,\"ax\"", "4: mov {ret}, {fault}", "j 3b", ".previous",
        ".section __ex_table,\"a\"", ".align 4", ".word 1b, 4b", ".word 2b, 4b", ".previous",
        ret = inout(reg) ret, existval = out(reg) existval,
        expval = in(reg) expval, newval = in(reg) newval, uaddr = in(reg) uaddr,
        fault = const -EFAULT,
    );

    #[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
    core::arch::asm!(
        "1: ld {existval}, [{uaddr}]", "brne {existval}, {expval}, 3f", "2: st {newval}, [{uaddr}]", "3:",
        ".section .fixup,\"ax\"", "4: mov {ret}, {fault}", "j 3b", ".previous",
        ".section __ex_table,\"a\"", ".align 4", ".word 1b, 4b", ".word 2b, 4b", ".previous",
        ret = inout(reg) ret, existval = out(reg) existval,
        expval = in(reg) expval, newval = in(reg) newval, uaddr = in(reg) uaddr,
        fault = const -EFAULT,
    );

    smp_mb();
    #[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
    preempt_enable();
    *uval = existval;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
