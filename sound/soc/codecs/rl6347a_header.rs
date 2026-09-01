/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rl6347a.h - RL6347A class device shared support
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 *
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// C dependency: <sound/hda_verbs.h> provides AC_VERB_SET_COEF_INDEX and
// AC_VERB_SET_PROC_COEF.

use core::ffi::{c_int, c_uint, c_void};

pub const fn VERB_CMD(V: c_uint, N: c_uint, D: c_uint) -> c_uint {
    (N << 20) | (V << 8) | D
}

pub const RL6347A_VENDOR_REGISTERS: c_uint = 0x20;

pub const RL6347A_COEF_INDEX: c_uint =
    VERB_CMD(AC_VERB_SET_COEF_INDEX, RL6347A_VENDOR_REGISTERS, 0);
pub const RL6347A_PROC_COEF: c_uint =
    VERB_CMD(AC_VERB_SET_PROC_COEF, RL6347A_VENDOR_REGISTERS, 0);

// External C type from another header.
#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rl6347a_priv {
    pub index_cache: *mut reg_default,
    pub index_cache_size: c_int,
}

unsafe extern "C" {
    pub fn rl6347a_hw_write(context: *mut c_void, reg: c_uint, value: c_uint) -> c_int;
    pub fn rl6347a_hw_read(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
