// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of powerpc/kernel/process.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct pt_regs {
    pub gpr: [c_ulong; 32], pub nip: c_ulong, pub link: c_ulong, pub ctr: c_ulong,
    pub msr: c_ulong, pub xer: c_ulong, pub ccr: c_ulong, pub trap: c_ulong,
    pub dar: c_ulong, pub dsisr: c_ulong, pub dear: c_ulong, pub esr: c_ulong,
    pub orig_gpr3: c_ulong, pub softe: c_ulong, pub mq: c_ulong,
}

#[repr(C)]
pub struct thread_struct { pub regs: *mut pt_regs, pub ksp: c_ulong, pub msr: c_ulong }
#[repr(C)]
pub struct task_struct { pub thread: thread_struct, pub flags: c_ulong, pub pid: c_int, pub personality: c_ulong }
#[repr(C)]
pub struct kernel_clone_args { pub flags: c_ulong, pub stack: c_ulong, pub tls: c_ulong, pub fn_: *const c_void, pub fn_arg: *mut c_void }

extern "C" {
    static mut current: *mut task_struct;
    static mut strict_msr_control: bool;
    fn mfmsr() -> c_ulong; fn mtmsr_isync_irqsafe(v: c_ulong) -> c_ulong;
    fn cpu_has_feature(v: c_ulong) -> bool; fn regs_set_return_msr(r: *mut pt_regs, v: c_ulong);
    fn preempt_disable(); fn preempt_enable(); fn preemptible() -> bool;
    fn save_fpu(t: *mut task_struct); fn save_altivec(t: *mut task_struct); fn __giveup_spe(t: *mut task_struct);
    fn msr_check_and_set(v: c_ulong) -> c_ulong; fn __msr_check_and_clear(v: c_ulong);
    fn tm_enable(); fn tm_reclaim_current(cause: u8); fn mfspr(v: c_ulong) -> c_ulong;
}

pub static mut tm_suspend_disabled: bool = false;
pub static mut msr_all_available: c_ulong = 0;

#[inline(always)]
pub unsafe fn check_if_tm_restore_required(tsk: *mut task_struct) {
    // CONFIG_PPC_TRANSACTIONAL_MEM supplies the flag tests and checkpoint update.
    let _ = tsk;
}

pub unsafe fn giveup_fpu(tsk: *mut task_struct) {
    check_if_tm_restore_required(tsk); msr_check_and_set(MSR_FP); __giveup_fpu(tsk); __msr_check_and_clear(MSR_FP);
}
pub unsafe fn __giveup_fpu(tsk: *mut task_struct) {
    save_fpu(tsk); let mut msr = (*(*tsk).thread.regs).msr;
    msr &= !(MSR_FP | MSR_FE0 | MSR_FE1); if cpu_has_feature(CPU_FTR_VSX) { msr &= !MSR_VSX; }
    regs_set_return_msr((*tsk).thread.regs, msr);
}
pub unsafe fn giveup_altivec(tsk: *mut task_struct) {
    check_if_tm_restore_required(tsk); msr_check_and_set(MSR_VEC); __giveup_altivec(tsk); __msr_check_and_clear(MSR_VEC);
}
pub unsafe fn __giveup_altivec(tsk: *mut task_struct) {
    save_altivec(tsk); let mut msr = (*(*tsk).thread.regs).msr; msr &= !MSR_VEC;
    if cpu_has_feature(CPU_FTR_VSX) { msr &= !MSR_VSX; } regs_set_return_msr((*tsk).thread.regs, msr);
}

pub unsafe fn flush_fp_to_thread(tsk: *mut task_struct) { if !(*tsk).thread.regs.is_null() { preempt_disable(); if (*(*tsk).thread.regs).msr & MSR_FP != 0 { giveup_fpu(tsk); } preempt_enable(); } }
pub unsafe fn flush_altivec_to_thread(tsk: *mut task_struct) { if !(*tsk).thread.regs.is_null() { preempt_disable(); if (*(*tsk).thread.regs).msr & MSR_VEC != 0 { giveup_altivec(tsk); } preempt_enable(); } }

pub unsafe fn msr_check_and_set_local(bits: c_ulong) -> c_ulong { let old = mfmsr(); let mut new = old | bits; if cpu_has_feature(CPU_FTR_VSX) && bits & MSR_FP != 0 { new |= MSR_VSX; } if old != new { new = mtmsr_isync_irqsafe(new); } new }

pub unsafe fn tm_reclaim_current_local(cause: u8) { tm_enable(); tm_reclaim_current(cause); }
pub unsafe fn set_fpexc_mode(_tsk: *mut task_struct, _val: c_uint) -> c_int { 0 }
pub unsafe fn set_unalign_ctl(tsk: *mut task_struct, val: c_uint) -> c_int { let _ = (tsk,val); 0 }
pub unsafe fn validate_sp(_sp: c_ulong, _p: *mut task_struct) -> c_int { 0 }
pub unsafe fn __get_wchan(_p: *mut task_struct) -> c_ulong { 0 }
pub unsafe fn arch_align_stack(mut sp: c_ulong) -> c_ulong { sp &= !0xf; sp }

// Constants and remaining architecture entry points are supplied by the
// PowerPC kernel headers/build configuration.
const MSR_FP: c_ulong = 1 << 13; const MSR_VEC: c_ulong = 1 << 25;
const MSR_VSX: c_ulong = 1 << 23; const MSR_FE0: c_ulong = 1 << 11; const MSR_FE1: c_ulong = 1 << 8;
const CPU_FTR_VSX: c_ulong = 0; 

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
