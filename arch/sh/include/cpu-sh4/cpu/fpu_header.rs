/* SPDX-License-Identifier: GPL-2.0
 *
 * linux/arch/sh/kernel/cpu/sh4/sh4_fpu.h
 *
 * Copyright (C) 2006 STMicroelectronics Limited
 * Author: Carl Shaw <carl.shaw@st.com>
 *
 * Definitions for SH4 FPU operations
 */

// C header guard: __CPU_SH4_FPU_H

pub const FPSCR_ENABLE_MASK: u32 = 0x00000f80u32;

pub const FPSCR_FMOV_DOUBLE: u32 = 1u32 << 1;

pub const FPSCR_CAUSE_INEXACT: u32 = 1u32 << 12;
pub const FPSCR_CAUSE_UNDERFLOW: u32 = 1u32 << 13;
pub const FPSCR_CAUSE_OVERFLOW: u32 = 1u32 << 14;
pub const FPSCR_CAUSE_DIVZERO: u32 = 1u32 << 15;
pub const FPSCR_CAUSE_INVALID: u32 = 1u32 << 16;
pub const FPSCR_CAUSE_ERROR: u32 = 1u32 << 17;

pub const FPSCR_DBL_PRECISION: u32 = 1u32 << 19;

#[inline]
pub const fn FPSCR_ROUNDING_MODE(x: u32) -> u32 {
    (x >> 20) & 3
}

pub const FPSCR_RM_NEAREST: u32 = 0;
pub const FPSCR_RM_ZERO: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
