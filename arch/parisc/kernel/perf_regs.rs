// SPDX-License-Identifier: GPL-2.0

/* Copyright (C) 2025 by Helge Deller <deller@gmx.de> */

// C dependencies: linux/perf_event.h, linux/perf_regs.h, and asm/ptrace.h.

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    match idx {
        PERF_REG_PARISC_R0..=PERF_REG_PARISC_R31 => {
            (*regs).gr[(idx - PERF_REG_PARISC_R0) as usize]
        }
        PERF_REG_PARISC_SR0..=PERF_REG_PARISC_SR7 => {
            (*regs).sr[(idx - PERF_REG_PARISC_SR0) as usize]
        }
        PERF_REG_PARISC_IASQ0..=PERF_REG_PARISC_IASQ1 => {
            (*regs).iasq[(idx - PERF_REG_PARISC_IASQ0) as usize]
        }
        PERF_REG_PARISC_IAOQ0..=PERF_REG_PARISC_IAOQ1 => {
            (*regs).iasq[(idx - PERF_REG_PARISC_IAOQ0) as usize]
        }
        PERF_REG_PARISC_SAR => (*regs).sar, // CR11
        PERF_REG_PARISC_IIR => (*regs).iir, // CR19
        PERF_REG_PARISC_ISR => (*regs).isr, // CR20
        PERF_REG_PARISC_IOR => (*regs).ior, // CR21
        PERF_REG_PARISC_IPSW => (*regs).ipsw, // CR22
        _ => {
            unsafe { WARN_ON_ONCE((idx as u32) >= PERF_REG_PARISC_MAX) };
            0
        }
    }
}

const REG_RESERVED: u64 = !((1u64 << PERF_REG_PARISC_MAX) - 1);

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || (mask & REG_RESERVED) != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn perf_reg_abi(task: *mut task_struct) -> u64 {
    // Build-time CONFIG_64BIT condition from IS_ENABLED(CONFIG_64BIT).
    if !cfg!(CONFIG_64BIT) {
        return PERF_SAMPLE_REGS_ABI_32;
    }

    if unsafe { test_tsk_thread_flag(task, TIF_32BIT) } {
        return PERF_SAMPLE_REGS_ABI_32;
    }

    PERF_SAMPLE_REGS_ABI_64
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    (*regs_user).regs = unsafe { task_pt_regs(current) };
    (*regs_user).abi = unsafe { perf_reg_abi(current) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
