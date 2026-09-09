/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2004 Cavium Networks
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, asm/mipsregs.h

/*
 * Cavium Octeons are MIPS64v2 processors
 */
pub const CPU_DCACHE_LINE_SIZE: usize = 128;
pub const CPU_ICACHE_LINE_SIZE: usize = 128;

pub const CPU_HAS_4KEX: i32 = 1;
pub const CPU_HAS_3K_CACHE: i32 = 0;
pub const CPU_HAS_4K_CACHE: i32 = 0;
pub const CPU_HAS_COUNTER: i32 = 1;
pub const CPU_HAS_WATCH: i32 = 1;
pub const CPU_HAS_DIVEC: i32 = 1;
pub const CPU_HAS_VCE: i32 = 0;
pub const CPU_HAS_CACHE_CDEX_P: i32 = 0;
pub const CPU_HAS_CACHE_CDEX_S: i32 = 0;
pub const CPU_HAS_PREFETCH: i32 = 1;

pub const CPU_HAS_LLSC: i32 = 1;
/*
 * We Disable LL/SC on non SMP systems as it is faster to disable
 * interrupts for atomic access than a LL/SC.
 */
#[cfg(feature = "CONFIG_SMP")]
pub const KERNEL_USES_LLSC: i32 = 1;
#[cfg(not(feature = "CONFIG_SMP"))]
pub const KERNEL_USES_LLSC: i32 = 0;

pub const CPU_HAS_VTAG_ICACHE: i32 = 1;
pub const CPU_HAS_DC_ALIASES: i32 = 0;
pub const CPU_HAS_IC_FILLS_F_DC: i32 = 0;
pub const CPU_HAS_64BITS: i32 = 1;
pub const CPU_HAS_OCTEON_CACHE: i32 = 1;
pub const CPU_HAS_MIPS32R1: i32 = 1;
pub const CPU_HAS_MIPS32R2: i32 = 1;
pub const CPU_HAS_MIPS64R1: i32 = 1;
pub const CPU_HAS_MIPS64R2: i32 = 1;
pub const CPU_HAS_DSP: i32 = 0;
pub const CPU_HAS_DSP2: i32 = 0;
pub const CPU_HAS_MIPSMT: i32 = 0;
pub const CPU_HAS_VINT: i32 = 0;
pub const CPU_HAS_VEIC: i32 = 0;

pub const CPU_HWRENA_IMPL_BITS: u32 = MIPS_HWRENA_IMPL1 | MIPS_HWRENA_IMPL2;
pub const CPU_HAS_WSBH: i32 = 1;

#[macro_export]
macro_rules! cpu_has_rixi {
    () => {
        cpu_data[0].cputype != CPU_CAVIUM_OCTEON
    };
}

pub const PREFETCH_STRIDE: usize = 128;

/*
 * All gcc versions that have OCTEON support define __OCTEON__ and have the
 * __builtin_popcount support.
 *
 * The original definition is active only when __OCTEON__ is defined.
 */
#[cfg(feature = "__OCTEON__")]
pub const ARCH_HAS_USABLE_BUILTIN_POPCOUNT: i32 = 1;

/*
 * The last 256MB are reserved for device to device mappings and the
 * BAR1 hole.
 */
pub const MAX_DMA32_PFN: u64 = (((1u64 << 32) - (1u64 << 28)) >> PAGE_SHIFT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
