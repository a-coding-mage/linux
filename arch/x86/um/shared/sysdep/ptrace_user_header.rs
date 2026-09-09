/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by generated/user_constants.h in the C source:
// HOST_ORIG_AX, HOST_AX, HOST_IP, and HOST_SP.

/// Equivalent of `PT_OFFSET(r) ((r) * sizeof(long))`.
#[inline]
pub const fn pt_offset(r: usize) -> usize {
    r * core::mem::size_of::<core::ffi::c_long>()
}

/// Equivalent of `PT_SYSCALL_NR(regs) ((regs)[HOST_ORIG_AX])`.
#[macro_export]
macro_rules! pt_syscall_nr {
    ($regs:expr) => {
        ($regs)[HOST_ORIG_AX]
    };
}

/// Equivalent of `PT_SYSCALL_NR_OFFSET PT_OFFSET(HOST_ORIG_AX)`.
pub const PT_SYSCALL_NR_OFFSET: usize = pt_offset(HOST_ORIG_AX);

/// Equivalent of `PT_SYSCALL_RET_OFFSET PT_OFFSET(HOST_AX)`.
pub const PT_SYSCALL_RET_OFFSET: usize = pt_offset(HOST_AX);

pub const REGS_IP_INDEX: usize = HOST_IP;
pub const REGS_SP_INDEX: usize = HOST_SP;

/*
 * glibc before 2.27 does not include PTRACE_SYSEMU_SINGLESTEP in its enum,
 * ensure we have a definition by (re-)defining it here.
 *
 * C conditional intent: define this only when the external environment has
 * not already supplied PTRACE_SYSEMU_SINGLESTEP.
 */
pub const PTRACE_SYSEMU_SINGLESTEP: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
