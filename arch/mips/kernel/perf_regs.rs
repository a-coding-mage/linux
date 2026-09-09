// SPDX-License-Identifier: GPL-2.0
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Some parts derived from x86 version of this file.
 *
 * Copyright (C) 2013 Cavium, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_32BIT)]
pub unsafe fn perf_reg_abi(_tsk: *mut task_struct) -> u64 {
    PERF_SAMPLE_REGS_ABI_32
}

#[cfg(not(CONFIG_32BIT))]
pub unsafe fn perf_reg_abi(tsk: *mut task_struct) -> u64 {
    // Must be CONFIG_64BIT.
    if test_tsk_thread_flag(tsk, TIF_32BIT_REGS) {
        PERF_SAMPLE_REGS_ABI_32
    } else {
        PERF_SAMPLE_REGS_ABI_64
    }
}

pub unsafe fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 {
        return -EINVAL;
    }
    if (mask & !((1u64 << PERF_REG_MIPS_MAX) - 1)) != 0 {
        return -EINVAL;
    }
    0
}

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    let mut v: i64;

    match idx {
        PERF_REG_MIPS_PC => {
            v = (*regs).cp0_epc as i64;
        }
        PERF_REG_MIPS_R1..=PERF_REG_MIPS_R25 => {
            v = (*regs).regs[(idx - PERF_REG_MIPS_R1 + 1) as usize] as i64;
        }
        PERF_REG_MIPS_R28..=PERF_REG_MIPS_R31 => {
            v = (*regs).regs[(idx - PERF_REG_MIPS_R28 + 28) as usize] as i64;
        }
        _ => {
            WARN_ON_ONCE(1);
            return 0;
        }
    }

    // Sign extend if 32-bit.
    v as u64
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
