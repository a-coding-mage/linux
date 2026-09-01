// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/addr2line.h.
// C include dependency preserved for context: <linux/types.h>

use std::os::raw::{c_char, c_int, c_uint};

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inline_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn cmd__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
