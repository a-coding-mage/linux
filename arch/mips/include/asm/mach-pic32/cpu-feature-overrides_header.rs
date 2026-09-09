/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/*
 * CPU feature overrides for PIC32 boards
 */

/* Corresponds to the C preprocessor condition CONFIG_CPU_MIPS32. */
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_vint: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_veic: i32 = 0;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_tlb: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_4kex: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_4k_cache: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_fpu: i32 = 0;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_counter: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_llsc: i32 = 1;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_has_nofpuex: i32 = 0;
#[cfg(CONFIG_CPU_MIPS32)]
pub const cpu_icache_snoops_remote_store: i32 = 1;

/* Corresponds to the C preprocessor condition CONFIG_CPU_MIPS64. */
#[cfg(CONFIG_CPU_MIPS64)]
compile_error!("This platform does not support 64bit.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
