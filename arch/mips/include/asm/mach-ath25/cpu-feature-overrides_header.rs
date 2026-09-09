/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Atheros AR231x/AR531x SoC specific CPU feature overrides
 *
 *  Copyright (C) 2008 Gabor Juhos <juhosg@openwrt.org>
 *
 *  This file was derived from: include/asm-mips/cpu-features.h
 *	Copyright (C) 2003, 2004 Ralf Baechle
 *	Copyright (C) 2004 Maciej W. Rozycki
 */

/*
 * The Atheros AR531x/AR231x SoCs have MIPS 4Kc/4KEc core.
 */
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_sb1_cache: i32 = 0;
pub const cpu_has_fpu: i32 = 0;
pub const cpu_has_32fpr: i32 = 0;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_ejtag: i32 = 1;

/* CONFIG_SOC_AR5312 is a build-time condition; preserve the source conditional intent. */
#[cfg(not(feature = "CONFIG_SOC_AR5312"))]
pub const cpu_has_llsc: i32 = 1;

/*
 * The MIPS 4Kc V0.9 core in the AR5312/AR2312 have problems with the
 * ll/sc instructions.
 */
#[cfg(feature = "CONFIG_SOC_AR5312")]
pub const cpu_has_llsc: i32 = 0;

pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;

pub const cpu_has_mips32r1: i32 = 1;

#[cfg(not(feature = "CONFIG_SOC_AR5312"))]
pub const cpu_has_mips32r2: i32 = 1;

pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;

pub const cpu_has_64bits: i32 = 0;
pub const cpu_has_64bit_zero_reg: i32 = 0;
pub const cpu_has_64bit_gp_regs: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
