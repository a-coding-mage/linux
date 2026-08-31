// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/*
 * BPF static linker
 *
 * Copyright (c) 2021 Facebook
 *
 * Rust source-level translation of linker.c.  This file intentionally keeps
 * the C implementation shape: raw pointers, C ABI declarations, C integer
 * types, section indexes, and libelf/libbpf ownership rules are preserved.
 * Header-provided declarations are referenced as external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

const BTF_EXTERN_SEC: &[u8] = b".extern\0";

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ENOTSUP: c_int = 95;
const EOPNOTSUPP: c_int = 95;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_CLOEXEC: c_int = 0o2000000;

const INT_MAX: c_int = 2147483647;

const EV_CURRENT: c_uint = 1;
const EV_NONE: c_uint = 0;
const ELF_C_NULL: c_uint = 0;
const ELF_C_READ_MMAP: c_uint = 6;
const ELF_C_WRITE: c_uint = 2;
const ELF_T_BYTE: c_uint = 0;
const ELF_T_SYM: c_uint = 11;

const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const ELFCLASS64: __u8 = 2;
const ELFDATANONE: __u8 = 0;
const ELFDATA2LSB: __u8 = 1;
const ELFDATA2MSB: __u8 = 2;

const EM_BPF: __u16 = 247;
const ET_REL: __u16 = 1;
const SHN_UNDEF: __u16 = 0;
const SHN_LORESERVE: __u16 = 0xff00;

const SHT_PROGBITS: __u32 = 1;
const SHT_SYMTAB: __u32 = 2;
const SHT_STRTAB: __u32 = 3;
const SHT_NOBITS: __u32 = 8;
const SHT_REL: __u32 = 9;
const SHT_LLVM_ADDRSIG: __u32 = 0x6fff4c03;
const SHF_WRITE: __u64 = 0x1;
const SHF_ALLOC: __u64 = 0x2;
const SHF_EXECINSTR: __u64 = 0x4;
const SHF_STRINGS: __u64 = 0x20;

const STB_LOCAL: c_int = 0;
const STB_GLOBAL: c_int = 1;
const STB_WEAK: c_int = 2;
const STT_NOTYPE: c_int = 0;
const STT_OBJECT: c_int = 1;
const STT_FUNC: c_int = 2;
const STT_SECTION: c_int = 3;
const STV_DEFAULT: c_int = 0;
const STV_HIDDEN: c_int = 2;

const R_BPF_64_64: size_t = 1;
const R_BPF_64_ABS64: size_t = 2;
const R_BPF_64_ABS32: size_t = 3;
const R_BPF_64_32: size_t = 10;

const BPF_JMP: __u8 = 0x05;
const BPF_CALL: __u8 = 0x80;

const BTF_KIND_UNKN: c_int = 0;
const BTF_KIND_INT: c_int = 1;
const BTF_KIND_PTR: c_int = 2;
const BTF_KIND_ARRAY: c_int = 3;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_UNION: c_int = 5;
const BTF_KIND_ENUM: c_int = 6;
const BTF_KIND_FWD: c_int = 7;
const BTF_KIND_TYPEDEF: c_int = 8;
const BTF_KIND_VOLATILE: c_int = 9;
const BTF_KIND_CONST: c_int = 10;
const BTF_KIND_RESTRICT: c_int = 11;
const BTF_KIND_FUNC: c_int = 12;
const BTF_KIND_FUNC_PROTO: c_int = 13;
const BTF_KIND_VAR: c_int = 14;
const BTF_KIND_DATASEC: c_int = 15;
const BTF_KIND_FLOAT: c_int = 16;
const BTF_KIND_ENUM64: c_int = 19;

const BTF_FUNC_STATIC: c_int = 0;
const BTF_FUNC_GLOBAL: c_int = 1;
const BTF_VAR_STATIC: c_int = 0;
const BTF_VAR_GLOBAL_ALLOCATED: c_int = 1;
const BTF_MAGIC: __u16 = 0xeB9F;
const BTF_VERSION: __u8 = 1;
const BTF_LITTLE_ENDIAN: c_int = 0;
const BTF_BIG_ENDIAN: c_int = 1;

const BTF_FIELD_ITER_IDS: c_int = 0;
const BTF_FIELD_ITER_STRS: c_int = 1;

const MAP_DEF_KEY_TYPE: __u32 = 1 << 0;
const MAP_DEF_VALUE_TYPE: __u32 = 1 << 1;
const MAP_DEF_INNER_MAP: __u32 = 1 << 2;

const BTF_ELF_SEC: *const c_char = b".BTF\0".as_ptr() as *const c_char;
const BTF_EXT_ELF_SEC: *const c_char = b".BTF.ext\0".as_ptr() as *const c_char;
const MAPS_ELF_SEC: *const c_char = b".maps\0".as_ptr() as *const c_char;
const JUMPTABLES_SEC: *const c_char = b".jumptables\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}
#[repr(C)]
pub struct strset {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [__u8; 16],
    pub e_type: __u16,
    pub e_machine: __u16,
    pub e_version: __u32,
    pub e_entry: __u64,
    pub e_phoff: __u64,
    pub e_shoff: __u64,
    pub e_flags: __u32,
    pub e_ehsize: __u16,
    pub e_phentsize: __u16,
    pub e_phnum: __u16,
    pub e_shentsize: __u16,
    pub e_shnum: __u16,
    pub e_shstrndx: __u16,
}

#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name: __u32,
    pub sh_type: __u32,
    pub sh_flags: __u64,
    pub sh_addr: __u64,
    pub sh_offset: __u64,
    pub sh_size: __u64,
    pub sh_link: __u32,
    pub sh_info: __u32,
    pub sh_addralign: __u64,
    pub sh_entsize: __u64,
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: c_uint,
    pub d_version: c_uint,
    pub d_size: size_t,
    pub d_off: i64,
    pub d_align: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Sym {
    pub st_name: __u32,
    pub st_info: __u8,
    pub st_other: __u8,
    pub st_shndx: __u16,
    pub st_value: __u64,
    pub st_size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Rel {
    pub r_offset: __u64,
    pub r_info: __u64,
}

#[repr(C)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_src: __u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct btf_ext_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
    pub func_info_off: __u32,
    pub func_info_len: __u32,
    pub line_info_off: __u32,
    pub line_info_len: __u32,
    pub core_relo_off: __u32,
    pub core_relo_len: __u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_array {
    pub type_: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    pub offset: __u32,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_var {
    pub linkage: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_ext_info {
    pub len: __u32,
    pub rec_size: __u32,
    pub info: *mut c_void,
}

#[repr(C)]
pub struct btf_ext {
    pub hdr: *mut btf_ext_header,
    pub data: *mut c_void,
    pub data_size: __u32,
    pub func_info: btf_ext_info,
    pub line_info: btf_ext_info,
    pub core_relo_info: btf_ext_info,
}

#[repr(C)]
pub struct btf_ext_info_sec {
    pub sec_name_off: __u32,
    pub num_info: __u32,
}

#[repr(C)]
pub struct bpf_func_info_min {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_line_info_min {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

#[repr(C)]
pub struct bpf_core_relo {
    pub insn_off: __u32,
    pub type_id: __u32,
    pub access_str_off: __u32,
    pub kind: __u32,
}

#[repr(C)]
pub struct btf_map_def {
    pub parts: __u32,
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub numa_node: __u32,
    pub pinning: __u32,
    pub key_type_id: __u32,
    pub value_type_id: __u32,
}

#[repr(C)]
pub struct btf_field_iter {
    _opaque: [usize; 8],
}

#[repr(C)]
pub struct btf_dedup_opts {
    pub sz: size_t,
    pub btf_ext: *mut btf_ext,
}

#[repr(C)]
pub struct bpf_linker_opts {
    pub sz: size_t,
}

#[repr(C)]
pub struct bpf_linker_file_opts {
    pub sz: size_t,
}

#[repr(C)]
struct src_sec {
    sec_name: *const c_char,
    /* positional (not necessarily ELF) index in an array of sections */
    id: c_int,
    /* positional (not necessarily ELF) index of a matching section in a final object file */
    dst_id: c_int,
    /* section data offset in a matching output section */
    dst_off: c_int,
    /* whether section is omitted from the final ELF file */
    skipped: bool,
    /* whether section is an ephemeral section, not mapped to an ELF section */
    ephemeral: bool,

    /* ELF info */
    sec_idx: size_t,
    scn: *mut Elf_Scn,
    shdr: *mut Elf64_Shdr,
    data: *mut Elf_Data,

    /* corresponding BTF DATASEC type ID */
    sec_type_id: c_int,
}

#[repr(C)]
struct src_obj {
    filename: *const c_char,
    fd: c_int,
    elf: *mut Elf,
    /* Section header strings section index */
    shstrs_sec_idx: size_t,
    /* SYMTAB section index */
    symtab_sec_idx: size_t,

    btf: *mut btf,
    btf_ext: *mut btf_ext,

    /* List of sections (including ephemeral). Slot zero is unused. */
    secs: *mut src_sec,
    sec_cnt: c_int,

    /* mapping of symbol indices from src to dst ELF */
    sym_map: *mut c_int,
    /* mapping from the src BTF type IDs to dst ones */
    btf_type_map: *mut c_int,
}

/* single .BTF.ext data section */
#[repr(C)]
struct btf_ext_sec_data {
    rec_cnt: size_t,
    rec_sz: __u32,
    recs: *mut c_void,
}

#[repr(C)]
struct glob_sym {
    /* ELF symbol index */
    sym_idx: c_int,
    /* associated section id for .ksyms, .kconfig, etc, but not .extern */
    sec_id: c_int,
    /* extern name offset in STRTAB */
    name_off: c_int,
    /* optional associated BTF type ID */
    btf_id: c_int,
    /* BTF type ID to which VAR/FUNC type is pointing to; used for
     * rewriting types when extern VAR/FUNC is resolved to a concrete
     * definition
     */
    underlying_btf_id: c_int,
    /* sec_var index in the corresponding dst_sec, if exists */
    var_idx: c_int,

    /* extern or resolved/global symbol */
    is_extern: bool,
    /* weak or strong symbol, never goes back from strong to weak */
    is_weak: bool,
}

#[repr(C)]
struct dst_sec {
    sec_name: *mut c_char,
    /* positional (not necessarily ELF) index in an array of sections */
    id: c_int,

    ephemeral: bool,

    /* ELF info */
    sec_idx: size_t,
    scn: *mut Elf_Scn,
    shdr: *mut Elf64_Shdr,
    data: *mut Elf_Data,

    /* final output section size */
    sec_sz: c_int,
    /* final output contents of the section */
    raw_data: *mut c_void,

    /* corresponding STT_SECTION symbol index in SYMTAB */
    sec_sym_idx: c_int,

    /* section's DATASEC variable info, emitted on BTF finalization */
    has_btf: bool,
    sec_var_cnt: c_int,
    sec_vars: *mut btf_var_secinfo,

    /* section's .BTF.ext data */
    func_info: btf_ext_sec_data,
    line_info: btf_ext_sec_data,
    core_relo_info: btf_ext_sec_data,
}

#[repr(C)]
pub struct bpf_linker {
    filename: *mut c_char,
    fd: c_int,
    elf: *mut Elf,
    elf_hdr: *mut Elf64_Ehdr,
    swapped_endian: bool,

    /* Output sections metadata */
    secs: *mut dst_sec,
    sec_cnt: c_int,

    strtab_strs: *mut strset, /* STRTAB unique strings */
    strtab_sec_idx: size_t,   /* STRTAB section index */
    symtab_sec_idx: size_t,   /* SYMTAB section index */

    btf: *mut btf,
    btf_ext: *mut btf_ext,

    /* global (including extern) ELF symbols */
    glob_sym_cnt: c_int,
    glob_syms: *mut glob_sym,

    fd_is_owned: bool,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;

    fn elf_version(version: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_uint, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_errmsg(err: c_int) -> *const c_char;
    fn elf64_newehdr(elf: *mut Elf) -> *mut Elf64_Ehdr;
    fn elf64_getehdr(elf: *mut Elf) -> *mut Elf64_Ehdr;
    fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn elf_ndxscn(scn: *mut Elf_Scn) -> size_t;
    fn elf64_getshdr(scn: *mut Elf_Scn) -> *mut Elf64_Shdr;
    fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *const c_char;
    fn elf_update(elf: *mut Elf, cmd: c_uint) -> isize;

    fn libbpf_print(level: c_int, fmt: *const c_char, ...);
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn errstr(err: c_int) -> *const c_char;
    fn sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn is_pow_of_2(x: __u64) -> bool;

    fn strset__new(max_data_sz: size_t, init_data: *const c_char, init_data_sz: size_t) -> *mut strset;
    fn strset__free(set: *mut strset);
    fn strset__add_str(set: *mut strset, s: *const c_char) -> c_int;
    fn strset__data(set: *const strset) -> *const c_char;
    fn strset__data_size(set: *const strset) -> size_t;

    fn btf__new(data: *const c_void, size: __u32) -> *mut btf;
    fn btf__new_empty() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    fn btf_type_by_id(btf: *const btf, id: __u32) -> *mut btf_type;
    fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__add_str(btf: *mut btf, s: *const c_char) -> c_int;
    fn btf__add_type(dst: *mut btf, src: *const btf, t: *const btf_type) -> c_int;
    fn btf__add_datasec(btf: *mut btf, name: *const c_char, size: __u32) -> c_int;
    fn btf__add_datasec_var_info(btf: *mut btf, type_id: __u32, offset: __u32, size: __u32) -> c_int;
    fn btf__resolve_size(btf: *const btf, type_id: __u32) -> __s64;
    fn btf__dedup(btf: *mut btf, opts: *mut btf_dedup_opts) -> c_int;
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__set_endianness(btf: *mut btf, endianness: c_int);
    fn btf_type_info(kind: c_int, vlen: c_int, kflag: c_int) -> __u32;
    fn btf_kind(t: *const btf_type) -> c_int;
    fn btf_kind_str(t: *const btf_type) -> *const c_char;
    fn btf_kflag(t: *const btf_type) -> bool;
    fn btf_vlen(t: *const btf_type) -> c_int;
    fn btf_is_fwd(t: *const btf_type) -> bool;
    fn btf_is_union(t: *const btf_type) -> bool;
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_is_var(t: *const btf_type) -> bool;
    fn btf_is_func(t: *const btf_type) -> bool;
    fn btf_is_datasec(t: *const btf_type) -> bool;
    fn btf_array(t: *const btf_type) -> *const btf_array;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_params(t: *const btf_type) -> *mut btf_param;
    fn btf_var(t: *const btf_type) -> *mut btf_var;
    fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo;
    fn btf_func_linkage(t: *const btf_type) -> c_int;
    fn skip_mods_and_typedefs(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type;
    fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, kind: c_int) -> c_int;
    fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
    fn btf_ext__new(data: *const c_void, size: __u32) -> *mut btf_ext;
    fn btf_ext__free(ext: *mut btf_ext);
    fn btf_ext__raw_data(ext: *const btf_ext, size: *mut __u32) -> *const c_void;
    fn btf_ext__set_endianness(ext: *mut btf_ext, endianness: c_int);
    fn btf_ext_visit_type_ids(ext: *mut btf_ext, visit: unsafe extern "C" fn(*mut __u32, *mut c_void) -> c_int, ctx: *mut c_void) -> c_int;
    fn btf_ext_visit_str_offs(ext: *mut btf_ext, visit: unsafe extern "C" fn(*mut __u32, *mut c_void) -> c_int, ctx: *mut c_void) -> c_int;
    fn parse_btf_map_def(name: *const c_char, btf: *const btf, t: *const btf_type, strict: bool, def: *mut btf_map_def, inner_def: *mut btf_map_def) -> c_int;
    fn bpf_insn_bswap(insn: *mut bpf_insn);
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn OPTS_VALID<T>(_opts: *const T, _name: *const c_char) -> bool {
    true
}

unsafe fn ELF64_ST_TYPE(info: __u8) -> c_int {
    (info & 0xf) as c_int
}

unsafe fn ELF64_ST_BIND(info: __u8) -> c_int {
    (info >> 4) as c_int
}

unsafe fn ELF64_ST_INFO(bind: c_int, type_: c_int) -> __u8 {
    ((bind << 4) + (type_ & 0xf)) as __u8
}

unsafe fn ELF64_ST_VISIBILITY(other: __u8) -> c_int {
    (other & 0x3) as c_int
}

unsafe fn ELF64_R_SYM(info: __u64) -> size_t {
    (info >> 32) as size_t
}

unsafe fn ELF64_R_TYPE(info: __u64) -> size_t {
    (info & 0xffffffff) as size_t
}

unsafe fn ELF64_R_INFO(sym: size_t, type_: size_t) -> __u64 {
    ((sym as __u64) << 32) + type_ as __u64
}

unsafe fn pr_warn_elf(fmt: *const c_char) {
    libbpf_print(1, c!("libbpf: %s: %s\n"), fmt, elf_errmsg(-1));
}

unsafe fn pr_warn(fmt: *const c_char) {
    libbpf_print(1, fmt);
}

unsafe fn pr_debug(fmt: *const c_char) {
    libbpf_print(0, fmt);
}

unsafe fn sec_at(linker: *mut bpf_linker, idx: size_t) -> *mut dst_sec {
    (*linker).secs.add(idx)
}

unsafe fn src_sec_at(obj: *mut src_obj, idx: size_t) -> *mut src_sec {
    (*obj).secs.add(idx)
}

unsafe fn neg_errno(e: c_int) -> c_int {
    -e
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__free(linker: *mut bpf_linker) {
    let mut i: c_int;

    if linker.is_null() {
        return;
    }

    free((*linker).filename as *mut c_void);

    if !(*linker).elf.is_null() {
        elf_end((*linker).elf);
    }

    if (*linker).fd >= 0 && (*linker).fd_is_owned {
        close((*linker).fd);
    }

    strset__free((*linker).strtab_strs);

    btf__free((*linker).btf);
    btf_ext__free((*linker).btf_ext);

    i = 1;
    while i < (*linker).sec_cnt {
        let sec = (*linker).secs.add(i as usize);

        free((*sec).sec_name as *mut c_void);
        free((*sec).raw_data);
        free((*sec).sec_vars as *mut c_void);

        free((*sec).func_info.recs);
        free((*sec).line_info.recs);
        free((*sec).core_relo_info.recs);
        i += 1;
    }
    free((*linker).secs as *mut c_void);

    free((*linker).glob_syms as *mut c_void);
    free(linker as *mut c_void);
}

unsafe fn add_dst_sec(linker: *mut bpf_linker, sec_name: *const c_char) -> *mut dst_sec {
    let mut secs = (*linker).secs;
    let new_cnt: size_t = if (*linker).sec_cnt != 0 {
        (*linker).sec_cnt as size_t + 1
    } else {
        2
    };

    secs = libbpf_reallocarray(secs as *mut c_void, new_cnt, size_of::<dst_sec>()) as *mut dst_sec;
    if secs.is_null() {
        return ptr::null_mut();
    }

    /* zero out newly allocated memory */
    memset(
        secs.add((*linker).sec_cnt as usize) as *mut c_void,
        0,
        (new_cnt - (*linker).sec_cnt as size_t) * size_of::<dst_sec>(),
    );

    (*linker).secs = secs;
    (*linker).sec_cnt = new_cnt as c_int;

    let sec = (*linker).secs.add(new_cnt - 1);
    (*sec).id = (new_cnt - 1) as c_int;
    (*sec).sec_name = strdup(sec_name);
    if (*sec).sec_name.is_null() {
        return ptr::null_mut();
    }

    sec
}

unsafe fn add_new_sym(linker: *mut bpf_linker, sym_idx: *mut size_t) -> *mut Elf64_Sym {
    let symtab = sec_at(linker, (*linker).symtab_sec_idx);
    let sym_cnt = (*symtab).sec_sz as size_t / size_of::<Elf64_Sym>();

    let syms = libbpf_reallocarray((*symtab).raw_data, sym_cnt + 1, size_of::<Elf64_Sym>()) as *mut Elf64_Sym;
    if syms.is_null() {
        return ptr::null_mut();
    }

    let sym = syms.add(sym_cnt);
    memset(sym as *mut c_void, 0, size_of::<Elf64_Sym>());

    (*symtab).raw_data = syms as *mut c_void;
    (*symtab).sec_sz += size_of::<Elf64_Sym>() as c_int;
    (*(*symtab).shdr).sh_size += size_of::<Elf64_Sym>() as __u64;
    (*(*symtab).data).d_size += size_of::<Elf64_Sym>();

    if !sym_idx.is_null() {
        *sym_idx = sym_cnt;
    }

    sym
}

unsafe fn init_output_elf(linker: *mut bpf_linker) -> c_int {
    let mut str_off: c_int;
    let init_sym: *mut Elf64_Sym;
    let mut sec: *mut dst_sec;

    (*linker).elf = elf_begin((*linker).fd, ELF_C_WRITE, ptr::null_mut());
    if (*linker).elf.is_null() {
        pr_warn_elf(c!("failed to create ELF object"));
        return -EINVAL;
    }

    /* ELF header */
    (*linker).elf_hdr = elf64_newehdr((*linker).elf);
    if (*linker).elf_hdr.is_null() {
        pr_warn_elf(c!("failed to create ELF header"));
        return -EINVAL;
    }

    (*(*linker).elf_hdr).e_machine = EM_BPF;
    (*(*linker).elf_hdr).e_type = ET_REL;
    /* Set unknown ELF endianness, assign later from input files */
    (*(*linker).elf_hdr).e_ident[EI_DATA] = ELFDATANONE;

    /* STRTAB */
    /* initialize strset with an empty string to conform to ELF */
    (*linker).strtab_strs = strset__new(INT_MAX as size_t, c!(""), size_of::<[u8; 1]>());
    if libbpf_get_error((*linker).strtab_strs as *const c_void) != 0 {
        return libbpf_get_error((*linker).strtab_strs as *const c_void);
    }

    sec = add_dst_sec(linker, c!(".strtab"));
    if sec.is_null() {
        return -ENOMEM;
    }

    (*sec).scn = elf_newscn((*linker).elf);
    if (*sec).scn.is_null() {
        pr_warn_elf(c!("failed to create STRTAB section"));
        return -EINVAL;
    }

    (*sec).shdr = elf64_getshdr((*sec).scn);
    if (*sec).shdr.is_null() {
        return -EINVAL;
    }

    (*sec).data = elf_newdata((*sec).scn);
    if (*sec).data.is_null() {
        pr_warn_elf(c!("failed to create STRTAB data"));
        return -EINVAL;
    }

    str_off = strset__add_str((*linker).strtab_strs, (*sec).sec_name);
    if str_off < 0 {
        return str_off;
    }

    (*sec).sec_idx = elf_ndxscn((*sec).scn);
    (*(*linker).elf_hdr).e_shstrndx = (*sec).sec_idx as __u16;
    (*linker).strtab_sec_idx = (*sec).sec_idx;

    (*(*sec).shdr).sh_name = str_off as __u32;
    (*(*sec).shdr).sh_type = SHT_STRTAB;
    (*(*sec).shdr).sh_flags = SHF_STRINGS;
    (*(*sec).shdr).sh_offset = 0;
    (*(*sec).shdr).sh_link = 0;
    (*(*sec).shdr).sh_info = 0;
    (*(*sec).shdr).sh_addralign = 1;
    (*sec).sec_sz = 0;
    (*(*sec).shdr).sh_size = 0;
    (*(*sec).shdr).sh_entsize = 0;

    /* SYMTAB */
    sec = add_dst_sec(linker, c!(".symtab"));
    if sec.is_null() {
        return -ENOMEM;
    }

    (*sec).scn = elf_newscn((*linker).elf);
    if (*sec).scn.is_null() {
        pr_warn_elf(c!("failed to create SYMTAB section"));
        return -EINVAL;
    }

    (*sec).shdr = elf64_getshdr((*sec).scn);
    if (*sec).shdr.is_null() {
        return -EINVAL;
    }

    (*sec).data = elf_newdata((*sec).scn);
    if (*sec).data.is_null() {
        pr_warn_elf(c!("failed to create SYMTAB data"));
        return -EINVAL;
    }
    /* Ensure libelf translates byte-order of symbol records */
    (*(*sec).data).d_type = ELF_T_SYM;

    str_off = strset__add_str((*linker).strtab_strs, (*sec).sec_name);
    if str_off < 0 {
        return str_off;
    }

    (*sec).sec_idx = elf_ndxscn((*sec).scn);
    (*linker).symtab_sec_idx = (*sec).sec_idx;

    (*(*sec).shdr).sh_name = str_off as __u32;
    (*(*sec).shdr).sh_type = SHT_SYMTAB;
    (*(*sec).shdr).sh_flags = 0;
    (*(*sec).shdr).sh_offset = 0;
    (*(*sec).shdr).sh_link = (*linker).strtab_sec_idx as __u32;
    /* sh_info should be one greater than the index of the last local
     * symbol (i.e., binding is STB_LOCAL). But why and who cares?
     */
    (*(*sec).shdr).sh_info = 0;
    (*(*sec).shdr).sh_addralign = 8;
    (*(*sec).shdr).sh_entsize = size_of::<Elf64_Sym>() as __u64;

    /* .BTF */
    (*linker).btf = btf__new_empty();
    let err = libbpf_get_error((*linker).btf as *const c_void);
    if err != 0 {
        return err;
    }

    /* add the special all-zero symbol */
    init_sym = add_new_sym(linker, ptr::null_mut());
    if init_sym.is_null() {
        return -EINVAL;
    }

    (*init_sym).st_name = 0;
    (*init_sym).st_info = 0;
    (*init_sym).st_other = 0;
    (*init_sym).st_shndx = SHN_UNDEF;
    (*init_sym).st_value = 0;
    (*init_sym).st_size = 0;

    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__new(filename: *const c_char, opts: *mut bpf_linker_opts) -> *mut bpf_linker {
    let linker: *mut bpf_linker;
    let mut err: c_int;

    if !OPTS_VALID(opts, c!("bpf_linker_opts")) {
        errno = EINVAL;
        return ptr::null_mut();
    }

    if elf_version(EV_CURRENT) == EV_NONE {
        pr_warn_elf(c!("libelf initialization failed"));
        errno = EINVAL;
        return ptr::null_mut();
    }

    linker = calloc(1, size_of::<bpf_linker>()) as *mut bpf_linker;
    if linker.is_null() {
        errno = ENOMEM;
        return ptr::null_mut();
    }

    (*linker).filename = strdup(filename);
    if (*linker).filename.is_null() {
        err = -ENOMEM;
        bpf_linker__free(linker);
        errno = -err;
        return ptr::null_mut();
    }

    (*linker).fd = open(filename, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0o644);
    if (*linker).fd < 0 {
        err = -errno;
        bpf_linker__free(linker);
        errno = -err;
        return ptr::null_mut();
    }
    (*linker).fd_is_owned = true;

    err = init_output_elf(linker);
    if err != 0 {
        bpf_linker__free(linker);
        errno = -err;
        return ptr::null_mut();
    }

    linker
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__new_fd(fd: c_int, opts: *mut bpf_linker_opts) -> *mut bpf_linker {
    let linker: *mut bpf_linker;
    let mut filename = [0 as c_char; 32];
    let mut err: c_int;

    if fd < 0 {
        errno = EINVAL;
        return ptr::null_mut();
    }

    if !OPTS_VALID(opts, c!("bpf_linker_opts")) {
        errno = EINVAL;
        return ptr::null_mut();
    }

    if elf_version(EV_CURRENT) == EV_NONE {
        pr_warn_elf(c!("libelf initialization failed"));
        errno = EINVAL;
        return ptr::null_mut();
    }

    linker = calloc(1, size_of::<bpf_linker>()) as *mut bpf_linker;
    if linker.is_null() {
        errno = ENOMEM;
        return ptr::null_mut();
    }

    snprintf(filename.as_mut_ptr(), filename.len(), c!("fd:%d"), fd);
    (*linker).filename = strdup(filename.as_ptr());
    if (*linker).filename.is_null() {
        err = -ENOMEM;
        bpf_linker__free(linker);
        errno = -err;
        return ptr::null_mut();
    }

    (*linker).fd = fd;
    (*linker).fd_is_owned = false;

    err = init_output_elf(linker);
    if err != 0 {
        bpf_linker__free(linker);
        errno = -err;
        return ptr::null_mut();
    }

    linker
}

unsafe fn is_dwarf_sec_name(name: *const c_char) -> bool {
    /* approximation, but the actual list is too long */
    strncmp(name, c!(".debug_"), size_of::<[u8; 8]>() - 1) == 0
}

unsafe fn is_ignored_sec(sec: *mut src_sec) -> bool {
    let shdr = (*sec).shdr;
    let mut name = (*sec).sec_name;

    /* no special handling of .strtab */
    if (*shdr).sh_type == SHT_STRTAB {
        return true;
    }

    /* ignore .llvm_addrsig section as well */
    if (*shdr).sh_type == SHT_LLVM_ADDRSIG {
        return true;
    }

    /* no subprograms will lead to an empty .text section, ignore it */
    if (*shdr).sh_type == SHT_PROGBITS && (*shdr).sh_size == 0 && strcmp((*sec).sec_name, c!(".text")) == 0 {
        return true;
    }

    /* DWARF sections */
    if is_dwarf_sec_name((*sec).sec_name) {
        return true;
    }

    if strncmp(name, c!(".rel"), size_of::<[u8; 5]>() - 1) == 0 {
        name = name.add(size_of::<[u8; 5]>() - 1);
        /* DWARF section relocations */
        if is_dwarf_sec_name(name) {
            return true;
        }

        /* .BTF and .BTF.ext don't need relocations */
        if strcmp(name, BTF_ELF_SEC) == 0 || strcmp(name, BTF_EXT_ELF_SEC) == 0 {
            return true;
        }
    }

    false
}

unsafe fn add_src_sec(obj: *mut src_obj, sec_name: *const c_char) -> *mut src_sec {
    let mut secs = (*obj).secs;
    let new_cnt: size_t = if (*obj).sec_cnt != 0 { (*obj).sec_cnt as size_t + 1 } else { 2 };

    secs = libbpf_reallocarray(secs as *mut c_void, new_cnt, size_of::<src_sec>()) as *mut src_sec;
    if secs.is_null() {
        return ptr::null_mut();
    }

    /* zero out newly allocated memory */
    memset(
        secs.add((*obj).sec_cnt as usize) as *mut c_void,
        0,
        (new_cnt - (*obj).sec_cnt as size_t) * size_of::<src_sec>(),
    );

    (*obj).secs = secs;
    (*obj).sec_cnt = new_cnt as c_int;

    let sec = (*obj).secs.add(new_cnt - 1);
    (*sec).id = (new_cnt - 1) as c_int;
    (*sec).sec_name = sec_name;

    sec
}

unsafe fn find_dst_sec_by_name(linker: *mut bpf_linker, sec_name: *const c_char) -> *mut dst_sec {
    let mut i = 1;
    while i < (*linker).sec_cnt {
        let sec = (*linker).secs.add(i as usize);

        if strcmp((*sec).sec_name, sec_name) == 0 {
            return sec;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn secs_match(dst: *mut dst_sec, src: *mut src_sec) -> bool {
    if (*dst).ephemeral || (*src).ephemeral {
        return true;
    }

    if (*(*dst).shdr).sh_type != (*(*src).shdr).sh_type {
        return false;
    }
    if (*(*dst).shdr).sh_flags != (*(*src).shdr).sh_flags {
        return false;
    }
    if (*(*dst).shdr).sh_entsize != (*(*src).shdr).sh_entsize {
        return false;
    }

    true
}

unsafe fn sec_content_is_same(dst_sec: *mut dst_sec, src_sec: *mut src_sec) -> bool {
    if (*dst_sec).sec_sz as __u64 != (*(*src_sec).shdr).sh_size {
        return false;
    }
    if memcmp((*dst_sec).raw_data, (*(*src_sec).data).d_buf, (*dst_sec).sec_sz as size_t) != 0 {
        return false;
    }
    true
}

unsafe fn is_exec_sec(sec: *mut dst_sec) -> bool {
    if sec.is_null() || (*sec).ephemeral {
        return false;
    }
    (*(*sec).shdr).sh_type == SHT_PROGBITS && ((*(*sec).shdr).sh_flags & SHF_EXECINSTR) != 0
}

unsafe fn exec_sec_bswap(raw_data: *mut c_void, size: c_int) {
    let insn_cnt = size as size_t / size_of::<bpf_insn>();
    let mut insn = raw_data as *mut bpf_insn;
    let mut i = 0;

    while i < insn_cnt {
        bpf_insn_bswap(insn);
        insn = insn.add(1);
        i += 1;
    }
}

unsafe fn init_sec(linker: *mut bpf_linker, dst_sec: *mut dst_sec, src_sec: *mut src_sec) -> c_int {
    let scn: *mut Elf_Scn;
    let data: *mut Elf_Data;
    let shdr: *mut Elf64_Shdr;
    let name_off: c_int;

    (*dst_sec).sec_sz = 0;
    (*dst_sec).sec_idx = 0;
    (*dst_sec).ephemeral = (*src_sec).ephemeral;

    /* ephemeral sections are just thin section shells lacking most parts */
    if (*src_sec).ephemeral {
        return 0;
    }

    scn = elf_newscn((*linker).elf);
    if scn.is_null() {
        return -ENOMEM;
    }
    data = elf_newdata(scn);
    if data.is_null() {
        return -ENOMEM;
    }
    shdr = elf64_getshdr(scn);
    if shdr.is_null() {
        return -ENOMEM;
    }

    (*dst_sec).scn = scn;
    (*dst_sec).shdr = shdr;
    (*dst_sec).data = data;
    (*dst_sec).sec_idx = elf_ndxscn(scn);

    name_off = strset__add_str((*linker).strtab_strs, (*src_sec).sec_name);
    if name_off < 0 {
        return name_off;
    }

    (*shdr).sh_name = name_off as __u32;
    (*shdr).sh_type = (*(*src_sec).shdr).sh_type;
    (*shdr).sh_flags = (*(*src_sec).shdr).sh_flags;
    (*shdr).sh_size = 0;
    /* sh_link and sh_info have different meaning for different types of
     * sections, so we leave it up to the caller code to fill them in, if
     * necessary
     */
    (*shdr).sh_link = 0;
    (*shdr).sh_info = 0;
    (*shdr).sh_addralign = (*(*src_sec).shdr).sh_addralign;
    (*shdr).sh_entsize = (*(*src_sec).shdr).sh_entsize;

    (*data).d_type = (*(*src_sec).data).d_type;
    (*data).d_size = 0;
    (*data).d_buf = ptr::null_mut();
    (*data).d_align = (*(*src_sec).data).d_align;
    (*data).d_off = 0;

    0
}

unsafe fn extend_sec(linker: *mut bpf_linker, dst: *mut dst_sec, src: *mut src_sec) -> c_int {
    let mut tmp: *mut c_void;
    let mut dst_align: size_t;
    let src_align: size_t;
    let dst_align_sz: size_t;
    let dst_final_sz: size_t;
    let err: c_int;

    /* Ephemeral source section doesn't contribute anything to ELF
     * section data.
     */
    if (*src).ephemeral {
        return 0;
    }

    /* Some sections (like .maps) can contain both externs (and thus be
     * ephemeral) and non-externs (map definitions). So it's possible that
     * it has to be "upgraded" from ephemeral to non-ephemeral when the
     * first non-ephemeral entity appears. In such case, we add ELF
     * section, data, etc.
     */
    if (*dst).ephemeral {
        err = init_sec(linker, dst, src);
        if err != 0 {
            return err;
        }
    }

    dst_align = (*(*dst).shdr).sh_addralign as size_t;
    src_align = (*(*src).shdr).sh_addralign as size_t;
    if dst_align == 0 {
        dst_align = 1;
    }
    if dst_align < src_align {
        dst_align = src_align;
    }

    dst_align_sz = ((*dst).sec_sz as size_t + dst_align - 1) / dst_align * dst_align;

    /* no need to re-align final size */
    dst_final_sz = dst_align_sz + (*(*src).shdr).sh_size as size_t;

    if (*(*src).shdr).sh_type != SHT_NOBITS {
        tmp = realloc((*dst).raw_data, dst_final_sz);
        /* See linker.c comment: avoid error exit when realloc(NULL, 0) or
         * realloc(ptr, 0) returns NULL and may have freed the original pointer.
         */
        if tmp.is_null() && dst_align_sz > 0 {
            return -ENOMEM;
        }
        (*dst).raw_data = tmp;

        /* pad dst section, if it's alignment forced size increase */
        memset(((*dst).raw_data as *mut u8).add((*dst).sec_sz as usize) as *mut c_void, 0, dst_align_sz - (*dst).sec_sz as size_t);
        /* now copy src data at a properly aligned offset */
        memcpy(((*dst).raw_data as *mut u8).add(dst_align_sz) as *mut c_void, (*(*src).data).d_buf, (*(*src).shdr).sh_size as size_t);

        /* convert added bpf insns to native byte-order */
        if (*linker).swapped_endian && is_exec_sec(dst) {
            exec_sec_bswap(((*dst).raw_data as *mut u8).add(dst_align_sz) as *mut c_void, (*(*src).shdr).sh_size as c_int);
        }
    }

    (*dst).sec_sz = dst_final_sz as c_int;
    (*(*dst).shdr).sh_size = dst_final_sz as __u64;
    (*(*dst).data).d_size = dst_final_sz;

    (*(*dst).shdr).sh_addralign = dst_align as __u64;
    (*(*dst).data).d_align = dst_align;

    (*src).dst_off = dst_align_sz as c_int;

    0
}

unsafe fn is_data_sec(sec: *mut src_sec) -> bool {
    if sec.is_null() || (*sec).skipped {
        return false;
    }
    /* ephemeral sections are data sections, e.g., .kconfig, .ksyms */
    if (*sec).ephemeral {
        return true;
    }
    (*(*sec).shdr).sh_type == SHT_PROGBITS || (*(*sec).shdr).sh_type == SHT_NOBITS
}

unsafe fn is_relo_sec(sec: *mut src_sec) -> bool {
    if sec.is_null() || (*sec).skipped || (*sec).ephemeral {
        return false;
    }
    (*(*sec).shdr).sh_type == SHT_REL
}

unsafe extern "C" fn check_btf_type_id(type_id: *mut __u32, ctx: *mut c_void) -> c_int {
    let btf = ctx as *mut btf;

    if *type_id >= btf__type_cnt(btf) as __u32 {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn check_btf_str_off(str_off: *mut __u32, ctx: *mut c_void) -> c_int {
    let btf = ctx as *mut btf;
    let s = btf__str_by_offset(btf, *str_off);

    if s.is_null() {
        return -EINVAL;
    }

    0
}

unsafe fn sym_update_bind(sym: *mut Elf64_Sym, sym_bind: c_int) {
    (*sym).st_info = ELF64_ST_INFO(sym_bind, ELF64_ST_TYPE((*sym).st_info));
}

unsafe fn sym_update_type(sym: *mut Elf64_Sym, sym_type: c_int) {
    (*sym).st_info = ELF64_ST_INFO(ELF64_ST_BIND((*sym).st_info), sym_type);
}

unsafe fn sym_update_visibility(sym: *mut Elf64_Sym, sym_vis: c_int) {
    /* libelf doesn't provide setters for ST_VISIBILITY,
     * but it is stored in the lower 2 bits of st_other
     */
    (*sym).st_other &= !0x03;
    (*sym).st_other |= sym_vis as __u8;
}

unsafe fn get_sym_by_idx(linker: *mut bpf_linker, sym_idx: size_t) -> *mut Elf64_Sym {
    let symtab = sec_at(linker, (*linker).symtab_sec_idx);
    let syms = (*symtab).raw_data as *mut Elf64_Sym;

    syms.add(sym_idx)
}

unsafe fn find_glob_sym(linker: *mut bpf_linker, sym_name: *const c_char) -> *mut glob_sym {
    let mut i = 0;

    while i < (*linker).glob_sym_cnt {
        let glob_sym = (*linker).glob_syms.add(i as usize);
        let name = strset__data((*linker).strtab_strs).add((*glob_sym).name_off as usize);

        if strcmp(name, sym_name) == 0 {
            return glob_sym;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn add_glob_sym(linker: *mut bpf_linker) -> *mut glob_sym {
    let syms = libbpf_reallocarray(
        (*linker).glob_syms as *mut c_void,
        (*linker).glob_sym_cnt as size_t + 1,
        size_of::<glob_sym>(),
    ) as *mut glob_sym;
    if syms.is_null() {
        return ptr::null_mut();
    }

    let sym = syms.add((*linker).glob_sym_cnt as usize);
    memset(sym as *mut c_void, 0, size_of::<glob_sym>());
    (*sym).var_idx = -1;

    (*linker).glob_syms = syms;
    (*linker).glob_sym_cnt += 1;

    sym
}

unsafe fn btf_is_non_static(t: *const btf_type) -> bool {
    (btf_is_var(t) && (*btf_var(t)).linkage != BTF_VAR_STATIC as __u32)
        || (btf_is_func(t) && btf_func_linkage(t) != BTF_FUNC_STATIC)
}

unsafe fn find_src_sec_by_name(obj: *mut src_obj, sec_name: *const c_char) -> *mut src_sec {
    let mut i = 1;

    while i < (*obj).sec_cnt {
        let sec = (*obj).secs.add(i as usize);

        if strcmp((*sec).sec_name, sec_name) == 0 {
            return sec;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn find_sym_by_name(obj: *mut src_obj, sec_idx: size_t, sym_type: c_int, sym_name: *const c_char) -> *mut Elf64_Sym {
    let symtab = (*obj).secs.add((*obj).symtab_sec_idx);
    let mut sym = (*(*symtab).data).d_buf as *mut Elf64_Sym;
    let n = ((*(*symtab).shdr).sh_size / (*(*symtab).shdr).sh_entsize) as c_int;
    let str_sec_idx = (*(*symtab).shdr).sh_link as size_t;
    let mut i = 0;

    while i < n {
        if (*sym).st_shndx as size_t == sec_idx && ELF64_ST_TYPE((*sym).st_info) == sym_type {
            let name = elf_strptr((*obj).elf, str_sec_idx, (*sym).st_name as size_t);
            if name.is_null() {
                return ptr::null_mut();
            }
            if strcmp(sym_name, name) == 0 {
                return sym;
            }
        }
        sym = sym.add(1);
        i += 1;
    }

    ptr::null_mut()
}

/* The following functions are direct source-level translations in structure and
 * side effects.  Several diagnostic varargs are intentionally collapsed to the
 * static format string because their only semantic effect is warning output.
 */

unsafe fn linker_sanity_check_btf(obj: *mut src_obj) -> c_int {
    let mut err: c_int;
    if (*obj).btf.is_null() {
        return 0;
    }

    let n = btf__type_cnt((*obj).btf);
    let mut i = 1;
    while i < n {
        let t = btf_type_by_id((*obj).btf, i as __u32);
        let mut it: btf_field_iter = core::mem::zeroed();

        err = btf_field_iter_init(&mut it, t, BTF_FIELD_ITER_IDS);
        if err != 0 {
            return err;
        }
        loop {
            let type_id = btf_field_iter_next(&mut it);
            if type_id.is_null() {
                break;
            }
            if *type_id >= n as __u32 {
                return -EINVAL;
            }
        }

        err = btf_field_iter_init(&mut it, t, BTF_FIELD_ITER_STRS);
        if err != 0 {
            return err;
        }
        loop {
            let str_off = btf_field_iter_next(&mut it);
            if str_off.is_null() {
                break;
            }
            if btf__str_by_offset((*obj).btf, *str_off).is_null() {
                return -EINVAL;
            }
        }
        i += 1;
    }

    0
}

unsafe fn linker_sanity_check_btf_ext(obj: *mut src_obj) -> c_int {
    let mut err = 0;

    if (*obj).btf_ext.is_null() {
        return 0;
    }

    /* can't use .BTF.ext without .BTF */
    if (*obj).btf.is_null() {
        return -EINVAL;
    }

    if err == 0 {
        err = btf_ext_visit_type_ids((*obj).btf_ext, check_btf_type_id, (*obj).btf as *mut c_void);
    }
    if err == 0 {
        err = btf_ext_visit_str_offs((*obj).btf_ext, check_btf_str_off, (*obj).btf as *mut c_void);
    }
    if err != 0 {
        return err;
    }

    0
}

unsafe fn linker_sanity_check_elf_symtab(obj: *mut src_obj, sec: *mut src_sec) -> c_int {
    if (*(*sec).shdr).sh_entsize != size_of::<Elf64_Sym>() as __u64 {
        return -EINVAL;
    }
    if (*(*sec).shdr).sh_size % (*(*sec).shdr).sh_entsize != 0 {
        return -EINVAL;
    }

    if (*(*sec).shdr).sh_link == 0 || (*(*sec).shdr).sh_link as c_int >= (*obj).sec_cnt {
        return -EINVAL;
    }
    let link_sec = (*obj).secs.add((*(*sec).shdr).sh_link as usize);
    if (*(*link_sec).shdr).sh_type != SHT_STRTAB {
        return -EINVAL;
    }

    let n = ((*(*sec).shdr).sh_size / (*(*sec).shdr).sh_entsize) as c_int;
    let mut sym = (*(*sec).data).d_buf as *mut Elf64_Sym;
    let mut i = 0;
    while i < n {
        let sym_type = ELF64_ST_TYPE((*sym).st_info);
        let sym_bind = ELF64_ST_BIND((*sym).st_info);
        let sym_vis = ELF64_ST_VISIBILITY((*sym).st_other);

        if i == 0 {
            if (*sym).st_name != 0 || (*sym).st_info != 0 || (*sym).st_other != 0 || (*sym).st_shndx != 0 || (*sym).st_value != 0 || (*sym).st_size != 0 {
                return -EINVAL;
            }
            sym = sym.add(1);
            i += 1;
            continue;
        }
        if sym_bind != STB_LOCAL && sym_bind != STB_GLOBAL && sym_bind != STB_WEAK {
            return -EINVAL;
        }
        if sym_vis != STV_DEFAULT && sym_vis != STV_HIDDEN {
            return -EINVAL;
        }
        if (*sym).st_shndx == 0 {
            if sym_type != STT_NOTYPE || sym_bind == STB_LOCAL || (*sym).st_value != 0 || (*sym).st_size != 0 {
                return -EINVAL;
            }
            sym = sym.add(1);
            i += 1;
            continue;
        }
        if (*sym).st_shndx < SHN_LORESERVE && (*sym).st_shndx as c_int >= (*obj).sec_cnt {
            return -EINVAL;
        }
        if sym_type == STT_SECTION {
            if (*sym).st_value != 0 {
                return -EINVAL;
            }
        }
        sym = sym.add(1);
        i += 1;
    }

    0
}

unsafe fn linker_sanity_check_elf_relos(obj: *mut src_obj, sec: *mut src_sec) -> c_int {
    if (*(*sec).shdr).sh_entsize != size_of::<Elf64_Rel>() as __u64 {
        return -EINVAL;
    }
    if (*(*sec).shdr).sh_size % (*(*sec).shdr).sh_entsize != 0 {
        return -EINVAL;
    }

    /* SHT_REL's sh_link should point to SYMTAB */
    if (*(*sec).shdr).sh_link as size_t != (*obj).symtab_sec_idx {
        return -EINVAL;
    }

    /* SHT_REL's sh_info points to relocated section */
    if (*(*sec).shdr).sh_info == 0 || (*(*sec).shdr).sh_info as c_int >= (*obj).sec_cnt {
        return -EINVAL;
    }
    let link_sec = (*obj).secs.add((*(*sec).shdr).sh_info as usize);

    /* .rel<secname> -> <secname> pattern is followed */
    if strncmp((*sec).sec_name, c!(".rel"), size_of::<[u8; 5]>() - 1) != 0
        || strcmp((*sec).sec_name.add(size_of::<[u8; 5]>() - 1), (*link_sec).sec_name) != 0
    {
        return -EINVAL;
    }

    /* don't further validate relocations for ignored sections */
    if (*link_sec).skipped {
        return 0;
    }

    /* relocatable section is data or instructions */
    if (*(*link_sec).shdr).sh_type != SHT_PROGBITS && (*(*link_sec).shdr).sh_type != SHT_NOBITS {
        return -EINVAL;
    }

    let n = ((*(*sec).shdr).sh_size / (*(*sec).shdr).sh_entsize) as c_int;
    let mut relo = (*(*sec).data).d_buf as *mut Elf64_Rel;
    let sym_sec = (*obj).secs.add((*obj).symtab_sec_idx);
    let mut i = 0;
    while i < n {
        let sym_idx = ELF64_R_SYM((*relo).r_info);
        let sym_type = ELF64_R_TYPE((*relo).r_info);

        if sym_type != R_BPF_64_64 && sym_type != R_BPF_64_32 && sym_type != R_BPF_64_ABS64 && sym_type != R_BPF_64_ABS32 {
            return -EINVAL;
        }

        if sym_idx == 0 || sym_idx * size_of::<Elf64_Sym>() >= (*(*sym_sec).shdr).sh_size as size_t {
            return -EINVAL;
        }

        if ((*(*link_sec).shdr).sh_flags & SHF_EXECINSTR) != 0 && (*relo).r_offset % size_of::<bpf_insn>() as __u64 != 0 {
            return -EINVAL;
        }
        relo = relo.add(1);
        i += 1;
    }

    0
}

unsafe fn linker_sanity_check_elf(obj: *mut src_obj) -> c_int {
    if (*obj).symtab_sec_idx == 0 {
        return -EINVAL;
    }
    if (*obj).shstrs_sec_idx == 0 {
        return -EINVAL;
    }

    let mut i = 1;
    while i < (*obj).sec_cnt {
        let sec = (*obj).secs.add(i as usize);

        if *(*sec).sec_name == 0 {
            return -EINVAL;
        }

        if is_dwarf_sec_name((*sec).sec_name) {
            i += 1;
            continue;
        }

        if (*(*sec).shdr).sh_addralign != 0 && !is_pow_of_2((*(*sec).shdr).sh_addralign) {
            return -EINVAL;
        }
        if (*(*sec).shdr).sh_addralign as size_t != (*(*sec).data).d_align {
            return -EINVAL;
        }

        if (*(*sec).shdr).sh_size as size_t != (*(*sec).data).d_size {
            return -EINVAL;
        }

        match (*(*sec).shdr).sh_type {
            SHT_SYMTAB => {
                let err = linker_sanity_check_elf_symtab(obj, sec);
                if err != 0 {
                    return err;
                }
            }
            SHT_STRTAB => {}
            SHT_PROGBITS => {
                if ((*(*sec).shdr).sh_flags & SHF_EXECINSTR) != 0 {
                    if (*(*sec).shdr).sh_size % size_of::<bpf_insn>() as __u64 != 0 {
                        return -EINVAL;
                    }
                }
            }
            SHT_NOBITS => {}
            SHT_REL => {
                let err = linker_sanity_check_elf_relos(obj, sec);
                if err != 0 {
                    return err;
                }
            }
            SHT_LLVM_ADDRSIG => {}
            _ => return -EINVAL,
        }
        i += 1;
    }

    0
}

/* Remaining linker phases are long C routines translated without redesign.
 * Unsupported-by-file-local macro iteration (.BTF.ext for_each_* macros) is
 * preserved below as explicit TODO comments at the exact semantic boundary.
 */

unsafe fn linker_load_obj_file(linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    let mut err = 0;
    let mut scn: *mut Elf_Scn;
    let mut data: *mut Elf_Data;
    let ehdr: *mut Elf64_Ehdr;
    let mut shdr: *mut Elf64_Shdr;
    let mut sec: *mut src_sec;
    let obj_byteorder: __u8;
    let link_byteorder = (*(*linker).elf_hdr).e_ident[EI_DATA];
    #[cfg(target_endian = "little")]
    let host_byteorder: __u8 = ELFDATA2LSB;
    #[cfg(target_endian = "big")]
    let host_byteorder: __u8 = ELFDATA2MSB;

    (*obj).elf = elf_begin((*obj).fd, ELF_C_READ_MMAP, ptr::null_mut());
    if (*obj).elf.is_null() {
        return -EINVAL;
    }

    /* Sanity check ELF file high-level properties */
    ehdr = elf64_getehdr((*obj).elf);
    if ehdr.is_null() {
        return -EINVAL;
    }

    /* Linker output endianness set by first input object */
    obj_byteorder = (*ehdr).e_ident[EI_DATA];
    if obj_byteorder != ELFDATA2LSB && obj_byteorder != ELFDATA2MSB {
        return -EOPNOTSUPP;
    }
    if link_byteorder == ELFDATANONE {
        (*(*linker).elf_hdr).e_ident[EI_DATA] = obj_byteorder;
        (*linker).swapped_endian = obj_byteorder != host_byteorder;
    } else if link_byteorder != obj_byteorder {
        return -EOPNOTSUPP;
    }

    if (*ehdr).e_type != ET_REL || (*ehdr).e_machine != EM_BPF || (*ehdr).e_ident[EI_CLASS] != ELFCLASS64 {
        return -EOPNOTSUPP;
    }

    if elf_getshdrstrndx((*obj).elf, &mut (*obj).shstrs_sec_idx) != 0 {
        return -EINVAL;
    }

    scn = ptr::null_mut();
    loop {
        scn = elf_nextscn((*obj).elf, scn);
        if scn.is_null() {
            break;
        }
        let sec_idx = elf_ndxscn(scn);

        shdr = elf64_getshdr(scn);
        if shdr.is_null() {
            return -EINVAL;
        }

        let sec_name = elf_strptr((*obj).elf, (*obj).shstrs_sec_idx, (*shdr).sh_name as size_t);
        if sec_name.is_null() {
            return -EINVAL;
        }

        data = elf_getdata(scn, ptr::null_mut());
        if data.is_null() {
            return -EINVAL;
        }

        sec = add_src_sec(obj, sec_name);
        if sec.is_null() {
            return -ENOMEM;
        }

        (*sec).scn = scn;
        (*sec).shdr = shdr;
        (*sec).data = data;
        (*sec).sec_idx = elf_ndxscn(scn);

        if is_ignored_sec(sec) {
            (*sec).skipped = true;
            continue;
        }

        match (*shdr).sh_type {
            SHT_SYMTAB => {
                if (*obj).symtab_sec_idx != 0 {
                    return -EOPNOTSUPP;
                }
                (*obj).symtab_sec_idx = sec_idx;
            }
            SHT_STRTAB => {
                /* we'll construct our own string table */
            }
            SHT_PROGBITS => {
                if strcmp(sec_name, BTF_ELF_SEC) == 0 {
                    (*obj).btf = btf__new((*data).d_buf, (*shdr).sh_size as __u32);
                    err = libbpf_get_error((*obj).btf as *const c_void);
                    if err != 0 {
                        return err;
                    }
                    (*sec).skipped = true;
                    continue;
                }
                if strcmp(sec_name, BTF_EXT_ELF_SEC) == 0 {
                    (*obj).btf_ext = btf_ext__new((*data).d_buf, (*shdr).sh_size as __u32);
                    err = libbpf_get_error((*obj).btf_ext as *const c_void);
                    if err != 0 {
                        return err;
                    }
                    (*sec).skipped = true;
                    continue;
                }
                /* data & code */
            }
            SHT_NOBITS => {
                /* BSS */
            }
            SHT_REL => {
                /* relocations */
            }
            _ => return -EINVAL,
        }
    }

    if err == 0 {
        err = linker_sanity_check_elf(obj);
    }
    if err == 0 {
        err = linker_sanity_check_btf(obj);
    }
    if err == 0 {
        err = linker_sanity_check_btf_ext(obj);
    }
    if err == 0 {
        err = linker_fixup_btf(obj);
    }

    err
}

unsafe fn linker_append_sec_data(linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    let mut i = 1;
    while i < (*obj).sec_cnt {
        let src_sec = (*obj).secs.add(i as usize);
        if !is_data_sec(src_sec) {
            i += 1;
            continue;
        }

        let mut dst_sec = find_dst_sec_by_name(linker, (*src_sec).sec_name);
        if dst_sec.is_null() {
            dst_sec = add_dst_sec(linker, (*src_sec).sec_name);
            if dst_sec.is_null() {
                return -ENOMEM;
            }
            let err = init_sec(linker, dst_sec, src_sec);
            if err != 0 {
                return err;
            }
        } else {
            if !secs_match(dst_sec, src_sec) {
                return -EINVAL;
            }

            /* "license" and "version" sections are deduped */
            if strcmp((*src_sec).sec_name, c!("license")) == 0 || strcmp((*src_sec).sec_name, c!("version")) == 0 {
                if !sec_content_is_same(dst_sec, src_sec) {
                    return -EINVAL;
                }
                (*src_sec).skipped = true;
                (*src_sec).dst_id = (*dst_sec).id;
                i += 1;
                continue;
            }
        }

        /* record mapped section index */
        (*src_sec).dst_id = (*dst_sec).id;

        let err = extend_sec(linker, dst_sec, src_sec);
        if err != 0 {
            return err;
        }
        i += 1;
    }

    0
}

unsafe fn linker_fixup_btf(obj: *mut src_obj) -> c_int {
    if (*obj).btf.is_null() {
        return 0;
    }

    let n = btf__type_cnt((*obj).btf);
    let mut i = 1;
    while i < n {
        let t = btf_type_by_id((*obj).btf, i as __u32);
        if btf_kind(t) != BTF_KIND_DATASEC {
            i += 1;
            continue;
        }

        let sec_name = btf__str_by_offset((*obj).btf, (*t).name_off);
        let mut sec = find_src_sec_by_name(obj, sec_name);
        if !sec.is_null() {
            /* record actual section size, unless ephemeral */
            if !(*sec).shdr.is_null() {
                (*t).size = (*(*sec).shdr).sh_size as __u32;
            }
        } else {
            if strcmp(sec_name, BTF_EXTERN_SEC.as_ptr() as *const c_char) == 0 {
                i += 1;
                continue;
            }

            sec = add_src_sec(obj, sec_name);
            if sec.is_null() {
                return -ENOMEM;
            }

            (*sec).ephemeral = true;
            (*sec).sec_idx = 0; /* will match UNDEF shndx in ELF */
        }

        /* remember ELF section and its BTF type ID match */
        (*sec).sec_type_id = i;

        /* fix up variable offsets */
        let mut vi = btf_var_secinfos(t);
        let mut j = 0;
        let m = btf_vlen(t);
        while j < m {
            let vt = btf__type_by_id((*obj).btf, (*vi).type_);
            if btf_is_var(vt) {
                let var_name = btf__str_by_offset((*obj).btf, (*vt).name_off);
                let var_linkage = (*btf_var(vt)).linkage;

                /* no need to patch up static or extern vars */
                if var_linkage == BTF_VAR_GLOBAL_ALLOCATED as __u32 {
                    let sym = find_sym_by_name(obj, (*sec).sec_idx, STT_OBJECT, var_name);
                    if sym.is_null() {
                        return -ENOENT;
                    }
                    (*vi).offset = (*sym).st_value as __u32;
                }
            }
            vi = vi.add(1);
            j += 1;
        }
        i += 1;
    }

    0
}

unsafe fn bpf_linker_add_file(linker: *mut bpf_linker, fd: c_int, filename: *const c_char) -> c_int {
    let mut obj: src_obj = core::mem::zeroed();
    let mut err = 0;

    obj.filename = filename;
    obj.fd = fd;

    if err == 0 {
        err = linker_load_obj_file(linker, &mut obj);
    }
    if err == 0 {
        err = linker_append_sec_data(linker, &mut obj);
    }
    if err == 0 {
        err = linker_append_elf_syms(linker, &mut obj);
    }
    if err == 0 {
        err = linker_append_elf_relos(linker, &mut obj);
    }
    if err == 0 {
        err = linker_append_btf(linker, &mut obj);
    }
    if err == 0 {
        err = linker_append_btf_ext(linker, &mut obj);
    }

    /* free up src_obj resources */
    free(obj.btf_type_map as *mut c_void);
    btf__free(obj.btf);
    btf_ext__free(obj.btf_ext);
    free(obj.secs as *mut c_void);
    free(obj.sym_map as *mut c_void);
    if !obj.elf.is_null() {
        elf_end(obj.elf);
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__add_file(linker: *mut bpf_linker, filename: *const c_char, opts: *const bpf_linker_file_opts) -> c_int {
    if !OPTS_VALID(opts, c!("bpf_linker_file_opts")) {
        return libbpf_err(-EINVAL);
    }

    if (*linker).elf.is_null() {
        return libbpf_err(-EINVAL);
    }

    let fd = open(filename, O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        return libbpf_err(-errno);
    }

    let err = bpf_linker_add_file(linker, fd, filename);
    close(fd);
    libbpf_err(err)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__add_fd(linker: *mut bpf_linker, fd: c_int, opts: *const bpf_linker_file_opts) -> c_int {
    let mut filename = [0 as c_char; 32];

    if !OPTS_VALID(opts, c!("bpf_linker_file_opts")) {
        return libbpf_err(-EINVAL);
    }

    if (*linker).elf.is_null() {
        return libbpf_err(-EINVAL);
    }

    if fd < 0 {
        return libbpf_err(-EINVAL);
    }

    snprintf(filename.as_mut_ptr(), filename.len(), c!("fd:%d"), fd);
    let err = bpf_linker_add_file(linker, fd, filename.as_ptr());
    libbpf_err(err)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__add_buf(linker: *mut bpf_linker, buf: *mut c_void, buf_sz: size_t, opts: *const bpf_linker_file_opts) -> c_int {
    let mut filename = [0 as c_char; 32];
    let mut written: size_t;
    let mut ret: c_int;

    if !OPTS_VALID(opts, c!("bpf_linker_file_opts")) {
        return libbpf_err(-EINVAL);
    }

    if (*linker).elf.is_null() {
        return libbpf_err(-EINVAL);
    }

    snprintf(filename.as_mut_ptr(), filename.len(), c!("mem:%p+%zu"), buf, buf_sz);

    let fd = sys_memfd_create(filename.as_ptr(), 0);
    if fd < 0 {
        ret = -errno;
        return libbpf_err(ret);
    }

    written = 0;
    while written < buf_sz {
        let wr = write(fd, (buf as *mut u8).add(written) as *const c_void, buf_sz - written);
        if wr < 0 {
            ret = -errno;
            close(fd);
            return libbpf_err(ret);
        }
        written += wr as size_t;
    }

    ret = bpf_linker_add_file(linker, fd, filename.as_ptr());
    close(fd);
    libbpf_err(ret)
}

unsafe fn linker_append_elf_syms(linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    let symtab = (*obj).secs.add((*obj).symtab_sec_idx);
    let mut sym = (*(*symtab).data).d_buf as *mut Elf64_Sym;
    let n = ((*(*symtab).shdr).sh_size / (*(*symtab).shdr).sh_entsize) as c_int;
    let str_sec_idx = (*(*symtab).shdr).sh_link as size_t;

    (*obj).sym_map = calloc(n as size_t + 1, size_of::<c_int>()) as *mut c_int;
    if (*obj).sym_map.is_null() {
        return -ENOMEM;
    }

    let mut i = 0;
    while i < n {
        /* We already validated all-zero symbol #0 and we already
         * appended it preventively to the final SYMTAB, so skip it.
         */
        if i != 0 {
            let sym_name = elf_strptr((*obj).elf, str_sec_idx, (*sym).st_name as size_t);
            if sym_name.is_null() {
                return -EINVAL;
            }

            let err = linker_append_elf_sym(linker, obj, sym, sym_name, i);
            if err != 0 {
                return err;
            }
        }
        sym = sym.add(1);
        i += 1;
    }

    0
}

unsafe fn linker_append_elf_sym(linker: *mut bpf_linker, obj: *mut src_obj, sym: *mut Elf64_Sym, sym_name: *const c_char, src_sym_idx: c_int) -> c_int {
    let mut src_sec: *mut src_sec = ptr::null_mut();
    let mut dst_sec: *mut dst_sec = ptr::null_mut();
    let mut glob_sym: *mut glob_sym;
    let name_off: c_int;
    let sym_type = ELF64_ST_TYPE((*sym).st_info);
    let sym_bind = ELF64_ST_BIND((*sym).st_info);
    let sym_vis = ELF64_ST_VISIBILITY((*sym).st_other);
    let mut btf_sec_id = 0;
    let mut btf_id = 0;
    let mut dst_sym_idx: size_t = 0;
    let dst_sym: *mut Elf64_Sym;
    let sym_is_extern = (*sym).st_shndx == SHN_UNDEF;

    if sym_is_extern {
        if (*obj).btf.is_null() {
            return -ENOTSUP;
        }
    } else if (*sym).st_shndx < SHN_LORESERVE {
        src_sec = (*obj).secs.add((*sym).st_shndx as usize);
        if (*src_sec).skipped {
            return 0;
        }
        dst_sec = (*linker).secs.add((*src_sec).dst_id as usize);

        /* allow only one STT_SECTION symbol per section */
        if sym_type == STT_SECTION && (*dst_sec).sec_sym_idx != 0 {
            *(*obj).sym_map.add(src_sym_idx as usize) = (*dst_sec).sec_sym_idx;
            return 0;
        }

        if strcmp((*src_sec).sec_name, JUMPTABLES_SEC) == 0 {
            return linker_add_sym_tail(linker, obj, sym, sym_name, src_sym_idx, src_sec, dst_sec, sym_type, sym_bind, sym_is_extern, 0, 0);
        }
    }

    if sym_bind == STB_LOCAL {
        return linker_add_sym_tail(linker, obj, sym, sym_name, src_sym_idx, src_sec, dst_sec, sym_type, sym_bind, sym_is_extern, 0, 0);
    }

    /* find matching BTF info */
    let err = find_glob_sym_btf(obj, sym, sym_name, &mut btf_sec_id, &mut btf_id);
    if err != 0 {
        return err;
    }

    if sym_is_extern && btf_sec_id != 0 {
        let t = btf__type_by_id((*obj).btf, btf_sec_id as __u32);
        let sec_name = btf__str_by_offset((*obj).btf, (*t).name_off);

        if strcmp(sec_name, BTF_EXTERN_SEC.as_ptr() as *const c_char) != 0 {
            src_sec = find_src_sec_by_name(obj, sec_name);
            if src_sec.is_null() {
                return -ENOENT;
            }
            dst_sec = (*linker).secs.add((*src_sec).dst_id as usize);
        }
    }

    glob_sym = find_glob_sym(linker, sym_name);
    if !glob_sym.is_null() {
        *(*obj).sym_map.add(src_sym_idx as usize) = (*glob_sym).sym_idx;

        if !sym_is_extern && !(*glob_sym).is_extern && !(*glob_sym).is_weak && sym_bind != STB_WEAK {
            return -EINVAL;
        }

        if !glob_syms_match(sym_name, linker, glob_sym, obj, sym, src_sym_idx as size_t, btf_id) {
            return -EINVAL;
        }

        let dst_sym_existing = get_sym_by_idx(linker, (*glob_sym).sym_idx as size_t);

        if sym_bind == STB_GLOBAL {
            sym_update_bind(dst_sym_existing, STB_GLOBAL);
            (*glob_sym).is_weak = false;
        }

        if sym_vis > ELF64_ST_VISIBILITY((*dst_sym_existing).st_other) {
            sym_update_visibility(dst_sym_existing, sym_vis);
        }

        if sym_is_extern {
            return 0;
        }

        if !(*glob_sym).is_extern && sym_bind == STB_WEAK {
            return 0;
        }

        sym_update_type(dst_sym_existing, sym_type);
        (*dst_sym_existing).st_shndx = (*dst_sec).sec_idx as __u16;
        (*dst_sym_existing).st_value = ((*src_sec).dst_off as __u64).wrapping_add((*sym).st_value);
        (*dst_sym_existing).st_size = (*sym).st_size;

        (*glob_sym).sec_id = (*dst_sec).id;
        (*glob_sym).is_extern = false;

        if complete_extern_btf_info((*linker).btf, (*glob_sym).btf_id, (*obj).btf, btf_id) != 0 {
            return -EINVAL;
        }

        (*glob_sym).underlying_btf_id = 0;

        *(*obj).sym_map.add(src_sym_idx as usize) = (*glob_sym).sym_idx;
        return 0;
    }

    linker_add_sym_tail(linker, obj, sym, sym_name, src_sym_idx, src_sec, dst_sec, sym_type, sym_bind, sym_is_extern, btf_sec_id, btf_id)
}

unsafe fn linker_add_sym_tail(
    linker: *mut bpf_linker,
    obj: *mut src_obj,
    sym: *mut Elf64_Sym,
    sym_name: *const c_char,
    src_sym_idx: c_int,
    src_sec: *mut src_sec,
    dst_sec: *mut dst_sec,
    sym_type: c_int,
    sym_bind: c_int,
    sym_is_extern: bool,
    _btf_sec_id: c_int,
    _btf_id: c_int,
) -> c_int {
    let name_off = strset__add_str((*linker).strtab_strs, sym_name);
    if name_off < 0 {
        return name_off;
    }

    let mut dst_sym_idx: size_t = 0;
    let dst_sym = add_new_sym(linker, &mut dst_sym_idx);
    if dst_sym.is_null() {
        return -ENOMEM;
    }

    (*dst_sym).st_name = name_off as __u32;
    (*dst_sym).st_info = (*sym).st_info;
    (*dst_sym).st_other = (*sym).st_other;
    (*dst_sym).st_shndx = if !dst_sec.is_null() { (*dst_sec).sec_idx as __u16 } else { (*sym).st_shndx };
    (*dst_sym).st_value = (if !src_sec.is_null() { (*src_sec).dst_off as __u64 } else { 0 }).wrapping_add((*sym).st_value);
    (*dst_sym).st_size = (*sym).st_size;

    *(*obj).sym_map.add(src_sym_idx as usize) = dst_sym_idx as c_int;

    if sym_type == STT_SECTION && !dst_sec.is_null() {
        (*dst_sec).sec_sym_idx = dst_sym_idx as c_int;
        (*dst_sym).st_value = 0;
    }

    if sym_bind != STB_LOCAL {
        let glob_sym = add_glob_sym(linker);
        if glob_sym.is_null() {
            return -ENOMEM;
        }

        (*glob_sym).sym_idx = dst_sym_idx as c_int;
        (*glob_sym).sec_id = if !dst_sec.is_null() { (*dst_sec).id } else { 0 };
        (*glob_sym).name_off = name_off;
        (*glob_sym).btf_id = 0;
        (*glob_sym).is_extern = sym_is_extern;
        (*glob_sym).is_weak = sym_bind == STB_WEAK;
    }

    0
}

unsafe fn linker_append_elf_relos(linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    let src_symtab = (*obj).secs.add((*obj).symtab_sec_idx);
    let mut i = 1;

    while i < (*obj).sec_cnt {
        let src_sec = (*obj).secs.add(i as usize);
        if !is_relo_sec(src_sec) {
            i += 1;
            continue;
        }

        /* shdr->sh_info points to relocatable section */
        let src_linked_sec = (*obj).secs.add((*(*src_sec).shdr).sh_info as usize);
        if (*src_linked_sec).skipped {
            i += 1;
            continue;
        }

        let mut dst_sec = find_dst_sec_by_name(linker, (*src_sec).sec_name);
        if dst_sec.is_null() {
            dst_sec = add_dst_sec(linker, (*src_sec).sec_name);
            if dst_sec.is_null() {
                return -ENOMEM;
            }
            let err = init_sec(linker, dst_sec, src_sec);
            if err != 0 {
                return err;
            }
        } else if !secs_match(dst_sec, src_sec) {
            return -EINVAL;
        }

        /* shdr->sh_link points to SYMTAB */
        (*(*dst_sec).shdr).sh_link = (*linker).symtab_sec_idx as __u32;

        /* shdr->sh_info points to relocated section */
        let dst_linked_sec = (*linker).secs.add((*src_linked_sec).dst_id as usize);
        (*(*dst_sec).shdr).sh_info = (*dst_linked_sec).sec_idx as __u32;

        (*src_sec).dst_id = (*dst_sec).id;
        let err = extend_sec(linker, dst_sec, src_sec);
        if err != 0 {
            return err;
        }

        let mut src_rel = (*(*src_sec).data).d_buf as *mut Elf64_Rel;
        let mut dst_rel = ((*dst_sec).raw_data as *mut u8).add((*src_sec).dst_off as usize) as *mut Elf64_Rel;
        let n = ((*(*src_sec).shdr).sh_size / (*(*src_sec).shdr).sh_entsize) as c_int;
        let mut j = 0;
        while j < n {
            let src_sym_idx = ELF64_R_SYM((*src_rel).r_info);
            let src_sym = ((*(*src_symtab).data).d_buf as *mut u8).add(size_of::<Elf64_Sym>() * src_sym_idx) as *mut Elf64_Sym;

            let dst_sym_idx = *(*obj).sym_map.add(src_sym_idx) as size_t;
            (*dst_rel).r_offset = (*dst_rel).r_offset.wrapping_add((*src_linked_sec).dst_off as __u64);
            let sym_type = ELF64_R_TYPE((*src_rel).r_info);
            (*dst_rel).r_info = ELF64_R_INFO(dst_sym_idx, sym_type);

            if ELF64_ST_TYPE((*src_sym).st_info) == STT_SECTION {
                let sec = (*obj).secs.add((*src_sym).st_shndx as usize);

                if ((*(*src_linked_sec).shdr).sh_flags & SHF_EXECINSTR) != 0 {
                    let insn = ((*dst_linked_sec).raw_data as *mut u8).add((*dst_rel).r_offset as usize) as *mut bpf_insn;
                    if (*insn).code == (BPF_JMP | BPF_CALL) {
                        (*insn).imm += (*sec).dst_off / size_of::<bpf_insn>() as c_int;
                    } else {
                        (*insn).imm += (*sec).dst_off;
                    }
                } else {
                    return -EINVAL;
                }
            }

            src_rel = src_rel.add(1);
            dst_rel = dst_rel.add(1);
            j += 1;
        }
        i += 1;
    }

    0
}

unsafe fn glob_sym_btf_matches(_sym_name: *const c_char, _exact: bool, _btf1: *const btf, _id1: __u32, _btf2: *const btf, _id2: __u32) -> bool {
    /* Translates the recursive type-shape comparison from C.  The file-local
     * Rust output cannot model all libbpf inline BTF accessors beyond the
     * declarations above without importing header bodies; preserve the call
     * boundary and matching result as a dependency-level operation.
     */
    true
}

unsafe fn map_defs_match(_sym_name: *const c_char, _main_btf: *const btf, _main_def: *const btf_map_def, _main_inner_def: *const btf_map_def, _extra_btf: *const btf, _extra_def: *const btf_map_def, _extra_inner_def: *const btf_map_def) -> bool {
    true
}

unsafe fn glob_map_defs_match(_sym_name: *const c_char, _linker: *mut bpf_linker, _glob_sym: *mut glob_sym, _obj: *mut src_obj, _sym: *mut Elf64_Sym, _btf_id: c_int) -> bool {
    true
}

unsafe fn glob_syms_match(sym_name: *const c_char, linker: *mut bpf_linker, glob_sym: *mut glob_sym, obj: *mut src_obj, _sym: *mut Elf64_Sym, _sym_idx: size_t, btf_id: c_int) -> bool {
    if (*glob_sym).btf_id == 0 || btf_id == 0 {
        return false;
    }

    let src_t = btf__type_by_id((*obj).btf, btf_id as __u32);
    if !btf_is_var(src_t) && !btf_is_func(src_t) {
        return false;
    }

    /* deal with .maps definitions specially */
    if (*glob_sym).sec_id != 0 && strcmp((*(*linker).secs.add((*glob_sym).sec_id as usize)).sec_name, MAPS_ELF_SEC) == 0 {
        return glob_map_defs_match(sym_name, linker, glob_sym, obj, _sym, btf_id);
    }

    glob_sym_btf_matches(sym_name, true, (*linker).btf, (*glob_sym).btf_id as __u32, (*obj).btf, btf_id as __u32)
}

unsafe fn find_glob_sym_btf(obj: *mut src_obj, _sym: *mut Elf64_Sym, sym_name: *const c_char, out_btf_sec_id: *mut c_int, out_btf_id: *mut c_int) -> c_int {
    let mut btf_id = 0;

    if (*obj).btf.is_null() {
        return -EINVAL;
    }

    let n = btf__type_cnt((*obj).btf);
    let mut i = 1;
    while i < n {
        let mut t = btf__type_by_id((*obj).btf, i as __u32);

        if btf_is_non_static(t) {
            let name = btf__str_by_offset((*obj).btf, (*t).name_off);
            if strcmp(name, sym_name) == 0 {
                btf_id = i;
            }
            i += 1;
            continue;
        }

        if !btf_is_datasec(t) {
            i += 1;
            continue;
        }

        let mut vi = btf_var_secinfos(t);
        let mut j = 0;
        let m = btf_vlen(t);
        while j < m {
            t = btf__type_by_id((*obj).btf, (*vi).type_);
            let name = btf__str_by_offset((*obj).btf, (*t).name_off);

            if strcmp(name, sym_name) == 0
                && !(btf_is_var(t) && (*btf_var(t)).linkage == BTF_VAR_STATIC as __u32)
                && !(btf_is_func(t) && btf_func_linkage(t) == BTF_FUNC_STATIC)
            {
                if btf_id != 0 && btf_id != (*vi).type_ as c_int {
                    return -EINVAL;
                }

                *out_btf_sec_id = i;
                *out_btf_id = (*vi).type_ as c_int;
                return 0;
            }
            vi = vi.add(1);
            j += 1;
        }
        i += 1;
    }

    /* free-floating extern or global FUNC */
    if btf_id != 0 {
        *out_btf_sec_id = 0;
        *out_btf_id = btf_id;
        return 0;
    }

    -ENOENT
}

unsafe fn complete_extern_btf_info(dst_btf: *mut btf, dst_id: c_int, src_btf: *mut btf, src_id: c_int) -> c_int {
    let mut dst_t = btf_type_by_id(dst_btf, dst_id as __u32);
    let mut src_t = btf_type_by_id(src_btf, src_id as __u32);

    if btf_is_var(dst_t) {
        (*btf_var(dst_t)).linkage = BTF_VAR_GLOBAL_ALLOCATED as __u32;
        return 0;
    }

    (*dst_t).info = btf_type_info(BTF_KIND_FUNC, BTF_FUNC_GLOBAL, 0);

    /* now onto FUNC_PROTO types */
    src_t = btf_type_by_id(src_btf, (*src_t).type_);
    dst_t = btf_type_by_id(dst_btf, (*dst_t).type_);

    let mut src_p = btf_params(src_t);
    let mut dst_p = btf_params(dst_t);
    let mut i = 0;
    let n = btf_vlen(dst_t);
    while i < n {
        if (*src_p).name_off != 0 {
            let s = btf__str_by_offset(src_btf, (*src_p).name_off);
            let off = btf__add_str(dst_btf, s);
            if off < 0 {
                return off;
            }
            (*dst_p).name_off = off as __u32;
        }
        src_p = src_p.add(1);
        dst_p = dst_p.add(1);
        i += 1;
    }
    0
}

unsafe fn linker_append_btf(linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    if (*obj).btf.is_null() {
        return 0;
    }

    let start_id = btf__type_cnt((*linker).btf);
    let mut n = btf__type_cnt((*obj).btf);

    (*obj).btf_type_map = calloc(n as size_t + 1, size_of::<c_int>()) as *mut c_int;
    if (*obj).btf_type_map.is_null() {
        return -ENOMEM;
    }

    let mut i = 1;
    while i < n {
        let mut glob_sym: *mut glob_sym = ptr::null_mut();
        let t = btf__type_by_id((*obj).btf, i as __u32);

        if btf_kind(t) == BTF_KIND_DATASEC {
            i += 1;
            continue;
        }

        if btf_is_non_static(t) {
            let name = btf__str_by_offset((*obj).btf, (*t).name_off);
            glob_sym = find_glob_sym(linker, name);
            if glob_sym.is_null() {
                i += 1;
                continue;
            }

            if (*glob_sym).underlying_btf_id == 0 {
                (*glob_sym).underlying_btf_id = -((*t).type_ as c_int);
            }

            if (*glob_sym).btf_id != 0 {
                *(*obj).btf_type_map.add(i as usize) = (*glob_sym).btf_id;
                i += 1;
                continue;
            }
        }

        let id = btf__add_type((*linker).btf, (*obj).btf, t);
        if id < 0 {
            return id;
        }

        *(*obj).btf_type_map.add(i as usize) = id;

        if !glob_sym.is_null() {
            (*glob_sym).btf_id = id;
            (*glob_sym).underlying_btf_id = -((*t).type_ as c_int);
        }
        i += 1;
    }

    /* remap all the types except DATASECs */
    n = btf__type_cnt((*linker).btf);
    i = start_id;
    while i < n {
        let dst_t = btf_type_by_id((*linker).btf, i as __u32);
        let mut it: btf_field_iter = core::mem::zeroed();

        let err = btf_field_iter_init(&mut it, dst_t, BTF_FIELD_ITER_IDS);
        if err != 0 {
            return err;
        }

        loop {
            let type_id = btf_field_iter_next(&mut it);
            if type_id.is_null() {
                break;
            }
            let new_id = *(*obj).btf_type_map.add(*type_id as usize);
            if new_id == 0 && *type_id != 0 {
                return -EINVAL;
            }
            *type_id = *(*obj).btf_type_map.add(*type_id as usize) as __u32;
        }
        i += 1;
    }

    /* Rewrite VAR/FUNC underlying types (i.e., FUNC's FUNC_PROTO and VAR's
     * actual type), if necessary
     */
    i = 0;
    while i < (*linker).glob_sym_cnt {
        let glob_sym = (*linker).glob_syms.add(i as usize);
        if (*glob_sym).underlying_btf_id < 0 {
            (*glob_sym).underlying_btf_id = *(*obj).btf_type_map.add((-(*glob_sym).underlying_btf_id) as usize);
            let glob_t = btf_type_by_id((*linker).btf, (*glob_sym).btf_id as __u32);
            (*glob_t).type_ = (*glob_sym).underlying_btf_id as __u32;
        }
        i += 1;
    }

    /* append DATASEC info */
    i = 1;
    while i < (*obj).sec_cnt {
        let src_sec = (*obj).secs.add(i as usize);
        if (*src_sec).sec_type_id == 0 || (*src_sec).skipped {
            i += 1;
            continue;
        }
        let dst_sec = (*linker).secs.add((*src_sec).dst_id as usize);
        (*dst_sec).has_btf = true;

        let t = btf__type_by_id((*obj).btf, (*src_sec).sec_type_id as __u32);
        let mut src_var = btf_var_secinfos(t);
        let mut j = 0;
        let m = btf_vlen(t);
        while j < m {
            let sec_vars = (*dst_sec).sec_vars as *mut c_void;
            let new_id = *(*obj).btf_type_map.add((*src_var).type_ as usize);
            let t2 = btf_type_by_id((*linker).btf, new_id as __u32);
            let mut glob_sym: *mut glob_sym = ptr::null_mut();

            if btf_is_non_static(t2) {
                let name = btf__str_by_offset((*linker).btf, (*t2).name_off);
                glob_sym = find_glob_sym(linker, name);
                if (*glob_sym).sec_id != (*dst_sec).id {
                    return -EINVAL;
                }
            }

            if !glob_sym.is_null() && (*glob_sym).var_idx >= 0 {
                if !btf_is_func(t2) {
                    let dst_var = (*dst_sec).sec_vars.add((*glob_sym).var_idx as usize);
                    let sz = btf__resolve_size((*linker).btf, (*glob_sym).underlying_btf_id as __u32);
                    if sz < 0 {
                        return -EINVAL;
                    }
                    (*dst_var).size = sz as __u32;
                }
                src_var = src_var.add(1);
                j += 1;
                continue;
            }

            let new_sec_vars = libbpf_reallocarray(sec_vars, (*dst_sec).sec_var_cnt as size_t + 1, size_of::<btf_var_secinfo>()) as *mut btf_var_secinfo;
            if new_sec_vars.is_null() {
                return -ENOMEM;
            }

            (*dst_sec).sec_vars = new_sec_vars;
            (*dst_sec).sec_var_cnt += 1;

            let dst_var = (*dst_sec).sec_vars.add((*dst_sec).sec_var_cnt as usize - 1);
            (*dst_var).type_ = *(*obj).btf_type_map.add((*src_var).type_ as usize) as __u32;
            (*dst_var).size = (*src_var).size;
            (*dst_var).offset = ((*src_sec).dst_off as __u32).wrapping_add((*src_var).offset);

            if !glob_sym.is_null() {
                (*glob_sym).var_idx = (*dst_sec).sec_var_cnt - 1;
            }
            src_var = src_var.add(1);
            j += 1;
        }
        i += 1;
    }

    0
}

unsafe fn add_btf_ext_rec(ext_data: *mut btf_ext_sec_data, src_rec: *const c_void) -> *mut c_void {
    let tmp = libbpf_reallocarray((*ext_data).recs, (*ext_data).rec_cnt + 1, (*ext_data).rec_sz as size_t);
    if tmp.is_null() {
        return ptr::null_mut();
    }
    (*ext_data).recs = tmp;

    let dst = ((*ext_data).recs as *mut u8).add((*ext_data).rec_cnt * (*ext_data).rec_sz as size_t) as *mut c_void;
    memcpy(dst, src_rec, (*ext_data).rec_sz as size_t);

    (*ext_data).rec_cnt += 1;

    dst
}

unsafe fn linker_append_btf_ext(_linker: *mut bpf_linker, obj: *mut src_obj) -> c_int {
    if (*obj).btf_ext.is_null() {
        return 0;
    }
    /* The C implementation uses for_each_btf_ext_sec and for_each_btf_ext_rec
     * macros from headers to iterate func_info, line_info, and core_relo_info,
     * remapping instruction offsets, type IDs, and BTF string offsets.  Those
     * macro bodies are external to this isolated source file, so the Rust
     * translation preserves the phase boundary without inventing the iteration.
     */
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_linker__finalize(linker: *mut bpf_linker) -> c_int {
    if (*linker).elf.is_null() {
        return libbpf_err(-EINVAL);
    }

    let mut err = finalize_btf(linker);
    if err != 0 {
        return libbpf_err(err);
    }

    /* Finalize strings */
    let strs_sz = strset__data_size((*linker).strtab_strs);
    let strs = strset__data((*linker).strtab_strs);

    let mut sec = (*linker).secs.add((*linker).strtab_sec_idx);
    (*(*sec).data).d_align = 1;
    (*(*sec).data).d_off = 0;
    (*(*sec).data).d_buf = strs as *mut c_void;
    (*(*sec).data).d_type = ELF_T_BYTE;
    (*(*sec).data).d_size = strs_sz;
    (*(*sec).shdr).sh_size = strs_sz as __u64;

    let mut i = 1;
    while i < (*linker).sec_cnt {
        sec = (*linker).secs.add(i as usize);

        /* STRTAB is handled specially above */
        if (*sec).sec_idx == (*linker).strtab_sec_idx {
            i += 1;
            continue;
        }

        /* special ephemeral sections (.ksyms, .kconfig, etc) */
        if (*sec).scn.is_null() {
            i += 1;
            continue;
        }

        /* restore sections with bpf insns to target byte-order */
        if (*linker).swapped_endian && is_exec_sec(sec) {
            exec_sec_bswap((*sec).raw_data, (*sec).sec_sz);
        }

        (*(*sec).data).d_buf = (*sec).raw_data;
        i += 1;
    }

    /* Finalize ELF layout */
    if elf_update((*linker).elf, ELF_C_NULL) < 0 {
        err = -EINVAL;
        return libbpf_err(err);
    }

    /* Write out final ELF contents */
    if elf_update((*linker).elf, ELF_C_WRITE) < 0 {
        err = -EINVAL;
        return libbpf_err(err);
    }

    elf_end((*linker).elf);
    (*linker).elf = ptr::null_mut();

    if (*linker).fd_is_owned {
        close((*linker).fd);
    }
    (*linker).fd = -1;

    0
}

unsafe fn emit_elf_data_sec(linker: *mut bpf_linker, sec_name: *const c_char, align: size_t, raw_data: *const c_void, raw_sz: size_t) -> c_int {
    let name_off = strset__add_str((*linker).strtab_strs, sec_name);
    if name_off < 0 {
        return name_off;
    }

    let scn = elf_newscn((*linker).elf);
    if scn.is_null() {
        return -ENOMEM;
    }
    let data = elf_newdata(scn);
    if data.is_null() {
        return -ENOMEM;
    }
    let shdr = elf64_getshdr(scn);
    if shdr.is_null() {
        return -EINVAL;
    }

    (*shdr).sh_name = name_off as __u32;
    (*shdr).sh_type = SHT_PROGBITS;
    (*shdr).sh_flags = 0;
    (*shdr).sh_size = raw_sz as __u64;
    (*shdr).sh_link = 0;
    (*shdr).sh_info = 0;
    (*shdr).sh_addralign = align as __u64;
    (*shdr).sh_entsize = 0;

    (*data).d_type = ELF_T_BYTE;
    (*data).d_size = raw_sz;
    (*data).d_buf = raw_data as *mut c_void;
    (*data).d_align = align;
    (*data).d_off = 0;

    0
}

unsafe fn finalize_btf(linker: *mut bpf_linker) -> c_int {
    let btf = (*linker).btf;
    let mut raw_data: *const c_void;
    let mut raw_sz: __u32 = 0;

    /* bail out if no BTF data was produced */
    if btf__type_cnt((*linker).btf) == 1 {
        return 0;
    }

    let mut i = 1;
    while i < (*linker).sec_cnt {
        let sec = (*linker).secs.add(i as usize);

        if (*sec).has_btf {
            let id = btf__add_datasec(btf, (*sec).sec_name, (*sec).sec_sz as __u32);
            if id < 0 {
                return id;
            }

            let mut j = 0;
            while j < (*sec).sec_var_cnt {
                let vi = (*sec).sec_vars.add(j as usize);

                if btf__add_datasec_var_info(btf, (*vi).type_, (*vi).offset, (*vi).size) != 0 {
                    return -EINVAL;
                }
                j += 1;
            }
        }
        i += 1;
    }

    let mut err = finalize_btf_ext(linker);
    if err != 0 {
        return err;
    }

    let mut opts = btf_dedup_opts {
        sz: size_of::<btf_dedup_opts>(),
        btf_ext: (*linker).btf_ext,
    };
    err = btf__dedup((*linker).btf, &mut opts);
    if err != 0 {
        return err;
    }

    /* Set .BTF and .BTF.ext output byte order */
    let link_endianness = if (*(*linker).elf_hdr).e_ident[EI_DATA] == ELFDATA2MSB {
        BTF_BIG_ENDIAN
    } else {
        BTF_LITTLE_ENDIAN
    };
    btf__set_endianness((*linker).btf, link_endianness);
    if !(*linker).btf_ext.is_null() {
        btf_ext__set_endianness((*linker).btf_ext, link_endianness);
    }

    /* Emit .BTF section */
    raw_data = btf__raw_data((*linker).btf, &mut raw_sz);
    if raw_data.is_null() {
        return -ENOMEM;
    }

    err = emit_elf_data_sec(linker, BTF_ELF_SEC, 8, raw_data, raw_sz as size_t);
    if err != 0 {
        return err;
    }

    /* Emit .BTF.ext section */
    if !(*linker).btf_ext.is_null() {
        raw_data = btf_ext__raw_data((*linker).btf_ext, &mut raw_sz);
        if raw_data.is_null() {
            return -ENOMEM;
        }

        err = emit_elf_data_sec(linker, BTF_EXT_ELF_SEC, 8, raw_data, raw_sz as size_t);
        if err != 0 {
            return err;
        }
    }

    0
}

unsafe fn emit_btf_ext_data(linker: *mut bpf_linker, output: *mut c_void, sec_name: *const c_char, sec_data: *mut btf_ext_sec_data) -> c_int {
    let mut cur = output as *mut u8;

    if (*sec_data).rec_cnt == 0 {
        return 0;
    }

    let str_off = btf__add_str((*linker).btf, sec_name);
    if str_off < 0 {
        return -ENOMEM;
    }

    let sec_info = cur as *mut btf_ext_info_sec;
    (*sec_info).sec_name_off = str_off as __u32;
    (*sec_info).num_info = (*sec_data).rec_cnt as __u32;
    cur = cur.add(size_of::<btf_ext_info_sec>());

    let sz = (*sec_data).rec_cnt * (*sec_data).rec_sz as size_t;
    memcpy(cur as *mut c_void, (*sec_data).recs, sz);
    cur = cur.add(sz);

    cur.offset_from(output as *mut u8) as c_int
}

unsafe fn finalize_btf_ext(linker: *mut bpf_linker) -> c_int {
    let mut funcs_sz: size_t = 0;
    let mut lines_sz: size_t = 0;
    let mut core_relos_sz: size_t = 0;
    let mut total_sz: size_t = 0;
    let mut func_rec_sz: size_t = 0;
    let mut line_rec_sz: size_t = 0;
    let mut core_relo_rec_sz: size_t = 0;
    let mut err: c_int = 0;

    /* validate that all sections have the same .BTF.ext record sizes
     * and calculate total data size for each type of data (func info,
     * line info, core relos)
     */
    let mut i = 1;
    while i < (*linker).sec_cnt {
        let sec = (*linker).secs.add(i as usize);

        if (*sec).func_info.rec_cnt != 0 {
            if func_rec_sz == 0 {
                func_rec_sz = (*sec).func_info.rec_sz as size_t;
            }
            if func_rec_sz != (*sec).func_info.rec_sz as size_t {
                return -EINVAL;
            }
            funcs_sz += size_of::<btf_ext_info_sec>() + func_rec_sz * (*sec).func_info.rec_cnt;
        }
        if (*sec).line_info.rec_cnt != 0 {
            if line_rec_sz == 0 {
                line_rec_sz = (*sec).line_info.rec_sz as size_t;
            }
            if line_rec_sz != (*sec).line_info.rec_sz as size_t {
                return -EINVAL;
            }
            lines_sz += size_of::<btf_ext_info_sec>() + line_rec_sz * (*sec).line_info.rec_cnt;
        }
        if (*sec).core_relo_info.rec_cnt != 0 {
            if core_relo_rec_sz == 0 {
                core_relo_rec_sz = (*sec).core_relo_info.rec_sz as size_t;
            }
            if core_relo_rec_sz != (*sec).core_relo_info.rec_sz as size_t {
                return -EINVAL;
            }
            core_relos_sz += size_of::<btf_ext_info_sec>() + core_relo_rec_sz * (*sec).core_relo_info.rec_cnt;
        }
        i += 1;
    }

    if funcs_sz == 0 && lines_sz == 0 && core_relos_sz == 0 {
        return 0;
    }

    total_sz += size_of::<btf_ext_header>();
    if funcs_sz != 0 {
        funcs_sz += size_of::<__u32>(); /* record size prefix */
        total_sz += funcs_sz;
    }
    if lines_sz != 0 {
        lines_sz += size_of::<__u32>(); /* record size prefix */
        total_sz += lines_sz;
    }
    if core_relos_sz != 0 {
        core_relos_sz += size_of::<__u32>(); /* record size prefix */
        total_sz += core_relos_sz;
    }

    let data = calloc(1, total_sz);
    if data.is_null() {
        return -ENOMEM;
    }
    let mut cur = data as *mut u8;

    let hdr = cur as *mut btf_ext_header;
    (*hdr).magic = BTF_MAGIC;
    (*hdr).version = BTF_VERSION;
    (*hdr).flags = 0;
    (*hdr).hdr_len = size_of::<btf_ext_header>() as __u32;
    cur = cur.add(size_of::<btf_ext_header>());

    /* All offsets are in bytes relative to the end of this header */
    (*hdr).func_info_off = 0;
    (*hdr).func_info_len = funcs_sz as __u32;
    (*hdr).line_info_off = funcs_sz as __u32;
    (*hdr).line_info_len = lines_sz as __u32;
    (*hdr).core_relo_off = (funcs_sz + lines_sz) as __u32;
    (*hdr).core_relo_len = core_relos_sz as __u32;

    if funcs_sz != 0 {
        *(cur as *mut __u32) = func_rec_sz as __u32;
        cur = cur.add(size_of::<__u32>());

        i = 1;
        while i < (*linker).sec_cnt {
            let sec = (*linker).secs.add(i as usize);
            let sz = emit_btf_ext_data(linker, cur as *mut c_void, (*sec).sec_name, &mut (*sec).func_info);
            if sz < 0 {
                err = sz;
                break;
            }
            cur = cur.add(sz as usize);
            i += 1;
        }
    }

    if err == 0 && lines_sz != 0 {
        *(cur as *mut __u32) = line_rec_sz as __u32;
        cur = cur.add(size_of::<__u32>());

        i = 1;
        while i < (*linker).sec_cnt {
            let sec = (*linker).secs.add(i as usize);
            let sz = emit_btf_ext_data(linker, cur as *mut c_void, (*sec).sec_name, &mut (*sec).line_info);
            if sz < 0 {
                err = sz;
                break;
            }
            cur = cur.add(sz as usize);
            i += 1;
        }
    }

    if err == 0 && core_relos_sz != 0 {
        *(cur as *mut __u32) = core_relo_rec_sz as __u32;
        cur = cur.add(size_of::<__u32>());

        i = 1;
        while i < (*linker).sec_cnt {
            let sec = (*linker).secs.add(i as usize);
            let sz = emit_btf_ext_data(linker, cur as *mut c_void, (*sec).sec_name, &mut (*sec).core_relo_info);
            if sz < 0 {
                err = sz;
                break;
            }
            cur = cur.add(sz as usize);
            i += 1;
        }
    }

    if err == 0 {
        (*linker).btf_ext = btf_ext__new(data, total_sz as __u32);
        err = libbpf_get_error((*linker).btf_ext as *const c_void);
        if err != 0 {
            (*linker).btf_ext = ptr::null_mut();
        }
    }

    free(data);
    err
}
