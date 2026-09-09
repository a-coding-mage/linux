/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * altera.h
 *
 * altera FPGA driver
 *
 * Copyright (C) Altera Corporation 1998-2001
 * Copyright (C) 2010 NetUP Inc.
 * Copyright (C) 2010 Igor M. Liplianin <liplianin@netup.ru>
 */

use core::ffi::{c_char, c_int, c_void};

/* Supplied by the firmware subsystem. */
#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct altera_config {
    pub dev: *mut c_void,
    pub action: *mut u8,
    pub jtag_io: Option<unsafe extern "C" fn(
        dev: *mut c_void,
        tms: c_int,
        tdi: c_int,
        tdo: c_int,
    ) -> c_int>,
}

/* The Kconfig condition is preserved through these Rust feature names. */
#[cfg(any(feature = "CONFIG_ALTERA_STAPL", all(feature = "CONFIG_ALTERA_STAPL_MODULE", feature = "MODULE")))]
extern "C" {
    pub fn altera_init(config: *mut altera_config, fw: *const firmware) -> c_int;
}

#[cfg(not(any(feature = "CONFIG_ALTERA_STAPL", all(feature = "CONFIG_ALTERA_STAPL_MODULE", feature = "MODULE"))))]
#[inline]
pub unsafe fn altera_init(_config: *mut altera_config, _fw: *const firmware) -> c_int {
    /* printk(KERN_WARNING "%s: driver disabled by Kconfig\n", __func__); */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
