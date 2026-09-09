/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/unicode.h.
// Dependencies corresponding to <linux/init.h> and <linux/dcache.h> are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct utf8data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utf8data_table {
    _private: [u8; 0],
}

pub const UNICODE_MAJ_SHIFT: u32 = 16;
pub const UNICODE_MIN_SHIFT: u32 = 8;

#[inline]
pub const fn unicode_age(maj: u32, min: u32, rev: u32) -> u32 {
    (maj << UNICODE_MAJ_SHIFT) | (min << UNICODE_MIN_SHIFT) | rev
}

pub const UTF8_LATEST: u32 = unicode_age(12, 1, 0);

#[inline]
pub fn unicode_major(age: u32) -> u8 {
    ((age >> UNICODE_MAJ_SHIFT) & 0xff) as u8
}

#[inline]
pub fn unicode_minor(age: u32) -> u8 {
    ((age >> UNICODE_MIN_SHIFT) & 0xff) as u8
}

#[inline]
pub fn unicode_rev(age: u32) -> u8 {
    (age & 0xff) as u8
}

/*
 * Two normalization forms are supported:
 * 1) NFDI
 *   - Apply unicode normalization form NFD.
 *   - Remove any Default_Ignorable_Code_Point.
 * 2) NFDICF
 *   - Apply unicode normalization form NFD.
 *   - Remove any Default_Ignorable_Code_Point.
 *   - Apply a full casefold (C + F).
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum utf8_normalization {
    UTF8_NFDI = 0,
    UTF8_NFDICF,
    UTF8_NMAX,
}

#[repr(C)]
pub struct unicode_map {
    pub version: u32,
    pub ntab: [*const utf8data; UTF8_NMAX as usize],
    pub tables: *const utf8data_table,
}

// `struct qstr` is declared by <linux/dcache.h> in the source header.
#[repr(C)]
pub struct qstr {
    _private: [u8; 0],
}

extern "C" {
    pub fn utf8_validate(um: *const unicode_map, str_: *const qstr) -> i32;

    pub fn utf8_strncmp(
        um: *const unicode_map,
        s1: *const qstr,
        s2: *const qstr,
    ) -> i32;

    pub fn utf8_strncasecmp(
        um: *const unicode_map,
        s1: *const qstr,
        s2: *const qstr,
    ) -> i32;

    pub fn utf8_strncasecmp_folded(
        um: *const unicode_map,
        cf: *const qstr,
        s1: *const qstr,
    ) -> i32;

    pub fn utf8_casefold(
        um: *const unicode_map,
        str_: *const qstr,
        dest: *mut u8,
        dlen: usize,
    ) -> i32;

    pub fn utf8_casefold_hash(
        um: *const unicode_map,
        salt: *const core::ffi::c_void,
        str_: *mut qstr,
    ) -> i32;

    pub fn utf8_load(version: u32) -> *mut unicode_map;
    pub fn utf8_unload(um: *mut unicode_map);

    pub fn utf8_parse_version(version: *mut core::ffi::c_char) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
