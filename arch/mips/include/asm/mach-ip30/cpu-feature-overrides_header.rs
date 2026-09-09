/* SPDX-License-Identifier: GPL-2.0 */
/*
 * IP30/Octane cpu-features overrides.
 *
 * Copyright (C) 2003 Ralf Baechle <ralf@linux-mips.org>
 *                  2004-2007 Stanislaw Skowronek <skylark@unaligned.org>
 *                  2009 Johannes Dickgreber <tanzy@gmx.de>
 *                  2015 Joshua Kinard <linux@kumba.dev>
 *
 */

// Dependency intent: symbols from <asm/cpu.h> are supplied by other files.

/*
 * IP30 only supports R1[024]000 processors, all using the same config
 */
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_tlbinv: i32 = 0;
pub const cpu_has_segments: i32 = 0;
pub const cpu_has_eva: i32 = 0;
pub const cpu_has_htw: i32 = 0;
pub const cpu_has_rixiex: i32 = 0;
pub const cpu_has_maar: i32 = 0;
pub const cpu_has_rw_llb: i32 = 0;
pub const cpu_has_3kex: i32 = 0;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_nofpuex: i32 = 0;
pub const cpu_has_32fpr: i32 = 1;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_watch: i32 = 1;
pub const cpu_has_64bits: i32 = 1;
pub const cpu_has_divec: i32 = 0;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_prefetch: i32 = 1;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;
pub const cpu_has_llsc: i32 = 1;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_rixi: i32 = 0;
pub const cpu_has_xpa: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_dc_aliases: i32 = 0;
pub const cpu_has_ic_fills_f_dc: i32 = 0;

pub const cpu_icache_snoops_remote_store: i32 = 1;

pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;
pub const cpu_has_mips32r6: i32 = 0;
pub const cpu_has_mips64r6: i32 = 0;

pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;
pub const cpu_has_inclusive_pcaches: i32 = 1;
pub const cpu_has_perf_cntr_intr_bit: i32 = 0;
pub const cpu_has_vz: i32 = 0;
pub const cpu_has_fre: i32 = 0;
pub const cpu_has_cdmm: i32 = 0;

#[inline]
pub const fn cpu_dcache_line_size() -> i32 {
    32
}

#[inline]
pub const fn cpu_icache_line_size() -> i32 {
    64
}

#[inline]
pub const fn cpu_scache_line_size() -> i32 {
    128
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
