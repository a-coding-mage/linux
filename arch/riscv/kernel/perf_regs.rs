// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd. */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_regs {
    pub regs: *mut pt_regs,
    pub abi: u64,
}

// PERF_REG_RISCV_MAX, PERF_SAMPLE_REGS_ABI_64, and PERF_SAMPLE_REGS_ABI_32
// are supplied by the corresponding architecture headers.
extern "C" {
    static mut current: *mut task_struct;
    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
}

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    if WARN_ON_ONCE((idx as u32) >= PERF_REG_RISCV_MAX as u32) {
        return 0;
    }

    *(regs as *mut usize).add(idx as usize) as u64
}

const REG_RESERVED: u64 = !((1u64 << PERF_REG_RISCV_MAX) - 1);

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || mask & REG_RESERVED != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn perf_reg_abi(_task: *mut task_struct) -> u64 {
    // Preserve the build-time architecture condition from the C source.
    #[cfg(target_pointer_width = "64")]
    {
        PERF_SAMPLE_REGS_ABI_64
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        PERF_SAMPLE_REGS_ABI_32
    }
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, _regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
