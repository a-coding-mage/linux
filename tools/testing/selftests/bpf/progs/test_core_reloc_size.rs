// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [::core::ffi::c_char; 256],
    pub out: [::core::ffi::c_char; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_size_output {
    pub int_sz: ::core::ffi::c_int,
    pub int_off: ::core::ffi::c_int,
    pub struct_sz: ::core::ffi::c_int,
    pub struct_off: ::core::ffi::c_int,
    pub union_sz: ::core::ffi::c_int,
    pub union_off: ::core::ffi::c_int,
    pub arr_sz: ::core::ffi::c_int,
    pub arr_off: ::core::ffi::c_int,
    pub arr_elem_sz: ::core::ffi::c_int,
    pub arr_elem_off: ::core::ffi::c_int,
    pub ptr_sz: ::core::ffi::c_int,
    pub ptr_off: ::core::ffi::c_int,
    pub enum_sz: ::core::ffi::c_int,
    pub enum_off: ::core::ffi::c_int,
    pub float_sz: ::core::ffi::c_int,
    pub float_off: ::core::ffi::c_int,
}

#[repr(C)]
pub struct core_reloc_size_struct_field {
    pub x: ::core::ffi::c_int,
}

#[repr(C)]
pub union core_reloc_size_union_field {
    pub x: ::core::ffi::c_int,
}

pub const VALUE: ::core::ffi::c_uint = 123;

#[repr(C)]
pub struct core_reloc_size {
    pub int_field: ::core::ffi::c_int,
    pub struct_field: core_reloc_size_struct_field,
    pub union_field: core_reloc_size_union_field,
    pub arr_field: [::core::ffi::c_int; 4],
    pub ptr_field: *mut ::core::ffi::c_void,
    pub enum_field: ::core::ffi::c_uint,
    pub float_field: f32,
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_size(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let in_: *mut core_reloc_size = ::core::ptr::addr_of_mut!(data.in_) as *mut core_reloc_size;
    let out: *mut core_reloc_size_output =
        ::core::ptr::addr_of_mut!(data.out) as *mut core_reloc_size_output;

    (*out).int_sz = ::core::mem::size_of_val(&(*in_).int_field) as ::core::ffi::c_int;
    (*out).int_off = ::core::mem::offset_of!(core_reloc_size, int_field) as ::core::ffi::c_int;

    (*out).struct_sz = ::core::mem::size_of_val(&(*in_).struct_field) as ::core::ffi::c_int;
    (*out).struct_off =
        ::core::mem::offset_of!(core_reloc_size, struct_field) as ::core::ffi::c_int;

    (*out).union_sz = ::core::mem::size_of_val(&(*in_).union_field) as ::core::ffi::c_int;
    (*out).union_off =
        ::core::mem::offset_of!(core_reloc_size, union_field) as ::core::ffi::c_int;

    (*out).arr_sz = ::core::mem::size_of_val(&(*in_).arr_field) as ::core::ffi::c_int;
    (*out).arr_off = ::core::mem::offset_of!(core_reloc_size, arr_field) as ::core::ffi::c_int;

    (*out).arr_elem_sz =
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int;
    (*out).arr_elem_off = (::core::mem::offset_of!(core_reloc_size, arr_field)
        + ::core::mem::size_of::<::core::ffi::c_int>())
        as ::core::ffi::c_int;

    (*out).ptr_sz = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_int;
    (*out).ptr_off = ::core::mem::offset_of!(core_reloc_size, ptr_field) as ::core::ffi::c_int;

    (*out).enum_sz = ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int;
    (*out).enum_off = ::core::mem::offset_of!(core_reloc_size, enum_field) as ::core::ffi::c_int;

    (*out).float_sz = ::core::mem::size_of::<f32>() as ::core::ffi::c_int;
    (*out).float_off = ::core::mem::offset_of!(core_reloc_size, float_field) as ::core::ffi::c_int;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
