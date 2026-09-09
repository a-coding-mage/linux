/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006, 07 Ralf Baechle (ralf@linux-mips.org)
 */

pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_3k_cache: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_32fpr: i32 = 1;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_watch: i32 = 0;
pub const cpu_has_divec: i32 = 1;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_prefetch: i32 = 0;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;

pub const cpu_has_inclusive_pcaches: i32 = 0;

pub const fn cpu_dcache_line_size() -> i32 {
    32
}

pub const fn cpu_icache_line_size() -> i32 {
    32
}

pub const fn cpu_scache_line_size() -> i32 {
    0
}

// The C source selects this value using the build-time CONFIG_64BIT condition.
#[cfg(feature = "CONFIG_64BIT")]
pub const cpu_has_llsc: i32 = 0;

#[cfg(not(feature = "CONFIG_64BIT"))]
pub const cpu_has_llsc: i32 = 1;

pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_mdmx: i32 = 0;
pub const cpu_has_mips3d: i32 = 0;
pub const cpu_has_smartmips: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_icache_snoops_remote_store: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;

pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
