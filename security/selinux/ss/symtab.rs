// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of the symbol table type.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* C dependencies: <linux/kernel.h>, <linux/string.h>, <linux/errno.h>, "symtab.h" */

use core::ffi::{c_char, c_int, c_uchar, c_void};

#[repr(C)]
pub struct hashtab {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symtab {
    pub table: hashtab,
    pub nprim: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_key_params {
    pub hash: Option<unsafe extern "C" fn(key: *const c_void) -> u32>,
    pub cmp: Option<unsafe extern "C" fn(key1: *const c_void, key2: *const c_void) -> c_int>,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn hashtab_init(h: *mut hashtab, nel: u32) -> c_int;
    fn hashtab_insert(
        h: *mut hashtab,
        key: *mut c_char,
        datum: *mut c_void,
        key_params: hashtab_key_params,
    ) -> c_int;
    fn hashtab_search(
        h: *const hashtab,
        key: *const c_char,
        key_params: hashtab_key_params,
    ) -> *mut c_void;
}

unsafe extern "C" fn symhash(mut key: *const c_void) -> u32 {
    /*
     * djb2a
     * Public domain from cdb v0.75
     */
    let mut hash: u32 = 5381;
    let mut c: c_uchar;

    loop {
        c = *(key as *const c_uchar);
        key = (key as *const c_uchar).add(1) as *const c_void;
        if c == 0 {
            break;
        }
        hash = ((hash << 5).wrapping_add(hash)) ^ c as u32;
    }

    hash
}

unsafe extern "C" fn symcmp(key1: *const c_void, key2: *const c_void) -> c_int {
    let keyp1: *const c_char;
    let keyp2: *const c_char;

    keyp1 = key1 as *const c_char;
    keyp2 = key2 as *const c_char;
    strcmp(keyp1, keyp2)
}

static SYMTAB_KEY_PARAMS: hashtab_key_params = hashtab_key_params {
    hash: Some(symhash),
    cmp: Some(symcmp),
};

#[no_mangle]
pub unsafe extern "C" fn symtab_init(s: *mut symtab, size: u32) -> c_int {
    (*s).nprim = 0;
    hashtab_init(&mut (*s).table, size)
}

#[no_mangle]
pub unsafe extern "C" fn symtab_insert(
    s: *mut symtab,
    name: *mut c_char,
    datum: *mut c_void,
) -> c_int {
    hashtab_insert(&mut (*s).table, name, datum, SYMTAB_KEY_PARAMS)
}

#[no_mangle]
pub unsafe extern "C" fn symtab_search(s: *const symtab, name: *const c_char) -> *mut c_void {
    hashtab_search(&(*s).table, name, SYMTAB_KEY_PARAMS)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
