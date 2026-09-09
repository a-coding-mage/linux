/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1999 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// Dependency supplied by the original UAPI header: <uapi/asm/sigcontext.h>

// The original declaration is present when _MIPS_SIM is _MIPS_SIM_ABI64 or
// _MIPS_SIM_NABI32. Rust build configuration should preserve that condition.
#[repr(C)]
pub struct sigcontext32 {
	pub sc_regmask: u32, /* Unused */
	pub sc_status: u32, /* Unused */
	pub sc_pc: u64,
	pub sc_regs: [u64; 32],
	pub sc_fpregs: [u64; 32],
	pub sc_acx: u32, /* Only MIPS32; was sc_ownedfp */
	pub sc_fpc_csr: u32,
	pub sc_fpc_eir: u32, /* Unused */
	pub sc_used_math: u32,
	pub sc_dsp: u32, /* dsp status, was sc_ssflags */
	pub sc_mdhi: u64,
	pub sc_mdlo: u64,
	pub sc_hi1: u32, /* Was sc_cause */
	pub sc_lo1: u32, /* Was sc_badvaddr */
	pub sc_hi2: u32, /* Was sc_sigset[4] */
	pub sc_lo2: u32,
	pub sc_hi3: u32,
	pub sc_lo3: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
