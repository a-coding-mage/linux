/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Some of the source code in this file came from fs/cifs/cifs_unicode.c
 * and then via server/unicode.c
 * cifs_unicode:  Unicode kernel case support
 *
 * Function:
 *     Convert a unicode character to upper or lower case using
 *     compressed tables.
 *
 *   Copyright (c) International Business Machines  Corp., 2000,2009
 *
 * Notes:
 *     These APIs are based on the C library functions.  The semantics
 *     should match the C functions but with expanded size operands.
 *
 *     The upper/lower functions are based on a table created by mkupr.
 *     This is a compressed table of upper and lower case conversion.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, including nls_ucs2_data.h and little-endian conversions.

pub type __le16 = u16;
pub type wchar_t = u16;

#[repr(C)]
pub struct UniCaseRange {
    pub start: wchar_t,
    pub end: wchar_t,
    pub table: *const i16,
}

extern "C" {
    pub static NlsUniUpperTable: [i16; 0];
    pub static NlsUniUpperRange: UniCaseRange;
    pub fn __le16_to_cpu(value: __le16) -> wchar_t;
    pub fn cpu_to_le16(value: wchar_t) -> __le16;
}

pub const UNI_ASTERISK: u16 = b'*' as u16 + 0xF000;
pub const UNI_QUESTION: u16 = b'?' as u16 + 0xF000;
pub const UNI_COLON: u16 = b':' as u16 + 0xF000;
pub const UNI_GRTRTHAN: u16 = b'>' as u16 + 0xF000;
pub const UNI_LESSTHAN: u16 = b'<' as u16 + 0xF000;
pub const UNI_PIPE: u16 = b'|' as u16 + 0xF000;
pub const UNI_SLASH: u16 = b'\\' as u16 + 0xF000;

pub unsafe fn UniStrcat(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t) -> *mut wchar_t {
    let anchor = ucs1;
    while *ucs1 != 0 { ucs1 = ucs1.add(1); }
    while { *ucs1 = *ucs2; ucs2 = ucs2.add(1); *ucs1 != 0 } { ucs1 = ucs1.add(1); }
    anchor
}

pub unsafe fn UniStrchr(mut ucs: *const wchar_t, uc: wchar_t) -> *mut wchar_t {
    while *ucs != uc && *ucs != 0 { ucs = ucs.add(1); }
    if *ucs == uc { ucs as *mut wchar_t } else { core::ptr::null_mut() }
}

pub unsafe fn UniStrcmp(mut ucs1: *const wchar_t, mut ucs2: *const wchar_t) -> i32 {
    while *ucs1 == *ucs2 && *ucs1 != 0 { ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); }
    (*ucs1 as i32) - (*ucs2 as i32)
}

pub unsafe fn UniStrcpy(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t) -> *mut wchar_t {
    let anchor = ucs1;
    loop { *ucs1 = *ucs2; let done = *ucs1 == 0; ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); if done { break; } }
    anchor
}

pub unsafe fn UniStrlen(mut ucs1: *const wchar_t) -> usize {
    let mut i: i32 = 0; while *ucs1 != 0 { ucs1 = ucs1.add(1); i += 1; } i as usize
}

pub unsafe fn UniStrnlen(mut ucs1: *const wchar_t, maxlen: i32) -> usize {
    let mut i: i32 = 0; while *ucs1 != 0 { ucs1 = ucs1.add(1); i += 1; if i >= maxlen { break; } } i as usize
}

pub unsafe fn UniStrncat(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t, mut n: usize) -> *mut wchar_t {
    let anchor = ucs1; while *ucs1 != 0 { ucs1 = ucs1.add(1); }
    while n != 0 && *ucs2 != 0 { *ucs1 = *ucs2; ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); n -= 1; }
    *ucs1 = 0; anchor
}

pub unsafe fn UniStrncmp(mut ucs1: *const wchar_t, mut ucs2: *const wchar_t, mut n: usize) -> i32 {
    if n == 0 { return 0; }
    while *ucs1 == *ucs2 && *ucs1 != 0 && { n -= 1; n != 0 } { ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); }
    (*ucs1 as i32) - (*ucs2 as i32)
}

pub unsafe fn UniStrncmp_le(mut ucs1: *const wchar_t, mut ucs2: *const wchar_t, mut n: usize) -> i32 {
    if n == 0 { return 0; }
    while *ucs1 == __le16_to_cpu(*ucs2 as __le16) && *ucs1 != 0 && { n -= 1; n != 0 } { ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); }
    (*ucs1 as i32) - (__le16_to_cpu(*ucs2 as __le16) as i32)
}

pub unsafe fn UniStrncpy(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t, mut n: usize) -> *mut wchar_t {
    let anchor = ucs1; while n != 0 && *ucs2 != 0 { *ucs1 = *ucs2; ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); n -= 1; }
    while n != 0 { *ucs1 = 0; ucs1 = ucs1.add(1); n -= 1; } anchor
}

pub unsafe fn UniStrncpy_le(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t, mut n: usize) -> *mut wchar_t {
    let anchor = ucs1; while n != 0 && *ucs2 != 0 { *ucs1 = __le16_to_cpu(*ucs2 as __le16); ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); n -= 1; }
    while n != 0 { *ucs1 = 0; ucs1 = ucs1.add(1); n -= 1; } anchor
}

pub unsafe fn UniStrstr(mut ucs1: *const wchar_t, ucs2: *const wchar_t) -> *mut wchar_t {
    let mut anchor1 = ucs1; let anchor2 = ucs2; let mut needle = ucs2;
    while *ucs1 != 0 { if *ucs1 == *needle { ucs1 = ucs1.add(1); needle = needle.add(1); } else { if *needle == 0 { return anchor1 as *mut wchar_t; } anchor1 = anchor1.add(1); ucs1 = anchor1; needle = anchor2; } }
    if *needle == 0 { anchor1 as *mut wchar_t } else { core::ptr::null_mut() }
}

pub unsafe fn UniToupper(mut uc: wchar_t) -> wchar_t {
    if (uc as usize) < NlsUniUpperTable.len() { return uc.wrapping_add(NlsUniUpperTable[uc as usize] as wchar_t); }
    let mut rp = &NlsUniUpperRange as *const UniCaseRange;
    while (*rp).start != 0 { if uc < (*rp).start { return uc; } if uc <= (*rp).end { return uc.wrapping_add(*((*rp).table.add((uc - (*rp).start) as usize)) as wchar_t); } rp = rp.add(1); }
    uc
}

pub unsafe fn UniStrupr(mut upin: *mut __le16) -> *mut __le16 {
    let up = upin; while *upin != 0 { *upin = cpu_to_le16(UniToupper(__le16_to_cpu(*upin))); upin = upin.add(1); } up
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
