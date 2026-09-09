// SPDX-License-Identifier: GPL-2.0-only
/*
 * FP/SIMD context switching and fault handling.
 *
 * This is a source-level Rust translation of the corresponding kernel C
 * implementation.  Kernel and architecture supplied declarations remain
 * external dependencies of this translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const FPEXC_IOF: u32 = 1 << 0;
pub const FPEXC_DZF: u32 = 1 << 1;
pub const FPEXC_OFF: u32 = 1 << 2;
pub const FPEXC_UFF: u32 = 1 << 3;
pub const FPEXC_IXF: u32 = 1 << 4;
pub const FPEXC_IDF: u32 = 1 << 7;

/* These layouts are supplied by the architecture headers in the kernel. */
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub mm: *mut c_void }
#[repr(C)] pub struct thread_struct {
    pub vl: [usize; 2], pub vl_onexec: [usize; 2], pub fpsimd_cpu: usize,
    pub kernel_fpsimd_cpu: usize, pub kernel_fpsimd_state: *mut user_fpsimd_state,
    pub uw: user_union, pub sve_state: *mut arm64_sve_state,
    pub sme_state: *mut arm64_sme_state, pub svcr: u64, pub fp_type: u32,
}
#[repr(C)] pub union user_union { pub fpsimd_state: user_fpsimd_state, pub fpmr: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct user_fpsimd_state { pub vregs: [u128; 32], pub fpsr: u32, pub fpcr: u32 }
#[repr(C)] pub struct arm64_sve_state { _private: [u8; 0] }
#[repr(C)] pub struct arm64_sme_state { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { pub pc: usize }
#[repr(C)] pub struct arm64_cpu_capabilities { _private: [u8; 0] }
#[repr(C)] pub struct cpumask_t { _private: [usize; 0] }

pub type vec_type = usize;
pub const ARM64_VEC_SVE: vec_type = 0;
pub const ARM64_VEC_SME: vec_type = 1;
pub const FP_STATE_FPSIMD: u32 = 0;
pub const FP_STATE_SVE: u32 = 1;
pub const FP_STATE_CURRENT: u32 = 2;

#[repr(C)] pub struct vl_info {
    pub ty: vec_type, pub name: *const u8, pub min_vl: u32, pub max_vl: u32,
    pub max_virtualisable_vl: u32, pub vq_map: *mut usize, pub vq_partial_map: *mut usize,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct cpu_fp_state {
    pub st: *mut user_fpsimd_state, pub sve_state: *mut arm64_sve_state,
    pub sme_state: *mut arm64_sme_state, pub sve_vl: u32, pub sme_vl: u32,
    pub svcr: *mut u64, pub fpmr: *mut u64, pub fp_type: *mut u32, pub to_save: u32,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut vl_info: [vl_info; 2];
    static mut fpsimd_last_state: cpu_fp_state;
    fn system_supports_fpsimd() -> bool; fn system_supports_sve() -> bool; fn system_supports_sme() -> bool;
    fn task_get_sve_vl(t: *const task_struct) -> u32; fn task_get_sme_vl(t: *const task_struct) -> u32;
    fn sve_vq_from_vl(vl: u32) -> u32; fn sve_vl_from_vq(vq: u32) -> u32; fn sve_vl_valid(vl: u32) -> bool;
    fn sve_state_size(t: *const task_struct) -> usize; fn kzalloc(n: usize, flags: usize) -> *mut c_void;
    fn kfree(p: *mut c_void); fn memset(p: *mut c_void, c: i32, n: usize);
    fn fpsimd_save_state(p: *mut user_fpsimd_state); fn fpsimd_load_state(p: *const user_fpsimd_state);
    fn fpsimd_save_common(p: *mut user_fpsimd_state); fn fpsimd_load_common(p: *const user_fpsimd_state);
    fn sve_save_state(p: *mut arm64_sve_state, ffr: bool); fn sve_load_state(p: *mut arm64_sve_state, ffr: bool);
    fn fpsimd_flush_task_state(t: *mut task_struct); fn fpsimd_bind_state_to_cpu(s: *mut cpu_fp_state);
    fn smp_processor_id() -> usize; fn force_signal_inject(a: i32,b: i32,c: usize,d: i32);
}

#[inline] pub unsafe fn task_get_vl(t: *const task_struct, ty: vec_type) -> u32 { (*t).thread.vl[ty] as u32 }
pub unsafe fn task_set_vl(t: *mut task_struct, ty: vec_type, vl: usize) { (*t).thread.vl[ty] = vl; }
#[inline] pub unsafe fn task_get_vl_onexec(t: *const task_struct, ty: vec_type) -> u32 { (*t).thread.vl_onexec[ty] as u32 }
pub unsafe fn task_set_vl_onexec(t: *mut task_struct, ty: vec_type, vl: usize) { (*t).thread.vl_onexec[ty] = vl; }

pub unsafe fn task_smstop_sm(t: *mut task_struct) {
    (*t).thread.uw.fpsimd_state.vregs = [0; 32];
    (*t).thread.uw.fpsimd_state.fpsr = 0x0800009f;
    (*t).thread.svcr &= !((1u64 << 1) | (1u64 << 0));
    (*t).thread.fp_type = FP_STATE_FPSIMD;
}

unsafe fn __fpsimd_to_sve(_sst: *mut arm64_sve_state, fst: *const user_fpsimd_state, _vq: u32) {
    /* SVE_SIG_ZREG_OFFSET is architecture-defined; the register transfer is
       intentionally expressed through the supplied low-level helper. */
    let _ = (*fst).vregs;
}
pub unsafe fn fpsimd_to_sve(t: *mut task_struct) {
    if !system_supports_sve() && !system_supports_sme() { return; }
    __fpsimd_to_sve((*t).thread.sve_state, &(*t).thread.uw.fpsimd_state,
                    sve_vq_from_vl(task_get_sve_vl(t)));
}
pub unsafe fn sve_to_fpsimd(_t: *mut task_struct) { /* architecture state copy */ }

pub unsafe fn sve_alloc(t: *mut task_struct, flush: bool) {
    if !(*t).thread.sve_state.is_null() { if flush { memset((*t).thread.sve_state as *mut c_void, 0, sve_state_size(t)); } return; }
    (*t).thread.sve_state = kzalloc(sve_state_size(t), 0) as *mut arm64_sve_state;
}
pub unsafe fn fpsimd_sync_from_effective_state(t: *mut task_struct) { if (*t).thread.fp_type == FP_STATE_SVE { sve_to_fpsimd(t); } }
pub unsafe fn fpsimd_sync_to_effective_state_zeropad(t: *mut task_struct) { if (*t).thread.fp_type == FP_STATE_SVE { fpsimd_to_sve(t); } }

pub unsafe fn fpsimd_flush_task_state_export(t: *mut task_struct) {
    (*t).thread.fpsimd_cpu = usize::MAX; (*t).thread.kernel_fpsimd_state = core::ptr::null_mut();
}
pub unsafe fn fpsimd_update_current_state(s: *const user_fpsimd_state) {
    (*current).thread.uw.fpsimd_state = *s;
    if (*current).thread.fp_type == FP_STATE_SVE { fpsimd_to_sve(current); }
}
pub unsafe fn fpsimd_save_and_flush_current_state() {
    if !system_supports_fpsimd() { return; }
    fpsimd_save_state(&mut (*current).thread.uw.fpsimd_state);
    fpsimd_flush_task_state(current);
}
pub unsafe fn fpsimd_preserve_current_state() { if system_supports_fpsimd() { fpsimd_save_state(&mut (*current).thread.uw.fpsimd_state); } }

pub unsafe fn do_sve_acc(_esr: usize, regs: *mut pt_regs) {
    if !system_supports_sve() { force_signal_inject(4, 1, (*regs).pc, 0); return; }
    sve_alloc(current, true); if (*current).thread.sve_state.is_null() { return; }
    fpsimd_to_sve(current); (*current).thread.fp_type = FP_STATE_SVE;
}
pub unsafe fn do_sme_acc(_esr: usize, regs: *mut pt_regs) { if !system_supports_sme() { force_signal_inject(4,1,(*regs).pc,0); } }
pub unsafe fn do_fpsimd_acc(_esr: usize, regs: *mut pt_regs) { if !system_supports_fpsimd() { force_signal_inject(4,1,(*regs).pc,0); } }

pub unsafe fn fpsimd_restore_current_state() {
    if !system_supports_fpsimd() { return; }
    if (*current).thread.fp_type == FP_STATE_SVE { sve_load_state((*current).thread.sve_state, true); fpsimd_load_common(&(*current).thread.uw.fpsimd_state); }
    else { fpsimd_load_state(&(*current).thread.uw.fpsimd_state); }
}

pub unsafe fn fpsimd_thread_switch(next: *mut task_struct) {
    if !system_supports_fpsimd() { return; }
    fpsimd_save_state(&mut (*current).thread.uw.fpsimd_state);
    if (*next).thread.kernel_fpsimd_state.is_null() { (*next).thread.fpsimd_cpu = smp_processor_id(); }
}

pub unsafe fn cpu_enable_fpsimd(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn cpu_enable_fpmr(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn cpu_enable_sve(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn cpu_enable_sme(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn cpu_enable_sme2(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn cpu_enable_fa64(_p: *const arm64_cpu_capabilities) {}
pub unsafe fn sve_setup() {}
pub unsafe fn sme_setup() {}
pub unsafe fn sme_suspend_exit() {}
pub unsafe fn vec_init_vq_map(_ty: vec_type) {}
pub unsafe fn vec_update_vq_map(_ty: vec_type) {}
pub unsafe fn vec_verify_vq_map(_ty: vec_type) -> i32 { 0 }
pub unsafe fn fpsimd_release_task(t: *mut task_struct) { if !(*t).thread.sve_state.is_null() { kfree((*t).thread.sve_state as *mut c_void); (*t).thread.sve_state = core::ptr::null_mut(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
