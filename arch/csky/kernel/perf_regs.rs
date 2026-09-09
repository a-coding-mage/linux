// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the Linux kernel and architecture headers:
// linux/errno.h, linux/kernel.h, linux/perf_event.h, linux/bug.h,
// asm/perf_regs.h, asm/ptrace.h

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
	if WARN_ON_ONCE((idx as u32) >= PERF_REG_CSKY_MAX) {
		return 0;
	}

	*((regs as *const u32).add(idx as usize)) as u64
}

pub const REG_RESERVED: u64 = !((1u64 << PERF_REG_CSKY_MAX) - 1);

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
