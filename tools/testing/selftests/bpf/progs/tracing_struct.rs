// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies preserved from:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_short, c_void};

type __u64 = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_struct_arg_1 {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_struct_arg_2 {
    pub a: c_long,
    pub b: c_long,
}

#[repr(C)]
pub struct bpf_testmod_struct_arg_3 {
    pub a: c_int,
    pub b: [c_int; 0],
}

#[repr(C)]
pub union bpf_testmod_union_arg_1 {
    pub a: c_char,
    pub b: c_short,
    pub arg: bpf_testmod_struct_arg_1,
}

#[repr(C)]
pub union bpf_testmod_union_arg_2 {
    pub a: c_int,
    pub b: c_long,
    pub arg: bpf_testmod_struct_arg_2,
}

pub static mut t1_a_a: c_long = 0;
pub static mut t1_a_b: c_long = 0;
pub static mut t1_b: c_long = 0;
pub static mut t1_c: c_long = 0;
pub static mut t1_ret: c_long = 0;
pub static mut t1_nregs: c_long = 0;
pub static mut t1_reg0: __u64 = 0;
pub static mut t1_reg1: __u64 = 0;
pub static mut t1_reg2: __u64 = 0;
pub static mut t1_reg3: __u64 = 0;
pub static mut t2_a: c_long = 0;
pub static mut t2_b_a: c_long = 0;
pub static mut t2_b_b: c_long = 0;
pub static mut t2_c: c_long = 0;
pub static mut t2_ret: c_long = 0;
pub static mut t3_a: c_long = 0;
pub static mut t3_b: c_long = 0;
pub static mut t3_c_a: c_long = 0;
pub static mut t3_c_b: c_long = 0;
pub static mut t3_ret: c_long = 0;
pub static mut t4_a_a: c_long = 0;
pub static mut t4_b: c_long = 0;
pub static mut t4_c: c_long = 0;
pub static mut t4_d: c_long = 0;
pub static mut t4_e_a: c_long = 0;
pub static mut t4_e_b: c_long = 0;
pub static mut t4_ret: c_long = 0;
pub static mut t5_ret: c_long = 0;
pub static mut t6: c_int = 0;

pub static mut ut1_a_a: c_long = 0;
pub static mut ut1_b: c_long = 0;
pub static mut ut1_c: c_long = 0;
pub static mut ut2_a: c_long = 0;
pub static mut ut2_b_a: c_long = 0;
pub static mut ut2_b_b: c_long = 0;

unsafe extern "C" {
    fn bpf_get_func_arg_cnt(ctx: *mut c_void) -> c_long;
    fn bpf_get_func_arg(ctx: *mut c_void, n: c_int, value: *mut __u64) -> c_long;
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_1"]
pub unsafe extern "C" fn test_struct_arg_1(
    a: bpf_testmod_struct_arg_2,
    b: c_int,
    c: c_int,
) -> c_int {
    t1_a_a = a.a;
    t1_a_b = a.b;
    t1_b = b as c_long;
    t1_c = c as c_long;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_1"]
pub unsafe extern "C" fn test_struct_arg_2(
    ctx: *mut c_void,
    _a: bpf_testmod_struct_arg_2,
    _b: c_int,
    _c: c_int,
    ret: c_int,
) -> c_int {
    t1_nregs = bpf_get_func_arg_cnt(ctx);
    /* a.a */
    bpf_get_func_arg(ctx, 0, &mut t1_reg0);
    /* a.b */
    bpf_get_func_arg(ctx, 1, &mut t1_reg1);
    /* b */
    bpf_get_func_arg(ctx, 2, &mut t1_reg2);
    t1_reg2 = t1_reg2 as c_int as __u64;
    /* c */
    bpf_get_func_arg(ctx, 3, &mut t1_reg3);
    t1_reg3 = t1_reg3 as c_int as __u64;

    t1_ret = ret as c_long;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_2"]
pub unsafe extern "C" fn test_struct_arg_3(
    a: c_int,
    b: bpf_testmod_struct_arg_2,
    c: c_int,
) -> c_int {
    t2_a = a as c_long;
    t2_b_a = b.a;
    t2_b_b = b.b;
    t2_c = c as c_long;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_2"]
pub unsafe extern "C" fn test_struct_arg_4(
    _a: c_int,
    _b: bpf_testmod_struct_arg_2,
    _c: c_int,
    ret: c_int,
) -> c_int {
    t2_ret = ret as c_long;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_3"]
pub unsafe extern "C" fn test_struct_arg_5(
    a: c_int,
    b: c_int,
    c: bpf_testmod_struct_arg_2,
) -> c_int {
    t3_a = a as c_long;
    t3_b = b as c_long;
    t3_c_a = c.a;
    t3_c_b = c.b;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_3"]
pub unsafe extern "C" fn test_struct_arg_6(
    _a: c_int,
    _b: c_int,
    _c: bpf_testmod_struct_arg_2,
    ret: c_int,
) -> c_int {
    t3_ret = ret as c_long;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_4"]
pub unsafe extern "C" fn test_struct_arg_7(
    a: bpf_testmod_struct_arg_1,
    b: c_int,
    c: c_int,
    d: c_int,
    e: bpf_testmod_struct_arg_2,
) -> c_int {
    t4_a_a = a.a as c_long;
    t4_b = b as c_long;
    t4_c = c as c_long;
    t4_d = d as c_long;
    t4_e_a = e.a;
    t4_e_b = e.b;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_4"]
pub unsafe extern "C" fn test_struct_arg_8(
    _a: bpf_testmod_struct_arg_1,
    _b: c_int,
    _c: c_int,
    _d: c_int,
    _e: bpf_testmod_struct_arg_2,
    ret: c_int,
) -> c_int {
    t4_ret = ret as c_long;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_5"]
pub unsafe extern "C" fn test_struct_arg_9() -> c_int {
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_struct_arg_5"]
pub unsafe extern "C" fn test_struct_arg_10(ret: c_int) -> c_int {
    t5_ret = ret as c_long;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_struct_arg_6"]
pub unsafe extern "C" fn test_struct_arg_11(a: *mut bpf_testmod_struct_arg_3) -> c_int {
    t6 = *(*a).b.as_ptr().add(0);
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_union_arg_1"]
pub unsafe extern "C" fn test_union_arg_1(
    a: bpf_testmod_union_arg_1,
    b: c_int,
    c: c_int,
) -> c_int {
    ut1_a_a = a.arg.a as c_long;
    ut1_b = b as c_long;
    ut1_c = c as c_long;
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_union_arg_2"]
pub unsafe extern "C" fn test_union_arg_2(
    a: c_int,
    b: bpf_testmod_union_arg_2,
) -> c_int {
    ut2_a = a as c_long;
    ut2_b_a = b.arg.a;
    ut2_b_b = b.arg.b;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
