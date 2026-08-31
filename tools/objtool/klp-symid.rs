// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Emit the .klp.symid table which allows "objtool klp diff" to reliably
 * disambiguate duplicate-named local symbols in vmlinux.
 *
 * Livepatch identifies a duplicate-named symbol by its position (sympos)
 * among the same-named kallsyms entries, counted in ascending address order
 * in the final linked vmlinux.  That order can't be derived from vmlinux.o
 * alone: the final link reorders sub-sections (.text.unlikely*, .data..*,
 * etc).
 *
 * Bridge the gap with a table which survives the final link: a single
 * non-alloc section containing an array of { id, addr } entries, where
 * 'id' is a unique counter identifier and 'addr' has a relocation to the
 * symbol.  The linker copies 'id' verbatim and resolves 'addr' to the symbol's
 * final address.
 *
 * The table is only emitted for vmlinux.o, and only when klp-build asks for it
 * with KLP_SYMIDS=1, which adds --klp-symids to the vmlinux.o objtool run.
 *
 * It can't survive --gc-sections, which sweeps the whole section; klp-build
 * rejects CONFIG_LD_DEAD_CODE_DATA_ELIMINATION.
 */

// Dependencies in the original C source:
// linux/string.h
// objtool/objtool.h
// objtool/warn.h
// objtool/endianness.h
// objtool/klp.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;

const SHF_ALLOC: u64 = 0x2;
const SHT_PROGBITS: c_uint = 1;
const R_ABS64: c_uint = 1;

type c_uint = u32;

extern "C" {
    static objname: *const c_char;
    static KLP_SYMID_SEC: *const c_char;

    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn str_ends_with(str_: *const c_char, suffix: *const c_char) -> bool;

    fn is_local_sym(sym: *mut symbol) -> bool;
    fn is_undef_sym(sym: *mut symbol) -> bool;
    fn is_func_sym(sym: *mut symbol) -> bool;
    fn is_object_sym(sym: *mut symbol) -> bool;
    fn is_prefix_func(sym: *mut symbol) -> bool;
    fn is_sec_sym(sym: *mut symbol) -> bool;
    fn is_file_sym(sym: *mut symbol) -> bool;

    fn bswap_if_needed(elf: *mut elf, val: u64) -> u64;
    fn elf_create_section(
        elf: *mut elf,
        name: *const c_char,
        idx: c_uint,
        entsize: usize,
        typ: c_uint,
        align: c_uint,
        flags: c_uint,
    ) -> *mut section;
    fn elf_add_data(
        elf: *mut elf,
        sec: *mut section,
        data: *const c_void,
        size: usize,
    ) -> *mut klp_symid;
    fn elf_create_reloc(
        elf: *mut elf,
        sec: *mut section,
        offset: u64,
        sym: *mut symbol,
        addend: i64,
        typ: c_uint,
    ) -> *mut c_void;

    fn elf_first_symbol(elf: *mut elf) -> *mut symbol;
    fn elf_next_symbol(elf: *mut elf, sym: *mut symbol) -> *mut symbol;
    fn elf_first_symbol_by_name(elf: *mut elf, name: *const c_char) -> *mut symbol;
    fn elf_next_symbol_by_name(elf: *mut elf, sym: *mut symbol) -> *mut symbol;
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct shdr {
    pub sh_flags: u64,
}

#[repr(C)]
pub struct section {
    pub sh: shdr,
    pub name: *const c_char,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub sec: *mut section,
}

#[repr(C)]
pub struct klp_symid {
    pub id: u64,
    pub addr: u64,
}

static discarded_secs: [*const c_char; 5] = [
    b".discard\0".as_ptr() as *const c_char,
    b".exitcall.exit\0".as_ptr() as *const c_char,
    b".modinfo\0".as_ptr() as *const c_char,
    b".no_trim_symbol\0".as_ptr() as *const c_char,
    b"__tracepoint_check\0".as_ptr() as *const c_char,
];

unsafe fn discarded_sec(sec: *mut section) -> bool {
    if ((*sec).sh.sh_flags & SHF_ALLOC) == 0 {
        return true;
    }

    for i in 0..discarded_secs.len() {
        if strstarts((*sec).name, discarded_secs[i]) {
            return true;
        }
    }

    false
}

unsafe fn symid_needed(elf: *mut elf, sym: *mut symbol) -> bool {
    let mut s: *mut symbol;

    if !is_local_sym(sym) || is_undef_sym(sym) {
        return false;
    }

    if !is_func_sym(sym) && !is_object_sym(sym) {
        return false;
    }

    if is_prefix_func(sym) {
        return false;
    }

    if discarded_sec((*sym).sec) {
        return false;
    }

    s = elf_first_symbol_by_name(elf, (*sym).name);
    while !s.is_null() {
        if s == sym || is_sec_sym(s) || is_file_sym(s) || is_undef_sym(s) {
            s = elf_next_symbol_by_name(elf, s);
            continue;
        }
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn klp_create_symid_sections(file: *mut objtool_file) -> c_int {
    let elf: *mut elf = (*file).elf;
    let mut symids: *mut klp_symid;
    let mut sec: *mut section;
    let mut sym: *mut symbol;
    let mut nr: u64 = 0;
    let mut i: u64 = 0;

    if !str_ends_with(objname, b"vmlinux.o\0".as_ptr() as *const c_char) {
        return 0;
    }

    sym = elf_first_symbol(elf);
    while !sym.is_null() {
        if symid_needed(elf, sym) {
            nr += 1;
        }
        sym = elf_next_symbol(elf, sym);
    }

    if nr == 0 {
        return 0;
    }

    sec = elf_create_section(
        elf,
        KLP_SYMID_SEC,
        0,
        size_of::<klp_symid>(),
        SHT_PROGBITS,
        8,
        0,
    );
    if sec.is_null() {
        return -1;
    }

    symids = elf_add_data(
        elf,
        sec,
        ptr::null(),
        (nr as usize) * size_of::<klp_symid>(),
    );
    if symids.is_null() {
        return -1;
    }

    sym = elf_first_symbol(elf);
    while !sym.is_null() {
        if !symid_needed(elf, sym) {
            sym = elf_next_symbol(elf, sym);
            continue;
        }

        (*symids.add(i as usize)).id = bswap_if_needed(elf, i);

        if elf_create_reloc(
            elf,
            sec,
            i * size_of::<klp_symid>() as u64
                + core::mem::offset_of!(klp_symid, addr) as u64,
            sym,
            0,
            R_ABS64,
        )
        .is_null()
        {
            return -1;
        }

        i += 1;
        sym = elf_next_symbol(elf, sym);
    }

    0
}
