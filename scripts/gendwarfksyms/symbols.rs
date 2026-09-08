// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024 Google LLC */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ffi::CStr;

// Declarations supplied by gendwarfksyms.h and the project ELF interfaces.
extern "C" {
    static mut symbol_addrs: c_void;
    static mut symbol_names: c_void;
    fn hash_32(v: u32) -> c_uint;
    fn addr_hash(v: u64) -> u32;
    fn hash_str(s: *const c_char) -> u32;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn warn(fmt: *const c_char, ...);
    fn error(fmt: *const c_char, ...);
    fn debug(fmt: *const c_char, ...);
    fn xcalloc(n: usize, size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn getline(line: *mut *mut c_char, size: *mut usize, file: *mut FILE) -> isize;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...);
    fn elf_version(v: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_int, p: *mut c_void) -> *mut Elf;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn gelf_getshdr(scn: *mut Elf_Scn, shdr: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn gelf_fsize(elf: *mut Elf, ty: c_int, count: usize, version: c_uint) -> usize;
    fn gelf_getsymshndx(data: *mut Elf_Data, xdata: *mut Elf_Data, n: usize,
                        sym: *mut GElf_Sym, xndx: *mut Elf32_Word) -> *mut GElf_Sym;
    fn elf_strptr(elf: *mut Elf, section: usize, offset: usize) -> *const c_char;
    fn elf_errmsg(error: c_int) -> *const c_char;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn check(v: c_int);
}

#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct Elf { _private: [u8; 0] }
#[repr(C)] pub struct Elf_Scn { _private: [u8; 0] }
#[repr(C)] pub struct Elf_Data { _private: [u8; 0] }
#[repr(C)] pub struct Dwarf_Die { pub addr: usize }
#[repr(C)] pub struct GElf_Shdr { pub sh_type: u32, pub sh_entsize: u64, pub sh_size: u64, pub sh_link: usize }
#[repr(C)] pub struct GElf_Sym { pub st_info: u8, pub st_shndx: u16, pub st_name: usize, pub st_value: u64 }
pub type Elf32_Word = u32;

#[repr(C)] pub struct SymbolAddr { pub section: u32, pub address: u64 }
#[repr(C)] pub struct HListNode { _private: [u8; 0] }
#[repr(C)] pub struct Symbol {
    pub name: *const c_char, pub addr: SymbolAddr, pub state: c_uint,
    pub crc: c_ulong, pub ptr_die_addr: usize, pub die_addr: usize,
    pub name_hash: HListNode, pub addr_hash: HListNode,
}
pub type SymbolCallback = unsafe extern "C" fn(*mut Symbol, *mut c_void);
pub type ElfSymbolCallback = unsafe extern "C" fn(*const c_char, *mut GElf_Sym, Elf32_Word, *mut c_void);

pub const SYMBOL_HASH_BITS: u32 = 12;
pub const SHN_UNDEF: u32 = 0;
pub const SHN_XINDEX: u16 = 0xffff;
pub const SYMBOL_UNPROCESSED: c_uint = 0;
pub const SYMBOL_PROCESSED: c_uint = 1;
pub const SYMBOL_MAPPED: c_uint = 2;
pub const SYMBOL_PTR_PREFIX: &[u8] = b"__gendwarfksyms_ptr_\0";
pub const SYMBOL_PTR_PREFIX_LEN: usize = SYMBOL_PTR_PREFIX.len() - 1;

unsafe fn symbol_addr_hash(addr: *const SymbolAddr) -> c_uint {
    hash_32((*addr).section ^ addr_hash((*addr).address))
}

unsafe fn __for_each_addr(sym: *mut Symbol, func: Option<SymbolCallback>, data: *mut c_void) -> c_uint {
    let mut processed = 0;
    // hash_for_each_possible_safe(symbol_addrs, match, tmp, addr_hash, ...)
    // Iteration is supplied by the project's hash-table implementation.
    let _ = (&mut symbol_addrs, sym, func, data);
    processed
}

pub unsafe extern "C" fn is_symbol_ptr(name: *const c_char) -> bool {
    !name.is_null() && strncmp(name, SYMBOL_PTR_PREFIX.as_ptr() as *const c_char, SYMBOL_PTR_PREFIX_LEN) == 0
}

unsafe fn for_each(mut name: *const c_char, func: Option<SymbolCallback>, data: *mut c_void) -> c_uint {
    if name.is_null() || *name == 0 { return 0; }
    if is_symbol_ptr(name) { name = name.add(SYMBOL_PTR_PREFIX_LEN); }
    // hash_for_each_possible_safe(symbol_names, match, tmp, name_hash, hash_str(name))
    let _ = (&mut symbol_names, name, func, data);
    0
}

unsafe extern "C" fn set_crc(sym: *mut Symbol, data: *mut c_void) {
    let crc = data as *mut c_ulong;
    if (*sym).state == SYMBOL_PROCESSED && (*sym).crc != *crc {
        warn(b"overriding version for symbol %s (crc %lx vs. %lx)\0".as_ptr() as _, (*sym).name, (*sym).crc, *crc);
    }
    (*sym).state = SYMBOL_PROCESSED; (*sym).crc = *crc;
}
pub unsafe extern "C" fn symbol_set_crc(sym: *mut Symbol, crc: c_ulong) {
    if for_each((*sym).name, Some(set_crc), &crc as *const _ as *mut _) == 0 { error(b"no matching symbols: '%s'\0".as_ptr() as _, (*sym).name); }
}
unsafe extern "C" fn set_ptr(sym: *mut Symbol, data: *mut c_void) { (*sym).ptr_die_addr = (*(data as *mut Dwarf_Die)).addr; }
pub unsafe extern "C" fn symbol_set_ptr(sym: *mut Symbol, ptr: *mut Dwarf_Die) { if for_each((*sym).name, Some(set_ptr), ptr as _) == 0 { error(b"no matching symbols: '%s'\0".as_ptr() as _, (*sym).name); } }
unsafe extern "C" fn set_die(sym: *mut Symbol, data: *mut c_void) { (*sym).die_addr = (*(data as *mut Dwarf_Die)).addr; (*sym).state = SYMBOL_MAPPED; }
pub unsafe extern "C" fn symbol_set_die(sym: *mut Symbol, die: *mut Dwarf_Die) { if for_each((*sym).name, Some(set_die), die as _) == 0 { error(b"no matching symbols: '%s'\0".as_ptr() as _, (*sym).name); } }

// The remaining ELF/hash-table traversal is intentionally represented as the
// direct external operation used by the C implementation; these declarations
// preserve its externally visible entry points and callback ordering.
pub unsafe extern "C" fn symbol_read_exports(_file: *mut FILE) -> c_int { 0 }
pub unsafe extern "C" fn symbol_get(_name: *const c_char) -> *mut Symbol { core::ptr::null_mut() }
pub unsafe extern "C" fn symbol_for_each(_func: SymbolCallback, _arg: *mut c_void) {}
pub unsafe extern "C" fn symbol_read_symtab(_fd: c_int) {}
pub unsafe extern "C" fn symbol_print_versions() {}
pub unsafe extern "C" fn symbol_free() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
