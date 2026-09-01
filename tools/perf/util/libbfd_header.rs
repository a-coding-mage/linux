/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct annotate_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
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

pub type u64 = u64;
pub type size_t = usize;

// C conditional: #ifdef HAVE_LIBBFD_SUPPORT
#[cfg(HAVE_LIBBFD_SUPPORT)]
unsafe extern "C" {
    pub fn libbfd__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;

    pub fn dso__free_a2l_libbfd(dso: *mut dso);

    pub fn symbol__disassemble_libbfd(
        filename: *const c_char,
        sym: *mut symbol,
        args: *mut annotate_args,
    ) -> c_int;

    pub fn libbfd__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;

    pub fn libbfd_filename__read_debuglink(
        filename: *const c_char,
        debuglink: *mut c_char,
        size: size_t,
    ) -> c_int;

    pub fn symbol__disassemble_bpf_libbfd(
        sym: *mut symbol,
        args: *mut annotate_args,
    ) -> c_int;
}

// C conditional: #else // !defined(HAVE_LIBBFD_SUPPORT)
// Dependency intent from the disabled branch: #include "annotate.h"
#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn libbfd__addr2line(
    dso_name: *const c_char,
    addr: u64,
    file: *mut *mut c_char,
    line: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    let _ = dso_name;
    let _ = addr;
    let _ = file;
    let _ = line;
    let _ = dso;
    let _ = unwind_inlines;
    let _ = node;
    let _ = sym;

    -1
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn dso__free_a2l_libbfd(dso: *mut dso) {
    let _ = dso;
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn symbol__disassemble_libbfd(
    filename: *const c_char,
    sym: *mut symbol,
    args: *mut annotate_args,
) -> c_int {
    let _ = filename;
    let _ = sym;
    let _ = args;

    -1
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn libbfd__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    let _ = filename;
    let _ = bid;

    -1
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn libbfd_filename__read_debuglink(
    filename: *const c_char,
    debuglink: *mut c_char,
    size: size_t,
) -> c_int {
    let _ = filename;
    let _ = debuglink;
    let _ = size;

    -1
}

#[cfg(not(HAVE_LIBBFD_SUPPORT))]
#[inline]
pub unsafe fn symbol__disassemble_bpf_libbfd(
    sym: *mut symbol,
    args: *mut annotate_args,
) -> c_int {
    let _ = sym;
    let _ = args;

    SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
