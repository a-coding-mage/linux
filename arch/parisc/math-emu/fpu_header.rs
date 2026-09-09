/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */

/* _MACHINE_FPU_INCLUDED allows multiple inclusion. */

pub const PA83_FPU_FLAG: u32 = 0x00000001;
pub const PA89_FPU_FLAG: u32 = 0x00000002;
pub const PA2_0_FPU_FLAG: u32 = 0x00000010;

pub const TIMEX_EXTEN_FLAG: u32 = 0x00000004;

pub const ROLEX_EXTEN_FLAG: u32 = 0x00000008;
pub const COPR_FP: u32 = 0x00000080; /* Floating point -- Coprocessor 0 */
pub const SFU_MPY_DIVIDE: u32 = 0x00008000; /* Multiply/Divide __ SFU 0 */

pub const EM_FPU_TYPE_OFFSET: u32 = 272;

/* version of EMULATION software for COPR,0,0 instruction */
pub const EMULATION_VERSION: u32 = 4;

/*
 * The only way to differentiate between TIMEX and ROLEX (or PCX-S and PCX-T)
 * is through the potential type field from the PDC_MODEL call.
 * The following flags are used to assist this differentiation.
 */

pub const ROLEX_POTENTIAL_KEY_FLAGS: u32 = PDC_MODEL_CPU_KEY_WORD_TO_IO;
pub const TIMEX_POTENTIAL_KEY_FLAGS: u32 =
    PDC_MODEL_CPU_KEY_QUAD_STORE | PDC_MODEL_CPU_KEY_RECIP_SQRT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
