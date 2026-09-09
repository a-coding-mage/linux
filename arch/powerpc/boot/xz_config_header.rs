/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Most of this is copied from lib/xz/xz_private.h; we can't use their defines
 * since the boot wrapper is not built in the same environment as the rest of
 * the kernel.
 *
 * The C includes are intentionally omitted; the referenced types and
 * functions are supplied by the corresponding Rust dependencies.
 */

#[inline]
pub unsafe fn swab32p(p: *mut core::ffi::c_void) -> u32 {
    let q = p as *const u32;
    swab32(unsafe { *q })
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! get_le32 {
    ($p:expr) => {{
        unsafe { *((($p) as *const u32)) }
    }};
}

#[cfg(target_endian = "little")]
#[inline]
pub fn cpu_to_be32(x: u32) -> u32 {
    swab32(x)
}

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn be32_to_cpup(p: *const u32) -> u32 {
    swab32p(p as *mut core::ffi::c_void)
}

#[cfg(not(target_endian = "little"))]
#[macro_export]
macro_rules! get_le32 {
    ($p:expr) => {{
        unsafe { $crate::swab32p(($p) as *mut core::ffi::c_void) }
    }};
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub fn cpu_to_be32(x: u32) -> u32 {
    x
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub unsafe fn be32_to_cpup(p: *const u32) -> u32 {
    unsafe { *p }
}

#[inline]
pub unsafe fn get_unaligned_be32(p: *const core::ffi::c_void) -> u32 {
    be32_to_cpup(p as *const u32)
}

#[inline]
pub unsafe fn put_unaligned_be32(val: u32, p: *mut core::ffi::c_void) {
    unsafe { *(p as *mut u32) = cpu_to_be32(val) };
}

#[macro_export]
macro_rules! memeq {
    ($a:expr, $b:expr, $size:expr) => {{
        unsafe { memcmp($a, $b, $size) == 0 }
    }};
}

#[macro_export]
macro_rules! memzero {
    ($buf:expr, $size:expr) => {{
        unsafe { memset($buf, 0, $size) }
    }};
}

/* Prevent the inclusion of the xz-preboot MM headers. */
/* #define DECOMPR_MM_H */
/* #define memmove memmove */

/* xz.h needs to be included directly since we need enum xz_mode. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
