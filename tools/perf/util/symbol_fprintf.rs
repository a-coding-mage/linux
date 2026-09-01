// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/symbol_fprintf.c.
// Dependencies from elf.h, stdio.h, dso.h, map.h, and symbol.h are declared
// here as external C interfaces and opaque C-compatible types.

use core::ffi::{c_char, c_int, c_ulong};

pub type size_t = usize;

pub const STB_LOCAL: c_int = 0;
pub const STB_GLOBAL: c_int = 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    pub addr: u64,
    pub map: *mut map,
}

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn symbol__binding(sym: *const symbol) -> c_int;
    fn map__start(map: *mut map) -> u64;
    fn dso__symbol_names_len(dso: *mut dso) -> size_t;
    fn dso__symbol_names(dso: *mut dso) -> *mut *mut symbol;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> size_t {
    unsafe {
        fprintf(
            fp,
            c" %lx-%lx %c %s\n".as_ptr(),
            (*sym).start,
            (*sym).end,
            if symbol__binding(sym) == STB_GLOBAL {
                'g' as c_int
            } else if symbol__binding(sym) == STB_LOCAL {
                'l' as c_int
            } else {
                'w' as c_int
            },
            (*sym).name,
        ) as size_t
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __symbol__fprintf_symname_offs(
    sym: *const symbol,
    al: *const addr_location,
    unknown_as_addr: bool,
    print_offsets: bool,
    fp: *mut FILE,
) -> size_t {
    let offset: c_ulong;
    let mut length: size_t;

    unsafe {
        if !sym.is_null() {
            length = fprintf(fp, c"%s".as_ptr(), (*sym).name) as size_t;
            if !al.is_null() && print_offsets {
                if (*al).addr < (*sym).end {
                    offset = ((*al).addr).wrapping_sub((*sym).start) as c_ulong;
                } else {
                    offset = ((*al).addr)
                        .wrapping_sub(map__start((*al).map))
                        .wrapping_sub((*sym).start) as c_ulong;
                }
                length = length.wrapping_add(fprintf(fp, c"+0x%lx".as_ptr(), offset) as size_t);
            }
            return length;
        } else if !al.is_null() && unknown_as_addr {
            return fprintf(fp, c"[%#lx]".as_ptr(), (*al).addr) as size_t;
        } else {
            return fprintf(fp, c"[unknown]".as_ptr()) as size_t;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__fprintf_symname_offs(
    sym: *const symbol,
    al: *const addr_location,
    fp: *mut FILE,
) -> size_t {
    unsafe { __symbol__fprintf_symname_offs(sym, al, false, true, fp) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __symbol__fprintf_symname(
    sym: *const symbol,
    al: *const addr_location,
    unknown_as_addr: bool,
    fp: *mut FILE,
) -> size_t {
    unsafe { __symbol__fprintf_symname_offs(sym, al, unknown_as_addr, false, fp) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__fprintf_symname(sym: *const symbol, fp: *mut FILE) -> size_t {
    unsafe { __symbol__fprintf_symname_offs(sym, core::ptr::null(), false, false, fp) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__fprintf_symbols_by_name(dso: *mut dso, fp: *mut FILE) -> size_t {
    let mut ret: size_t = 0;

    unsafe {
        let mut i: size_t = 0;
        while i < dso__symbol_names_len(dso) {
            let pos: *mut symbol = *dso__symbol_names(dso).add(i);

            ret = ret.wrapping_add(fprintf(fp, c"%s\n".as_ptr(), (*pos).name) as size_t);
            i += 1;
        }
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
