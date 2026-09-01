// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int};

pub type u64 = u64;

#[repr(C)]
pub struct annotate_args {
    _private: [u8; 0],
}

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
    pub fn llvm__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line: *mut u32,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;

    pub fn symbol__disassemble_llvm(
        filename: *const c_char,
        sym: *mut symbol,
        args: *mut annotate_args,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
