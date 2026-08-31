// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2019 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type bool_t = bool;
type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const EUCLEAN: c_int = 117;
const EOPNOTSUPP: c_int = 95;
const ERANGE: c_int = 34;
const SHRT_MAX: __u64 = 32767;

/* External kernel/libbpf constants, types, and helpers are provided by other files. */
const BPF_CORE_SPEC_MAX_LEN: usize = 64;
const BPF_CORE_FIELD_BYTE_OFFSET: bpf_core_relo_kind = 0;
const BPF_CORE_FIELD_BYTE_SIZE: bpf_core_relo_kind = 1;
const BPF_CORE_FIELD_EXISTS: bpf_core_relo_kind = 2;
const BPF_CORE_FIELD_SIGNED: bpf_core_relo_kind = 3;
const BPF_CORE_FIELD_LSHIFT_U64: bpf_core_relo_kind = 4;
const BPF_CORE_FIELD_RSHIFT_U64: bpf_core_relo_kind = 5;
const BPF_CORE_TYPE_ID_LOCAL: bpf_core_relo_kind = 6;
const BPF_CORE_TYPE_ID_TARGET: bpf_core_relo_kind = 7;
const BPF_CORE_TYPE_EXISTS: bpf_core_relo_kind = 8;
const BPF_CORE_TYPE_MATCHES: bpf_core_relo_kind = 9;
const BPF_CORE_TYPE_SIZE: bpf_core_relo_kind = 10;
const BPF_CORE_ENUMVAL_EXISTS: bpf_core_relo_kind = 11;
const BPF_CORE_ENUMVAL_VALUE: bpf_core_relo_kind = 12;

const BTF_KIND_UNKN: __u16 = 0;
const BTF_KIND_INT: __u16 = 1;
const BTF_KIND_PTR: __u16 = 2;
const BTF_KIND_ARRAY: __u16 = 3;
const BTF_KIND_STRUCT: __u16 = 4;
const BTF_KIND_UNION: __u16 = 5;
const BTF_KIND_ENUM: __u16 = 6;
const BTF_KIND_FWD: __u16 = 7;
const BTF_KIND_FUNC_PROTO: __u16 = 13;
const BTF_KIND_FLOAT: __u16 = 16;
const BTF_KIND_ENUM64: __u16 = 19;
const BTF_INT_SIGNED: __u8 = 1;

const BPF_LD: __u8 = 0x00;
const BPF_LDX: __u8 = 0x01;
const BPF_ST: __u8 = 0x02;
const BPF_STX: __u8 = 0x03;
const BPF_ALU: __u8 = 0x04;
const BPF_JMP: __u8 = 0x05;
const BPF_ALU64: __u8 = 0x07;
const BPF_K: __u8 = 0x00;
const BPF_W: c_int = 0x00;
const BPF_H: c_int = 0x08;
const BPF_B: c_int = 0x10;
const BPF_DW: c_int = 0x18;
const BPF_IMM: __u8 = 0x00;
const BPF_CALL: __u8 = 0x80;

type bpf_core_relo_kind = c_uint;

#[repr(C)]
pub struct btf {
    _priv: [u8; 0],
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
    pub val: __u32,
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
pub struct bpf_core_relo {
    pub insn_off: __u32,
    pub type_id: __u32,
    pub access_str_off: __u32,
    pub kind: bpf_core_relo_kind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_accessor {
    pub type_id: __u32,
    pub idx: __u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_spec {
    pub btf: *const btf,
    pub root_type_id: __u32,
    pub relo_kind: bpf_core_relo_kind,
    pub raw_spec: [c_int; BPF_CORE_SPEC_MAX_LEN],
    pub raw_len: c_int,
    pub spec: [bpf_core_accessor; BPF_CORE_SPEC_MAX_LEN],
    pub len: c_int,
    pub bit_offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_relo_res {
    pub orig_val: __u64,
    pub new_val: __u64,
    pub poison: bool_t,
    pub validate: bool_t,
    pub fail_memsz_adjust: bool_t,
    pub orig_sz: __u32,
    pub new_sz: __u32,
    pub orig_type_id: __u32,
    pub new_type_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_core_cand {
    pub btf: *const btf,
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_core_cand_list {
    pub len: c_int,
    pub cands: *mut bpf_core_cand,
}

#[repr(C)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_reg: __u8,
    pub src_reg: __u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum libbpf_print_level {
    LIBBPF_WARN,
    LIBBPF_INFO,
    LIBBPF_DEBUG,
}

macro_rules! pr_warn {
    ($($arg:tt)*) => {{}};
}
macro_rules! pr_debug {
    ($($arg:tt)*) => {{}};
}

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;

    fn btf_type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf_type_skip_modifiers(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type;
    fn btf_type_str(t: *const btf_type) -> *const c_char;
    fn btf_name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf_resolve_size(btf: *const btf, t: *const btf_type, size: *mut c_int) -> *const btf_type;
    fn IS_ERR(ptr: *const btf_type) -> bool_t;
    fn PTR_ERR(ptr: *const btf_type) -> __s64;

    fn btf_kind(t: *const btf_type) -> __u16;
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn btf_array(t: *const btf_type) -> *const btf_array;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_params(t: *const btf_type) -> *mut btf_param;
    fn btf_enum(t: *const btf_type) -> *const btf_enum;
    fn btf_enum64(t: *const btf_type) -> *const btf_enum64;
    fn btf_enum64_value(e: *const btf_enum64) -> __u64;
    fn btf_member_bit_offset(t: *const btf_type, member_idx: __u32) -> __u32;
    fn btf_member_bitfield_size(t: *const btf_type, member_idx: __u32) -> __u32;
    fn btf_int_offset(t: *const btf_type) -> __u32;
    fn btf_int_encoding(t: *const btf_type) -> __u8;
    fn btf_is_composite(t: *const btf_type) -> bool_t;
    fn btf_is_array(t: *const btf_type) -> bool_t;
    fn btf_is_enum(t: *const btf_type) -> bool_t;
    fn btf_is_any_enum(t: *const btf_type) -> bool_t;
    fn btf_is_int(t: *const btf_type) -> bool_t;
    fn btf_is_ptr(t: *const btf_type) -> bool_t;
    fn btf_kind_core_compat(local_type: *const btf_type, targ_type: *const btf_type) -> bool_t;
    fn str_is_empty(s: *const c_char) -> bool_t;
    fn bpf_core_essential_name_len(name: *const c_char) -> size_t;
    fn bpf_core_types_are_compat(local_btf: *const btf, local_id: __u32, targ_btf: *const btf, targ_id: __u32) -> c_int;
    fn bpf_core_types_match(local_btf: *const btf, local_id: __u32, targ_btf: *const btf, targ_id: __u32) -> c_int;
}

#[inline]
unsafe fn btf_kind_str(t: *const btf_type) -> *const c_char {
    unsafe { btf_type_str(t) }
}

#[inline]
unsafe fn is_ldimm64_insn(insn: *mut bpf_insn) -> bool_t {
    unsafe { (*insn).code as c_int == (BPF_LD | BPF_IMM) as c_int | BPF_DW }
}

#[inline]
unsafe fn skip_mods_and_typedefs(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type {
    unsafe { btf_type_skip_modifiers(btf, id, res_id) }
}

#[inline]
unsafe fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char {
    unsafe { btf_name_by_offset(btf, offset) }
}

#[inline]
unsafe fn btf__resolve_size(btf: *const btf, type_id: __u32) -> __s64 {
    unsafe {
        let mut size: c_int = 0;
        let mut t = btf_type_by_id(btf, type_id);
        t = btf_resolve_size(btf, t, &mut size);
        if IS_ERR(t) {
            return PTR_ERR(t);
        }
        size as __s64
    }
}

#[inline]
fn BTF_INFO_KFLAG(info: __u32) -> bool_t {
    (info & (1 << 31)) != 0
}

#[inline]
fn BPF_CLASS(code: __u8) -> __u8 {
    code & 0x07
}

#[inline]
fn BPF_SIZE(code: __u8) -> c_int {
    (code & 0x18) as c_int
}

#[inline]
fn BPF_SRC(code: __u8) -> __u8 {
    code & 0x08
}

#[inline]
fn BPF_MODE(code: __u8) -> __u8 {
    code & 0xe0
}

unsafe fn cstr_eq_lit(s: *const c_char, lit: &[u8]) -> bool {
    unsafe {
        if s.is_null() {
            return false;
        }
        let mut i = 0usize;
        while *s.add(i) != 0 {
            if i >= lit.len() || *s.add(i) as u8 != lit[i] {
                return false;
            }
            i += 1;
        }
        i == lit.len()
    }
}

unsafe fn parse_spec_index(mut s: *const c_char, val: *mut c_int, parsed_len: *mut c_int) -> c_int {
    unsafe {
        let start = s;
        let mut v: c_int = 0;
        let mut any = false;
        while *s >= b'0' as c_char && *s <= b'9' as c_char {
            any = true;
            v = v.wrapping_mul(10).wrapping_add((*s - b'0' as c_char) as c_int);
            s = s.add(1);
        }
        if !any {
            return 0;
        }
        *val = v;
        *parsed_len = s.offset_from(start) as c_int;
        1
    }
}

unsafe fn is_flex_arr(btf: *const btf, acc: *const bpf_core_accessor, arr: *const btf_array) -> bool_t {
    unsafe {
        let t: *const btf_type;
        /* not a flexible array, if not inside a struct or has non-zero size */
        if (*acc).name.is_null() || (*arr).nelems > 0 {
            return false;
        }
        /* has to be the last member of enclosing struct */
        t = btf_type_by_id(btf, (*acc).type_id);
        (*acc).idx == btf_vlen(t) - 1
    }
}

unsafe fn core_relo_kind_str(kind: bpf_core_relo_kind) -> *const c_char {
    match kind {
        BPF_CORE_FIELD_BYTE_OFFSET => b"byte_off\0".as_ptr() as *const c_char,
        BPF_CORE_FIELD_BYTE_SIZE => b"byte_sz\0".as_ptr() as *const c_char,
        BPF_CORE_FIELD_EXISTS => b"field_exists\0".as_ptr() as *const c_char,
        BPF_CORE_FIELD_SIGNED => b"signed\0".as_ptr() as *const c_char,
        BPF_CORE_FIELD_LSHIFT_U64 => b"lshift_u64\0".as_ptr() as *const c_char,
        BPF_CORE_FIELD_RSHIFT_U64 => b"rshift_u64\0".as_ptr() as *const c_char,
        BPF_CORE_TYPE_ID_LOCAL => b"local_type_id\0".as_ptr() as *const c_char,
        BPF_CORE_TYPE_ID_TARGET => b"target_type_id\0".as_ptr() as *const c_char,
        BPF_CORE_TYPE_EXISTS => b"type_exists\0".as_ptr() as *const c_char,
        BPF_CORE_TYPE_MATCHES => b"type_matches\0".as_ptr() as *const c_char,
        BPF_CORE_TYPE_SIZE => b"type_size\0".as_ptr() as *const c_char,
        BPF_CORE_ENUMVAL_EXISTS => b"enumval_exists\0".as_ptr() as *const c_char,
        BPF_CORE_ENUMVAL_VALUE => b"enumval_value\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

fn core_relo_is_field_based(kind: bpf_core_relo_kind) -> bool_t {
    matches!(kind, BPF_CORE_FIELD_BYTE_OFFSET | BPF_CORE_FIELD_BYTE_SIZE | BPF_CORE_FIELD_EXISTS | BPF_CORE_FIELD_SIGNED | BPF_CORE_FIELD_LSHIFT_U64 | BPF_CORE_FIELD_RSHIFT_U64)
}

fn core_relo_is_type_based(kind: bpf_core_relo_kind) -> bool_t {
    matches!(kind, BPF_CORE_TYPE_ID_LOCAL | BPF_CORE_TYPE_ID_TARGET | BPF_CORE_TYPE_EXISTS | BPF_CORE_TYPE_MATCHES | BPF_CORE_TYPE_SIZE)
}

fn core_relo_is_enumval_based(kind: bpf_core_relo_kind) -> bool_t {
    matches!(kind, BPF_CORE_ENUMVAL_EXISTS | BPF_CORE_ENUMVAL_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn __bpf_core_types_are_compat(local_btf: *const btf, mut local_id: __u32, targ_btf: *const btf, mut targ_id: __u32, mut level: c_int) -> c_int {
    unsafe {
        let mut local_type = btf_type_by_id(local_btf, local_id);
        let mut targ_type = btf_type_by_id(targ_btf, targ_id);
        let mut depth = 32;
        if !btf_kind_core_compat(local_type, targ_type) {
            return 0;
        }
        loop {
            depth -= 1;
            if depth < 0 {
                return -EINVAL;
            }
            local_type = skip_mods_and_typedefs(local_btf, local_id, &mut local_id);
            targ_type = skip_mods_and_typedefs(targ_btf, targ_id, &mut targ_id);
            if local_type.is_null() || targ_type.is_null() {
                return -EINVAL;
            }
            if !btf_kind_core_compat(local_type, targ_type) {
                return 0;
            }
            match btf_kind(local_type) {
                BTF_KIND_UNKN | BTF_KIND_STRUCT | BTF_KIND_UNION | BTF_KIND_ENUM | BTF_KIND_FWD | BTF_KIND_ENUM64 => return 1,
                BTF_KIND_INT => return (btf_int_offset(local_type) == 0 && btf_int_offset(targ_type) == 0) as c_int,
                BTF_KIND_PTR => {
                    local_id = (*local_type).type_;
                    targ_id = (*targ_type).type_;
                }
                BTF_KIND_ARRAY => {
                    local_id = (*btf_array(local_type)).type_;
                    targ_id = (*btf_array(targ_type)).type_;
                }
                BTF_KIND_FUNC_PROTO => {
                    let mut local_p = btf_params(local_type);
                    let mut targ_p = btf_params(targ_type);
                    let local_vlen = btf_vlen(local_type);
                    let targ_vlen = btf_vlen(targ_type);
                    if local_vlen != targ_vlen {
                        return 0;
                    }
                    for _ in 0..local_vlen {
                        if level <= 0 {
                            return -EINVAL;
                        }
                        skip_mods_and_typedefs(local_btf, (*local_p).type_, &mut local_id);
                        skip_mods_and_typedefs(targ_btf, (*targ_p).type_, &mut targ_id);
                        let err = __bpf_core_types_are_compat(local_btf, local_id, targ_btf, targ_id, level - 1);
                        if err <= 0 {
                            return err;
                        }
                        local_p = local_p.add(1);
                        targ_p = targ_p.add(1);
                    }
                    skip_mods_and_typedefs(local_btf, (*local_type).type_, &mut local_id);
                    skip_mods_and_typedefs(targ_btf, (*targ_type).type_, &mut targ_id);
                }
                _ => {
                    pr_warn!("unexpected kind %s relocated, local [%u], target [%u]\n", btf_kind_str(local_type), local_id, targ_id);
                    return 0;
                }
            }
            level = level;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_core_parse_spec(prog_name: *const c_char, btf: *const btf, relo: *const bpf_core_relo, spec: *mut bpf_core_spec) -> c_int {
    unsafe {
        let mut access_idx: c_int = 0;
        let mut parsed_len: c_int = 0;
        let mut id: __u32 = 0;
        let mut sz: __s64;
        let mut spec_str = btf__name_by_offset(btf, (*relo).access_str_off);
        if str_is_empty(spec_str) || *spec_str == b':' as c_char {
            return -EINVAL;
        }
        memset(spec as *mut c_void, 0, mem::size_of::<bpf_core_spec>());
        (*spec).btf = btf;
        (*spec).root_type_id = (*relo).type_id;
        (*spec).relo_kind = (*relo).kind;
        if core_relo_is_type_based((*relo).kind) {
            if !cstr_eq_lit(spec_str, b"0") {
                return -EINVAL;
            }
            return 0;
        }
        while *spec_str != 0 {
            if *spec_str == b':' as c_char {
                spec_str = spec_str.add(1);
            }
            if parse_spec_index(spec_str, &mut access_idx, &mut parsed_len) != 1 {
                return -EINVAL;
            }
            if access_idx < 0 {
                return -EINVAL;
            }
            if (*spec).raw_len as usize == BPF_CORE_SPEC_MAX_LEN {
                return -E2BIG;
            }
            spec_str = spec_str.add(parsed_len as usize);
            (*spec).raw_spec[(*spec).raw_len as usize] = access_idx;
            (*spec).raw_len += 1;
        }
        if (*spec).raw_len == 0 {
            return -EINVAL;
        }
        let mut t = skip_mods_and_typedefs(btf, (*relo).type_id, &mut id);
        if t.is_null() {
            return -EINVAL;
        }
        access_idx = (*spec).raw_spec[0];
        let mut acc = &mut (*spec).spec[0] as *mut bpf_core_accessor;
        (*acc).type_id = id;
        (*acc).idx = access_idx as __u32;
        (*spec).len += 1;
        if core_relo_is_enumval_based((*relo).kind) {
            if !btf_is_any_enum(t) || (*spec).raw_len > 1 || access_idx as __u32 >= btf_vlen(t) {
                return -EINVAL;
            }
            let name_off = if btf_is_enum(t) {
                (*btf_enum(t).add(access_idx as usize)).name_off
            } else {
                (*btf_enum64(t).add(access_idx as usize)).name_off
            };
            (*acc).name = btf__name_by_offset(btf, name_off);
            return 0;
        }
        if !core_relo_is_field_based((*relo).kind) {
            return -EINVAL;
        }
        sz = btf__resolve_size(btf, id);
        if sz < 0 {
            return sz as c_int;
        }
        (*spec).bit_offset = (access_idx as __u32).wrapping_mul(sz as __u32).wrapping_mul(8);
        for i in 1..(*spec).raw_len {
            t = skip_mods_and_typedefs(btf, id, &mut id);
            if t.is_null() {
                return -EINVAL;
            }
            access_idx = (*spec).raw_spec[i as usize];
            acc = &mut (*spec).spec[(*spec).len as usize] as *mut bpf_core_accessor;
            if btf_is_composite(t) {
                if access_idx as __u32 >= btf_vlen(t) {
                    return -EINVAL;
                }
                let bit_offset = btf_member_bit_offset(t, access_idx as __u32);
                (*spec).bit_offset = (*spec).bit_offset.wrapping_add(bit_offset);
                let m = btf_members(t).add(access_idx as usize);
                if (*m).name_off != 0 {
                    let name = btf__name_by_offset(btf, (*m).name_off);
                    if str_is_empty(name) {
                        return -EINVAL;
                    }
                    (*acc).type_id = id;
                    (*acc).idx = access_idx as __u32;
                    (*acc).name = name;
                    (*spec).len += 1;
                }
                id = (*m).type_;
            } else if btf_is_array(t) {
                let a = btf_array(t);
                t = skip_mods_and_typedefs(btf, (*a).type_, &mut id);
                if t.is_null() {
                    return -EINVAL;
                }
                let flex = is_flex_arr(btf, acc.sub(1), a);
                if !flex && access_idx as __u32 >= (*a).nelems {
                    return -EINVAL;
                }
                (*spec).spec[(*spec).len as usize].type_id = id;
                (*spec).spec[(*spec).len as usize].idx = access_idx as __u32;
                (*spec).len += 1;
                sz = btf__resolve_size(btf, id);
                if sz < 0 {
                    return sz as c_int;
                }
                (*spec).bit_offset = (*spec).bit_offset.wrapping_add((access_idx as __u32).wrapping_mul(sz as __u32).wrapping_mul(8));
            } else {
                pr_warn!("prog '%s': relo for [%u] %s (at idx %d) captures type [%u] of unexpected kind %s\n", prog_name, (*relo).type_id, spec_str, i, id, btf_kind_str(t));
                return -EINVAL;
            }
        }
        0
    }
}

unsafe fn bpf_core_fields_are_compat(local_btf: *const btf, mut local_id: __u32, targ_btf: *const btf, mut targ_id: __u32) -> c_int {
    unsafe {
        loop {
            let local_type = skip_mods_and_typedefs(local_btf, local_id, &mut local_id);
            let targ_type = skip_mods_and_typedefs(targ_btf, targ_id, &mut targ_id);
            if local_type.is_null() || targ_type.is_null() {
                return -EINVAL;
            }
            if btf_is_composite(local_type) && btf_is_composite(targ_type) {
                return 1;
            }
            if !btf_kind_core_compat(local_type, targ_type) {
                return 0;
            }
            match btf_kind(local_type) {
                BTF_KIND_PTR | BTF_KIND_FLOAT => return 1,
                BTF_KIND_FWD | BTF_KIND_ENUM64 | BTF_KIND_ENUM => {
                    let local_name = btf__name_by_offset(local_btf, (*local_type).name_off);
                    let targ_name = btf__name_by_offset(targ_btf, (*targ_type).name_off);
                    let local_len = bpf_core_essential_name_len(local_name);
                    let targ_len = bpf_core_essential_name_len(targ_name);
                    return (local_len == 0 || targ_len == 0 || (local_len == targ_len && strncmp(local_name, targ_name, local_len) == 0)) as c_int;
                }
                BTF_KIND_INT => return (btf_int_offset(local_type) == 0 && btf_int_offset(targ_type) == 0) as c_int,
                BTF_KIND_ARRAY => {
                    local_id = (*btf_array(local_type)).type_;
                    targ_id = (*btf_array(targ_type)).type_;
                }
                _ => return 0,
            }
        }
    }
}

unsafe fn bpf_core_match_member(local_btf: *const btf, local_acc: *const bpf_core_accessor, targ_btf: *const btf, mut targ_id: __u32, spec: *mut bpf_core_spec, next_targ_id: *mut __u32) -> c_int {
    unsafe {
        let targ_type = skip_mods_and_typedefs(targ_btf, targ_id, &mut targ_id);
        if targ_type.is_null() {
            return -EINVAL;
        }
        if !btf_is_composite(targ_type) {
            return 0;
        }
        let local_id = (*local_acc).type_id;
        let local_type = btf_type_by_id(local_btf, local_id);
        let local_member = btf_members(local_type).add((*local_acc).idx as usize);
        let local_name = btf__name_by_offset(local_btf, (*local_member).name_off);
        let n = btf_vlen(targ_type);
        let mut m = btf_members(targ_type);
        for i in 0..n {
            let bit_offset = btf_member_bit_offset(targ_type, i);
            if (*spec).raw_len as usize == BPF_CORE_SPEC_MAX_LEN {
                return -E2BIG;
            }
            (*spec).bit_offset = (*spec).bit_offset.wrapping_add(bit_offset);
            (*spec).raw_spec[(*spec).raw_len as usize] = i as c_int;
            (*spec).raw_len += 1;
            let targ_name = btf__name_by_offset(targ_btf, (*m).name_off);
            if str_is_empty(targ_name) {
                let found = bpf_core_match_member(local_btf, local_acc, targ_btf, (*m).type_, spec, next_targ_id);
                if found != 0 {
                    return found;
                }
            } else if strcmp(local_name, targ_name) == 0 {
                let targ_acc = &mut (*spec).spec[(*spec).len as usize] as *mut bpf_core_accessor;
                (*spec).len += 1;
                (*targ_acc).type_id = targ_id;
                (*targ_acc).idx = i;
                (*targ_acc).name = targ_name;
                *next_targ_id = (*m).type_;
                let found = bpf_core_fields_are_compat(local_btf, (*local_member).type_, targ_btf, (*m).type_);
                if found == 0 {
                    (*spec).len -= 1;
                }
                return found;
            }
            (*spec).bit_offset = (*spec).bit_offset.wrapping_sub(bit_offset);
            (*spec).raw_len -= 1;
            m = m.add(1);
        }
        0
    }
}

unsafe fn bpf_core_spec_match(local_spec: *mut bpf_core_spec, targ_btf: *const btf, mut targ_id: __u32, targ_spec: *mut bpf_core_spec) -> c_int {
    unsafe {
        memset(targ_spec as *mut c_void, 0, mem::size_of::<bpf_core_spec>());
        (*targ_spec).btf = targ_btf;
        (*targ_spec).root_type_id = targ_id;
        (*targ_spec).relo_kind = (*local_spec).relo_kind;
        if core_relo_is_type_based((*local_spec).relo_kind) {
            if (*local_spec).relo_kind == BPF_CORE_TYPE_MATCHES {
                return bpf_core_types_match((*local_spec).btf, (*local_spec).root_type_id, targ_btf, targ_id);
            }
            return bpf_core_types_are_compat((*local_spec).btf, (*local_spec).root_type_id, targ_btf, targ_id);
        }
        let mut local_acc = &(*local_spec).spec[0] as *const bpf_core_accessor;
        let mut targ_acc = &mut (*targ_spec).spec[0] as *mut bpf_core_accessor;
        if core_relo_is_enumval_based((*local_spec).relo_kind) {
            let targ_type = skip_mods_and_typedefs((*targ_spec).btf, targ_id, &mut targ_id);
            if !btf_is_any_enum(targ_type) {
                return 0;
            }
            let local_essent_len = bpf_core_essential_name_len((*local_acc).name);
            for i in 0..btf_vlen(targ_type) {
                let name_off = if btf_is_enum(targ_type) { (*btf_enum(targ_type).add(i as usize)).name_off } else { (*btf_enum64(targ_type).add(i as usize)).name_off };
                let targ_name = btf__name_by_offset((*targ_spec).btf, name_off);
                let targ_essent_len = bpf_core_essential_name_len(targ_name);
                if targ_essent_len != local_essent_len {
                    continue;
                }
                if strncmp((*local_acc).name, targ_name, local_essent_len) == 0 {
                    (*targ_acc).type_id = targ_id;
                    (*targ_acc).idx = i;
                    (*targ_acc).name = targ_name;
                    (*targ_spec).len += 1;
                    (*targ_spec).raw_spec[(*targ_spec).raw_len as usize] = (*targ_acc).idx as c_int;
                    (*targ_spec).raw_len += 1;
                    return 1;
                }
            }
            return 0;
        }
        if !core_relo_is_field_based((*local_spec).relo_kind) {
            return -EINVAL;
        }
        for i in 0..(*local_spec).len {
            let targ_type = skip_mods_and_typedefs((*targ_spec).btf, targ_id, &mut targ_id);
            if targ_type.is_null() {
                return -EINVAL;
            }
            if !(*local_acc).name.is_null() {
                let matched = bpf_core_match_member((*local_spec).btf, local_acc, targ_btf, targ_id, targ_spec, &mut targ_id);
                if matched <= 0 {
                    return matched;
                }
            } else {
                if i > 0 {
                    if !btf_is_array(targ_type) {
                        return 0;
                    }
                    let a = btf_array(targ_type);
                    let flex = is_flex_arr(targ_btf, targ_acc.sub(1), a);
                    if !flex && (*local_acc).idx >= (*a).nelems {
                        return 0;
                    }
                    if skip_mods_and_typedefs(targ_btf, (*a).type_, &mut targ_id).is_null() {
                        return -EINVAL;
                    }
                }
                if (*targ_spec).raw_len as usize == BPF_CORE_SPEC_MAX_LEN {
                    return -E2BIG;
                }
                (*targ_acc).type_id = targ_id;
                (*targ_acc).idx = (*local_acc).idx;
                (*targ_acc).name = ptr::null();
                (*targ_spec).len += 1;
                (*targ_spec).raw_spec[(*targ_spec).raw_len as usize] = (*targ_acc).idx as c_int;
                (*targ_spec).raw_len += 1;
                let sz = btf__resolve_size(targ_btf, targ_id);
                if sz < 0 {
                    return sz as c_int;
                }
                (*targ_spec).bit_offset = (*targ_spec).bit_offset.wrapping_add((*local_acc).idx.wrapping_mul(sz as __u32).wrapping_mul(8));
            }
            local_acc = local_acc.add(1);
            targ_acc = targ_acc.add(1);
        }
        1
    }
}

unsafe fn bpf_core_calc_field_relo(_prog_name: *const c_char, relo: *const bpf_core_relo, spec: *const bpf_core_spec, val: *mut __u64, field_sz: *mut __u32, type_id: *mut __u32, validate: *mut bool_t) -> c_int {
    unsafe {
        *field_sz = 0;
        if (*relo).kind == BPF_CORE_FIELD_EXISTS {
            *val = (!spec.is_null()) as __u64;
            return 0;
        }
        if spec.is_null() {
            return -EUCLEAN;
        }
        let acc = &(*spec).spec[((*spec).len - 1) as usize] as *const bpf_core_accessor;
        let t = btf_type_by_id((*spec).btf, (*acc).type_id);
        if (*acc).name.is_null() {
            if (*relo).kind == BPF_CORE_FIELD_BYTE_OFFSET {
                *val = ((*spec).bit_offset / 8) as __u64;
                let mut elem_id: __u32 = 0;
                let mut et = skip_mods_and_typedefs((*spec).btf, (*acc).type_id, &mut elem_id);
                while btf_is_array(et) {
                    et = skip_mods_and_typedefs((*spec).btf, (*btf_array(et)).type_, &mut elem_id);
                }
                let sz = btf__resolve_size((*spec).btf, elem_id);
                if sz < 0 {
                    return -EINVAL;
                }
                *field_sz = sz as __u32;
                *type_id = (*acc).type_id;
            } else if (*relo).kind == BPF_CORE_FIELD_BYTE_SIZE {
                let sz = btf__resolve_size((*spec).btf, (*acc).type_id);
                if sz < 0 {
                    return -EINVAL;
                }
                *val = sz as __u64;
            } else {
                return -EINVAL;
            }
            if !validate.is_null() {
                *validate = true;
            }
            return 0;
        }
        let m = btf_members(t).add((*acc).idx as usize);
        let mut field_type_id: __u32 = 0;
        let mt = skip_mods_and_typedefs((*spec).btf, (*m).type_, &mut field_type_id);
        let bit_off = (*spec).bit_offset;
        let mut bit_sz = btf_member_bitfield_size(t, (*acc).idx);
        let bitfield = bit_sz > 0;
        let mut byte_sz: __u32;
        let byte_off: __u32;
        if bitfield {
            byte_sz = (*mt).size;
            let mut bo = bit_off / 8 / byte_sz * byte_sz;
            while bit_off + bit_sz - bo * 8 > byte_sz * 8 {
                if byte_sz >= 8 {
                    return -E2BIG;
                }
                byte_sz *= 2;
                bo = bit_off / 8 / byte_sz * byte_sz;
            }
            byte_off = bo;
        } else {
            let sz = btf__resolve_size((*spec).btf, field_type_id);
            if sz < 0 {
                return -EINVAL;
            }
            byte_sz = sz as __u32;
            byte_off = (*spec).bit_offset / 8;
            bit_sz = byte_sz * 8;
        }
        if !validate.is_null() {
            *validate = !bitfield;
        }
        match (*relo).kind {
            BPF_CORE_FIELD_BYTE_OFFSET => {
                *val = byte_off as __u64;
                if !bitfield {
                    let mut elem_id: __u32 = 0;
                    let mut et = skip_mods_and_typedefs((*spec).btf, field_type_id, &mut elem_id);
                    while btf_is_array(et) {
                        et = skip_mods_and_typedefs((*spec).btf, (*btf_array(et)).type_, &mut elem_id);
                    }
                    let sz = btf__resolve_size((*spec).btf, elem_id);
                    if sz < 0 {
                        return -EINVAL;
                    }
                    *field_sz = sz as __u32;
                    *type_id = field_type_id;
                }
            }
            BPF_CORE_FIELD_BYTE_SIZE => *val = byte_sz as __u64,
            BPF_CORE_FIELD_SIGNED => {
                *val = ((btf_is_any_enum(mt) && BTF_INFO_KFLAG((*mt).info)) || (btf_is_int(mt) && (btf_int_encoding(mt) & BTF_INT_SIGNED) != 0)) as __u64;
                if !validate.is_null() {
                    *validate = true;
                }
            }
            BPF_CORE_FIELD_LSHIFT_U64 => {
                #[cfg(target_endian = "little")]
                {
                    *val = (64 - (bit_off + bit_sz - byte_off * 8)) as __u64;
                }
                #[cfg(not(target_endian = "little"))]
                {
                    *val = ((8 - byte_sz) * 8 + (bit_off - byte_off * 8)) as __u64;
                }
            }
            BPF_CORE_FIELD_RSHIFT_U64 => {
                *val = (64 - bit_sz) as __u64;
                if !validate.is_null() {
                    *validate = true;
                }
            }
            _ => return -EOPNOTSUPP,
        }
        0
    }
}

unsafe fn bpf_core_calc_type_relo(relo: *const bpf_core_relo, spec: *const bpf_core_spec, val: *mut __u64, validate: *mut bool_t) -> c_int {
    unsafe {
        if !validate.is_null() {
            *validate = true;
        }
        if spec.is_null() {
            *val = 0;
            return 0;
        }
        match (*relo).kind {
            BPF_CORE_TYPE_ID_TARGET => {
                *val = (*spec).root_type_id as __u64;
                if !validate.is_null() {
                    *validate = false;
                }
            }
            BPF_CORE_TYPE_EXISTS | BPF_CORE_TYPE_MATCHES => *val = 1,
            BPF_CORE_TYPE_SIZE => {
                let sz = btf__resolve_size((*spec).btf, (*spec).root_type_id);
                if sz < 0 {
                    return -EINVAL;
                }
                *val = sz as __u64;
            }
            _ => return -EOPNOTSUPP,
        }
        0
    }
}

unsafe fn bpf_core_calc_enumval_relo(relo: *const bpf_core_relo, spec: *const bpf_core_spec, val: *mut __u64) -> c_int {
    unsafe {
        match (*relo).kind {
            BPF_CORE_ENUMVAL_EXISTS => *val = (!spec.is_null()) as __u64,
            BPF_CORE_ENUMVAL_VALUE => {
                if spec.is_null() {
                    return -EUCLEAN;
                }
                let t = btf_type_by_id((*spec).btf, (*spec).spec[0].type_id);
                if btf_is_enum(t) {
                    *val = (*btf_enum(t).add((*spec).spec[0].idx as usize)).val as __u64;
                } else {
                    *val = btf_enum64_value(btf_enum64(t).add((*spec).spec[0].idx as usize));
                }
            }
            _ => return -EOPNOTSUPP,
        }
        0
    }
}

unsafe fn bpf_core_calc_relo(prog_name: *const c_char, relo: *const bpf_core_relo, relo_idx: c_int, local_spec: *const bpf_core_spec, targ_spec: *const bpf_core_spec, res: *mut bpf_core_relo_res) -> c_int {
    unsafe {
        let mut err = -EOPNOTSUPP;
        (*res).orig_val = 0;
        (*res).new_val = 0;
        (*res).poison = false;
        (*res).validate = true;
        (*res).fail_memsz_adjust = false;
        (*res).orig_sz = 0;
        (*res).new_sz = 0;
        (*res).orig_type_id = 0;
        (*res).new_type_id = 0;
        if core_relo_is_field_based((*relo).kind) {
            err = bpf_core_calc_field_relo(prog_name, relo, local_spec, &mut (*res).orig_val, &mut (*res).orig_sz, &mut (*res).orig_type_id, &mut (*res).validate);
            if err == 0 {
                err = bpf_core_calc_field_relo(prog_name, relo, targ_spec, &mut (*res).new_val, &mut (*res).new_sz, &mut (*res).new_type_id, ptr::null_mut());
            }
            if err == 0 {
                (*res).fail_memsz_adjust = false;
                if (*res).orig_sz != (*res).new_sz {
                    let orig_t = btf_type_by_id((*local_spec).btf, (*res).orig_type_id);
                    let new_t = btf_type_by_id((*targ_spec).btf, (*res).new_type_id);
                    if !(btf_is_ptr(orig_t) && btf_is_ptr(new_t))
                        && !(btf_is_int(orig_t) && btf_is_int(new_t)
                            && btf_int_encoding(orig_t) != BTF_INT_SIGNED
                            && btf_int_encoding(new_t) != BTF_INT_SIGNED)
                    {
                        (*res).fail_memsz_adjust = true;
                    }
                }
            }
        } else if core_relo_is_type_based((*relo).kind) {
            err = bpf_core_calc_type_relo(relo, local_spec, &mut (*res).orig_val, &mut (*res).validate);
            if err == 0 {
                err = bpf_core_calc_type_relo(relo, targ_spec, &mut (*res).new_val, ptr::null_mut());
            }
        } else if core_relo_is_enumval_based((*relo).kind) {
            err = bpf_core_calc_enumval_relo(relo, local_spec, &mut (*res).orig_val);
            if err == 0 {
                err = bpf_core_calc_enumval_relo(relo, targ_spec, &mut (*res).new_val);
            }
        }
        if err == -EUCLEAN {
            (*res).poison = true;
            err = 0;
        } else if err == -EOPNOTSUPP {
            pr_warn!("prog '%s': relo #%d: unrecognized CO-RE relocation %s (%u) at insn #%u\n", prog_name, relo_idx, core_relo_kind_str((*relo).kind), (*relo).kind, (*relo).insn_off / 8);
        }
        err
    }
}

unsafe fn bpf_core_poison_insn(_prog_name: *const c_char, _relo_idx: c_int, _insn_idx: c_int, insn: *mut bpf_insn) {
    unsafe {
        pr_debug!("prog '%s': relo #%d: substituting insn #%d w/ invalid insn\n", _prog_name, _relo_idx, _insn_idx);
        (*insn).code = BPF_JMP | BPF_CALL;
        (*insn).dst_reg = 0;
        (*insn).src_reg = 0;
        (*insn).off = 0;
        (*insn).imm = 195896080;
    }
}

unsafe fn insn_bpf_size_to_bytes(insn: *mut bpf_insn) -> c_int {
    unsafe {
        match BPF_SIZE((*insn).code) {
            BPF_DW => 8,
            BPF_W => 4,
            BPF_H => 2,
            BPF_B => 1,
            _ => -1,
        }
    }
}

fn insn_bytes_to_bpf_size(sz: __u32) -> c_int {
    match sz {
        8 => BPF_DW,
        4 => BPF_W,
        2 => BPF_H,
        1 => BPF_B,
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_core_patch_insn(prog_name: *const c_char, insn: *mut bpf_insn, insn_idx: c_int, relo: *const bpf_core_relo, relo_idx: c_int, res: *const bpf_core_relo_res) -> c_int {
    unsafe {
        let class = BPF_CLASS((*insn).code);
        if (*res).poison {
            if is_ldimm64_insn(insn) {
                bpf_core_poison_insn(prog_name, relo_idx, insn_idx + 1, insn.add(1));
            }
            bpf_core_poison_insn(prog_name, relo_idx, insn_idx, insn);
            return 0;
        }
        let mut orig_val = (*res).orig_val;
        let new_val = (*res).new_val;
        match class {
            BPF_ALU | BPF_ALU64 => {
                if BPF_SRC((*insn).code) != BPF_K {
                    return -EINVAL;
                }
                if (*res).validate && (*insn).imm as __u64 != orig_val {
                    return -EINVAL;
                }
                orig_val = (*insn).imm as __u64;
                (*insn).imm = new_val as i32;
                pr_debug!("prog '%s': relo #%d: patched insn #%d (ALU/ALU64) imm %llu -> %llu\n", prog_name, relo_idx, insn_idx, orig_val, new_val);
            }
            BPF_LDX | BPF_ST | BPF_STX => {
                if (*res).validate && (*insn).off as __u64 != orig_val {
                    return -EINVAL;
                }
                if new_val > SHRT_MAX {
                    return -ERANGE;
                }
                if (*res).fail_memsz_adjust {
                    if is_ldimm64_insn(insn) {
                        bpf_core_poison_insn(prog_name, relo_idx, insn_idx + 1, insn.add(1));
                    }
                    bpf_core_poison_insn(prog_name, relo_idx, insn_idx, insn);
                    return 0;
                }
                orig_val = (*insn).off as __u64;
                (*insn).off = new_val as i16;
                pr_debug!("prog '%s': relo #%d: patched insn #%d (LDX/ST/STX) off %llu -> %llu\n", prog_name, relo_idx, insn_idx, orig_val, new_val);
                if (*res).new_sz != (*res).orig_sz {
                    let insn_bytes_sz = insn_bpf_size_to_bytes(insn);
                    if insn_bytes_sz as __u32 != (*res).orig_sz {
                        return -EINVAL;
                    }
                    let insn_bpf_sz = insn_bytes_to_bpf_size((*res).new_sz);
                    if insn_bpf_sz < 0 {
                        return -EINVAL;
                    }
                    (*insn).code = BPF_MODE((*insn).code) | insn_bpf_sz as __u8 | BPF_CLASS((*insn).code);
                }
            }
            BPF_LD => {
                if !is_ldimm64_insn(insn)
                    || (*insn.add(0)).src_reg != 0
                    || (*insn.add(0)).off != 0
                    || (*insn.add(1)).code != 0
                    || (*insn.add(1)).dst_reg != 0
                    || (*insn.add(1)).src_reg != 0
                    || (*insn.add(1)).off != 0
                {
                    return -EINVAL;
                }
                let imm = ((*insn.add(0)).imm as __u32 as __u64) | (((*insn.add(1)).imm as __u64) << 32);
                if (*res).validate && imm != orig_val {
                    return -EINVAL;
                }
                (*insn.add(0)).imm = new_val as i32;
                (*insn.add(1)).imm = (new_val >> 32) as i32;
            }
            _ => {
                pr_warn!("prog '%s': relo #%d: trying to relocate unrecognized insn #%d, code:0x%x, src:0x%x, dst:0x%x, off:0x%x, imm:0x%x\n", prog_name, relo_idx, insn_idx, (*insn).code, (*insn).src_reg, (*insn).dst_reg, (*insn).off, (*insn).imm);
                return -EINVAL;
            }
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_core_format_spec(buf: *mut c_char, mut buf_sz: size_t, spec: *const bpf_core_spec) -> c_int {
    unsafe {
        let mut out = buf;
        let mut len: c_int = 0;
        macro_rules! append_buf {
            ($fmt:expr $(, $arg:expr)*) => {{
                let mut r = snprintf(out, buf_sz, $fmt.as_ptr() as *const c_char $(, $arg)*);
                len += r;
                if r as size_t >= buf_sz {
                    r = buf_sz as c_int;
                }
                out = out.add(r as usize);
                buf_sz = buf_sz.saturating_sub(r as usize);
            }};
        }
        let type_id = (*spec).root_type_id;
        let mut t = btf_type_by_id((*spec).btf, type_id);
        let mut s = btf__name_by_offset((*spec).btf, (*t).name_off);
        append_buf!(b"<%s> [%u] %s %s\0", core_relo_kind_str((*spec).relo_kind), type_id, btf_kind_str(t), if str_is_empty(s) { b"<anon>\0".as_ptr() as *const c_char } else { s });
        if core_relo_is_type_based((*spec).relo_kind) {
            return len;
        }
        if core_relo_is_enumval_based((*spec).relo_kind) {
            t = skip_mods_and_typedefs((*spec).btf, type_id, ptr::null_mut());
            if btf_is_enum(t) {
                let e = btf_enum(t).add((*spec).raw_spec[0] as usize);
                s = btf__name_by_offset((*spec).btf, (*e).name_off);
                if BTF_INFO_KFLAG((*t).info) {
                    append_buf!(b"::%s = %d\0", s, (*e).val as c_int);
                } else {
                    append_buf!(b"::%s = %u\0", s, (*e).val);
                }
            } else {
                let e = btf_enum64(t).add((*spec).raw_spec[0] as usize);
                s = btf__name_by_offset((*spec).btf, (*e).name_off);
                if BTF_INFO_KFLAG((*t).info) {
                    append_buf!(b"::%s = %lld\0", s, btf_enum64_value(e) as i64);
                } else {
                    append_buf!(b"::%s = %llu\0", s, btf_enum64_value(e));
                }
            }
            return len;
        }
        if core_relo_is_field_based((*spec).relo_kind) {
            for i in 0..(*spec).len {
                if !(*spec).spec[i as usize].name.is_null() {
                    append_buf!(b".%s\0", (*spec).spec[i as usize].name);
                } else if i > 0 || (*spec).spec[i as usize].idx > 0 {
                    append_buf!(b"[%u]\0", (*spec).spec[i as usize].idx);
                }
            }
            append_buf!(b" (\0");
            for i in 0..(*spec).raw_len {
                append_buf!(b"%s%d\0", if i == 0 { b"\0".as_ptr() as *const c_char } else { b":\0".as_ptr() as *const c_char }, (*spec).raw_spec[i as usize]);
            }
            if (*spec).bit_offset % 8 != 0 {
                append_buf!(b" @ offset %u.%u)\0", (*spec).bit_offset / 8, (*spec).bit_offset % 8);
            } else {
                append_buf!(b" @ offset %u)\0", (*spec).bit_offset / 8);
            }
            return len;
        }
        len
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_core_calc_relo_insn(prog_name: *const c_char, relo: *const bpf_core_relo, relo_idx: c_int, local_btf: *const btf, cands: *mut bpf_core_cand_list, specs_scratch: *mut bpf_core_spec, targ_res: *mut bpf_core_relo_res) -> c_int {
    unsafe {
        let local_spec = specs_scratch.add(0);
        let cand_spec = specs_scratch.add(1);
        let targ_spec = specs_scratch.add(2);
        let mut cand_res: bpf_core_relo_res = mem::zeroed();
        let local_id = (*relo).type_id;
        let local_type = btf_type_by_id(local_btf, local_id);
        let local_name = btf__name_by_offset(local_btf, (*local_type).name_off);
        if local_name.is_null() {
            return -EINVAL;
        }
        let mut err = bpf_core_parse_spec(prog_name, local_btf, relo, local_spec);
        if err != 0 {
            return -EINVAL;
        }
        let mut spec_buf = [0 as c_char; 256];
        bpf_core_format_spec(spec_buf.as_mut_ptr(), spec_buf.len(), local_spec);
        if (*relo).kind == BPF_CORE_TYPE_ID_LOCAL {
            memset(targ_res as *mut c_void, 0, mem::size_of::<bpf_core_relo_res>());
            (*targ_res).validate = false;
            (*targ_res).poison = false;
            (*targ_res).orig_val = (*local_spec).root_type_id as __u64;
            (*targ_res).new_val = (*local_spec).root_type_id as __u64;
            return 0;
        }
        if str_is_empty(local_name) {
            return -EOPNOTSUPP;
        }
        let mut j: c_int = 0;
        for i in 0..(*cands).len {
            let cand = (*cands).cands.add(i as usize);
            err = bpf_core_spec_match(local_spec, (*cand).btf, (*cand).id, cand_spec);
            if err < 0 {
                return err;
            }
            if err == 0 {
                continue;
            }
            err = bpf_core_calc_relo(prog_name, relo, relo_idx, local_spec, cand_spec, &mut cand_res);
            if err != 0 {
                return err;
            }
            if j == 0 {
                *targ_res = cand_res;
                *targ_spec = *cand_spec;
            } else if (*cand_spec).bit_offset != (*targ_spec).bit_offset {
                return -EINVAL;
            } else if cand_res.poison != (*targ_res).poison || cand_res.new_val != (*targ_res).new_val {
                return -EINVAL;
            }
            *(*cands).cands.add(j as usize) = *cand;
            j += 1;
        }
        if j > 0 {
            (*cands).len = j;
        }
        if j == 0 {
            err = bpf_core_calc_relo(prog_name, relo, relo_idx, local_spec, ptr::null(), targ_res);
            if err != 0 {
                return err;
            }
        }
        0
    }
}

unsafe fn bpf_core_names_match(local_btf: *const btf, local_name_off: size_t, targ_btf: *const btf, targ_name_off: size_t) -> bool_t {
    unsafe {
        let local_n = btf__name_by_offset(local_btf, local_name_off as __u32);
        let targ_n = btf__name_by_offset(targ_btf, targ_name_off as __u32);
        if str_is_empty(targ_n) {
            return str_is_empty(local_n);
        }
        let targ_len = bpf_core_essential_name_len(targ_n);
        let local_len = bpf_core_essential_name_len(local_n);
        targ_len == local_len && strncmp(local_n, targ_n, local_len) == 0
    }
}

unsafe fn bpf_core_enums_match(local_btf: *const btf, local_t: *const btf_type, targ_btf: *const btf, targ_t: *const btf_type) -> c_int {
    unsafe {
        let local_vlen = btf_vlen(local_t);
        let targ_vlen = btf_vlen(targ_t);
        if (*local_t).size != (*targ_t).size || local_vlen > targ_vlen {
            return 0;
        }
        for i in 0..local_vlen {
            let mut matched = false;
            let local_n_off = if btf_is_enum(local_t) { (*btf_enum(local_t).add(i as usize)).name_off } else { (*btf_enum64(local_t).add(i as usize)).name_off };
            for j in 0..targ_vlen {
                let targ_n_off = if btf_is_enum(targ_t) { (*btf_enum(targ_t).add(j as usize)).name_off } else { (*btf_enum64(targ_t).add(j as usize)).name_off };
                if bpf_core_names_match(local_btf, local_n_off as size_t, targ_btf, targ_n_off as size_t) {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return 0;
            }
        }
        1
    }
}

unsafe fn bpf_core_composites_match(local_btf: *const btf, local_t: *const btf_type, targ_btf: *const btf, targ_t: *const btf_type, behind_ptr: bool_t, level: c_int) -> c_int {
    unsafe {
        let mut local_m = btf_members(local_t);
        let local_vlen = btf_vlen(local_t);
        let targ_vlen = btf_vlen(targ_t);
        if local_vlen > targ_vlen {
            return 0;
        }
        for _i in 0..local_vlen {
            let mut targ_m = btf_members(targ_t);
            let mut matched = false;
            for _j in 0..targ_vlen {
                if !bpf_core_names_match(local_btf, (*local_m).name_off as size_t, targ_btf, (*targ_m).name_off as size_t) {
                    targ_m = targ_m.add(1);
                    continue;
                }
                let err = __bpf_core_types_match(local_btf, (*local_m).type_, targ_btf, (*targ_m).type_, behind_ptr, level - 1);
                if err < 0 {
                    return err;
                }
                if err > 0 {
                    matched = true;
                    break;
                }
                targ_m = targ_m.add(1);
            }
            if !matched {
                return 0;
            }
            local_m = local_m.add(1);
        }
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn __bpf_core_types_match(local_btf: *const btf, mut local_id: __u32, targ_btf: *const btf, mut targ_id: __u32, mut behind_ptr: bool_t, mut level: c_int) -> c_int {
    unsafe {
        if level <= 0 {
            return -EINVAL;
        }
        let mut depth = 32;
        loop {
            depth -= 1;
            if depth < 0 {
                return -EINVAL;
            }
            let local_t = skip_mods_and_typedefs(local_btf, local_id, &mut local_id);
            let targ_t = skip_mods_and_typedefs(targ_btf, targ_id, &mut targ_id);
            if local_t.is_null() || targ_t.is_null() {
                return -EINVAL;
            }
            if !bpf_core_names_match(local_btf, (*local_t).name_off as size_t, targ_btf, (*targ_t).name_off as size_t) {
                return 0;
            }
            let local_k = btf_kind(local_t);
            let targ_k = btf_kind(targ_t);
            match local_k {
                BTF_KIND_UNKN => return (local_k == targ_k) as c_int,
                BTF_KIND_FWD => {
                    let local_f = BTF_INFO_KFLAG((*local_t).info);
                    if behind_ptr {
                        if local_k == targ_k {
                            return (local_f == BTF_INFO_KFLAG((*targ_t).info)) as c_int;
                        }
                        return ((targ_k == BTF_KIND_STRUCT && !local_f) || (targ_k == BTF_KIND_UNION && local_f)) as c_int;
                    }
                    if local_k != targ_k {
                        return 0;
                    }
                    return (local_f == BTF_INFO_KFLAG((*targ_t).info)) as c_int;
                }
                BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
                    if !btf_is_any_enum(targ_t) {
                        return 0;
                    }
                    return bpf_core_enums_match(local_btf, local_t, targ_btf, targ_t);
                }
                BTF_KIND_STRUCT | BTF_KIND_UNION => {
                    if behind_ptr {
                        let targ_f = BTF_INFO_KFLAG((*targ_t).info);
                        if local_k == targ_k {
                            return 1;
                        }
                        if targ_k != BTF_KIND_FWD {
                            return 0;
                        }
                        return ((local_k == BTF_KIND_UNION) == targ_f) as c_int;
                    }
                    if local_k != targ_k {
                        return 0;
                    }
                    return bpf_core_composites_match(local_btf, local_t, targ_btf, targ_t, behind_ptr, level);
                }
                BTF_KIND_INT => {
                    if local_k != targ_k {
                        return 0;
                    }
                    let local_sgn = btf_int_encoding(local_t) & BTF_INT_SIGNED;
                    let targ_sgn = btf_int_encoding(targ_t) & BTF_INT_SIGNED;
                    return ((*local_t).size == (*targ_t).size && local_sgn == targ_sgn) as c_int;
                }
                BTF_KIND_PTR => {
                    if local_k != targ_k {
                        return 0;
                    }
                    behind_ptr = true;
                    local_id = (*local_t).type_;
                    targ_id = (*targ_t).type_;
                }
                BTF_KIND_ARRAY => {
                    let local_array = btf_array(local_t);
                    let targ_array = btf_array(targ_t);
                    if local_k != targ_k {
                        return 0;
                    }
                    if (*local_array).nelems != (*targ_array).nelems {
                        return 0;
                    }
                    local_id = (*local_array).type_;
                    targ_id = (*targ_array).type_;
                }
                BTF_KIND_FUNC_PROTO => {
                    let mut local_p = btf_params(local_t);
                    let mut targ_p = btf_params(targ_t);
                    let local_vlen = btf_vlen(local_t);
                    let targ_vlen = btf_vlen(targ_t);
                    if local_k != targ_k || local_vlen != targ_vlen {
                        return 0;
                    }
                    for _ in 0..local_vlen {
                        let err = __bpf_core_types_match(local_btf, (*local_p).type_, targ_btf, (*targ_p).type_, behind_ptr, level - 1);
                        if err <= 0 {
                            return err;
                        }
                        local_p = local_p.add(1);
                        targ_p = targ_p.add(1);
                    }
                    local_id = (*local_t).type_;
                    targ_id = (*targ_t).type_;
                }
                _ => {
                    pr_warn!("unexpected kind %s relocated, local [%u], target [%u]\n", btf_kind_str(local_t), local_id, targ_id);
                    return 0;
                }
            }
            level = level;
        }
    }
}
