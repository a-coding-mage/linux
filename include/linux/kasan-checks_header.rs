/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The annotations present in this file are only relevant for the software
 * KASAN modes that rely on compiler instrumentation, and will be optimized
 * away for the hardware tag-based KASAN mode. Use kasan_check_byte() instead.
 */

/*
 * __kasan_check_*: Always available when KASAN is enabled. This may be used
 * even in compilation units that selectively disable KASAN, but must use KASAN
 * to validate access to an address.   Never use these in header files!
 */
#[cfg(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS"))]
unsafe extern "C" {
    pub fn __kasan_check_read(p: *const core::ffi::c_void, size: u32) -> bool;
    pub fn __kasan_check_write(p: *const core::ffi::c_void, size: u32) -> bool;
}

#[cfg(not(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS")))]
#[inline]
pub unsafe fn __kasan_check_read(_p: *const core::ffi::c_void, _size: u32) -> bool {
    true
}

#[cfg(not(any(feature = "CONFIG_KASAN_GENERIC", feature = "CONFIG_KASAN_SW_TAGS")))]
#[inline]
pub unsafe fn __kasan_check_write(_p: *const core::ffi::c_void, _size: u32) -> bool {
    true
}

/*
 * kasan_check_*: Only available when the particular compilation unit has KASAN
 * instrumentation enabled. May be used in header files.
 */
#[cfg(feature = "__SANITIZE_ADDRESS__")]
#[inline]
pub unsafe fn kasan_check_read(p: *const core::ffi::c_void, size: u32) -> bool {
    __kasan_check_read(p, size)
}

#[cfg(feature = "__SANITIZE_ADDRESS__")]
#[inline]
pub unsafe fn kasan_check_write(p: *const core::ffi::c_void, size: u32) -> bool {
    __kasan_check_write(p, size)
}

#[cfg(not(feature = "__SANITIZE_ADDRESS__"))]
#[inline]
pub unsafe fn kasan_check_read(_p: *const core::ffi::c_void, _size: u32) -> bool {
    true
}

#[cfg(not(feature = "__SANITIZE_ADDRESS__"))]
#[inline]
pub unsafe fn kasan_check_write(_p: *const core::ffi::c_void, _size: u32) -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
