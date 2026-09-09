/* SPDX-License-Identifier: GPL-2.0 */

// __HAVE_ARCH_MEMSET
pub const __HAVE_ARCH_MEMSET: bool = true;

extern "C" {
    pub fn memset(dest: *mut core::ffi::c_void, value: core::ffi::c_int, count: usize)
        -> *mut core::ffi::c_void;
}

// __HAVE_ARCH_MEMCPY
pub const __HAVE_ARCH_MEMCPY: bool = true;

extern "C" {
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
