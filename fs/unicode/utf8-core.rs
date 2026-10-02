// SPDX-License-Identifier: GPL-2.0
// Unicode core APIs, translated from the retained utf8-core.c.
// Configured C headers own every shared layout and imported declaration.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/utf8_core_generated.rs"));
}
#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

use bindings::*;
use core::mem::MaybeUninit;
use core::ptr::null;
use kernel::ffi::{c_char, c_int, c_void};

#[inline]
fn neg(error: u32) -> c_int {
    -(error as c_int)
}

#[inline]
fn err_ptr(error: c_int) -> *mut unicode_map {
    error as isize as *mut unicode_map
}

#[no_mangle]
pub unsafe extern "C" fn utf8_validate(um: *const unicode_map, str_: *const qstr) -> c_int {
    if utf8nlen(um, UTF8_NFDI, (*str_).name.cast(),
        (*str_).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn utf8_strncmp(
    um: *const unicode_map, s1: *const qstr, s2: *const qstr,
) -> c_int {
    let mut cur1 = MaybeUninit::<utf8cursor>::uninit();
    let mut cur2 = MaybeUninit::<utf8cursor>::uninit();
    if utf8ncursor(cur1.as_mut_ptr(), um, UTF8_NFDI, (*s1).name.cast(),
        (*s1).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    if utf8ncursor(cur2.as_mut_ptr(), um, UTF8_NFDI, (*s2).name.cast(),
        (*s2).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    loop {
        let c1 = utf8byte(cur1.as_mut_ptr());
        let c2 = utf8byte(cur2.as_mut_ptr());
        if c1 < 0 || c2 < 0 { return neg(EINVAL); }
        if c1 != c2 { return 1; }
        if c1 == 0 { return 0; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn utf8_strncasecmp(
    um: *const unicode_map, s1: *const qstr, s2: *const qstr,
) -> c_int {
    let mut cur1 = MaybeUninit::<utf8cursor>::uninit();
    let mut cur2 = MaybeUninit::<utf8cursor>::uninit();
    if utf8ncursor(cur1.as_mut_ptr(), um, UTF8_NFDICF, (*s1).name.cast(),
        (*s1).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    if utf8ncursor(cur2.as_mut_ptr(), um, UTF8_NFDICF, (*s2).name.cast(),
        (*s2).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    loop {
        let c1 = utf8byte(cur1.as_mut_ptr());
        let c2 = utf8byte(cur2.as_mut_ptr());
        if c1 < 0 || c2 < 0 { return neg(EINVAL); }
        if c1 != c2 { return 1; }
        if c1 == 0 { return 0; }
    }
}

// As in C, cf must be a valid, NUL-terminated, already casefolded string.
// Its qstr length does not limit the original comparison.
#[no_mangle]
pub unsafe extern "C" fn utf8_strncasecmp_folded(
    um: *const unicode_map, cf: *const qstr, s1: *const qstr,
) -> c_int {
    let mut cur1 = MaybeUninit::<utf8cursor>::uninit();
    let mut i: c_int = 0;
    if utf8ncursor(cur1.as_mut_ptr(), um, UTF8_NFDICF, (*s1).name.cast(),
        (*s1).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    loop {
        let c1 = utf8byte(cur1.as_mut_ptr());
        let c2 = *(*cf).name.offset(i as isize) as c_int;
        // The kernel's C flags give signed arithmetic wrapping semantics.
        i = i.wrapping_add(1);
        if c1 < 0 { return neg(EINVAL); }
        if c1 != c2 { return 1; }
        if c1 == 0 { return 0; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn utf8_casefold(
    um: *const unicode_map, str_: *const qstr, dest: *mut u8, dlen: usize,
) -> c_int {
    let mut cur = MaybeUninit::<utf8cursor>::uninit();
    if utf8ncursor(cur.as_mut_ptr(), um, UTF8_NFDICF, (*str_).name.cast(),
        (*str_).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    for nlen in 0..dlen {
        let c = utf8byte(cur.as_mut_ptr());
        // Preserve the C write before testing either terminator or error.
        *dest.add(nlen) = c as u8;
        if c == 0 { return nlen as c_int; }
        if c == -1 { break; }
    }
    neg(EINVAL)
}

#[no_mangle]
pub unsafe extern "C" fn utf8_casefold_hash(
    um: *const unicode_map, salt: *const c_void, str_: *mut qstr,
) -> c_int {
    let mut cur = MaybeUninit::<utf8cursor>::uninit();
    let mut hash = rust_utf8_core_init_name_hash(salt);
    if utf8ncursor(cur.as_mut_ptr(), um, UTF8_NFDICF, (*str_).name.cast(),
        (*str_).__bindgen_anon_1.__bindgen_anon_1.len as usize) < 0 {
        return neg(EINVAL);
    }
    loop {
        let c = utf8byte(cur.as_mut_ptr());
        if c == 0 { break; }
        if c < 0 { return neg(EINVAL); }
        hash = rust_utf8_core_partial_name_hash(c as u8 as _, hash);
    }
    (*str_).__bindgen_anon_1.__bindgen_anon_1.hash = rust_utf8_core_end_name_hash(hash);
    0
}

unsafe fn find_table_version(
    table: *const utf8data, nr_entries: usize, version: u32,
) -> *const utf8data {
    // These are the original table invariants, established by the retained
    // data and utf8version_is_supported(), not a new malformed-table API.
    let mut i = nr_entries.wrapping_sub(1);
    while version < (*table.add(i)).maxage { i = i.wrapping_sub(1); }
    if version > (*table.add(i)).maxage { null() } else { table.add(i) }
}

#[no_mangle]
pub unsafe extern "C" fn utf8_load(version: u32) -> *mut unicode_map {
    let um = rust_utf8_core_alloc();
    if um.is_null() { return err_ptr(neg(ENOMEM)); }
    (*um).version = version;

    (*um).tables = rust_utf8_core_symbol_request();
    if (*um).tables.is_null() {
        kfree(um.cast());
        return err_ptr(neg(EINVAL));
    }
    let result = 'loaded: {
        if utf8version_is_supported(um, version) == 0 { break 'loaded false; }
        (*um).ntab[UTF8_NFDI as usize] = find_table_version(
            (*(*um).tables).utf8nfdidata,
            (*(*um).tables).utf8nfdidata_size as usize, (*um).version);
        if (*um).ntab[UTF8_NFDI as usize].is_null() { break 'loaded false; }
        (*um).ntab[UTF8_NFDICF as usize] = find_table_version(
            (*(*um).tables).utf8nfdicfdata,
            (*(*um).tables).utf8nfdicfdata_size as usize, (*um).version);
        if (*um).ntab[UTF8_NFDICF as usize].is_null() { break 'loaded false; }
        true
    };
    if result { return um; }
    rust_utf8_core_symbol_put();
    kfree(um.cast());
    err_ptr(neg(EINVAL))
}

#[no_mangle]
pub unsafe extern "C" fn utf8_unload(um: *mut unicode_map) {
    if !um.is_null() {
        rust_utf8_core_symbol_put();
        kfree(um.cast());
    }
}

#[no_mangle]
pub unsafe extern "C" fn utf8_parse_version(version: *mut c_char) -> c_int {
    let mut args = MaybeUninit::<[substring_t; 3]>::uninit();
    let args = args.as_mut_ptr().cast::<substring_t>();
    let (mut maj, mut min, mut rev) = (0, 0, 0);
    let token = [
        match_token { token: 1, pattern: c"%u.%u.%u".as_ptr().cast() },
        match_token { token: 0, pattern: null() },
    ];
    if match_token(version, token.as_ptr(), args) != 1 { return neg(EINVAL); }
    if match_uint(args, &mut maj) != 0 || match_uint(args.add(1), &mut min) != 0
        || match_uint(args.add(2), &mut rev) != 0 {
        return neg(EINVAL);
    }
    if maj > u8::MAX as u32 || min > u8::MAX as u32 || rev > u8::MAX as u32 {
        return neg(EINVAL);
    }
    ((maj << UNICODE_MAJ_SHIFT) | (min << UNICODE_MIN_SHIFT) | rev) as c_int
}

// These are the nine original non-GPL-only exports, owned by this Rust object.
ffi_export::export_symbol!(utf8_validate, utf8_validate, "", "");
ffi_export::export_symbol!(utf8_strncmp, utf8_strncmp, "", "");
ffi_export::export_symbol!(utf8_strncasecmp, utf8_strncasecmp, "", "");
ffi_export::export_symbol!(utf8_strncasecmp_folded, utf8_strncasecmp_folded, "", "");
ffi_export::export_symbol!(utf8_casefold, utf8_casefold, "", "");
ffi_export::export_symbol!(utf8_casefold_hash, utf8_casefold_hash, "", "");
ffi_export::export_symbol!(utf8_load, utf8_load, "", "");
ffi_export::export_symbol!(utf8_unload, utf8_unload, "", "");
ffi_export::export_symbol!(utf8_parse_version, utf8_parse_version, "", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
