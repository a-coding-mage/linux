// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/sched.h, and asm-generic/compat.h

pub const COMPAT_UTS_MACHINE: &[u8] = b"riscv32\0\0";

// Architecture specific compatibility types

pub unsafe fn is_compat_task() -> i32 {
    // Preserves the build-time CONFIG_COMPAT condition from IS_ENABLED(CONFIG_COMPAT).
    if !cfg!(feature = "CONFIG_COMPAT") {
        return 0;
    }

    test_thread_flag(TIF_32BIT)
}

pub unsafe fn is_compat_thread(thread: *mut thread_info) -> i32 {
    // Preserves the build-time CONFIG_COMPAT condition from IS_ENABLED(CONFIG_COMPAT).
    if !cfg!(feature = "CONFIG_COMPAT") {
        return 0;
    }

    test_ti_thread_flag(thread, TIF_32BIT)
}

pub unsafe fn set_compat_task(is_compat: bool) {
    if is_compat {
        set_thread_flag(TIF_32BIT);
    } else {
        clear_thread_flag(TIF_32BIT);
    }
}

#[repr(C)]
pub struct compat_user_regs_struct {
    pub pc: compat_ulong_t,
    pub ra: compat_ulong_t,
    pub sp: compat_ulong_t,
    pub gp: compat_ulong_t,
    pub tp: compat_ulong_t,
    pub t0: compat_ulong_t,
    pub t1: compat_ulong_t,
    pub t2: compat_ulong_t,
    pub s0: compat_ulong_t,
    pub s1: compat_ulong_t,
    pub a0: compat_ulong_t,
    pub a1: compat_ulong_t,
    pub a2: compat_ulong_t,
    pub a3: compat_ulong_t,
    pub a4: compat_ulong_t,
    pub a5: compat_ulong_t,
    pub a6: compat_ulong_t,
    pub a7: compat_ulong_t,
    pub s2: compat_ulong_t,
    pub s3: compat_ulong_t,
    pub s4: compat_ulong_t,
    pub s5: compat_ulong_t,
    pub s6: compat_ulong_t,
    pub s7: compat_ulong_t,
    pub s8: compat_ulong_t,
    pub s9: compat_ulong_t,
    pub s10: compat_ulong_t,
    pub s11: compat_ulong_t,
    pub t3: compat_ulong_t,
    pub t4: compat_ulong_t,
    pub t5: compat_ulong_t,
    pub t6: compat_ulong_t,
}

pub unsafe fn regs_to_cregs(
    cregs: *mut compat_user_regs_struct,
    regs: *mut pt_regs,
) {
    (*cregs).pc = (*regs).epc as compat_ulong_t;
    (*cregs).ra = (*regs).ra as compat_ulong_t;
    (*cregs).sp = (*regs).sp as compat_ulong_t;
    (*cregs).gp = (*regs).gp as compat_ulong_t;
    (*cregs).tp = (*regs).tp as compat_ulong_t;
    (*cregs).t0 = (*regs).t0 as compat_ulong_t;
    (*cregs).t1 = (*regs).t1 as compat_ulong_t;
    (*cregs).t2 = (*regs).t2 as compat_ulong_t;
    (*cregs).s0 = (*regs).s0 as compat_ulong_t;
    (*cregs).s1 = (*regs).s1 as compat_ulong_t;
    (*cregs).a0 = (*regs).a0 as compat_ulong_t;
    (*cregs).a1 = (*regs).a1 as compat_ulong_t;
    (*cregs).a2 = (*regs).a2 as compat_ulong_t;
    (*cregs).a3 = (*regs).a3 as compat_ulong_t;
    (*cregs).a4 = (*regs).a4 as compat_ulong_t;
    (*cregs).a5 = (*regs).a5 as compat_ulong_t;
    (*cregs).a6 = (*regs).a6 as compat_ulong_t;
    (*cregs).a7 = (*regs).a7 as compat_ulong_t;
    (*cregs).s2 = (*regs).s2 as compat_ulong_t;
    (*cregs).s3 = (*regs).s3 as compat_ulong_t;
    (*cregs).s4 = (*regs).s4 as compat_ulong_t;
    (*cregs).s5 = (*regs).s5 as compat_ulong_t;
    (*cregs).s6 = (*regs).s6 as compat_ulong_t;
    (*cregs).s7 = (*regs).s7 as compat_ulong_t;
    (*cregs).s8 = (*regs).s8 as compat_ulong_t;
    (*cregs).s9 = (*regs).s9 as compat_ulong_t;
    (*cregs).s10 = (*regs).s10 as compat_ulong_t;
    (*cregs).s11 = (*regs).s11 as compat_ulong_t;
    (*cregs).t3 = (*regs).t3 as compat_ulong_t;
    (*cregs).t4 = (*regs).t4 as compat_ulong_t;
    (*cregs).t5 = (*regs).t5 as compat_ulong_t;
    (*cregs).t6 = (*regs).t6 as compat_ulong_t;
}

pub unsafe fn cregs_to_regs(
    cregs: *mut compat_user_regs_struct,
    regs: *mut pt_regs,
) {
    (*regs).epc = (*cregs).pc as core::ffi::c_ulong;
    (*regs).ra = (*cregs).ra as core::ffi::c_ulong;
    (*regs).sp = (*cregs).sp as core::ffi::c_ulong;
    (*regs).gp = (*cregs).gp as core::ffi::c_ulong;
    (*regs).tp = (*cregs).tp as core::ffi::c_ulong;
    (*regs).t0 = (*cregs).t0 as core::ffi::c_ulong;
    (*regs).t1 = (*cregs).t1 as core::ffi::c_ulong;
    (*regs).t2 = (*cregs).t2 as core::ffi::c_ulong;
    (*regs).s0 = (*cregs).s0 as core::ffi::c_ulong;
    (*regs).s1 = (*cregs).s1 as core::ffi::c_ulong;
    (*regs).a0 = (*cregs).a0 as core::ffi::c_ulong;
    (*regs).a1 = (*cregs).a1 as core::ffi::c_ulong;
    (*regs).a2 = (*cregs).a2 as core::ffi::c_ulong;
    (*regs).a3 = (*cregs).a3 as core::ffi::c_ulong;
    (*regs).a4 = (*cregs).a4 as core::ffi::c_ulong;
    (*regs).a5 = (*cregs).a5 as core::ffi::c_ulong;
    (*regs).a6 = (*cregs).a6 as core::ffi::c_ulong;
    (*regs).a7 = (*cregs).a7 as core::ffi::c_ulong;
    (*regs).s2 = (*cregs).s2 as core::ffi::c_ulong;
    (*regs).s3 = (*cregs).s3 as core::ffi::c_ulong;
    (*regs).s4 = (*cregs).s4 as core::ffi::c_ulong;
    (*regs).s5 = (*cregs).s5 as core::ffi::c_ulong;
    (*regs).s6 = (*cregs).s6 as core::ffi::c_ulong;
    (*regs).s7 = (*cregs).s7 as core::ffi::c_ulong;
    (*regs).s8 = (*cregs).s8 as core::ffi::c_ulong;
    (*regs).s9 = (*cregs).s9 as core::ffi::c_ulong;
    (*regs).s10 = (*cregs).s10 as core::ffi::c_ulong;
    (*regs).s11 = (*cregs).s11 as core::ffi::c_ulong;
    (*regs).t3 = (*cregs).t3 as core::ffi::c_ulong;
    (*regs).t4 = (*cregs).t4 as core::ffi::c_ulong;
    (*regs).t5 = (*cregs).t5 as core::ffi::c_ulong;
    (*regs).t6 = (*cregs).t6 as core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
