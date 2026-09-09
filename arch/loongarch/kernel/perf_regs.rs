// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 2013 Cavium, Inc.
 */

// Dependency: linux/perf_event.h
// Dependency: asm/ptrace.h

// CONFIG_32BIT and CONFIG_64BIT are build-time configuration conditions.
#[cfg(CONFIG_32BIT)]
pub unsafe fn perf_reg_abi(_tsk: *mut task_struct) -> u64 {
    PERF_SAMPLE_REGS_ABI_32
}

// Must be CONFIG_64BIT.
#[cfg(not(CONFIG_32BIT))]
pub unsafe fn perf_reg_abi(tsk: *mut task_struct) -> u64 {
    if test_tsk_thread_flag(tsk, TIF_32BIT_REGS) {
        PERF_SAMPLE_REGS_ABI_32
    } else {
        PERF_SAMPLE_REGS_ABI_64
    }
}

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 {
        return -EINVAL;
    }
    if (mask & !((1u64 << PERF_REG_LOONGARCH_MAX) - 1)) != 0 {
        return -EINVAL;
    }
    0
}

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    if WARN_ON_ONCE((idx as u32) >= PERF_REG_LOONGARCH_MAX) {
        return 0;
    }

    if (idx as u32) == PERF_REG_LOONGARCH_PC {
        return (*regs).csr_era;
    }

    (*regs).regs[idx as usize]
}

pub unsafe fn perf_get_regs_user(
    regs_user: *mut perf_regs,
    regs: *mut pt_regs,
) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
