/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor feature overrides translated as Rust constants.
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_fpu: i32 = 0;
pub const cpu_has_32fpr: i32 = 0;
pub const cpu_has_counter: i32 = 1;

// Conditional on CONFIG_BCM47XX_BCMA && !CONFIG_BCM47XX_SSB.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_watch: i32 = 1;
// Conditional on CONFIG_BCM47XX_SSB && !CONFIG_BCM47XX_BCMA.
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_watch: i32 = 0;

pub const cpu_has_divec: i32 = 1;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_prefetch: i32 = 1;
pub const cpu_has_mcheck: i32 = 1;
pub const cpu_has_ejtag: i32 = 1;
pub const cpu_has_llsc: i32 = 1;

// cpu_has_mips16
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_rixi: i32 = 0;
pub const cpu_has_mmips: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
// cpu_has_dc_aliases
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_has_pindexed_dcache: i32 = 0;
pub const cpu_icache_snoops_remote_store: i32 = 0;

pub const cpu_has_mips_2: i32 = 1;
pub const cpu_has_mips_3: i32 = 0;
pub const cpu_has_mips32r1: i32 = 1;
// Conditional on CONFIG_BCM47XX_BCMA && !CONFIG_BCM47XX_SSB.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_mips32r2: i32 = 1;
// Conditional on CONFIG_BCM47XX_SSB && !CONFIG_BCM47XX_BCMA.
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// Conditional on CONFIG_BCM47XX_BCMA && !CONFIG_BCM47XX_SSB.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_dsp: i32 = 1;
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_dsp2: i32 = 1;
// Conditional on CONFIG_BCM47XX_SSB && !CONFIG_BCM47XX_BCMA.
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_dsp: i32 = 0;
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
// cpu_has_userlocal

pub const cpu_has_nofpuex: i32 = 0;
pub const cpu_has_64bits: i32 = 0;
pub const cpu_has_64bit_zero_reg: i32 = 0;
// Conditional on CONFIG_BCM47XX_BCMA && !CONFIG_BCM47XX_SSB.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_vint: i32 = 1;
// Conditional on CONFIG_BCM47XX_SSB && !CONFIG_BCM47XX_BCMA.
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_vint: i32 = 0;
pub const cpu_has_veic: i32 = 0;
pub const cpu_has_inclusive_pcaches: i32 = 0;

// Conditional on CONFIG_BCM47XX_BCMA && !CONFIG_BCM47XX_SSB.
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const fn cpu_dcache_line_size() -> i32 { 32 }
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const fn cpu_icache_line_size() -> i32 { 32 }
#[cfg(all(feature = "CONFIG_BCM47XX_BCMA", not(feature = "CONFIG_BCM47XX_SSB")))]
pub const cpu_has_perf_cntr_intr_bit: i32 = 1;
// Conditional on CONFIG_BCM47XX_SSB && !CONFIG_BCM47XX_BCMA.
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const fn cpu_dcache_line_size() -> i32 { 16 }
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const fn cpu_icache_line_size() -> i32 { 16 }
#[cfg(all(feature = "CONFIG_BCM47XX_SSB", not(feature = "CONFIG_BCM47XX_BCMA")))]
pub const cpu_has_perf_cntr_intr_bit: i32 = 0;

pub const fn cpu_scache_line_size() -> i32 { 0 }
pub const cpu_has_vz: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
