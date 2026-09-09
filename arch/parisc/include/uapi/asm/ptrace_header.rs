/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* written by Philipp Rumpf, Copyright (C) 1999 SuSE GmbH Nuernberg
** Copyright (C) 2000 Grant Grundler, Hewlett-Packard
*/

/* Dependency: __u64 is supplied by the Linux types translation. */

/* This struct defines the way the registers are stored on the
 * stack during a system call.
 *
 * N.B. gdb/strace care about the size and offsets within this
 * structure. If you change things, you may break object compatibility
 * for those applications.
 *
 * Please do NOT use this structure for future programs, but use
 * user_regs_struct (see below) instead.
 *
 * It can be accessed through PTRACE_PEEKUSR/PTRACE_POKEUSR only.
 */
#[repr(C)]
pub struct pt_regs {
    pub gr: [core::ffi::c_ulong; 32], /* PSW is in gr[0] */
    pub fr: [u64; 32],
    pub sr: [core::ffi::c_ulong; 8],
    pub iasq: [core::ffi::c_ulong; 2],
    pub iaoq: [core::ffi::c_ulong; 2],
    pub cr27: core::ffi::c_ulong,
    pub pad0: core::ffi::c_ulong, /* available for other uses */
    pub orig_r28: core::ffi::c_ulong,
    pub ksp: core::ffi::c_ulong,
    pub kpc: core::ffi::c_ulong,
    pub sar: core::ffi::c_ulong, /* CR11 */
    pub iir: core::ffi::c_ulong, /* CR19 */
    pub isr: core::ffi::c_ulong, /* CR20 */
    pub ior: core::ffi::c_ulong, /* CR21 */
    pub ipsw: core::ffi::c_ulong, /* CR22 */
}

/**
 * struct user_regs_struct - User general purpose registers
 *
 * This is the user-visible general purpose register state structure
 * which is used to define the elf_gregset_t.
 *
 * It can be accessed through PTRACE_GETREGSET with NT_PRSTATUS
 * and through PTRACE_GETREGS.
 */
#[repr(C)]
pub struct user_regs_struct {
    pub gr: [core::ffi::c_ulong; 32], /* PSW is in gr[0] */
    pub sr: [core::ffi::c_ulong; 8],
    pub iaoq: [core::ffi::c_ulong; 2],
    pub iasq: [core::ffi::c_ulong; 2],
    pub sar: core::ffi::c_ulong, /* CR11 */
    pub iir: core::ffi::c_ulong, /* CR19 */
    pub isr: core::ffi::c_ulong, /* CR20 */
    pub ior: core::ffi::c_ulong, /* CR21 */
    pub ipsw: core::ffi::c_ulong, /* CR22 */
    pub cr0: core::ffi::c_ulong,
    pub cr24: core::ffi::c_ulong,
    pub cr25: core::ffi::c_ulong,
    pub cr26: core::ffi::c_ulong,
    pub cr27: core::ffi::c_ulong,
    pub cr28: core::ffi::c_ulong,
    pub cr29: core::ffi::c_ulong,
    pub cr30: core::ffi::c_ulong,
    pub cr31: core::ffi::c_ulong,
    pub cr8: core::ffi::c_ulong,
    pub cr9: core::ffi::c_ulong,
    pub cr12: core::ffi::c_ulong,
    pub cr13: core::ffi::c_ulong,
    pub cr10: core::ffi::c_ulong,
    pub cr15: core::ffi::c_ulong,
    pub _pad: [core::ffi::c_ulong; 80 - 64], /* pad to ELF_NGREG (80) */
}

/**
 * struct user_fp_struct - User floating point registers
 *
 * This is the user-visible floating point register state structure.
 * It uses the same layout and size as elf_fpregset_t.
 *
 * It can be accessed through PTRACE_GETREGSET with NT_PRFPREG
 * and through PTRACE_GETFPREGS.
 */
#[repr(C)]
pub struct user_fp_struct {
    pub fr: [u64; 32],
}

/*
 * The numbers chosen here are somewhat arbitrary but absolutely MUST
 * not overlap with any of the number assigned in <linux/ptrace.h>.
 *
 * These ones are taken from IA-64 on the assumption that theirs are
 * the most correct (and we also want to support PTRACE_SINGLEBLOCK
 * since we have taken branch traps too)
 */
pub const PTRACE_SINGLEBLOCK: i32 = 12; /* resume execution until next branch */

pub const PTRACE_GETREGS: i32 = 18;
pub const PTRACE_SETREGS: i32 = 19;
pub const PTRACE_GETFPREGS: i32 = 14;
pub const PTRACE_SETFPREGS: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
