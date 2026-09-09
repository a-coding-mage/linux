/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1999 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

/* scalar FP context was used */
pub const USED_FP: u32 = 1 << 0;

/* the value of Status.FR when context was saved */
pub const USED_FR1: u32 = 1 << 1;

/* FR=1, but with odd singles in bits 63:32 of preceding even double */
pub const USED_HYBRID_FPRS: u32 = 1 << 2;

/* extended context was used, see struct extcontext for details */
pub const USED_EXTCONTEXT: u32 = 1 << 3;

/*
 * The original conditions compare _MIPS_SIM against ABI-specific C macros.
 * They are represented here by the corresponding Rust configuration names.
 */
#[cfg(mips_sim_abi32)]
#[repr(C)]
pub struct sigcontext {
    pub sc_regmask: u32,       /* Unused */
    pub sc_status: u32,        /* Unused */
    pub sc_pc: u64,
    pub sc_regs: [u64; 32],
    pub sc_fpregs: [u64; 32],
    pub sc_acx: u32,            /* Was sc_ownedfp */
    pub sc_fpc_csr: u32,
    pub sc_fpc_eir: u32,       /* Unused */
    pub sc_used_math: u32,
    pub sc_dsp: u32,            /* dsp status, was sc_ssflags */
    pub sc_mdhi: u64,
    pub sc_mdlo: u64,
    pub sc_hi1: u32,             /* Was sc_cause */
    pub sc_lo1: u32,             /* Was sc_badvaddr */
    pub sc_hi2: u32,             /* Was sc_sigset[4] */
    pub sc_lo2: u32,
    pub sc_hi3: u32,
    pub sc_lo3: u32,
}

#[cfg(any(mips_sim_abi64, mips_sim_nabi32))]
#[repr(C)]
pub struct sigcontext {
    pub sc_regs: [u64; 32],
    pub sc_fpregs: [u64; 32],
    pub sc_mdhi: u64,
    pub sc_hi1: u64,
    pub sc_hi2: u64,
    pub sc_hi3: u64,
    pub sc_mdlo: u64,
    pub sc_lo1: u64,
    pub sc_lo2: u64,
    pub sc_lo3: u64,
    pub sc_pc: u64,
    pub sc_fpc_csr: u32,
    pub sc_used_math: u32,
    pub sc_dsp: u32,
    pub sc_reserved: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
