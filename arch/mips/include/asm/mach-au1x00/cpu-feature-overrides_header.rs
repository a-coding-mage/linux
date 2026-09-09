/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// __ASM_MACH_AU1X00_CPU_FEATURE_OVERRIDES_H

pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_ftlb: i32 = 0;
pub const cpu_has_tlbinv: i32 = 0;
pub const cpu_has_segments: i32 = 0;
pub const cpu_has_eva: i32 = 0;
pub const cpu_has_htw: i32 = 0;
pub const cpu_has_ldpte: i32 = 0;
pub const cpu_has_rixiex: i32 = 0;
pub const cpu_has_maar: i32 = 0;
pub const cpu_has_rw_llb: i32 = 0;
pub const cpu_has_3kex: i32 = 0;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
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
pub const cpu_has_guestctl0ext: i32 = 0;
pub const cpu_has_guestctl1: i32 = 0;
pub const cpu_has_guestctl2: i32 = 0;
pub const cpu_has_guestid: i32 = 0;
pub const cpu_has_drg: i32 = 0;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_rixi: i32 = 0;
pub const cpu_has_mmips: i32 = 0;
pub const cpu_has_lpa: i32 = 0;
pub const cpu_has_mhv: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_dc_aliases: i32 = 0;
pub const cpu_has_ic_fills_f_dc: i32 = 1;
pub const cpu_has_pindexed_dcache: i32 = 0;
pub const cpu_has_mips32r1: i32 = 1;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips32r6: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;
pub const cpu_has_mips64r6: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_dsp3: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_vp: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;
pub const cpu_has_nofpuex: i32 = 0;
pub const cpu_has_64bits: i32 = 0;
pub const cpu_has_64bit_zero_reg: i32 = 0;
pub const cpu_has_vint: i32 = 0;
pub const cpu_has_veic: i32 = 0;
pub const cpu_has_inclusive_pcaches: i32 = 0;

pub const cpu_dcache_line_size: i32 = 32;
pub const cpu_icache_line_size: i32 = 32;
pub const cpu_scache_line_size: i32 = 0;
pub const cpu_tcache_line_size: i32 = 0;

pub const cpu_has_perf_cntr_intr_bit: i32 = 0;
pub const cpu_has_vz: i32 = 0;
pub const cpu_has_msa: i32 = 0;
pub const cpu_has_ufr: i32 = 0;
pub const cpu_has_fre: i32 = 0;
pub const cpu_has_cdmm: i32 = 0;
pub const cpu_has_small_pages: i32 = 0;
pub const cpu_has_nan_legacy: i32 = 1;
pub const cpu_has_nan_2008: i32 = 1;
pub const cpu_has_ebase_wg: i32 = 0;
pub const cpu_has_badinstr: i32 = 0;
pub const cpu_has_badinstrp: i32 = 0;
pub const cpu_has_contextconfig: i32 = 0;
pub const cpu_has_perf: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
