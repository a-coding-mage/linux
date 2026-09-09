/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and include directives are intentionally omitted.
// Dependencies supplied by the surrounding kernel translation are referenced
// by name below.

use core::arch::asm;

// Equivalent to the C FUTEX_OP_FUNC macro. The s390 inline assembly is kept
// as a direct low-level translation of the original operation sequence.
macro_rules! futex_op_func {
    ($function:ident, $operation:expr) => {
        #[inline]
        unsafe fn $function(
            oparg: i32,
            old: *mut i32,
            uaddr: *mut u32,
        ) -> i32 {
            let mut sacf_flag: bool;
            let mut rc: i32;
            let mut new: i32;

            instrument_copy_from_user_before(
                old as *mut core::ffi::c_void,
                uaddr as *const core::ffi::c_void,
                core::mem::size_of::<i32>(),
            );
            sacf_flag = enable_sacf_uaccess();
            asm!(
                "sacf 256",
                "0: l {old}, {uaddr}",
                "1: {operation}",
                "2: cs {old}, {new}, {uaddr}",
                "3: jl 1b",
                "lhi {rc}, 0",
                "4: sacf 768",
                old = inout(reg) *old => *old,
                new = lateout(reg) new,
                uaddr = inout(reg) *uaddr => *uaddr,
                rc = lateout(reg) rc,
                oparg = in(reg) oparg,
                operation = const $operation,
                options(nostack)
            );
            disable_sacf_uaccess(sacf_flag);
            if rc == 0 {
                instrument_copy_from_user_after(
                    old as *mut core::ffi::c_void,
                    uaddr as *const core::ffi::c_void,
                    core::mem::size_of::<i32>(),
                    0,
                );
            }
            rc
        }
    };
}

futex_op_func!(__futex_atomic_set, "lr {new}, {oparg}");
futex_op_func!(__futex_atomic_add, "lr {new}, {old}\n ar {new}, {oparg}");
futex_op_func!(__futex_atomic_or, "lr {new}, {old}\n or {new}, {oparg}");
futex_op_func!(__futex_atomic_and, "lr {new}, {old}\n nr {new}, {oparg}");
futex_op_func!(__futex_atomic_xor, "lr {new}, {old}\n xr {new}, {oparg}");

#[inline]
unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    let mut old: i32 = 0;
    let rc = match op {
        FUTEX_OP_SET => __futex_atomic_set(oparg, &mut old, uaddr),
        FUTEX_OP_ADD => __futex_atomic_add(oparg, &mut old, uaddr),
        FUTEX_OP_OR => __futex_atomic_or(oparg, &mut old, uaddr),
        FUTEX_OP_ANDN => __futex_atomic_and(!oparg, &mut old, uaddr),
        FUTEX_OP_XOR => __futex_atomic_xor(oparg, &mut old, uaddr),
        _ => -ENOSYS,
    };
    if rc == 0 {
        *oval = old;
    }
    rc
}

#[inline]
unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    mut oldval: u32,
    newval: u32,
) -> i32 {
    let mut sacf_flag: bool;
    let mut rc: i32;

    instrument_copy_from_user_before(
        uval as *mut core::ffi::c_void,
        uaddr as *const core::ffi::c_void,
        core::mem::size_of::<u32>(),
    );
    sacf_flag = enable_sacf_uaccess();
    asm!(
        "sacf 256",
        "0: cs {old}, {new}, {uaddr}",
        "1: lhi {rc}, 0",
        "2: sacf 768",
        old = inout(reg) oldval,
        new = in(reg) newval,
        uaddr = inout(reg) *uaddr => *uaddr,
        rc = lateout(reg) rc,
        options(nostack)
    );
    disable_sacf_uaccess(sacf_flag);
    *uval = oldval;
    instrument_copy_from_user_after(
        uval as *mut core::ffi::c_void,
        uaddr as *const core::ffi::c_void,
        core::mem::size_of::<u32>(),
        0,
    );
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
