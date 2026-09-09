// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016 Anju T, IBM Corporation.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut PERF_REG_EXTENDED_MASK: u64;

    fn get_pmcs_ext_regs(idx: i32) -> u64;
    fn mfspr(spr: u32) -> u64;
    fn is_sier_available() -> bool;
    fn regs_get_register(regs: *mut pt_regs, offset: usize) -> u64;
    fn is_tsk_32bit_task(task: *mut task_struct) -> bool;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    static mut current: *mut task_struct;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

#[repr(C)]
pub struct pt_regs {
    pub gpr: [u64; 32],
    pub nip: u64,
    pub msr: u64,
    pub orig_gpr3: u64,
    pub ctr: u64,
    pub link: u64,
    pub xer: u64,
    pub ccr: u64,
    pub softe: u64,
    pub mq: u64,
    pub trap: u64,
    pub dar: u64,
    pub dsisr: u64,
}

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct perf_regs {
    pub regs: *mut pt_regs,
    pub abi: u64,
}

// Values are supplied by asm/perf_regs.h in the complete kernel translation.
extern "C" {
    static PERF_REG_POWERPC_MAX: usize;
    static PERF_REG_EXTENDED_MAX: i32;
    static PERF_REG_POWERPC_R0: usize;
    static PERF_REG_POWERPC_PMC1: i32;
    static PERF_REG_POWERPC_PMC6: i32;
    static PERF_REG_POWERPC_MMCR0: i32;
    static PERF_REG_POWERPC_MMCR1: i32;
    static PERF_REG_POWERPC_MMCR2: i32;
    static PERF_REG_POWERPC_MMCR3: i32;
    static PERF_REG_POWERPC_SIER: i32;
    static PERF_REG_POWERPC_SIER2: i32;
    static PERF_REG_POWERPC_SIER3: i32;
    static PERF_REG_POWERPC_SDAR: i32;
    static PERF_REG_POWERPC_SIAR: i32;
    static PERF_REG_POWERPC_MMCRA: i32;
    static SPRN_MMCR0: u32;
    static SPRN_MMCR1: u32;
    static SPRN_MMCR2: u32;
    static SPRN_MMCR3: u32;
    static SPRN_SIER2: u32;
    static SPRN_SIER3: u32;
    static SPRN_SDAR: u32;
    static SPRN_SIAR: u32;
    static PERF_SAMPLE_REGS_ABI_32: u64;
    static PERF_SAMPLE_REGS_ABI_64: u64;
    static PERF_SAMPLE_REGS_ABI_NONE: u64;
}

// CONFIG_PPC64 selects `softe`; otherwise the source selects `mq`.
pub static mut pt_regs_offset: [u32; 48] = [0; 48];

unsafe fn get_ext_regs_value(idx: i32) -> u64 {
    if idx >= PERF_REG_POWERPC_PMC1 && idx <= PERF_REG_POWERPC_PMC6 {
        return get_pmcs_ext_regs(idx - PERF_REG_POWERPC_PMC1);
    }
    match idx {
        x if x == PERF_REG_POWERPC_MMCR0 => mfspr(SPRN_MMCR0),
        x if x == PERF_REG_POWERPC_MMCR1 => mfspr(SPRN_MMCR1),
        x if x == PERF_REG_POWERPC_MMCR2 => mfspr(SPRN_MMCR2),
        // CONFIG_PPC64-only cases from the C source.
        x if x == PERF_REG_POWERPC_MMCR3 => mfspr(SPRN_MMCR3),
        x if x == PERF_REG_POWERPC_SIER2 => mfspr(SPRN_SIER2),
        x if x == PERF_REG_POWERPC_SIER3 => mfspr(SPRN_SIER3),
        x if x == PERF_REG_POWERPC_SDAR => mfspr(SPRN_SDAR),
        x if x == PERF_REG_POWERPC_SIAR => mfspr(SPRN_SIAR),
        _ => 0,
    }
}

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    // IS_ENABLED(CONFIG_FSL_EMB_PERF_EVENT) || IS_ENABLED(CONFIG_PPC32)
    // is represented by the corresponding build configuration in the kernel.
    if idx == PERF_REG_POWERPC_SIER && !is_sier_available() {
        return 0;
    }
    if idx >= PERF_REG_POWERPC_MAX as i32 && idx < PERF_REG_EXTENDED_MAX {
        return get_ext_regs_value(idx);
    }
    if WARN_ON_ONCE(idx >= PERF_REG_EXTENDED_MAX) {
        return 0;
    }
    regs_get_register(regs, pt_regs_offset[idx as usize] as usize)
}

pub unsafe fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || (mask & !(PERF_REG_EXTENDED_MASK | 0)) != 0 {
        return -22; // -EINVAL
    }
    0
}

pub unsafe fn perf_reg_abi(task: *mut task_struct) -> u64 {
    if is_tsk_32bit_task(task) {
        PERF_SAMPLE_REGS_ABI_32
    } else {
        PERF_SAMPLE_REGS_ABI_64
    }
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = if !(*regs_user).regs.is_null() {
        perf_reg_abi(current)
    } else {
        PERF_SAMPLE_REGS_ABI_NONE
    };
    let _ = regs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
