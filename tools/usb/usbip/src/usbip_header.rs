// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

// C header guard removed for Rust translation.
// If HAVE_CONFIG_H is set in the C build, this header includes "../config.h".

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    /* usbip commands */
    pub fn usbip_attach(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn usbip_detach(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn usbip_list(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn usbip_bind(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn usbip_unbind(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn usbip_port_show(argc: c_int, argv: *mut *mut c_char) -> c_int;

    pub fn usbip_attach_usage();
    pub fn usbip_detach_usage();
    pub fn usbip_list_usage();
    pub fn usbip_bind_usage();
    pub fn usbip_unbind_usage();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
