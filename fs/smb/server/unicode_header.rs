/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Some of the source code in this file came from fs/cifs/cifs_unicode.c
 * cifs_unicode:  Unicode kernel case support
 *
 * Function:
 *     Convert a unicode character to upper or lower case using
 *     compressed tables.
 *
 *   Copyright (c) International Business Machines  Corp., 2000,2009
 *
 *
 * Notes:
 *     These APIs are based on the C library functions.  The semantics
 *     should match the C functions but with expanded size operands.
 *
 *     The upper/lower functions are based on a table created by mkupr.
 *     This is a compressed table of upper and lower case conversion.
 *
 */

// The original header includes declarations supplied by the kernel and NLS
// support headers. They remain external dependencies of this translation.

use core::ffi::{c_char, c_int};

pub type __le16 = u16;

#[repr(C)]
pub struct nls_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unicode_map {
    _private: [u8; 0],
}

// Original condition: #ifdef __KERNEL__
#[cfg(__KERNEL__)]
extern "C" {
    pub fn smb_strtoUTF16(
        to: *mut __le16,
        from: *const c_char,
        len: c_int,
        codepage: *const nls_table,
    ) -> c_int;

    pub fn smb_strndup_from_utf16(
        src: *const c_char,
        maxlen: c_int,
        is_unicode: bool,
        codepage: *const nls_table,
    ) -> *mut c_char;

    pub fn smbConvertToUTF16(
        target: *mut __le16,
        source: *const c_char,
        srclen: c_int,
        cp: *const nls_table,
        mapchars: c_int,
    ) -> c_int;

    pub fn ksmbd_extract_sharename(
        um: *mut unicode_map,
        treename: *const c_char,
    ) -> *mut c_char;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
