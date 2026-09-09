// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019 Facebook */
// Translated from bpf_struct_ops.c. Linux/BPF declarations referenced here are
// supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub const MAX_TRAMP_IMAGE_PAGES: usize = 8;
pub const VALUE_PREFIX: &[u8] = b"bpf_struct_ops_\0";
pub const VALUE_PREFIX_LEN: usize = VALUE_PREFIX.len() - 1;
pub const MAYBE_NULL_SUFFIX: &[u8] = b"__nullable\0";
pub const REFCOUNTED_SUFFIX: &[u8] = b"__ref\0";
pub const ARENA_SUFFIX: &[u8] = b"__arena\0";
pub const ARENA_MAYBE_NULL_SUFFIX: &[u8] = b"__arena__nullable\0";

#[repr(C)]
pub struct bpf_struct_ops_value { pub common: bpf_struct_ops_common_value, pub data: [u8; 0] }
#[repr(C)]
pub struct bpf_struct_ops_map {
    pub map: bpf_map, pub st_ops_desc: *const bpf_struct_ops_desc, pub lock: mutex,
    pub links: *mut *mut bpf_link, pub ksyms: *mut *mut bpf_ksym,
    pub funcs_cnt: u32, pub image_pages_cnt: u32,
    pub image_pages: [*mut core::ffi::c_void; MAX_TRAMP_IMAGE_PAGES],
    pub btf: *mut btf, pub uvalue: *mut bpf_struct_ops_value, pub kvalue: bpf_struct_ops_value,
}
#[repr(C)] pub struct bpf_struct_ops_link { pub link: bpf_link, pub map: *mut bpf_map, pub wait_hup: wait_queue_head_t }
#[repr(C)] pub struct bpf_verifier_ops {}
#[repr(C)] pub struct bpf_prog_ops { pub test_run: Option<unsafe extern "C" fn()> }

extern "C" {
    static mut btf_vmlinux: *mut btf;
    static mut update_mutex: mutex;
    static mut bpf_struct_ops_verifier_ops: bpf_verifier_ops;
    static mut bpf_struct_ops_prog_ops: bpf_prog_ops;
}

/* External kernel types and helpers are intentionally unresolved dependencies. */
extern "C" {
    fn btf_type_by_id(_: *mut btf, _: i32) -> *const btf_type;
    fn btf_vlen(_: *const btf_type) -> u32;
    fn btf_type_member(_: *const btf_type) -> *const btf_member;
    fn btf_name_by_offset(_: *mut btf, _: u32) -> *const core::ffi::c_char;
    fn btf_find_by_name_kind(_: *mut btf, _: *const core::ffi::c_char, _: u32) -> i32;
    fn btf_type_resolve_ptr(_: *mut btf, _: u32, _: *mut u32) -> *const btf_type;
    fn btf_type_is_struct(_: *const btf_type) -> bool;
    fn __btf_type_is_struct(_: *const btf_type) -> bool;
    fn btf_type_resolve_func_ptr(_: *mut btf, _: u32, _: *mut u32) -> *const btf_type;
    fn btf_type_vlen(_: *const btf_type) -> u32;
    fn btf_get_name(_: *mut btf) -> *const core::ffi::c_char;
    fn bpf_jit_charge_modmem(_: usize) -> i32;
    fn bpf_jit_uncharge_modmem(_: usize);
    fn arch_alloc_bpf_trampoline(_: usize) -> *mut core::ffi::c_void;
    fn arch_free_bpf_trampoline(_: *mut core::ffi::c_void, _: usize);
    fn bpf_struct_ops_desc_release(_: *mut bpf_struct_ops_desc);
}

#[repr(C)] pub struct bpf_struct_ops_common_value { pub state: i32, pub refcnt: refcount_t }
#[repr(C)] pub struct bpf_struct_ops_desc { pub st_ops: *const bpf_struct_ops, pub type_: *const btf_type, pub type_id: i32, pub value_id: i32, pub value_type: *const btf_type, pub arg_info: *mut bpf_struct_ops_arg_info }
#[repr(C)] pub struct bpf_struct_ops_arg_info { pub info: *mut bpf_ctx_arg_aux, pub cnt: u32 }
#[repr(C)] pub struct bpf_struct_ops { pub name: *const core::ffi::c_char, pub cfi_stubs: *mut *mut core::ffi::c_void, pub func_models: *mut btf_func_model, pub owner: *mut module, pub init: Option<unsafe extern "C" fn(*mut btf) -> i32>, pub init_member: Option<unsafe extern "C" fn(*const btf_type, *const btf_member, *mut u8, *mut u8) -> i32>, pub validate: Option<unsafe extern "C" fn(*mut u8) -> i32>, pub reg: Option<unsafe extern "C" fn(*mut u8, *mut bpf_link) -> i32>, pub unreg: Option<unsafe extern "C" fn(*mut u8, *mut bpf_link)>, pub update: Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut bpf_link) -> i32> }

#[repr(C)] pub struct bpf_map { pub map_type: u32, pub map_flags: u32, pub key_size: u32, pub value_size: u32, pub max_entries: u32, pub btf_vmlinux_value_type_id: u32, pub id: u32, pub refcnt: atomic64_t, pub usercnt: atomic64_t }
#[repr(C)] pub struct bpf_link { pub prog: *mut bpf_prog }
#[repr(C)] pub struct bpf_ksym { pub name: [u8; 128] }
#[repr(C)] pub struct bpf_prog { pub type_: u32, pub expected_attach_type: u32, pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub id: u32, pub attach_btf_id: u32 }
#[repr(C)] pub struct btf {}
#[repr(C)] pub struct btf_type { pub name_off: u32, pub info: u32, pub size: u32, pub type_: u32 }
#[repr(C)] pub struct btf_member { pub name_off: u32, pub type_: u32, pub offset: u32 }
#[repr(C)] pub struct btf_param { pub name_off: u32, pub type_: u32 }
#[repr(C)] pub struct btf_func_model { pub ret_size: u32, pub arg_flags: [u32; 8] }
#[repr(C)] pub struct bpf_ctx_arg_aux { pub btf_id: u32, pub btf: *mut btf, pub offset: i32, pub reg_type: u32, pub refcounted: bool }
#[repr(C)] pub struct bpf_tramp_nodes { pub nodes: [bpf_tramp_node; 8], pub nr_nodes: u32 }
#[repr(C)] pub struct bpf_tramp_node {}
#[repr(C)] pub struct bpf_tramp_link { pub link: bpf_link, pub node: bpf_tramp_node }
#[repr(C)] pub struct mutex {}
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct atomic64_t { pub value: i64 }
#[repr(C)] pub struct wait_queue_head_t {}
#[repr(C)] pub struct module {}
#[repr(C)] pub struct seq_file {}
#[repr(C)] pub struct file { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct poll_table_struct {}
#[repr(C)] pub union bpf_attr { pub key_size: u32 }

pub unsafe fn bpf_struct_ops_image_alloc() -> *mut core::ffi::c_void {
    if bpf_jit_charge_modmem(4096) != 0 { return ptr::null_mut(); }
    let image = arch_alloc_bpf_trampoline(4096);
    if image.is_null() { bpf_jit_uncharge_modmem(4096); }
    image
}
pub unsafe extern "C" fn bpf_struct_ops_image_free(image: *mut core::ffi::c_void) { if !image.is_null() { arch_free_bpf_trampoline(image, 4096); bpf_jit_uncharge_modmem(4096); } }

/* The remaining routines retain the C ABI and are declared here so dependent
 * translation units can provide the kernel-specific implementations. */
extern "C" {
    fn is_valid_value_type(btf: *mut btf, value_id: i32, ty: *const btf_type, value_name: *const i8) -> bool;
    fn prepare_arg_info(btf: *mut btf, st_ops_name: *const i8, member_name: *const i8, func_proto: *const btf_type, stub_func_addr: *mut core::ffi::c_void, model: *mut btf_func_model, arg_info: *mut bpf_struct_ops_arg_info) -> i32;
    pub fn bpf_struct_ops_supported(st_ops: *const bpf_struct_ops, moff: u32) -> i32;
    pub fn bpf_struct_ops_desc_init(desc: *mut bpf_struct_ops_desc, btf: *mut btf, log: *mut core::ffi::c_void) -> i32;
    pub fn bpf_struct_ops_get(kdata: *const core::ffi::c_void) -> bool;
    pub fn bpf_struct_ops_put(kdata: *const core::ffi::c_void);
    pub fn bpf_struct_ops_id(kdata: *const core::ffi::c_void) -> u32;
    pub fn bpf_struct_ops_for_each_prog(kdata: *const core::ffi::c_void, cb: Option<unsafe extern "C" fn(*mut bpf_prog, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void) -> i32;
    pub fn bpf_prog_assoc_struct_ops(prog: *mut bpf_prog, map: *mut bpf_map) -> i32;
    pub fn bpf_prog_disassoc_struct_ops(prog: *mut bpf_prog);
    pub fn bpf_prog_get_assoc_struct_ops(aux: *const bpf_prog_aux) -> *mut core::ffi::c_void;
    pub fn bpf_map_struct_ops_info_fill(info: *mut core::ffi::c_void, map: *mut bpf_map);
    pub fn bpf_struct_ops_link_create(attr: *mut bpf_attr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
