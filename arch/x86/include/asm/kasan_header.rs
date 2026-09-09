/* SPDX-License-Identifier: GPL-2.0 */

// KASAN_SHADOW_OFFSET is supplied by the build configuration.
pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET;
pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;

/*
 * Compiler uses shadow offset assuming that addresses start
 * from 0. Kernel addresses don't start from 0, so shadow
 * for kernel really starts from compiler's shadow offset +
 * 'kernel address space start' >> KASAN_SHADOW_SCALE_SHIFT
 */
pub const KASAN_SHADOW_START: usize = KASAN_SHADOW_OFFSET
    + (((!0usize) << __VIRTUAL_MASK_SHIFT) >> KASAN_SHADOW_SCALE_SHIFT);

/*
 * 47 bits for kernel address -> (47 - KASAN_SHADOW_SCALE_SHIFT) bits for shadow
 * 56 bits for kernel address -> (56 - KASAN_SHADOW_SCALE_SHIFT) bits for shadow
 */
pub const KASAN_SHADOW_END: usize = KASAN_SHADOW_START
    + (1usize << (__VIRTUAL_MASK_SHIFT - KASAN_SHADOW_SCALE_SHIFT));

#[cfg(CONFIG_KASAN)]
extern "C" {
    pub fn kasan_early_init();
    pub fn kasan_init();
    pub fn kasan_populate_shadow_for_vaddr(
        va: *mut core::ffi::c_void,
        size: usize,
        nid: i32,
    );
}

#[cfg(not(CONFIG_KASAN))]
pub unsafe fn kasan_early_init() {}

#[cfg(not(CONFIG_KASAN))]
pub unsafe fn kasan_init() {}

#[cfg(not(CONFIG_KASAN))]
pub unsafe fn kasan_populate_shadow_for_vaddr(
    _va: *mut core::ffi::c_void,
    _size: usize,
    _nid: i32,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
