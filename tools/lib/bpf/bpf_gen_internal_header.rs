/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (c) 2021 Facebook */

/* Translated from lib/bpf/bpf_gen_internal.h. */
/* C dependencies: "bpf.h" and "libbpf_internal.h". */

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type __u32 = u32;
pub type __u64 = u64;

/* External declarations supplied by included headers in the original C file. */
#[repr(C)]
pub struct gen_loader_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_core_relo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_create_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

pub type bpf_map_type = c_int;
pub type bpf_prog_type = c_int;
pub type bpf_attach_type = c_int;

#[repr(C)]
pub struct ksym_relo_desc {
    pub name: *const c_char,
    pub kind: c_int,
    pub insn_idx: c_int,
    pub is_weak: bool,
    pub is_typeless: bool,
    pub is_ld64: bool,
}

#[repr(C)]
pub union ksym_desc__bindgen_ty_1 {
    /* used for kfunc */
    pub off: c_int,
    /* used for typeless ksym */
    pub typeless: bool,
}

#[repr(C)]
pub struct ksym_desc {
    pub name: *const c_char,
    pub ref_: c_int,
    pub kind: c_int,
    pub anon_1: ksym_desc__bindgen_ty_1,
    pub insn: c_int,
    pub is_ld64: bool,
}

#[repr(C)]
pub struct bpf_gen {
    pub opts: *mut gen_loader_opts,
    pub data_start: *mut c_void,
    pub data_cur: *mut c_void,
    pub insn_start: *mut c_void,
    pub insn_cur: *mut c_void,
    pub swapped_endian: bool,
    pub cleanup_label: ssize_t,
    pub nr_progs: __u32,
    pub nr_maps: __u32,
    pub log_level: c_int,
    pub error: c_int,
    pub relos: *mut ksym_relo_desc,
    pub relo_cnt: c_int,
    pub core_relos: *mut bpf_core_relo,
    pub core_relo_cnt: c_int,
    pub attach_target: [c_char; 128],
    pub attach_kind: c_int,
    pub ksyms: *mut ksym_desc,
    pub nr_ksyms: __u32,
    pub fd_array: c_int,
    pub nr_fd_array: c_int,
}

unsafe extern "C" {
    pub fn bpf_gen__init(gen: *mut bpf_gen, log_level: c_int, nr_progs: c_int, nr_maps: c_int);
    pub fn bpf_gen__finish(gen: *mut bpf_gen, nr_progs: c_int, nr_maps: c_int) -> c_int;
    pub fn bpf_gen__free(gen: *mut bpf_gen);
    pub fn bpf_gen__load_btf(gen: *mut bpf_gen, raw_data: *const c_void, raw_size: __u32);
    pub fn bpf_gen__map_create(
        gen: *mut bpf_gen,
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        map_attr: *mut bpf_map_create_opts,
        map_idx: c_int,
    );
    pub fn bpf_gen__prog_load(
        gen: *mut bpf_gen,
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *mut bpf_insn,
        insn_cnt: size_t,
        load_attr: *mut bpf_prog_load_opts,
        prog_idx: c_int,
    );
    pub fn bpf_gen__map_update_elem(
        gen: *mut bpf_gen,
        map_idx: c_int,
        value: *mut c_void,
        value_size: __u32,
        flags: __u64,
    );
    pub fn bpf_gen__map_freeze(gen: *mut bpf_gen, map_idx: c_int);
    pub fn bpf_gen__record_attach_target(
        gen: *mut bpf_gen,
        name: *const c_char,
        type_: bpf_attach_type,
    );
    pub fn bpf_gen__record_extern(
        gen: *mut bpf_gen,
        name: *const c_char,
        is_weak: bool,
        is_typeless: bool,
        is_ld64: bool,
        kind: c_int,
        insn_idx: c_int,
    );
    pub fn bpf_gen__record_relo_core(gen: *mut bpf_gen, core_relo: *const bpf_core_relo);
    pub fn bpf_gen__populate_outer_map(
        gen: *mut bpf_gen,
        outer_map_idx: c_int,
        key: c_int,
        inner_map_idx: c_int,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
