/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *      names.h  --  USB name database manipulation routines
 *
 *      Copyright (C) 1999, 2000  Thomas Sailer (sailer@ife.ee.ethz.ch)
 *
 *	Copyright (C) 2005 Takahiro Hirofuchi
 *	       - names_free() is added.
 */

use std::os::raw::{c_char, c_int};

/* used by usbip_common.c */
unsafe extern "C" {
    pub fn names_vendor(vendorid: u16) -> *const c_char;
    pub fn names_product(vendorid: u16, productid: u16) -> *const c_char;
    pub fn names_class(classid: u8) -> *const c_char;
    pub fn names_subclass(classid: u8, subclassid: u8) -> *const c_char;
    pub fn names_protocol(classid: u8, subclassid: u8, protocolid: u8) -> *const c_char;
    pub fn names_init(n: *mut c_char) -> c_int;
    pub fn names_free();
}
