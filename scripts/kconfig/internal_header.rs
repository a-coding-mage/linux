/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by hashtable.h in the C source.

pub const SYMBOL_HASHSIZE: u32 = 1u32 << 14;

// C: extern HASHTABLE_DECLARE(sym_hashtable, SYMBOL_HASHSIZE);
// The hashtable declaration is supplied by the external hashtable dependency.

// C: #define for_all_symbols(sym) hash_for_each(sym_hashtable, sym, node)
// The hash_for_each operation and sym_hashtable are supplied by the external
// hashtable dependency.

pub const EXPR_HASHSIZE: u32 = 1u32 << 14;

// C: extern HASHTABLE_DECLARE(expr_hashtable, EXPR_HASHSIZE);
// The hashtable declaration is supplied by the external hashtable dependency.

unsafe extern "C" {
    pub fn expr_invalidate_all();
}

#[repr(C)]
pub struct menu {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut current_menu: *mut menu;
    pub static mut current_entry: *mut menu;

    pub static cur_filename: *const ::core::ffi::c_char;
    pub static mut cur_lineno: ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
