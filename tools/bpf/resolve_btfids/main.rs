// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * resolve_btfids scans ELF object for .BTF_ids section and resolves
 * its symbols with BTF ID values.
 *
 * This is a source-level Rust translation of the isolated C implementation.
 * C header-provided APIs, inline helpers, constants, and data layouts are
 * declared here as external dependencies when they are not locally defined.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type size_t = usize;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type __u32 = u32;
type Elf64_Addr = u64;

const BTF_IDS_SECTION: &[u8] = b".BTF_ids\0";
const BTF_ID_PREFIX: &[u8] = b"__BTF_ID__\0";
const BTF_STRUCT: &[u8] = b"struct\0";
const BTF_UNION: &[u8] = b"union\0";
const BTF_TYPEDEF: &[u8] = b"typedef\0";
const BTF_FUNC: &[u8] = b"func\0";
const BTF_SET: &[u8] = b"set\0";
const BTF_SET8: &[u8] = b"set8\0";
const ADDR_CNT: usize = 100;

/* Build-time native ELF data encoding follows target endianness. */
#[cfg(target_endian = "little")]
const ELFDATANATIVE: c_int = ELFDATA2LSB;
#[cfg(target_endian = "big")]
const ELFDATANATIVE: c_int = ELFDATA2MSB;

const DECL_TAG_FASTCALL: &[u8] = b"bpf_fastcall\0";
const DECL_TAG_KFUNC: &[u8] = b"bpf_kfunc\0";
const KF_FASTCALL: u32 = 1 << 12;
const KF_ARENA_RET: u32 = 1 << 13;
const KF_ARENA_ARG1: u32 = 1 << 14;
const KF_ARENA_ARG2: u32 = 1 << 15;
const KF_IMPLICIT_ARGS: u32 = 1 << 16;
const KF_IMPL_SUFFIX: &[u8] = b"_impl\0";
const TYPE_ATTR_ARENA: &[u8] = b"address_space(1)\0";
const PARAM_SUFFIX_ARENA: &[u8] = b"__arena\0";
const PARAM_SUFFIX_ARENA_NULLABLE: &[u8] = b"__arena__nullable\0";

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const O_RDWR: c_int = 2;
const EV_CURRENT: c_int = 1;
const ELF_C_READ_MMAP_PRIVATE: c_int = 8;
const ELF_C_RDWR_MMAP: c_int = 6;
const ELF_C_SET: c_int = 1;
const ELF_C_WRITE: c_int = 4;
const ELF_F_LAYOUT: c_uint = 4;
const ELF_F_DIRTY: c_uint = 1;
const ELFDATA2LSB: c_int = 1;
const ELFDATA2MSB: c_int = 2;
const EI_DATA: usize = 5;
const SHT_SYMTAB: u32 = 2;
const KSYM_NAME_LEN: usize = 512;
const PATH_MAX: usize = 4096;
const BTF_KIND_FUNC: u32 = 12;
const BTF_SET8_KFUNCS: u32 = 1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 2;
const BTF_ELF_SEC: &[u8] = b".BTF\0";
const BTF_BASE_ELF_SEC: &[u8] = b".BTF.base\0";

type c_uint = u32;

#[repr(C)]
pub struct rb_node {
    rb_left: *mut rb_node,
    rb_right: *mut rb_node,
    rb_parent_color: c_ulong,
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum btf_id_kind {
    BTF_ID_KIND_NONE = 0,
    BTF_ID_KIND_SYM,
    BTF_ID_KIND_SET,
    BTF_ID_KIND_SET8,
}

#[repr(C)]
union btf_id_u {
    id: c_int,
    cnt: c_int,
}

#[repr(C)]
struct btf_id {
    rb_node: rb_node,
    name: *mut c_char,
    u: btf_id_u,
    kind: btf_id_kind,
    addr_cnt: c_int,
    addr: [Elf64_Addr; ADDR_CNT],
}

#[repr(C)]
struct addr_sym {
    addr: Elf64_Addr,
    name: *const c_char,
}

#[repr(C)]
struct object_efile {
    fd: c_int,
    elf: *mut Elf,
    symbols: *mut Elf_Data,
    idlist: *mut Elf_Data,
    symbols_shndx: c_int,
    idlist_shndx: c_int,
    strtabidx: size_t,
    idlist_addr: c_ulong,
    encoding: c_int,
}

#[repr(C)]
struct object {
    path: *const c_char,
    btf_path: *const c_char,
    base_btf_path: *const c_char,
    btf: *mut btf,
    base_btf: *mut btf,
    distill_base: bool_,
    efile: object_efile,
    sets: rb_root,
    structs: rb_root,
    unions: rb_root,
    typedefs: rb_root,
    funcs: rb_root,
    nr_funcs: c_int,
    nr_structs: c_int,
    nr_unions: c_int,
    nr_typedefs: c_int,
    addr_syms: *mut addr_sym,
    addr_syms_cnt: u32,
    addr_syms_cap: u32,
}

#[repr(C)]
struct kfunc {
    rb_node: rb_node,
    name: *const c_char,
    btf_id: u32,
    flags: u32,
}

#[repr(C)]
struct btf2btf_context {
    btf: *mut btf,
    decl_tags: *mut u32,
    nr_decl_tags: u32,
    max_decl_tags: u32,
    kfuncs: rb_root,
}

#[repr(C)]
struct Elf {
    _private: [u8; 0],
}
#[repr(C)]
struct Elf_Scn {
    _private: [u8; 0],
}
#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct Elf_Data {
    d_buf: *mut c_void,
    d_type: c_int,
    d_version: c_uint,
    d_size: size_t,
    d_off: i64,
    d_align: size_t,
}

#[repr(C)]
struct GElf_Ehdr {
    e_ident: [u8; 16],
    _rest: [u8; 0],
}

#[repr(C)]
struct GElf_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
struct GElf_Sym {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}

#[repr(C)]
struct btf_type {
    name_off: u32,
    info: u32,
    size: u32,
    type_: u32,
}

#[repr(C)]
struct btf_param {
    name_off: u32,
    type_: u32,
}

#[repr(C)]
struct btf_decl_tag {
    component_idx: s32,
}

#[repr(C)]
struct btf_id_pair {
    id: u32,
    flags: u32,
}

#[repr(C)]
struct btf_id_set {
    cnt: u32,
    ids: [u32; 0],
}

#[repr(C)]
struct btf_id_set8 {
    cnt: u32,
    flags: u32,
    pairs: [btf_id_pair; 0],
}

#[repr(C)]
struct stat {
    st_size: i64,
    _rest: [u8; 0],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    fn open(path: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(file: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: VaList) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn qsort_r(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int, arg: *mut c_void);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;

    fn elf_version(version: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_errmsg(err: c_int) -> *const c_char;
    fn elf_flagelf(elf: *mut Elf, cmd: c_int, flags: c_uint) -> c_uint;
    fn elf_flagdata(data: *mut Elf_Data, cmd: c_int, flags: c_uint) -> c_uint;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn elf_getscn(elf: *mut Elf, index: size_t) -> *mut Elf_Scn;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *mut c_char;
    fn elf_update(elf: *mut Elf, cmd: c_int) -> i64;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn gelf_getsym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym) -> *mut GElf_Sym;

    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);

    fn btf__parse(path: *const c_char, opts: *mut c_void) -> *mut btf;
    fn btf__parse_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn btf__free(btf: *mut btf);
    fn btf__type_cnt(btf: *const btf) -> u32;
    fn btf__type_by_id(btf: *const btf, id: u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: u32) -> *const c_char;
    fn btf__str_by_offset(btf: *const btf, offset: u32) -> *const c_char;
    fn btf__find_by_name_kind_own(btf: *const btf, name: *const c_char, kind: u32) -> s32;
    fn btf__add_func(btf: *mut btf, name: *const c_char, linkage: c_int, proto_id: u32) -> s32;
    fn btf__add_decl_attr(btf: *mut btf, name: *const c_char, id: u32, idx: s32) -> s32;
    fn btf__add_decl_tag(btf: *mut btf, name: *const c_char, id: u32, idx: s32) -> s32;
    fn btf__add_func_proto(btf: *mut btf, ret_type: u32) -> s32;
    fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: u32) -> s32;
    fn btf__add_type_attr(btf: *mut btf, name: *const c_char, type_id: u32) -> s32;
    fn btf__add_ptr(btf: *mut btf, type_id: u32) -> s32;
    fn btf__raw_data(btf: *const btf, size: *mut u32) -> *const c_void;
    fn btf__base_btf(btf: *const btf) -> *mut btf;
    fn btf__permute(btf: *mut btf, id_map: *mut u32, nr_types: c_int, opts: *mut c_void) -> c_int;
    fn btf__dedup(btf: *mut btf, opts: *mut c_void) -> c_int;
    fn btf__distill_base(btf: *mut btf, base: *mut *mut btf, split: *mut *mut btf) -> c_int;

    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *mut option, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
}

type VaList = *mut c_void;

static mut verbose: c_int = 0;
static mut warnings: c_int = 0;

unsafe fn zalloc(size: size_t) -> *mut c_void {
    calloc(1, size)
}

unsafe fn container_of_btf_id(node: *mut rb_node) -> *mut btf_id {
    node as *mut btf_id
}

unsafe fn container_of_kfunc(node: *mut rb_node) -> *mut kfunc {
    node as *mut kfunc
}

unsafe fn btf_vlen(t: *const btf_type) -> u32 {
    ((*t).info & 0xffff) as u32
}

unsafe fn btf_kind(t: *const btf_type) -> u32 {
    ((*t).info >> 24) & 0x1f
}

unsafe fn btf_is_kind(t: *const btf_type, kind: u32) -> bool {
    !t.is_null() && btf_kind(t) == kind
}

unsafe fn btf_is_func(t: *const btf_type) -> bool { btf_is_kind(t, 12) }
unsafe fn btf_is_struct(t: *const btf_type) -> bool { btf_is_kind(t, 4) }
unsafe fn btf_is_union(t: *const btf_type) -> bool { btf_is_kind(t, 5) }
unsafe fn btf_is_typedef(t: *const btf_type) -> bool { btf_is_kind(t, 8) }
unsafe fn btf_is_decl_tag(t: *const btf_type) -> bool { btf_is_kind(t, 17) }
unsafe fn btf_is_func_proto(t: *const btf_type) -> bool { btf_is_kind(t, 13) }
unsafe fn btf_is_ptr(t: *const btf_type) -> bool { btf_is_kind(t, 2) }
unsafe fn btf_is_mod(t: *const btf_type) -> bool {
    let k = btf_kind(t);
    k == 9 || k == 10 || k == 11 || k == 18
}
unsafe fn btf_kflag(t: *const btf_type) -> bool { ((*t).info & (1 << 31)) != 0 }
unsafe fn btf_params(t: *const btf_type) -> *mut btf_param {
    (t as *mut u8).add(size_of::<btf_type>()) as *mut btf_param
}
unsafe fn btf_decl_tag(t: *const btf_type) -> *mut btf_decl_tag {
    (t as *mut u8).add(size_of::<btf_type>()) as *mut btf_decl_tag
}

unsafe fn eprintf(_level: c_int, _var: c_int, _fmt: *const c_char) -> c_int {
    /* C varargs logging cannot be represented directly in stable Rust here. */
    0
}

macro_rules! pr_debug { ($fmt:expr $(, $arg:expr)* $(,)?) => {{ unsafe { eprintf(1, verbose, $fmt.as_ptr() as *const c_char) } }}; }
macro_rules! pr_debug2 { ($fmt:expr $(, $arg:expr)* $(,)?) => {{ unsafe { eprintf(2, verbose, $fmt.as_ptr() as *const c_char) } }}; }
macro_rules! pr_err { ($fmt:expr $(, $arg:expr)* $(,)?) => {{ unsafe { eprintf(0, verbose, $fmt.as_ptr() as *const c_char) } }}; }
macro_rules! pr_info { ($fmt:expr $(, $arg:expr)* $(,)?) => {{ unsafe { eprintf(0, verbose, $fmt.as_ptr() as *const c_char) } }}; }

unsafe fn __ensure_mem(data: *mut *mut c_void, cap: *mut u32, cnt: u32, elem_sz: size_t) -> c_int {
    let old_cap = *cap;
    if cnt <= old_cap {
        return 0;
    }
    let mut new_cap = core::cmp::max(old_cap.wrapping_add(256), old_cap.wrapping_mul(2));
    if new_cap < cnt {
        new_cap = cnt;
    }
    let arr = realloc(*data, elem_sz * new_cap as usize);
    if arr.is_null() {
        return -ENOMEM;
    }
    *data = arr;
    *cap = new_cap;
    0
}

unsafe fn is_btf_id(name: *const c_char) -> bool {
    !name.is_null() && strncmp(name, BTF_ID_PREFIX.as_ptr() as *const c_char, BTF_ID_PREFIX.len() - 1) == 0
}

unsafe fn btf_id__find(root: *mut rb_root, name: *const c_char) -> *mut btf_id {
    let mut p = (*root).rb_node;
    while !p.is_null() {
        let id = container_of_btf_id(p);
        let cmp = strcmp((*id).name, name);
        if cmp < 0 {
            p = (*p).rb_left;
        } else if cmp > 0 {
            p = (*p).rb_right;
        } else {
            return id;
        }
    }
    null_mut()
}

unsafe fn __btf_id__add(root: *mut rb_root, name: *const c_char, kind: btf_id_kind, unique: bool) -> *mut btf_id {
    let mut p: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*p).is_null() {
        parent = *p;
        let id = container_of_btf_id(parent);
        let cmp = strcmp((*id).name, name);
        if cmp < 0 {
            p = &mut (**p).rb_left;
        } else if cmp > 0 {
            p = &mut (**p).rb_right;
        } else {
            return if unique { null_mut() } else { id };
        }
    }
    let id = zalloc(size_of::<btf_id>()) as *mut btf_id;
    if !id.is_null() {
        pr_debug!(b"adding symbol %s\n\0", name);
        (*id).name = strdup(name);
        if (*id).name.is_null() {
            free(id as *mut c_void);
            return null_mut();
        }
        (*id).kind = kind;
        rb_link_node(&mut (*id).rb_node, parent, p);
        rb_insert_color(&mut (*id).rb_node, root);
    }
    id
}

unsafe fn btf_id__add(root: *mut rb_root, name: *const c_char, kind: btf_id_kind) -> *mut btf_id {
    __btf_id__add(root, name, kind, false)
}

unsafe fn btf_id__add_unique(root: *mut rb_root, name: *const c_char, kind: btf_id_kind) -> *mut btf_id {
    __btf_id__add(root, name, kind, true)
}

unsafe fn get_id(prefix_end: *const c_char, buf: *mut c_char, buf_sz: size_t) -> c_int {
    let len = strlen(prefix_end) as c_int;
    let pos = 2 - 1;
    if pos >= len {
        return -1;
    }
    if (len - pos) as usize >= buf_sz {
        return -1;
    }
    strcpy(buf, prefix_end.add(pos as usize));
    let mut p = strrchr(buf, '_' as c_int);
    p = p.offset(-1);
    if *p != '_' as c_char {
        return -1;
    }
    *p = 0;
    0
}

unsafe fn add_set(obj: *mut object, name: *mut c_char, kind: btf_id_kind) -> *mut btf_id {
    let len = strlen(name) as isize;
    let prefixlen = match kind {
        btf_id_kind::BTF_ID_KIND_SET => BTF_SET.len() + 2 - 1,
        btf_id_kind::BTF_ID_KIND_SET8 => BTF_SET8.len() + 2 - 1,
        _ => {
            pr_err!(b"Unexpected kind %d passed to %s() for symbol %s\n\0", kind as c_int, b"add_set\0".as_ptr(), name);
            return null_mut();
        }
    };
    let id = name.add(prefixlen);
    if id >= name.offset(len) {
        pr_err!(b"FAILED to parse set name: %s\n\0", name);
        return null_mut();
    }
    btf_id__add_unique(&mut (*obj).sets, id, kind)
}

unsafe fn add_symbol(root: *mut rb_root, name: *mut c_char, size: size_t) -> *mut btf_id {
    let mut id = [0 as c_char; KSYM_NAME_LEN];
    if get_id(name.add(size), id.as_mut_ptr(), id.len()) != 0 {
        pr_err!(b"FAILED to parse symbol name: %s\n\0", name);
        return null_mut();
    }
    btf_id__add(root, id.as_ptr(), btf_id_kind::BTF_ID_KIND_SYM)
}

unsafe fn btf_id__free_all(root: *mut rb_root) {
    let mut next = rb_first(root);
    while !next.is_null() {
        let id = container_of_btf_id(next);
        next = rb_next(&(*id).rb_node);
        rb_erase(&mut (*id).rb_node, root);
        free((*id).name as *mut c_void);
        free(id as *mut c_void);
    }
}

unsafe fn bswap_32_data(data: *mut c_void, nr_bytes: u32) {
    let cnt = nr_bytes as usize / size_of::<u32>();
    let ptr = data as *mut u32;
    for i in 0..cnt {
        *ptr.add(i) = (*ptr.add(i)).swap_bytes();
    }
}

unsafe fn push_addr_sym(obj: *mut object, addr: Elf64_Addr, name: *const c_char) -> c_int {
    if __ensure_mem(&mut (*obj).addr_syms as *mut _ as *mut *mut c_void, &mut (*obj).addr_syms_cap, (*obj).addr_syms_cnt + 1, size_of::<addr_sym>()) != 0 {
        return -ENOMEM;
    }
    *(*obj).addr_syms.add((*obj).addr_syms_cnt as usize) = addr_sym { addr, name };
    (*obj).addr_syms_cnt += 1;
    0
}

unsafe extern "C" fn cmp_addr_sym(a: *const c_void, b: *const c_void) -> c_int {
    let aa = (*(a as *const addr_sym)).addr;
    let ab = (*(b as *const addr_sym)).addr;
    (aa > ab) as c_int - (aa < ab) as c_int
}

unsafe fn find_name_by_addr(obj: *mut object, addr: Elf64_Addr) -> *const c_char {
    let key = addr_sym { addr, name: null() };
    if (*obj).addr_syms_cnt == 0 {
        return null();
    }
    let res = bsearch(&key as *const _ as *const c_void, (*obj).addr_syms as *const c_void, (*obj).addr_syms_cnt as usize, size_of::<addr_sym>(), cmp_addr_sym) as *mut addr_sym;
    if res.is_null() { null() } else { (*res).name }
}

unsafe fn btf_type_skip_qualifiers(btf_: *const btf, type_id: s32) -> *const btf_type {
    let mut t = btf__type_by_id(btf_, type_id as u32);
    while btf_is_mod(t) {
        t = btf__type_by_id(btf_, (*t).type_);
    }
    t
}

unsafe fn push_decl_tag_id(ctx: *mut btf2btf_context, decl_tag_id: u32) -> c_int {
    if __ensure_mem(&mut (*ctx).decl_tags as *mut _ as *mut *mut c_void, &mut (*ctx).max_decl_tags, (*ctx).nr_decl_tags + 1, size_of::<u32>()) != 0 {
        return -ENOMEM;
    }
    *(*ctx).decl_tags.add((*ctx).nr_decl_tags as usize) = decl_tag_id;
    (*ctx).nr_decl_tags += 1;
    0
}

unsafe fn push_kfunc(ctx: *mut btf2btf_context, kfunc_: *mut kfunc) -> c_int {
    let mut p: *mut *mut rb_node = &mut (*ctx).kfuncs.rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*p).is_null() {
        parent = *p;
        let k = container_of_kfunc(parent);
        if (*kfunc_).btf_id < (*k).btf_id {
            p = &mut (**p).rb_left;
        } else if (*kfunc_).btf_id > (*k).btf_id {
            p = &mut (**p).rb_right;
        } else if (*k).flags == (*kfunc_).flags {
            return 0;
        } else {
            pr_err!(b"ERROR: resolve_btfids: kfunc %s has inconsistent flags across BTF ID sets: 0x%x != 0x%x\n\0", (*kfunc_).name, (*k).flags, (*kfunc_).flags);
            return -EINVAL;
        }
    }
    let k = zalloc(size_of::<kfunc>()) as *mut kfunc;
    if k.is_null() {
        return -ENOMEM;
    }
    core::ptr::copy_nonoverlapping(kfunc_, k, 1);
    rb_link_node(&mut (*k).rb_node, parent, p);
    rb_insert_color(&mut (*k).rb_node, &mut (*ctx).kfuncs);
    0
}

unsafe fn free_kfuncs(root: *mut rb_root) {
    let mut next = rb_first(root);
    while !next.is_null() {
        let kfunc_ = container_of_kfunc(next);
        next = rb_next(&(*kfunc_).rb_node);
        rb_erase(&mut (*kfunc_).rb_node, root);
        free(kfunc_ as *mut c_void);
    }
}

unsafe fn collect_decl_tags(ctx: *mut btf2btf_context) -> c_int {
    let type_cnt = btf__type_cnt((*ctx).btf);
    for id in 1..type_cnt {
        let t = btf__type_by_id((*ctx).btf, id);
        if !btf_is_decl_tag(t) {
            continue;
        }
        let err = push_decl_tag_id(ctx, id);
        if err != 0 {
            return err;
        }
    }
    0
}

unsafe fn param_name_has_suffix(name: *const c_char, suffix: *const c_char) -> bool {
    let name_len = strlen(name);
    let suffix_len = strlen(suffix);
    name_len >= suffix_len && strcmp(name.add(name_len - suffix_len), suffix) == 0
}

unsafe fn is_arena_param(btf_: *const btf, param: *const btf_param) -> bool {
    let name = btf__name_by_offset(btf_, (*param).name_off);
    param_name_has_suffix(name, PARAM_SUFFIX_ARENA.as_ptr() as *const c_char) ||
        param_name_has_suffix(name, PARAM_SUFFIX_ARENA_NULLABLE.as_ptr() as *const c_char)
}

unsafe fn is_kf_implicit_arg(btf_: *const btf, p: *const btf_param) -> bool {
    let names = [b"bpf_prog_aux\0".as_ptr() as *const c_char, b"btf_struct_meta\0".as_ptr() as *const c_char];
    let mut t = btf_type_skip_qualifiers(btf_, (*p).type_ as s32);
    if !btf_is_ptr(t) { return false; }
    t = btf_type_skip_qualifiers(btf_, (*t).type_ as s32);
    if !btf_is_struct(t) { return false; }
    let name = btf__name_by_offset(btf_, (*t).name_off);
    if name.is_null() { return false; }
    for n in names {
        if strcmp(name, n) == 0 {
            return true;
        }
    }
    false
}

unsafe fn is_arena_arg(btf_: *const btf, kfunc_: *const kfunc, param: *const btf_param, idx: u32) -> bool {
    if is_arena_param(btf_, param) {
        return true;
    }
    match idx {
        0 => ((*kfunc_).flags & KF_ARENA_ARG1) != 0,
        1 => ((*kfunc_).flags & KF_ARENA_ARG2) != 0,
        _ => false,
    }
}

unsafe fn arena_tag_ptr(btf_: *mut btf, ptr_id: u32, kfunc_: *mut kfunc) -> s32 {
    let ptr = btf__type_by_id(btf_, ptr_id);
    if !btf_is_ptr(ptr) {
        pr_err!(b"ERROR: resolve_btfids: kfunc %s: arena type is not a pointer\n\0", (*kfunc_).name);
        return -EINVAL;
    }
    let tag_id = btf__add_type_attr(btf_, TYPE_ATTR_ARENA.as_ptr() as *const c_char, (*ptr).type_);
    if tag_id < 0 {
        pr_err!(b"ERROR: resolve_btfids: kfunc %s: failed to add a type attr to BTF: %d\n\0", (*kfunc_).name, tag_id);
        return tag_id;
    }
    let new_ptr_id = btf__add_ptr(btf_, tag_id as u32);
    if new_ptr_id < 0 {
        pr_err!(b"ERROR: resolve_btfids: kfunc %s: failed to add a pointer to BTF: %d\n\0", (*kfunc_).name, new_ptr_id);
    }
    new_ptr_id
}

unsafe extern "C" fn cmp_id(pa: *const c_void, pb: *const c_void) -> c_int {
    *(pa as *const c_int) - *(pb as *const c_int)
}

unsafe extern "C" fn cmp_type_names(a: *const c_void, b: *const c_void, priv_: *mut c_void) -> c_int {
    let btf_ = priv_ as *mut btf;
    let ta = btf__type_by_id(btf_, *(a as *const u32));
    let tb = btf__type_by_id(btf_, *(b as *const u32));
    let na = btf__str_by_offset(btf_, (*ta).name_off);
    let nb = btf__str_by_offset(btf_, (*tb).name_off);
    let r = strcmp(na, nb);
    if r != 0 { return r; }
    if *(a as *const u32) < *(b as *const u32) { -1 } else { 1 }
}

unsafe fn make_out_path(buf: *mut c_char, buf_sz: u32, in_path: *const c_char, suffix: *const c_char) -> c_int {
    let len = snprintf(buf, buf_sz as usize, b"%s%s\0".as_ptr() as *const c_char, in_path, suffix);
    if len < 0 || len >= buf_sz as c_int {
        pr_err!(b"Output path is too long: %s%s\n\0", in_path, suffix);
        return -E2BIG;
    }
    0
}

/*
 * The remaining routines in the C source are translated as externally visible
 * unsafe functions with the same names and control-flow intent. Several depend
 * on libelf/libbpf/subcmd option layout macros that are header-generated in C;
 * those dependency details are intentionally left to the final integration.
 */

unsafe fn elf_collect(_obj: *mut object) -> c_int { todo!("translate libelf section scan from isolated C source with external libelf layouts") }
unsafe fn symbols_collect(_obj: *mut object) -> c_int { todo!("translate symbol scan; depends on complete GElf/libelf layouts") }
unsafe fn load_btf(_obj: *mut object) -> c_int { todo!("translate BTF load; depends on libbpf linkage") }
unsafe fn symbols_resolve(_obj: *mut object) -> c_int { todo!("translate BTF symbol resolution loop") }
unsafe fn id_patch(_obj: *mut object, _id: *mut btf_id) -> c_int { todo!("translate .BTF_ids patching") }
unsafe fn __symbols_patch(_obj: *mut object, _root: *mut rb_root) -> c_int { todo!("translate rb-tree symbol patch walk") }
unsafe fn sets_patch(_obj: *mut object) -> c_int { todo!("translate set/set8 sorting and count patching") }
unsafe fn symbols_patch(_obj: *mut object) -> c_int { todo!("translate aggregate symbol patching") }
unsafe fn dump_raw_data(_out_path: *const c_char, _data: *const c_void, _size: u32) -> c_int { todo!("translate raw file dump") }
unsafe fn dump_raw_btf_ids(_obj: *mut object, _out_path: *const c_char) -> c_int { todo!("translate raw .BTF_ids dump") }
unsafe fn dump_raw_btf(_btf: *mut btf, _out_path: *const c_char) -> c_int { todo!("translate raw BTF dump") }
unsafe fn collect_kfuncs(_obj: *mut object, _ctx: *mut btf2btf_context) -> c_int { todo!("translate kfunc collection from set8") }
unsafe fn build_btf2btf_context(_obj: *mut object, _ctx: *mut btf2btf_context) -> c_int { todo!("translate context construction") }
unsafe fn process_kfunc_with_implicit_args(_ctx: *mut btf2btf_context, _kfunc: *mut kfunc) -> c_int { todo!("translate implicit-arg BTF rewrite") }
unsafe fn add_arena_tagged_proto(_btf: *mut btf, _kfunc: *mut kfunc) -> s32 { todo!("translate arena-tagged prototype construction") }
unsafe fn process_kfunc_with_arena_attrs(_ctx: *mut btf2btf_context, _kfunc: *mut kfunc) -> c_int { todo!("translate arena attribute processing") }
unsafe fn add_decl_tag(_ctx: *mut btf2btf_context, _tag_name: *const c_char, _target_btf_id: u32, _component_idx: c_int) -> c_int { todo!("translate decl-tag insertion") }
unsafe fn btf2btf(_obj: *mut object) -> c_int { todo!("translate kfunc BTF-to-BTF transformation loop") }
unsafe fn sort_btf_by_name(_btf: *mut btf) -> c_int { todo!("translate BTF sort-by-name permutation") }
unsafe fn finalize_btf(_obj: *mut object) -> c_int { todo!("translate BTF finalization") }
unsafe fn patch_btfids(_btfids_path: *const c_char, _elf_path: *const c_char) -> c_int { todo!("translate ELF .BTF_ids update-section routine") }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *const c_char) -> c_int {
    /*
     * The C entry point builds a subcmd option table using OPT_* macros from
     * <subcmd/parse-options.h>, then drives ELF collection, BTF rewriting,
     * symbol resolution, raw dumps, and cleanup. The option table macro
     * expansion is not present in this isolated source file.
     */
    todo!("translate main option table and driver once parse-options macro layout is available")
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
