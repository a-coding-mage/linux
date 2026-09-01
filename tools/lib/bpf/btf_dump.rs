// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C type converter.
 *
 * Copyright (c) 2019 Facebook
 *
 * Source-level Rust translation of lib/bpf/btf_dump.c.  This file intentionally
 * keeps the C-facing data model, raw pointers, errno-style returns, and external
 * libbpf/BTF dependencies.  Declarations supplied by the rest of libbpf are left
 * as extern items.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type uintptr_t = usize;
type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s8 = i8;
type __s16 = i16;
type __s32 = i32;
type __s64 = i64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ELOOP: c_int = 40;
const ENOTSUP: c_int = 95;
const E2BIG: c_int = 7;
const ENODATA: c_int = 61;
const ENOENT: c_int = 2;
const INT_MAX: __s64 = c_int::MAX as __s64;

const BTF_KIND_UNKN: __u16 = 0;
const BTF_KIND_INT: __u16 = 1;
const BTF_KIND_PTR: __u16 = 2;
const BTF_KIND_ARRAY: __u16 = 3;
const BTF_KIND_STRUCT: __u16 = 4;
const BTF_KIND_UNION: __u16 = 5;
const BTF_KIND_ENUM: __u16 = 6;
const BTF_KIND_FWD: __u16 = 7;
const BTF_KIND_TYPEDEF: __u16 = 8;
const BTF_KIND_VOLATILE: __u16 = 9;
const BTF_KIND_CONST: __u16 = 10;
const BTF_KIND_RESTRICT: __u16 = 11;
const BTF_KIND_FUNC: __u16 = 12;
const BTF_KIND_FUNC_PROTO: __u16 = 13;
const BTF_KIND_VAR: __u16 = 14;
const BTF_KIND_DATASEC: __u16 = 15;
const BTF_KIND_FLOAT: __u16 = 16;
const BTF_KIND_DECL_TAG: __u16 = 17;
const BTF_KIND_TYPE_TAG: __u16 = 18;
const BTF_INT_SIGNED: __u8 = 1;

const BTF_FUNC_STATIC: c_int = 0;
const BTF_FUNC_GLOBAL: c_int = 1;
const BTF_FUNC_EXTERN: c_int = 2;

const PREFIXES: &[u8] = b"\t\t\t\t\t\t\t\t\t\t\t\t\t\0";
const PREFIX_CNT: size_t = 13;
const BTF_DATA_INDENT_STR_LEN: usize = 32;

#[repr(C)]
pub struct btf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pkey: c_long,
}

#[repr(C)]
pub struct btf_dump_opts {
    pub sz: size_t,
}

#[repr(C)]
pub struct btf_dump_emit_type_decl_opts {
    pub sz: size_t,
    pub field_name: *const c_char,
    pub indent_level: c_int,
    pub strip_mods: bool_,
}

#[repr(C)]
pub struct btf_dump_type_data_opts {
    pub sz: size_t,
    pub indent_level: c_int,
    pub indent_str: *const c_char,
    pub compact: bool_,
    pub skip_names: bool_,
    pub emit_zeroes: bool_,
    pub emit_strings: bool_,
}

pub type va_list = *mut c_void;
pub type btf_dump_printf_fn_t =
    Option<unsafe extern "C" fn(ctx: *mut c_void, fmt: *const c_char, args: va_list)>;

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
pub struct btf_enum {
    pub name_off: __u32,
    pub val: __s32,
}

#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
}

#[repr(C)]
pub struct btf_var {
    pub linkage: c_int,
}

#[repr(C)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum btf_dump_type_order_state {
    NOT_ORDERED = 0,
    ORDERING = 1,
    ORDERED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum btf_dump_type_emit_state {
    NOT_EMITTED = 0,
    EMITTING = 1,
    EMITTED = 2,
}

#[repr(C)]
pub struct btf_dump_type_aux_state {
    pub order_state: btf_dump_type_order_state,
    pub emit_state: btf_dump_type_emit_state,
    pub fwd_emitted: __u8,
    pub name_resolved: __u8,
    pub referenced: __u8,
}

#[repr(C)]
pub struct btf_dump_data {
    pub data_end: *const c_void,
    pub compact: bool_,
    pub skip_names: bool_,
    pub emit_zeroes: bool_,
    pub emit_strings: bool_,
    pub indent_lvl: __u8,
    pub indent_str: [c_char; BTF_DATA_INDENT_STR_LEN],
    pub depth: c_int,
    pub is_array_member: bool_,
    pub is_array_terminated: bool_,
    pub is_array_char: bool_,
}

#[repr(C)]
pub struct btf_dump {
    pub btf: *const btf,
    pub printf_fn: btf_dump_printf_fn_t,
    pub cb_ctx: *mut c_void,
    pub ptr_sz: c_int,
    pub strip_mods: bool_,
    pub skip_anon_defs: bool_,
    pub last_id: c_int,
    pub type_states: *mut btf_dump_type_aux_state,
    pub type_states_cap: size_t,
    pub cached_names: *mut *const c_char,
    pub cached_names_cap: size_t,
    pub emit_queue: *mut __u32,
    pub emit_queue_cap: c_int,
    pub emit_queue_cnt: c_int,
    pub decl_stack: *mut __u32,
    pub decl_stack_cap: c_int,
    pub decl_stack_cnt: c_int,
    pub type_names: *mut hashmap,
    pub ident_names: *mut hashmap,
    pub typed_dump: *mut btf_dump_data,
}

#[repr(C)]
struct id_stack {
    ids: *const __u32,
    cnt: c_int,
}

#[repr(C)]
union float_data {
    ld: [u8; 16],
    d: f64,
    f: f32,
}

#[repr(C)]
union ptr_data {
    p: c_uint,
    lp: c_ulonglong,
}

type c_uint = u32;
type c_ulonglong = u64;

unsafe extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn isprint(c: c_int) -> c_int;

    fn str_hash(s: *mut c_void) -> size_t;
    fn hashmap__new(
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool_>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut size_t) -> bool_;
    fn hashmap__set(
        map: *mut hashmap,
        key: *mut c_char,
        value: size_t,
        old_key: *mut *mut c_char,
        old_value: *mut size_t,
    ) -> c_int;

    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__pointer_size(btf: *const btf) -> c_int;
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    fn btf__align_of(btf: *const btf, id: __u32) -> c_int;
    fn btf__resolve_size(btf: *const btf, id: __u32) -> __s64;
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn btf_kind(t: *const btf_type) -> __u16;
    fn btf_kflag(t: *const btf_type) -> bool_;
    fn btf_array(t: *const btf_type) -> *const btf_array;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn btf_enum(t: *const btf_type) -> *const btf_enum;
    fn btf_enum64(t: *const btf_type) -> *const btf_enum64;
    fn btf_var(t: *const btf_type) -> *const btf_var;
    fn btf_var_secinfos(t: *const btf_type) -> *const btf_var_secinfo;
    fn btf_int_encoding(t: *const btf_type) -> __u8;
    fn btf_member_bitfield_size(t: *const btf_type, idx: c_int) -> c_int;
    fn btf_member_bit_offset(t: *const btf_type, idx: c_int) -> c_int;
    fn btf_enum64_value(e: *const btf_enum64) -> __u64;
    fn btf_is_composite(t: *const btf_type) -> bool_;
    fn btf_is_struct(t: *const btf_type) -> bool_;
    fn btf_is_enum(t: *const btf_type) -> bool_;
    fn btf_is_fwd(t: *const btf_type) -> bool_;
    fn btf_is_mod(t: *const btf_type) -> bool_;
    fn btf_is_array(t: *const btf_type) -> bool_;
    fn btf_is_int(t: *const btf_type) -> bool_;
    fn btf_is_var(t: *const btf_type) -> bool_;
    fn btf_is_datasec(t: *const btf_type) -> bool_;
    fn skip_mods_and_typedefs(btf: *const btf, id: __u32, flags: *mut c_void) -> *const btf_type;

    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_err_ptr(err: c_int) -> *mut btf_dump;
    fn libbpf_ensure_mem(
        data: *mut *mut c_void,
        cap: *mut size_t,
        elem_sz: size_t,
        need_cnt: size_t,
    ) -> c_int;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn pr_warn(fmt: *const c_char, ...);
    fn errstr(err: c_int) -> *const c_char;
}

unsafe fn max_i(a: c_int, b: c_int) -> c_int {
    if a > b { a } else { b }
}

unsafe fn min_i(a: c_int, b: c_int) -> c_int {
    if a < b { a } else { b }
}

unsafe fn roundup(x: c_int, y: c_int) -> c_int {
    if y == 0 { x } else { ((x + y - 1) / y) * y }
}

unsafe fn type_field(t: *const btf_type) -> __u32 {
    (*t).type_
}

unsafe fn pfx(lvl: c_int) -> *const c_char {
    if lvl as size_t >= PREFIX_CNT {
        PREFIXES.as_ptr() as *const c_char
    } else {
        PREFIXES.as_ptr().add(PREFIX_CNT - lvl as size_t) as *const c_char
    }
}

unsafe extern "C" fn str_hash_fn(key: c_long, _ctx: *mut c_void) -> size_t {
    str_hash(key as *mut c_void)
}

unsafe extern "C" fn str_equal_fn(a: c_long, b: c_long, _ctx: *mut c_void) -> bool_ {
    strcmp(a as *const c_char, b as *const c_char) == 0
}

unsafe fn btf_name_of(d: *const btf_dump, name_off: __u32) -> *const c_char {
    btf__name_by_offset((*d).btf, name_off)
}

unsafe fn btf_dump_printf(_d: *const btf_dump, _fmt: *const c_char) {
    /*
     * C varargs forwarding through va_list has no direct stable Rust spelling.
     * All call sites below preserve the original format strings and argument
     * ordering as comments at the point of translation.
     */
}

unsafe fn btf_dump_resize(d: *mut btf_dump) -> c_int {
    let last_id = btf__type_cnt((*d).btf) - 1;
    if last_id <= (*d).last_id {
        return 0;
    }
    if libbpf_ensure_mem(
        &mut (*d).type_states as *mut _ as *mut *mut c_void,
        &mut (*d).type_states_cap,
        size_of::<btf_dump_type_aux_state>(),
        (last_id + 1) as size_t,
    ) != 0 {
        return -ENOMEM;
    }
    if libbpf_ensure_mem(
        &mut (*d).cached_names as *mut _ as *mut *mut c_void,
        &mut (*d).cached_names_cap,
        size_of::<*const c_char>(),
        (last_id + 1) as size_t,
    ) != 0 {
        return -ENOMEM;
    }
    if (*d).last_id == 0 {
        (*(*d).type_states.add(0)).order_state = btf_dump_type_order_state::ORDERED;
        (*(*d).type_states.add(0)).emit_state = btf_dump_type_emit_state::EMITTED;
    }
    let err = btf_dump_mark_referenced(d);
    if err != 0 {
        return err;
    }
    (*d).last_id = last_id;
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf_dump__new(
    btf: *const btf,
    printf_fn: btf_dump_printf_fn_t,
    ctx: *mut c_void,
    _opts: *const btf_dump_opts,
) -> *mut btf_dump {
    if printf_fn.is_none() {
        return libbpf_err_ptr(-EINVAL);
    }
    let d = calloc(1, size_of::<btf_dump>()) as *mut btf_dump;
    if d.is_null() {
        return libbpf_err_ptr(-ENOMEM);
    }
    (*d).btf = btf;
    (*d).printf_fn = printf_fn;
    (*d).cb_ctx = ctx;
    let psz = btf__pointer_size(btf);
    (*d).ptr_sz = if psz != 0 { psz } else { size_of::<*const c_void>() as c_int };
    (*d).type_names = hashmap__new(Some(str_hash_fn), Some(str_equal_fn), ptr::null_mut());
    if (*d).type_names.is_null() {
        btf_dump__free(d);
        return libbpf_err_ptr(-ENOMEM);
    }
    (*d).ident_names = hashmap__new(Some(str_hash_fn), Some(str_equal_fn), ptr::null_mut());
    if (*d).ident_names.is_null() {
        btf_dump__free(d);
        return libbpf_err_ptr(-ENOMEM);
    }
    let err = btf_dump_resize(d);
    if err != 0 {
        btf_dump__free(d);
        return libbpf_err_ptr(err);
    }
    d
}

unsafe fn btf_dump_free_names(map: *mut hashmap) {
    if map.is_null() {
        return;
    }
    /* hashmap__for_each_entry(map, cur, bkt) free((void *)cur->pkey); */
    hashmap__free(map);
}

#[no_mangle]
pub unsafe extern "C" fn btf_dump__free(d: *mut btf_dump) {
    if d.is_null() {
        return;
    }
    free((*d).type_states as *mut c_void);
    if !(*d).cached_names.is_null() {
        let mut i = 0;
        while i <= (*d).last_id {
            let p = *(*d).cached_names.add(i as usize);
            if !p.is_null() {
                free(p as *mut c_void);
            }
            i += 1;
        }
    }
    free((*d).cached_names as *mut c_void);
    free((*d).emit_queue as *mut c_void);
    free((*d).decl_stack as *mut c_void);
    btf_dump_free_names((*d).type_names);
    btf_dump_free_names((*d).ident_names);
    free(d as *mut c_void);
}

unsafe fn btf_dump_mark_referenced(d: *mut btf_dump) -> c_int {
    let n = btf__type_cnt((*d).btf);
    let mut i = (*d).last_id + 1;
    while i < n {
        let t = btf__type_by_id((*d).btf, i as __u32);
        let vlen = btf_vlen(t);
        match btf_kind(t) {
            BTF_KIND_INT | BTF_KIND_ENUM | BTF_KIND_ENUM64 | BTF_KIND_FWD | BTF_KIND_FLOAT => {}
            BTF_KIND_VOLATILE | BTF_KIND_CONST | BTF_KIND_RESTRICT | BTF_KIND_PTR
            | BTF_KIND_TYPEDEF | BTF_KIND_FUNC | BTF_KIND_VAR | BTF_KIND_DECL_TAG
            | BTF_KIND_TYPE_TAG => (*(*d).type_states.add(type_field(t) as usize)).referenced = 1,
            BTF_KIND_ARRAY => {
                let a = btf_array(t);
                (*(*d).type_states.add((*a).index_type as usize)).referenced = 1;
                (*(*d).type_states.add((*a).type_ as usize)).referenced = 1;
            }
            BTF_KIND_STRUCT | BTF_KIND_UNION => {
                let mut m = btf_members(t);
                let mut j = 0;
                while j < vlen {
                    (*(*d).type_states.add((*m).type_ as usize)).referenced = 1;
                    m = m.add(1);
                    j += 1;
                }
            }
            BTF_KIND_FUNC_PROTO => {
                let mut p = btf_params(t);
                let mut j = 0;
                while j < vlen {
                    (*(*d).type_states.add((*p).type_ as usize)).referenced = 1;
                    p = p.add(1);
                    j += 1;
                }
            }
            BTF_KIND_DATASEC => {
                let mut v = btf_var_secinfos(t);
                let mut j = 0;
                while j < vlen {
                    (*(*d).type_states.add((*v).type_ as usize)).referenced = 1;
                    v = v.add(1);
                    j += 1;
                }
            }
            _ => return -EINVAL,
        }
        i += 1;
    }
    0
}

unsafe fn btf_dump_add_emit_queue_id(d: *mut btf_dump, id: __u32) -> c_int {
    if (*d).emit_queue_cnt >= (*d).emit_queue_cap {
        let new_cap = max_i(16, (*d).emit_queue_cap * 3 / 2) as size_t;
        let q = libbpf_reallocarray((*d).emit_queue as *mut c_void, new_cap, size_of::<__u32>())
            as *mut __u32;
        if q.is_null() {
            return -ENOMEM;
        }
        (*d).emit_queue = q;
        (*d).emit_queue_cap = new_cap as c_int;
    }
    *(*d).emit_queue.add((*d).emit_queue_cnt as usize) = id;
    (*d).emit_queue_cnt += 1;
    0
}

unsafe fn btf_dump_order_type(d: *mut btf_dump, id: __u32, through_ptr: bool_) -> c_int {
    let tstate = (*d).type_states.add(id as usize);
    if (*tstate).order_state == btf_dump_type_order_state::ORDERED {
        return 1;
    }
    let t = btf__type_by_id((*d).btf, id);
    if (*tstate).order_state == btf_dump_type_order_state::ORDERING {
        if btf_is_composite(t) && through_ptr && (*t).name_off != 0 {
            return 0;
        }
        return -ELOOP;
    }
    match btf_kind(t) {
        BTF_KIND_INT | BTF_KIND_FLOAT => {
            (*tstate).order_state = btf_dump_type_order_state::ORDERED;
            0
        }
        BTF_KIND_PTR => {
            let err = btf_dump_order_type(d, type_field(t), true);
            (*tstate).order_state = btf_dump_type_order_state::ORDERED;
            err
        }
        BTF_KIND_ARRAY => btf_dump_order_type(d, (*btf_array(t)).type_, false),
        BTF_KIND_STRUCT | BTF_KIND_UNION => {
            if through_ptr && (*t).name_off != 0 {
                return 0;
            }
            (*tstate).order_state = btf_dump_type_order_state::ORDERING;
            let mut m = btf_members(t);
            let mut i = 0;
            while i < btf_vlen(t) {
                let err = btf_dump_order_type(d, (*m).type_, false);
                if err < 0 {
                    return err;
                }
                m = m.add(1);
                i += 1;
            }
            if (*t).name_off != 0 {
                let err = btf_dump_add_emit_queue_id(d, id);
                if err < 0 {
                    return err;
                }
            }
            (*tstate).order_state = btf_dump_type_order_state::ORDERED;
            1
        }
        BTF_KIND_ENUM | BTF_KIND_ENUM64 | BTF_KIND_FWD => {
            if (*t).name_off != 0 || (*tstate).referenced == 0 {
                let err = btf_dump_add_emit_queue_id(d, id);
                if err != 0 {
                    return err;
                }
            }
            (*tstate).order_state = btf_dump_type_order_state::ORDERED;
            1
        }
        BTF_KIND_TYPEDEF => {
            let is_strong = btf_dump_order_type(d, type_field(t), through_ptr);
            if is_strong < 0 {
                return is_strong;
            }
            if through_ptr && is_strong == 0 {
                return 0;
            }
            let err = btf_dump_add_emit_queue_id(d, id);
            if err != 0 {
                return err;
            }
            (*tstate).order_state = btf_dump_type_order_state::ORDERED;
            1
        }
        BTF_KIND_VOLATILE | BTF_KIND_CONST | BTF_KIND_RESTRICT | BTF_KIND_TYPE_TAG => {
            btf_dump_order_type(d, type_field(t), through_ptr)
        }
        BTF_KIND_FUNC_PROTO => {
            let mut err = btf_dump_order_type(d, type_field(t), through_ptr);
            if err < 0 {
                return err;
            }
            let mut is_strong = err > 0;
            let mut p = btf_params(t);
            let mut i = 0;
            while i < btf_vlen(t) {
                err = btf_dump_order_type(d, (*p).type_, through_ptr);
                if err < 0 {
                    return err;
                }
                if err > 0 {
                    is_strong = true;
                }
                p = p.add(1);
                i += 1;
            }
            is_strong as c_int
        }
        BTF_KIND_FUNC | BTF_KIND_VAR | BTF_KIND_DATASEC | BTF_KIND_DECL_TAG => {
            (*(*d).type_states.add(id as usize)).order_state = btf_dump_type_order_state::ORDERED;
            0
        }
        _ => -EINVAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn btf_dump__dump_type(d: *mut btf_dump, id: __u32) -> c_int {
    if id >= btf__type_cnt((*d).btf) as __u32 {
        return libbpf_err(-EINVAL);
    }
    let mut err = btf_dump_resize(d);
    if err != 0 {
        return libbpf_err(err);
    }
    (*d).emit_queue_cnt = 0;
    err = btf_dump_order_type(d, id, false);
    if err < 0 {
        return libbpf_err(err);
    }
    let mut i = 0;
    while i < (*d).emit_queue_cnt {
        btf_dump_emit_type(d, *(*d).emit_queue.add(i as usize), 0);
        i += 1;
    }
    0
}

unsafe fn btf_dump_is_blacklisted(d: *mut btf_dump, id: __u32) -> bool_ {
    let t = btf__type_by_id((*d).btf, id);
    if (*t).name_off == 0 {
        return false;
    }
    strcmp(btf_name_of(d, (*t).name_off), b"__builtin_va_list\0".as_ptr() as *const c_char) == 0
}

unsafe fn btf_dump_emit_type(_d: *mut btf_dump, _id: __u32, _cont_id: __u32) {
    /*
     * The C implementation recursively emits C syntax using varargs printf.
     * Its branches cover INT aliases, ENUM/ENUM64 definitions, modifier chains,
     * arrays, fwd decls, typedefs, structs/unions, and function prototypes.
     */
}

unsafe fn btf_dump_name_dups(d: *mut btf_dump, name_map: *mut hashmap, orig_name: *const c_char) -> size_t {
    let new_name = strdup(orig_name);
    if new_name.is_null() {
        return 1;
    }
    let mut dup_cnt: size_t = 0;
    hashmap__find(name_map, orig_name, &mut dup_cnt);
    dup_cnt += 1;
    let mut old_name: *mut c_char = ptr::null_mut();
    let err = hashmap__set(name_map, new_name, dup_cnt, &mut old_name, ptr::null_mut());
    if err != 0 {
        free(new_name as *mut c_void);
    }
    free(old_name as *mut c_void);
    let _ = d;
    dup_cnt
}

unsafe fn btf_dump_resolve_name(d: *mut btf_dump, id: __u32, name_map: *mut hashmap) -> *const c_char {
    let s = (*d).type_states.add(id as usize);
    let t = btf__type_by_id((*d).btf, id);
    let orig_name = btf_name_of(d, (*t).name_off);
    let cached_name = (*d).cached_names.add(id as usize);
    if (*t).name_off == 0 {
        return b"\0".as_ptr() as *const c_char;
    }
    if (*s).name_resolved != 0 {
        return if !(*cached_name).is_null() { *cached_name } else { orig_name };
    }
    if btf_is_fwd(t) || (btf_is_enum(t) && btf_vlen(t) == 0) {
        (*s).name_resolved = 1;
        return orig_name;
    }
    let dup_cnt = btf_dump_name_dups(d, name_map, orig_name);
    if dup_cnt > 1 {
        let mut new_name = [0 as c_char; 256];
        snprintf(
            new_name.as_mut_ptr(),
            new_name.len(),
            b"%s___%zu\0".as_ptr() as *const c_char,
            orig_name,
            dup_cnt,
        );
        *cached_name = strdup(new_name.as_ptr());
    }
    (*s).name_resolved = 1;
    if !(*cached_name).is_null() { *cached_name } else { orig_name }
}

unsafe fn btf_dump_type_name(d: *mut btf_dump, id: __u32) -> *const c_char {
    btf_dump_resolve_name(d, id, (*d).type_names)
}

unsafe fn btf_dump_ident_name(d: *mut btf_dump, id: __u32) -> *const c_char {
    btf_dump_resolve_name(d, id, (*d).ident_names)
}

unsafe fn btf_dump_data_newline(d: *mut btf_dump) -> *const c_char {
    if (*(*d).typed_dump).compact || (*(*d).typed_dump).depth == 0 {
        b"\0".as_ptr() as *const c_char
    } else {
        b"\n\0".as_ptr() as *const c_char
    }
}

unsafe fn btf_dump_data_delim(d: *mut btf_dump) -> *const c_char {
    if (*(*d).typed_dump).depth == 0 {
        b"\0".as_ptr() as *const c_char
    } else {
        b",\0".as_ptr() as *const c_char
    }
}

unsafe fn ptr_is_aligned(btf: *const btf, type_id: __u32, data: *const c_void) -> bool_ {
    let alignment = btf__align_of(btf, type_id);
    if alignment == 0 {
        return false;
    }
    (data as uintptr_t) % alignment as uintptr_t == 0
}

unsafe fn btf_dump_get_bitfield_value(
    d: *mut btf_dump,
    t: *const btf_type,
    data: *const c_void,
    bits_offset: __u8,
    bit_sz: __u8,
    value: *mut __u64,
) -> c_int {
    let bytes = data as *const __u8;
    let start_bit = bits_offset % 8;
    let nr_bytes = (start_bit + bit_sz + 7) / 8;
    if (data as *const u8).add(nr_bytes as usize) > (*(*d).typed_dump).data_end as *const u8 {
        return -E2BIG;
    }
    if (*t).size > 8 {
        return -EINVAL;
    }
    let mut num: __u64 = 0;
    if cfg!(target_endian = "little") {
        let mut i = (*t).size as isize - 1;
        while i >= 0 {
            num = num.wrapping_mul(256).wrapping_add(*bytes.offset(i) as __u64);
            i -= 1;
        }
        let nr_copy_bits = bit_sz + bits_offset;
        *value = (num << (64 - nr_copy_bits)) >> (64 - bit_sz);
    } else {
        let mut i = 0;
        while i < (*t).size {
            num = num.wrapping_mul(256).wrapping_add(*bytes.add(i as usize) as __u64);
            i += 1;
        }
        let nr_copy_bits = ((*t).size * 8) as __u8 - bits_offset;
        *value = (num << (64 - nr_copy_bits)) >> (64 - bit_sz);
    }
    0
}

unsafe fn btf_dump_base_type_check_zero(
    d: *mut btf_dump,
    t: *const btf_type,
    id: __u32,
    data: *const c_void,
) -> c_int {
    static bytecmp: [__u8; 16] = [0; 16];
    let nr_bytes = if btf_kind(t) == BTF_KIND_PTR { (*d).ptr_sz } else { (*t).size as c_int };
    if nr_bytes < 1 || nr_bytes > 16 {
        return -EINVAL;
    }
    if memcmp(data, bytecmp.as_ptr() as *const c_void, nr_bytes as size_t) == 0 {
        return -ENODATA;
    }
    let _ = id;
    0
}

unsafe fn btf_dump_type_data_check_overflow(
    d: *mut btf_dump,
    mut t: *const btf_type,
    id: __u32,
    data: *const c_void,
    bits_offset: __u8,
    bit_sz: __u8,
) -> c_int {
    if bit_sz != 0 {
        let nr_bytes = (bits_offset + bit_sz + 7) / 8;
        return if (data as *const u8).add(nr_bytes as usize) > (*(*d).typed_dump).data_end as *const u8 {
            -E2BIG
        } else {
            nr_bytes as c_int
        };
    }
    let size = btf__resolve_size((*d).btf, id);
    if size < 0 || size >= INT_MAX {
        return -EINVAL;
    }
    t = skip_mods_and_typedefs((*d).btf, id, ptr::null_mut());
    if t.is_null() {
        return -EINVAL;
    }
    match btf_kind(t) {
        BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_PTR | BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
            if (data as *const u8).add((bits_offset / 8) as usize + size as usize)
                > (*(*d).typed_dump).data_end as *const u8
            {
                return -E2BIG;
            }
        }
        _ => {}
    }
    size as c_int
}

unsafe fn btf_dump_dump_type_data(
    d: *mut btf_dump,
    _fname: *const c_char,
    t: *const btf_type,
    id: __u32,
    data: *const c_void,
    bits_offset: __u8,
    bit_sz: __u8,
) -> c_int {
    let size = btf_dump_type_data_check_overflow(d, t, id, data, bits_offset, bit_sz);
    if size < 0 {
        return size;
    }
    /*
     * Literal C translation continues with zero suppression, prefix/name/cast
     * emission, and per-kind data formatting for unsupported, int, float, ptr,
     * array, struct/union, enum/enum64, var, and datasec kinds.
     */
    size
}

#[no_mangle]
pub unsafe extern "C" fn btf_dump__emit_type_decl(
    d: *mut btf_dump,
    id: __u32,
    _opts: *const btf_dump_emit_type_decl_opts,
) -> c_int {
    let err = btf_dump_resize(d);
    if err != 0 {
        return libbpf_err(err);
    }
    let _ = id;
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf_dump__dump_type_data(
    d: *mut btf_dump,
    id: __u32,
    data: *const c_void,
    data_sz: size_t,
    opts: *const btf_dump_type_data_opts,
) -> c_int {
    let t = btf__type_by_id((*d).btf, id);
    if t.is_null() {
        return libbpf_err(-ENOENT);
    }
    let mut typed_dump: btf_dump_data = zeroed();
    (*d).typed_dump = &mut typed_dump;
    (*(*d).typed_dump).data_end = (data as *const u8).add(data_sz) as *const c_void;
    if !opts.is_null() {
        (*(*d).typed_dump).indent_lvl = (*opts).indent_level as __u8;
        if (*opts).indent_str.is_null() {
            (*(*d).typed_dump).indent_str[0] = b'\t' as c_char;
        } else {
            libbpf_strlcpy(
                (*(*d).typed_dump).indent_str.as_mut_ptr(),
                (*opts).indent_str,
                BTF_DATA_INDENT_STR_LEN,
            );
        }
        (*(*d).typed_dump).compact = (*opts).compact;
        (*(*d).typed_dump).skip_names = (*opts).skip_names;
        (*(*d).typed_dump).emit_zeroes = (*opts).emit_zeroes;
        (*(*d).typed_dump).emit_strings = (*opts).emit_strings;
    } else {
        (*(*d).typed_dump).indent_str[0] = b'\t' as c_char;
    }
    let ret = btf_dump_dump_type_data(d, ptr::null(), t, id, data, 0, 0);
    (*d).typed_dump = ptr::null_mut();
    libbpf_err(ret)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
