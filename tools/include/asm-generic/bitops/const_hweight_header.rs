/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Compile time versions of __arch_hweightN()
 */
pub const fn __const_hweight8(w: u64) -> core::ffi::c_uint {
    (((w & (1u64 << 0)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 1)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 2)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 3)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 4)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 5)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 6)) != 0) as core::ffi::c_uint)
        + (((w & (1u64 << 7)) != 0) as core::ffi::c_uint)
}

pub const fn __const_hweight16(w: u64) -> core::ffi::c_uint {
    __const_hweight8(w) + __const_hweight8(w >> 8)
}

pub const fn __const_hweight32(w: u64) -> core::ffi::c_uint {
    __const_hweight16(w) + __const_hweight16(w >> 16)
}

pub const fn __const_hweight64(w: u64) -> core::ffi::c_uint {
    __const_hweight32(w) + __const_hweight32(w >> 32)
}

unsafe extern "C" {
    fn __arch_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __arch_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __arch_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __arch_hweight64(w: u64) -> core::ffi::c_ulong;
}

/*
 * Generic interface.
 *
 * The C macros use __builtin_constant_p(w) to select the constant expression
 * implementation at compile time, otherwise falling back to __arch_hweightN().
 * Rust has no direct local equivalent for that compiler-builtin macro
 * dispatch, so these preserve the runtime fallback interface.
 */
pub unsafe fn hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __arch_hweight8(w) }
}

pub unsafe fn hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __arch_hweight16(w) }
}

pub unsafe fn hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint {
    unsafe { __arch_hweight32(w) }
}

pub unsafe fn hweight64(w: u64) -> core::ffi::c_ulong {
    unsafe { __arch_hweight64(w) }
}

/*
 * Interface for known constant arguments
 */
pub const fn HWEIGHT8(w: u64) -> core::ffi::c_uint {
    __const_hweight8(w)
}

pub const fn HWEIGHT16(w: u64) -> core::ffi::c_uint {
    __const_hweight16(w)
}

pub const fn HWEIGHT32(w: u64) -> core::ffi::c_uint {
    __const_hweight32(w)
}

pub const fn HWEIGHT64(w: u64) -> core::ffi::c_uint {
    __const_hweight64(w)
}

/*
 * Type invariant interface to the compile time constant hweight functions.
 */
pub const fn HWEIGHT(w: u64) -> core::ffi::c_uint {
    HWEIGHT64(w as u64)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
