// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2019 Facebook */

// Translation of bpf/bpftool/btf.c. C includes are represented by external
// declarations and opaque types supplied by the surrounding translated tree.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type va_list = *mut c_void;

const KFUNC_DECL_TAG: &[u8] = b"bpf_kfunc\0";
const FASTCALL_DECL_TAG: &[u8] = b"bpf_fastcall\0";

const MAX_ROOT_IDS: usize = 16;
const MAX_BTF_FILES: usize = 64;
const BTF_NAME_BUFF_LEN: usize = 64;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const ENOTSUP: c_int = 95;
const ENOENT: c_int = 2;

const BPF_F_RDONLY: __u32 = 1 << 3;

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
const BTF_KIND_DECL_TAG: c_int = 17;
const BTF_KIND_TYPE_TAG: c_int = 18;
const BTF_KIND_ENUM64: c_int = 19;
const BTF_KIND_MAX: c_int = BTF_KIND_ENUM64;
const NR_BTF_KINDS: usize = 20;

const BTF_INT_SIGNED: __u8 = 1;
const BTF_INT_CHAR: __u8 = 2;
const BTF_INT_BOOL: __u8 = 4;
const BTF_VAR_STATIC: __u32 = 0;
const BTF_VAR_GLOBAL_ALLOCATED: __u32 = 1;
const BTF_VAR_GLOBAL_EXTERN: __u32 = 2;
const BTF_FUNC_STATIC: __u32 = 0;
const BTF_FUNC_GLOBAL: __u32 = 1;
const BTF_FUNC_EXTERN: __u32 = 2;

const BPF_OBJ_UNKNOWN: bpf_obj_type = 0;
const BPF_OBJ_PROG: bpf_obj_type = 1;
const BPF_OBJ_MAP: bpf_obj_type = 2;
const BPF_OBJ_BTF: bpf_obj_type = 4;
type bpf_obj_type = c_uint;

const SYSFS_VMLINUX: &[u8] = b"/sys/kernel/btf/vmlinux\0";
const SYSFS_PREFIX: &[u8] = b"/sys/kernel/btf/\0";

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_ulong,
    pub value: c_ulong,
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
pub struct btf_enum {
    pub name_off: __u32,
    pub val: i32,
}

#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
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
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_decl_tag {
    pub component_idx: i32,
}

#[repr(C)]
pub struct bpf_btf_info {
    pub btf: __u64,
    pub btf_size: __u32,
    pub id: __u32,
    pub name: __u64,
    pub name_len: __u32,
    pub kernel_btf: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub _pad0: [u8; 64],
    pub btf_id: __u32,
}

#[repr(C)]
pub struct bpf_map_info {
    pub _pad0: [u8; 64],
    pub btf_id: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
}

#[repr(C)]
pub struct bpf_get_fd_by_id_opts {
    pub sz: size_t,
    pub open_flags: __u32,
}

#[repr(C)]
pub struct btf_dump_emit_type_decl_opts {
    pub sz: size_t,
    pub field_name: *const c_char,
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

#[repr(C)]
struct sort_datum {
    index: c_int,
    type_rank: c_int,
    sort_name: *const c_char,
    own_name: *const c_char,
    disambig_hash: __u64,
}

#[repr(C)]
struct ptr_array {
    cnt: __u32,
    cap: __u32,
    elems: *mut *const c_void,
}

static BTF_KIND_STR: [*const c_char; NR_BTF_KINDS] = [
    b"UNKNOWN\0".as_ptr() as *const c_char,
    b"INT\0".as_ptr() as *const c_char,
    b"PTR\0".as_ptr() as *const c_char,
    b"ARRAY\0".as_ptr() as *const c_char,
    b"STRUCT\0".as_ptr() as *const c_char,
    b"UNION\0".as_ptr() as *const c_char,
    b"ENUM\0".as_ptr() as *const c_char,
    b"FWD\0".as_ptr() as *const c_char,
    b"TYPEDEF\0".as_ptr() as *const c_char,
    b"VOLATILE\0".as_ptr() as *const c_char,
    b"CONST\0".as_ptr() as *const c_char,
    b"RESTRICT\0".as_ptr() as *const c_char,
    b"FUNC\0".as_ptr() as *const c_char,
    b"FUNC_PROTO\0".as_ptr() as *const c_char,
    b"VAR\0".as_ptr() as *const c_char,
    b"DATASEC\0".as_ptr() as *const c_char,
    b"FLOAT\0".as_ptr() as *const c_char,
    b"DECL_TAG\0".as_ptr() as *const c_char,
    b"TYPE_TAG\0".as_ptr() as *const c_char,
    b"ENUM64\0".as_ptr() as *const c_char,
];

unsafe extern "C" {
    static mut json_wtr: *mut json_writer_t;
    static mut json_output: bool;
    static mut errno: c_int;
    static mut base_btf: *mut btf;
    static mut refs_table: *mut hashmap;
    static bin_name: *const c_char;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn vfprintf(stream: *mut c_void, fmt: *const c_char, args: va_list) -> c_int;
    static mut stdout: *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn qsort_r(base: *mut c_void, nmemb: size_t, size: size_t,
               compar: unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int,
               arg: *mut c_void);

    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__base_btf(btf: *const btf) -> *const btf;
    fn btf__parse(path: *const c_char, opts: *const c_void) -> *mut btf;
    fn btf__parse_split(path: *const c_char, base: *mut btf) -> *mut btf;
    fn btf__new_empty_split(base: *mut btf) -> *mut btf;
    fn btf__add_btf(dst: *mut btf, src: *mut btf) -> c_int;
    fn btf__dedup(btf: *mut btf, opts: *const c_void) -> c_int;
    fn btf__load_from_kernel_by_id_split(id: __u32, base: *mut btf) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf_dump__new(btf: *const btf,
                     printf_fn: unsafe extern "C" fn(*mut c_void, *const c_char, va_list),
                     ctx: *mut c_void,
                     opts: *const c_void) -> *mut btf_dump;
    fn btf_dump__free(d: *mut btf_dump);
    fn btf_dump__dump_type(d: *mut btf_dump, id: __u32) -> c_int;
    fn btf_dump__emit_type_decl(d: *mut btf_dump, id: __u32,
                                opts: *mut btf_dump_emit_type_decl_opts) -> c_int;

    fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_btf_get_info_by_fd(fd: c_int, info: *mut bpf_btf_info, len: *mut __u32) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut c_void, len: *mut __u32) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut c_void, len: *mut __u32) -> c_int;
    fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_btf_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;

    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_uint_field(w: *mut json_writer_t, name: *const c_char, val: __u64);
    fn jsonw_int_field(w: *mut json_writer_t, name: *const c_char, val: i64);
    fn jsonw_string_field(w: *mut json_writer_t, name: *const c_char, val: *const c_char);
    fn jsonw_bool_field(w: *mut json_writer_t, name: *const c_char, val: bool);
    fn jsonw_uint(w: *mut json_writer_t, val: __u64);
    fn jsonw_null(w: *mut json_writer_t);

    fn p_err(fmt: *const c_char, ...);
    fn p_info(fmt: *const c_char, ...);
    fn usage();
    fn is_prefix(str: *const c_char, prefix: *const c_char) -> bool;
    fn map_parse_fd_and_info(argc: *mut c_int, argv: *mut *mut *mut c_char,
                             info: *mut bpf_map_info, len: *mut __u32, flags: __u32) -> c_int;
    fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn hashmap__new(hash_fn: *const c_void, equal_fn: *const c_void, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__append(map: *mut hashmap, key: __u32, value: __u32) -> c_int;
    fn hashmap__for_each_key_entry(map: *mut hashmap, entry: *mut *mut hashmap_entry, key: __u32) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn hash_fn_for_key_as_id() -> c_void;
    fn equal_fn_for_key_as_id() -> c_void;
    fn build_obj_refs_table(tab: *mut *mut hashmap, typ: bpf_obj_type);
    fn delete_obj_refs_table(tab: *mut hashmap);
    fn emit_obj_refs_plain(tab: *mut hashmap, id: __u32, prefix: *const c_char);
    fn emit_obj_refs_json(tab: *mut hashmap, id: __u32, w: *mut json_writer_t);
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char,
                  help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;
    fn str_hash(str: *const c_char) -> __u64;
}

#[inline]
unsafe fn btf_kind(t: *const btf_type) -> c_int { (((*t).info >> 24) & 0x1f) as c_int }
#[inline]
unsafe fn btf_vlen(t: *const btf_type) -> __u32 { (*t).info & 0xffff }
#[inline]
unsafe fn btf_kflag(t: *const btf_type) -> bool { ((*t).info & (1u32 << 31)) != 0 }
#[inline]
unsafe fn btf_is_enum(t: *const btf_type) -> bool { btf_kind(t) == BTF_KIND_ENUM }
#[inline]
unsafe fn btf_is_decl_tag(t: *const btf_type) -> bool { btf_kind(t) == BTF_KIND_DECL_TAG }
#[inline]
unsafe fn btf_is_func(t: *const btf_type) -> bool { btf_kind(t) == BTF_KIND_FUNC }
#[inline]
unsafe fn btf_array_ptr(t: *const btf_type) -> *mut btf_array { t.add(1) as *mut btf_array }
#[inline]
unsafe fn btf_members(t: *const btf_type) -> *const btf_member { t.add(1) as *const btf_member }
#[inline]
unsafe fn btf_enum_ptr(t: *const btf_type) -> *const btf_enum { t.add(1) as *const btf_enum }
#[inline]
unsafe fn btf_enum64_ptr(t: *const btf_type) -> *const btf_enum64 { t.add(1) as *const btf_enum64 }
#[inline]
unsafe fn btf_decl_tag_ptr(t: *const btf_type) -> *const btf_decl_tag { t.add(1) as *const btf_decl_tag }
#[inline]
fn BTF_INT_ENCODING(v: __u32) -> __u8 { ((v & 0x0f000000) >> 24) as __u8 }
#[inline]
fn BTF_INT_OFFSET(v: __u32) -> __u32 { (v & 0x00ff0000) >> 16 }
#[inline]
fn BTF_INT_BITS(v: __u32) -> __u32 { v & 0x000000ff }
#[inline]
fn BTF_MEMBER_BIT_OFFSET(v: __u32) -> __u32 { v & 0x00ffffff }
#[inline]
fn BTF_MEMBER_BITFIELD_SIZE(v: __u32) -> __u32 { v >> 24 }
#[inline]
fn ptr_to_u64<T>(ptr: *mut T) -> __u64 { ptr as usize as __u64 }
#[inline]
unsafe fn u64_to_ptr<T>(val: __u64) -> *mut T { val as usize as *mut T }
#[inline]
unsafe fn cstr_or_invalid(p: *const c_char) -> *const c_char {
    if p.is_null() { b"(invalid)\0".as_ptr() as *const c_char } else { p }
}

unsafe fn next_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) {
    *argc -= 1;
    *argv = (*argv).add(1);
}

unsafe fn req_args(argc: c_int, n: c_int) -> bool { argc >= n }
unsafe fn get_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) -> *mut c_char {
    let arg = **argv;
    next_arg(argc, argv);
    arg
}
unsafe fn bad_arg() -> c_int { -1 }

unsafe fn btf_int_enc_str(encoding: __u8) -> *const c_char {
    match encoding {
        0 => b"(none)\0".as_ptr() as *const c_char,
        BTF_INT_SIGNED => b"SIGNED\0".as_ptr() as *const c_char,
        BTF_INT_CHAR => b"CHAR\0".as_ptr() as *const c_char,
        BTF_INT_BOOL => b"BOOL\0".as_ptr() as *const c_char,
        _ => b"UNKN\0".as_ptr() as *const c_char,
    }
}

unsafe fn btf_var_linkage_str(linkage: __u32) -> *const c_char {
    match linkage {
        BTF_VAR_STATIC => b"static\0".as_ptr() as *const c_char,
        BTF_VAR_GLOBAL_ALLOCATED => b"global\0".as_ptr() as *const c_char,
        BTF_VAR_GLOBAL_EXTERN => b"extern\0".as_ptr() as *const c_char,
        _ => b"(unknown)\0".as_ptr() as *const c_char,
    }
}

unsafe fn btf_func_linkage_str(t: *const btf_type) -> *const c_char {
    match btf_vlen(t) {
        BTF_FUNC_STATIC => b"static\0".as_ptr() as *const c_char,
        BTF_FUNC_GLOBAL => b"global\0".as_ptr() as *const c_char,
        BTF_FUNC_EXTERN => b"extern\0".as_ptr() as *const c_char,
        _ => b"(unknown)\0".as_ptr() as *const c_char,
    }
}

unsafe fn btf_str(btf_: *const btf, off: __u32) -> *const c_char {
    if off == 0 {
        return b"(anon)\0".as_ptr() as *const c_char;
    }
    cstr_or_invalid(btf__name_by_offset(btf_, off))
}

fn btf_kind_safe(kind: c_int) -> c_int {
    if kind <= BTF_KIND_MAX { kind } else { BTF_KIND_UNKN }
}

unsafe fn dump_btf_type(btf_: *const btf, id: __u32, t: *const btf_type) -> c_int {
    let w = json_wtr;
    let kind = btf_kind(t);
    if json_output {
        jsonw_start_object(w);
        jsonw_uint_field(w, b"id\0".as_ptr() as *const c_char, id as __u64);
        jsonw_string_field(w, b"kind\0".as_ptr() as *const c_char, BTF_KIND_STR[btf_kind_safe(kind) as usize]);
        jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, btf_str(btf_, (*t).name_off));
    } else {
        printf(b"[%u] %s '%s'\0".as_ptr() as *const c_char, id, BTF_KIND_STR[btf_kind_safe(kind) as usize], btf_str(btf_, (*t).name_off));
    }

    match kind {
        BTF_KIND_INT => {
            let v = *(t.add(1) as *const __u32);
            let enc = btf_int_enc_str(BTF_INT_ENCODING(v));
            if json_output {
                jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64);
                jsonw_uint_field(w, b"bits_offset\0".as_ptr() as *const c_char, BTF_INT_OFFSET(v) as __u64);
                jsonw_uint_field(w, b"nr_bits\0".as_ptr() as *const c_char, BTF_INT_BITS(v) as __u64);
                jsonw_string_field(w, b"encoding\0".as_ptr() as *const c_char, enc);
            } else {
                printf(b" size=%u bits_offset=%u nr_bits=%u encoding=%s\0".as_ptr() as *const c_char, (*t).size, BTF_INT_OFFSET(v), BTF_INT_BITS(v), enc);
            }
        }
        BTF_KIND_PTR | BTF_KIND_CONST | BTF_KIND_VOLATILE | BTF_KIND_RESTRICT |
        BTF_KIND_TYPEDEF | BTF_KIND_TYPE_TAG => {
            if json_output {
                jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*t).type_ as __u64);
            } else {
                printf(b" type_id=%u\0".as_ptr() as *const c_char, (*t).type_);
            }
        }
        BTF_KIND_ARRAY => {
            let arr = btf_array_ptr(t);
            if json_output {
                jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*arr).type_ as __u64);
                jsonw_uint_field(w, b"index_type_id\0".as_ptr() as *const c_char, (*arr).index_type as __u64);
                jsonw_uint_field(w, b"nr_elems\0".as_ptr() as *const c_char, (*arr).nelems as __u64);
            } else {
                printf(b" type_id=%u index_type_id=%u nr_elems=%u\0".as_ptr() as *const c_char, (*arr).type_, (*arr).index_type, (*arr).nelems);
            }
        }
        BTF_KIND_STRUCT | BTF_KIND_UNION => {
            let mut m = t.add(1) as *const btf_member;
            let vlen = btf_vlen(t);
            if json_output {
                jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64);
                jsonw_uint_field(w, b"vlen\0".as_ptr() as *const c_char, vlen as __u64);
                jsonw_name(w, b"members\0".as_ptr() as *const c_char);
                jsonw_start_array(w);
            } else {
                printf(b" size=%u vlen=%u\0".as_ptr() as *const c_char, (*t).size, vlen);
            }
            for _ in 0..vlen {
                let name = btf_str(btf_, (*m).name_off);
                let (bit_off, bit_sz) = if btf_kflag(t) {
                    (BTF_MEMBER_BIT_OFFSET((*m).offset), BTF_MEMBER_BITFIELD_SIZE((*m).offset))
                } else {
                    ((*m).offset, 0)
                };
                if json_output {
                    jsonw_start_object(w);
                    jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, name);
                    jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*m).type_ as __u64);
                    jsonw_uint_field(w, b"bits_offset\0".as_ptr() as *const c_char, bit_off as __u64);
                    if bit_sz != 0 { jsonw_uint_field(w, b"bitfield_size\0".as_ptr() as *const c_char, bit_sz as __u64); }
                    jsonw_end_object(w);
                } else {
                    printf(b"\n\t'%s' type_id=%u bits_offset=%u\0".as_ptr() as *const c_char, name, (*m).type_, bit_off);
                    if bit_sz != 0 { printf(b" bitfield_size=%u\0".as_ptr() as *const c_char, bit_sz); }
                }
                m = m.add(1);
            }
            if json_output { jsonw_end_array(w); }
        }
        BTF_KIND_ENUM => {
            let mut v = t.add(1) as *const btf_enum;
            let vlen = btf_vlen(t);
            let encoding = if btf_kflag(t) { b"SIGNED\0" } else { b"UNSIGNED\0" };
            if json_output {
                jsonw_string_field(w, b"encoding\0".as_ptr() as *const c_char, encoding.as_ptr() as *const c_char);
                jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64);
                jsonw_uint_field(w, b"vlen\0".as_ptr() as *const c_char, vlen as __u64);
                jsonw_name(w, b"values\0".as_ptr() as *const c_char);
                jsonw_start_array(w);
            } else {
                printf(b" encoding=%s size=%u vlen=%u\0".as_ptr() as *const c_char, encoding.as_ptr() as *const c_char, (*t).size, vlen);
            }
            for _ in 0..vlen {
                let name = btf_str(btf_, (*v).name_off);
                if json_output {
                    jsonw_start_object(w);
                    jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, name);
                    if btf_kflag(t) { jsonw_int_field(w, b"val\0".as_ptr() as *const c_char, (*v).val as i64); }
                    else { jsonw_uint_field(w, b"val\0".as_ptr() as *const c_char, (*v).val as __u32 as __u64); }
                    jsonw_end_object(w);
                } else if btf_kflag(t) {
                    printf(b"\n\t'%s' val=%d\0".as_ptr() as *const c_char, name, (*v).val);
                } else {
                    printf(b"\n\t'%s' val=%u\0".as_ptr() as *const c_char, name, (*v).val as __u32);
                }
                v = v.add(1);
            }
            if json_output { jsonw_end_array(w); }
        }
        BTF_KIND_ENUM64 => {
            let mut v = btf_enum64_ptr(t);
            let vlen = btf_vlen(t);
            let encoding = if btf_kflag(t) { b"SIGNED\0" } else { b"UNSIGNED\0" };
            if json_output {
                jsonw_string_field(w, b"encoding\0".as_ptr() as *const c_char, encoding.as_ptr() as *const c_char);
                jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64);
                jsonw_uint_field(w, b"vlen\0".as_ptr() as *const c_char, vlen as __u64);
                jsonw_name(w, b"values\0".as_ptr() as *const c_char);
                jsonw_start_array(w);
            } else {
                printf(b" encoding=%s size=%u vlen=%u\0".as_ptr() as *const c_char, encoding.as_ptr() as *const c_char, (*t).size, vlen);
            }
            for _ in 0..vlen {
                let name = btf_str(btf_, (*v).name_off);
                let val = (((*v).val_hi32 as __u64) << 32) | (*v).val_lo32 as __u64;
                if json_output {
                    jsonw_start_object(w);
                    jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, name);
                    if btf_kflag(t) { jsonw_int_field(w, b"val\0".as_ptr() as *const c_char, val as i64); }
                    else { jsonw_uint_field(w, b"val\0".as_ptr() as *const c_char, val); }
                    jsonw_end_object(w);
                } else if btf_kflag(t) {
                    printf(b"\n\t'%s' val=%lldLL\0".as_ptr() as *const c_char, name, val as i64);
                } else {
                    printf(b"\n\t'%s' val=%lluULL\0".as_ptr() as *const c_char, name, val);
                }
                v = v.add(1);
            }
            if json_output { jsonw_end_array(w); }
        }
        BTF_KIND_FWD => {
            let fwd_kind = if btf_kflag(t) { b"union\0" } else { b"struct\0" };
            if json_output { jsonw_string_field(w, b"fwd_kind\0".as_ptr() as *const c_char, fwd_kind.as_ptr() as *const c_char); }
            else { printf(b" fwd_kind=%s\0".as_ptr() as *const c_char, fwd_kind.as_ptr() as *const c_char); }
        }
        BTF_KIND_FUNC => {
            let linkage = btf_func_linkage_str(t);
            if json_output {
                jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*t).type_ as __u64);
                jsonw_string_field(w, b"linkage\0".as_ptr() as *const c_char, linkage);
            } else {
                printf(b" type_id=%u linkage=%s\0".as_ptr() as *const c_char, (*t).type_, linkage);
            }
        }
        BTF_KIND_FUNC_PROTO => {
            let mut p = t.add(1) as *const btf_param;
            let vlen = btf_vlen(t);
            if json_output {
                jsonw_uint_field(w, b"ret_type_id\0".as_ptr() as *const c_char, (*t).type_ as __u64);
                jsonw_uint_field(w, b"vlen\0".as_ptr() as *const c_char, vlen as __u64);
                jsonw_name(w, b"params\0".as_ptr() as *const c_char);
                jsonw_start_array(w);
            } else {
                printf(b" ret_type_id=%u vlen=%u\0".as_ptr() as *const c_char, (*t).type_, vlen);
            }
            for _ in 0..vlen {
                let name = btf_str(btf_, (*p).name_off);
                if json_output {
                    jsonw_start_object(w);
                    jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, name);
                    jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*p).type_ as __u64);
                    jsonw_end_object(w);
                } else {
                    printf(b"\n\t'%s' type_id=%u\0".as_ptr() as *const c_char, name, (*p).type_);
                }
                p = p.add(1);
            }
            if json_output { jsonw_end_array(w); }
        }
        BTF_KIND_VAR => {
            let v = t.add(1) as *const btf_var;
            let linkage = btf_var_linkage_str((*v).linkage);
            if json_output {
                jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*t).type_ as __u64);
                jsonw_string_field(w, b"linkage\0".as_ptr() as *const c_char, linkage);
            } else {
                printf(b" type_id=%u, linkage=%s\0".as_ptr() as *const c_char, (*t).type_, linkage);
            }
        }
        BTF_KIND_DATASEC => {
            let mut v = t.add(1) as *const btf_var_secinfo;
            let vlen = btf_vlen(t);
            if json_output {
                jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64);
                jsonw_uint_field(w, b"vlen\0".as_ptr() as *const c_char, vlen as __u64);
                jsonw_name(w, b"vars\0".as_ptr() as *const c_char);
                jsonw_start_array(w);
            } else {
                printf(b" size=%u vlen=%u\0".as_ptr() as *const c_char, (*t).size, vlen);
            }
            for _ in 0..vlen {
                if json_output {
                    jsonw_start_object(w);
                    jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*v).type_ as __u64);
                    jsonw_uint_field(w, b"offset\0".as_ptr() as *const c_char, (*v).offset as __u64);
                    jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*v).size as __u64);
                    jsonw_end_object(w);
                } else {
                    printf(b"\n\ttype_id=%u offset=%u size=%u\0".as_ptr() as *const c_char, (*v).type_, (*v).offset, (*v).size);
                    if (*v).type_ < btf__type_cnt(btf_) as __u32 {
                        let vt = btf__type_by_id(btf_, (*v).type_);
                        printf(b" (%s '%s')\0".as_ptr() as *const c_char, BTF_KIND_STR[btf_kind_safe(btf_kind(vt)) as usize], btf_str(btf_, (*vt).name_off));
                    }
                }
                v = v.add(1);
            }
            if json_output { jsonw_end_array(w); }
        }
        BTF_KIND_FLOAT => {
            if json_output { jsonw_uint_field(w, b"size\0".as_ptr() as *const c_char, (*t).size as __u64); }
            else { printf(b" size=%u\0".as_ptr() as *const c_char, (*t).size); }
        }
        BTF_KIND_DECL_TAG => {
            let tag = t.add(1) as *const btf_decl_tag;
            if json_output {
                jsonw_uint_field(w, b"type_id\0".as_ptr() as *const c_char, (*t).type_ as __u64);
                jsonw_int_field(w, b"component_idx\0".as_ptr() as *const c_char, (*tag).component_idx as i64);
            } else {
                printf(b" type_id=%u component_idx=%d\0".as_ptr() as *const c_char, (*t).type_, (*tag).component_idx);
            }
        }
        _ => {}
    }

    if json_output { jsonw_end_object(json_wtr); } else { printf(b"\n\0".as_ptr() as *const c_char); }
    0
}

unsafe fn dump_btf_raw(btf_: *const btf, root_type_ids: *mut __u32, root_type_cnt: c_int) -> c_int {
    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_name(json_wtr, b"types\0".as_ptr() as *const c_char);
        jsonw_start_array(json_wtr);
    }
    if root_type_cnt != 0 {
        for i in 0..root_type_cnt {
            let id = *root_type_ids.add(i as usize);
            dump_btf_type(btf_, id, btf__type_by_id(btf_, id));
        }
    } else {
        let base = btf__base_btf(btf_);
        let cnt = btf__type_cnt(btf_);
        let mut start_id = 1;
        if !base.is_null() { start_id = btf__type_cnt(base); }
        for i in start_id..cnt {
            dump_btf_type(btf_, i as __u32, btf__type_by_id(btf_, i as __u32));
        }
    }
    if json_output {
        jsonw_end_array(json_wtr);
        jsonw_end_object(json_wtr);
    }
    0
}

unsafe fn ptr_array_push(ptr_: *const c_void, arr: *mut ptr_array) -> c_int {
    if (*arr).cnt == (*arr).cap {
        let new_cap = (if (*arr).cap != 0 { (*arr).cap } else { 16 }).wrapping_mul(2);
        let tmp = realloc((*arr).elems as *mut c_void, mem::size_of::<*const c_void>() * new_cap as usize);
        if tmp.is_null() { return -ENOMEM; }
        (*arr).elems = tmp as *mut *const c_void;
        (*arr).cap = new_cap;
    }
    *(*arr).elems.add((*arr).cnt as usize) = ptr_;
    (*arr).cnt += 1;
    0
}

unsafe fn ptr_array_free(arr: *mut ptr_array) {
    free((*arr).elems as *mut c_void);
}

unsafe extern "C" fn cmp_kfuncs(pa: *const c_void, pb: *const c_void, ctx: *mut c_void) -> c_int {
    let btf_ = ctx as *mut btf;
    let a = *(pa as *const *const btf_type);
    let b = *(pb as *const *const btf_type);
    strcmp(btf__str_by_offset(btf_, (*a).name_off), btf__str_by_offset(btf_, (*b).name_off))
}

unsafe fn dump_btf_kfuncs(d: *mut btf_dump, btf_: *const btf) -> c_int {
    let mut opts = btf_dump_emit_type_decl_opts { sz: mem::size_of::<btf_dump_emit_type_decl_opts>(), field_name: ptr::null() };
    let cnt = btf__type_cnt(btf_) as __u32;
    let mut fastcalls = ptr_array { cnt: 0, cap: 0, elems: ptr::null_mut() };
    let mut kfuncs = ptr_array { cnt: 0, cap: 0, elems: ptr::null_mut() };
    let mut err = 0;
    printf(b"\n/* BPF kfuncs */\n\0".as_ptr() as *const c_char);
    printf(b"#ifndef BPF_NO_KFUNC_PROTOTYPES\n\0".as_ptr() as *const c_char);
    for i in 1..cnt {
        let t = btf__type_by_id(btf_, i);
        if !btf_is_decl_tag(t) { continue; }
        if (*btf_decl_tag_ptr(t)).component_idx != -1 { continue; }
        let ft = btf__type_by_id(btf_, (*t).type_);
        if !btf_is_func(ft) { continue; }
        let name = btf__name_by_offset(btf_, (*t).name_off);
        if strncmp(name, KFUNC_DECL_TAG.as_ptr() as *const c_char, KFUNC_DECL_TAG.len()) == 0 {
            err = ptr_array_push(ft as *const c_void, &mut kfuncs);
            if err != 0 { break; }
        }
        if strncmp(name, FASTCALL_DECL_TAG.as_ptr() as *const c_char, FASTCALL_DECL_TAG.len()) == 0 {
            err = ptr_array_push(ft as *const c_void, &mut fastcalls);
            if err != 0 { break; }
        }
    }
    if err == 0 {
        qsort_r(kfuncs.elems as *mut c_void, kfuncs.cnt as size_t, mem::size_of::<*const c_void>(), cmp_kfuncs, btf_ as *mut c_void);
        for i in 0..kfuncs.cnt {
            let t = *kfuncs.elems.add(i as usize) as *const btf_type;
            printf(b"extern \0".as_ptr() as *const c_char);
            for j in 0..fastcalls.cnt {
                if *fastcalls.elems.add(j as usize) == t as *const c_void {
                    printf(b"__bpf_fastcall \0".as_ptr() as *const c_char);
                    break;
                }
            }
            opts.field_name = btf__name_by_offset(btf_, (*t).name_off);
            err = btf_dump__emit_type_decl(d, (*t).type_, &mut opts);
            if err != 0 { break; }
            printf(b" __weak __ksym;\n\0".as_ptr() as *const c_char);
        }
    }
    printf(b"#endif\n\n\0".as_ptr() as *const c_char);
    ptr_array_free(&mut fastcalls);
    ptr_array_free(&mut kfuncs);
    err
}

unsafe extern "C" fn btf_dump_printf(_ctx: *mut c_void, fmt: *const c_char, args: va_list) {
    vfprintf(stdout, fmt, args);
}

unsafe fn btf_type_rank(btf_: *const btf, index: __u32, mut has_name: bool) -> c_int {
    let t = btf__type_by_id(btf_, index);
    let kind = btf_kind(t);
    let max_rank = 10;
    if (*t).name_off != 0 { has_name = true; }
    match kind {
        BTF_KIND_ENUM | BTF_KIND_ENUM64 => if has_name { 1 } else { 0 },
        BTF_KIND_INT | BTF_KIND_FLOAT => 2,
        BTF_KIND_STRUCT | BTF_KIND_UNION => if has_name { 3 } else { max_rank },
        BTF_KIND_FUNC_PROTO => if has_name { 4 } else { max_rank },
        BTF_KIND_ARRAY => if has_name { btf_type_rank(btf_, (*btf_array_ptr(t)).type_, has_name) } else { max_rank },
        BTF_KIND_TYPE_TAG | BTF_KIND_CONST | BTF_KIND_PTR | BTF_KIND_VOLATILE |
        BTF_KIND_RESTRICT | BTF_KIND_TYPEDEF | BTF_KIND_DECL_TAG =>
            if has_name { btf_type_rank(btf_, (*t).type_, has_name) } else { max_rank },
        _ => max_rank,
    }
}

unsafe fn btf_type_sort_name(btf_: *const btf, index: __u32, from_ref: bool) -> *const c_char {
    let t = btf__type_by_id(btf_, index);
    match btf_kind(t) {
        BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
            let mut name_off = (*t).name_off;
            if !from_ref && name_off == 0 && btf_vlen(t) != 0 {
                name_off = if btf_kind(t) == BTF_KIND_ENUM64 { (*btf_enum64_ptr(t)).name_off } else { (*btf_enum_ptr(t)).name_off };
            }
            btf__name_by_offset(btf_, name_off)
        }
        BTF_KIND_ARRAY => btf_type_sort_name(btf_, (*btf_array_ptr(t)).type_, true),
        BTF_KIND_TYPE_TAG | BTF_KIND_CONST | BTF_KIND_PTR | BTF_KIND_VOLATILE |
        BTF_KIND_RESTRICT | BTF_KIND_TYPEDEF | BTF_KIND_DECL_TAG =>
            btf_type_sort_name(btf_, (*t).type_, true),
        _ => btf__name_by_offset(btf_, (*t).name_off),
    }
}

fn hasher(hash: __u64, val: __u64) -> __u64 { hash.wrapping_mul(31).wrapping_add(val) }

unsafe fn btf_name_hasher(hash: __u64, btf_: *const btf, name_off: __u32) -> __u64 {
    if name_off == 0 { return hash; }
    hasher(hash, str_hash(btf__name_by_offset(btf_, name_off)))
}

unsafe fn btf_type_disambig_hash(btf_: *const btf, id: __u32, include_members: bool) -> __u64 {
    let t = btf__type_by_id(btf_, id);
    let mut hash: __u64 = 0;
    hash = btf_name_hasher(hash, btf_, (*t).name_off);
    match btf_kind(t) {
        BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
            for i in 0..btf_vlen(t) {
                let name_off = if btf_is_enum(t) { (*btf_enum_ptr(t).add(i as usize)).name_off } else { (*btf_enum64_ptr(t).add(i as usize)).name_off };
                hash = btf_name_hasher(hash, btf_, name_off);
            }
        }
        BTF_KIND_STRUCT | BTF_KIND_UNION => {
            if include_members {
                for i in 0..btf_vlen(t) {
                    let m = btf_members(t).add(i as usize);
                    hash = btf_name_hasher(hash, btf_, (*m).name_off);
                    /* resolve field type's name and hash it as well */
                    hash = hasher(hash, btf_type_disambig_hash(btf_, (*m).type_, false));
                }
            }
        }
        BTF_KIND_TYPE_TAG | BTF_KIND_CONST | BTF_KIND_PTR | BTF_KIND_VOLATILE |
        BTF_KIND_RESTRICT | BTF_KIND_TYPEDEF | BTF_KIND_DECL_TAG => {
            hash = hasher(hash, btf_type_disambig_hash(btf_, (*t).type_, include_members));
        }
        BTF_KIND_ARRAY => {
            let arr = btf_array_ptr(t);
            hash = hasher(hash, (*arr).nelems as __u64);
            hash = hasher(hash, btf_type_disambig_hash(btf_, (*arr).type_, include_members));
        }
        _ => {}
    }
    hash
}

unsafe extern "C" fn btf_type_compare(left: *const c_void, right: *const c_void) -> c_int {
    let d1 = left as *const sort_datum;
    let d2 = right as *const sort_datum;
    let mut r = (*d1).type_rank - (*d2).type_rank;
    if r == 0 { r = strcmp((*d1).sort_name, (*d2).sort_name); }
    if r == 0 { r = strcmp((*d1).own_name, (*d2).own_name); }
    if r != 0 { return r; }
    if (*d1).disambig_hash != (*d2).disambig_hash {
        return if (*d1).disambig_hash < (*d2).disambig_hash { -1 } else { 1 };
    }
    (*d1).index - (*d2).index
}

unsafe fn sort_btf_c(btf_: *const btf) -> *mut sort_datum {
    let n = btf__type_cnt(btf_);
    let datums = malloc(mem::size_of::<sort_datum>() * n as usize) as *mut sort_datum;
    if datums.is_null() { return ptr::null_mut(); }
    for i in 0..n {
        let d = datums.add(i as usize);
        let t = btf__type_by_id(btf_, i as __u32);
        (*d).index = i;
        (*d).type_rank = btf_type_rank(btf_, i as __u32, false);
        (*d).sort_name = btf_type_sort_name(btf_, i as __u32, false);
        (*d).own_name = btf__name_by_offset(btf_, (*t).name_off);
        (*d).disambig_hash = btf_type_disambig_hash(btf_, i as __u32, true);
    }
    qsort(datums as *mut c_void, n as size_t, mem::size_of::<sort_datum>(), btf_type_compare);
    datums
}

unsafe fn dump_btf_c(btf_: *const btf, root_type_ids: *mut __u32, root_type_cnt: c_int, sort_dump: bool) -> c_int {
    let mut datums: *mut sort_datum = ptr::null_mut();
    let d = btf_dump__new(btf_, btf_dump_printf, ptr::null_mut(), ptr::null());
    if d.is_null() { return -errno; }
    let mut err = 0;
    printf(b"#ifndef __VMLINUX_H__\n#define __VMLINUX_H__\n\n\0".as_ptr() as *const c_char);
    printf(b"#ifndef BPF_NO_PRESERVE_ACCESS_INDEX\n#pragma clang attribute push (__attribute__((preserve_access_index)), apply_to = record)\n#endif\n\n\0".as_ptr() as *const c_char);
    printf(b"#ifndef __ksym\n#define __ksym __attribute__((section(\".ksyms\")))\n#endif\n\n\0".as_ptr() as *const c_char);
    printf(b"#ifndef __weak\n#define __weak __attribute__((weak))\n#endif\n\n\0".as_ptr() as *const c_char);
    printf(b"#ifndef __bpf_fastcall\n#if __has_attribute(bpf_fastcall)\n#define __bpf_fastcall __attribute__((bpf_fastcall))\n#else\n#define __bpf_fastcall\n#endif\n#endif\n\n\0".as_ptr() as *const c_char);
    if root_type_cnt != 0 {
        for i in 0..root_type_cnt {
            err = btf_dump__dump_type(d, *root_type_ids.add(i as usize));
            if err != 0 { break; }
        }
    } else {
        let cnt = btf__type_cnt(btf_);
        if sort_dump { datums = sort_btf_c(btf_); }
        for i in 1..cnt {
            let idx = if !datums.is_null() { (*datums.add(i as usize)).index } else { i };
            err = btf_dump__dump_type(d, idx as __u32);
            if err != 0 { break; }
        }
        if err == 0 { err = dump_btf_kfuncs(d, btf_); }
    }
    if err == 0 {
        printf(b"#ifndef BPF_NO_PRESERVE_ACCESS_INDEX\n#pragma clang attribute pop\n#endif\n\n#endif /* __VMLINUX_H__ */\n\0".as_ptr() as *const c_char);
    }
    free(datums as *mut c_void);
    btf_dump__free(d);
    err
}

unsafe fn get_vmlinux_btf_from_sysfs() -> *mut btf {
    let base = btf__parse(SYSFS_VMLINUX.as_ptr() as *const c_char, ptr::null());
    if base.is_null() {
        p_err(b"failed to parse vmlinux BTF at '%s': %d\n\0".as_ptr() as *const c_char, SYSFS_VMLINUX.as_ptr() as *const c_char, -errno);
    }
    base
}

unsafe fn btf_is_kernel_module(btf_id: __u32) -> bool {
    let mut btf_info: bpf_btf_info = mem::zeroed();
    let mut btf_name = [0 as c_char; BTF_NAME_BUFF_LEN];
    let btf_fd = bpf_btf_get_fd_by_id(btf_id);
    if btf_fd < 0 {
        p_err(b"can't get BTF object by id (%u): %s\0".as_ptr() as *const c_char, btf_id, strerror(errno));
        return false;
    }
    let mut len = mem::size_of::<bpf_btf_info>() as __u32;
    btf_info.name = ptr_to_u64(btf_name.as_mut_ptr());
    btf_info.name_len = btf_name.len() as __u32;
    let err = bpf_btf_get_info_by_fd(btf_fd, &mut btf_info, &mut len);
    close(btf_fd);
    if err != 0 {
        p_err(b"can't get BTF (ID %u) object info: %s\0".as_ptr() as *const c_char, btf_id, strerror(errno));
        return false;
    }
    btf_info.kernel_btf != 0 && strncmp(btf_name.as_ptr(), b"vmlinux\0".as_ptr() as *const c_char, btf_name.len()) != 0
}

unsafe fn merge_btf_files(files: *const *const c_char, nr_files: c_int, vmlinux_base: *mut btf) -> *mut btf {
    let combined = btf__new_empty_split(vmlinux_base);
    if combined.is_null() {
        p_err(b"failed to create combined BTF: %s\0".as_ptr() as *const c_char, strerror(errno));
        return ptr::null_mut();
    }
    for j in 0..nr_files {
        let file = *files.add(j as usize);
        let mod_ = btf__parse_split(file, vmlinux_base);
        if mod_.is_null() {
            p_err(b"failed to load BTF from %s: %s\0".as_ptr() as *const c_char, file, strerror(errno));
            btf__free(combined);
            return ptr::null_mut();
        }
        let ret = btf__add_btf(combined, mod_);
        btf__free(mod_);
        if ret < 0 {
            p_err(b"failed to merge BTF from %s: %s\0".as_ptr() as *const c_char, file, strerror(-ret));
            btf__free(combined);
            return ptr::null_mut();
        }
    }
    let ret = btf__dedup(combined, ptr::null());
    if ret != 0 {
        p_err(b"failed to dedup combined BTF: %s\0".as_ptr() as *const c_char, strerror(-ret));
        btf__free(combined);
        return ptr::null_mut();
    }
    combined
}

unsafe extern "C" fn do_dump(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut dump_c = false;
    let mut sort_dump_c = true;
    let mut btf_: *mut btf = ptr::null_mut();
    let mut base: *mut btf = ptr::null_mut();
    let mut root_type_ids = [0u32; MAX_ROOT_IDS];
    let mut root_type_cnt: c_int = 0;
    let mut btf_id: __u32 = !0;
    let mut fd: c_int = -1;
    let mut err: c_int = 0;
    if !req_args(argc, 2) { usage(); return -1; }
    let src = get_arg(&mut argc, &mut argv);
    if is_prefix(src, b"map\0".as_ptr() as *const c_char) {
        let mut info: bpf_map_info = mem::zeroed();
        let mut len = mem::size_of::<bpf_map_info>() as __u32;
        if !req_args(argc, 2) { usage(); return -1; }
        fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, BPF_F_RDONLY);
        if fd < 0 { return -1; }
        btf_id = info.btf_id;
        if argc != 0 && is_prefix(*argv, b"key\0".as_ptr() as *const c_char) {
            root_type_ids[root_type_cnt as usize] = info.btf_key_type_id; root_type_cnt += 1; next_arg(&mut argc, &mut argv);
        } else if argc != 0 && is_prefix(*argv, b"value\0".as_ptr() as *const c_char) {
            root_type_ids[root_type_cnt as usize] = info.btf_value_type_id; root_type_cnt += 1; next_arg(&mut argc, &mut argv);
        } else if argc != 0 && is_prefix(*argv, b"all\0".as_ptr() as *const c_char) {
            next_arg(&mut argc, &mut argv);
        } else if argc != 0 && is_prefix(*argv, b"kv\0".as_ptr() as *const c_char) {
            root_type_ids[root_type_cnt as usize] = info.btf_key_type_id; root_type_cnt += 1;
            root_type_ids[root_type_cnt as usize] = info.btf_value_type_id; root_type_cnt += 1;
            next_arg(&mut argc, &mut argv);
        } else {
            root_type_ids[root_type_cnt as usize] = info.btf_key_type_id; root_type_cnt += 1;
            root_type_ids[root_type_cnt as usize] = info.btf_value_type_id; root_type_cnt += 1;
        }
    } else if is_prefix(src, b"prog\0".as_ptr() as *const c_char) {
        let mut info: bpf_prog_info = mem::zeroed();
        let mut len = mem::size_of::<bpf_prog_info>() as __u32;
        if !req_args(argc, 2) { usage(); return -1; }
        fd = prog_parse_fd(&mut argc, &mut argv);
        if fd < 0 { return -1; }
        err = bpf_prog_get_info_by_fd(fd, &mut info as *mut _ as *mut c_void, &mut len);
        if err != 0 { p_err(b"can't get prog info: %s\0".as_ptr() as *const c_char, strerror(errno)); goto_done_dump(fd, btf_, base); return err; }
        btf_id = info.btf_id;
    } else if is_prefix(src, b"id\0".as_ptr() as *const c_char) {
        let mut endptr: *mut c_char = ptr::null_mut();
        btf_id = strtoul(*argv, &mut endptr, 0) as __u32;
        if *endptr != 0 { p_err(b"can't parse %s as ID\0".as_ptr() as *const c_char, *argv); return -1; }
        next_arg(&mut argc, &mut argv);
    } else if is_prefix(src, b"file\0".as_ptr() as *const c_char) {
        let mut vmlinux_base = base_btf;
        let mut files: [*const c_char; MAX_BTF_FILES] = [ptr::null(); MAX_BTF_FILES];
        let mut nr_files: c_int = 0;
        if strcmp(*argv, SYSFS_VMLINUX.as_ptr() as *const c_char) != 0 {
            files[nr_files as usize] = *argv; nr_files += 1;
        } else { p_info(b"skipping %s (will be loaded as base)\0".as_ptr() as *const c_char, *argv); }
        next_arg(&mut argc, &mut argv);
        while argc != 0 && is_prefix(*argv, b"file\0".as_ptr() as *const c_char) {
            next_arg(&mut argc, &mut argv);
            if !req_args(argc, 1) { err = -EINVAL; break; }
            if strcmp(*argv, SYSFS_VMLINUX.as_ptr() as *const c_char) == 0 {
                p_info(b"skipping %s (will be loaded as base)\0".as_ptr() as *const c_char, *argv);
                next_arg(&mut argc, &mut argv);
                continue;
            }
            if nr_files >= MAX_BTF_FILES as c_int {
                p_err(b"too many BTF files (max %d)\0".as_ptr() as *const c_char, MAX_BTF_FILES as c_int);
                err = -E2BIG; break;
            }
            files[nr_files as usize] = *argv; nr_files += 1; next_arg(&mut argc, &mut argv);
        }
        if err != 0 { goto_done_dump(fd, btf_, base); return err; }
        if vmlinux_base.is_null() {
            for j in 0..nr_files {
                if strncmp(files[j as usize], SYSFS_PREFIX.as_ptr() as *const c_char, SYSFS_PREFIX.len() - 1) == 0 {
                    base = get_vmlinux_btf_from_sysfs(); vmlinux_base = base; break;
                }
            }
        }
        if nr_files == 0 { nr_files = 1; files[0] = SYSFS_VMLINUX.as_ptr() as *const c_char; }
        if nr_files == 1 {
            btf_ = btf__parse_split(files[0], if !base.is_null() { base } else { base_btf });
            if btf_.is_null() {
                err = -errno;
                p_err(b"failed to load BTF from %s: %s\0".as_ptr() as *const c_char, files[0], strerror(errno));
                goto_done_dump(fd, btf_, base); return err;
            }
        } else {
            if vmlinux_base.is_null() {
                p_err(b"base BTF is required when merging multiple BTF files; use -B/--base-btf or use sysfs paths\0".as_ptr() as *const c_char);
                err = -EINVAL; goto_done_dump(fd, btf_, base); return err;
            }
            btf_ = merge_btf_files(files.as_ptr(), nr_files, vmlinux_base);
            if btf_.is_null() { err = -errno; goto_done_dump(fd, btf_, base); return err; }
        }
    } else {
        err = -1;
        p_err(b"unrecognized BTF source specifier: '%s'\0".as_ptr() as *const c_char, src);
        goto_done_dump(fd, btf_, base); return err;
    }

    let have_id_filtering = root_type_cnt != 0;
    while argc != 0 {
        if is_prefix(*argv, b"format\0".as_ptr() as *const c_char) {
            next_arg(&mut argc, &mut argv);
            if argc < 1 {
                p_err(b"expecting value for 'format' option\n\0".as_ptr() as *const c_char);
                err = -EINVAL; break;
            }
            if strcmp(*argv, b"c\0".as_ptr() as *const c_char) == 0 { dump_c = true; }
            else if strcmp(*argv, b"raw\0".as_ptr() as *const c_char) == 0 { dump_c = false; }
            else {
                p_err(b"unrecognized format specifier: '%s', possible values: raw, c\0".as_ptr() as *const c_char, *argv);
                err = -EINVAL; break;
            }
            next_arg(&mut argc, &mut argv);
        } else if is_prefix(*argv, b"root_id\0".as_ptr() as *const c_char) {
            let mut end: *mut c_char = ptr::null_mut();
            if have_id_filtering {
                p_err(b"cannot use root_id with other type filtering\0".as_ptr() as *const c_char);
                err = -EINVAL; break;
            } else if root_type_cnt == MAX_ROOT_IDS as c_int {
                p_err(b"only %d root_id are supported\0".as_ptr() as *const c_char, MAX_ROOT_IDS as c_int);
                err = -E2BIG; break;
            }
            next_arg(&mut argc, &mut argv);
            let root_id = strtoul(*argv, &mut end, 0) as __u32;
            if *end != 0 { err = -1; p_err(b"can't parse %s as root ID\0".as_ptr() as *const c_char, *argv); break; }
            for i in 0..root_type_cnt {
                if root_type_ids[i as usize] == root_id {
                    err = -EINVAL; p_err(b"duplicate root_id %u supplied\0".as_ptr() as *const c_char, root_id); break;
                }
            }
            if err != 0 { break; }
            root_type_ids[root_type_cnt as usize] = root_id; root_type_cnt += 1;
            next_arg(&mut argc, &mut argv);
        } else if is_prefix(*argv, b"unsorted\0".as_ptr() as *const c_char) {
            sort_dump_c = false; next_arg(&mut argc, &mut argv);
        } else {
            p_err(b"unrecognized option: '%s'\0".as_ptr() as *const c_char, *argv);
            err = -EINVAL; break;
        }
    }
    if err != 0 { goto_done_dump(fd, btf_, base); return err; }
    if btf_.is_null() {
        if base_btf.is_null() && btf_is_kernel_module(btf_id) {
            p_info(b"Warning: valid base BTF was not specified with -B option, falling back to standard base BTF (%s)\0".as_ptr() as *const c_char, SYSFS_VMLINUX.as_ptr() as *const c_char);
            base_btf = get_vmlinux_btf_from_sysfs();
        }
        btf_ = btf__load_from_kernel_by_id_split(btf_id, base_btf);
        if btf_.is_null() {
            err = -errno;
            p_err(b"get btf by id (%u): %s\0".as_ptr() as *const c_char, btf_id, strerror(errno));
            goto_done_dump(fd, btf_, base); return err;
        }
    }
    /* Invalid root IDs causes half emitted boilerplate and then unclean
     * exit. It's an ugly user experience, so handle common error here.
     */
    for i in 0..root_type_cnt {
        if root_type_ids[i as usize] >= btf__type_cnt(btf_) as __u32 {
            err = -EINVAL;
            p_err(b"invalid root ID: %u\0".as_ptr() as *const c_char, root_type_ids[i as usize]);
            goto_done_dump(fd, btf_, base); return err;
        }
    }
    if dump_c {
        if json_output {
            p_err(b"JSON output for C-syntax dump is not supported\0".as_ptr() as *const c_char);
            err = -ENOTSUP;
        } else {
            err = dump_btf_c(btf_, root_type_ids.as_mut_ptr(), root_type_cnt, sort_dump_c);
        }
    } else {
        err = dump_btf_raw(btf_, root_type_ids.as_mut_ptr(), root_type_cnt);
    }
    goto_done_dump(fd, btf_, base);
    err
}

unsafe fn goto_done_dump(fd: c_int, btf_: *mut btf, base: *mut btf) {
    close(fd);
    btf__free(btf_);
    btf__free(base);
}

unsafe fn btf_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();
    if !is_prefix(**argv, b"id\0".as_ptr() as *const c_char) {
        p_err(b"expected 'id', got: '%s'?\0".as_ptr() as *const c_char, **argv);
        return -1;
    }
    *argc -= 1; *argv = (*argv).add(1);
    let id = strtoul(**argv, &mut endptr, 0) as c_uint;
    if *endptr != 0 {
        p_err(b"can't parse %s as ID\0".as_ptr() as *const c_char, **argv);
        return -1;
    }
    *argc -= 1; *argv = (*argv).add(1);
    let fd = bpf_btf_get_fd_by_id(id);
    if fd < 0 {
        p_err(b"can't get BTF object by id (%u): %s\0".as_ptr() as *const c_char, id, strerror(errno));
    }
    fd
}

unsafe fn build_btf_type_table(tab: *mut hashmap, typ: bpf_obj_type, info: *mut c_void, len: *mut __u32) -> c_int {
    let names = [b"unknown\0".as_ptr() as *const c_char, b"prog\0".as_ptr() as *const c_char, b"map\0".as_ptr() as *const c_char];
    let mut opts_ro = bpf_get_fd_by_id_opts { sz: mem::size_of::<bpf_get_fd_by_id_opts>(), open_flags: BPF_F_RDONLY };
    let mut id: __u32 = 0;
    loop {
        let mut err = match typ {
            BPF_OBJ_PROG => bpf_prog_get_next_id(id, &mut id),
            BPF_OBJ_MAP => bpf_map_get_next_id(id, &mut id),
            _ => { p_err(b"unexpected object type: %u\0".as_ptr() as *const c_char, typ); hashmap__free(tab); return -1; }
        };
        if err != 0 {
            if errno == ENOENT { return 0; }
            p_err(b"can't get next %s: %s%s\0".as_ptr() as *const c_char, names[typ as usize], strerror(errno), if errno == EINVAL { b" -- kernel too old?\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char);
            hashmap__free(tab); return err;
        }
        let fd = match typ {
            BPF_OBJ_PROG => bpf_prog_get_fd_by_id(id),
            BPF_OBJ_MAP => bpf_map_get_fd_by_id_opts(id, &opts_ro),
            _ => { p_err(b"unexpected object type: %u\0".as_ptr() as *const c_char, typ); hashmap__free(tab); return -1; }
        };
        if fd < 0 {
            if errno == ENOENT { continue; }
            p_err(b"can't get %s by id (%u): %s\0".as_ptr() as *const c_char, names[typ as usize], id, strerror(errno));
            hashmap__free(tab); return -1;
        }
        memset(info, 0, *len as size_t);
        err = if typ == BPF_OBJ_PROG { bpf_prog_get_info_by_fd(fd, info, len) } else { bpf_map_get_info_by_fd(fd, info, len) };
        close(fd);
        if err != 0 {
            p_err(b"can't get %s info: %s\0".as_ptr() as *const c_char, names[typ as usize], strerror(errno));
            hashmap__free(tab); return err;
        }
        let btf_id = match typ {
            BPF_OBJ_PROG => (*(info as *mut bpf_prog_info)).btf_id,
            BPF_OBJ_MAP => (*(info as *mut bpf_map_info)).btf_id,
            _ => { p_err(b"unexpected object type: %u\0".as_ptr() as *const c_char, typ); hashmap__free(tab); return -1; }
        };
        if btf_id == 0 { continue; }
        err = hashmap__append(tab, btf_id, id);
        if err != 0 {
            p_err(b"failed to append entry to hashmap for BTF ID %u, object ID %u: %s\0".as_ptr() as *const c_char, btf_id, id, strerror(-err));
            hashmap__free(tab); return err;
        }
    }
}

unsafe fn build_btf_tables(btf_prog_table: *mut hashmap, btf_map_table: *mut hashmap) -> c_int {
    let mut prog_info: bpf_prog_info = mem::zeroed();
    let mut prog_len = mem::size_of::<bpf_prog_info>() as __u32;
    let mut map_info: bpf_map_info = mem::zeroed();
    let mut map_len = mem::size_of::<bpf_map_info>() as __u32;
    let mut err = build_btf_type_table(btf_prog_table, BPF_OBJ_PROG, &mut prog_info as *mut _ as *mut c_void, &mut prog_len);
    if err != 0 { return err; }
    err = build_btf_type_table(btf_map_table, BPF_OBJ_MAP, &mut map_info as *mut _ as *mut c_void, &mut map_len);
    if err != 0 { hashmap__free(btf_prog_table); return err; }
    0
}

unsafe fn show_btf_plain(info: *mut bpf_btf_info, _fd: c_int, btf_prog_table: *mut hashmap, btf_map_table: *mut hashmap) {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let name = u64_to_ptr::<c_char>((*info).name);
    printf(b"%u: \0".as_ptr() as *const c_char, (*info).id);
    if (*info).kernel_btf != 0 { printf(b"name [%s]  \0".as_ptr() as *const c_char, name); }
    else if !name.is_null() && *name != 0 { printf(b"name %s  \0".as_ptr() as *const c_char, name); }
    else { printf(b"name <anon>  \0".as_ptr() as *const c_char); }
    printf(b"size %uB\0".as_ptr() as *const c_char, (*info).btf_size);
    let mut n = 0;
    while hashmap__for_each_key_entry(btf_prog_table, &mut entry, (*info).id) {
        printf(b"%s%lu\0".as_ptr() as *const c_char, if n == 0 { b"  prog_ids \0".as_ptr() } else { b",\0".as_ptr() } as *const c_char, (*entry).value);
        n += 1;
    }
    n = 0;
    while hashmap__for_each_key_entry(btf_map_table, &mut entry, (*info).id) {
        printf(b"%s%lu\0".as_ptr() as *const c_char, if n == 0 { b"  map_ids \0".as_ptr() } else { b",\0".as_ptr() } as *const c_char, (*entry).value);
        n += 1;
    }
    emit_obj_refs_plain(refs_table, (*info).id, b"\n\tpids \0".as_ptr() as *const c_char);
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn show_btf_json(info: *mut bpf_btf_info, _fd: c_int, btf_prog_table: *mut hashmap, btf_map_table: *mut hashmap) {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let name = u64_to_ptr::<c_char>((*info).name);
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, b"id\0".as_ptr() as *const c_char, (*info).id as __u64);
    jsonw_uint_field(json_wtr, b"size\0".as_ptr() as *const c_char, (*info).btf_size as __u64);
    jsonw_name(json_wtr, b"prog_ids\0".as_ptr() as *const c_char);
    jsonw_start_array(json_wtr);
    while hashmap__for_each_key_entry(btf_prog_table, &mut entry, (*info).id) { jsonw_uint(json_wtr, (*entry).value as __u64); }
    jsonw_end_array(json_wtr);
    jsonw_name(json_wtr, b"map_ids\0".as_ptr() as *const c_char);
    jsonw_start_array(json_wtr);
    while hashmap__for_each_key_entry(btf_map_table, &mut entry, (*info).id) { jsonw_uint(json_wtr, (*entry).value as __u64); }
    jsonw_end_array(json_wtr);
    emit_obj_refs_json(refs_table, (*info).id, json_wtr);
    jsonw_bool_field(json_wtr, b"kernel\0".as_ptr() as *const c_char, (*info).kernel_btf != 0);
    if !name.is_null() && *name != 0 { jsonw_string_field(json_wtr, b"name\0".as_ptr() as *const c_char, name); }
    jsonw_end_object(json_wtr);
}

unsafe fn show_btf(fd: c_int, btf_prog_table: *mut hashmap, btf_map_table: *mut hashmap) -> c_int {
    let mut info: bpf_btf_info = mem::zeroed();
    let mut len = mem::size_of::<bpf_btf_info>() as __u32;
    let mut name = [0 as c_char; 64];
    let mut err = bpf_btf_get_info_by_fd(fd, &mut info, &mut len);
    if err != 0 {
        p_err(b"can't get BTF object info: %s\0".as_ptr() as *const c_char, strerror(errno));
        return -1;
    }
    /* if kernel support emitting BTF object name, pass name pointer */
    if info.name_len != 0 {
        info = mem::zeroed();
        info.name_len = name.len() as __u32;
        info.name = ptr_to_u64(name.as_mut_ptr());
        len = mem::size_of::<bpf_btf_info>() as __u32;
        err = bpf_btf_get_info_by_fd(fd, &mut info, &mut len);
        if err != 0 {
            p_err(b"can't get BTF object info: %s\0".as_ptr() as *const c_char, strerror(errno));
            return -1;
        }
    }
    if json_output { show_btf_json(&mut info, fd, btf_prog_table, btf_map_table); }
    else { show_btf_plain(&mut info, fd, btf_prog_table, btf_map_table); }
    0
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut fd: c_int = -1;
    let mut id: __u32 = 0;
    if argc == 2 {
        fd = btf_parse_fd(&mut argc, &mut argv);
        if fd < 0 { return -1; }
    }
    if argc != 0 {
        if fd >= 0 { close(fd); }
        return bad_arg();
    }
    let btf_prog_table = hashmap__new(hash_fn_for_key_as_id as *const c_void, equal_fn_for_key_as_id as *const c_void, ptr::null_mut());
    let btf_map_table = hashmap__new(hash_fn_for_key_as_id as *const c_void, equal_fn_for_key_as_id as *const c_void, ptr::null_mut());
    if IS_ERR(btf_prog_table as *const c_void) || IS_ERR(btf_map_table as *const c_void) {
        hashmap__free(btf_prog_table);
        hashmap__free(btf_map_table);
        if fd >= 0 { close(fd); }
        p_err(b"failed to create hashmap for object references\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut err = build_btf_tables(btf_prog_table, btf_map_table);
    if err != 0 {
        if fd >= 0 { close(fd); }
        return err;
    }
    build_obj_refs_table(&mut refs_table, BPF_OBJ_BTF);
    if fd >= 0 {
        err = show_btf(fd, btf_prog_table, btf_map_table);
        close(fd);
        hashmap__free(btf_prog_table);
        hashmap__free(btf_map_table);
        delete_obj_refs_table(refs_table);
        return err;
    }
    if json_output { jsonw_start_array(json_wtr); }
    loop {
        err = bpf_btf_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT { err = 0; break; }
            p_err(b"can't get next BTF object: %s%s\0".as_ptr() as *const c_char, strerror(errno), if errno == EINVAL { b" -- kernel too old?\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char);
            err = -1; break;
        }
        fd = bpf_btf_get_fd_by_id(id);
        if fd < 0 {
            if errno == ENOENT { continue; }
            p_err(b"can't get BTF object by id (%u): %s\0".as_ptr() as *const c_char, id, strerror(errno));
            err = -1; break;
        }
        err = show_btf(fd, btf_prog_table, btf_map_table);
        close(fd);
        if err != 0 { break; }
    }
    if json_output { jsonw_end_array(json_wtr); }
    hashmap__free(btf_prog_table);
    hashmap__free(btf_map_table);
    delete_obj_refs_table(refs_table);
    err
}

unsafe extern "C" fn do_help(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }
    fprintf(stderr,
            b"Usage: %1$s %2$s { show | list } [id BTF_ID]\n       %1$s %2$s dump BTF_SRC [format FORMAT] [root_id ROOT_ID]\n       %1$s %2$s help\n\n       BTF_SRC := { id BTF_ID | prog PROG | map MAP [{key | value | kv | all}] |\n                    file FILE [file FILE]... }\n       FORMAT  := { raw | c [unsorted] }\n       HELP_SPEC_MAP\n       HELP_SPEC_PROGRAM\n       HELP_SPEC_OPTIONS |\n                    {-B|--base-btf} }\n\0".as_ptr() as *const c_char,
            bin_name, b"btf\0".as_ptr() as *const c_char);
    0
}

static CMDS: [cmd; 5] = [
    cmd { cmd: b"show\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"list\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"help\0".as_ptr() as *const c_char, func: Some(do_help) },
    cmd { cmd: b"dump\0".as_ptr() as *const c_char, func: Some(do_dump) },
    cmd { cmd: ptr::null(), func: None },
];

#[no_mangle]
pub unsafe extern "C" fn do_btf(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(CMDS.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
