/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency declarations corresponding to <list_types.h> are supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct search_data {
    pub head: *mut list_head,
    pub target: *mut menu,
}

extern "C" {
    pub static mut jump_key_char: i32;

    pub fn next_jump_key(key: i32) -> i32;
    pub fn handle_search_keys(key: i32, start: usize, end: usize, data: *mut c_void) -> i32;
    pub fn get_jump_key_char() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
