/* SPDX-License-Identifier: GPL-2.0 */

// This declaration is intended for non-assembler builds.

// The following items are enabled when CONFIG_KASAN is enabled.
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;

/* Start of area covered by KASAN */
#[cfg(CONFIG_KASAN)]
pub const KASAN_START_VADDR: usize = 0x9000_0000;

/* Start of the shadow map */
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_START: usize = XCHAL_PAGE_TABLE_VADDR + XCHAL_PAGE_TABLE_SIZE;

/* Size of the shadow map */
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_SIZE: usize =
    0usize.wrapping_sub(KASAN_START_VADDR) >> KASAN_SHADOW_SCALE_SHIFT;

/* End of the shadow map */
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_END: usize = KASAN_SHADOW_START + KASAN_SHADOW_SIZE;

/* Offset for mem to shadow address transformation */
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET;

#[cfg(CONFIG_KASAN)]
extern "C" {
    pub fn kasan_early_init();
    pub fn kasan_init();
}

#[cfg(not(CONFIG_KASAN))]
#[inline]
pub fn kasan_early_init() {}

#[cfg(not(CONFIG_KASAN))]
#[inline]
pub fn kasan_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
