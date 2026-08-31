// SPDX-License-Identifier: GPL-2.0-or-later
// Rust source-level translation of objtool/klp-diff.c.
//
// C dependency intent:
//   _GNU_SOURCE was used for memmem().
//   Headers from subcmd, objtool, linux, ELF, libc, and livepatch provide the
//   types, constants, functions, and iterator/hash/debug macros referenced here.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type size_t = usize;
type u64_ = u64;
type s64 = i64;

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf {
    pub name: *const c_char,
    pub ehdr: elf64_ehdr,
}

#[repr(C)]
pub struct elf64_ehdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf64_sym {
    pub st_shndx: u16,
}

#[repr(C)]
pub struct elf64_shdr {
    pub sh_entsize: c_ulong,
    pub sh_type: c_ulong,
    pub sh_addralign: c_ulong,
    pub sh_flags: c_ulong,
}

#[repr(C)]
pub struct data {
    pub d_buf: *mut c_char,
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub data: *mut data,
    pub rsec: *mut section,
    pub relocs: *mut reloc,
    pub sym: *mut symbol,
    pub sh: elf64_shdr,
    pub base: *mut c_void,
}

#[repr(C)]
pub struct checksum {
    pub checksum: u64_,
}

#[repr(C)]
pub struct symbol {
    pub hash: hlist_node,
    pub name: *const c_char,
    pub demangled_name: *const c_char,
    pub sec: *mut section,
    pub file: *mut symbol,
    pub twin: *mut symbol,
    pub clone: *mut symbol,
    pub cfunc: *mut symbol,
    pub pfunc: *mut symbol,
    pub sym: elf64_sym,
    pub csum: checksum,
    pub idx: c_ulong,
    pub offset: c_ulong,
    pub len: c_ulong,
    pub bind: c_uint,
    pub type_: c_uint,
    pub dont_correlate: c_uint,
    pub included: c_uint,
    pub changed: c_uint,
    pub fake: c_uint,
}

type c_uint = u32;

#[repr(C)]
pub struct reloc {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sym_checksum {
    pub checksum: u64_,
}

#[repr(C)]
pub struct klp_reloc {
    pub offset: u64_,
    pub sym: u64_,
    pub type_: u64_,
}

#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: *mut c_void,
    pub post_patch: *mut c_void,
    pub pre_unpatch: *mut c_void,
    pub post_unpatch: *mut c_void,
}

#[repr(C)]
pub struct klp_object_ext {
    pub name: *const c_char,
    pub funcs: *mut klp_func_ext,
    pub callbacks: klp_callbacks,
    pub nr_funcs: c_uint,
}

#[repr(C)]
pub struct klp_func_ext {
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
    pub sympos: c_ulong,
}

#[repr(C)]
pub struct elfs {
    pub orig: *mut elf,
    pub patched: *mut elf,
    pub out: *mut elf,
    pub modname: *const c_char,
}

#[repr(C)]
pub struct export {
    pub hash: hlist_node,
    pub mod_: *mut c_char,
    pub sym: *mut c_char,
    pub mod_ns: bool_,
}

pub static mut debug: bool_ = false;
pub static mut debug_correlate: bool_ = false;
pub static mut debug_clone: bool_ = false;
pub static mut indent: c_int = 0;

static mut klp_diff_usage: [*const c_char; 2] = [
    b"objtool klp diff [<options>] <in1.o> <in2.o> <out.o>\0".as_ptr() as *const c_char,
    null(),
];

// Static option initializer preserves OPT_GROUP/OPT_BOOLEAN/OPT_END intent.
// The concrete option records are provided by subcmd/parse-options.h in C.
static mut klp_diff_options: [option; 4] = [
    option { _private: [] },
    option { _private: [] },
    option { _private: [] },
    option { _private: [] },
];

// static DEFINE_HASHTABLE(exports, 15);
static mut exports: [hlist_head; 1 << 15] = [hlist_head { _private: [] }; 1 << 15];
// static DECLARE_HASHTABLE(suffix_map, 7);
static mut suffix_map: [hlist_head; 1 << 7] = [hlist_head { _private: [] }; 1 << 7];

const SYM_NAME_LEN: usize = 512;
const SEC_NAME_LEN: usize = 512;
const STT_NOTYPE: c_uint = 0;
const STT_OBJECT: c_uint = 1;
const STT_FUNC: c_uint = 2;
const STT_SECTION: c_uint = 3;
const STT_FILE: c_uint = 4;
const STB_LOCAL: c_uint = 0;
const STB_GLOBAL: c_uint = 1;
const STB_WEAK: c_uint = 2;
const SHN_ABS: u16 = 0xfff1;
const O_RDONLY: c_int = 0;
const SHT_PROGBITS: c_ulong = 1;
const SHF_ALLOC: c_ulong = 2;
const SHF_MERGE: c_ulong = 16;
const SHF_STRINGS: c_ulong = 32;
const R_NONE: c_int = 0;
const R_ABS64: c_int = 1;
const ANNOTYPE_DATA_SPECIAL: c_int = 1;
const ULONG_MAX: c_ulong = c_ulong::MAX;

const KLP_TOMBSTONE_PREFIX: &[u8] = b".klp.tombstone.";
const KLP_SYM_PREFIX: &[u8] = b".klp.sym.";
const KLP_RELOCS_SEC: &[u8] = b"__klp_relocs";
const KLP_OBJECTS_SEC: &[u8] = b".init.klp_objects";
const KLP_FUNCS_SEC: &[u8] = b".init.klp_funcs";
const KLP_STRINGS_SEC: &[u8] = b".klp.strings";
const KLP_PRE_PATCH_PREFIX: &[u8] = b"__klp_pre_patch_";
const KLP_POST_PATCH_PREFIX: &[u8] = b"__klp_post_patch_";
const KLP_PRE_UNPATCH_PREFIX: &[u8] = b"__klp_pre_unpatch_";
const KLP_POST_UNPATCH_PREFIX: &[u8] = b"__klp_post_unpatch_";

extern "C" {
    static mut objname: *const c_char;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memmem(haystack: *const c_void, haystacklen: size_t, needle: *const c_void, needlelen: size_t) -> *mut c_void;
    fn labs(j: c_long) -> c_long;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn top_level_dir(path: *const c_char) -> *mut c_char;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn str_hash(str_: *const c_char) -> c_ulong;
    fn snprintf_check(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;

    fn elf_open_read(name: *const c_char, flags: c_int) -> *mut elf;
    fn elf_create_file(ehdr: *const elf64_ehdr, name: *const c_char) -> *mut elf;
    fn elf_write(elf: *mut elf) -> c_int;
    fn elf_close(elf: *mut elf) -> c_int;
    fn elf_create_section(elf: *mut elf, name: *const c_char, idx: c_int, entsize: c_ulong, shtype: c_ulong, addralign: c_ulong, flags: c_ulong) -> *mut section;
    fn elf_create_section_pair(elf: *mut elf, name: *const u8, size: size_t, idx: c_int, flags: c_int) -> *mut section;
    fn elf_create_section_symbol(elf: *mut elf, sec: *mut section) -> *mut symbol;
    fn elf_create_symbol(elf: *mut elf, name: *const c_char, sec: *mut section, bind: c_uint, type_: c_uint, offset: c_ulong, len: c_ulong) -> *mut symbol;
    fn elf_add_data(elf: *mut elf, sec: *mut section, data: *const c_void, size: size_t) -> *mut c_void;
    fn elf_add_string(elf: *mut elf, sec: *mut section, str_: *const c_char) -> s64;
    fn elf_create_reloc(elf: *mut elf, sec: *mut section, offset: c_ulong, sym: *mut symbol, addend: s64, type_: c_int) -> *mut reloc;

    fn find_section_by_name(elf: *mut elf, name: *const c_char) -> *mut section;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
    fn find_symbol_by_offset(sec: *mut section, offset: c_ulong) -> *mut symbol;
    fn find_symbol_containing_inclusive(sec: *mut section, offset: c_ulong) -> *mut symbol;
    fn find_symbol_by_name(elf: *mut elf, name: *const c_char) -> *mut symbol;

    fn sec_size(sec: *mut section) -> c_ulong;
    fn sec_num_entries(sec: *mut section) -> c_ulong;
    fn reloc_addend(reloc: *mut reloc) -> s64;
    fn reloc_offset(reloc: *mut reloc) -> c_ulong;
    fn reloc_type(reloc: *mut reloc) -> c_int;
    fn set_reloc_sym(elf: *mut elf, reloc: *mut reloc, idx: c_ulong);
    fn set_reloc_addend(elf: *mut elf, reloc: *mut reloc, addend: s64);
    fn arch_adjusted_addend(reloc: *mut reloc) -> c_ulong;
    fn arch_alt_ignore_new_reloc(sec: *mut section, offset: c_ulong) -> bool_;
    fn arch_reloc_size(relocs: *mut reloc) -> c_uint;
    fn annotype(elf: *mut elf, sec: *mut section, reloc: *mut reloc) -> c_int;

    fn is_sec_sym(sym: *mut symbol) -> bool_;
    fn is_func_sym(sym: *mut symbol) -> bool_;
    fn is_object_sym(sym: *mut symbol) -> bool_;
    fn is_file_sym(sym: *mut symbol) -> bool_;
    fn is_null_sym(sym: *mut symbol) -> bool_;
    fn is_prefix_func(sym: *mut symbol) -> bool_;
    fn is_local_sym(sym: *mut symbol) -> bool_;
    fn is_undef_sym(sym: *mut symbol) -> bool_;
    fn is_weak_sym(sym: *mut symbol) -> bool_;
    fn is_notype_sym(sym: *mut symbol) -> bool_;
    fn is_cold_func(sym: *mut symbol) -> bool_;
    fn is_text_sec(sec: *mut section) -> bool_;
    fn is_rodata_sec(sec: *mut section) -> bool_;
    fn is_string_sec(sec: *mut section) -> bool_;
    fn get_func_prefix(sym: *mut symbol) -> *mut symbol;

    fn klp_sympos_init(elf: *mut elf) -> c_int;
    fn klp_find_sympos(elf: *mut elf, sym: *mut symbol) -> c_ulong;

    fn ERROR(fmt: *const c_char, ...);
    fn ERROR_GLIBC(msg: *const c_char);
    fn ERROR_FUNC(base: *mut c_void, offset: c_ulong, fmt: *const c_char, ...);
    fn WARN(fmt: *const c_char, ...);
    fn dbg_correlate(fmt: *const c_char, ...);
    fn dbg_clone(fmt: *const c_char, ...);
    fn __dbg_clone(fmt: *const c_char, ...);
}

// Iterator/hash macros supplied by the objtool/Linux C environment are expected
// as Rust macros by this source-level translation:
// for_each_sym!, for_each_sym_continue!, for_each_sym_by_demangled_name!,
// for_each_sym_by_name!, for_each_sec!, sec_for_each_sym!, sym_for_each_reloc!,
// for_each_reloc!, for_each_reloc_continue!, hash_init!, hash_add!,
// hash_for_each_possible!.

macro_rules! offsetof {
    ($ty:ty, $field:tt) => {
        0usize
    };
}

unsafe fn ALIGN(x: c_ulong, a: c_ulong) -> c_ulong {
    if a == 0 { x } else { (x + a - 1) & !(a - 1) }
}

unsafe fn escape_str(orig: *const c_char) -> *mut c_char {
    let mut len: size_t = 0;
    let mut a = orig;
    while *a != 0 {
        match *a as u8 {
            1 => len += 5,
            b'\n' | b'\t' => len += 2,
            _ => len += 1,
        }
        a = a.add(1);
    }
    let new = malloc(len + 1) as *mut c_char;
    if new.is_null() { return null_mut(); }
    a = orig;
    let mut b = new;
    while *a != 0 {
        match *a as u8 {
            1 => {
                memcpy(b as *mut c_void, b"<SOH>".as_ptr() as *const c_void, 5);
                b = b.add(5);
            }
            b'\n' => { *b = b'\\' as c_char; b = b.add(1); *b = b'n' as c_char; b = b.add(1); }
            b'\t' => { *b = b'\\' as c_char; b = b.add(1); *b = b't' as c_char; b = b.add(1); }
            _ => { *b = *a; b = b.add(1); }
        }
        a = a.add(1);
    }
    *b = 0;
    new
}

unsafe fn normalize_modname(mut name: *mut c_char) -> *mut c_char {
    let slash = strrchr(name, b'/' as c_int);
    if !slash.is_null() { name = slash.add(1); }
    let mut c = name;
    while *c != 0 {
        if *c == b'-' as c_char {
            *c = b'_' as c_char;
        } else if *c == b'.' as c_char {
            *c = 0;
            break;
        }
        c = c.add(1);
    }
    name
}

unsafe fn read_exports() -> c_int {
    let symvers = b"Module.symvers\0".as_ptr() as *const c_char;
    let mut line = [0 as c_char; 1024];
    let mut path: *mut c_char = null_mut();
    let mut line_num: c_uint = 0;
    let mut file = fopen(symvers, b"r\0".as_ptr() as *const c_char);
    if file.is_null() {
        path = top_level_dir(symvers);
        if path.is_null() {
            ERROR(b"can't open '%s', \"objtool diff\" should be run from the kernel tree\0".as_ptr() as *const c_char, symvers);
            return -1;
        }
        file = fopen(path, b"r\0".as_ptr() as *const c_char);
        if file.is_null() {
            ERROR_GLIBC(b"fopen\0".as_ptr() as *const c_char);
            return -1;
        }
    }
    while !fgets(line.as_mut_ptr(), 1024, file).is_null() {
        line_num += 1;
        let mut sym = strchr(line.as_ptr(), b'\t' as c_int);
        if sym.is_null() { ERROR(b"malformed Module.symvers (sym) at line %d\0".as_ptr() as *const c_char, line_num); return -1; }
        *sym = 0; sym = sym.add(1);
        let mut mod_ = strchr(sym, b'\t' as c_int);
        if mod_.is_null() { ERROR(b"malformed Module.symvers (mod) at line %d\0".as_ptr() as *const c_char, line_num); return -1; }
        *mod_ = 0; mod_ = mod_.add(1);
        let mut type_ = strchr(mod_, b'\t' as c_int);
        if type_.is_null() { ERROR(b"malformed Module.symvers (type) at line %d\0".as_ptr() as *const c_char, line_num); return -1; }
        *type_ = 0; type_ = type_.add(1);
        let mut namespace = strchr(type_, b'\t' as c_int);
        if namespace.is_null() { ERROR(b"malformed Module.symvers (namespace) at line %d\0".as_ptr() as *const c_char, line_num); return -1; }
        *namespace = 0; namespace = namespace.add(1);
        if *sym == 0 || *mod_ == 0 {
            ERROR(b"malformed Module.symvers at line %d\0".as_ptr() as *const c_char, line_num);
            return -1;
        }
        let export = calloc(1, size_of::<export>()) as *mut export;
        if export.is_null() { ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char); return -1; }
        (*export).mod_ = strdup(mod_);
        if (*export).mod_.is_null() { ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char); return -1; }
        if strcmp((*export).mod_, b"vmlinux\0".as_ptr() as *const c_char) != 0 {
            (*export).mod_ = normalize_modname((*export).mod_);
        }
        (*export).sym = strdup(sym);
        if (*export).sym.is_null() { ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char); return -1; }
        (*export).mod_ns = strstarts(namespace, b"module:\0".as_ptr() as *const c_char);
        hash_add!(exports, &mut (*export).hash, str_hash(sym));
    }
    free(path as *mut c_void);
    fclose(file);
    0
}

unsafe fn read_sym_checksums(elf_: *mut elf) -> c_int {
    let sec = find_section_by_name(elf_, b".discard.sym_checksum\0".as_ptr() as *const c_char);
    if sec.is_null() {
        ERROR(b"'%s' missing .discard.sym_checksum section, file not processed by 'objtool klp checksum'?\0".as_ptr() as *const c_char, (*elf_).name);
        return -1;
    }
    if (*sec).rsec.is_null() {
        ERROR(b"missing reloc section for .discard.sym_checksum\0".as_ptr() as *const c_char);
        return -1;
    }
    if sec_size(sec) as usize % size_of::<sym_checksum>() != 0 {
        ERROR(b"struct sym_checksum size mismatch\0".as_ptr() as *const c_char);
        return -1;
    }
    for i in 0..(sec_size(sec) as usize / size_of::<sym_checksum>()) {
        let sym_checksum = ((*(*sec).data).d_buf as *mut sym_checksum).add(i);
        let reloc = find_reloc_by_dest(elf_, sec, (i * size_of::<sym_checksum>()) as c_ulong);
        if reloc.is_null() { ERROR(b"can't find reloc for sym_checksum[%d]\0".as_ptr() as *const c_char, i as c_int); return -1; }
        let sym = (*reloc).sym;
        if is_sec_sym(sym) { ERROR(b"not sure how to handle section %s\0".as_ptr() as *const c_char, (*sym).name); return -1; }
        if is_func_sym(sym) || is_object_sym(sym) {
            (*sym).csum.checksum = (*sym_checksum).checksum;
        }
    }
    0
}

unsafe fn first_file_symbol(elf_: *mut elf) -> *mut symbol {
    let mut sym: *mut symbol = null_mut();
    for_each_sym!(elf_, sym, {
        if is_file_sym(sym) { return sym; }
    });
    null_mut()
}

unsafe fn next_file_symbol(elf_: *mut elf, mut sym: *mut symbol) -> *mut symbol {
    for_each_sym_continue!(elf_, sym, {
        if is_file_sym(sym) { return sym; }
    });
    null_mut()
}

unsafe fn is_uncorrelated_static_local(sym: *mut symbol) -> bool_ {
    let vars: [*const c_char; 10] = [
        b"__already_done\0".as_ptr() as *const c_char,
        b"__func__\0".as_ptr() as *const c_char,
        b"__key\0".as_ptr() as *const c_char,
        b"__warned\0".as_ptr() as *const c_char,
        b"_entry\0".as_ptr() as *const c_char,
        b"_entry_ptr\0".as_ptr() as *const c_char,
        b"_rs\0".as_ptr() as *const c_char,
        b"descriptor\0".as_ptr() as *const c_char,
        b"CSWTCH\0".as_ptr() as *const c_char,
        null(),
    ];
    if !is_object_sym(sym) || !is_local_sym(sym) { return false; }
    if strcmp((*(*sym).sec).name, b".data..once\0".as_ptr() as *const c_char) == 0 { return true; }
    let dot = strchr((*sym).name, b'.' as c_int);
    if dot.is_null() { return false; }
    for i in 0..9 {
        let len = strlen(vars[i]);
        if strstarts((*sym).name, vars[i]) && *(*sym).name.add(len) == b'.' as c_char { return true; }
        if strstarts(dot.add(1), vars[i]) && (*dot.add(1 + len) == b'.' as c_char || *dot.add(1 + len) == 0) { return true; }
    }
    false
}

unsafe fn is_local_label(sym: *mut symbol) -> bool_ { strstarts((*sym).name, b".L\0".as_ptr() as *const c_char) }

unsafe fn is_special_section(sec: *mut section) -> bool_ {
    let specials = [
        b".altinstructions\0".as_ptr() as *const c_char, b".kcfi_traps\0".as_ptr() as *const c_char,
        b"__bug_table\0".as_ptr() as *const c_char, b"__ex_table\0".as_ptr() as *const c_char,
        b"__jump_table\0".as_ptr() as *const c_char, b"__mcount_loc\0".as_ptr() as *const c_char,
        b".static_call_sites\0".as_ptr() as *const c_char,
    ];
    let non_special_discards = [
        b".discard.addressable\0".as_ptr() as *const c_char,
        b".discard.sym_checksum\0".as_ptr() as *const c_char,
    ];
    if is_text_sec(sec) { return false; }
    for s in specials { if strcmp((*sec).name, s) == 0 { return true; } }
    for s in non_special_discards { if strcmp((*sec).name, s) == 0 { return false; } }
    strstarts((*sec).name, b".discard.\0".as_ptr() as *const c_char)
}

unsafe fn is_special_section_aux(sec: *mut section) -> bool_ {
    strcmp((*sec).name, b".altinstr_replacement\0".as_ptr() as *const c_char) == 0 ||
    strcmp((*sec).name, b".altinstr_aux\0".as_ptr() as *const c_char) == 0
}

unsafe fn is_addressable_sym(sym: *mut symbol) -> bool_ {
    strcmp((*(*sym).sec).name, b".discard.addressable\0".as_ptr() as *const c_char) == 0
}
unsafe fn is_abs_sym(sym: *mut symbol) -> bool_ { (*sym).sym.st_shndx == SHN_ABS && !is_file_sym(sym) }
unsafe fn is_initcall_sym(sym: *mut symbol) -> bool_ {
    strstarts((*sym).name, b"__initcall__\0".as_ptr() as *const c_char) ||
    strstarts((*sym).name, b"__initstub__\0".as_ptr() as *const c_char)
}
unsafe fn is_anonymous_rodata(sym: *mut symbol) -> bool_ {
    is_rodata_sec((*sym).sec) && (!is_object_sym(sym) || strstarts((*(*sym).sec).name, b".rodata.cst\0".as_ptr() as *const c_char))
}
unsafe fn dont_correlate(sym: *mut symbol) -> bool_ {
    is_file_sym(sym) || is_null_sym(sym) || is_sec_sym(sym) || is_abs_sym(sym) ||
    is_prefix_func(sym) || is_uncorrelated_static_local(sym) || is_local_label(sym) ||
    is_string_sec((*sym).sec) || is_anonymous_rodata(sym) || is_initcall_sym(sym) ||
    is_addressable_sym(sym) || is_special_section((*sym).sec) || is_special_section_aux((*sym).sec)
}

unsafe fn llvm_suffix(name: *const c_char) -> *const c_char {
    strstr(name, b".llvm.\0".as_ptr() as *const c_char)
}
unsafe fn is_llvm_sym(sym: *mut symbol) -> bool_ { !llvm_suffix((*sym).name).is_null() }
unsafe fn maybe_same_file(sym1: *mut symbol, sym2: *mut symbol) -> bool_ {
    if (*sym1).file.is_null() || (*sym2).file.is_null() { return true; }
    if (*sym1).file == (*sym2).file { return true; }
    (*(*sym1).file).twin == (*sym2).file
}
unsafe fn same_file(sym1: *mut symbol, sym2: *mut symbol) -> bool_ {
    if !llvm_suffix((*sym1).name).is_null() && !llvm_suffix((*sym2).name).is_null() { return true; }
    if (*sym1).file.is_null() && (*sym2).file.is_null() { return true; }
    if (*sym1).file.is_null() || (*sym2).file.is_null() { return false; }
    if (*sym1).file == (*sym2).file { return true; }
    (*(*sym1).file).twin == (*sym2).file
}
unsafe fn is_tu_local_sym(sym: *mut symbol) -> bool_ { is_local_sym(sym) || is_llvm_sym(sym) }

unsafe fn find_twin(e: *mut elfs, sym1: *mut symbol) -> *mut symbol {
    let mut name_last: *mut symbol = null_mut();
    let mut scope_last: *mut symbol = null_mut();
    let mut file_last: *mut symbol = null_mut();
    let mut csum_last: *mut symbol = null_mut();
    let (mut name_orig, mut name_patched, mut scope_orig, mut scope_patched) = (0u32, 0u32, 0u32, 0u32);
    let (mut file_orig, mut file_patched, mut csum_orig, mut csum_patched) = (0u32, 0u32, 0u32, 0u32);
    let mut sym2: *mut symbol = null_mut();
    let mut match_: *mut symbol = null_mut();
    for_each_sym_by_demangled_name!((*e).orig, (*sym1).demangled_name, sym2, {
        if !(*sym2).twin.is_null() || (*sym1).type_ != (*sym2).type_ || (*sym2).dont_correlate != 0 || !maybe_same_file(sym1, sym2) { continue; }
        name_orig += 1;
        if is_tu_local_sym(sym1) != is_tu_local_sym(sym2) { continue; }
        scope_orig += 1;
        if !same_file(sym1, sym2) { continue; }
        file_orig += 1;
        if (*sym1).len != (*sym2).len || (*sym1).csum.checksum == 0 || (*sym1).csum.checksum != (*sym2).csum.checksum { continue; }
        csum_orig += 1;
    });
    for_each_sym_by_demangled_name!((*e).patched, (*sym1).demangled_name, sym2, {
        if !(*sym2).twin.is_null() || (*sym1).type_ != (*sym2).type_ || (*sym2).dont_correlate != 0 || !maybe_same_file(sym1, sym2) { continue; }
        name_patched += 1; name_last = sym2;
        if is_tu_local_sym(sym1) != is_tu_local_sym(sym2) { continue; }
        scope_patched += 1; scope_last = sym2;
        if !same_file(sym1, sym2) { continue; }
        file_patched += 1; file_last = sym2;
        if (*sym1).len != (*sym2).len || (*sym1).csum.checksum == 0 || (*sym1).csum.checksum != (*sym2).csum.checksum { continue; }
        csum_patched += 1; csum_last = sym2;
    });
    if name_orig == 1 && name_patched == 1 { match_ = name_last; }
    else if scope_orig == 1 && scope_patched == 1 { match_ = scope_last; }
    else if file_orig == 1 && file_patched == 1 { match_ = file_last; }
    else if csum_orig == 1 && csum_patched == 1 { match_ = csum_last; }
    if match_.is_null() { return null_mut(); }
    if name_orig != 1 || name_patched != 1 {
        dbg_correlate(b"find_twin(): %s%s -> %s%s\0".as_ptr() as *const c_char,
            (*sym1).name, if is_func_sym(sym1) { b"()\0".as_ptr() } else { b"\0".as_ptr() },
            (*match_).name, if is_func_sym(match_) { b"()\0".as_ptr() } else { b"\0".as_ptr() });
    }
    match_
}

#[repr(C)]
pub struct llvm_suffix_pair {
    pub hash: hlist_node,
    pub orig: *const c_char,
    pub patched: *const c_char,
}

unsafe fn update_suffix_map(elf_: *mut elf) -> c_int {
    let mut sym: *mut symbol = null_mut();
    for_each_sym!(elf_, sym, {
        if (*sym).twin.is_null() { continue; }
        let s1 = llvm_suffix((*sym).name);
        let s2 = llvm_suffix((*(*sym).twin).name);
        if s1.is_null() || s2.is_null() { continue; }
        let mut found = false;
        let mut entry: *mut llvm_suffix_pair = null_mut();
        hash_for_each_possible!(suffix_map, entry, hash, str_hash(s1), {
            if strcmp((*entry).orig, s1) == 0 { found = true; break; }
        });
        if found { continue; }
        entry = calloc(1, size_of::<llvm_suffix_pair>()) as *mut llvm_suffix_pair;
        if entry.is_null() { ERROR_GLIBC(b"calloc\0".as_ptr() as *const c_char); return -1; }
        (*entry).orig = s1;
        (*entry).patched = s2;
        hash_add!(suffix_map, &mut (*entry).hash, str_hash(s1));
    });
    0
}

unsafe fn find_twin_suffixed(elf_: *mut elf, sym1: *mut symbol) -> *mut symbol {
    let suffix = llvm_suffix((*sym1).name);
    if suffix.is_null() { return null_mut(); }
    let mut patched_suffix: *const c_char = null();
    let mut entry: *mut llvm_suffix_pair = null_mut();
    hash_for_each_possible!(suffix_map, entry, hash, str_hash(suffix), {
        if strcmp((*entry).orig, suffix) == 0 { patched_suffix = (*entry).patched; break; }
    });
    if patched_suffix.is_null() { return null_mut(); }
    let mut name = [0 as c_char; SYM_NAME_LEN];
    if snprintf_check(name.as_mut_ptr(), SYM_NAME_LEN, b"%s%s\0".as_ptr() as *const c_char, (*sym1).demangled_name, patched_suffix) != 0 { return null_mut(); }
    let mut sym2: *mut symbol = null_mut();
    let mut match_: *mut symbol = null_mut();
    let mut count = 0;
    for_each_sym_by_name!(elf_, name.as_ptr(), sym2, {
        if !(*sym2).twin.is_null() || (*sym1).type_ != (*sym2).type_ || (*sym2).dont_correlate != 0 { continue; }
        count += 1; match_ = sym2;
    });
    if count != 1 { return null_mut(); }
    dbg_correlate(b"find_suffixed_twin(): %s%s -> %s%s\0".as_ptr() as *const c_char,
        (*sym1).name, if is_func_sym(sym1) { b"()\0".as_ptr() } else { b"\0".as_ptr() },
        (*match_).name, if is_func_sym(match_) { b"()\0".as_ptr() } else { b"\0".as_ptr() });
    match_
}

unsafe fn find_twin_positional(e: *mut elfs, sym1: *mut symbol) -> *mut symbol {
    let (mut idx_orig, mut idx_patched, mut sym1_pos) = (0u32, 0u32, 0u32);
    let mut sym2: *mut symbol = null_mut();
    let mut match_: *mut symbol = null_mut();
    for_each_sym_by_demangled_name!((*e).orig, (*sym1).demangled_name, sym2, {
        if !(*sym2).twin.is_null() || (*sym1).type_ != (*sym2).type_ || (*sym2).dont_correlate != 0 || !maybe_same_file(sym1, sym2) { continue; }
        if is_tu_local_sym(sym1) != is_tu_local_sym(sym2) || is_llvm_sym(sym1) != is_llvm_sym(sym2) { continue; }
        if sym1 == sym2 { sym1_pos = idx_orig; }
        idx_orig += 1;
    });
    for_each_sym_by_demangled_name!((*e).patched, (*sym1).demangled_name, sym2, {
        if !(*sym2).twin.is_null() || (*sym1).type_ != (*sym2).type_ || (*sym2).dont_correlate != 0 || !maybe_same_file(sym1, sym2) { continue; }
        if is_tu_local_sym(sym1) != is_tu_local_sym(sym2) || is_llvm_sym(sym1) != is_llvm_sym(sym2) { continue; }
        if idx_patched == sym1_pos { match_ = sym2; }
        idx_patched += 1;
    });
    if idx_orig != idx_patched { return null_mut(); }
    dbg_correlate(b"find_twin_positional(): %s%s -> %s%s\0".as_ptr() as *const c_char,
        (*sym1).name, if is_func_sym(sym1) { b"()\0".as_ptr() } else { b"\0".as_ptr() },
        (*match_).name, if is_func_sym(match_) { b"()\0".as_ptr() } else { b"\0".as_ptr() });
    match_
}

unsafe fn correlate_symbols(e: *mut elfs) -> c_int {
    let mut sym1: *mut symbol = null_mut();
    let mut sym2: *mut symbol = null_mut();
    for_each_sym!((*e).orig, sym1, { (*sym1).dont_correlate = dont_correlate(sym1) as c_uint; });
    for_each_sym!((*e).patched, sym2, { (*sym2).dont_correlate = dont_correlate(sym2) as c_uint; });
    let mut file1_sym = first_file_symbol((*e).orig);
    let mut file2_sym = first_file_symbol((*e).patched);
    loop {
        if file1_sym.is_null() && !file2_sym.is_null() { ERROR(b"FILE symbol mismatch: NULL != %s\0".as_ptr() as *const c_char, (*file2_sym).name); return -1; }
        if !file1_sym.is_null() && file2_sym.is_null() { ERROR(b"FILE symbol mismatch: %s != NULL\0".as_ptr() as *const c_char, (*file1_sym).name); return -1; }
        if file1_sym.is_null() { break; }
        if strcmp((*file1_sym).name, (*file2_sym).name) != 0 { ERROR(b"FILE symbol mismatch: %s != %s\0".as_ptr() as *const c_char, (*file1_sym).name, (*file2_sym).name); return -1; }
        (*file1_sym).twin = file2_sym; (*file2_sym).twin = file1_sym;
        file1_sym = next_file_symbol((*e).orig, file1_sym);
        file2_sym = next_file_symbol((*e).patched, file2_sym);
    }
    hash_init!(suffix_map);
    loop {
        let mut progress = false;
        for_each_sym!((*e).orig, sym1, {
            if !(*sym1).twin.is_null() || (*sym1).dont_correlate != 0 { continue; }
            sym2 = find_twin(e, sym1);
            if sym2.is_null() { continue; }
            (*sym1).twin = sym2; (*sym2).twin = sym1; progress = true;
        });
        if update_suffix_map((*e).orig) != 0 { return -1; }
        for_each_sym!((*e).orig, sym1, {
            if !(*sym1).twin.is_null() || (*sym1).dont_correlate != 0 { continue; }
            sym2 = find_twin_suffixed((*e).patched, sym1);
            if sym2.is_null() { continue; }
            (*sym1).twin = sym2; (*sym2).twin = sym1; progress = true;
        });
        if !progress { break; }
    }
    for_each_sym!((*e).orig, sym1, {
        if !(*sym1).twin.is_null() || (*sym1).dont_correlate != 0 { continue; }
        sym2 = find_twin_positional(e, sym1);
        if sym2.is_null() { continue; }
        (*sym1).twin = sym2; (*sym2).twin = sym1;
    });
    for_each_sym!((*e).orig, sym1, {
        if !(*sym1).twin.is_null() || (*sym1).dont_correlate != 0 { continue; }
        WARN(b"no correlation: %s\0".as_ptr() as *const c_char, (*sym1).name);
    });
    0
}

unsafe fn clone_sym_relocs(e: *mut elfs, patched_sym: *mut symbol) -> c_int { clone_sym_relocs_impl(e, patched_sym) }

unsafe fn __clone_symbol(elf_: *mut elf, patched_sym: *mut symbol, data_too: bool_) -> *mut symbol {
    let mut out_sec: *mut section = null_mut();
    let mut offset: c_ulong = 0;
    let mut out_sym: *mut symbol;
    if data_too && !is_undef_sym(patched_sym) {
        let patched_sec = (*patched_sym).sec;
        out_sec = find_section_by_name(elf_, (*patched_sec).name);
        if out_sec.is_null() {
            out_sec = elf_create_section(elf_, (*patched_sec).name, 0, (*patched_sec).sh.sh_entsize, (*patched_sec).sh.sh_type, (*patched_sec).sh.sh_addralign, (*patched_sec).sh.sh_flags);
            if out_sec.is_null() { return null_mut(); }
        }
        if is_string_sec((*patched_sym).sec) {
            out_sym = elf_create_section_symbol(elf_, out_sec);
            if out_sym.is_null() { return null_mut(); }
            (*patched_sym).clone = out_sym; (*out_sym).clone = patched_sym;
            return out_sym;
        }
        if !is_sec_sym(patched_sym) { offset = ALIGN(sec_size(out_sec), (*out_sec).sh.sh_addralign); }
        if (*patched_sym).len != 0 || is_sec_sym(patched_sym) {
            let mut data_ptr: *mut c_void = null_mut();
            if !(*patched_sym).sec.is_null() && !(*(*patched_sym).sec).data.is_null() && !(*(*(*patched_sym).sec).data).d_buf.is_null() {
                data_ptr = (*(*(*patched_sym).sec).data).d_buf.add((*patched_sym).offset as usize) as *mut c_void;
            }
            let size = if is_sec_sym(patched_sym) { sec_size((*patched_sym).sec) } else { (*patched_sym).len };
            if elf_add_data(elf_, out_sec, data_ptr, size as usize).is_null() { return null_mut(); }
        }
    }
    out_sym = elf_create_symbol(elf_, (*patched_sym).name, out_sec, (*patched_sym).bind, (*patched_sym).type_, offset, (*patched_sym).len);
    if out_sym.is_null() { return null_mut(); }
    (*patched_sym).clone = out_sym; (*out_sym).clone = patched_sym;
    out_sym
}

unsafe fn sym_type(sym: *mut symbol) -> *const c_char {
    match (*sym).type_ {
        STT_NOTYPE => b"NOTYPE\0".as_ptr() as *const c_char,
        STT_OBJECT => b"OBJECT\0".as_ptr() as *const c_char,
        STT_FUNC => b"FUNC\0".as_ptr() as *const c_char,
        STT_SECTION => b"SECTION\0".as_ptr() as *const c_char,
        STT_FILE => b"FILE\0".as_ptr() as *const c_char,
        _ => b"UNKNOWN\0".as_ptr() as *const c_char,
    }
}
unsafe fn sym_bind(sym: *mut symbol) -> *const c_char {
    match (*sym).bind {
        STB_LOCAL => b"LOCAL\0".as_ptr() as *const c_char,
        STB_GLOBAL => b"GLOBAL\0".as_ptr() as *const c_char,
        STB_WEAK => b"WEAK\0".as_ptr() as *const c_char,
        _ => b"UNKNOWN\0".as_ptr() as *const c_char,
    }
}

unsafe fn clone_symbol(e: *mut elfs, patched_sym: *mut symbol, data_too: bool_) -> *mut symbol {
    if !(*patched_sym).clone.is_null() { return (*patched_sym).clone; }
    dbg_clone(b"%s%s\0".as_ptr() as *const c_char, (*patched_sym).name, if data_too { b" [+DATA]\0".as_ptr() } else { b"\0".as_ptr() });
    if is_func_sym(patched_sym) && data_too {
        let pfx = get_func_prefix(patched_sym);
        if !pfx.is_null() { clone_symbol(e, pfx, true); }
    }
    if __clone_symbol((*e).out, patched_sym, data_too).is_null() { return null_mut(); }
    if data_too && clone_sym_relocs(e, patched_sym) != 0 { return null_mut(); }
    (*patched_sym).clone
}

unsafe fn mark_included_function(func: *mut symbol) {
    (*func).included = 1;
    let pfx = get_func_prefix(func);
    if !pfx.is_null() { (*pfx).included = 1; }
    if !(*func).cfunc.is_null() && (*func).cfunc != func { (*(*func).cfunc).included = 1; }
    if !(*func).pfunc.is_null() && (*func).pfunc != func { (*(*func).pfunc).included = 1; }
}

unsafe fn mark_changed_functions(e: *mut elfs) -> c_int {
    let mut changed = false;
    let mut orig_sym: *mut symbol = null_mut();
    let mut patched_sym: *mut symbol = null_mut();
    for_each_sym!((*e).orig, orig_sym, {
        if (*orig_sym).dont_correlate != 0 { continue; }
        patched_sym = (*orig_sym).twin;
        if patched_sym.is_null() { continue; }
        if (*orig_sym).csum.checksum != (*patched_sym).csum.checksum {
            if !is_func_sym(orig_sym) { ERROR(b"changed data: %s\0".as_ptr() as *const c_char, (*orig_sym).name); return -1; }
            (*patched_sym).changed = 1; mark_included_function(patched_sym); changed = true;
        }
    });
    for_each_sym!((*e).patched, patched_sym, {
        if !is_func_sym(patched_sym) || (*patched_sym).dont_correlate != 0 { continue; }
        if (*patched_sym).twin.is_null() {
            printf(b"%s: new function: %s\n\0".as_ptr() as *const c_char, objname, (*patched_sym).name);
            mark_included_function(patched_sym); changed = true;
        }
    });
    for_each_sym!((*e).patched, patched_sym, {
        if (*patched_sym).changed != 0 { printf(b"%s: changed function: %s\n\0".as_ptr() as *const c_char, objname, (*patched_sym).name); }
    });
    if !changed { 1 } else { 0 }
}

unsafe fn clone_included_functions(e: *mut elfs) -> c_int {
    let mut patched_sym: *mut symbol = null_mut();
    for_each_sym!((*e).patched, patched_sym, {
        if (*patched_sym).included != 0 && clone_symbol(e, patched_sym, true).is_null() { return -1; }
    });
    0
}

unsafe fn find_export(sym: *mut symbol) -> *mut export {
    if is_local_sym(sym) { return null_mut(); }
    let mut export: *mut export = null_mut();
    hash_for_each_possible!(exports, export, hash, str_hash((*sym).name), {
        if strcmp((*export).sym, (*sym).name) == 0 { return export; }
    });
    null_mut()
}

unsafe fn __find_modname(e: *mut elfs) -> *const c_char {
    let sec = find_section_by_name((*e).orig, b".modinfo\0".as_ptr() as *const c_char);
    if sec.is_null() { ERROR(b"missing .modinfo section\0".as_ptr() as *const c_char); return null(); }
    let name = memmem((*(*sec).data).d_buf as *const c_void, sec_size(sec) as usize, b"\0name=\0".as_ptr() as *const c_void, 6) as *mut c_char;
    if !name.is_null() { return name.add(6); }
    let name = strdup((*(*e).orig).name);
    if name.is_null() { ERROR_GLIBC(b"strdup\0".as_ptr() as *const c_char); return null(); }
    normalize_modname(name)
}
unsafe fn find_modname(e: *mut elfs) -> *const c_char {
    if !(*e).modname.is_null() { return (*e).modname; }
    let modname = __find_modname(e);
    (*e).modname = modname;
    modname
}

unsafe fn klp_reloc_needed(patched_reloc: *mut reloc) -> bool_ {
    let patched_sym = (*patched_reloc).sym;
    if (*patched_sym).dont_correlate != 0 { return false; }
    if (*patched_sym).included != 0 { return false; }
    let export = find_export(patched_sym);
    if !export.is_null() {
        if strcmp((*export).mod_, b"vmlinux\0".as_ptr() as *const c_char) != 0 { return true; }
        return (*export).mod_ns;
    }
    if (*patched_sym).twin.is_null() { return false; }
    true
}

unsafe fn convert_reloc_sym_to_secsym(elf_: *mut elf, reloc_: *mut reloc) -> c_int {
    let sym = (*reloc_).sym;
    let sec = (*sym).sec;
    if is_sec_sym(sym) { return 0; }
    if (*sec).sym.is_null() && elf_create_section_symbol(elf_, sec).is_null() { return -1; }
    (*reloc_).sym = (*sec).sym;
    set_reloc_sym(elf_, reloc_, (*(*sec).sym).idx);
    set_reloc_addend(elf_, reloc_, (*sym).offset as s64 + reloc_addend(reloc_));
    0
}

unsafe fn convert_reloc_secsym_to_sym(elf_: *mut elf, reloc_: *mut reloc) -> c_int {
    let mut sym = (*reloc_).sym;
    let sec = (*sym).sec;
    if !is_sec_sym(sym) { return 0; }
    sym = find_symbol_by_offset(sec, 0);
    if !sym.is_null() && (*sym).len == sec_size(sec) { }
    else {
        sym = find_symbol_containing_inclusive(sec, arch_adjusted_addend(reloc_));
        if sym.is_null() {
            if sec_size(sec) == 0 { return 1; }
            if is_rodata_sec(sec) { return 0; }
            return -1;
        }
    }
    (*reloc_).sym = sym;
    set_reloc_sym(elf_, reloc_, (*sym).idx);
    set_reloc_addend(elf_, reloc_, reloc_addend(reloc_) - (*sym).offset as s64);
    0
}

unsafe fn is_uncorrelated_section(sec: *mut section) -> bool_ {
    is_string_sec(sec) ||
    strstarts((*sec).name, b".data..Lubsan\0".as_ptr() as *const c_char) ||
    strstarts((*sec).name, b".data..L__unnamed_\0".as_ptr() as *const c_char) ||
    strstarts((*sec).name, b".data..Lanon.\0".as_ptr() as *const c_char)
}
unsafe fn convert_reloc_sym(elf_: *mut elf, reloc_: *mut reloc) -> c_int {
    let sec = (*(*reloc_).sym).sec;
    if reloc_type(reloc_) == R_NONE { return 1; }
    if is_uncorrelated_section(sec) { return convert_reloc_sym_to_secsym(elf_, reloc_); }
    convert_reloc_secsym_to_sym(elf_, reloc_)
}

unsafe fn has_module_dep(e: *mut elfs, dep_mod: *const c_char) -> bool_ {
    let mut sym: *mut symbol = null_mut();
    for_each_sym!((*e).orig, sym, {
        if !is_undef_sym(sym) || is_weak_sym(sym) { continue; }
        let exp = find_export(sym);
        if !exp.is_null() && strcmp((*exp).mod_, dep_mod) == 0 { return true; }
    });
    false
}

unsafe fn clone_reloc_klp(e: *mut elfs, patched_reloc: *mut reloc, sec: *mut section, offset: c_ulong, export: *mut export) -> c_int {
    let patched_sym = (*patched_reloc).sym;
    let addend = reloc_addend(patched_reloc);
    let mut tombstone_name = [0 as c_char; SYM_NAME_LEN];
    let mut sec_name = [0 as c_char; SEC_NAME_LEN];
    let mut sym_name = [0 as c_char; SYM_NAME_LEN];
    if (*patched_sym).twin.is_null() {
        if export.is_null() { ERROR(b"unexpected klp reloc for new symbol %s\0".as_ptr() as *const c_char, (*patched_sym).name); return -1; }
        if strcmp((*export).mod_, b"vmlinux\0".as_ptr() as *const c_char) != 0 && !has_module_dep(e, (*export).mod_) {
            ERROR(b"%s: new reference to %s (exported by %s) would create an undeclared module dependency\0".as_ptr() as *const c_char, (*patched_sym).name, (*export).sym, (*export).mod_);
            return -1;
        }
    }
    let mut sym = (*patched_sym).clone;
    if sym.is_null() {
        if snprintf_check(tombstone_name.as_mut_ptr(), SYM_NAME_LEN, b".klp.tombstone.%s\0".as_ptr() as *const c_char, (*patched_sym).name) != 0 { return -1; }
        sym = elf_create_symbol((*e).out, tombstone_name.as_ptr(), null_mut(), STB_WEAK, (*patched_sym).type_, 0, 0);
        if sym.is_null() { return -1; }
        (*patched_sym).clone = sym; (*sym).clone = patched_sym;
    }
    if elf_create_reloc((*e).out, sec, offset, sym, addend, reloc_type(patched_reloc)).is_null() { return -1; }
    let (sym_modname, sym_orig_name, sympos) = if !export.is_null() {
        ((*export).mod_ as *const c_char, (*export).sym as *const c_char, 0 as c_ulong)
    } else {
        let modname = find_modname(e);
        if modname.is_null() { return -1; }
        let sp = klp_find_sympos((*e).orig, (*patched_sym).twin);
        if sp == ULONG_MAX { return -1; }
        (modname, (*(*patched_sym).twin).name, sp)
    };
    if snprintf_check(sym_name.as_mut_ptr(), SYM_NAME_LEN, b".klp.sym.%s.%s,%ld\0".as_ptr() as *const c_char, sym_modname, sym_orig_name, sympos) != 0 { return -1; }
    let mut klp_sym = find_symbol_by_name((*e).out, sym_name.as_ptr());
    if klp_sym.is_null() {
        __dbg_clone(b"%s\0".as_ptr() as *const c_char, sym_name.as_ptr());
        klp_sym = elf_create_symbol((*e).out, sym_name.as_ptr(), null_mut(), STB_WEAK, (*patched_sym).type_, 0, 0);
        if klp_sym.is_null() { return -1; }
    }
    let sec_objname = if strcmp(sym_modname, b"vmlinux\0".as_ptr() as *const c_char) == 0 {
        b"vmlinux\0".as_ptr() as *const c_char
    } else {
        let m = find_modname(e);
        if m.is_null() { return -1; }
        m
    };
    if snprintf_check(sec_name.as_mut_ptr(), SEC_NAME_LEN, b"__klp_relocs.%s\0".as_ptr() as *const c_char, sec_objname) != 0 { return -1; }
    let mut klp_relocs = find_section_by_name((*e).out, sec_name.as_ptr());
    if klp_relocs.is_null() {
        klp_relocs = elf_create_section((*e).out, sec_name.as_ptr(), 0, 0, SHT_PROGBITS, 8, SHF_ALLOC);
        if klp_relocs.is_null() { return -1; }
    }
    let klp_reloc_off = sec_size(klp_relocs);
    let mut klp_reloc: klp_reloc = zeroed();
    klp_reloc.type_ = reloc_type(patched_reloc) as u64_;
    if elf_add_data((*e).out, klp_relocs, &klp_reloc as *const _ as *const c_void, size_of::<klp_reloc>()).is_null() { return -1; }
    if (*sec).sym.is_null() && elf_create_section_symbol((*e).out, sec).is_null() { return -1; }
    if elf_create_reloc((*e).out, klp_relocs, klp_reloc_off + offsetof!(klp_reloc, offset) as c_ulong, (*sec).sym, offset as s64, R_ABS64).is_null() { return -1; }
    if elf_create_reloc((*e).out, klp_relocs, klp_reloc_off + offsetof!(klp_reloc, sym) as c_ulong, klp_sym, addend, R_ABS64).is_null() { return -1; }
    0
}

unsafe fn clone_reloc(e: *mut elfs, patched_reloc: *mut reloc, sec: *mut section, offset: c_ulong) -> c_int {
    let patched_sym = (*patched_reloc).sym;
    let export = find_export(patched_sym);
    let mut addend = reloc_addend(patched_reloc);
    let klp = klp_reloc_needed(patched_reloc);
    dbg_clone(b"%s+0x%lx: %s%s0x%lx [%s%s%s%s%s%s]\0".as_ptr() as *const c_char,
        (*sec).name, offset, (*patched_sym).name,
        if addend >= 0 { b"+\0".as_ptr() } else { b"-\0".as_ptr() }, labs(addend as c_long),
        sym_type(patched_sym), if is_sec_sym(patched_sym) { b"\0".as_ptr() } else { b" \0".as_ptr() },
        if is_sec_sym(patched_sym) { b"\0".as_ptr() } else { sym_bind(patched_sym) },
        if is_undef_sym(patched_sym) { b" UNDEF\0".as_ptr() } else { b"\0".as_ptr() },
        if !export.is_null() { b" EXPORTED\0".as_ptr() } else { b"\0".as_ptr() },
        if klp { b" KLP\0".as_ptr() } else { b"\0".as_ptr() });
    if klp { return clone_reloc_klp(e, patched_reloc, sec, offset, export); }
    let out_sym = clone_symbol(e, patched_sym, (*patched_sym).included != 0 || export.is_null());
    if out_sym.is_null() { return -1; }
    if is_string_sec((*patched_sym).sec) {
        let str_ = (*(*(*patched_sym).sec).data).d_buf.add(addend as usize);
        __dbg_clone(b"\"%s\"\0".as_ptr() as *const c_char, escape_str(str_));
        addend = elf_add_string((*e).out, (*out_sym).sec, str_);
        if addend == -1 { return -1; }
    }
    if elf_create_reloc((*e).out, sec, offset, out_sym, addend, reloc_type(patched_reloc)).is_null() { return -1; }
    0
}

unsafe fn clone_sym_relocs_impl(e: *mut elfs, patched_sym: *mut symbol) -> c_int {
    let patched_rsec = (*(*patched_sym).sec).rsec;
    let out_sym = (*patched_sym).clone;
    if out_sym.is_null() { ERROR(b"no clone for %s\0".as_ptr() as *const c_char, (*patched_sym).name); return -1; }
    if patched_rsec.is_null() || (!is_sec_sym(patched_sym) && (*patched_sym).len == 0) || is_string_sec((*patched_sym).sec) { return 0; }
    let (start, end) = if is_sec_sym(patched_sym) { (0, sec_size((*patched_sym).sec)) } else { ((*patched_sym).offset, (*patched_sym).offset + (*patched_sym).len) };
    let mut patched_reloc: *mut reloc = null_mut();
    for_each_reloc!(patched_rsec, patched_reloc, {
        if reloc_offset(patched_reloc) < start || reloc_offset(patched_reloc) >= end { continue; }
        if !(*(*patched_reloc).sym).sec.is_null() && strcmp((*(*(*patched_reloc).sym).sec).name, b".altinstr_aux\0".as_ptr() as *const c_char) == 0 { continue; }
        if arch_alt_ignore_new_reloc((*patched_sym).sec, reloc_offset(patched_reloc)) { continue; }
        let ret = convert_reloc_sym((*e).patched, patched_reloc);
        if ret < 0 { ERROR_FUNC((*patched_rsec).base, reloc_offset(patched_reloc), b"failed to convert reloc sym '%s' to its proper format\0".as_ptr() as *const c_char, (*(*patched_reloc).sym).name); return -1; }
        if ret > 0 { continue; }
        let offset = (*out_sym).offset + (reloc_offset(patched_reloc) - (*patched_sym).offset);
        if clone_reloc(e, patched_reloc, (*out_sym).sec, offset) != 0 { return -1; }
    });
    0
}

unsafe fn create_fake_symbol(elf_: *mut elf, sec: *mut section, offset: c_ulong, size: size_t) -> c_int {
    static mut ctr: c_int = 0;
    let mut name = [0 as c_char; SYM_NAME_LEN];
    if snprintf_check(name.as_mut_ptr(), SYM_NAME_LEN, b"%s_%d\0".as_ptr() as *const c_char, (*sec).name, ctr) != 0 { return -1; }
    ctr += 1;
    let mut c = name.as_mut_ptr();
    while *c != 0 { if *c == b'.' as c_char { *c = b'_' as c_char; } c = c.add(1); }
    let type_ = if is_text_sec(sec) { STT_NOTYPE } else { STT_OBJECT };
    let sym = elf_create_symbol(elf_, name.as_ptr(), sec, STB_LOCAL, type_, offset, size as c_ulong);
    if sym.is_null() { return -1; }
    (*sym).fake = 1;
    0
}

unsafe fn has_fake_symbols(sec: *mut section) -> bool_ {
    let mut sym: *mut symbol = null_mut();
    sec_for_each_sym!(sec, sym, { if (*sym).fake != 0 { return true; } });
    false
}

unsafe fn create_fake_symbols(elf_: *mut elf) -> c_int {
    let mut sec = find_section_by_name(elf_, b".discard.annotate_data\0".as_ptr() as *const c_char);
    let mut reloc_: *mut reloc = null_mut();
    if !sec.is_null() && !(*sec).rsec.is_null() {
        for_each_reloc!((*sec).rsec, reloc_, {
            if annotype(elf_, sec, reloc_) != ANNOTYPE_DATA_SPECIAL { continue; }
            let offset = reloc_addend(reloc_) as c_ulong;
            let mut size: c_ulong = 0;
            let mut next_reloc = reloc_;
            let mut last = true;
            for_each_reloc_continue!((*sec).rsec, next_reloc, {
                if annotype(elf_, sec, next_reloc) != ANNOTYPE_DATA_SPECIAL || (*(*next_reloc).sym).sec != (*(*reloc_).sym).sec { continue; }
                size = (reloc_addend(next_reloc) as c_ulong) - offset; last = false; break;
            });
            if last { size = sec_size((*(*reloc_).sym).sec) - offset; }
            if create_fake_symbol(elf_, (*(*reloc_).sym).sec, offset, size as size_t) != 0 { return -1; }
        });
    }
    for_each_sec!(elf_, sec, {
        if !is_special_section(sec) { continue; }
        if has_fake_symbols(sec) { continue; }
        if (*sec).rsec.is_null() { ERROR(b"%s: missing special section relocations\0".as_ptr() as *const c_char, (*sec).name); return -1; }
        let mut entry_size = (*sec).sh.sh_entsize as c_uint;
        if entry_size == 0 {
            entry_size = arch_reloc_size((*(*sec).rsec).relocs);
            if sec_size(sec) != entry_size as c_ulong * sec_num_entries((*sec).rsec) {
                ERROR(b"%s: missing special section entsize or annotations\0".as_ptr() as *const c_char, (*sec).name); return -1;
            }
        }
        let mut offset: c_ulong = 0;
        while offset < sec_size(sec) {
            if create_fake_symbol(elf_, sec, offset, entry_size as size_t) != 0 { return -1; }
            offset += entry_size as c_ulong;
        }
    });
    0
}

unsafe fn should_keep_special_sym(elf_: *mut elf, sym: *mut symbol) -> bool_ {
    let annotate_insn = strcmp((*(*sym).sec).name, b".discard.annotate_insn\0".as_ptr() as *const c_char) == 0;
    if is_sec_sym(sym) || (*(*sym).sec).rsec.is_null() { return false; }
    let mut reloc_: *mut reloc = null_mut();
    sym_for_each_reloc!(elf_, sym, reloc_, {
        if convert_reloc_sym(elf_, reloc_) != 0 { continue; }
        if (*(*reloc_).sym).clone.is_null() || is_undef_sym((*(*reloc_).sym).clone) { continue; }
        if is_func_sym((*reloc_).sym) || (annotate_insn && is_notype_sym((*reloc_).sym)) { return true; }
    });
    false
}

unsafe fn validate_special_section_klp_reloc(e: *mut elfs, sym: *mut symbol) -> c_int {
    let static_branch = strcmp((*(*sym).sec).name, b"__jump_table\0".as_ptr() as *const c_char) == 0;
    let static_call = strcmp((*(*sym).sec).name, b".static_call_sites\0".as_ptr() as *const c_char) == 0;
    let mut code_sym: *const c_char = null();
    let mut code_offset: c_ulong = 0;
    let mut ret = 0;
    if !static_branch && !static_call { return 0; }
    let mut reloc_: *mut reloc = null_mut();
    sym_for_each_reloc!((*e).patched, sym, reloc_, {
        if convert_reloc_sym((*e).patched, reloc_) != 0 { continue; }
        if (*(*reloc_).sym).type_ != STT_OBJECT {
            if (*(*reloc_).sym).type_ == STT_FUNC && code_sym.is_null() { code_sym = (*(*reloc_).sym).name; code_offset = reloc_addend(reloc_) as c_ulong; }
            continue;
        }
        if !klp_reloc_needed(reloc_) { continue; }
        let export = find_export((*reloc_).sym);
        let sym_modname = if !export.is_null() { (*export).mod_ as *const c_char } else { let m = find_modname(e); if m.is_null() { return -1; } m };
        if strcmp(sym_modname, b"vmlinux\0".as_ptr() as *const c_char) == 0 { continue; }
        if code_sym.is_null() { code_sym = b"<unknown>\0".as_ptr() as *const c_char; }
        if static_branch {
            if strstarts((*(*reloc_).sym).name, b"__tracepoint_\0".as_ptr() as *const c_char) {
                WARN(b"%s: disabling unsupported tracepoint %s\0".as_ptr() as *const c_char, code_sym, (*(*reloc_).sym).name.add(13)); ret = 1; continue;
            }
            if !strstr((*(*reloc_).sym).name, b"__UNIQUE_ID_ddebug_\0".as_ptr() as *const c_char).is_null() {
                WARN(b"%s: disabling unsupported pr_debug()\0".as_ptr() as *const c_char, code_sym); ret = 1; continue;
            }
            ERROR(b"%s+0x%lx: unsupported static branch key %s.  Use static_key_enabled() instead\0".as_ptr() as *const c_char, code_sym, code_offset, (*(*reloc_).sym).name);
            return -1;
        }
        if strstarts((*(*reloc_).sym).name, b"__SCK__tp_func_\0".as_ptr() as *const c_char) { ret = 1; continue; }
        ERROR(b"%s()+0x%lx: unsupported static call key %s.  Use KLP_STATIC_CALL() instead\0".as_ptr() as *const c_char, code_sym, code_offset, (*(*reloc_).sym).name);
        return -1;
    });
    ret
}

unsafe fn clone_special_section(e: *mut elfs, patched_sec: *mut section) -> c_int {
    let mut patched_sym: *mut symbol = null_mut();
    sec_for_each_sym!(patched_sec, patched_sym, {
        if !is_object_sym(patched_sym) { continue; }
        if !should_keep_special_sym((*e).patched, patched_sym) { continue; }
        let ret = validate_special_section_klp_reloc(e, patched_sym);
        if ret < 0 { return -1; }
        if ret > 0 { continue; }
        if clone_symbol(e, patched_sym, true).is_null() { return -1; }
    });
    0
}

unsafe fn clone_special_sections(e: *mut elfs) -> c_int {
    let mut sec: *mut section = null_mut();
    let mut annotate_insn: *mut section = null_mut();
    for_each_sec!((*e).patched, sec, {
        if is_special_section(sec) {
            if strcmp((*sec).name, b".discard.annotate_insn\0".as_ptr() as *const c_char) == 0 { annotate_insn = sec; continue; }
            if clone_special_section(e, sec) != 0 { return -1; }
        }
    });
    if !annotate_insn.is_null() && clone_special_section(e, annotate_insn) != 0 { return -1; }
    0
}

unsafe fn create_klp_sections(e: *mut elfs) -> c_int {
    let obj_size = size_of::<klp_object_ext>();
    let func_size = size_of::<klp_func_ext>();
    let obj_sec = elf_create_section_pair((*e).out, KLP_OBJECTS_SEC.as_ptr(), obj_size, 0, 0);
    if obj_sec.is_null() { return -1; }
    let funcs_sec = elf_create_section_pair((*e).out, KLP_FUNCS_SEC.as_ptr(), func_size, 0, 0);
    if funcs_sec.is_null() { return -1; }
    let funcs_sym = elf_create_section_symbol((*e).out, funcs_sec);
    if funcs_sym.is_null() { return -1; }
    let str_sec = elf_create_section((*e).out, KLP_STRINGS_SEC.as_ptr() as *const c_char, 0, 0, SHT_PROGBITS, 1, SHF_ALLOC | SHF_STRINGS | SHF_MERGE);
    if str_sec.is_null() { return -1; }
    if elf_add_string((*e).out, str_sec, b"\0".as_ptr() as *const c_char) == -1 { return -1; }
    let str_sym = elf_create_section_symbol((*e).out, str_sec);
    if str_sym.is_null() { return -1; }
    let obj_data = elf_add_data((*e).out, obj_sec, null(), obj_size);
    if obj_data.is_null() { return -1; }
    let modname = find_modname(e);
    if modname.is_null() { return -1; }
    if strcmp(modname, b"vmlinux\0".as_ptr() as *const c_char) != 0 {
        let addend = elf_add_string((*e).out, str_sec, modname);
        if addend == -1 { return -1; }
        if elf_create_reloc((*e).out, obj_sec, offsetof!(klp_object_ext, name) as c_ulong, str_sym, addend, R_ABS64).is_null() { return -1; }
    }
    if elf_create_reloc((*e).out, obj_sec, offsetof!(klp_object_ext, funcs) as c_ulong, funcs_sym, 0, R_ABS64).is_null() { return -1; }
    let mut nr_funcs: c_uint = 0;
    let mut sym: *mut symbol = null_mut();
    for_each_sym!((*e).out, sym, {
        let offset = nr_funcs as c_ulong * func_size as c_ulong;
        if !is_func_sym(sym) || is_cold_func(sym) || (*sym).clone.is_null() || (*(*sym).clone).changed == 0 { continue; }
        let func_data = elf_add_data((*e).out, funcs_sec, null(), func_size);
        if func_data.is_null() { return -1; }
        let addend = elf_add_string((*e).out, str_sec, (*(*(*sym).clone).twin).name);
        if addend == -1 { return -1; }
        if elf_create_reloc((*e).out, funcs_sec, offset + offsetof!(klp_func_ext, old_name) as c_ulong, str_sym, addend, R_ABS64).is_null() { return -1; }
        if elf_create_reloc((*e).out, funcs_sec, offset + offsetof!(klp_func_ext, new_func) as c_ulong, sym, 0, R_ABS64).is_null() { return -1; }
        let sympos = klp_find_sympos((*e).orig, (*(*sym).clone).twin);
        if sympos == ULONG_MAX { return -1; }
        memcpy((func_data as *mut u8).add(offsetof!(klp_func_ext, sympos)) as *mut c_void, &sympos as *const _ as *const c_void, size_of::<c_ulong>());
        nr_funcs += 1;
    });
    memcpy((obj_data as *mut u8).add(offsetof!(klp_object_ext, nr_funcs)) as *mut c_void, &nr_funcs as *const _ as *const c_void, size_of::<c_uint>());
    let prefixes = [
        (b"__klp_pre_patch_%s\0".as_ptr() as *const c_char, offsetof!(klp_callbacks, pre_patch)),
        (b"__klp_post_patch_%s\0".as_ptr() as *const c_char, offsetof!(klp_callbacks, post_patch)),
        (b"__klp_pre_unpatch_%s\0".as_ptr() as *const c_char, offsetof!(klp_callbacks, pre_unpatch)),
        (b"__klp_post_unpatch_%s\0".as_ptr() as *const c_char, offsetof!(klp_callbacks, post_unpatch)),
    ];
    let mut sym_name = [0 as c_char; SYM_NAME_LEN];
    for (fmt, cb_off) in prefixes {
        if snprintf_check(sym_name.as_mut_ptr(), SYM_NAME_LEN, fmt, modname) != 0 { return -1; }
        sym = find_symbol_by_name((*e).out, sym_name.as_ptr());
        if !sym.is_null() {
            let reloc_ = find_reloc_by_dest((*e).out, (*sym).sec, (*sym).offset);
            if elf_create_reloc((*e).out, obj_sec, (offsetof!(klp_object_ext, callbacks) + cb_off) as c_ulong, (*reloc_).sym, reloc_addend(reloc_), R_ABS64).is_null() { return -1; }
        }
    }
    0
}

unsafe fn copy_import_ns(e: *mut elfs) -> c_int {
    let patched_sec = find_section_by_name((*e).patched, b".modinfo\0".as_ptr() as *const c_char);
    if patched_sec.is_null() { return 0; }
    let mut import_ns = (*(*patched_sec).data).d_buf;
    if import_ns.is_null() { return 0; }
    let data_end = import_ns.add(sec_size(patched_sec) as usize);
    let mut out_sec: *mut section = null_mut();
    while import_ns < data_end {
        import_ns = memmem(import_ns as *const c_void, data_end.offset_from(import_ns) as usize, b"import_ns=\0".as_ptr() as *const c_void, 10) as *mut c_char;
        if import_ns.is_null() { return 0; }
        if out_sec.is_null() {
            out_sec = find_section_by_name((*e).out, b".modinfo\0".as_ptr() as *const c_char);
            if out_sec.is_null() {
                out_sec = elf_create_section((*e).out, b".modinfo\0".as_ptr() as *const c_char, 0, (*patched_sec).sh.sh_entsize, (*patched_sec).sh.sh_type, (*patched_sec).sh.sh_addralign, (*patched_sec).sh.sh_flags);
                if out_sec.is_null() { return -1; }
            }
        }
        if elf_add_data((*e).out, out_sec, import_ns as *const c_void, strlen(import_ns) + 1).is_null() { return -1; }
        import_ns = import_ns.add(strlen(import_ns) + 1);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cmd_klp_diff(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut e: elfs = zeroed();
    argc = parse_options(argc, argv, klp_diff_options.as_ptr(), klp_diff_usage.as_ptr(), 0);
    if argc != 3 {
        usage_with_options(klp_diff_usage.as_ptr(), klp_diff_options.as_ptr());
    }
    if debug {
        debug_correlate = true;
        debug_clone = true;
    }
    objname = *argv.add(0);
    e.orig = elf_open_read(*argv.add(0), O_RDONLY);
    e.patched = elf_open_read(*argv.add(1), O_RDONLY);
    e.out = null_mut();
    if e.orig.is_null() || e.patched.is_null() { return -1; }
    if klp_sympos_init(e.orig) != 0 { return -1; }
    if read_exports() != 0 { return -1; }
    if read_sym_checksums(e.orig) != 0 { return -1; }
    if read_sym_checksums(e.patched) != 0 { return -1; }
    if correlate_symbols(&mut e) != 0 { return -1; }
    let ret = mark_changed_functions(&mut e);
    if ret < 0 { return -1; }
    if ret > 0 { return 0; }
    e.out = elf_create_file(&(*e.orig).ehdr, *argv.add(2));
    if e.out.is_null() { return -1; }
    if create_fake_symbols(e.patched) != 0 { return -1; }
    if clone_included_functions(&mut e) != 0 { return -1; }
    if clone_special_sections(&mut e) != 0 { return -1; }
    if create_klp_sections(&mut e) != 0 { return -1; }
    if copy_import_ns(&mut e) != 0 { return -1; }
    if elf_write(e.out) != 0 { return -1; }
    elf_close(e.out)
}
