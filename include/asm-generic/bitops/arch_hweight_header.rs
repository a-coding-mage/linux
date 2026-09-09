/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are supplied by the corresponding software
// hweight implementation.
unsafe extern "C" {
    fn __sw_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __sw_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __sw_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __sw_hweight64(w: u64) -> core::ffi::c_ulong;
}

pub unsafe fn __arch_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __sw_hweight32(w) }
}

pub unsafe fn __arch_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __sw_hweight16(w) }
}

pub unsafe fn __arch_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __sw_hweight8(w) }
}

pub unsafe fn __arch_hweight64(w: u64) -> core::ffi::c_ulong {
    unsafe { __sw_hweight64(w) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
