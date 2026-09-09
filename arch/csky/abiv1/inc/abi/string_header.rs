/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: __ABI_CSKY_STRING_H

pub const __HAVE_ARCH_MEMCPY: bool = true;
unsafe extern "C" {
    pub fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: __kernel_size_t)
        -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMMOVE: bool = true;
unsafe extern "C" {
    pub fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: __kernel_size_t)
        -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMSET: bool = true;
unsafe extern "C" {
    pub fn memset(dest: *mut core::ffi::c_void, value: core::ffi::c_int, n: __kernel_size_t)
        -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
