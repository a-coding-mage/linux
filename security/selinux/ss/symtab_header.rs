/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A symbol table (symtab) maintains associations between symbol
 * strings and datum values.  The type of the datum values
 * is arbitrary.  The symbol table type is implemented
 * using the hash table type (hashtab).
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* depends on hashtab.h */

#[repr(C)]
pub struct symtab {
    pub table: hashtab, /* hash table (keyed on a string) */
    pub nprim: u32,    /* number of primary names in table */
}

unsafe extern "C" {
    pub fn symtab_init(s: *mut symtab, size: u32) -> core::ffi::c_int;

    pub fn symtab_insert(
        s: *mut symtab,
        name: *mut core::ffi::c_char,
        datum: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn symtab_search(
        s: *const symtab,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
