// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependency intent:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_testmod_struct_arg_4 {
    pub a: u64,
    pub b: i32,
}

#[repr(C)]
pub struct bpf_testmod_struct_arg_5 {
    pub a: i8,
    pub b: i16,
    pub c: i32,
    pub d: i64,
}

#[no_mangle]
pub static mut t7_a: i64 = 0;
#[no_mangle]
pub static mut t7_b: i64 = 0;
#[no_mangle]
pub static mut t7_c: i64 = 0;
#[no_mangle]
pub static mut t7_d: i64 = 0;
#[no_mangle]
pub static mut t7_e: i64 = 0;
#[no_mangle]
pub static mut t7_f_a: i64 = 0;
#[no_mangle]
pub static mut t7_f_b: i64 = 0;
#[no_mangle]
pub static mut t7_ret: i64 = 0;

#[no_mangle]
pub static mut t8_a: i64 = 0;
#[no_mangle]
pub static mut t8_b: i64 = 0;
#[no_mangle]
pub static mut t8_c: i64 = 0;
#[no_mangle]
pub static mut t8_d: i64 = 0;
#[no_mangle]
pub static mut t8_e: i64 = 0;
#[no_mangle]
pub static mut t8_f_a: i64 = 0;
#[no_mangle]
pub static mut t8_f_b: i64 = 0;
#[no_mangle]
pub static mut t8_g: i64 = 0;
#[no_mangle]
pub static mut t8_ret: i64 = 0;

#[no_mangle]
pub static mut t9_a: i64 = 0;
#[no_mangle]
pub static mut t9_b: i64 = 0;
#[no_mangle]
pub static mut t9_c: i64 = 0;
#[no_mangle]
pub static mut t9_d: i64 = 0;
#[no_mangle]
pub static mut t9_e: i64 = 0;
#[no_mangle]
pub static mut t9_f: i64 = 0;
#[no_mangle]
pub static mut t9_g: i64 = 0;
#[no_mangle]
pub static mut t9_h_a: i64 = 0;
#[no_mangle]
pub static mut t9_h_b: i64 = 0;
#[no_mangle]
pub static mut t9_h_c: i64 = 0;
#[no_mangle]
pub static mut t9_h_d: i64 = 0;
#[no_mangle]
pub static mut t9_i: i64 = 0;
#[no_mangle]
pub static mut t9_ret: i64 = 0;

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_7"]
pub unsafe extern "C" fn test_struct_many_args_1(
    a: u64,
    b: *mut core::ffi::c_void,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: bpf_testmod_struct_arg_4,
) -> i32 {
    t7_a = a as i64;
    t7_b = b as i64;
    t7_c = c as i64;
    t7_d = d as i64;
    t7_e = e as i64;
    t7_f_a = f.a as i64;
    t7_f_b = f.b as i64;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_7"]
pub unsafe extern "C" fn test_struct_many_args_2(
    _a: u64,
    _b: *mut core::ffi::c_void,
    _c: i16,
    _d: i32,
    _e: *mut core::ffi::c_void,
    _f: bpf_testmod_struct_arg_4,
    ret: i32,
) -> i32 {
    t7_ret = ret as i64;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_8"]
pub unsafe extern "C" fn test_struct_many_args_3(
    a: u64,
    b: *mut core::ffi::c_void,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: bpf_testmod_struct_arg_4,
    g: i32,
) -> i32 {
    t8_a = a as i64;
    t8_b = b as i64;
    t8_c = c as i64;
    t8_d = d as i64;
    t8_e = e as i64;
    t8_f_a = f.a as i64;
    t8_f_b = f.b as i64;
    t8_g = g as i64;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_8"]
pub unsafe extern "C" fn test_struct_many_args_4(
    _a: u64,
    _b: *mut core::ffi::c_void,
    _c: i16,
    _d: i32,
    _e: *mut core::ffi::c_void,
    _f: bpf_testmod_struct_arg_4,
    _g: i32,
    ret: i32,
) -> i32 {
    t8_ret = ret as i64;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_9"]
pub unsafe extern "C" fn test_struct_many_args_5(
    a: u64,
    b: *mut core::ffi::c_void,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i16,
    h: bpf_testmod_struct_arg_5,
    i: i64,
) -> i32 {
    t9_a = a as i64;
    t9_b = b as i64;
    t9_c = c as i64;
    t9_d = d as i64;
    t9_e = e as i64;
    t9_f = f as i64;
    t9_g = g as i64;
    t9_h_a = h.a as i64;
    t9_h_b = h.b as i64;
    t9_h_c = h.c as i64;
    t9_h_d = h.d;
    t9_i = i;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_9"]
pub unsafe extern "C" fn test_struct_many_args_6(
    _a: u64,
    _b: *mut core::ffi::c_void,
    _c: i16,
    _d: i32,
    _e: *mut core::ffi::c_void,
    _f: i8,
    _g: i16,
    _h: bpf_testmod_struct_arg_5,
    _i: i64,
    ret: i32,
) -> i32 {
    t9_ret = ret as i64;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
