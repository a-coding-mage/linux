/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  IDT RC32434 specific CPU feature overrides
 *
 *  Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 *
 *  This file was derived from: include/asm-mips/cpu-features.h
 *	Copyright (C) 2003, 2004 Ralf Baechle
 *	Copyright (C) 2004 Maciej W. Rozycki
 */

/*
 * The IDT RC32434 SOC has a built-in MIPS 4Kc core.
 */
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_sb1_cache: i32 = 0;
pub const cpu_has_fpu: i32 = 0;
pub const cpu_has_32fpr: i32 = 0;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_watch: i32 = 1;
pub const cpu_has_divec: i32 = 1;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_prefetch: i32 = 1;
pub const cpu_has_mcheck: i32 = 1;
pub const cpu_has_ejtag: i32 = 1;
pub const cpu_has_llsc: i32 = 1;

pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;

pub const cpu_has_vtag_icache: i32 = 0;

pub const cpu_has_mips32r1: i32 = 1;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;

/* #define cpu_has_nofpuex		? */
pub const cpu_has_64bits: i32 = 0;
pub const cpu_has_64bit_zero_reg: i32 = 0;
pub const cpu_has_64bit_gp_regs: i32 = 0;

pub const cpu_has_inclusive_pcaches: i32 = 0;

#[inline]
pub const fn cpu_dcache_line_size() -> i32 {
    16
}

#[inline]
pub const fn cpu_icache_line_size() -> i32 {
    16
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
