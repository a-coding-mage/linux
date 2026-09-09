/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The C header exposes these declarations only when __ASSEMBLER__ is not
 * defined.  This Rust translation is likewise intended for non-assembler use.
 * The ioctl values encode _IOWR('c', nr, __u32[8]); __u32[8] is 32 bytes.
 */

pub const X86_IOC_RDMSR_REGS: u32 = 0xC02063A0;
pub const X86_IOC_WRMSR_REGS: u32 = 0xC02063A1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
