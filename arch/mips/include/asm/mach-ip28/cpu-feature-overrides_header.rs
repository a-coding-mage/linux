/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Ralf Baechle
 * 6/2004\tpf
 */

// Dependency intent: <asm/cpu.h>

/*
 * IP28 only comes with R10000 family processors all using the same config
 */
pub const cpu_has_watch: i32 = 1;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_divec: i32 = 0;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_p: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_prefetch: i32 = 1;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;

pub const cpu_has_llsc: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_dc_aliases: i32 = 0; // see probe_pcache()
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_icache_snoops_remote_store: i32 = 1;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;

pub const cpu_has_nofpuex: i32 = 0;
pub const cpu_has_64bits: i32 = 1;

pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_4k_cache: i32 = 1;

pub const cpu_has_inclusive_pcaches: i32 = 1;

#[inline]
pub const fn cpu_dcache_line_size() -> i32 {
    32
}

#[inline]
pub const fn cpu_icache_line_size() -> i32 {
    64
}

pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
