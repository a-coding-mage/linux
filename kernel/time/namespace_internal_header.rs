/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations corresponding to <linux/mutex.h> and the C
// forward declaration of `struct time_namespace`.
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct time_namespace {
    _private: [u8; 0],
}

/*
 * Protects possibly multiple offsets writers racing each other
 * and tasks entering the namespace.
 */
unsafe extern "C" {
    pub static mut timens_offset_lock: mutex;
}

// CONFIG_TIME_NS_VDSO selects the external implementations below.  When it
// is not enabled, the C header provides the following inline no-op forms.
#[cfg(CONFIG_TIME_NS_VDSO)]
unsafe extern "C" {
    pub fn timens_vdso_alloc_vvar_page(ns: *mut time_namespace) -> i32;
    pub fn timens_vdso_free_vvar_page(ns: *mut time_namespace);
}

#[cfg(not(CONFIG_TIME_NS_VDSO))]
#[inline]
pub unsafe fn timens_vdso_alloc_vvar_page(_ns: *mut time_namespace) -> i32 {
    0
}

#[cfg(not(CONFIG_TIME_NS_VDSO))]
#[inline]
pub unsafe fn timens_vdso_free_vvar_page(_ns: *mut time_namespace) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
