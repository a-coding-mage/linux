/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018 Facebook */

// Translated from the C header. Types and symbols supplied by other headers
// remain external dependencies.

pub const KF_ACQUIRE: u32 = 1 << 0;
pub const KF_RELEASE: u32 = 1 << 1;
pub const KF_RET_NULL: u32 = 1 << 2;
pub const KF_SLEEPABLE: u32 = 1 << 5;
pub const KF_DESTRUCTIVE: u32 = 1 << 6;
pub const KF_RCU: u32 = 1 << 7;
pub const KF_ITER_NEW: u32 = 1 << 8;
pub const KF_ITER_NEXT: u32 = 1 << 9;
pub const KF_ITER_DESTROY: u32 = 1 << 10;
pub const KF_RCU_PROTECTED: u32 = 1 << 11;
pub const KF_FASTCALL: u32 = 1 << 12;
pub const KF_ARENA_RET: u32 = 1 << 13;
pub const KF_ARENA_ARG1: u32 = 1 << 14;
pub const KF_ARENA_ARG2: u32 = 1 << 15;
pub const KF_IMPLICIT_ARGS: u32 = 1 << 16;
pub const KF_SPINLOCK_SAFE: u32 = 1 << 17;

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut module,
    pub set: *mut btf_id_set8,
    pub filter: Option<unsafe extern "C" fn(*const bpf_prog, u32) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct btf_id_dtor_kfunc { pub btf_id: u32, pub kfunc_btf_id: u32 }

#[repr(C)]
pub struct btf_struct_meta { pub btf_id: u32, pub record: *mut btf_record }

#[repr(C)]
pub struct btf_struct_metas {
    pub cnt: u32,
    pub types: [btf_struct_meta; 0],
}

#[repr(C)]
pub enum btf_field_iter_kind { BTF_FIELD_ITER_IDS, BTF_FIELD_ITER_STRS }

#[repr(C)]
pub struct btf_field_desc {
    pub t_off_cnt: ::core::ffi::c_int,
    pub t_offs: [::core::ffi::c_int; 2],
    pub m_sz: ::core::ffi::c_int,
    pub m_off_cnt: ::core::ffi::c_int,
    pub m_offs: [::core::ffi::c_int; 1],
}

#[repr(C)]
pub struct btf_field_iter {
    pub desc: btf_field_desc,
    pub p: *mut ::core::ffi::c_void,
    pub m_idx: ::core::ffi::c_int,
    pub off_idx: ::core::ffi::c_int,
    pub vlen: ::core::ffi::c_int,
}

extern "C" {
    pub fn btf_get_name(btf: *const btf) -> *const ::core::ffi::c_char;
    pub fn btf_get(btf: *mut btf);
    pub fn btf_put(btf: *mut btf);
    pub fn btf_type_id_size(btf: *const btf, type_id: *mut u32, ret_size: *mut u32) -> *const btf_type;
    pub fn btf_type_seq_show(btf: *const btf, type_id: u32, obj: *mut ::core::ffi::c_void, m: *mut seq_file);
    pub fn btf_get_fd_by_id(id: u32) -> ::core::ffi::c_int;
    pub fn btf_obj_id(btf: *const btf) -> u32;
    pub fn btf_is_kernel(btf: *const btf) -> bool;
    pub fn btf_is_module(btf: *const btf) -> bool;
    pub fn btf_is_vmlinux(btf: *const btf) -> bool;
    pub fn btf_nr_types(btf: *const btf) -> u32;
    pub fn btf_base_btf(btf: *const btf) -> *mut btf;
    pub fn btf_type_is_i32(t: *const btf_type) -> bool;
    pub fn btf_type_is_i64(t: *const btf_type) -> bool;
    pub fn btf_type_is_primitive(t: *const btf_type) -> bool;
    pub fn btf_type_is_void(t: *const btf_type) -> bool;
    pub fn btf_type_vlen(t: *const btf_type) -> u32;
    pub fn btf_type_skip_modifiers(btf: *const btf, id: u32, res_id: *mut u32) -> *const btf_type;
    pub fn btf_type_resolve_ptr(btf: *const btf, id: u32, res_id: *mut u32) -> *const btf_type;
    pub fn btf_type_resolve_func_ptr(btf: *const btf, id: u32, res_id: *mut u32) -> *const btf_type;
    pub fn btf_type_str(t: *const btf_type) -> *const ::core::ffi::c_char;
}

// The following declarations preserve the remaining header API. Their
// concrete layouts and helper macros are supplied by the corresponding UAPI
// and kernel headers.
extern "C" {
    pub static btf_fops: file_operations;
    pub fn btf_header(btf: *const btf) -> *const btf_header;
    pub fn btf_new_fd(attr: *const bpf_attr, uattr: bpfptr_t, attr_log: *mut bpf_log_attr) -> ::core::ffi::c_int;
    pub fn btf_get_by_fd(fd: ::core::ffi::c_int) -> *mut btf;
    pub fn btf_type_name_to_buf(btf: *const btf, type_id: u32, buf: *mut ::core::ffi::c_char, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn btf_param_match_suffix(btf: *const btf, arg: *const btf_param, suffix: *const ::core::ffi::c_char) -> bool;
    pub fn btf_ctx_arg_offset(btf: *const btf, func_proto: *const btf_type, arg_no: u32) -> ::core::ffi::c_int;
    pub fn btf_ctx_arg_idx(btf: *mut btf, func_proto: *const btf_type, off: ::core::ffi::c_int) -> u32;
    pub fn btf_member_is_reg_int(btf: *const btf, s: *const btf_type, m: *const btf_member, expected_offset: u32, expected_size: u32) -> bool;
    pub fn btf_parse_fields(btf: *const btf, t: *const btf_type, field_mask: u32, value_size: u32) -> *mut btf_record;
    pub fn btf_check_and_fixup_fields(btf: *const btf, rec: *mut btf_record) -> ::core::ffi::c_int;
    pub fn btf_find_by_name_kind(btf: *const btf, name: *const ::core::ffi::c_char, kind: u8) -> i32;
    pub fn bpf_find_btf_id(name: *const ::core::ffi::c_char, kind: u32, btf_p: *mut *mut btf) -> i32;
    pub fn btf_get_module_btf(module: *const module) -> *mut btf;
    pub fn btf_relocate_id(btf: *const btf, id: u32) -> u32;
    pub fn btf_resolve_size(btf: *const btf, ty: *const btf_type, type_size: *mut u32) -> *const btf_type;
    pub fn btf_type_by_id(btf: *const btf, type_id: u32) -> *const btf_type;
    pub fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
    pub fn btf_relocate(btf: *mut btf, base_btf: *const btf, map_ids: *mut *mut u32) -> ::core::ffi::c_int;
    pub fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, iter_kind: btf_field_iter_kind) -> ::core::ffi::c_int;
    pub fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut u32;
    pub fn btf_name_by_offset(btf: *const btf, offset: u32) -> *const ::core::ffi::c_char;
    pub fn btf_str_by_offset(btf: *const btf, offset: u32) -> *const ::core::ffi::c_char;
    pub fn btf_parse_vmlinux() -> *mut btf;
    pub fn bpf_prog_get_target_btf(prog: *const bpf_prog) -> *mut btf;
    pub fn btf_kfunc_flags(btf: *const btf, kfunc_btf_id: u32, prog: *const bpf_prog) -> *mut u32;
    pub fn btf_kfunc_check_flag(btf: *const btf, kfunc_btf_id: u32, flag: u32) -> ::core::ffi::c_int;
    pub fn btf_kfunc_is_allowed(btf: *const btf, kfunc_btf_id: u32, prog: *const bpf_prog) -> bool;
    pub fn btf_find_dtor_kfunc(btf: *mut btf, btf_id: u32) -> i32;
    pub fn btf_find_struct_meta(btf: *const btf, btf_id: u32) -> *mut btf_struct_meta;
    pub fn btf_is_projection_of(pname: *const ::core::ffi::c_char, tname: *const ::core::ffi::c_char) -> bool;
    pub fn btf_types_are_same(btf1: *const btf, id1: u32, btf2: *const btf, id2: u32) -> bool;
    pub fn btf_check_iter_arg(btf: *mut btf, func: *const btf_type, arg_idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// C flexible-array and opaque kernel declarations.
#[repr(C)] pub struct btf;
#[repr(C)] pub struct btf_member;
#[repr(C)] pub struct btf_type { pub name_off: u32, pub info: u32, pub size: u32, pub type_: u32 }
#[repr(C)] pub struct btf_array;
#[repr(C)] pub struct btf_enum;
#[repr(C)] pub struct btf_enum64 { pub val_hi32: u32, pub val_lo32: u32 }
#[repr(C)] pub struct btf_var_secinfo;
#[repr(C)] pub struct btf_param;
#[repr(C)] pub struct btf_decl_tag;
#[repr(C)] pub struct btf_record;
#[repr(C)] pub struct btf_id_set8;
#[repr(C)] pub struct module;
#[repr(C)] pub struct bpf_prog;
#[repr(C)] pub struct bpf_attr;
#[repr(C)] pub struct bpf_log_attr;
#[repr(C)] pub struct btf_header;
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct bpfptr_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
