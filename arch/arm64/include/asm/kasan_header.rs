/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/linkage.h, asm/memory.h, asm/mte-kasan.h, asm/pgtable-types.h

macro_rules! arch_kasan_set_tag {
    ($addr:expr, $tag:expr) => {
        __tag_set($addr, $tag)
    };
}

macro_rules! arch_kasan_reset_tag {
    ($addr:expr) => {
        __tag_reset($addr)
    };
}

macro_rules! arch_kasan_get_tag {
    ($addr:expr) => {
        __tag_get($addr)
    };
}

// Equivalent to: defined(CONFIG_KASAN_GENERIC) || defined(CONFIG_KASAN_SW_TAGS)
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
extern "C" {
    pub fn kasan_early_init();
    pub fn kasan_init();
}

// static inline void kasan_init(void) { }
#[cfg(not(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS")))]
#[inline]
pub fn kasan_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
