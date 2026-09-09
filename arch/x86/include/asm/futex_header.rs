/* SPDX-License-Identifier: GPL-2.0 */
// Translation of the x86 futex header. The original contents are kernel-only
// and depend on symbols supplied by the surrounding kernel translation unit.

#[macro_export]
macro_rules! unsafe_atomic_op1 {
    ($insn:expr, $oval:expr, $uaddr:expr, $oparg:expr, $label:lifetime) => {{
        let mut oldval: i32 = 0;
        let mut ret: i32 = 0;
        unsafe {
            core::arch::asm!(
                "1:",
                "{insn}",
                "2:",
                insn = const $insn,
                out(reg) oldval,
                out(reg) ret,
                inout(reg) $oparg => _,
                in("memory") $uaddr,
            );
        }
        if ret != 0 {
            break $label;
        }
        *$oval = oldval;
    }};
}

#[macro_export]
macro_rules! unsafe_atomic_op2 {
    ($insn:expr, $oval:expr, $uaddr:expr, $oparg:expr, $label:lifetime) => {{
        let mut oldval: i32 = 0;
        let mut ret: i32 = 0;
        let mut tem: i32;
        unsafe {
            core::arch::asm!(
                "1: movl {oldval}, [{uaddr}]",
                "2: movl {oldval}, {tem}",
                "{insn}",
                "3: lock cmpxchgl {tem}, [{uaddr}]",
                "jnz 2b",
                "4:",
                oldval = inout("eax") oldval,
                ret = lateout(reg) ret,
                tem = lateout(reg) tem,
                uaddr = in(reg) $uaddr,
                in(reg) $oparg,
                insn = const $insn,
            );
        }
        if ret != 0 {
            break $label;
        }
        *$oval = oldval;
    }};
}

#[inline(always)]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    'Efault: {
        // scoped_user_rw_access(uaddr, Efault) is a kernel user-memory access
        // guard and is supplied by the surrounding translation unit.
        match op {
            FUTEX_OP_SET => {
                unsafe_atomic_op1!("xchgl %0, %2", oval, uaddr, oparg, 'Efault);
            }
            FUTEX_OP_ADD => {
                unsafe_atomic_op1!(concat!(LOCK_PREFIX, "xaddl %0, %2"), oval, uaddr, oparg, 'Efault);
            }
            FUTEX_OP_OR => {
                unsafe_atomic_op2!("orl %4, %3", oval, uaddr, oparg, 'Efault);
            }
            FUTEX_OP_ANDN => {
                unsafe_atomic_op2!("andl %4, %3", oval, uaddr, !oparg, 'Efault);
            }
            FUTEX_OP_XOR => {
                unsafe_atomic_op2!("xorl %4, %3", oval, uaddr, oparg, 'Efault);
            }
            _ => return -ENOSYS,
        }
        return 0;
    }
    -EFAULT
}

#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    mut oldval: u32,
    newval: u32,
) -> i32 {
    let mut ret: i32 = 0;
    'Efault: {
        unsafe {
            core::arch::asm!(
                "1: lock cmpxchgl {newval}, [{uaddr}]",
                "2:",
                inout(reg) ret,
                inout("eax") oldval,
                uaddr = in(reg) uaddr,
                newval = in(reg) newval,
                options(readonly),
            );
        }
        unsafe { *uval = oldval; }
        return ret;
    }
    -EFAULT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
