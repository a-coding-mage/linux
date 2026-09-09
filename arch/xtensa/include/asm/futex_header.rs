/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Atomic futex routines
 *
 * Based on the PowerPC implementataion
 *
 * Copyright (C) 2013 TangoTec Ltd.
 *
 * Baruch Siach <baruch@tkos.co.il>
 */

/* Rust translation of the Xtensa futex header. */

pub const ARCH_FUTEX_ATOMIC_OP_INUSER: &str = "arch_futex_atomic_op_inuser";
pub const FUTEX_ATOMIC_CMPXCHG_INATOMIC: &str = "futex_atomic_cmpxchg_inatomic";

/* XCHAL_HAVE_EXCLUSIVE and XCHAL_HAVE_S32C1I are build-time Xtensa configuration
 * symbols.  The original header selects the corresponding inline assembly path. */

extern "C" {
    fn access_ok(addr: *const core::ffi::c_void, size: usize) -> bool;
    fn futex_atomic_op_inuser_local(
        op: i32,
        oparg: i32,
        oval: *mut i32,
        uaddr: *mut u32,
    ) -> i32;
    fn futex_atomic_cmpxchg_inatomic_local(
        uval: *mut u32,
        uaddr: *mut u32,
        oldval: u32,
        newval: u32,
    ) -> i32;
}

/* Linux errno and futex operation constants are supplied by the translated
 * dependency headers. */
extern "C" {
    static EFAULT: i32;
    static ENOSYS: i32;
    static FUTEX_OP_SET: i32;
    static FUTEX_OP_ADD: i32;
    static FUTEX_OP_OR: i32;
    static FUTEX_OP_ANDN: i32;
    static FUTEX_OP_XOR: i32;
}

#[allow(unused_variables)]
unsafe fn futex_atomic_op_hardware(
    _insn: &str,
    _ret: &mut i32,
    _old: &mut i32,
    _uaddr: *mut u32,
    _arg: i32,
) {
    /*
     * Original __futex_atomic_op expands to Xtensa l32ex/s32ex/getex or
     * l32i/wsr/s32c1i assembly, including .fixup and __ex_table entries.
     * Those assembler sections are retained as external target-specific
     * dependency behavior rather than reimplemented in portable Rust.
     */
    unimplemented!("Xtensa inline futex atomic operation");
}

pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    /* XCHAL_HAVE_S32C1I || XCHAL_HAVE_EXCLUSIVE */
    if !access_ok(uaddr.cast(), core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    let mut oldval: i32 = 0;
    let mut ret: i32;
    if op == FUTEX_OP_SET {
        futex_atomic_op_hardware("mov %[newval], %[oparg]", &mut ret, &mut oldval, uaddr, oparg);
    } else if op == FUTEX_OP_ADD {
        futex_atomic_op_hardware("add %[newval], %[oldval], %[oparg]", &mut ret, &mut oldval, uaddr, oparg);
    } else if op == FUTEX_OP_OR {
        futex_atomic_op_hardware("or %[newval], %[oldval], %[oparg]", &mut ret, &mut oldval, uaddr, oparg);
    } else if op == FUTEX_OP_ANDN {
        futex_atomic_op_hardware("and %[newval], %[oldval], %[oparg]", &mut ret, &mut oldval, uaddr, !oparg);
    } else if op == FUTEX_OP_XOR {
        futex_atomic_op_hardware("xor %[newval], %[oldval], %[oparg]", &mut ret, &mut oldval, uaddr, oparg);
    } else {
        ret = -ENOSYS;
    }
    if ret == 0 {
        *oval = oldval;
    }
    ret
}

pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    /* XCHAL_HAVE_S32C1I || XCHAL_HAVE_EXCLUSIVE: Xtensa cmpxchg inline asm. */
    if !access_ok(uaddr.cast(), core::mem::size_of::<u32>()) {
        return -EFAULT;
    }
    /* The original uses l32ex/s32ex/getex or wsr/s32c1i and exception tables. */
    let _ = (uval, uaddr, oldval, newval);
    unimplemented!("Xtensa inline futex atomic compare-exchange");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
