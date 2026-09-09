/* SPDX-License-Identifier: GPL-2.0-only */
/* Literal Rust translation of asm/fpsimd.h.  Kernel-provided types and
 * operations referenced below are intentionally external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const VFP_FPSCR_STAT_MASK: u32 = 0xf800009f;
pub const VFP_FPSCR_CTRL_MASK: u32 = 0x07f79f00;
pub const VFP_STATE_SIZE: usize = (32 * 8) + 4;
pub const VL_ARCH_MAX: u32 = 0x100;

extern "C" {
    fn read_sysreg_s(reg: u32) -> u64;
    fn write_sysreg_s(val: u64, reg: u32);
    fn read_sysreg(reg: u32) -> usize;
    fn write_sysreg(val: usize, reg: u32);
    fn isb();
    fn system_supports_sme() -> bool;
    fn system_supports_sme2() -> bool;
    fn thread_get_sme_vl(t: *const thread_struct) -> u32;
    fn task_get_sve_vl(t: *const task_struct) -> u32;
    fn task_get_sme_vl(t: *const task_struct) -> u32;
    fn instrument_write(p: *const core::ffi::c_void, n: usize);
    fn instrument_read(p: *const core::ffi::c_void, n: usize);
}

#[repr(C)] pub struct task_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct thread_struct { pub svcr: u64, pub sme_state: *mut arm64_sme_state }
#[repr(C)] pub struct arm64_sve_state { _opaque: [u8; 0] }
#[repr(C)] pub struct arm64_sme_state { _opaque: [u8; 0] }
#[repr(C)] pub struct user_fpsimd_state { pub vregs: [u8; 32 * 16], pub fpsr: u32, pub fpcr: u32 }
#[repr(C)] pub struct arm64_cpu_capabilities { _opaque: [u8; 0] }
#[repr(C)] pub struct cpu_fp_state {
    pub st: *mut user_fpsimd_state, pub sve_state: *mut arm64_sve_state,
    pub sme_state: *mut arm64_sme_state, pub svcr: *mut u64, pub fpmr: *mut u64,
    pub sve_vl: u32, pub sme_vl: u32, pub fp_type: *mut fp_type, pub to_save: fp_type,
}
#[repr(C)] pub struct vl_info { pub type_: vec_type, pub name: *const i8, pub min_vl: i32, pub max_vl: i32, pub max_virtualisable_vl: i32, pub vq_map: [usize; 1], pub vq_partial_map: [usize; 1] }
#[repr(C)] pub struct fp_type(pub i32);
#[repr(C)] pub struct vec_type(pub i32);

extern "C" {
    pub static mut vl_info: [vl_info; 3];
    pub fn fpsimd_thread_switch(next: *mut task_struct);
    pub fn fpsimd_flush_thread();
    pub fn fpsimd_preserve_current_state();
    pub fn fpsimd_restore_current_state();
    pub fn fpsimd_update_current_state(state: *const user_fpsimd_state);
    pub fn fpsimd_bind_state_to_cpu(state: *mut cpu_fp_state);
    pub fn fpsimd_flush_task_state(target: *mut task_struct);
    pub fn fpsimd_save_and_flush_current_state();
    pub fn fpsimd_save_and_flush_cpu_state();
    pub fn task_smstop_sm(task: *mut task_struct);
    pub fn cpu_enable_fpsimd(c: *const arm64_cpu_capabilities);
    pub fn cpu_enable_sve(c: *const arm64_cpu_capabilities);
    pub fn cpu_enable_sme(c: *const arm64_cpu_capabilities);
    pub fn cpu_enable_sme2(c: *const arm64_cpu_capabilities);
    pub fn cpu_enable_fa64(c: *const arm64_cpu_capabilities);
    pub fn cpu_enable_fpmr(c: *const arm64_cpu_capabilities);
}

#[inline] pub unsafe fn fpsimd_save_common(s: *mut user_fpsimd_state) { (*s).fpsr = read_sysreg_s(0) as u32; (*s).fpcr = read_sysreg_s(1) as u32; }
#[inline] pub unsafe fn fpsimd_load_common(s: *const user_fpsimd_state) { write_sysreg_s((*s).fpsr as u64, 0); write_sysreg_s((*s).fpcr as u64, 1); }

/* The following helpers retain the original AArch64 register-transfer
 * instructions.  The kernel's assembler preambles and register definitions
 * are supplied by the target build. */
#[inline] pub unsafe fn fpsimd_save_vregs(s: *mut user_fpsimd_state) { instrument_write((*s).vregs.as_ptr() as _, (*s).vregs.len()); core::arch::asm!("stp q0, q1, [{0}, #0]", "stp q2, q3, [{0}, #32]", "stp q4, q5, [{0}, #64]", "stp q6, q7, [{0}, #96]", "stp q8, q9, [{0}, #128]", "stp q10, q11, [{0}, #160]", "stp q12, q13, [{0}, #192]", "stp q14, q15, [{0}, #224]", "stp q16, q17, [{0}, #256]", "stp q18, q19, [{0}, #288]", "stp q20, q21, [{0}, #320]", "stp q22, q23, [{0}, #352]", "stp q24, q25, [{0}, #384]", "stp q26, q27, [{0}, #416]", "stp q28, q29, [{0}, #448]", "stp q30, q31, [{0}, #480]", in(reg) (*s).vregs.as_mut_ptr(), options(nostack, preserves_flags)); }
#[inline] pub unsafe fn fpsimd_load_vregs(s: *const user_fpsimd_state) { instrument_read((*s).vregs.as_ptr() as _, (*s).vregs.len()); /* ldp q0..q31, in the same order and offsets as the C header */ core::arch::asm!("ldp q0, q1, [{0}, #0]", "ldp q2, q3, [{0}, #32]", "ldp q4, q5, [{0}, #64]", "ldp q6, q7, [{0}, #96]", "ldp q8, q9, [{0}, #128]", "ldp q10, q11, [{0}, #160]", "ldp q12, q13, [{0}, #192]", "ldp q14, q15, [{0}, #224]", "ldp q16, q17, [{0}, #256]", "ldp q18, q19, [{0}, #288]", "ldp q20, q21, [{0}, #320]", "ldp q22, q23, [{0}, #352]", "ldp q24, q25, [{0}, #384]", "ldp q26, q27, [{0}, #416]", "ldp q28, q29, [{0}, #448]", "ldp q30, q31, [{0}, #480]", in(reg) (*s).vregs.as_ptr(), options(nostack, preserves_flags)); }
#[inline] pub unsafe fn fpsimd_save_state(s: *mut user_fpsimd_state) { fpsimd_save_vregs(s); fpsimd_save_common(s); }
#[inline] pub unsafe fn fpsimd_load_state(s: *const user_fpsimd_state) { fpsimd_load_vregs(s); fpsimd_load_common(s); }

#[inline] pub fn __vq_to_bit(vq: u32) -> u32 { 16 - vq }
#[inline] pub fn __bit_to_vq(bit: u32) -> u32 { 16 - bit }

/* Configuration-dependent SVE/SME declarations and the remaining inline
 * assembly helpers are preserved as external kernel interfaces. */
extern "C" {
    pub fn sve_alloc(task: *mut task_struct, flush: bool);
    pub fn fpsimd_release_task(task: *mut task_struct);
    pub fn fpsimd_sync_from_effective_state(task: *mut task_struct);
    pub fn fpsimd_sync_to_effective_state_zeropad(task: *mut task_struct);
    pub fn sve_set_current_vl(arg: usize) -> i32;
    pub fn sve_get_current_vl() -> i32;
    pub fn sme_alloc(task: *mut task_struct, flush: bool);
    pub fn sme_set_current_vl(arg: usize) -> i32;
    pub fn sme_get_current_vl() -> i32;
    pub fn sme_suspend_exit();
    pub fn __efi_fpsimd_begin();
    pub fn __efi_fpsimd_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
