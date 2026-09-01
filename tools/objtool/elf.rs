// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * elf.c - ELF access library
 *
 * Adapted from kpatch (https://github.com/dynup/kpatch):
 * Copyright (C) 2013-2015 Josh Poimboeuf <jpoimboe@redhat.com>
 * Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{self, size_of};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type bool_ = bool;
type Elf32_Word = u32;
type Elf64_Xword = u64;
type Elf_Cmd = c_int;

#[repr(C)]
pub struct rb_node {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct elf_hash_node {
    pub next: *mut elf_hash_node,
}

#[repr(C)]
pub struct Elf {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: c_uint,
    pub d_size: size_t,
    pub d_off: i64,
    pub d_align: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Ehdr {
    pub e_shstrndx: Elf32_Word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Shdr {
    pub sh_name: Elf32_Word,
    pub sh_type: Elf32_Word,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: Elf32_Word,
    pub sh_info: Elf32_Word,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Sym {
    pub st_name: Elf32_Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf32_Word,
    pub st_value: u64,
    pub st_size: u64,
}

#[repr(C)]
pub struct elf {
    pub elf: *mut Elf,
    pub fd: c_int,
    pub name: *mut c_char,
    pub tmp_name: *mut c_char,
    pub ehdr: GElf_Ehdr,
    pub sections: list_head,
    pub symbols: list_head,
    pub section_data: *mut section,
    pub symbol_data: *mut symbol,
    pub num_files: c_uint,
    pub num_relocs: c_ulong,
    pub changed: bool,
    pub section_bits: c_int,
    pub section_name_bits: c_int,
    pub symbol_bits: c_int,
    pub symbol_name_bits: c_int,
    pub reloc_bits: c_int,
    pub section_table: *mut *mut elf_hash_node,
    pub section_name_table: *mut *mut elf_hash_node,
    pub symbol_table: *mut *mut elf_hash_node,
    pub symbol_name_table: *mut *mut elf_hash_node,
    pub reloc_table: *mut *mut elf_hash_node,
}

#[repr(C)]
pub struct section {
    pub list: list_head,
    pub symbol_list: list_head,
    pub symbol_tree: rb_root_cached,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub idx: c_uint,
    pub name: *mut c_char,
    pub sh: GElf_Shdr,
    pub data: *mut Elf_Data,
    pub sym: *mut symbol,
    pub rsec: *mut section,
    pub base: *mut section,
    pub relocs: *mut reloc,
    pub nr_alloc_relocs: c_ulong,
    pub rodata: bool,
    pub truncate: bool,
}

#[repr(C)]
pub struct symbol {
    pub list: list_head,
    pub global_list: list_head,
    pub pv_target: list_head,
    pub node: rb_node,
    pub __subtree_last: c_ulong,
    pub hash: elf_hash_node,
    pub name_hash: elf_hash_node,
    pub idx: c_uint,
    pub name: *mut c_char,
    pub demangled_name: *const c_char,
    pub sym: GElf_Sym,
    pub sec: *mut section,
    pub file: *mut symbol,
    pub alias: *mut symbol,
    pub pfunc: *mut symbol,
    pub cfunc: *mut symbol,
    pub relocs: *mut reloc,
    pub group_sec: *mut section,
    pub offset: c_ulong,
    pub len: c_ulong,
    pub type_: c_uint,
    pub bind: c_uint,
    pub prefix: c_uint,
    pub klp: c_uint,
    pub cold: c_uint,
}

#[repr(C)]
pub struct reloc {
    pub hash: elf_hash_node,
    pub sec: *mut section,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct opts_t {
    pub stats: bool,
}

#[repr(C)]
pub struct symbol_hole {
    pub key: c_ulong,
    pub sym: *const symbol,
}

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANON: c_int = 0x20;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_ACCMODE: c_int = 3;
const EV_CURRENT: c_uint = 1;
const ELF_C_READ_MMAP: Elf_Cmd = 1;
const ELF_C_RDWR: Elf_Cmd = 2;
const ELF_C_WRITE: Elf_Cmd = 3;
const ELF_C_SET: Elf_Cmd = 4;
const ELF_F_DIRTY: c_uint = 0x1;
const ELFCLASS64: c_uint = 2;
const ELF_T_SYM: c_uint = 1;
const ELF_T_WORD: c_uint = 2;
const ELF_T_RELA: c_uint = 3;
const ELF_T_BYTE: c_uint = 4;
const SHT_NULL: c_uint = 0;
const SHT_PROGBITS: c_uint = 1;
const SHT_SYMTAB: c_uint = 2;
const SHT_STRTAB: c_uint = 3;
const SHT_RELA: c_uint = 4;
const SHT_GROUP: c_uint = 17;
const SHF_ALLOC: c_uint = 0x2;
const SHF_INFO_LINK: c_uint = 0x40;
const STB_LOCAL: c_uint = 0;
const STT_SECTION: c_uint = 3;
const STT_FUNC: c_uint = 2;
const SHN_UNDEF: Elf32_Word = 0;
const SHN_LORESERVE: Elf32_Word = 0xff00;
const SHN_XINDEX: Elf32_Word = 0xffff;
const OFFSET_STRIDE_MASK: c_ulong = 0;

unsafe extern "C" {
    static mut opts: opts_t;
    static mut errno: c_int;

    fn jhash(key: *const c_void, length: u32, initval: u32) -> u32;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: c_long) -> *mut c_void;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn rename(old: *const c_char, new: *const c_char) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn elf_version(version: c_uint) -> c_uint;
    fn elf_getshdrnum(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getscn(elf: *mut Elf, idx: size_t) -> *mut Elf_Scn;
    fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn;
    fn elf_ndxscn(scn: *mut Elf_Scn) -> size_t;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *mut c_char;
    fn elf_begin(fd: c_int, cmd: Elf_Cmd, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_flagelf(elf: *mut Elf, cmd: Elf_Cmd, flags: c_uint) -> c_uint;
    fn elf_update(elf: *mut Elf, cmd: Elf_Cmd) -> c_long;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn gelf_newehdr(elf: *mut Elf, class: c_uint) -> *mut GElf_Ehdr;
    fn gelf_update_ehdr(elf: *mut Elf, src: *mut GElf_Ehdr) -> c_int;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn gelf_update_shdr(scn: *mut Elf_Scn, src: *mut GElf_Shdr) -> c_int;
    fn gelf_getsymshndx(data: *mut Elf_Data, shndxdata: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym, shndx: *mut Elf32_Word) -> *mut GElf_Sym;
    fn gelf_update_symshndx(data: *mut Elf_Data, shndxdata: *mut Elf_Data, ndx: c_int, src: *mut GElf_Sym, shndx: Elf32_Word) -> c_int;

    fn rb_find(key: *const c_void, root: *const rb_root, cmp: unsafe extern "C" fn(*const c_void, *const rb_node) -> c_int) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_prev(node: *const rb_node) -> *mut rb_node;
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn __sym_iter_first(root: *mut rb_root_cached, start: c_ulong, end: c_ulong) -> *mut symbol;
    fn __sym_iter_next(sym: *mut symbol, start: c_ulong, end: c_ulong) -> *mut symbol;
    fn __sym_insert(sym: *mut symbol, root: *mut rb_root_cached);

    fn ERROR(fmt: *const c_char, ...);
    fn ERROR_GLIBC(msg: *const c_char);
    fn ERROR_ELF(msg: *const c_char);
    fn ERROR_FUNC(sec: *mut section, offset: c_ulong, msg: *const c_char);
}

unsafe fn strstarts(s: *const c_char, prefix: *const c_char) -> bool {
    strncmp(s, prefix, strlen(prefix)) == 0
}

unsafe fn GELF_ST_TYPE(info: u8) -> c_uint {
    (info & 0xf) as c_uint
}

unsafe fn GELF_ST_BIND(info: u8) -> c_uint {
    (info >> 4) as c_uint
}

unsafe fn GELF_ST_INFO(bind: c_uint, type_: c_uint) -> u8 {
    ((bind << 4) + (type_ & 0xf)) as u8
}

unsafe fn ALIGN(x: u64, a: u64) -> u64 {
    (x + a - 1) & !(a - 1)
}

unsafe fn max_ulong(a: c_ulong, b: c_ulong) -> c_ulong {
    if a > b { a } else { b }
}

unsafe fn max_uint(a: c_uint, b: c_uint) -> c_uint {
    if a > b { a } else { b }
}

unsafe fn ilog2(mut x: size_t) -> c_int {
    let mut r = 0;
    while x > 1 {
        x >>= 1;
        r += 1;
    }
    r
}

unsafe fn roundup_pow_of_two(mut x: c_ulong) -> c_ulong {
    if x <= 1 {
        return 1;
    }
    x -= 1;
    let mut shift = 1;
    while shift < size_of::<c_ulong>() * 8 {
        x |= x >> shift;
        shift <<= 1;
    }
    x + 1
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    let next = (*head).next;
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = head;
    (*head).next = new;
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn sec_size(sec: *const section) -> u64 {
    (*sec).sh.sh_size
}

unsafe fn sec_num_entries(sec: *const section) -> c_uint {
    if (*sec).sh.sh_entsize == 0 {
        0
    } else {
        ((*sec).sh.sh_size / (*sec).sh.sh_entsize) as c_uint
    }
}

unsafe fn mark_sec_changed(elf: *mut elf, sec: *mut section, changed: bool) {
    let _ = sec;
    (*elf).changed = changed;
}

unsafe fn sec_changed(_sec: *mut section) -> bool {
    true
}

unsafe fn is_sec_sym(sym: *const symbol) -> bool {
    (*sym).type_ == STT_SECTION
}

unsafe fn is_file_sym(_sym: *const symbol) -> bool {
    false
}

unsafe fn is_func_sym(sym: *const symbol) -> bool {
    (*sym).type_ == STT_FUNC
}

unsafe fn is_object_sym(_sym: *const symbol) -> bool {
    false
}

unsafe fn is_local_sym(sym: *const symbol) -> bool {
    (*sym).bind == STB_LOCAL
}

unsafe fn is_undef_sym(sym: *const symbol) -> bool {
    (*(*sym).sec).idx == SHN_UNDEF
}

unsafe fn is_reloc_sec(sec: *const section) -> bool {
    (*sec).sh.sh_type == SHT_RELA
}

unsafe fn is_text_sec(_sec: *const section) -> bool {
    false
}

unsafe fn elf_rela_size(_elf: *mut elf) -> c_ulong {
    24
}

unsafe fn elf_addr_size(_elf: *mut elf) -> c_uint {
    8
}

unsafe fn elf_text_rela_type(_elf: *mut elf) -> c_uint {
    0
}

unsafe fn elf_data_rela_type(_elf: *mut elf) -> c_uint {
    0
}

unsafe fn reloc_hash(reloc: *mut reloc) -> u32 {
    sec_offset_hash((*reloc).sec, reloc_offset(reloc)) as u32
}

unsafe fn sec_offset_hash(_sec: *mut section, offset: c_ulong) -> c_ulong {
    offset
}

unsafe fn reloc_offset(_reloc: *mut reloc) -> c_ulong {
    0
}

unsafe fn reloc_sym(_reloc: *mut reloc) -> c_uint {
    0
}

unsafe fn set_reloc_offset(_elf: *mut elf, _reloc: *mut reloc, _offset: c_ulong) {}
unsafe fn set_reloc_sym(_elf: *mut elf, _reloc: *mut reloc, _idx: c_uint) {}
unsafe fn set_reloc_type(_elf: *mut elf, _reloc: *mut reloc, _type: c_uint) {}
unsafe fn set_reloc_addend(_elf: *mut elf, _reloc: *mut reloc, _addend: s64) {}
unsafe fn sym_next_reloc(_reloc: *mut reloc) -> *mut reloc { ptr::null_mut() }
unsafe fn set_sym_next_reloc(_reloc: *mut reloc, _next: *mut reloc) {}

unsafe fn table_entry(table: *mut *mut elf_hash_node, bits: c_int, key: c_ulong) -> *mut *mut elf_hash_node {
    table.add((key & ((1usize << bits) as c_ulong - 1)) as usize)
}

unsafe fn elf_hash_add(table: *mut *mut elf_hash_node, bits: c_int, node: *mut elf_hash_node, key: c_ulong) {
    let head = table_entry(table, bits, key);
    (*node).next = *head;
    *head = node;
}

unsafe fn __elf_hash_del(node: *mut elf_hash_node, head: *mut *mut elf_hash_node) {
    let mut cur: *mut elf_hash_node;
    let mut prev: *mut elf_hash_node;

    if node == *head {
        *head = (*node).next;
        return;
    }

    prev = ptr::null_mut();
    cur = *head;
    while !cur.is_null() {
        if cur == node {
            (*prev).next = (*cur).next;
            break;
        }
        prev = cur;
        cur = (*cur).next;
    }
}

unsafe fn elf_alloc_hash(table: *mut *mut *mut elf_hash_node, bits: *mut c_int, size: size_t, name: *const c_char) -> *mut *mut elf_hash_node {
    *bits = if 10 > ilog2(size) { 10 } else { ilog2(size) };
    *table = mmap(ptr::null_mut(), size_of::<*mut elf_hash_node>() << *bits, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0) as *mut *mut elf_hash_node;
    if *table == (-1isize as *mut *mut elf_hash_node) {
        ERROR_GLIBC(name);
        *table = ptr::null_mut();
    }
    *table
}

unsafe fn str_hash(name: *const c_char) -> u32 {
    jhash(name as *const c_void, strlen(name) as u32, 0)
}

#[no_mangle]
pub unsafe extern "C" fn str_hash_demangled(str_: *const c_char) -> u32 {
    jhash(str_ as *const c_void, demangled_name_len(str_) as u32, 0)
}

unsafe fn __sym_start(s: *mut symbol) -> c_ulong {
    (*s).offset
}

unsafe fn __sym_last(s: *mut symbol) -> c_ulong {
    (*s).offset + if (*s).len != 0 { (*s).len - 1 } else { 0 }
}

/*
 * Find the last symbol before @offset.
 */
unsafe extern "C" fn symbol_hole_by_offset(key: *const c_void, node: *const rb_node) -> c_int {
    let s = node as *const symbol;
    let sh = key as *mut symbol_hole;

    if (*sh).key < (*s).offset {
        return -1;
    }

    if (*sh).key >= (*s).offset + (*s).len {
        (*sh).sym = s;
        return 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn find_section_by_name(elf: *const elf, name: *const c_char) -> *mut section {
    let key = str_hash(name) as c_ulong;
    let mut node = *table_entry((*elf).section_name_table, (*elf).section_name_bits, key);
    while !node.is_null() {
        let sec = node as *mut section;
        if strcmp((*sec).name, name) == 0 {
            return sec;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

unsafe fn find_section_by_index(elf: *mut elf, idx: c_uint) -> *mut section {
    let mut node = *table_entry((*elf).section_table, (*elf).section_bits, idx as c_ulong);
    while !node.is_null() {
        let sec = node as *mut section;
        if (*sec).idx == idx {
            return sec;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

unsafe fn find_symbol_by_index(elf: *mut elf, idx: c_uint) -> *mut symbol {
    let mut node = *table_entry((*elf).symbol_table, (*elf).symbol_bits, idx as c_ulong);
    while !node.is_null() {
        let sym = node as *mut symbol;
        if (*sym).idx == idx {
            return sym;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn find_symbol_by_offset(sec: *mut section, offset: c_ulong) -> *mut symbol {
    let tree = &mut (*sec).symbol_tree as *mut rb_root_cached;
    let mut sym = __sym_iter_first(tree, offset, offset);
    while !sym.is_null() {
        if (*sym).offset == offset && !is_sec_sym(sym) {
            return (*sym).alias;
        }
        sym = __sym_iter_next(sym, offset, offset);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn find_func_by_offset(sec: *mut section, offset: c_ulong) -> *mut symbol {
    let tree = &mut (*sec).symbol_tree as *mut rb_root_cached;
    let mut func = __sym_iter_first(tree, offset, offset);
    while !func.is_null() {
        if (*func).offset == offset && is_func_sym(func) {
            return (*func).alias;
        }
        func = __sym_iter_next(func, offset, offset);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn find_symbol_containing(sec: *const section, offset: c_ulong) -> *mut symbol {
    let tree = &(*sec).symbol_tree as *const rb_root_cached as *mut rb_root_cached;
    let mut sym: *mut symbol = ptr::null_mut();
    let mut tmp = __sym_iter_first(tree, offset, offset);
    while !tmp.is_null() {
        if (*tmp).len != 0 {
            if sym.is_null() {
                sym = tmp;
                tmp = __sym_iter_next(tmp, offset, offset);
                continue;
            }

            if (*sym).offset != (*tmp).offset || (*sym).len != (*tmp).len {
                /*
                 * In the rare case of overlapping symbols,
                 * pick the smaller one.
                 *
                 * TODO: outlaw overlapping symbols
                 */
                if (*tmp).len < (*sym).len {
                    sym = tmp;
                }
            }
        }
        tmp = __sym_iter_next(tmp, offset, offset);
    }

    if !sym.is_null() { (*sym).alias } else { ptr::null_mut() }
}

/*
 * Also match the symbol end address which can be used for a bounds comparison.
 */
#[no_mangle]
pub unsafe extern "C" fn find_symbol_containing_inclusive(sec: *const section, offset: c_ulong) -> *mut symbol {
    let mut sym = find_symbol_containing(sec, offset);

    if sym.is_null() && offset != 0 {
        sym = find_symbol_containing(sec, offset - 1);
    }

    sym
}

/*
 * Returns size of hole starting at @offset.
 */
#[no_mangle]
pub unsafe extern "C" fn find_symbol_hole_containing(sec: *const section, offset: c_ulong) -> c_int {
    let mut hole = symbol_hole {
        key: offset,
        sym: ptr::null(),
    };
    let mut n: *mut rb_node;

    /*
     * Find the rightmost symbol for which @offset is after it.
     */
    n = rb_find(&mut hole as *mut _ as *const c_void, &(*sec).symbol_tree.rb_root, symbol_hole_by_offset);

    /* found a symbol that contains @offset */
    if !n.is_null() {
        return 0; /* not a hole */
    }

    /*
     * @offset >= sym->offset + sym->len, find symbol after it.
     * When hole.sym is empty, use the first node to compute the hole.
     * If there is no symbol in the section, the first node will be NULL,
     * in which case, -1 is returned to skip the whole section.
     */
    if !hole.sym.is_null() {
        n = rb_next(&(*hole.sym).node);
    } else {
        n = rb_first_cached(&(*sec).symbol_tree);
    }

    if n.is_null() {
        return -1; /* until end of address space */
    }

    /* hole until start of next symbol */
    let s = n as *mut symbol;
    ((*s).offset - offset) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn find_func_containing(sec: *mut section, offset: c_ulong) -> *mut symbol {
    let tree = &mut (*sec).symbol_tree as *mut rb_root_cached;
    let mut func = __sym_iter_first(tree, offset, offset);
    while !func.is_null() {
        if is_func_sym(func) {
            return (*func).alias;
        }
        func = __sym_iter_next(func, offset, offset);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn find_symbol_by_name(elf: *const elf, name: *const c_char) -> *mut symbol {
    let key = str_hash(name) as c_ulong;
    let mut node = *table_entry((*elf).symbol_name_table, (*elf).symbol_name_bits, key);
    while !node.is_null() {
        let sym = node as *mut symbol;
        if strcmp((*sym).name, name) == 0 {
            return sym;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

/* Find local symbol with matching STT_FILE */
unsafe fn find_local_symbol_by_file_and_name(elf: *const elf, file: *mut symbol, name: *const c_char) -> *mut symbol {
    let key = str_hash_demangled(name) as c_ulong;
    let mut node = *table_entry((*elf).symbol_name_table, (*elf).symbol_name_bits, key);
    while !node.is_null() {
        let sym = node as *mut symbol;
        if (*sym).bind == STB_LOCAL && (*sym).file == file && strcmp((*sym).name, name) == 0 {
            return sym;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn find_global_symbol_by_name(elf: *const elf, name: *const c_char) -> *mut symbol {
    let key = str_hash_demangled(name) as c_ulong;
    let mut node = *table_entry((*elf).symbol_name_table, (*elf).symbol_name_bits, key);
    while !node.is_null() {
        let sym = node as *mut symbol;
        if strcmp((*sym).name, name) == 0 && !is_local_sym(sym) {
            return sym;
        }
        node = (*node).next;
    }
    ptr::null_mut()
}

/* If there are multiple matches, return the first one in the range */
#[no_mangle]
pub unsafe extern "C" fn find_reloc_by_dest_range(elf: *const elf, sec: *mut section, offset: c_ulong, len: c_uint) -> *mut reloc {
    let mut r: *mut reloc = ptr::null_mut();
    let rsec = (*sec).rsec;
    if rsec.is_null() {
        return ptr::null_mut();
    }

    let mut o = offset;
    while o < offset + len as c_ulong {
        let key = sec_offset_hash(rsec, o);
        let mut node = *table_entry((*elf).reloc_table, (*elf).reloc_bits, key);
        while !node.is_null() {
            let reloc = node as *mut reloc;
            if (*reloc).sec != rsec {
                node = (*node).next;
                continue;
            }

            if reloc_offset(reloc) >= offset && reloc_offset(reloc) < offset + len as c_ulong {
                if r.is_null() || reloc_offset(reloc) < reloc_offset(r) {
                    r = reloc;
                }
            }
            node = (*node).next;
        }
        if !r.is_null() && (reloc_offset(r) & OFFSET_STRIDE_MASK) == o {
            return r;
        }
        o += 1;
    }

    r
}

#[no_mangle]
pub unsafe extern "C" fn find_reloc_by_dest(elf: *const elf, sec: *mut section, offset: c_ulong) -> *mut reloc {
    find_reloc_by_dest_range(elf, sec, offset, 1)
}

unsafe fn is_dwarf_section(sec: *mut section) -> bool {
    strncmp((*sec).name, b".debug_\0".as_ptr() as *const c_char, 7) == 0
}

unsafe fn read_sections(elf: *mut elf) -> c_int {
    let mut s: *mut Elf_Scn = ptr::null_mut();
    let mut shstrndx: size_t = 0;
    let mut sections_nr: size_t = 0;

    if elf_getshdrnum((*elf).elf, &mut sections_nr) != 0 {
        ERROR_ELF(b"elf_getshdrnum\0".as_ptr() as *const c_char);
        return -1;
    }

    if elf_getshdrstrndx((*elf).elf, &mut shstrndx) != 0 {
        ERROR_ELF(b"elf_getshdrstrndx\0".as_ptr() as *const c_char);
        return -1;
    }

    if elf_alloc_hash(&mut (*elf).section_table, &mut (*elf).section_bits, sections_nr, b"mmap fail section\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).section_name_table, &mut (*elf).section_name_bits, sections_nr, b"mmap fail section_name\0".as_ptr() as *const c_char).is_null() {
        return -1;
    }

    (*elf).section_data = calloc(sections_nr, size_of::<section>()) as *mut section;
    if (*elf).section_data.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut i: size_t = 0;
    while i < sections_nr {
        let sec = (*elf).section_data.add(i);

        INIT_LIST_HEAD(&mut (*sec).symbol_list);

        s = elf_getscn((*elf).elf, i);
        if s.is_null() {
            ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
            return -1;
        }

        (*sec).idx = elf_ndxscn(s) as c_uint;

        if gelf_getshdr(s, &mut (*sec).sh).is_null() {
            ERROR_ELF(b"gelf_getshdr\0".as_ptr() as *const c_char);
            return -1;
        }

        (*sec).name = elf_strptr((*elf).elf, shstrndx, (*sec).sh.sh_name as size_t);
        if (*sec).name.is_null() {
            ERROR_ELF(b"elf_strptr\0".as_ptr() as *const c_char);
            return -1;
        }

        if sec_size(sec) != 0 && !is_dwarf_section(sec) {
            (*sec).data = elf_getdata(s, ptr::null_mut());
            if (*sec).data.is_null() {
                ERROR_ELF(b"elf_getdata\0".as_ptr() as *const c_char);
                return -1;
            }
            if (*(*sec).data).d_off != 0 || (*(*sec).data).d_size as u64 != sec_size(sec) {
                ERROR(b"unexpected data attributes for %s\0".as_ptr() as *const c_char, (*sec).name);
                return -1;
            }
        }

        list_add_tail(&mut (*sec).list, &mut (*elf).sections);
        elf_hash_add((*elf).section_table, (*elf).section_bits, &mut (*sec).hash, (*sec).idx as c_ulong);
        elf_hash_add((*elf).section_name_table, (*elf).section_name_bits, &mut (*sec).name_hash, str_hash((*sec).name) as c_ulong);

        if is_reloc_sec(sec) {
            (*elf).num_relocs += sec_num_entries(sec) as c_ulong;
        }
        i += 1;
    }

    if opts.stats {
        printf(b"nr_sections: %lu\n\0".as_ptr() as *const c_char, sections_nr as c_ulong);
        printf(b"section_bits: %d\n\0".as_ptr() as *const c_char, (*elf).section_bits);
    }

    /* sanity check, one more call to elf_nextscn() should return NULL */
    if !elf_nextscn((*elf).elf, s).is_null() {
        ERROR(b"section entry mismatch\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

/*
 * Returns desired length of the demangled name.
 * If name doesn't need demangling, return strlen(name).
 */
unsafe fn demangled_name_len(name: *const c_char) -> ssize_t {
    let mut idx: ssize_t;
    let p: *const c_char;

    p = strstr(name, b".llvm.\0".as_ptr() as *const c_char);
    if !p.is_null() {
        return p.offset_from(name) as ssize_t;
    }

    if !strstarts(name, b"__UNIQUE_ID_\0".as_ptr() as *const c_char) && strchr(name, '.' as c_int).is_null() {
        return strlen(name) as ssize_t;
    }

    idx = strlen(name) as ssize_t - 1;
    while idx >= 0 {
        let c = *name.offset(idx) as c_int;

        if isdigit(c) == 0 && c != '.' as c_int && c != '_' as c_int {
            break;
        }
        idx -= 1;
    }
    if idx <= 0 {
        return strlen(name) as ssize_t;
    }
    idx + 1
}

/*
 * Remove number suffix of a symbol.
 *
 * Specifically, remove trailing numbers for "__UNIQUE_ID_" symbols and
 * symbols with '.'.
 *
 * With CONFIG_LTO_CLANG_THIN, it is possible to have nested __UNIQUE_ID_,
 * such as
 *
 *   __UNIQUE_ID_addressable___UNIQUE_ID_pci_invalid_bar_694_695
 *
 * to remove both trailing numbers, also remove trailing '_'.
 *
 * For symbols with llvm suffix, i.e., foo.llvm.<hash>, remove the
 * .llvm.<hash> part.
 */
unsafe fn demangle_name(sym: *mut symbol) -> *const c_char {
    let str_: *mut c_char;
    let len: ssize_t;

    if !is_func_sym(sym) && !is_object_sym(sym) {
        return (*sym).name;
    }

    len = demangled_name_len((*sym).name);
    if len as size_t == strlen((*sym).name) {
        return (*sym).name;
    }

    str_ = strndup((*sym).name, len as size_t);
    if str_.is_null() {
        ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char);
        return ptr::null();
    }

    str_
}

unsafe fn elf_add_symbol(elf: *mut elf, sym: *mut symbol) -> c_int {
    INIT_LIST_HEAD(&mut (*sym).pv_target);
    (*sym).alias = sym;

    (*sym).type_ = GELF_ST_TYPE((*sym).sym.st_info);
    (*sym).bind = GELF_ST_BIND((*sym).sym.st_info);

    if is_file_sym(sym) {
        (*elf).num_files += 1;
    }

    (*sym).offset = (*sym).sym.st_value as c_ulong;
    (*sym).len = (*sym).sym.st_size as c_ulong;

    let mut iter = __sym_iter_first(&mut (*(*sym).sec).symbol_tree, (*sym).offset, (*sym).offset);
    while !iter.is_null() {
        if !is_undef_sym(iter) && (*iter).offset == (*sym).offset && (*iter).type_ == (*sym).type_ && (*iter).len == (*sym).len {
            (*iter).alias = sym;
        }
        iter = __sym_iter_next(iter, (*sym).offset, (*sym).offset);
    }

    __sym_insert(sym, &mut (*(*sym).sec).symbol_tree);
    let pnode = rb_prev(&(*sym).node);
    let entry = if !pnode.is_null() {
        &mut (*(pnode as *mut symbol)).list as *mut list_head
    } else {
        &mut (*(*sym).sec).symbol_list as *mut list_head
    };
    list_add(&mut (*sym).list, entry);

    (*sym).demangled_name = demangle_name(sym);
    if (*sym).demangled_name.is_null() {
        return -1;
    }

    list_add_tail(&mut (*sym).global_list, &mut (*elf).symbols);
    elf_hash_add((*elf).symbol_table, (*elf).symbol_bits, &mut (*sym).hash, (*sym).idx as c_ulong);
    elf_hash_add((*elf).symbol_name_table, (*elf).symbol_name_bits, &mut (*sym).name_hash, str_hash((*sym).demangled_name) as c_ulong);

    if is_func_sym(sym)
        && (strstarts((*sym).name, b"__pfx_\0".as_ptr() as *const c_char)
            || strstarts((*sym).name, b"__cfi_\0".as_ptr() as *const c_char)
            || strstarts((*sym).name, b"__pi___pfx_\0".as_ptr() as *const c_char)
            || strstarts((*sym).name, b"__pi___cfi_\0".as_ptr() as *const c_char)) {
        (*sym).prefix = 1;
    }

    if strstarts((*sym).name, b".klp.sym\0".as_ptr() as *const c_char) {
        (*sym).klp = 1;
    }

    if (*sym).klp == 0 && !is_sec_sym(sym) && !strstr((*sym).name, b".cold\0".as_ptr() as *const c_char).is_null() {
        (*sym).cold = 1;

        /*
         * Clang doesn't mark cold subfunctions as STT_FUNC, which
         * breaks several objtool assumptions.  Fake it.
         */
        (*sym).type_ = STT_FUNC;
    }

    (*sym).pfunc = sym;
    (*sym).cfunc = sym;

    0
}

unsafe fn read_symbols(elf: *mut elf) -> c_int {
    let symtab = find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char);
    let mut symtab_shndx: *mut section = ptr::null_mut();
    let mut file: *mut symbol = ptr::null_mut();
    let mut shndx_data: *mut Elf_Data = ptr::null_mut();
    let mut shndx: Elf32_Word = 0;
    let symbols_nr: c_int;

    if !symtab.is_null() {
        symtab_shndx = find_section_by_name(elf, b".symtab_shndx\0".as_ptr() as *const c_char);
        if !symtab_shndx.is_null() {
            shndx_data = (*symtab_shndx).data;
        }
        symbols_nr = sec_num_entries(symtab) as c_int;
    } else {
        /*
         * A missing symbol table is actually possible if it's an empty
         * .o file. This can happen for thunk_64.o. Make sure to at
         * least allocate the symbol hash tables so we can do symbol
         * lookups without crashing.
         */
        symbols_nr = 0;
    }

    if elf_alloc_hash(&mut (*elf).symbol_table, &mut (*elf).symbol_bits, symbols_nr as size_t, b"mmap fail symbol\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).symbol_name_table, &mut (*elf).symbol_name_bits, symbols_nr as size_t, b"mmap fail symbol_name\0".as_ptr() as *const c_char).is_null() {
        return -1;
    }

    (*elf).symbol_data = calloc(symbols_nr as size_t, size_of::<symbol>()) as *mut symbol;
    if (*elf).symbol_data.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return -1;
    }

    INIT_LIST_HEAD(&mut (*elf).symbols);

    let mut i = 0;
    while i < symbols_nr {
        let sym = (*elf).symbol_data.add(i as usize);

        (*sym).idx = i as c_uint;

        if gelf_getsymshndx((*symtab).data, shndx_data, i, &mut (*sym).sym, &mut shndx).is_null() {
            ERROR_ELF(b"gelf_getsymshndx\0".as_ptr() as *const c_char);
            return -1;
        }

        (*sym).name = elf_strptr((*elf).elf, (*symtab).sh.sh_link as size_t, (*sym).sym.st_name as size_t);
        if (*sym).name.is_null() {
            ERROR_ELF(b"elf_strptr\0".as_ptr() as *const c_char);
            return -1;
        }

        /*
         * "klp diff" renames the placeholder symbols of KLP relocs to
         * hide them from modpost.  Hide the prefix from the rest of
         * objtool so its many name-based heuristics (noreturns,
         * uaccess safe list, ...) still see the original symbol name.
         *
         * st_name is left alone, so the renamed symbol is preserved in
         * the output file.
         */
        let klp_tombstone_prefix = b".klp.sym.\0".as_ptr() as *const c_char;
        if strstarts((*sym).name, klp_tombstone_prefix) {
            (*sym).name = (*sym).name.add(strlen(klp_tombstone_prefix));
        }

        if (((*sym).sym.st_shndx > SHN_UNDEF && (*sym).sym.st_shndx < SHN_LORESERVE)
            || (!shndx_data.is_null() && (*sym).sym.st_shndx == SHN_XINDEX)) {
            if (*sym).sym.st_shndx != SHN_XINDEX {
                shndx = (*sym).sym.st_shndx;
            }

            (*sym).sec = find_section_by_index(elf, shndx);
            if (*sym).sec.is_null() {
                ERROR(b"couldn't find section for symbol %s\0".as_ptr() as *const c_char, (*sym).name);
                return -1;
            }
            if GELF_ST_TYPE((*sym).sym.st_info) == STT_SECTION {
                (*sym).name = (*(*sym).sec).name;
                (*(*sym).sec).sym = sym;
            }
        } else {
            (*sym).sec = find_section_by_index(elf, 0);
        }

        if elf_add_symbol(elf, sym) != 0 {
            return -1;
        }

        if is_file_sym(sym) {
            file = sym;
        } else if (*sym).bind == STB_LOCAL && !is_sec_sym(sym) {
            (*sym).file = file;
        }
        i += 1;
    }

    if opts.stats {
        printf(b"nr_symbols: %lu\n\0".as_ptr() as *const c_char, symbols_nr as c_ulong);
        printf(b"symbol_bits: %d\n\0".as_ptr() as *const c_char, (*elf).symbol_bits);
    }

    /* Create parent/child links for any cold subfunctions */
    let mut sec = (*elf).sections.next as *mut section;
    while sec != (&mut (*elf).sections as *mut list_head as *mut section) {
        let mut sym = (*sec).symbol_list.next as *mut symbol;
        while sym != (&mut (*sec).symbol_list as *mut list_head as *mut symbol) {
            if (*sym).cold != 0 {
                let coldstr = strstr((*sym).name, b".cold\0".as_ptr() as *const c_char);
                if coldstr.is_null() {
                    ERROR(b"%s(): cold subfunction without \".cold\"?\0".as_ptr() as *const c_char, (*sym).name);
                    return -1;
                }

                let pnamelen = coldstr.offset_from((*sym).name) as size_t;
                let pname = strndup((*sym).name, pnamelen);
                if pname.is_null() {
                    ERROR(b"%s(): failed to allocate memory\0".as_ptr() as *const c_char, (*sym).name);
                    return -1;
                }

                let mut pfunc = find_local_symbol_by_file_and_name(elf, (*sym).file, pname);
                if pfunc.is_null() {
                    pfunc = find_global_symbol_by_name(elf, pname);
                }
                free(pname as *mut c_void);

                if pfunc.is_null() {
                    ERROR(b"%s(): can't find parent function\0".as_ptr() as *const c_char, (*sym).name);
                    return -1;
                }

                (*sym).pfunc = (*pfunc).alias;
                (*pfunc).cfunc = sym;
                (*(*pfunc).alias).cfunc = sym;

                /*
                 * Unfortunately, -fnoreorder-functions puts the child
                 * inside the parent.  Remove the overlap so we can
                 * have sane assumptions.
                 *
                 * Note that pfunc->len now no longer matches
                 * pfunc->sym.st_size.
                 */
                if (*sym).sec == (*pfunc).sec
                    && (*sym).offset >= (*pfunc).offset
                    && (*sym).offset + (*sym).len == (*pfunc).offset + (*pfunc).len {
                    (*pfunc).len -= (*sym).len;
                }
            }
            sym = (*sym).list.next as *mut symbol;
        }
        sec = (*sec).list.next as *mut section;
    }

    0
}

unsafe fn mark_group_syms(elf: *mut elf) -> c_int {
    let symtab = find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char);
    if symtab.is_null() {
        ERROR(b"no .symtab\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut sec = (*elf).sections.next as *mut section;
    while sec != (&mut (*elf).sections as *mut list_head as *mut section) {
        if (*sec).sh.sh_type == SHT_GROUP && (*sec).sh.sh_link == (*symtab).idx {
            let sym = find_symbol_by_index(elf, (*sec).sh.sh_info);
            if sym.is_null() {
                ERROR(b"%s: can't find SHT_GROUP signature symbol\0".as_ptr() as *const c_char, (*sec).name);
                return -1;
            }

            (*sym).group_sec = sec;
        }
        sec = (*sec).list.next as *mut section;
    }

    0
}

/*
 * @sym's idx has changed.  Update the relocs which reference it.
 */
unsafe fn elf_update_sym_relocs(elf: *mut elf, sym: *mut symbol) -> c_int {
    let mut reloc = (*sym).relocs;

    while !reloc.is_null() {
        set_reloc_sym(elf, reloc, (*(*reloc).sym).idx);
        reloc = sym_next_reloc(reloc);
    }

    0
}

/*
 * The libelf API is terrible; gelf_update_sym*() takes a data block relative
 * index value, *NOT* the symbol index. As such, iterate the data blocks and
 * adjust index until it fits.
 *
 * If no data block is found, allow adding a new data block provided the index
 * is only one past the end.
 */
unsafe fn elf_update_symbol(elf: *mut elf, symtab: *mut section, symtab_shndx: *mut section, sym: *mut symbol) -> c_int {
    let mut shndx: Elf32_Word;
    let mut symtab_data: *mut Elf_Data = ptr::null_mut();
    let mut shndx_data: *mut Elf_Data = ptr::null_mut();
    let entsize = (*symtab).sh.sh_entsize;
    let mut idx = (*sym).idx as c_int;
    let is_special_shndx = (*sym).sym.st_shndx >= SHN_LORESERVE && (*sym).sym.st_shndx != SHN_XINDEX;

    shndx = if is_special_shndx { (*sym).sym.st_shndx } else { (*(*sym).sec).idx };

    let s = elf_getscn((*elf).elf, (*symtab).idx as size_t);
    if s.is_null() {
        ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut t: *mut Elf_Scn = ptr::null_mut();
    if !symtab_shndx.is_null() {
        t = elf_getscn((*elf).elf, (*symtab_shndx).idx as size_t);
        if t.is_null() {
            ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    loop {
        /* get next data descriptor for the relevant sections */
        symtab_data = elf_getdata(s, symtab_data);
        if !t.is_null() {
            shndx_data = elf_getdata(t, shndx_data);
        }

        /* end-of-list */
        if symtab_data.is_null() {
            /*
             * Over-allocate to avoid O(n^2) symbol creation
             * behaviour.  The down side is that libelf doesn't
             * like this; see elf_truncate_section() for the fixup.
             */
            let num = max_uint(1, (*sym).idx / 3);
            let mut buf: *mut c_void;

            if idx != 0 {
                /* we don't do holes in symbol tables */
                ERROR(b"index out of range\0".as_ptr() as *const c_char);
                return -1;
            }

            /* if @idx == 0, it's the next contiguous entry, create it */
            symtab_data = elf_newdata(s);
            if !t.is_null() {
                shndx_data = elf_newdata(t);
            }

            buf = calloc(num as size_t, entsize as size_t);
            if buf.is_null() {
                ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
                return -1;
            }

            (*symtab_data).d_buf = buf;
            (*symtab_data).d_size = num as size_t * entsize as size_t;
            (*symtab_data).d_align = 1;
            (*symtab_data).d_type = ELF_T_SYM;

            mark_sec_changed(elf, symtab, true);
            (*symtab).truncate = true;

            if !t.is_null() {
                buf = calloc(num as size_t, size_of::<Elf32_Word>());
                if buf.is_null() {
                    ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
                    return -1;
                }

                (*shndx_data).d_buf = buf;
                (*shndx_data).d_size = num as size_t * size_of::<Elf32_Word>();
                (*shndx_data).d_align = size_of::<Elf32_Word>();
                (*shndx_data).d_type = ELF_T_WORD;

                mark_sec_changed(elf, symtab_shndx, true);
                (*symtab_shndx).truncate = true;
            }

            break;
        }

        /* empty blocks should not happen */
        if (*symtab_data).d_size == 0 {
            ERROR(b"zero size data\0".as_ptr() as *const c_char);
            return -1;
        }

        /* is this the right block? */
        let max_idx = ((*symtab_data).d_size as u64 / entsize) as c_int;
        if idx < max_idx {
            break;
        }

        /* adjust index and try again */
        idx -= max_idx;
    }

    /* something went side-ways */
    if idx < 0 {
        ERROR(b"negative index\0".as_ptr() as *const c_char);
        return -1;
    }

    /* setup extended section index magic and write the symbol */
    if shndx < SHN_LORESERVE || is_special_shndx {
        (*sym).sym.st_shndx = shndx;
        if shndx_data.is_null() {
            shndx = 0;
        }
    } else {
        (*sym).sym.st_shndx = SHN_XINDEX;
        if shndx_data.is_null() {
            ERROR(b"no .symtab_shndx\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    if gelf_update_symshndx(symtab_data, shndx_data, idx, &mut (*sym).sym, shndx) == 0 {
        ERROR_ELF(b"gelf_update_symshndx\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_symbol(elf: *mut elf, name: *const c_char, sec: *mut section, bind: c_uint, type_: c_uint, offset: c_ulong, size: size_t) -> *mut symbol {
    let sym = calloc(1, size_of::<symbol>()) as *mut symbol;
    if sym.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*sym).name = strdup(name);
    if (*sym).name.is_null() {
        ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    if type_ != STT_SECTION {
        (*sym).sym.st_name = elf_add_string(elf, ptr::null_mut(), (*sym).name);
        if (*sym).sym.st_name == u32::MAX {
            return ptr::null_mut();
        }
    }

    if !sec.is_null() {
        (*sym).sec = sec;
    } else {
        (*sym).sec = find_section_by_index(elf, 0);
        if (*sym).sec.is_null() {
            ERROR(b"no NULL section\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }
    }

    (*sym).sym.st_info = GELF_ST_INFO(bind, type_);
    (*sym).sym.st_value = offset as u64;
    (*sym).sym.st_size = size as u64;

    let symtab = find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char);
    if symtab.is_null() {
        ERROR(b"no .symtab\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    let symtab_shndx = find_section_by_name(elf, b".symtab_shndx\0".as_ptr() as *const c_char);

    let mut new_idx = sec_num_entries(symtab) as Elf32_Word;

    if bind == STB_LOCAL {
        /*
         * Move the first global symbol, as per sh_info, into a new, higher
         * symbol index. This frees up a spot for a new local symbol.
         */
        let first_non_local = (*symtab).sh.sh_info;
        let old = find_symbol_by_index(elf, first_non_local);
        if !old.is_null() {
            __elf_hash_del(&mut (*old).hash, table_entry((*elf).symbol_table, (*elf).symbol_bits, (*old).idx as c_ulong));
            elf_hash_add((*elf).symbol_table, (*elf).symbol_bits, &mut (*old).hash, new_idx as c_ulong);
            (*old).idx = new_idx;

            if elf_update_symbol(elf, symtab, symtab_shndx, old) != 0 {
                ERROR(b"elf_update_symbol move\0".as_ptr() as *const c_char);
                return ptr::null_mut();
            }

            if elf_update_sym_relocs(elf, old) != 0 {
                return ptr::null_mut();
            }

            if !(*old).group_sec.is_null() {
                (*(*old).group_sec).sh.sh_info = new_idx;
                mark_sec_changed(elf, (*old).group_sec, true);
            }

            new_idx = first_non_local;
        }

        /*
         * Either way, we will add a LOCAL symbol.
         */
        (*symtab).sh.sh_info += 1;
    }

    (*sym).idx = new_idx;
    if (*sym).idx != 0 && elf_update_symbol(elf, symtab, symtab_shndx, sym) != 0 {
        return ptr::null_mut();
    }

    (*symtab).sh.sh_size += (*symtab).sh.sh_entsize;
    mark_sec_changed(elf, symtab, true);

    if !symtab_shndx.is_null() {
        (*symtab_shndx).sh.sh_size += size_of::<Elf32_Word>() as u64;
        mark_sec_changed(elf, symtab_shndx, true);
    }

    if elf_add_symbol(elf, sym) != 0 {
        return ptr::null_mut();
    }

    sym
}

#[no_mangle]
pub unsafe extern "C" fn elf_write_symbol(elf: *mut elf, sym: *mut symbol) -> c_int {
    let symtab = find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char);
    if symtab.is_null() {
        ERROR(b"no .symtab\0".as_ptr() as *const c_char);
        return -1;
    }

    let symtab_shndx = find_section_by_name(elf, b".symtab_shndx\0".as_ptr() as *const c_char);

    if elf_update_symbol(elf, symtab, symtab_shndx, sym) != 0 {
        return -1;
    }

    mark_sec_changed(elf, symtab, true);

    0
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_section_symbol(elf: *mut elf, sec: *mut section) -> *mut symbol {
    let sym = elf_create_symbol(elf, (*sec).name, sec, STB_LOCAL, STT_SECTION, 0, 0);
    if sym.is_null() {
        return ptr::null_mut();
    }

    (*sec).sym = sym;

    sym
}

#[no_mangle]
pub unsafe extern "C" fn elf_init_reloc(elf: *mut elf, rsec: *mut section, reloc_idx: c_uint, offset: c_ulong, sym: *mut symbol, addend: s64, type_: c_uint) -> *mut reloc {
    let empty: reloc = mem::zeroed();

    if reloc_idx >= sec_num_entries(rsec) {
        ERROR(b"%s: bad reloc_idx %u for %s with %d relocs\0".as_ptr() as *const c_char, b"elf_init_reloc\0".as_ptr() as *const c_char, reloc_idx, (*rsec).name, sec_num_entries(rsec));
        return ptr::null_mut();
    }

    let reloc = (*rsec).relocs.add(reloc_idx as usize);

    if memcmp(reloc as *const c_void, &empty as *const _ as *const c_void, size_of::<reloc>()) != 0 {
        ERROR(b"%s: %s: reloc %d already initialized!\0".as_ptr() as *const c_char, b"elf_init_reloc\0".as_ptr() as *const c_char, (*rsec).name, reloc_idx);
        return ptr::null_mut();
    }

    (*reloc).sec = rsec;
    (*reloc).sym = sym;

    set_reloc_offset(elf, reloc, offset);
    set_reloc_sym(elf, reloc, (*sym).idx);
    set_reloc_type(elf, reloc, type_);
    set_reloc_addend(elf, reloc, addend);

    elf_hash_add((*elf).reloc_table, (*elf).reloc_bits, &mut (*reloc).hash, reloc_hash(reloc) as c_ulong);
    set_sym_next_reloc(reloc, (*sym).relocs);
    (*sym).relocs = reloc;

    reloc
}

#[no_mangle]
pub unsafe extern "C" fn elf_init_reloc_text_sym(elf: *mut elf, sec: *mut section, offset: c_ulong, reloc_idx: c_uint, insn_sec: *mut section, insn_off: c_ulong) -> *mut reloc {
    let mut sym = (*insn_sec).sym;
    let addend = insn_off as s64;

    if !is_text_sec(insn_sec) {
        ERROR(b"bad call to %s() for data symbol %s\0".as_ptr() as *const c_char, b"elf_init_reloc_text_sym\0".as_ptr() as *const c_char, (*sym).name);
        return ptr::null_mut();
    }

    if sym.is_null() {
        /*
         * Due to how weak functions work, we must use section based
         * relocations. Symbol based relocations would result in the
         * weak and non-weak function annotations being overlaid on the
         * non-weak function after linking.
         */
        sym = elf_create_section_symbol(elf, insn_sec);
        if sym.is_null() {
            return ptr::null_mut();
        }
    }

    elf_init_reloc(elf, (*sec).rsec, reloc_idx, offset, sym, addend, elf_text_rela_type(elf))
}

#[no_mangle]
pub unsafe extern "C" fn elf_init_reloc_data_sym(elf: *mut elf, sec: *mut section, offset: c_ulong, reloc_idx: c_uint, sym: *mut symbol, addend: s64) -> *mut reloc {
    if is_text_sec(sec) {
        ERROR(b"bad call to %s() for text symbol %s\0".as_ptr() as *const c_char, b"elf_init_reloc_data_sym\0".as_ptr() as *const c_char, (*sym).name);
        return ptr::null_mut();
    }

    elf_init_reloc(elf, (*sec).rsec, reloc_idx, offset, sym, addend, elf_data_rela_type(elf))
}

unsafe fn read_relocs(elf: *mut elf) -> c_int {
    let mut max_reloc: c_ulong = 0;

    if elf_alloc_hash(&mut (*elf).reloc_table, &mut (*elf).reloc_bits, (*elf).num_relocs as size_t, b"mmap fail reloc\0".as_ptr() as *const c_char).is_null() {
        return -1;
    }

    let mut rsec = (*elf).sections.next as *mut section;
    while rsec != (&mut (*elf).sections as *mut list_head as *mut section) {
        if !is_reloc_sec(rsec) {
            rsec = (*rsec).list.next as *mut section;
            continue;
        }

        (*rsec).base = find_section_by_index(elf, (*rsec).sh.sh_info);
        if (*rsec).base.is_null() {
            ERROR(b"can't find base section for reloc section %s\0".as_ptr() as *const c_char, (*rsec).name);
            return -1;
        }

        (*(*rsec).base).rsec = rsec;

        /* nr_alloc_relocs=0: libelf owns d_buf */
        (*rsec).nr_alloc_relocs = 0;

        (*rsec).relocs = calloc(sec_num_entries(rsec) as size_t, size_of::<reloc>()) as *mut reloc;
        if (*rsec).relocs.is_null() {
            ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
            return -1;
        }

        let mut nr_reloc: c_ulong = 0;
        let mut i = 0;
        while i < sec_num_entries(rsec) {
            let reloc = (*rsec).relocs.add(i as usize);

            (*reloc).sec = rsec;
            let symndx = reloc_sym(reloc);
            let sym = find_symbol_by_index(elf, symndx);
            (*reloc).sym = sym;
            if (*reloc).sym.is_null() {
                ERROR(b"can't find reloc entry symbol %d for %s\0".as_ptr() as *const c_char, symndx, (*rsec).name);
                return -1;
            }

            elf_hash_add((*elf).reloc_table, (*elf).reloc_bits, &mut (*reloc).hash, reloc_hash(reloc) as c_ulong);
            set_sym_next_reloc(reloc, (*sym).relocs);
            (*sym).relocs = reloc;

            nr_reloc += 1;
            i += 1;
        }
        max_reloc = max_ulong(max_reloc, nr_reloc);
        rsec = (*rsec).list.next as *mut section;
    }

    if opts.stats {
        printf(b"max_reloc: %lu\n\0".as_ptr() as *const c_char, max_reloc);
        printf(b"num_relocs: %lu\n\0".as_ptr() as *const c_char, (*elf).num_relocs);
        printf(b"reloc_bits: %d\n\0".as_ptr() as *const c_char, (*elf).reloc_bits);
    }

    0
}

unsafe fn mark_rodata(elf: *mut elf) {
    let mut sec = (*elf).sections.next as *mut section;
    while sec != (&mut (*elf).sections as *mut list_head as *mut section) {
        if (strstarts((*sec).name, b".rodata\0".as_ptr() as *const c_char) && strstr((*sec).name, b".str1.\0".as_ptr() as *const c_char).is_null())
            || strstarts((*sec).name, b".data.rel.ro\0".as_ptr() as *const c_char) {
            (*sec).rodata = true;
        }
        sec = (*sec).list.next as *mut section;
    }
}

#[no_mangle]
pub unsafe extern "C" fn elf_open_read(name: *const c_char, flags: c_int) -> *mut elf {
    elf_version(EV_CURRENT);

    let elf = malloc(size_of::<elf>()) as *mut elf;
    if elf.is_null() {
        ERROR_GLIBC(b"malloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }
    memset(elf as *mut c_void, 0, size_of::<elf>());

    INIT_LIST_HEAD(&mut (*elf).sections);

    (*elf).fd = open(name, flags);
    if (*elf).fd == -1 {
        fprintf(stderr, b"objtool: Can't open '%s': %s\n\0".as_ptr() as *const c_char, name, strerror(errno));
        elf_close(elf);
        return ptr::null_mut();
    }

    (*elf).name = strdup(name);
    if (*elf).name.is_null() {
        ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    let cmd: Elf_Cmd = if (flags & O_ACCMODE) == O_RDONLY {
        ELF_C_READ_MMAP
    } else if (flags & O_ACCMODE) == O_RDWR {
        ELF_C_RDWR
    } else {
        ELF_C_WRITE
    };

    (*elf).elf = elf_begin((*elf).fd, cmd, ptr::null_mut());
    if (*elf).elf.is_null() {
        ERROR_ELF(b"elf_begin\0".as_ptr() as *const c_char);
        elf_close(elf);
        return ptr::null_mut();
    }

    if gelf_getehdr((*elf).elf, &mut (*elf).ehdr).is_null() {
        ERROR_ELF(b"gelf_getehdr\0".as_ptr() as *const c_char);
        elf_close(elf);
        return ptr::null_mut();
    }

    if read_sections(elf) != 0 {
        elf_close(elf);
        return ptr::null_mut();
    }

    mark_rodata(elf);

    if read_symbols(elf) != 0 {
        elf_close(elf);
        return ptr::null_mut();
    }

    if mark_group_syms(elf) != 0 {
        elf_close(elf);
        return ptr::null_mut();
    }

    if read_relocs(elf) != 0 {
        elf_close(elf);
        return ptr::null_mut();
    }

    elf
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_file(ehdr: *mut GElf_Ehdr, name: *const c_char) -> *mut elf {
    elf_version(EV_CURRENT);

    let elf = calloc(1, size_of::<elf>()) as *mut elf;
    if elf.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*elf).sections);

    let tmp_name = malloc(strlen(name) + 8) as *mut c_char;
    if tmp_name.is_null() {
        ERROR_GLIBC(b"malloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    sprintf(tmp_name, b"%s.XXXXXX\0".as_ptr() as *const c_char, name);

    (*elf).fd = mkstemp(tmp_name);
    if (*elf).fd == -1 {
        ERROR_GLIBC(b"can't create tmp file\0".as_ptr() as *const c_char);
        exit(1);
    }

    (*elf).tmp_name = tmp_name;

    (*elf).name = strdup(name);
    if (*elf).name.is_null() {
        ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*elf).elf = elf_begin((*elf).fd, ELF_C_WRITE, ptr::null_mut());
    if (*elf).elf.is_null() {
        ERROR_ELF(b"elf_begin\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    if gelf_newehdr((*elf).elf, ELFCLASS64).is_null() {
        ERROR_ELF(b"gelf_newehdr\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    memcpy(&mut (*elf).ehdr as *mut _ as *mut c_void, ehdr as *const c_void, size_of::<GElf_Ehdr>());

    if gelf_update_ehdr((*elf).elf, &mut (*elf).ehdr) == 0 {
        ERROR_ELF(b"gelf_update_ehdr\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*elf).symbols);

    if elf_alloc_hash(&mut (*elf).section_table, &mut (*elf).section_bits, 1000, b"mmap fail section\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).section_name_table, &mut (*elf).section_name_bits, 1000, b"mmap fail section_name\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).symbol_table, &mut (*elf).symbol_bits, 10000, b"mmap fail symbol\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).symbol_name_table, &mut (*elf).symbol_name_bits, 10000, b"mmap fail symbol_name\0".as_ptr() as *const c_char).is_null()
        || elf_alloc_hash(&mut (*elf).reloc_table, &mut (*elf).reloc_bits, 100000, b"mmap fail reloc\0".as_ptr() as *const c_char).is_null() {
        return ptr::null_mut();
    }

    let null = elf_create_section(elf, ptr::null(), 0, 0, SHT_NULL, 0, 0);
    let shstrtab = elf_create_section(elf, ptr::null(), 0, 0, SHT_STRTAB, 1, 0);
    let strtab = elf_create_section(elf, ptr::null(), 0, 0, SHT_STRTAB, 1, 0);

    if null.is_null() || shstrtab.is_null() || strtab.is_null() {
        return ptr::null_mut();
    }

    (*null).name = b"\0".as_ptr() as *mut c_char;
    (*shstrtab).name = b".shstrtab\0".as_ptr() as *mut c_char;
    (*strtab).name = b".strtab\0".as_ptr() as *mut c_char;

    (*null).sh.sh_name = elf_add_string(elf, shstrtab, (*null).name);
    (*shstrtab).sh.sh_name = elf_add_string(elf, shstrtab, (*shstrtab).name);
    (*strtab).sh.sh_name = elf_add_string(elf, shstrtab, (*strtab).name);

    if (*null).sh.sh_name == u32::MAX || (*shstrtab).sh.sh_name == u32::MAX || (*strtab).sh.sh_name == u32::MAX {
        return ptr::null_mut();
    }

    elf_hash_add((*elf).section_name_table, (*elf).section_name_bits, &mut (*null).name_hash, str_hash((*null).name) as c_ulong);
    elf_hash_add((*elf).section_name_table, (*elf).section_name_bits, &mut (*strtab).name_hash, str_hash((*strtab).name) as c_ulong);
    elf_hash_add((*elf).section_name_table, (*elf).section_name_bits, &mut (*shstrtab).name_hash, str_hash((*shstrtab).name) as c_ulong);

    if elf_add_string(elf, strtab, b"\0".as_ptr() as *const c_char) == u32::MAX {
        return ptr::null_mut();
    }

    let symtab = elf_create_section(elf, b".symtab\0".as_ptr() as *const c_char, 0x18, 0x18, SHT_SYMTAB, 0x8, 0);
    if symtab.is_null() {
        return ptr::null_mut();
    }

    (*symtab).sh.sh_link = (*strtab).idx;
    (*symtab).sh.sh_info = 1;

    (*elf).ehdr.e_shstrndx = (*shstrtab).idx;
    if gelf_update_ehdr((*elf).elf, &mut (*elf).ehdr) == 0 {
        ERROR_ELF(b"gelf_update_ehdr\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    let sym = calloc(1, size_of::<symbol>()) as *mut symbol;
    if sym.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*sym).name = b"\0".as_ptr() as *mut c_char;
    (*sym).sec = null;
    elf_add_symbol(elf, sym);

    elf
}

#[no_mangle]
pub unsafe extern "C" fn elf_add_string(elf: *mut elf, mut strtab: *mut section, str_: *const c_char) -> c_uint {
    if strtab.is_null() {
        strtab = find_section_by_name(elf, b".strtab\0".as_ptr() as *const c_char);
    }
    if strtab.is_null() {
        ERROR(b"can't find .strtab section\0".as_ptr() as *const c_char);
        return u32::MAX;
    }

    if (*strtab).sh.sh_addralign == 0 {
        ERROR(b"'%s': invalid sh_addralign\0".as_ptr() as *const c_char, (*strtab).name);
        return u32::MAX;
    }

    let offset = ALIGN(sec_size(strtab), (*strtab).sh.sh_addralign);

    if elf_add_data(elf, strtab, str_ as *const c_void, strlen(str_) + 1).is_null() {
        return u32::MAX;
    }

    offset as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn elf_add_data(elf: *mut elf, sec: *mut section, data: *const c_void, size: size_t) -> *mut c_void {
    if (*sec).sh.sh_addralign == 0 {
        ERROR(b"'%s': invalid sh_addralign\0".as_ptr() as *const c_char, (*sec).name);
        return ptr::null_mut();
    }

    let s = elf_getscn((*elf).elf, (*sec).idx as size_t);
    if s.is_null() {
        ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*sec).data = elf_newdata(s);
    if (*sec).data.is_null() {
        ERROR_ELF(b"elf_newdata\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*(*sec).data).d_buf = calloc(1, size);
    if (*(*sec).data).d_buf.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    if !data.is_null() {
        memcpy((*(*sec).data).d_buf, data, size);
    }

    (*(*sec).data).d_size = size;
    (*(*sec).data).d_align = (*sec).sh.sh_addralign as size_t;

    let offset = ALIGN(sec_size(sec), (*sec).sh.sh_addralign);
    (*sec).sh.sh_size = offset + size as u64;

    mark_sec_changed(elf, sec, true);

    (*(*sec).data).d_buf
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_section(elf: *mut elf, name: *const c_char, size: size_t, entsize: size_t, type_: c_uint, align: c_uint, flags: c_uint) -> *mut section {
    if !name.is_null() && !find_section_by_name(elf, name).is_null() {
        ERROR(b"section '%s' already exists\0".as_ptr() as *const c_char, name);
        return ptr::null_mut();
    }

    let sec = calloc(1, size_of::<section>()) as *mut section;
    if sec.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*sec).symbol_list);

    /* don't actually create the section, just the data structures */
    if type_ != SHT_NULL {
        let s = elf_newscn((*elf).elf);
        if s.is_null() {
            ERROR_ELF(b"elf_newscn\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }

        (*sec).idx = elf_ndxscn(s) as c_uint;

        if size != 0 {
            (*sec).data = elf_newdata(s);
            if (*sec).data.is_null() {
                ERROR_ELF(b"elf_newdata\0".as_ptr() as *const c_char);
                return ptr::null_mut();
            }

            (*(*sec).data).d_size = size;
            (*(*sec).data).d_align = 1;

            (*(*sec).data).d_buf = calloc(1, size);
            if (*(*sec).data).d_buf.is_null() {
                ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
                return ptr::null_mut();
            }
        }

        if gelf_getshdr(s, &mut (*sec).sh).is_null() {
            ERROR_ELF(b"gelf_getshdr\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }

        (*sec).sh.sh_size = size as u64;
        (*sec).sh.sh_entsize = entsize as u64;
        (*sec).sh.sh_type = type_;
        (*sec).sh.sh_addralign = align as u64;
        (*sec).sh.sh_flags = flags as u64;

        if !name.is_null() {
            (*sec).name = strdup(name);
            if (*sec).name.is_null() {
                ERROR(b"strdup\0".as_ptr() as *const c_char);
                return ptr::null_mut();
            }

            /* Add section name to .shstrtab (or .strtab for Clang) */
            let mut shstrtab = find_section_by_name(elf, b".shstrtab\0".as_ptr() as *const c_char);
            if shstrtab.is_null() {
                shstrtab = find_section_by_name(elf, b".strtab\0".as_ptr() as *const c_char);
                if shstrtab.is_null() {
                    ERROR(b"can't find .shstrtab or .strtab\0".as_ptr() as *const c_char);
                    return ptr::null_mut();
                }
            }
            (*sec).sh.sh_name = elf_add_string(elf, shstrtab, (*sec).name);
            if (*sec).sh.sh_name == u32::MAX {
                return ptr::null_mut();
            }

            elf_hash_add((*elf).section_name_table, (*elf).section_name_bits, &mut (*sec).name_hash, str_hash((*sec).name) as c_ulong);
        }
    }

    list_add_tail(&mut (*sec).list, &mut (*elf).sections);
    elf_hash_add((*elf).section_table, (*elf).section_bits, &mut (*sec).hash, (*sec).idx as c_ulong);

    mark_sec_changed(elf, sec, true);

    sec
}

unsafe fn elf_alloc_reloc(elf: *mut elf, rsec: *mut section) -> c_int {
    let nr_relocs_old = sec_num_entries(rsec);
    let nr_relocs_new = nr_relocs_old + 1;

    if (*rsec).data.is_null() {
        (*rsec).data = elf_newdata(elf_getscn((*elf).elf, (*rsec).idx as size_t));
        if (*rsec).data.is_null() {
            ERROR_ELF(b"elf_newdata\0".as_ptr() as *const c_char);
            return -1;
        }

        (*(*rsec).data).d_align = 1;
        (*(*rsec).data).d_type = ELF_T_RELA;
        (*(*rsec).data).d_buf = ptr::null_mut();
    }

    (*(*rsec).data).d_size = nr_relocs_new as size_t * elf_rela_size(elf) as size_t;
    (*rsec).sh.sh_size = (*(*rsec).data).d_size as u64;

    let nr_alloc = max_ulong(64, roundup_pow_of_two(nr_relocs_new as c_ulong));
    if nr_alloc <= (*rsec).nr_alloc_relocs {
        return 0;
    }

    if !(*(*rsec).data).d_buf.is_null() && (*rsec).nr_alloc_relocs == 0 {
        let orig_buf = (*(*rsec).data).d_buf;

        /*
         * The original d_buf is owned by libelf so it can't be
         * realloced.
         */
        (*(*rsec).data).d_buf = malloc(nr_alloc as size_t * elf_rela_size(elf) as size_t);
        if (*(*rsec).data).d_buf.is_null() {
            ERROR_GLIBC(b"malloc\0".as_ptr() as *const c_char);
            return -1;
        }
        memcpy((*(*rsec).data).d_buf, orig_buf, nr_relocs_old as size_t * elf_rela_size(elf) as size_t);
    } else {
        (*(*rsec).data).d_buf = realloc((*(*rsec).data).d_buf, nr_alloc as size_t * elf_rela_size(elf) as size_t);
        if (*(*rsec).data).d_buf.is_null() {
            ERROR_GLIBC(b"realloc\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    (*rsec).nr_alloc_relocs = nr_alloc;

    let old_relocs = (*rsec).relocs;
    let new_relocs = calloc(nr_alloc as size_t, size_of::<reloc>()) as *mut reloc;
    if new_relocs.is_null() {
        ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
        return -1;
    }

    if !old_relocs.is_null() {
        /*
         * The struct reloc's address has changed.  Update all the symbols and
         * relocs which reference it.
         */
        let old_relocs_end = old_relocs.add(nr_relocs_old as usize);
        let mut sym = (*elf).symbols.next as *mut symbol;
        while sym != (&mut (*elf).symbols as *mut list_head as *mut symbol) {
            let mut reloc = (*sym).relocs;
            if !reloc.is_null() {
                if reloc >= old_relocs && reloc < old_relocs_end {
                    (*sym).relocs = new_relocs.add(reloc.offset_from(old_relocs) as usize);
                }

                loop {
                    let next_reloc = sym_next_reloc(reloc);

                    if next_reloc.is_null() {
                        break;
                    }

                    if next_reloc >= old_relocs && next_reloc < old_relocs_end {
                        set_sym_next_reloc(reloc, new_relocs.add(next_reloc.offset_from(old_relocs) as usize));
                    }

                    reloc = next_reloc;
                }
            }
            sym = (*sym).global_list.next as *mut symbol;
        }

        memcpy(new_relocs as *mut c_void, old_relocs as *const c_void, nr_relocs_old as size_t * size_of::<reloc>());

        let mut i = 0;
        while i < nr_relocs_old {
            let old = old_relocs.add(i as usize);
            let new = new_relocs.add(i as usize);
            let key = reloc_hash(old);

            __elf_hash_del(&mut (*old).hash, table_entry((*elf).reloc_table, (*elf).reloc_bits, key as c_ulong));
            elf_hash_add((*elf).reloc_table, (*elf).reloc_bits, &mut (*new).hash, key as c_ulong);
            i += 1;
        }

        free(old_relocs as *mut c_void);
    }
    (*rsec).relocs = new_relocs;
    0
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_rela_section(elf: *mut elf, sec: *mut section, nr_relocs: c_uint) -> *mut section {
    let rsec_name = malloc(strlen((*sec).name) + strlen(b".rela\0".as_ptr() as *const c_char) + 1) as *mut c_char;
    if rsec_name.is_null() {
        ERROR_GLIBC(b"malloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }
    strcpy(rsec_name, b".rela\0".as_ptr() as *const c_char);
    strcat(rsec_name, (*sec).name);

    let rsec = elf_create_section(elf, rsec_name, nr_relocs as size_t * elf_rela_size(elf) as size_t, elf_rela_size(elf) as size_t, SHT_RELA, elf_addr_size(elf), SHF_INFO_LINK);
    free(rsec_name as *mut c_void);
    if rsec.is_null() {
        return ptr::null_mut();
    }

    if nr_relocs != 0 {
        (*(*rsec).data).d_type = ELF_T_RELA;

        (*rsec).nr_alloc_relocs = nr_relocs as c_ulong;
        (*rsec).relocs = calloc(nr_relocs as size_t, size_of::<reloc>()) as *mut reloc;
        if (*rsec).relocs.is_null() {
            ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }
    }

    (*rsec).sh.sh_link = (*find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char)).idx;
    (*rsec).sh.sh_info = (*sec).idx;

    (*sec).rsec = rsec;
    (*rsec).base = sec;

    rsec
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_reloc(elf: *mut elf, sec: *mut section, offset: c_ulong, sym: *mut symbol, addend: s64, type_: c_uint) -> *mut reloc {
    let mut rsec = (*sec).rsec;

    if rsec.is_null() {
        rsec = elf_create_rela_section(elf, sec, 0);
        if rsec.is_null() {
            return ptr::null_mut();
        }
    }

    if !find_reloc_by_dest(elf, sec, offset).is_null() {
        ERROR_FUNC(sec, offset, b"duplicate reloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    if elf_alloc_reloc(elf, rsec) != 0 {
        return ptr::null_mut();
    }

    mark_sec_changed(elf, rsec, true);

    elf_init_reloc(elf, rsec, sec_num_entries(rsec) - 1, offset, sym, addend, type_)
}

#[no_mangle]
pub unsafe extern "C" fn elf_create_section_pair(elf: *mut elf, name: *const c_char, entsize: size_t, nr: c_uint, nr_relocs: c_uint) -> *mut section {
    let sec = elf_create_section(elf, name, nr as size_t * entsize, entsize, SHT_PROGBITS, 1, SHF_ALLOC);
    if sec.is_null() {
        return ptr::null_mut();
    }

    if elf_create_rela_section(elf, sec, nr_relocs).is_null() {
        return ptr::null_mut();
    }

    sec
}

#[no_mangle]
pub unsafe extern "C" fn elf_write_insn(elf: *mut elf, sec: *mut section, offset: c_ulong, len: c_uint, insn: *const c_char) -> c_int {
    let data = (*sec).data;

    if (*data).d_type != ELF_T_BYTE || (*data).d_off != 0 {
        ERROR(b"write to unexpected data for section: %s\0".as_ptr() as *const c_char, (*sec).name);
        return -1;
    }

    memcpy(((*data).d_buf as *mut u8).add(offset as usize) as *mut c_void, insn as *const c_void, len as size_t);

    mark_sec_changed(elf, sec, true);

    0
}

/*
 * When Elf_Scn::sh_size is smaller than the combined Elf_Data::d_size
 * do you:
 *
 *   A) adhere to the section header and truncate the data, or
 *   B) ignore the section header and write out all the data you've got?
 *
 * Yes, libelf sucks and we need to manually truncate if we over-allocate data.
 */
unsafe fn elf_truncate_section(elf: *mut elf, sec: *mut section) -> c_int {
    let mut size = sec_size(sec);
    let mut truncated = false;
    let mut data: *mut Elf_Data = ptr::null_mut();

    let s = elf_getscn((*elf).elf, (*sec).idx as size_t);
    if s.is_null() {
        ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
        return -1;
    }

    loop {
        /* get next data descriptor for the relevant section */
        data = elf_getdata(s, data);
        if data.is_null() {
            if size != 0 {
                ERROR(b"end of section data but non-zero size left\n\0".as_ptr() as *const c_char);
                return -1;
            }
            return 0;
        }

        if truncated {
            /* when we remove symbols */
            ERROR(b"truncated; but more data\n\0".as_ptr() as *const c_char);
            return -1;
        }

        if (*data).d_size == 0 {
            ERROR(b"zero size data\0".as_ptr() as *const c_char);
            return -1;
        }

        if (*data).d_size as u64 > size {
            truncated = true;
            (*data).d_size = size as size_t;
        }

        size -= (*data).d_size as u64;
    }
}

#[no_mangle]
pub unsafe extern "C" fn elf_write(elf: *mut elf) -> c_int {
    let mut sec = (*elf).sections.next as *mut section;

    /* Update changed relocation sections and section headers: */
    while sec != (&mut (*elf).sections as *mut list_head as *mut section) {
        if (*sec).truncate && elf_truncate_section(elf, sec) != 0 {
            return -1;
        }

        if sec_changed(sec) {
            let s = elf_getscn((*elf).elf, (*sec).idx as size_t);
            if s.is_null() {
                ERROR_ELF(b"elf_getscn\0".as_ptr() as *const c_char);
                return -1;
            }

            /* Note this also flags the section dirty */
            if gelf_update_shdr(s, &mut (*sec).sh) == 0 {
                ERROR_ELF(b"gelf_update_shdr\0".as_ptr() as *const c_char);
                return -1;
            }

            mark_sec_changed(elf, sec, false);
        }
        sec = (*sec).list.next as *mut section;
    }

    /* Make sure the new section header entries get updated properly. */
    elf_flagelf((*elf).elf, ELF_C_SET, ELF_F_DIRTY);

    /* Write all changes to the file. */
    if elf_update((*elf).elf, ELF_C_WRITE) < 0 {
        ERROR_ELF(b"elf_update\0".as_ptr() as *const c_char);
        return -1;
    }

    (*elf).changed = false;

    0
}

#[no_mangle]
pub unsafe extern "C" fn elf_close(elf: *mut elf) -> c_int {
    if !(*elf).elf.is_null() {
        elf_end((*elf).elf);
    }

    if (*elf).fd > 0 {
        close((*elf).fd);
    }

    if !(*elf).tmp_name.is_null() && rename((*elf).tmp_name, (*elf).name) != 0 {
        return -1;
    }

    /*
     * NOTE: All remaining allocations are leaked on purpose.  Objtool is
     * about to exit anyway.
     */
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
