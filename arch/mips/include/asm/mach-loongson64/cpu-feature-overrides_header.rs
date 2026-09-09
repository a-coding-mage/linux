/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009 Wu Zhangjin <wuzhangjin@gmail.com>
 * Copyright (C) 2009 Philippe Vachon <philippe@cowpig.ca>
 * Copyright (C) 2009 Zhang Le <r0bertz@gentoo.org>
 *
 * reference: /proc/cpuinfo,
 *	arch/mips/kernel/cpu-probe.c(cpu_probe_legacy),
 *	arch/mips/kernel/proc.c(show_cpuinfo),
 *	loongson2f user manual.
 */

// The original C header guard is omitted from executable Rust syntax.

pub const cpu_has_32fpr: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_64bits: i32 = 1;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_dc_aliases: i32 = if PAGE_SIZE < 0x4000 { 1 } else { 0 };
pub const cpu_has_divec: i32 = 0;
pub const cpu_has_inclusive_pcaches: i32 = 1;
pub const cpu_has_llsc: i32 = 1;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_veic: i32 = 0;
pub const cpu_has_vint: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_wsbh: i32 = 1;
pub const cpu_has_ic_fills_f_dc: i32 = 1;
pub const cpu_hwrena_impl_bits: u32 = 0xc0000000;
pub const cpu_has_mac2008_only: i32 = 1;
pub const cpu_has_mips_r2_exec_hazard: i32 = 0;
pub const cpu_has_perf_cntr_intr_bit: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
