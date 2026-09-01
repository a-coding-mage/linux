// SPDX-License-Identifier: GPL-2.0
/*
 * PC-Speaker driver for Linux
 *
 * Copyright (C) 2001-2008  Stas Sergeev
 */

// Header guard __PCSP_INPUT_H__ omitted in Rust.

unsafe extern "C" {
    pub fn pcspkr_input_init(rdev: *mut *mut input_dev, dev: *mut device) -> ::core::ffi::c_int;
    pub fn pcspkr_stop_sound();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
