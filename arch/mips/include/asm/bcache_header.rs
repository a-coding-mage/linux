/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1997, 1999 by Ralf Baechle
 * Copyright (c) 1999 Silicon Graphics, Inc.
 */

// Dependency: linux/types.h

/* Some R4000 / R4400 / R4600 / R5000 machines may have a non-dma-coherent,
   chipset implemented caches. On machines with other CPUs the CPU does the
   cache thing itself. */
#[repr(C)]
pub struct bcache_ops {
    pub bc_enable: Option<unsafe extern "C" fn()>,
    pub bc_disable: Option<unsafe extern "C" fn()>,
    pub bc_wback_inv: Option<unsafe extern "C" fn(page: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong)>,
    pub bc_inv: Option<unsafe extern "C" fn(page: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong)>,
    pub bc_prefetch_enable: Option<unsafe extern "C" fn()>,
    pub bc_prefetch_disable: Option<unsafe extern "C" fn()>,
    pub bc_prefetch_is_enabled: Option<unsafe extern "C" fn() -> bool>,
}

unsafe extern "C" {
    pub fn indy_sc_init();
}

// The following items are active when CONFIG_BOARD_SCACHE is defined.
#[cfg(feature = "CONFIG_BOARD_SCACHE")]
extern "C" {
    pub static mut bcops: *mut bcache_ops;
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_enable() {
    ((*bcops).bc_enable.unwrap())();
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_disable() {
    ((*bcops).bc_disable.unwrap())();
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_wback_inv(page: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) {
    ((*bcops).bc_wback_inv.unwrap())(page, size);
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_inv(page: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) {
    ((*bcops).bc_inv.unwrap())(page, size);
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_prefetch_enable() {
    if let Some(f) = (*bcops).bc_prefetch_enable {
        f();
    }
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_prefetch_disable() {
    if let Some(f) = (*bcops).bc_prefetch_disable {
        f();
    }
}

#[cfg(feature = "CONFIG_BOARD_SCACHE")]
#[inline]
pub unsafe fn bc_prefetch_is_enabled() -> bool {
    if let Some(f) = (*bcops).bc_prefetch_is_enabled {
        return f();
    }

    false
}

// Not R4000 / R4400 / R4600 / R5000. These are no-op macro equivalents when
// CONFIG_BOARD_SCACHE is not defined.
#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_enable() {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_disable() {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_wback_inv(_page: ::core::ffi::c_ulong, _size: ::core::ffi::c_ulong) {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_inv(_page: ::core::ffi::c_ulong, _size: ::core::ffi::c_ulong) {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_prefetch_enable() {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_prefetch_disable() {}

#[cfg(not(feature = "CONFIG_BOARD_SCACHE"))]
#[inline]
pub fn bc_prefetch_is_enabled() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
