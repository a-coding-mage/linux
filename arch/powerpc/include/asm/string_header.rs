/* SPDX-License-Identifier: GPL-2.0 */

/* This header is active only for kernel builds. */

/* When CONFIG_KASAN is not enabled:
 * __HAVE_ARCH_STRNCMP, __HAVE_ARCH_MEMCHR, __HAVE_ARCH_MEMCMP, and
 * __HAVE_ARCH_MEMSET16 are defined.
 */
pub const __HAVE_ARCH_MEMSET: bool = true;
pub const __HAVE_ARCH_MEMCPY: bool = true;
pub const __HAVE_ARCH_MEMMOVE: bool = true;
pub const __HAVE_ARCH_MEMCPY_FLUSHCACHE: bool = true;

extern "C" {
    pub fn strcpy(dest: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char)
        -> *mut ::core::ffi::c_char;
    pub fn strlen(s: *const ::core::ffi::c_char) -> __kernel_size_t;
    pub fn strcmp(a: *const ::core::ffi::c_char, b: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn strncmp(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
        n: __kernel_size_t,
    ) -> ::core::ffi::c_int;
    pub fn strcat(dest: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char)
        -> *mut ::core::ffi::c_char;
    pub fn memset(s: *mut ::core::ffi::c_void, c: ::core::ffi::c_int, n: __kernel_size_t)
        -> *mut ::core::ffi::c_void;
    pub fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memmove(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memcmp(
        a: *const ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        n: __kernel_size_t,
    ) -> ::core::ffi::c_int;
    pub fn memchr(
        s: *const ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: __kernel_size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memcpy_flushcache(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        size: usize,
    );
}

/* CONFIG_KASAN conditional declarations and aliases. */
extern "C" {
    pub fn __memset(s: *mut ::core::ffi::c_void, c: ::core::ffi::c_int, count: __kernel_size_t)
        -> *mut ::core::ffi::c_void;
    pub fn __memcpy(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn __memmove(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut ::core::ffi::c_void;
}

/* CONFIG_CC_HAS_KASAN_MEMINTRINSIC_PREFIX may instead alias __memset,
 * __memcpy, and __memmove to the ordinary intrinsics. */
/* For non-instrumented files, CONFIG_KASAN aliases memcpy, memmove, and
 * memset to the corresponding __mem functions and defines __NO_FORTIFY. */

/* CONFIG_PPC64, when CONFIG_KASAN is not enabled. */
pub const __HAVE_ARCH_MEMSET32: bool = true;
pub const __HAVE_ARCH_MEMSET64: bool = true;

extern "C" {
    pub fn __memset16(p: *mut u16, v: u16, n: __kernel_size_t) -> *mut ::core::ffi::c_void;
    pub fn __memset32(p: *mut u32, v: u32, n: __kernel_size_t) -> *mut ::core::ffi::c_void;
    pub fn __memset64(p: *mut u64, v: u64, n: __kernel_size_t) -> *mut ::core::ffi::c_void;
}

#[inline]
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn memset16(p: *mut u16, v: u16, n: __kernel_size_t) -> *mut ::core::ffi::c_void {
    __memset16(p, v, n * 2)
}

#[inline]
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn memset32(p: *mut u32, v: u32, n: __kernel_size_t) -> *mut ::core::ffi::c_void {
    __memset32(p, v, n * 4)
}

#[inline]
#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn memset64(p: *mut u64, v: u64, n: __kernel_size_t) -> *mut ::core::ffi::c_void {
    __memset64(p, v, n * 8)
}

/* On non-PPC64 builds, when CONFIG_KASAN is not enabled. */
pub const __HAVE_ARCH_STRLEN: bool = true;

extern "C" {
    #[cfg(not(feature = "CONFIG_PPC64"))]
    pub fn memset16(p: *mut u16, v: u16, n: __kernel_size_t)
        -> *mut ::core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
