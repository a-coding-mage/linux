/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

/* Plane-0 Unicode character */
pub type wchar_t = u16;
pub const MAX_WCHAR_T: u16 = 0xffff;

/* Arbitrary Unicode character */
pub type unicode_t = u32;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nls_table {
    pub charset: *const core::ffi::c_char,
    pub alias: *const core::ffi::c_char,
    pub uni2char: Option<unsafe extern "C" fn(uni: wchar_t, out: *mut u8, boundlen: i32) -> i32>,
    pub char2uni: Option<unsafe extern "C" fn(rawstring: *const u8, boundlen: i32, uni: *mut wchar_t) -> i32>,
    pub charset2lower: *const u8,
    pub charset2upper: *const u8,
    pub owner: *mut module,
    pub next: *mut nls_table,
}

/* this value hold the maximum octet of charset */
pub const NLS_MAX_CHARSET_SIZE: usize = 6; /* for UTF-8 */

/* Byte order for UTF-16 strings */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum utf16_endian {
    UTF16_HOST_ENDIAN,
    UTF16_LITTLE_ENDIAN,
    UTF16_BIG_ENDIAN,
}

/* nls_base.c */
unsafe extern "C" {
    pub fn __register_nls(table: *mut nls_table, owner: *mut module) -> i32;
    pub fn unregister_nls(table: *mut nls_table) -> i32;
    pub fn load_nls(charset: *const core::ffi::c_char) -> *mut nls_table;
    pub fn unload_nls(table: *mut nls_table);
    pub fn load_nls_default() -> *mut nls_table;

    pub fn utf8_to_utf32(s: *const u8, len: i32, pu: *mut unicode_t) -> i32;
    pub fn utf32_to_utf8(u: unicode_t, s: *mut u8, maxlen: i32) -> i32;
    pub fn utf8s_to_utf16s(
        s: *const u8,
        len: i32,
        endian: utf16_endian,
        pwcs: *mut wchar_t,
        maxlen: i32,
    ) -> i32;
    pub fn utf16s_to_utf8s(
        pwcs: *const wchar_t,
        len: i32,
        endian: utf16_endian,
        s: *mut u8,
        maxlen: i32,
    ) -> i32;
}

#[inline]
pub unsafe fn nls_tolower(t: *mut nls_table, c: u8) -> u8 {
    let nc = *(*t).charset2lower.add(c as usize);
    if nc != 0 { nc } else { c }
}

#[inline]
pub unsafe fn nls_toupper(t: *mut nls_table, c: u8) -> u8 {
    let nc = *(*t).charset2upper.add(c as usize);
    if nc != 0 { nc } else { c }
}

#[inline]
pub unsafe fn nls_strnicmp(t: *mut nls_table, mut s1: *const u8, mut s2: *const u8, mut len: i32) -> i32 {
    while len != 0 {
        let c1 = *s1;
        s1 = s1.add(1);
        let c2 = *s2;
        s2 = s2.add(1);
        if nls_tolower(t, c1) != nls_tolower(t, c2) { return 1; }
        len -= 1;
    }
    0
}

/*
 * nls_nullsize - return length of null character for codepage
 * @codepage - codepage for which to return length of NULL terminator
 *
 * Since we can't guarantee that the null terminator will be a particular
 * length, we have to check against the codepage. If there's a problem
 * determining it, assume a single-byte NULL terminator.
 */
#[inline]
pub unsafe fn nls_nullsize(codepage: *const nls_table) -> i32 {
    let mut tmp = [0u8; NLS_MAX_CHARSET_SIZE];
    let charlen = ((*codepage).uni2char.expect("uni2char")).call(0, tmp.as_mut_ptr(), NLS_MAX_CHARSET_SIZE as i32);
    if charlen > 0 { charlen } else { 1 }
}

/* Build-time macro equivalents; THIS_MODULE and MODULE_ALIAS are external. */
#[macro_export]
macro_rules! register_nls {
    ($nls:expr) => { $crate::__register_nls($nls, THIS_MODULE) };
}

#[macro_export]
macro_rules! MODULE_ALIAS_NLS {
    ($name:ident) => { MODULE_ALIAS!(concat!("nls_", stringify!($name))) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
