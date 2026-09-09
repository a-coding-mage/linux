/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  arch/arm/include/asm/cache.h
 */

// #define L1_CACHE_SHIFT CONFIG_ARM_L1_CACHE_SHIFT
pub const L1_CACHE_SHIFT: u32 = CONFIG_ARM_L1_CACHE_SHIFT;
pub const L1_CACHE_BYTES: u32 = 1u32 << L1_CACHE_SHIFT;

/*
 * Memory returned by kmalloc() may be used for DMA, so we must make
 * sure that all such allocations are cache aligned. Otherwise,
 * unrelated code may cause parts of the buffer to be read into the
 * cache before the transfer is done, causing old data to be seen by the
 * CPU.
 */
pub const ARCH_DMA_MINALIGN: u32 = L1_CACHE_BYTES;

/*
 * With EABI on ARMv5 and above we must have 64-bit aligned slab pointers.
 *
 * Conditional intent preserved from:
 * #if defined(CONFIG_AEABI) && (__LINUX_ARM_ARCH__ >= 5)
 */
#[cfg(all(CONFIG_AEABI, arm_arch_at_least_5))]
pub const ARCH_SLAB_MINALIGN: usize = 8;

// C macro: __read_mostly __section(".data..read_mostly")
// Rust users should apply the equivalent section attribute at the declaration site.

/*
 * The C declaration is conditional on CONFIG_ARCH_HAS_CACHE_LINE_SIZE and
 * is omitted for assembly sources.
 */
#[cfg(CONFIG_ARCH_HAS_CACHE_LINE_SIZE)]
unsafe extern "C" {
    pub fn cache_line_size() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
