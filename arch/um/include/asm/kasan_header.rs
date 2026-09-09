/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies from <linux/init.h> and <linux/const.h> are supplied externally.

pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET as usize;

/* Used in kasan_mem_to_shadow to divide by 8. */
pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;

#[cfg(feature = "CONFIG_X86_64")]
pub const KASAN_HOST_USER_SPACE_END_ADDR: usize = 0x0000_7fff_ffff_ffffusize;

#[cfg(feature = "CONFIG_X86_64")]
/* KASAN_SHADOW_SIZE is the size of total address space divided by 8. */
pub const KASAN_SHADOW_SIZE: usize =
    (KASAN_HOST_USER_SPACE_END_ADDR.wrapping_add(1)) >> KASAN_SHADOW_SCALE_SHIFT;

// KASAN_SHADOW_SIZE is not defined for non-CONFIG_X86_64 sub-architectures.

pub const KASAN_SHADOW_START: usize = KASAN_SHADOW_OFFSET;
pub const KASAN_SHADOW_END: usize =
    KASAN_SHADOW_START.wrapping_add(KASAN_SHADOW_SIZE);

#[cfg(feature = "CONFIG_KASAN")]
extern "C" {
    pub fn kasan_init();
}

#[cfg(not(feature = "CONFIG_KASAN"))]
#[inline]
pub fn kasan_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
