// SPDX-License-Identifier: GPL-2.0+
/*
 * g_hid.h -- Header file for USB HID gadget driver
 *
 * Copyright (C) 2010 Fabien Chouteau <fabien.chouteau@barco.com>
 */

#[repr(C)]
pub struct hidg_func_descriptor {
    pub subclass: u8,
    pub protocol: u8,
    pub report_length: u16,
    pub report_desc_length: u16,
    // Flexible array member; additional descriptor bytes follow this header.
    pub report_desc: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
