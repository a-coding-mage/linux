/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 04, 07 Ralf Baechle (ralf@linux-mips.org)
 *
 * SNI RM200 C apparently was only shipped with R4600 V2.0 and R5000 processors.
 */

pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_32fpr: i32 = 1;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_watch: i32 = 0;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_divec: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 1;
pub const cpu_has_prefetch: i32 = 0;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;
pub const cpu_has_llsc: i32 = 1;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_dc_aliases: i32 = if PAGE_SIZE < 0x4000 { 1 } else { 0 };
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_nofpuex: i32 = 0;
pub const cpu_has_64bits: i32 = 1;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;

pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
