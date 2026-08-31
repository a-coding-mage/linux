// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies translated as external intent:
// <linux/bpf.h>, <stdint.h>, <stdbool.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

use core::ffi::c_void;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [u8; 256],
    pub out: [u8; 256],
    pub skip: bool,
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
};

#[repr(C)]
pub struct a_struct {
    pub x: i32,
}

#[repr(C)]
pub union a_complex_struct_x {
    pub a: *mut a_struct,
    pub b: *mut c_void,
}

#[repr(C)]
pub struct a_complex_struct {
    pub x: a_complex_struct_x,
    pub y: core::ffi::c_long,
}

#[repr(C)]
pub union a_union {
    pub y: i32,
    pub z: i32,
}

pub type named_struct_typedef = a_struct;

#[repr(C)]
pub struct anon_struct_typedef {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
pub struct struct_ptr_typedef_pointee {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

pub type struct_ptr_typedef = *mut struct_ptr_typedef_pointee;

#[repr(C)]
pub enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
}

pub type int_typedef = i32;

#[repr(C)]
pub enum enum_typedef {
    TYPEDEF_ENUM_VAL1 = 0,
    TYPEDEF_ENUM_VAL2 = 1,
}

pub type void_ptr_typedef = *mut c_void;
pub type restrict_ptr_typedef = *mut i32;

pub type func_proto_typedef = Option<unsafe extern "C" fn(core::ffi::c_long) -> i32>;

pub type arr_typedef = [u8; 20];

#[repr(C)]
pub struct core_reloc_type_based_output {
    pub struct_exists: bool,
    pub complex_struct_exists: bool,
    pub union_exists: bool,
    pub enum_exists: bool,
    pub typedef_named_struct_exists: bool,
    pub typedef_anon_struct_exists: bool,
    pub typedef_struct_ptr_exists: bool,
    pub typedef_int_exists: bool,
    pub typedef_enum_exists: bool,
    pub typedef_void_ptr_exists: bool,
    pub typedef_restrict_ptr_exists: bool,
    pub typedef_func_proto_exists: bool,
    pub typedef_arr_exists: bool,

    pub struct_matches: bool,
    pub complex_struct_matches: bool,
    pub union_matches: bool,
    pub enum_matches: bool,
    pub typedef_named_struct_matches: bool,
    pub typedef_anon_struct_matches: bool,
    pub typedef_struct_ptr_matches: bool,
    pub typedef_int_matches: bool,
    pub typedef_enum_matches: bool,
    pub typedef_void_ptr_matches: bool,
    pub typedef_restrict_ptr_matches: bool,
    pub typedef_func_proto_matches: bool,
    pub typedef_arr_matches: bool,

    pub struct_sz: i32,
    pub union_sz: i32,
    pub enum_sz: i32,
    pub typedef_named_struct_sz: i32,
    pub typedef_anon_struct_sz: i32,
    pub typedef_struct_ptr_sz: i32,
    pub typedef_int_sz: i32,
    pub typedef_enum_sz: i32,
    pub typedef_void_ptr_sz: i32,
    pub typedef_func_proto_sz: i32,
    pub typedef_arr_sz: i32,
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_type_based(_ctx: *mut c_void) -> i32 {
    /* Support for the BPF_TYPE_MATCHES argument to the
     * __builtin_preserve_type_info builtin was added at some point during
     * development of clang 15 and it's what we require for this test. Part of it
     * could run with merely __builtin_preserve_type_info (which could be checked
     * separately), but we have to find an upper bound.
     */
    // C condition preserved:
    // #if __has_builtin(__builtin_preserve_type_info) && __clang_major__ >= 15
    {
        let out = (&raw mut data.out).cast::<core_reloc_type_based_output>();

        (*out).struct_exists = bpf_core_type_exists!(a_struct);
        (*out).complex_struct_exists = bpf_core_type_exists!(a_complex_struct);
        (*out).union_exists = bpf_core_type_exists!(a_union);
        (*out).enum_exists = bpf_core_type_exists!(an_enum);
        (*out).typedef_named_struct_exists = bpf_core_type_exists!(named_struct_typedef);
        (*out).typedef_anon_struct_exists = bpf_core_type_exists!(anon_struct_typedef);
        (*out).typedef_struct_ptr_exists = bpf_core_type_exists!(struct_ptr_typedef);
        (*out).typedef_int_exists = bpf_core_type_exists!(int_typedef);
        (*out).typedef_enum_exists = bpf_core_type_exists!(enum_typedef);
        (*out).typedef_void_ptr_exists = bpf_core_type_exists!(void_ptr_typedef);
        (*out).typedef_restrict_ptr_exists = bpf_core_type_exists!(restrict_ptr_typedef);
        (*out).typedef_func_proto_exists = bpf_core_type_exists!(func_proto_typedef);
        (*out).typedef_arr_exists = bpf_core_type_exists!(arr_typedef);

        (*out).struct_matches = bpf_core_type_matches!(a_struct);
        (*out).complex_struct_matches = bpf_core_type_matches!(a_complex_struct);
        (*out).union_matches = bpf_core_type_matches!(a_union);
        (*out).enum_matches = bpf_core_type_matches!(an_enum);
        (*out).typedef_named_struct_matches = bpf_core_type_matches!(named_struct_typedef);
        (*out).typedef_anon_struct_matches = bpf_core_type_matches!(anon_struct_typedef);
        (*out).typedef_struct_ptr_matches = bpf_core_type_matches!(struct_ptr_typedef);
        (*out).typedef_int_matches = bpf_core_type_matches!(int_typedef);
        (*out).typedef_enum_matches = bpf_core_type_matches!(enum_typedef);
        (*out).typedef_void_ptr_matches = bpf_core_type_matches!(void_ptr_typedef);
        (*out).typedef_restrict_ptr_matches = bpf_core_type_matches!(restrict_ptr_typedef);
        (*out).typedef_func_proto_matches = bpf_core_type_matches!(func_proto_typedef);
        (*out).typedef_arr_matches = bpf_core_type_matches!(arr_typedef);

        (*out).struct_sz = bpf_core_type_size!(a_struct);
        (*out).union_sz = bpf_core_type_size!(a_union);
        (*out).enum_sz = bpf_core_type_size!(an_enum);
        (*out).typedef_named_struct_sz = bpf_core_type_size!(named_struct_typedef);
        (*out).typedef_anon_struct_sz = bpf_core_type_size!(anon_struct_typedef);
        (*out).typedef_struct_ptr_sz = bpf_core_type_size!(struct_ptr_typedef);
        (*out).typedef_int_sz = bpf_core_type_size!(int_typedef);
        (*out).typedef_enum_sz = bpf_core_type_size!(enum_typedef);
        (*out).typedef_void_ptr_sz = bpf_core_type_size!(void_ptr_typedef);
        (*out).typedef_func_proto_sz = bpf_core_type_size!(func_proto_typedef);
        (*out).typedef_arr_sz = bpf_core_type_size!(arr_typedef);
    }
    // #else
    // data.skip = true;
    // #endif
    0
}
