/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Translated from jfs_unicode.h.  The included kernel and NLS declarations
// are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

pub type __le16 = u16;
pub type size_t = usize;
pub type wchar_t = u32;

#[repr(C)]
pub struct component_name {
    pub name: *mut c_char,
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nls_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct UniCaseRange {
    pub start: wchar_t,
    pub end: wchar_t,
    pub table: *const i16,
}

extern "C" {
    pub fn get_UCSname(comp: *mut component_name, dentry: *mut dentry) -> c_int;
    pub fn jfs_strfromUCS_le(
        out: *mut c_char,
        in_: *const __le16,
        len: c_int,
        nls: *mut nls_table,
    ) -> c_int;

    pub static NlsUniUpperTable: [i16; 0];
    pub static NlsUniUpperRange: *const UniCaseRange;
}

// #define free_UCSname(COMP) kfree((COMP)->name)
#[inline]
pub unsafe fn free_UCSname(comp: *mut component_name) {
    // The kernel's kfree is supplied by the surrounding translation unit.
    extern "C" {
        fn kfree(ptr: *mut c_void);
    }
    kfree((*comp).name.cast::<c_void>());
}

/*
 * UniStrcpy:  Copy a string
 */
#[inline]
pub unsafe fn UniStrcpy(mut ucs1: *mut wchar_t, mut ucs2: *const wchar_t) -> *mut wchar_t {
    let anchor = ucs1;
    loop {
        *ucs1 = *ucs2;
        ucs1 = ucs1.add(1);
        ucs2 = ucs2.add(1);
        if *ucs1.sub(1) == 0 {
            break;
        }
    }
    anchor
}

/*
 * UniStrncpy:  Copy length limited string with pad
 */
#[inline]
pub unsafe fn UniStrncpy_le(
    mut ucs1: *mut __le16,
    mut ucs2: *const __le16,
    mut n: size_t,
) -> *mut __le16 {
    let anchor = ucs1;
    while n != 0 && *ucs2 != 0 {
        *ucs1 = *ucs2;
        ucs1 = ucs1.add(1);
        ucs2 = ucs2.add(1);
        n -= 1;
    }
    n += 1;
    while n != 0 {
        *ucs1 = 0;
        ucs1 = ucs1.add(1);
        n -= 1;
    }
    anchor
}

/* UniStrncmp_le: Compare length limited string - native to little-endian */
#[inline]
pub unsafe fn UniStrncmp_le(mut ucs1: *const wchar_t, mut ucs2: *const __le16, mut n: size_t) -> c_int {
    if n == 0 { return 0; }
    while *ucs1 == u16::from_le(*ucs2) as wchar_t && *ucs1 != 0 && n > 1 {
        ucs1 = ucs1.add(1);
        ucs2 = ucs2.add(1);
        n -= 1;
    }
    (*ucs1 as c_int) - (u16::from_le(*ucs2) as c_int)
}

/* UniStrncpy_to_le: Copy length limited string with pad to little-endian */
#[inline]
pub unsafe fn UniStrncpy_to_le(mut ucs1: *mut __le16, mut ucs2: *const wchar_t, mut n: size_t) -> *mut __le16 {
    let anchor = ucs1;
    while n != 0 && *ucs2 != 0 {
        *ucs1 = (*ucs2 as u16).to_le();
        ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); n -= 1;
    }
    n += 1;
    while n != 0 { *ucs1 = 0; ucs1 = ucs1.add(1); n -= 1; }
    anchor
}

/* UniStrncpy_from_le: Copy length limited string with pad from little-endian */
#[inline]
pub unsafe fn UniStrncpy_from_le(mut ucs1: *mut wchar_t, mut ucs2: *const __le16, mut n: size_t) -> *mut wchar_t {
    let anchor = ucs1;
    while n != 0 && *ucs2 != 0 {
        *ucs1 = u16::from_le(*ucs2) as wchar_t;
        ucs1 = ucs1.add(1); ucs2 = ucs2.add(1); n -= 1;
    }
    n += 1;
    while n != 0 { *ucs1 = 0; ucs1 = ucs1.add(1); n -= 1; }
    anchor
}

/* UniToupper: Convert a unicode character to upper case */
#[inline]
pub unsafe fn UniToupper(uc: wchar_t) -> wchar_t {
    if (uc as usize) < NlsUniUpperTable.len() {
        uc.wrapping_add(NlsUniUpperTable[uc as usize] as wchar_t)
    } else {
        let mut rp = NlsUniUpperRange;
        while (*rp).start != 0 {
            if uc < (*rp).start { return uc; }
            if uc <= (*rp).end {
                return uc.wrapping_add(*(*rp).table.add((uc - (*rp).start) as usize) as wchar_t);
            }
            rp = rp.add(1);
        }
        uc
    }
}

/* UniStrupr: Upper case a unicode string */
#[inline]
pub unsafe fn UniStrupr(upin: *mut wchar_t) -> *mut wchar_t {
    let mut up = upin;
    while *up != 0 { *up = UniToupper(*up); up = up.add(1); }
    upin
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
