/* SPDX-License-Identifier: GPL-2.0 */

/*
 * We don't do inline string functions, since the
 * optimised inline asm versions are not small.
 *
 * The __underscore versions of some functions are for KASan to be able
 * to replace them with instrumented versions.
 */

pub const __HAVE_ARCH_STRRCHR: bool = true;
unsafe extern "C" {
    pub fn strrchr(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
}

pub const __HAVE_ARCH_STRCHR: bool = true;
unsafe extern "C" {
    pub fn strchr(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
}

pub const __HAVE_ARCH_MEMCPY: bool = true;
unsafe extern "C" {
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
    pub fn __memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMMOVE: bool = true;
unsafe extern "C" {
    pub fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
    pub fn __memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMCHR: bool = true;
unsafe extern "C" {
    pub fn memchr(
        s: *const core::ffi::c_void,
        c: i32,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMSET: bool = true;
unsafe extern "C" {
    pub fn memset(
        s: *mut core::ffi::c_void,
        c: i32,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
    pub fn __memset(
        s: *mut core::ffi::c_void,
        c: i32,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMSET32: bool = true;
unsafe extern "C" {
    pub fn __memset32(p: *mut u32, v: u32, n: __kernel_size_t) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn memset32(p: *mut u32, v: u32, n: __kernel_size_t) -> *mut core::ffi::c_void {
    unsafe { __memset32(p, v, n.wrapping_mul(4)) }
}

pub const __HAVE_ARCH_MEMSET64: bool = true;
unsafe extern "C" {
    pub fn __memset64(
        p: *mut u64,
        first: u32,
        n: __kernel_size_t,
        second: u32,
    ) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct memset64_word_parts {
    pub first: u32,
    pub second: u32,
}

#[repr(C)]
pub union memset64_word {
    pub val: u64,
    pub parts: memset64_word_parts,
}

#[inline]
pub unsafe fn memset64(p: *mut u64, v: u64, n: __kernel_size_t) -> *mut core::ffi::c_void {
    let word = memset64_word { val: v };
    let parts = unsafe { word.parts };
    unsafe { __memset64(p, parts.first, n.wrapping_mul(8), parts.second) }
}

/*
 * For files that are not instrumented (e.g. mm/slub.c) we
 * must use non-instrumented versions of the mem*
 * functions named __memcpy() etc. All such kernel code
 * has been tagged with KASAN_SANITIZE_file.o = n, which
 * means that the address sanitization argument isn't passed
 * to the compiler, and __SANITIZE_ADDRESS__ is not set.
 * As a result these defines kick in when CONFIG_KASAN is
 * enabled and __SANITIZE_ADDRESS__ is not defined:
 *
 * memcpy(dst, src, len) -> __memcpy(dst, src, len)
 * memmove(dst, src, len) -> __memmove(dst, src, len)
 * memset(s, c, n) -> __memset(s, c, n)
 *
 * __NO_FORTIFY is likewise defined in that configuration.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
