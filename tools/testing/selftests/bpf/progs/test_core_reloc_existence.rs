// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// Dependencies from the original C source:
// linux/bpf.h, stdint.h, bpf/bpf_helpers.h, bpf/bpf_core_read.h

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct data_t {
    pub in_: [::core::ffi::c_char; 256],
    pub out: [::core::ffi::c_char; 256],
}

#[no_mangle]
pub static mut data: data_t = data_t {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_existence_output {
    pub a_exists: ::core::ffi::c_int,
    pub a_value: ::core::ffi::c_int,
    pub b_exists: ::core::ffi::c_int,
    pub b_value: ::core::ffi::c_int,
    pub c_exists: ::core::ffi::c_int,
    pub c_value: ::core::ffi::c_int,
    pub arr_exists: ::core::ffi::c_int,
    pub arr_value: ::core::ffi::c_int,
    pub s_exists: ::core::ffi::c_int,
    pub s_value: ::core::ffi::c_int,
}

#[repr(C)]
pub struct core_reloc_existence_s {
    pub x: ::core::ffi::c_int,
}

#[repr(C)]
pub struct core_reloc_existence {
    pub s: core_reloc_existence_s,
    pub arr: [::core::ffi::c_int; 1],
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_existence(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let in_: *mut core_reloc_existence = (&raw mut data.in_).cast::<core_reloc_existence>();
    let out: *mut core_reloc_existence_output =
        (&raw mut data.out).cast::<core_reloc_existence_output>();

    (*out).a_exists = bpf_core_field_exists!((*in_).a);
    if bpf_core_field_exists!(core_reloc_existence, a) != 0 {
        (*out).a_value = BPF_CORE_READ!(in_, a);
    } else {
        (*out).a_value = 0xff000001u32 as ::core::ffi::c_int;
    }

    (*out).b_exists = bpf_core_field_exists!((*in_).b);
    if bpf_core_field_exists!(core_reloc_existence, b) != 0 {
        (*out).b_value = BPF_CORE_READ!(in_, b);
    } else {
        (*out).b_value = 0xff000002u32 as ::core::ffi::c_int;
    }

    (*out).c_exists = bpf_core_field_exists!((*in_).c);
    if bpf_core_field_exists!(core_reloc_existence, c) != 0 {
        (*out).c_value = BPF_CORE_READ!(in_, c);
    } else {
        (*out).c_value = 0xff000003u32 as ::core::ffi::c_int;
    }

    (*out).arr_exists = bpf_core_field_exists!((*in_).arr);
    if bpf_core_field_exists!(core_reloc_existence, arr) != 0 {
        (*out).arr_value = BPF_CORE_READ!(in_, arr[0]);
    } else {
        (*out).arr_value = 0xff000004u32 as ::core::ffi::c_int;
    }

    (*out).s_exists = bpf_core_field_exists!((*in_).s);
    if bpf_core_field_exists!(core_reloc_existence, s) != 0 {
        (*out).s_value = BPF_CORE_READ!(in_, s.x);
    } else {
        (*out).s_value = 0xff000005u32 as ::core::ffi::c_int;
    }

    let _ = ctx;
    0
}
