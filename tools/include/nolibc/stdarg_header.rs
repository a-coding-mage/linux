/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Variadic argument support for NOLIBC
 * Copyright (C) 2005-2020 Rich Felker, et al.
 */

// C header guard `_NOLIBC_STDARG_H` omitted in Rust.

// typedef __builtin_va_list va_list;
pub type va_list = core::ffi::VaListImpl<'static>;

// The original C macros map directly to compiler builtins:
// #define va_start(v, l)   __builtin_va_start(v, l)
// #define va_end(v)        __builtin_va_end(v)
// #define va_arg(v, l)     __builtin_va_arg(v, l)
// #define va_copy(d, s)    __builtin_va_copy(d, s)
//
// Rust does not expose stable file-local equivalents for these C variadic
// builtins. Code using this translated header should use Rust's variadic
// argument support or target-provided bindings for the corresponding builtins.
