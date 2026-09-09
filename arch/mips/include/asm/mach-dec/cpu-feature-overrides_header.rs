/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CPU feature overrides for DECstation systems. Two variations are
 * generally applicable.
 *
 * Copyright (C) 2013 Maciej W. Rozycki
 */

/* Generic ones first. */
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_tlbinv: i32 = 0;
pub const cpu_has_segments: i32 = 0;
pub const cpu_has_eva: i32 = 0;
pub const cpu_has_htw: i32 = 0;
pub const cpu_has_rixiex: i32 = 0;
pub const cpu_has_maar: i32 = 0;
pub const cpu_has_rw_llb: i32 = 0;
pub const cpu_has_divec: i32 = 0;
pub const cpu_has_prefetch: i32 = 0;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_rixi: i32 = 0;
pub const cpu_has_xpa: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_has_pindexed_dcache: i32 = 0;
pub const cpu_icache_snoops_remote_store: i32 = 1;
pub const cpu_has_mips_4: i32 = 0;
pub const cpu_has_mips_5: i32 = 0;
pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;
pub const cpu_has_perf_cntr_intr_bit: i32 = 0;
pub const cpu_has_vz: i32 = 0;
pub const cpu_has_fre: i32 = 0;
pub const cpu_has_cdmm: i32 = 0;

/* R3k-specific ones. */
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_3kex: i32 = 1;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_4kex: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_3k_cache: i32 = 1;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_4k_cache: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_32fpr: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_counter: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_watch: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_vce: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_cache_cdex_p: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_cache_cdex_s: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_llsc: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_dc_aliases: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_mips_2: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_mips_3: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_nofpuex: i32 = 1;
#[cfg(CONFIG_CPU_R3000)]
pub const cpu_has_inclusive_pcaches: i32 = 0;
#[cfg(CONFIG_CPU_R3000)]
pub const fn cpu_dcache_line_size() -> i32 { 4 }
#[cfg(CONFIG_CPU_R3000)]
pub const fn cpu_icache_line_size() -> i32 { 4 }
#[cfg(CONFIG_CPU_R3000)]
pub const fn cpu_scache_line_size() -> i32 { 0 }

/* R4k-specific ones. */
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_3kex: i32 = 0;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_4kex: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_3k_cache: i32 = 0;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_4k_cache: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_32fpr: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_counter: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_watch: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_vce: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_cache_cdex_p: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_cache_cdex_s: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_llsc: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_dc_aliases: bool = PAGE_SIZE < 0x4000;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_mips_2: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_mips_3: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_nofpuex: i32 = 0;
#[cfg(CONFIG_CPU_R4X00)]
pub const cpu_has_inclusive_pcaches: i32 = 1;
#[cfg(CONFIG_CPU_R4X00)]
pub const fn cpu_dcache_line_size() -> i32 { 16 }
#[cfg(CONFIG_CPU_R4X00)]
pub const fn cpu_icache_line_size() -> i32 { 16 }
#[cfg(CONFIG_CPU_R4X00)]
pub const fn cpu_scache_line_size() -> i32 { 32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
