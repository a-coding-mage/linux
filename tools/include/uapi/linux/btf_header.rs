/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2018 Facebook */

/* C source included <linux/types.h> for __u8, __u16, __u32, and __s32. */

pub const BTF_MAGIC: u32 = 0xeB9F;
pub const BTF_VERSION: u32 = 1;

/*
 * BTF layout section consists of a struct btf_layout for each known
 * kind at BTF encoding time.
 */
#[repr(C)]
pub struct btf_layout {
    pub info_sz: __u8,  /* size of singular element after btf_type */
    pub elem_sz: __u8,  /* size of each of btf_vlen(t) elements */
    pub flags: __u16,   /* currently unused */
}

#[repr(C)]
pub struct btf_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,

    /* All offsets are in bytes relative to the end of this header */
    pub type_off: __u32,    /* offset of type section */
    pub type_len: __u32,    /* length of type section */
    pub str_off: __u32,     /* offset of string section */
    pub str_len: __u32,     /* length of string section */
    pub layout_off: __u32,  /* offset of layout section */
    pub layout_len: __u32,  /* length of layout section */
}

#[repr(C)]
pub enum btf_max {
    /* Max possible kind */
    BTF_MAX_KIND = 0x0000007f,
    /* Max # of type identifier */
    BTF_MAX_TYPE = 0x000fffff,
    /* Max offset into the string section */
    BTF_MAX_NAME_OFFSET = 0x00ffffff,
    /* Max # of struct/union/enum members or func args */
    BTF_MAX_VLEN = 0x00ffffff,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    /*
     * "info" bits arrangement
     * bits  0-23: vlen (e.g. # of struct's members)
     * bits 24-30: kind (e.g. int, ptr, array...etc)
     * bit     31: kind_flag, currently used by
     *             struct, union, enum, fwd, enum64,
     *             decl_tag and type_tag
     */
    pub info: __u32,
    /*
     * "size" is used by INT, ENUM, STRUCT, UNION, DATASEC and ENUM64.
     * "size" tells the size of the type it is describing.
     *
     * "type" is used by PTR, TYPEDEF, VOLATILE, CONST, RESTRICT,
     * FUNC, FUNC_PROTO, VAR, DECL_TAG and TYPE_TAG.
     * "type" is a type_id referring to another type.
     */
    pub __bindgen_anon_1: btf_type__bindgen_ty_1,
}

#[repr(C)]
pub union btf_type__bindgen_ty_1 {
    pub size: __u32,
    pub type_: __u32,
}

pub const fn BTF_INFO_KIND(info: __u32) -> __u32 {
    (info >> 24) & 0x7f
}

pub const fn BTF_INFO_VLEN(info: __u32) -> __u32 {
    info & 0xffffff
}

pub const fn BTF_INFO_KFLAG(info: __u32) -> __u32 {
    info >> 31
}

pub const BTF_KIND_UNKN: u32 = 0;       /* Unknown */
pub const BTF_KIND_INT: u32 = 1;        /* Integer */
pub const BTF_KIND_PTR: u32 = 2;        /* Pointer */
pub const BTF_KIND_ARRAY: u32 = 3;      /* Array */
pub const BTF_KIND_STRUCT: u32 = 4;     /* Struct */
pub const BTF_KIND_UNION: u32 = 5;      /* Union */
pub const BTF_KIND_ENUM: u32 = 6;       /* Enumeration up to 32-bit values */
pub const BTF_KIND_FWD: u32 = 7;        /* Forward */
pub const BTF_KIND_TYPEDEF: u32 = 8;    /* Typedef */
pub const BTF_KIND_VOLATILE: u32 = 9;   /* Volatile */
pub const BTF_KIND_CONST: u32 = 10;     /* Const */
pub const BTF_KIND_RESTRICT: u32 = 11;  /* Restrict */
pub const BTF_KIND_FUNC: u32 = 12;      /* Function */
pub const BTF_KIND_FUNC_PROTO: u32 = 13; /* Function Proto */
pub const BTF_KIND_VAR: u32 = 14;       /* Variable */
pub const BTF_KIND_DATASEC: u32 = 15;   /* Section */
pub const BTF_KIND_FLOAT: u32 = 16;     /* Floating point */
pub const BTF_KIND_DECL_TAG: u32 = 17;  /* Decl Tag */
pub const BTF_KIND_TYPE_TAG: u32 = 18;  /* Type Tag */
pub const BTF_KIND_ENUM64: u32 = 19;    /* Enumeration up to 64-bit values */

pub const NR_BTF_KINDS: u32 = 20;
pub const BTF_KIND_MAX: u32 = NR_BTF_KINDS - 1;

/*
 * For some specific BTF_KIND, "struct btf_type" is immediately
 * followed by extra data.
 */

/*
 * BTF_KIND_INT is followed by a u32 and the following
 * is the 32 bits arrangement:
 */
pub const fn BTF_INT_ENCODING(VAL: __u32) -> __u32 {
    (VAL & 0x0f000000) >> 24
}

pub const fn BTF_INT_OFFSET(VAL: __u32) -> __u32 {
    (VAL & 0x00ff0000) >> 16
}

pub const fn BTF_INT_BITS(VAL: __u32) -> __u32 {
    VAL & 0x000000ff
}

/* Attributes stored in the BTF_INT_ENCODING */
pub const BTF_INT_SIGNED: u32 = 1 << 0;
pub const BTF_INT_CHAR: u32 = 1 << 1;
pub const BTF_INT_BOOL: u32 = 1 << 2;

/*
 * BTF_KIND_ENUM is followed by multiple "struct btf_enum".
 * The exact number of btf_enum is stored in the vlen (of the
 * info in "struct btf_type").
 */
#[repr(C)]
pub struct btf_enum {
    pub name_off: __u32,
    pub val: __s32,
}

/* BTF_KIND_ARRAY is followed by one "struct btf_array" */
#[repr(C)]
pub struct btf_array {
    pub type_: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

/*
 * BTF_KIND_STRUCT and BTF_KIND_UNION are followed
 * by multiple "struct btf_member".  The exact number
 * of btf_member is stored in the vlen (of the info in
 * "struct btf_type").
 */
#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    /*
     * If the type info kind_flag is set, the btf_member offset
     * contains both member bitfield size and bit offset. The
     * bitfield size is set for bitfield members. If the type
     * info kind_flag is not set, the offset contains only bit
     * offset.
     */
    pub offset: __u32,
}

/*
 * If the struct/union type info kind_flag is set, the
 * following two macros are used to access bitfield_size
 * and bit_offset from btf_member.offset.
 */
pub const fn BTF_MEMBER_BITFIELD_SIZE(val: __u32) -> __u32 {
    val >> 24
}

pub const fn BTF_MEMBER_BIT_OFFSET(val: __u32) -> __u32 {
    val & 0xffffff
}

/*
 * BTF_KIND_FUNC_PROTO is followed by multiple "struct btf_param".
 * The exact number of btf_param is stored in the vlen (of the
 * info in "struct btf_type").
 */
#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

pub const BTF_VAR_STATIC: u32 = 0;
pub const BTF_VAR_GLOBAL_ALLOCATED: u32 = 1;
pub const BTF_VAR_GLOBAL_EXTERN: u32 = 2;

#[repr(C)]
pub enum btf_func_linkage {
    BTF_FUNC_STATIC = 0,
    BTF_FUNC_GLOBAL = 1,
    BTF_FUNC_EXTERN = 2,
}

/*
 * BTF_KIND_VAR is followed by a single "struct btf_var" to describe
 * additional information related to the variable such as its linkage.
 */
#[repr(C)]
pub struct btf_var {
    pub linkage: __u32,
}

/*
 * BTF_KIND_DATASEC is followed by multiple "struct btf_var_secinfo"
 * to describe all BTF_KIND_VAR types it contains along with it's
 * in-section offset as well as size.
 */
#[repr(C)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

/*
 * BTF_KIND_DECL_TAG is followed by a single "struct btf_decl_tag" to describe
 * additional information related to the tag applied location.
 * If component_idx == -1, the tag is applied to a struct, union,
 * variable or function. Otherwise, it is applied to a struct/union
 * member or a func argument, and component_idx indicates which member
 * or argument (0 ... vlen-1).
 */
#[repr(C)]
pub struct btf_decl_tag {
    pub component_idx: __s32,
}

/*
 * BTF_KIND_ENUM64 is followed by multiple "struct btf_enum64".
 * The exact number of btf_enum64 is stored in the vlen (of the
 * info in "struct btf_type").
 */
#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
}
