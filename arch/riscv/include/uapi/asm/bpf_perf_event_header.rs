/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `user_regs_struct` is supplied by the translated ptrace header.
#[allow(non_camel_case_types)]
pub type bpf_user_pt_regs_t = user_regs_struct;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
