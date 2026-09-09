/* SPDX-License-Identifier: GPL-2.0 */

/*
 * In the C header these macros provide the normal and KASAN-prefixed global
 * symbol declarations/exports when CONFIG_KASAN is enabled without compiler
 * KASAN memory-intrinsic prefix support.  They are intentionally retained as
 * conditional intent here; the corresponding linker/compiler declarations are
 * supplied by the surrounding translation unit.
 */

/* C preprocessor conditionals are represented by Rust cfg attributes below. */

pub const KASAN_SHADOW_SCALE_SHIFT: u32 = 3;

#[cfg(all(feature = "CONFIG_EXECMEM", feature = "CONFIG_PPC32"))]
pub const KASAN_KERN_START: usize =
    (PAGE_OFFSET - SZ_256M) & !(SZ_256M - 1);

#[cfg(not(all(feature = "CONFIG_EXECMEM", feature = "CONFIG_PPC32")))]
pub const KASAN_KERN_START: usize = PAGE_OFFSET;

pub const KASAN_SHADOW_START: usize =
    KASAN_SHADOW_OFFSET + (KASAN_KERN_START >> KASAN_SHADOW_SCALE_SHIFT);

pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET;

#[cfg(feature = "CONFIG_PPC32")]
pub const KASAN_SHADOW_END: usize =
    (0usize.wrapping_sub(KASAN_SHADOW_START) >> KASAN_SHADOW_SCALE_SHIFT);

#[cfg(all(not(feature = "CONFIG_PPC32"), feature = "CONFIG_PPC_BOOK3S_64"))]
pub const KASAN_SHADOW_END: u64 = 0xc00fc00000000000u64;

#[cfg(all(
    not(feature = "CONFIG_PPC32"),
    not(feature = "CONFIG_PPC_BOOK3S_64")
))]
pub const KASAN_SHADOW_END: u64 = 0xc000200000000000u64;

#[cfg(feature = "CONFIG_KASAN")]
extern "C" {
    pub fn kasan_early_init();
    pub fn kasan_mmu_init();
    pub fn kasan_init();
    pub fn kasan_late_init();
}

#[cfg(not(feature = "CONFIG_KASAN"))]
#[inline]
pub fn kasan_init() {}

#[cfg(not(feature = "CONFIG_KASAN"))]
#[inline]
pub fn kasan_mmu_init() {}

#[cfg(not(feature = "CONFIG_KASAN"))]
#[inline]
pub fn kasan_late_init() {}

extern "C" {
    pub fn kasan_update_early_region(k_start: ::core::ffi::c_ulong,
                                     k_end: ::core::ffi::c_ulong,
                                     pte: pte_t);
    pub fn kasan_init_shadow_page_tables(
        k_start: ::core::ffi::c_ulong,
        k_end: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn kasan_init_region(start: *mut ::core::ffi::c_void, size: usize)
        -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
