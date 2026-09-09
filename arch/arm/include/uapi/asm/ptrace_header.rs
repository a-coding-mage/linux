/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  arch/arm/include/asm/ptrace.h
 *
 *  Copyright (C) 1996-2003 Russell King
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependency: <asm/hwcap.h>

pub const PTRACE_GETREGS: i32 = 12;
pub const PTRACE_SETREGS: i32 = 13;
pub const PTRACE_GETFPREGS: i32 = 14;
pub const PTRACE_SETFPREGS: i32 = 15;
// PTRACE_ATTACH is 16
// PTRACE_DETACH is 17
pub const PTRACE_GETWMMXREGS: i32 = 18;
pub const PTRACE_SETWMMXREGS: i32 = 19;
// 20 is unused
pub const PTRACE_OLDSETOPTIONS: i32 = 21;
pub const PTRACE_GET_THREAD_AREA: i32 = 22;
pub const PTRACE_SET_SYSCALL: i32 = 23;
// PTRACE_SYSCALL is 24
pub const PTRACE_GETCRUNCHREGS: i32 = 25; // obsolete
pub const PTRACE_SETCRUNCHREGS: i32 = 26; // obsolete
pub const PTRACE_GETVFPREGS: i32 = 27;
pub const PTRACE_SETVFPREGS: i32 = 28;
pub const PTRACE_GETHBPREGS: i32 = 29;
pub const PTRACE_SETHBPREGS: i32 = 30;
pub const PTRACE_GETFDPIC: i32 = 31;

pub const PTRACE_GETFDPIC_EXEC: i32 = 0;
pub const PTRACE_GETFDPIC_INTERP: i32 = 1;

/* PSR bits. On V7M there is no mode contained in the PSR. */
pub const USR26_MODE: u32 = 0x00000000;
pub const FIQ26_MODE: u32 = 0x00000001;
pub const IRQ26_MODE: u32 = 0x00000002;
pub const SVC26_MODE: u32 = 0x00000003;
// __KERNEL__ && CONFIG_CPU_V7M selects the alternate values below.
pub const USR_MODE: u32 = 0x00000010;
pub const SVC_MODE: u32 = 0x00000013;
pub const FIQ_MODE: u32 = 0x00000011;
pub const IRQ_MODE: u32 = 0x00000012;
pub const MON_MODE: u32 = 0x00000016;
pub const ABT_MODE: u32 = 0x00000017;
pub const HYP_MODE: u32 = 0x0000001a;
pub const UND_MODE: u32 = 0x0000001b;
pub const SYSTEM_MODE: u32 = 0x0000001f;
pub const MODE32_BIT: u32 = 0x00000010;
pub const MODE_MASK: u32 = 0x0000001f;

pub const V4_PSR_T_BIT: u32 = 0x00000020; // >= V4T, but not V7M
pub const V7M_PSR_T_BIT: u32 = 0x01000000;
// __KERNEL__ && CONFIG_CPU_V7M selects V7M_PSR_T_BIT; compatibility value:
pub const PSR_T_BIT: u32 = V4_PSR_T_BIT;

pub const PSR_F_BIT: u32 = 0x00000040;
pub const PSR_I_BIT: u32 = 0x00000080;
pub const PSR_A_BIT: u32 = 0x00000100;
pub const PSR_E_BIT: u32 = 0x00000200;
pub const PSR_J_BIT: u32 = 0x01000000;
pub const PSR_Q_BIT: u32 = 0x08000000;
pub const PSR_V_BIT: u32 = 0x10000000;
pub const PSR_C_BIT: u32 = 0x20000000;
pub const PSR_Z_BIT: u32 = 0x40000000;
pub const PSR_N_BIT: u32 = 0x80000000;

pub const PSR_f: u32 = 0xff000000;
pub const PSR_s: u32 = 0x00ff0000;
pub const PSR_x: u32 = 0x0000ff00;
pub const PSR_c: u32 = 0x000000ff;

pub const APSR_MASK: u32 = 0xf80f0000;
pub const PSR_ISET_MASK: u32 = 0x01000010;
pub const PSR_IT_MASK: u32 = 0x0600fc00;
pub const PSR_ENDIAN_MASK: u32 = 0x00000200;

/* Magic values for PTRACE_PEEKUSR. */
pub const PT_TEXT_ADDR: u32 = 0x10000;
pub const PT_DATA_ADDR: u32 = 0x10004;
pub const PT_TEXT_END_ADDR: u32 = 0x10008;

// The following declarations are excluded when compiling assembly. The
// pt_regs declaration is excluded for kernel builds.
#[repr(C)]
pub struct pt_regs {
    pub uregs: [core::ffi::c_long; 18],
}

macro_rules! ARM_cpsr { ($regs:expr) => { $regs.uregs[16] }; }
macro_rules! ARM_pc { ($regs:expr) => { $regs.uregs[15] }; }
macro_rules! ARM_lr { ($regs:expr) => { $regs.uregs[14] }; }
macro_rules! ARM_sp { ($regs:expr) => { $regs.uregs[13] }; }
macro_rules! ARM_ip { ($regs:expr) => { $regs.uregs[12] }; }
macro_rules! ARM_fp { ($regs:expr) => { $regs.uregs[11] }; }
macro_rules! ARM_r10 { ($regs:expr) => { $regs.uregs[10] }; }
macro_rules! ARM_r9 { ($regs:expr) => { $regs.uregs[9] }; }
macro_rules! ARM_r8 { ($regs:expr) => { $regs.uregs[8] }; }
macro_rules! ARM_r7 { ($regs:expr) => { $regs.uregs[7] }; }
macro_rules! ARM_r6 { ($regs:expr) => { $regs.uregs[6] }; }
macro_rules! ARM_r5 { ($regs:expr) => { $regs.uregs[5] }; }
macro_rules! ARM_r4 { ($regs:expr) => { $regs.uregs[4] }; }
macro_rules! ARM_r3 { ($regs:expr) => { $regs.uregs[3] }; }
macro_rules! ARM_r2 { ($regs:expr) => { $regs.uregs[2] }; }
macro_rules! ARM_r1 { ($regs:expr) => { $regs.uregs[1] }; }
macro_rules! ARM_r0 { ($regs:expr) => { $regs.uregs[0] }; }
macro_rules! ARM_ORIG_r0 { ($regs:expr) => { $regs.uregs[17] }; }

pub const ARM_VFPREGS_SIZE: usize = 32 * 8 + 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
