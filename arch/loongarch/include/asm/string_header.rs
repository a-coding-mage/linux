/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

//! Rust translation of the LoongArch string header.
//!
//! The C header's `CONFIG_64BIT` and KASAN conditions are represented with
//! Rust configuration predicates where applicable.  The original symbols
//! supplied by other headers or build configuration remain external.

use core::ffi::c_void;

#[cfg(feature = "CONFIG_64BIT")]
pub const __HAVE_ARCH_MEMSET: () = ();

#[cfg(feature = "CONFIG_64BIT")]
extern "C" {
    pub fn memset(s: *mut c_void, c: i32, count: usize) -> *mut c_void;
    pub fn __memset(s: *mut c_void, c: i32, count: usize) -> *mut c_void;
}

#[cfg(feature = "CONFIG_64BIT")]
pub const __HAVE_ARCH_MEMCPY: () = ();

#[cfg(feature = "CONFIG_64BIT")]
extern "C" {
    pub fn memcpy(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
    pub fn __memcpy(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
}

#[cfg(feature = "CONFIG_64BIT")]
pub const __HAVE_ARCH_MEMMOVE: () = ();

#[cfg(feature = "CONFIG_64BIT")]
extern "C" {
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn __memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/*
 * For files that are not instrumented (e.g. mm/slub.c) we
 * should use not instrumented version of mem* functions.
 *
 * The C condition is:
 *   defined(CONFIG_KASAN) && !defined(__SANITIZE_ADDRESS__)
 */
#[cfg(all(feature = "CONFIG_KASAN", not(feature = "__SANITIZE_ADDRESS__")))]
#[macro_export]
macro_rules! memset {
    ($s:expr, $c:expr, $n:expr) => { $crate::__memset($s, $c, $n) };
}

#[cfg(all(feature = "CONFIG_KASAN", not(feature = "__SANITIZE_ADDRESS__")))]
#[macro_export]
macro_rules! memcpy {
    ($dst:expr, $src:expr, $len:expr) => { $crate::__memcpy($dst, $src, $len) };
}

#[cfg(all(feature = "CONFIG_KASAN", not(feature = "__SANITIZE_ADDRESS__")))]
#[macro_export]
macro_rules! memmove {
    ($dst:expr, $src:expr, $len:expr) => { $crate::__memmove($dst, $src, $len) };
}

/* FORTIFY_SOURCE uses __builtin_memcpy, etc. */
#[cfg(all(
    feature = "CONFIG_KASAN",
    not(feature = "__SANITIZE_ADDRESS__"),
    not(feature = "__NO_FORTIFY")
))]
pub const __NO_FORTIFY: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
