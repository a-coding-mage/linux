/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/sched/task_stack.h.

#[repr(C)]
pub struct perf_regs {
	pub abi: u64,
	pub regs: *mut pt_regs,
}

// CONFIG_HAVE_PERF_REGS is a build-time condition supplied by the surrounding
// kernel configuration.  The declarations below correspond to the enabled
// configuration; the fallback implementations are provided otherwise.
#[cfg(CONFIG_HAVE_PERF_REGS)]
extern "C" {
	pub fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64;
	pub fn perf_reg_validate(mask: u64) -> i32;
	pub fn perf_reg_abi(task: *mut task_struct) -> u64;
	pub fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs);
}

#[cfg(not(CONFIG_HAVE_PERF_REGS))]
pub const PERF_REG_EXTENDED_MASK: u64 = 0;

#[cfg(not(CONFIG_HAVE_PERF_REGS))]
#[inline]
pub unsafe fn perf_reg_value(_regs: *mut pt_regs, _idx: i32) -> u64 {
	0
}

#[cfg(not(CONFIG_HAVE_PERF_REGS))]
#[inline]
pub unsafe fn perf_reg_validate(mask: u64) -> i32 {
	if mask != 0 { -ENOSYS } else { 0 }
}

#[cfg(not(CONFIG_HAVE_PERF_REGS))]
#[inline]
pub unsafe fn perf_reg_abi(_task: *mut task_struct) -> u64 {
	PERF_SAMPLE_REGS_ABI_NONE
}

#[cfg(not(CONFIG_HAVE_PERF_REGS))]
#[inline]
pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, _regs: *mut pt_regs) {
	(*regs_user).regs = task_pt_regs(current);
	(*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
