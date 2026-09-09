/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the ARM Linux futex header.
// The original include dependencies and __KERNEL__ build condition are supplied
// by the surrounding kernel translation unit.

#[cfg(feature = "CONFIG_SMP")]
macro_rules! __futex_atomic_op {
    ($insn:expr, $ret:expr, $oldval:expr, $tmp:expr, $uaddr:expr, $oparg:expr) => {{
        let mut __ua_flags: u32;
        smp_mb();
        prefetchw($uaddr);
        __ua_flags = uaccess_save_and_enable();
        unsafe {
            core::arch::asm!(
                "1: ldrex {old}, [{addr}]",
                $insn,
                "2: strex {tmp}, {ret}, [{addr}]",
                "teq {tmp}, #0",
                "bne 1b",
                "mov {ret}, #0",
                "3:",
                ".pushsection __ex_table,\"a\"",
                ".align 3",
                ".long 1b, 4f, 2b, 4f",
                ".popsection",
                ".pushsection .text.fixup,\"ax\"",
                ".align 2",
                "4: mov {ret}, {fault}",
                "b 3b",
                ".popsection",
                ret = inout(reg) $ret,
                old = inout(reg) $oldval,
                tmp = inout(reg) $tmp,
                addr = in(reg) $uaddr,
                in(reg) $oparg,
                fault = const -EFAULT,
                options(nostack)
            );
        }
        uaccess_restore(__ua_flags);
    }};
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut __ua_flags: u32;
    let mut ret: i32;
    let mut val: u32;
    if !access_ok(uaddr, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }
    smp_mb();
    prefetchw(uaddr);
    __ua_flags = uaccess_save_and_enable();
    core::arch::asm!(
        "1: ldrex {val}, [{addr}]",
        "teq {val}, {old}",
        "ite eq",
        "2: strexeq {ret}, {new}, [{addr}]",
        "movne {ret}, #0",
        "teq {ret}, #0",
        "bne 1b",
        "3:",
        "4: mov {ret}, {fault}",
        "b 3b",
        ret = out(reg) ret, val = out(reg) val, addr = in(reg) uaddr,
        old = in(reg) oldval, new = in(reg) newval, fault = const -EFAULT,
        options(nostack)
    );
    uaccess_restore(__ua_flags);
    smp_mb();
    *uval = val;
    ret
}

// !CONFIG_SMP uses preemption disabling and TUSER load/store instructions.
// The corresponding inline-assembly macro is retained as a declaration-level
// placeholder because TUSER is supplied by the ARM kernel dependencies.
#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! __futex_atomic_op {
    ($insn:expr, $ret:expr, $oldval:expr, $tmp:expr, $uaddr:expr, $oparg:expr) => {{
        let __ua_flags = uaccess_save_and_enable();
        unsafe { core::arch::asm!($insn, in(reg) $uaddr, in(reg) $oparg, out(reg) $ret, out(reg) $oldval, out(reg) $tmp); }
        uaccess_restore(__ua_flags);
    }};
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe fn futex_atomic_cmpxchg_inatomic(uval: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u32;
    if !access_ok(uaddr, core::mem::size_of::<u32>()) { return -EFAULT; }
    preempt_disable();
    let __ua_flags = uaccess_save_and_enable();
    core::arch::asm!(".syntax unified", "1: ldr {val}, [{addr}]", "teq {val}, {old}", "it eq", "2: streq {new}, [{addr}]", val = out(reg) val, addr = in(reg) uaddr, old = in(reg) oldval, new = in(reg) newval, inout(reg) ret);
    uaccess_restore(__ua_flags);
    *uval = val;
    preempt_enable();
    ret
}

pub unsafe fn arch_futex_atomic_op_inuser(op: i32, oparg: i32, oval: *mut i32, uaddr: *mut u32) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;
    let mut tmp: i32 = 0;
    if !access_ok(uaddr, core::mem::size_of::<u32>()) { return -EFAULT; }
    #[cfg(not(feature = "CONFIG_SMP"))] preempt_disable();
    ret = match op {
        FUTEX_OP_SET => { __futex_atomic_op!("mov {ret}, {arg}", ret, oldval, tmp, uaddr, oparg); ret }
        FUTEX_OP_ADD => { __futex_atomic_op!("add {ret}, {old}, {arg}", ret, oldval, tmp, uaddr, oparg); ret }
        FUTEX_OP_OR => { __futex_atomic_op!("orr {ret}, {old}, {arg}", ret, oldval, tmp, uaddr, oparg); ret }
        FUTEX_OP_ANDN => { __futex_atomic_op!("and {ret}, {old}, {arg}", ret, oldval, tmp, uaddr, !oparg); ret }
        FUTEX_OP_XOR => { __futex_atomic_op!("eor {ret}, {old}, {arg}", ret, oldval, tmp, uaddr, oparg); ret }
        _ => -ENOSYS,
    };
    #[cfg(not(feature = "CONFIG_SMP"))] preempt_enable();
    *oval = oldval;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
