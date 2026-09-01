/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/* Translated from objtool/include/objtool/elf.h. */
/* C includes removed; referenced types/functions/constants are future dependencies. */

pub const SEC_NAME_LEN: usize = 1024;
pub const SYM_NAME_LEN: usize = 512;

#[inline]
pub unsafe fn str_hash(str_: *const ::core::ffi::c_char) -> u32 {
    unsafe { jhash(str_ as *const ::core::ffi::c_void, strlen(str_), 0) }
}

unsafe extern "C" {
    pub fn str_hash_demangled(str_: *const ::core::ffi::c_char) -> u32;
}

/* bswap_if_needed(elf, val) maps to __bswap_if_needed(&elf->ehdr, val) in C. */

/* LIBELF_USE_DEPRECATED aliases elf_getshdrnum/elf_getshdrstrndx to deprecated libelf names in C. */

/* Fallback in C: define ELF_C_READ_MMAP as ELF_C_READ when unavailable. */

#[repr(C)]
pub struct elf_hash_node {
    pub next: *mut elf_hash_node,
}

#[repr(C)]
pub struct section {
    pub list: list_head,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub sh: GElf_Shdr,
    pub symbol_tree: rb_root_cached,
    pub symbol_list: list_head,
    pub base: *mut section,
    pub rsec: *mut section,
    pub sym: *mut symbol,
    pub data: *mut Elf_Data,
    pub name: *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_int,
    pub _changed: bool,
    pub text: bool,
    pub rodata: bool,
    pub noinstr: bool,
    pub init: bool,
    pub truncate: bool,
    pub relocs: *mut reloc,
    pub nr_alloc_relocs: ::core::ffi::c_ulong,
    pub twin: *mut section,
}

#[repr(C)]
pub struct symbol {
    pub list: list_head,
    pub global_list: list_head,
    pub node: rb_node,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub sym: GElf_Sym,
    pub sec: *mut section,
    pub name: *const ::core::ffi::c_char,
    pub demangled_name: *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_uint,
    pub len: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_ulong,
    pub __subtree_last: ::core::ffi::c_ulong,
    pub pfunc: *mut symbol,
    pub cfunc: *mut symbol,
    pub alias: *mut symbol,
    pub file: *mut symbol,
    pub bind: ::core::ffi::c_uchar,
    pub type_: ::core::ffi::c_uchar,
    /*
     * C stores the following as u8 bitfields. Rust has no C-compatible bitfield
     * syntax without an external dependency, so preserve the declared fields as
     * bytes in source order for file-local translation.
     */
    pub uaccess_safe: u8,
    pub static_call_tramp: u8,
    pub retpoline_thunk: u8,
    pub return_thunk: u8,
    pub fentry: u8,
    pub profiling_func: u8,
    pub warned: u8,
    pub embedded_insn: u8,
    pub local_label: u8,
    pub frame_pointer: u8,
    pub ignore: u8,
    pub nocfi: u8,
    pub cold: u8,
    pub prefix: u8,
    pub debug_checksum: u8,
    pub changed: u8,
    pub included: u8,
    pub klp: u8,
    pub dont_correlate: u8,
    pub fake: u8,
    pub pv_target: list_head,
    pub relocs: *mut reloc,
    pub group_sec: *mut section,
    pub csum: checksum,
    pub twin: *mut symbol,
    pub clone: *mut symbol,
}

#[repr(C)]
pub struct reloc {
    pub hash: elf_hash_node,
    pub sec: *mut section,
    pub sym: *mut symbol,
    pub _sym_next_reloc: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct elf {
    pub elf: *mut Elf,
    pub ehdr: GElf_Ehdr,
    pub fd: ::core::ffi::c_int,
    pub changed: bool,
    pub name: *const ::core::ffi::c_char,
    pub tmp_name: *const ::core::ffi::c_char,
    pub num_files: ::core::ffi::c_uint,
    pub sections: list_head,
    pub symbols: list_head,
    pub num_relocs: ::core::ffi::c_ulong,
    pub symbol_bits: ::core::ffi::c_int,
    pub symbol_name_bits: ::core::ffi::c_int,
    pub section_bits: ::core::ffi::c_int,
    pub section_name_bits: ::core::ffi::c_int,
    pub reloc_bits: ::core::ffi::c_int,
    pub symbol_hash: *mut *mut elf_hash_node,
    pub symbol_name_hash: *mut *mut elf_hash_node,
    pub section_hash: *mut *mut elf_hash_node,
    pub section_name_hash: *mut *mut elf_hash_node,
    pub reloc_hash: *mut *mut elf_hash_node,
    pub section_data: *mut section,
    pub symbol_data: *mut symbol,
}

/* __elf_table, __elf_bits, __elf_table_entry, elf_list_entry, and
 * elf_hash_for_each_possible are C macro helpers for typed hash-table iteration.
 */

unsafe extern "C" {
    pub fn elf_open_read(
        name: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut elf;
    pub fn elf_create_file(ehdr: *mut GElf_Ehdr, name: *const ::core::ffi::c_char) -> *mut elf;
    pub fn elf_create_section(
        elf: *mut elf,
        name: *const ::core::ffi::c_char,
        size: size_t,
        entsize: size_t,
        type_: ::core::ffi::c_uint,
        align: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> *mut section;
    pub fn elf_create_section_pair(
        elf: *mut elf,
        name: *const ::core::ffi::c_char,
        entsize: size_t,
        nr: ::core::ffi::c_uint,
        reloc_nr: ::core::ffi::c_uint,
    ) -> *mut section;
    pub fn elf_create_rela_section(
        elf: *mut elf,
        sec: *mut section,
        reloc_nr: ::core::ffi::c_uint,
    ) -> *mut section;
    pub fn elf_create_symbol(
        elf: *mut elf,
        name: *const ::core::ffi::c_char,
        sec: *mut section,
        bind: ::core::ffi::c_uint,
        type_: ::core::ffi::c_uint,
        offset: ::core::ffi::c_ulong,
        size: size_t,
    ) -> *mut symbol;
    pub fn elf_create_section_symbol(elf: *mut elf, sec: *mut section) -> *mut symbol;
    pub fn elf_add_data(
        elf: *mut elf,
        sec: *mut section,
        data: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn elf_add_string(
        elf: *mut elf,
        strtab: *mut section,
        str_: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    pub fn elf_create_reloc(
        elf: *mut elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
        sym: *mut symbol,
        addend: s64,
        type_: ::core::ffi::c_uint,
    ) -> *mut reloc;
    pub fn elf_init_reloc(
        elf: *mut elf,
        rsec: *mut section,
        reloc_idx: ::core::ffi::c_uint,
        offset: ::core::ffi::c_ulong,
        sym: *mut symbol,
        addend: s64,
        type_: ::core::ffi::c_uint,
    ) -> *mut reloc;
    pub fn elf_init_reloc_text_sym(
        elf: *mut elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
        reloc_idx: ::core::ffi::c_uint,
        insn_sec: *mut section,
        insn_off: ::core::ffi::c_ulong,
    ) -> *mut reloc;
    pub fn elf_init_reloc_data_sym(
        elf: *mut elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
        reloc_idx: ::core::ffi::c_uint,
        sym: *mut symbol,
        addend: s64,
    ) -> *mut reloc;
    pub fn elf_write_symbol(elf: *mut elf, sym: *mut symbol) -> ::core::ffi::c_int;
    pub fn elf_write_insn(
        elf: *mut elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
        len: ::core::ffi::c_uint,
        insn: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn elf_write(elf: *mut elf) -> ::core::ffi::c_int;
    pub fn elf_close(elf: *mut elf) -> ::core::ffi::c_int;
    pub fn find_section_by_name(
        elf: *const elf,
        name: *const ::core::ffi::c_char,
    ) -> *mut section;
    pub fn find_func_by_offset(sec: *mut section, offset: ::core::ffi::c_ulong) -> *mut symbol;
    pub fn find_symbol_by_offset(sec: *mut section, offset: ::core::ffi::c_ulong) -> *mut symbol;
    pub fn find_symbol_by_name(
        elf: *const elf,
        name: *const ::core::ffi::c_char,
    ) -> *mut symbol;
    pub fn find_global_symbol_by_name(
        elf: *const elf,
        name: *const ::core::ffi::c_char,
    ) -> *mut symbol;
    pub fn find_symbol_containing(sec: *const section, offset: ::core::ffi::c_ulong) -> *mut symbol;
    pub fn find_symbol_containing_inclusive(
        sec: *const section,
        offset: ::core::ffi::c_ulong,
    ) -> *mut symbol;
    pub fn find_symbol_hole_containing(
        sec: *const section,
        offset: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn find_reloc_by_dest(
        elf: *const elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
    ) -> *mut reloc;
    pub fn find_reloc_by_dest_range(
        elf: *const elf,
        sec: *mut section,
        offset: ::core::ffi::c_ulong,
        len: ::core::ffi::c_uint,
    ) -> *mut reloc;
    pub fn find_func_containing(sec: *mut section, offset: ::core::ffi::c_ulong) -> *mut symbol;
}

/*
 * Try to see if it's a whole archive (vmlinux.o or module).
 *
 * Note this will miss the case where a module only has one source file.
 */
#[inline]
pub unsafe fn has_multiple_files(elf: *mut elf) -> bool {
    unsafe { (*elf).num_files > 1 }
}

#[inline]
pub unsafe fn elf_addr_size(elf: *mut elf) -> size_t {
    unsafe {
        if (*elf).ehdr.e_ident[EI_CLASS as usize] == ELFCLASS32 as u8 {
            4
        } else {
            8
        }
    }
}

#[inline]
pub unsafe fn elf_rela_size(elf: *mut elf) -> size_t {
    unsafe {
        if elf_addr_size(elf) == 4 {
            ::core::mem::size_of::<Elf32_Rela>() as size_t
        } else {
            ::core::mem::size_of::<Elf64_Rela>() as size_t
        }
    }
}

#[inline]
pub unsafe fn elf_data_rela_type(elf: *mut elf) -> ::core::ffi::c_uint {
    unsafe {
        if elf_addr_size(elf) == 4 {
            R_DATA32
        } else {
            R_DATA64
        }
    }
}

#[inline]
pub unsafe fn elf_text_rela_type(elf: *mut elf) -> ::core::ffi::c_uint {
    unsafe {
        if elf_addr_size(elf) == 4 {
            R_TEXT32
        } else {
            R_TEXT64
        }
    }
}

#[inline]
pub unsafe fn is_undef_sym(sym: *mut symbol) -> bool {
    unsafe { (*(*sym).sec).idx == 0 }
}

#[inline]
pub unsafe fn is_null_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).idx == 0 }
}

#[inline]
pub unsafe fn is_sec_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).type_ == STT_SECTION as u8 }
}

#[inline]
pub unsafe fn is_object_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).type_ == STT_OBJECT as u8 }
}

#[inline]
pub unsafe fn is_func_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).type_ == STT_FUNC as u8 }
}

#[inline]
pub unsafe fn is_file_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).type_ == STT_FILE as u8 }
}

#[inline]
pub unsafe fn is_notype_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).type_ == STT_NOTYPE as u8 }
}

#[inline]
pub unsafe fn is_global_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).bind == STB_GLOBAL as u8 }
}

#[inline]
pub unsafe fn is_weak_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).bind == STB_WEAK as u8 }
}

#[inline]
pub unsafe fn is_local_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).bind == STB_LOCAL as u8 }
}

#[inline]
pub unsafe fn is_alias_sym(sym: *mut symbol) -> bool {
    unsafe { (*sym).alias != sym }
}

#[inline]
pub unsafe fn is_prefix_func(sym: *mut symbol) -> bool {
    unsafe { (*sym).prefix != 0 }
}

#[inline]
pub unsafe fn is_cold_func(sym: *mut symbol) -> bool {
    unsafe { (*sym).cold != 0 }
}

#[inline]
pub unsafe fn is_reloc_sec(sec: *mut section) -> bool {
    unsafe { (*sec).sh.sh_type == SHT_RELA || (*sec).sh.sh_type == SHT_REL }
}

#[inline]
pub unsafe fn is_string_sec(sec: *mut section) -> bool {
    unsafe { ((*sec).sh.sh_flags & SHF_STRINGS as u64) != 0 }
}

#[inline]
pub unsafe fn is_text_sec(sec: *mut section) -> bool {
    unsafe { ((*sec).sh.sh_flags & SHF_EXECINSTR as u64) != 0 }
}

#[inline]
pub unsafe fn is_rodata_sec(sec: *mut section) -> bool {
    unsafe { (*sec).rodata }
}

#[inline]
pub unsafe fn sec_changed(sec: *mut section) -> bool {
    unsafe { (*sec)._changed }
}

#[inline]
pub unsafe fn mark_sec_changed(elf: *mut elf, sec: *mut section, changed: bool) {
    unsafe {
        (*sec)._changed = changed;
        (*elf).changed |= changed;
    }
}

#[inline]
pub unsafe fn sec_num_entries(sec: *mut section) -> ::core::ffi::c_uint {
    unsafe { ((*sec).sh.sh_size / (*sec).sh.sh_entsize) as ::core::ffi::c_uint }
}

#[inline]
pub unsafe fn reloc_idx(reloc: *mut reloc) -> ::core::ffi::c_uint {
    unsafe { reloc.offset_from((*(*reloc).sec).relocs) as ::core::ffi::c_uint }
}

#[inline]
pub unsafe fn reloc_rel(reloc: *mut reloc) -> *mut ::core::ffi::c_void {
    unsafe {
        let rsec = (*reloc).sec;
        ((*(*rsec).data).d_buf as *mut u8)
            .add((reloc_idx(reloc) as u64 * (*rsec).sh.sh_entsize) as usize)
            as *mut ::core::ffi::c_void
    }
}

#[inline]
pub unsafe fn is_32bit_reloc(reloc: *mut reloc) -> bool {
    /*
     * Elf32_Rel:   8 bytes
     * Elf32_Rela: 12 bytes
     * Elf64_Rel:  16 bytes
     * Elf64_Rela: 24 bytes
     */
    unsafe { (*(*reloc).sec).sh.sh_entsize < 16 }
}

#[inline]
pub unsafe fn sec_size(sec: *mut section) -> ::core::ffi::c_ulong {
    unsafe { (*sec).sh.sh_size as ::core::ffi::c_ulong }
}

#[inline]
unsafe fn __get_reloc_field_r_offset(reloc: *mut reloc) -> u64 {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_offset as u64
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_offset as u64
        }
    }
}

#[inline]
unsafe fn __set_reloc_field_r_offset(reloc: *mut reloc, val: u64) {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_offset = val as _;
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_offset = val as _;
        }
    }
}

#[inline]
unsafe fn __get_reloc_field_r_addend(reloc: *mut reloc) -> s64 {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_addend as s64
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_addend as s64
        }
    }
}

#[inline]
unsafe fn __set_reloc_field_r_addend(reloc: *mut reloc, val: s64) {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_addend = val as _;
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_addend = val as _;
        }
    }
}

#[inline]
unsafe fn __get_reloc_field_r_info(reloc: *mut reloc) -> u64 {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_info as u64
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_info as u64
        }
    }
}

#[inline]
unsafe fn __set_reloc_field_r_info(reloc: *mut reloc, val: u64) {
    unsafe {
        if is_32bit_reloc(reloc) {
            (*(reloc_rel(reloc) as *mut Elf32_Rela)).r_info = val as _;
        } else {
            (*(reloc_rel(reloc) as *mut Elf64_Rela)).r_info = val as _;
        }
    }
}

#[inline]
pub unsafe fn reloc_offset(reloc: *mut reloc) -> u64 {
    unsafe { __get_reloc_field_r_offset(reloc) }
}

#[inline]
pub unsafe fn set_reloc_offset(elf: *mut elf, reloc: *mut reloc, offset: u64) {
    unsafe {
        __set_reloc_field_r_offset(reloc, offset);
        mark_sec_changed(elf, (*reloc).sec, true);
    }
}

#[inline]
pub unsafe fn reloc_addend(reloc: *mut reloc) -> s64 {
    unsafe { __get_reloc_field_r_addend(reloc) }
}

#[inline]
pub unsafe fn set_reloc_addend(elf: *mut elf, reloc: *mut reloc, addend: s64) {
    unsafe {
        __set_reloc_field_r_addend(reloc, addend);
        mark_sec_changed(elf, (*reloc).sec, true);
    }
}

#[inline]
pub unsafe fn reloc_sym(reloc: *mut reloc) -> ::core::ffi::c_uint {
    unsafe {
        let info = __get_reloc_field_r_info(reloc);
        if is_32bit_reloc(reloc) {
            ELF32_R_SYM(info)
        } else {
            ELF64_R_SYM(info)
        }
    }
}

#[inline]
pub unsafe fn reloc_type(reloc: *mut reloc) -> ::core::ffi::c_uint {
    unsafe {
        let info = __get_reloc_field_r_info(reloc);
        if is_32bit_reloc(reloc) {
            ELF32_R_TYPE(info)
        } else {
            ELF64_R_TYPE(info)
        }
    }
}

#[inline]
pub unsafe fn set_reloc_sym(elf: *mut elf, reloc: *mut reloc, sym: ::core::ffi::c_uint) {
    unsafe {
        let info = if is_32bit_reloc(reloc) {
            ELF32_R_INFO(sym, reloc_type(reloc))
        } else {
            ELF64_R_INFO(sym, reloc_type(reloc))
        };
        __set_reloc_field_r_info(reloc, info);
        mark_sec_changed(elf, (*reloc).sec, true);
    }
}

#[inline]
pub unsafe fn set_reloc_type(elf: *mut elf, reloc: *mut reloc, type_: ::core::ffi::c_uint) {
    unsafe {
        let info = if is_32bit_reloc(reloc) {
            ELF32_R_INFO(reloc_sym(reloc), type_)
        } else {
            ELF64_R_INFO(reloc_sym(reloc), type_)
        };
        __set_reloc_field_r_info(reloc, info);
        mark_sec_changed(elf, (*reloc).sec, true);
    }
}

#[inline]
pub unsafe fn annotype(elf: *mut elf, sec: *mut section, reloc: *mut reloc) -> ::core::ffi::c_uint {
    unsafe {
        let type_ = *(((*(*sec).data).d_buf as *mut u8)
            .add((reloc_idx(reloc) * 8 + 4) as usize) as *mut u32);
        __bswap_if_needed(&mut (*elf).ehdr, type_)
    }
}

pub const RELOC_JUMP_TABLE_BIT: ::core::ffi::c_ulong = 1;

/* Does reloc mark the beginning of a jump table? */
#[inline]
pub unsafe fn is_jump_table(reloc: *mut reloc) -> bool {
    unsafe { ((*reloc)._sym_next_reloc & RELOC_JUMP_TABLE_BIT) != 0 }
}

#[inline]
pub unsafe fn set_jump_table(reloc: *mut reloc) {
    unsafe {
        (*reloc)._sym_next_reloc |= RELOC_JUMP_TABLE_BIT;
    }
}

#[inline]
pub unsafe fn sym_next_reloc(reloc: *mut reloc) -> *mut reloc {
    unsafe { ((*reloc)._sym_next_reloc & !RELOC_JUMP_TABLE_BIT) as *mut reloc }
}

#[inline]
pub unsafe fn set_sym_next_reloc(reloc: *mut reloc, next: *mut reloc) {
    unsafe {
        let bit = (*reloc)._sym_next_reloc & RELOC_JUMP_TABLE_BIT;
        (*reloc)._sym_next_reloc = next as ::core::ffi::c_ulong | bit;
    }
}

/* for_each_sec, sec_for_each_sym, sec_prev_sym, for_each_sym,
 * for_each_sym_continue, for_each_sym_by_name, and
 * for_each_sym_by_demangled_name are C list/hash iteration macros.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
