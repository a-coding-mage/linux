/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 95, 96, 97, 98, 99, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependency: __u64 and the other Linux integer types are represented by
// their corresponding fixed-width Rust integer types.

/* 0 - 31 are integer registers, 32 - 63 are fp registers.  */
pub const FPR_BASE: i32 = 32;
pub const PC: i32 = 64;
pub const CAUSE: i32 = 65;
pub const BADVADDR: i32 = 66;
pub const MMHI: i32 = 67;
pub const MMLO: i32 = 68;
pub const FPC_CSR: i32 = 69;
pub const FPC_EIR: i32 = 70;
pub const DSP_BASE: i32 = 71; /* 3 more hi / lo register pairs */
pub const DSP_CONTROL: i32 = 77;
pub const ACX: i32 = 78;

/*
 * This struct defines the registers as used by PTRACE_{GET,SET}REGS. The
 * format is the same for both 32- and 64-bit processes. Registers for 32-bit
 * processes are sign extended.
 */
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct PtRegs {
	/* Saved main processor registers. */
	pub regs: [u64; 32],

	/* Saved special registers. */
	pub lo: u64,
	pub hi: u64,
	pub cp0_epc: u64,
	pub cp0_badvaddr: u64,
	pub cp0_status: u64,
	pub cp0_cause: u64,
}

/* The kernel spelling is selected by __KERNEL__ in the C header. */
#[cfg(feature = "kernel")]
pub type UserPtRegs = PtRegs;

/* Arbitrarily choose the same ptrace numbers as used by the Sparc code. */
pub const PTRACE_GETREGS: i32 = 12;
pub const PTRACE_SETREGS: i32 = 13;
pub const PTRACE_GETFPREGS: i32 = 14;
pub const PTRACE_SETFPREGS: i32 = 15;
/* #define PTRACE_GETFPXREGS 18 */
/* #define PTRACE_SETFPXREGS 19 */

pub const PTRACE_OLDSETOPTIONS: i32 = 21;

pub const PTRACE_GET_THREAD_AREA: i32 = 25;
pub const PTRACE_SET_THREAD_AREA: i32 = 26;

/* Calls to trace a 64bit program from a 32bit program. */
pub const PTRACE_PEEKTEXT_3264: u32 = 0xc0;
pub const PTRACE_PEEKDATA_3264: u32 = 0xc1;
pub const PTRACE_POKETEXT_3264: u32 = 0xc2;
pub const PTRACE_POKEDATA_3264: u32 = 0xc3;
pub const PTRACE_GET_THREAD_AREA_3264: u32 = 0xc4;

/* Read and write watchpoint registers. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PtWatchStyle {
	PtWatchStyleMips32,
	PtWatchStyleMips64,
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct Mips32WatchRegs {
	pub watchlo: [u32; 8],
	/* Lower 16 bits of watchhi. */
	pub watchhi: [u16; 8],
	/* Valid mask and I R W bits.
	 * bit 0 -- 1 if W bit is usable.
	 * bit 1 -- 1 if R bit is usable.
	 * bit 2 -- 1 if I bit is usable.
	 * bits 3 - 11 -- Valid watchhi mask bits.
	 */
	pub watch_masks: [u16; 8],
	/* The number of valid watch register pairs. */
	pub num_valid: u32,
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct Mips64WatchRegs {
	pub watchlo: [u64; 8],
	pub watchhi: [u16; 8],
	pub watch_masks: [u16; 8],
	pub num_valid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PtWatchRegsData {
	pub mips32: Mips32WatchRegs,
	pub mips64: Mips64WatchRegs,
}

#[repr(C)]
pub struct PtWatchRegs {
	pub style: PtWatchStyle,
	pub data: PtWatchRegsData,
}

pub const PTRACE_GET_WATCH_REGS: u32 = 0xd0;
pub const PTRACE_SET_WATCH_REGS: u32 = 0xd1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
