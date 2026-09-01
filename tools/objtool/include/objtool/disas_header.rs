/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025, Oracle and/or its affiliates.
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct alternative {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disas_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disassemble_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objtool_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct instruction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* DISAS: declarations provided when built with libopcodes disassembly support. */
#[cfg(DISAS)]
unsafe extern "C" {
    pub fn disas_context_create(file: *mut objtool_file) -> *mut disas_context;
    pub fn disas_context_destroy(dctx: *mut disas_context);
    pub fn disas_warned_funcs(dctx: *mut disas_context);
    pub fn disas_funcs(dctx: *mut disas_context);
    pub fn disas_info_init(
        dinfo: *mut disassemble_info,
        arch: c_int,
        mach32: c_int,
        mach64: c_int,
        options: *const c_char,
    ) -> c_int;
    pub fn disas_insn(dctx: *mut disas_context, insn: *mut instruction) -> usize;
    pub fn disas_result(dctx: *mut disas_context) -> *mut c_char;
    pub fn disas_print_info(
        stream: *mut FILE,
        insn: *mut instruction,
        depth: c_int,
        format: *const c_char,
        ...
    );
    pub fn disas_print_insn(
        stream: *mut FILE,
        dctx: *mut disas_context,
        insn: *mut instruction,
        depth: c_int,
        format: *const c_char,
        ...
    );
    pub fn disas_alt_name(alt: *mut alternative) -> *mut c_char;
    pub fn disas_alt_type_name(insn: *mut instruction) -> *const c_char;
}

#[cfg(not(DISAS))]
unsafe extern "C" {
    pub fn WARN(format: *const c_char, ...);
}

#[cfg(not(DISAS))]
pub unsafe fn disas_context_create(_file: *mut objtool_file) -> *mut disas_context {
    unsafe {
        WARN(c"Rebuild with libopcodes for disassembly support".as_ptr());
    }
    core::ptr::null_mut()
}

#[cfg(not(DISAS))]
pub unsafe fn disas_context_destroy(_dctx: *mut disas_context) {}

#[cfg(not(DISAS))]
pub unsafe fn disas_warned_funcs(_dctx: *mut disas_context) {}

#[cfg(not(DISAS))]
pub unsafe fn disas_funcs(_dctx: *mut disas_context) {}

#[cfg(not(DISAS))]
pub unsafe fn disas_info_init(
    _dinfo: *mut disassemble_info,
    _arch: c_int,
    _mach32: c_int,
    _mach64: c_int,
    _options: *const c_char,
) -> c_int {
    -1
}

#[cfg(not(DISAS))]
pub unsafe fn disas_insn(_dctx: *mut disas_context, _insn: *mut instruction) -> usize {
    usize::MAX
}

#[cfg(not(DISAS))]
pub unsafe fn disas_result(_dctx: *mut disas_context) -> *mut c_char {
    core::ptr::null_mut()
}

#[cfg(not(DISAS))]
pub unsafe extern "C" fn disas_print_info(
    _stream: *mut FILE,
    _insn: *mut instruction,
    _depth: c_int,
    _format: *const c_char,
    ...
) {
}

#[cfg(not(DISAS))]
pub unsafe extern "C" fn disas_print_insn(
    _stream: *mut FILE,
    _dctx: *mut disas_context,
    _insn: *mut instruction,
    _depth: c_int,
    _format: *const c_char,
    ...
) {
}

#[cfg(not(DISAS))]
pub unsafe fn disas_alt_name(_alt: *mut alternative) -> *mut c_char {
    core::ptr::null_mut()
}

#[cfg(not(DISAS))]
pub unsafe fn disas_alt_type_name(_insn: *mut instruction) -> *const c_char {
    core::ptr::null()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
