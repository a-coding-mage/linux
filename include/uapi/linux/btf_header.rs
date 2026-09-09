/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2018 Facebook */

pub const BTF_MAGIC: u16 = 0xeB9F;
pub const BTF_VERSION: u8 = 1;

/*
 * BTF layout section consists of a struct btf_layout for each known
 * kind at BTF encoding time.
 */
#[repr(C)]
pub struct btf_layout {
    pub info_sz: u8,
    pub elem_sz: u8,
    pub flags: u16,
}

#[repr(C)]
pub struct btf_header {
    pub magic: u16,
    pub version: u8,
    pub flags: u8,
    pub hdr_len: u32,
    /* All offsets are in bytes relative to the end of this header */
    pub type_off: u32,
    pub type_len: u32,
    pub str_off: u32,
    pub str_len: u32,
    pub layout_off: u32,
    pub layout_len: u32,
}

#[repr(u32)]
pub enum btf_max {
    BTF_MAX_KIND = 0x0000007f,
    BTF_MAX_TYPE = 0x000fffff,
    BTF_MAX_NAME_OFFSET = 0x00ffffff,
    BTF_MAX_VLEN = 0x00ffffff,
}

#[repr(C)]
pub union btf_type__bindgen_ty_1 {
    pub size: u32,
    pub type_: u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: u32,
    pub info: u32,
    pub __bindgen_anon_1: btf_type__bindgen_ty_1,
}

#[macro_export]
macro_rules! BTF_INFO_KIND { ($info:expr) => { (($info >> 24) & 0x7f) }; }
#[macro_export]
macro_rules! BTF_INFO_VLEN { ($info:expr) => { ($info & 0xffffff) }; }
#[macro_export]
macro_rules! BTF_INFO_KFLAG { ($info:expr) => { ($info >> 31) }; }

pub const BTF_KIND_UNKN: u32 = 0;
pub const BTF_KIND_INT: u32 = 1;
pub const BTF_KIND_PTR: u32 = 2;
pub const BTF_KIND_ARRAY: u32 = 3;
pub const BTF_KIND_STRUCT: u32 = 4;
pub const BTF_KIND_UNION: u32 = 5;
pub const BTF_KIND_ENUM: u32 = 6;
pub const BTF_KIND_FWD: u32 = 7;
pub const BTF_KIND_TYPEDEF: u32 = 8;
pub const BTF_KIND_VOLATILE: u32 = 9;
pub const BTF_KIND_CONST: u32 = 10;
pub const BTF_KIND_RESTRICT: u32 = 11;
pub const BTF_KIND_FUNC: u32 = 12;
pub const BTF_KIND_FUNC_PROTO: u32 = 13;
pub const BTF_KIND_VAR: u32 = 14;
pub const BTF_KIND_DATASEC: u32 = 15;
pub const BTF_KIND_FLOAT: u32 = 16;
pub const BTF_KIND_DECL_TAG: u32 = 17;
pub const BTF_KIND_TYPE_TAG: u32 = 18;
pub const BTF_KIND_ENUM64: u32 = 19;
pub const NR_BTF_KINDS: u32 = 20;
pub const BTF_KIND_MAX: u32 = NR_BTF_KINDS - 1;

#[macro_export]
macro_rules! BTF_INT_ENCODING { ($val:expr) => { (($val & 0x0f000000) >> 24) }; }
#[macro_export]
macro_rules! BTF_INT_OFFSET { ($val:expr) => { (($val & 0x00ff0000) >> 16) }; }
#[macro_export]
macro_rules! BTF_INT_BITS { ($val:expr) => { ($val & 0x000000ff) }; }

pub const BTF_INT_SIGNED: u32 = 1 << 0;
pub const BTF_INT_CHAR: u32 = 1 << 1;
pub const BTF_INT_BOOL: u32 = 1 << 2;

#[repr(C)]
pub struct btf_enum { pub name_off: u32, pub val: i32 }

#[repr(C)]
pub struct btf_array { pub type_: u32, pub index_type: u32, pub nelems: u32 }

#[repr(C)]
pub struct btf_member { pub name_off: u32, pub type_: u32, pub offset: u32 }

#[macro_export]
macro_rules! BTF_MEMBER_BITFIELD_SIZE { ($val:expr) => { ($val >> 24) }; }
#[macro_export]
macro_rules! BTF_MEMBER_BIT_OFFSET { ($val:expr) => { ($val & 0xffffff) }; }

#[repr(C)]
pub struct btf_param { pub name_off: u32, pub type_: u32 }

pub const BTF_VAR_STATIC: u32 = 0;
pub const BTF_VAR_GLOBAL_ALLOCATED: u32 = 1;
pub const BTF_VAR_GLOBAL_EXTERN: u32 = 2;

#[repr(u32)]
pub enum btf_func_linkage { BTF_FUNC_STATIC = 0, BTF_FUNC_GLOBAL = 1, BTF_FUNC_EXTERN = 2 }

#[repr(C)]
pub struct btf_var { pub linkage: u32 }

#[repr(C)]
pub struct btf_var_secinfo { pub type_: u32, pub offset: u32, pub size: u32 }

#[repr(C)]
pub struct btf_decl_tag { pub component_idx: i32 }

#[repr(C)]
pub struct btf_enum64 { pub name_off: u32, pub val_lo32: u32, pub val_hi32: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
