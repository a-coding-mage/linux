/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/asm/ptrace_32.h>.

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, tra);

pub unsafe fn regs_return_value(regs: *mut pt_regs) -> libc::c_long {
    (*regs).regs[0]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
