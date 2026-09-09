// SPDX-License-Identifier: GPL-2.0
// __NO_FORTIFY
//
// This file exports some critical string functions and compiler
// built-in functions (where calls are emitted by the compiler
// itself that we cannot avoid even in kernel code) to modules.
//
// "_user.c" code that previously used exports here such as hostfs
// really should be considered part of the 'hypervisor' and define
// its own API boundary like hostfs does now; don't add exports to
// this file for such cases.

use core::ffi::{c_char, c_int, c_void};

// If it's not defined, the export is included in lib/string.c.
// Preserves the __HAVE_ARCH_STRSTR conditional.
#[cfg(__HAVE_ARCH_STRSTR)]
extern "C" {
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}
// EXPORT_SYMBOL(strstr);

// Preserves the !__x86_64__ conditional.
#[cfg(not(target_arch = "x86_64"))]
extern "C" {
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn memset(dest: *mut c_void, value: c_int, n: usize) -> *mut c_void;
}
// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memmove);
// EXPORT_SYMBOL(memset);

// Preserves the _FORTIFY_SOURCE conditional.
#[cfg(_FORTIFY_SOURCE)]
extern "C" {
    pub fn __sprintf_chk(
        str_: *mut c_char,
        flag: c_int,
        len: usize,
        format: *const c_char,
    ) -> c_int;
}
// EXPORT_SYMBOL(__sprintf_chk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
