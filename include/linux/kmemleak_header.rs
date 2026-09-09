/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/kmemleak.h
 *
 * Copyright (C) 2008 ARM Limited
 * Written by Catalin Marinas <catalin.marinas@arm.com>
 */

// C dependencies: linux/slab.h and linux/vmalloc.h.
// CONFIG_DEBUG_KMEMLEAK is preserved as the Rust cfg feature of the same name.

#[cfg(feature = "CONFIG_DEBUG_KMEMLEAK")]
extern "C" {
    pub fn kmemleak_init();
    pub fn kmemleak_alloc(ptr: *const core::ffi::c_void, size: usize, min_count: i32, gfp: gfp_t);
    pub fn kmemleak_alloc_percpu(ptr: *const core::ffi::c_void, size: usize, gfp: gfp_t);
    pub fn kmemleak_vmalloc(area: *const vm_struct, size: usize, gfp: gfp_t);
    pub fn kmemleak_free(ptr: *const core::ffi::c_void);
    pub fn kmemleak_free_part(ptr: *const core::ffi::c_void, size: usize);
    pub fn kmemleak_free_percpu(ptr: *const core::ffi::c_void);
    pub fn kmemleak_update_trace(ptr: *const core::ffi::c_void);
    pub fn kmemleak_not_leak(ptr: *const core::ffi::c_void);
    pub fn kmemleak_transient_leak(ptr: *const core::ffi::c_void);
    pub fn kmemleak_ignore(ptr: *const core::ffi::c_void);
    pub fn kmemleak_ignore_percpu(ptr: *const core::ffi::c_void);
    pub fn kmemleak_scan_area(ptr: *const core::ffi::c_void, size: usize, gfp: gfp_t);
    pub fn kmemleak_no_scan(ptr: *const core::ffi::c_void);
    pub fn kmemleak_alloc_phys(phys: phys_addr_t, size: usize, gfp: gfp_t);
    pub fn kmemleak_free_part_phys(phys: phys_addr_t, size: usize);
    pub fn kmemleak_ignore_phys(phys: phys_addr_t);
}

#[cfg(feature = "CONFIG_DEBUG_KMEMLEAK")]
#[inline]
pub unsafe fn kmemleak_alloc_recursive(ptr: *const core::ffi::c_void, size: usize,
                                        min_count: i32, flags: slab_flags_t, gfp: gfp_t) {
    if (flags & SLAB_NOLEAKTRACE) == 0 {
        kmemleak_alloc(ptr, size, min_count, gfp);
    }
}

#[cfg(feature = "CONFIG_DEBUG_KMEMLEAK")]
#[inline]
pub unsafe fn kmemleak_free_recursive(ptr: *const core::ffi::c_void, flags: slab_flags_t) {
    if (flags & SLAB_NOLEAKTRACE) == 0 {
        kmemleak_free(ptr);
    }
}

#[inline]
pub unsafe fn kmemleak_erase(ptr: *mut *mut core::ffi::c_void) {
    *ptr = core::ptr::null_mut();
}

#[cfg(not(feature = "CONFIG_DEBUG_KMEMLEAK"))]
macro_rules! kmemleak_empty {
    ($name:ident ( $( $arg:ident : $ty:ty ),* $(,)? )) => {
        #[inline]
        pub unsafe fn $name($( $arg: $ty ),*) {}
    };
}

#[cfg(not(feature = "CONFIG_DEBUG_KMEMLEAK"))]
mod kmemleak_disabled {
    #[allow(unused_imports)]
    use super::*;

    kmemleak_empty!(kmemleak_init());
    kmemleak_empty!(kmemleak_alloc(ptr: *const core::ffi::c_void, size: usize, min_count: i32, gfp: gfp_t));
    kmemleak_empty!(kmemleak_alloc_recursive(ptr: *const core::ffi::c_void, size: usize, min_count: i32, flags: slab_flags_t, gfp: gfp_t));
    kmemleak_empty!(kmemleak_alloc_percpu(ptr: *const core::ffi::c_void, size: usize, gfp: gfp_t));
    kmemleak_empty!(kmemleak_vmalloc(area: *const vm_struct, size: usize, gfp: gfp_t));
    kmemleak_empty!(kmemleak_free(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_free_part(ptr: *const core::ffi::c_void, size: usize));
    kmemleak_empty!(kmemleak_free_recursive(ptr: *const core::ffi::c_void, flags: slab_flags_t));
    kmemleak_empty!(kmemleak_free_percpu(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_update_trace(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_not_leak(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_transient_leak(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_ignore_percpu(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_ignore(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_scan_area(ptr: *const core::ffi::c_void, size: usize, gfp: gfp_t));
    kmemleak_empty!(kmemleak_erase(ptr: *mut *mut core::ffi::c_void));
    kmemleak_empty!(kmemleak_no_scan(ptr: *const core::ffi::c_void));
    kmemleak_empty!(kmemleak_alloc_phys(phys: phys_addr_t, size: usize, gfp: gfp_t));
    kmemleak_empty!(kmemleak_free_part_phys(phys: phys_addr_t, size: usize));
    kmemleak_empty!(kmemleak_ignore_phys(phys: phys_addr_t));
}

// External types and constants are supplied by the translated Linux dependencies.
extern "Rust" {
    type vm_struct;
    type gfp_t;
    type slab_flags_t;
    type phys_addr_t;
    static SLAB_NOLEAKTRACE: slab_flags_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
