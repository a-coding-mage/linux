/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/static_key.h.

// CONFIG_ARCH_DEFER_KASAN || CONFIG_KASAN_HW_TAGS
// Global runtime flag for KASAN modes that need runtime control.
// Used by ARCH_DEFER_KASAN architectures and HW_TAGS mode.
#[cfg(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS))]
extern "C" {
    static kasan_flag_enabled: StaticKey;
}

// Runtime control for shadow memory initialization or HW_TAGS mode.
// Uses static key for architectures that need deferred KASAN or HW_TAGS.
#[cfg(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS))]
#[inline(always)]
pub unsafe fn kasan_enabled() -> bool {
    static_branch_likely(&kasan_flag_enabled)
}

#[cfg(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_enable() {
    static_branch_enable(&kasan_flag_enabled);
}

// For architectures that can enable KASAN early, use compile-time check.
#[cfg(not(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS)))]
#[inline(always)]
pub const fn kasan_enabled() -> bool {
    cfg!(CONFIG_KASAN)
}

#[cfg(not(any(CONFIG_ARCH_DEFER_KASAN, CONFIG_KASAN_HW_TAGS)))]
#[inline]
pub fn kasan_enable() {}

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[inline]
pub unsafe fn kasan_hw_tags_enabled() -> bool {
    kasan_enabled()
}

#[cfg(not(CONFIG_KASAN_HW_TAGS))]
#[inline]
pub const fn kasan_hw_tags_enabled() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
