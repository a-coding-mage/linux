/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Boolean types support for NOLIBC
 * Copyright (C) 2024 Thomas Weißschuh <linux@weissschuh.net>
 */

/* C header guard _NOLIBC_STDBOOL_H omitted in Rust. */

pub type bool = core::ffi::c_char;

pub const true_: bool = 1;
pub const false_: bool = 0;

pub const __bool_true_false_are_defined: core::ffi::c_int = 1;
