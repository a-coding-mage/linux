/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/ptrace.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/* Dependency: linux/types.h supplies the C __u32 type. */

/* Registers used by strace */

pub const REG_A_BASE: u32 = 0x0000;
pub const REG_AR_BASE: u32 = 0x0100;
pub const REG_PC: u32 = 0x0020;
pub const REG_PS: u32 = 0x02e6;
pub const REG_WB: u32 = 0x0248;
pub const REG_WS: u32 = 0x0249;
pub const REG_LBEG: u32 = 0x0200;
pub const REG_LEND: u32 = 0x0201;
pub const REG_LCOUNT: u32 = 0x0202;
pub const REG_SAR: u32 = 0x0203;

pub const SYSCALL_NR: u32 = 0x00ff;

/* Other PTRACE_ values defined in <linux/ptrace.h> using values 0-9,16,17,24 */

pub const PTRACE_GETREGS: u32 = 12;
pub const PTRACE_SETREGS: u32 = 13;
pub const PTRACE_GETXTREGS: u32 = 18;
pub const PTRACE_SETXTREGS: u32 = 19;
pub const PTRACE_GETHBPREGS: u32 = 20;
pub const PTRACE_SETHBPREGS: u32 = 21;
pub const PTRACE_GETFDPIC: u32 = 22;

pub const PTRACE_GETFDPIC_EXEC: u32 = 0;
pub const PTRACE_GETFDPIC_INTERP: u32 = 1;

/* The declarations below are excluded when compiling as an assembler source. */
#[repr(C)]
pub struct user_pt_regs {
	pub pc: u32,
	pub ps: u32,
	pub lbeg: u32,
	pub lend: u32,
	pub lcount: u32,
	pub sar: u32,
	pub windowstart: u32,
	pub windowbase: u32,
	pub threadptr: u32,
	pub syscall: u32,
	pub reserved: [u32; 6 + 48],
	pub a: [u32; 64],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
