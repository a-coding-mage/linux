/* SPDX-License-Identifier: GPL-2.0 */

// Linux kernel-only header. The included kernel declarations and build-time
// configuration are supplied by the surrounding translation unit.

macro_rules! __futex_atomic_op {
    ($insn:literal, $ret:ident, $oldval:ident, $uaddr:ident, $oparg:ident) => {
        unsafe {
            core::arch::asm!(
                "lwarx {old}, 0, {addr}",
                $insn,
                "stwcx. {ret}, 0, {addr}",
                "bne- 0b",
                "li {ret}, 0",
                "0:",
                "1: li {ret}, {fault}",
                "b 0b",
                old = inout(reg) $oldval,
                ret = inout(reg) $ret,
                addr = in(reg) $uaddr,
                arg = in(reg) $oparg,
                fault = const -EFAULT,
                options(nostack)
            );
        }
    };
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    mut uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;

    uaddr = masked_user_access_begin(uaddr);

    match op {
        FUTEX_OP_SET => {
            __futex_atomic_op!("mr {ret}, {arg}", ret, oldval, uaddr, oparg);
        }
        FUTEX_OP_ADD => {
            __futex_atomic_op!("add {ret}, {old}, {arg}", ret, oldval, uaddr, oparg);
        }
        FUTEX_OP_OR => {
            __futex_atomic_op!("or {ret}, {old}, {arg}", ret, oldval, uaddr, oparg);
        }
        FUTEX_OP_ANDN => {
            __futex_atomic_op!("andc {ret}, {old}, {arg}", ret, oldval, uaddr, oparg);
        }
        FUTEX_OP_XOR => {
            __futex_atomic_op!("xor {ret}, {old}, {arg}", ret, oldval, uaddr, oparg);
        }
        _ => {
            ret = -ENOSYS;
        }
    }
    user_access_end();

    unsafe { *oval = oldval };
    ret
}

#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    mut uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut prev: u32;

    uaddr = masked_user_access_begin(uaddr);

    core::arch::asm!(
        "0: lwarx {prev}, 0, {addr}",
        "cmpw 0, {prev}, {old}",
        "bne- 2f",
        "1: stwcx. {new}, 0, {addr}",
        "bne- 0b",
        "2:",
        "3: li {ret}, {fault}",
        "b 2b",
        prev = lateout(reg) prev,
        ret = inout(reg) ret,
        addr = in(reg) uaddr,
        old = in(reg) oldval,
        new = in(reg) newval,
        fault = const -EFAULT,
        options(nostack)
    );

    user_access_end();
    *uval = prev;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
