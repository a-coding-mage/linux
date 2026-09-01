// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <stdbool.h>,
// <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>

use core::ffi::c_void;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [i8; 256],
    pub out: [i8; 256],
    pub skip: bool,
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
};

/* some types are shared with test_core_reloc_type_based.c */
#[repr(C)]
pub struct a_struct {
    pub x: i32,
}

#[repr(C)]
pub union a_union {
    pub y: i32,
    pub z: i32,
}

#[repr(i32)]
pub enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
}

pub type named_struct_typedef = a_struct;

pub type func_proto_typedef = Option<unsafe extern "C" fn(i64) -> i32>;

pub type arr_typedef = [i8; 20];

#[repr(C)]
pub struct core_reloc_type_id_output {
    pub local_anon_struct: i32,
    pub local_anon_union: i32,
    pub local_anon_enum: i32,
    pub local_anon_func_proto_ptr: i32,
    pub local_anon_void_ptr: i32,
    pub local_anon_arr: i32,

    pub local_struct: i32,
    pub local_union: i32,
    pub local_enum: i32,
    pub local_int: i32,
    pub local_struct_typedef: i32,
    pub local_func_proto_typedef: i32,
    pub local_arr_typedef: i32,

    pub targ_struct: i32,
    pub targ_union: i32,
    pub targ_enum: i32,
    pub targ_int: i32,
    pub targ_struct_typedef: i32,
    pub targ_func_proto_typedef: i32,
    pub targ_arr_typedef: i32,
}

/* preserve types even if Clang doesn't support built-in */
#[no_mangle]
pub static mut t1: a_struct = a_struct { x: 0 };
#[no_mangle]
pub static mut t2: a_union = a_union { y: 0 };
#[no_mangle]
pub static mut t3: i32 = 0;
#[no_mangle]
pub static mut t4: named_struct_typedef = named_struct_typedef { x: 0 };
#[no_mangle]
pub static mut t5: func_proto_typedef = None;
#[no_mangle]
pub static mut t6: arr_typedef = [0; 20];

#[repr(C)]
pub struct local_anon_struct {
    pub marker_field: i32,
}

#[repr(C)]
pub union local_anon_union {
    pub marker_field: i32,
}

#[repr(i32)]
pub enum local_anon_enum {
    MARKER_ENUM_VAL = 123,
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_type_id(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* We use __builtin_btf_type_id() in this tests, but up until the time
     * __builtin_preserve_type_info() was added it contained a bug that
     * would make this test fail. The bug was fixed ([0]) with addition of
     * __builtin_preserve_type_info(), though, so that's what we are using
     * to detect whether this test has to be executed, however strange
     * that might look like.
     *
     *   [0] https://github.com/llvm/llvm-project/commit/00602ee7ef0bf6c68d690a2bd729c12b95c95c99
     */
    // C condition: #if __has_builtin(__builtin_preserve_type_info)
    #[cfg(has_builtin_preserve_type_info)]
    {
        let out = (&mut data.out as *mut [i8; 256]).cast::<core_reloc_type_id_output>();

        (*out).local_anon_struct = bpf_core_type_id_local!(local_anon_struct);
        (*out).local_anon_union = bpf_core_type_id_local!(local_anon_union);
        (*out).local_anon_enum = bpf_core_type_id_local!(local_anon_enum);
        (*out).local_anon_func_proto_ptr =
            bpf_core_type_id_local!(Option<unsafe extern "C" fn(i32) -> bool>);
        (*out).local_anon_void_ptr = bpf_core_type_id_local!(*mut c_void);
        (*out).local_anon_arr = bpf_core_type_id_local!([bool; 47]);

        (*out).local_struct = bpf_core_type_id_local!(a_struct);
        (*out).local_union = bpf_core_type_id_local!(a_union);
        (*out).local_enum = bpf_core_type_id_local!(an_enum);
        (*out).local_int = bpf_core_type_id_local!(i32);
        (*out).local_struct_typedef = bpf_core_type_id_local!(named_struct_typedef);
        (*out).local_func_proto_typedef = bpf_core_type_id_local!(func_proto_typedef);
        (*out).local_arr_typedef = bpf_core_type_id_local!(arr_typedef);

        (*out).targ_struct = bpf_core_type_id_kernel!(a_struct);
        (*out).targ_union = bpf_core_type_id_kernel!(a_union);
        (*out).targ_enum = bpf_core_type_id_kernel!(an_enum);
        (*out).targ_int = bpf_core_type_id_kernel!(i32);
        (*out).targ_struct_typedef = bpf_core_type_id_kernel!(named_struct_typedef);
        (*out).targ_func_proto_typedef = bpf_core_type_id_kernel!(func_proto_typedef);
        (*out).targ_arr_typedef = bpf_core_type_id_kernel!(arr_typedef);
    }

    // C fallback: #else
    #[cfg(not(has_builtin_preserve_type_info))]
    {
        data.skip = true;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
