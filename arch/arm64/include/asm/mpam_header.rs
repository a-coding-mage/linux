/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2025 Arm Ltd. */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/arm_mpam.h, linux/bitfield.h, linux/jump_label.h,
// linux/percpu.h, linux/sched.h, and asm/sysreg.h.

extern "C" {
    pub static mut mpam_enabled: StaticKey;
    pub static mut arm64_mpam_default: u64;
    pub static mut arm64_mpam_current: u64;

    /*
     * The value of the MPAM0_EL1 sysreg when a task is in resctrl's default group.
     * This is used by the context switch code to use the resctrl CPU property
     * instead. The value is modified when CDP is enabled/disabled by mounting
     * the resctrl filesystem.
     */
    pub static mut arm64_mpam_global_default: u64;
}

// Opaque external kernel types and operations.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

extern "C" {
    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn smp_processor_id() -> i32;
    fn system_supports_sme() -> bool;
    fn isb();
    fn write_sysreg_s(value: u64, reg: u32);
    fn task_thread_info(tsk: *mut TaskStruct) -> *mut ThreadInfo;
}

#[repr(C)]
pub struct ThreadInfo {
    pub mpam_partid_pmg: u64,
}

// These values and FIELD_PREP are provided by the imported architecture headers.
extern "C" {
    fn FIELD_PREP(mask: u64, value: u64) -> u64;
}

pub const SYS_MPAM1_EL1: u32 = 0;
pub const SYS_MPAMSM_EL1: u32 = 0;
pub const SYS_MPAM0_EL1: u32 = 0;
pub const MPAM1_EL1_MPAMEN: u64 = 0;
pub const MPAMSM_EL1_PARTID_D: u64 = 0;
pub const MPAMSM_EL1_PMG_D: u64 = 0;
pub const MPAM0_EL1_PARTID_D: u64 = 0;
pub const MPAM0_EL1_PARTID_I: u64 = 0;
pub const MPAM0_EL1_PMG_D: u64 = 0;
pub const MPAM0_EL1_PMG_I: u64 = 0;

#[inline]
pub unsafe fn __mpam_regval(partid_d: u16, partid_i: u16, pmg_d: u8, pmg_i: u8) -> u64 {
    FIELD_PREP(MPAM0_EL1_PARTID_D, partid_d as u64)
        | FIELD_PREP(MPAM0_EL1_PARTID_I, partid_i as u64)
        | FIELD_PREP(MPAM0_EL1_PMG_D, pmg_d as u64)
        | FIELD_PREP(MPAM0_EL1_PMG_I, pmg_i as u64)
}

#[inline]
pub unsafe fn mpam_set_cpu_defaults(
    cpu: i32,
    partid_d: u16,
    partid_i: u16,
    pmg_d: u8,
    pmg_i: u8,
) {
    let default_val = __mpam_regval(partid_d, partid_i, pmg_d, pmg_i);
    // WRITE_ONCE(per_cpu(arm64_mpam_default, cpu), default_val);
    let _ = (cpu, default_val);
}

#[inline]
pub unsafe fn mpam_get_regval(tsk: *mut TaskStruct) -> u64 {
    // READ_ONCE(task_thread_info(tsk)->mpam_partid_pmg)
    core::ptr::read_volatile(&(*task_thread_info(tsk)).mpam_partid_pmg)
}

#[inline]
pub unsafe fn mpam_set_task_partid_pmg(
    tsk: *mut TaskStruct,
    partid_d: u16,
    partid_i: u16,
    pmg_d: u8,
    pmg_i: u8,
) {
    let regval = __mpam_regval(partid_d, partid_i, pmg_d, pmg_i);
    core::ptr::write_volatile(&mut (*task_thread_info(tsk)).mpam_partid_pmg, regval);
}

#[inline]
pub unsafe fn mpam_thread_switch(tsk: *mut TaskStruct) {
    let oldregval: u64;
    let cpu = smp_processor_id();
    let mut regval = mpam_get_regval(tsk);

    if !static_branch_likely(&mpam_enabled) {
        return;
    }

    if regval == core::ptr::read_volatile(&arm64_mpam_global_default) {
        // regval = READ_ONCE(per_cpu(arm64_mpam_default, cpu));
        let _ = cpu;
    }

    // oldregval = READ_ONCE(per_cpu(arm64_mpam_current, cpu));
    oldregval = core::ptr::read_volatile(&arm64_mpam_current);
    if oldregval == regval {
        return;
    }

    write_sysreg_s(regval | MPAM1_EL1_MPAMEN, SYS_MPAM1_EL1);
    if system_supports_sme() {
        write_sysreg_s(
            regval & (MPAMSM_EL1_PARTID_D | MPAMSM_EL1_PMG_D),
            SYS_MPAMSM_EL1,
        );
    }
    isb();

    /* Synchronising the EL0 write is left until the ERET to EL0 */
    write_sysreg_s(regval, SYS_MPAM0_EL1);

    core::ptr::write_volatile(&mut arm64_mpam_current, regval);
}

// When CONFIG_ARM64_MPAM is disabled, mpam_thread_switch is an empty inline function.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
