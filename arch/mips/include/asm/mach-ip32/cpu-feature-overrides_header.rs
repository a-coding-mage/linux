/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 Ilya A. Volynets-Evenbakh
 * Copyright (C) 2005, 07 Ralf Baechle (ralf@linux-mips.org)
 */

/*
 * R5000 has an interesting "restriction":  ll(d)/sc(d)
 * instructions to XKPHYS region simply do uncached bus
 * requests. This breaks all the atomic bitops functions.
 * so, for 64bit IP32 kernel we just don't use ll/sc.
 * This does not affect luserland.
 *
 * The original value is selected by CONFIG_CPU_R5000,
 * CONFIG_CPU_NEVADA, and CONFIG_64BIT build conditions.
 */
#[cfg(all(
    any(CONFIG_CPU_R5000, CONFIG_CPU_NEVADA),
    CONFIG_64BIT
))]
pub const cpu_has_llsc: i32 = 0;

#[cfg(not(all(
    any(CONFIG_CPU_R5000, CONFIG_CPU_NEVADA),
    CONFIG_64BIT
)))]
pub const cpu_has_llsc: i32 = 1;

/* Settings which are common for all ip32 CPUs */
pub const cpu_has_tlb: i32 = 1;
pub const cpu_has_4kex: i32 = 1;
pub const cpu_has_32fpr: i32 = 1;
pub const cpu_has_counter: i32 = 1;
pub const cpu_has_mips16: i32 = 0;
pub const cpu_has_mips16e2: i32 = 0;
pub const cpu_has_vce: i32 = 0;
pub const cpu_has_cache_cdex_s: i32 = 0;
pub const cpu_has_mcheck: i32 = 0;
pub const cpu_has_ejtag: i32 = 0;
pub const cpu_has_vtag_icache: i32 = 0;
pub const cpu_has_ic_fills_f_dc: i32 = 0;
pub const cpu_has_dsp: i32 = 0;
pub const cpu_has_dsp2: i32 = 0;
pub const cpu_has_4k_cache: i32 = 1;
pub const cpu_has_mipsmt: i32 = 0;
pub const cpu_has_userlocal: i32 = 0;

pub const cpu_has_mips32r1: i32 = 0;
pub const cpu_has_mips32r2: i32 = 0;
pub const cpu_has_mips64r1: i32 = 0;
pub const cpu_has_mips64r2: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
