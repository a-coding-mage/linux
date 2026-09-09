// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependency intent: <linux/module.h>

// EXPORT_SYMBOL declarations below expose these externally supplied symbols
// to loadable modules. The corresponding implementations are provided by the
// surrounding kernel environment.
unsafe extern "C" {
    pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
    pub fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> i32;
    pub fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strlen(s: *const core::ffi::c_char) -> usize;
}

// CONFIG_HAVE_EFFICIENT_UNALIGNED_STRING_OPS
// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memset);
// EXPORT_SYMBOL(memmove);

// EXPORT_SYMBOL(memcmp);
// EXPORT_SYMBOL(strcmp);
// EXPORT_SYMBOL(strcpy);
// EXPORT_SYMBOL(strlen);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
