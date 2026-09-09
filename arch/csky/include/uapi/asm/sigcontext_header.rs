/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by <asm/ptrace.h> in the C header.

#[repr(C)]
pub struct sigcontext {
    pub sc_pt_regs: pt_regs,
    pub sc_user_fp: user_fp,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
