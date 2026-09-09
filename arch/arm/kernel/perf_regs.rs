// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// linux/errno.h, linux/kernel.h, linux/perf_event.h, linux/bug.h,
// linux/sched/task_stack.h, asm/perf_regs.h, and asm/ptrace.h.

extern "C" {
    static mut current: *mut task_struct;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

#[repr(C)]
pub struct pt_regs {
    pub uregs: [u64; PERF_REG_ARM_MAX as usize],
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

// Supplied by asm/perf_regs.h.
extern "C" {
    static PERF_REG_ARM_MAX: u32;
    static PERF_SAMPLE_REGS_ABI_32: u64;
}

const EINVAL: i32 = 22;

const REG_RESERVED: u64 = !((1u64 << PERF_REG_ARM_MAX) - 1);

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    if WARN_ON_ONCE((idx as u32) >= PERF_REG_ARM_MAX) {
        return 0;
    }

    (*regs).uregs[idx as usize]
}

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || (mask & REG_RESERVED) != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn perf_reg_abi(_task: *mut task_struct) -> u64 {
    PERF_SAMPLE_REGS_ABI_32
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, _regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
