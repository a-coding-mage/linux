/* SPDX-License-Identifier: GPL-2.0 */

// The following declarations are enabled when CONFIG_KASAN is enabled.
// The build-time condition is preserved here as intent; configuration is
// supplied by the surrounding translation unit.

pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;
pub const KASAN_SHADOW_SIZE: usize =
    1usize << (_REGION1_SHIFT - KASAN_SHADOW_SCALE_SHIFT);
pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET as usize;
pub const KASAN_SHADOW_START: usize = KASAN_SHADOW_OFFSET;
pub const KASAN_SHADOW_END: usize = KASAN_SHADOW_START + KASAN_SHADOW_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
