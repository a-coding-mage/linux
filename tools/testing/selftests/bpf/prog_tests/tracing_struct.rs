// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Dependencies translated from:
 * #include <test_progs.h>
 * #include "tracing_struct.skel.h"
 * #include "tracing_struct_many_args.skel.h"
 * #include "tracing_struct_int128.skel.h"
 */

use core::ffi::{c_char, c_int};

extern "C" {
    fn ASSERT_OK_PTR(ptr: *mut core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn trigger_module_test_read(sz: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();

    fn tracing_struct__open_and_load() -> *mut tracing_struct;
    fn tracing_struct__attach(skel: *mut tracing_struct) -> c_int;
    fn tracing_struct__destroy(skel: *mut tracing_struct);

    fn tracing_struct_many_args__open_and_load() -> *mut tracing_struct_many_args;
    fn tracing_struct_many_args__attach(skel: *mut tracing_struct_many_args) -> c_int;
    fn tracing_struct_many_args__destroy(skel: *mut tracing_struct_many_args);

    fn tracing_struct_int128__open_and_load() -> *mut tracing_struct_int128;
    fn tracing_struct_int128__attach(skel: *mut tracing_struct_int128) -> c_int;
    fn tracing_struct_int128__destroy(skel: *mut tracing_struct_int128);
}

#[repr(C)]
pub struct tracing_struct {
    pub bss: *mut tracing_struct_bss,
}

#[repr(C)]
pub struct tracing_struct_many_args {
    pub bss: *mut tracing_struct_many_args_bss,
}

#[repr(C)]
pub struct tracing_struct_int128 {
    pub bss: *mut tracing_struct_int128_bss,
}

extern "C" {
    pub type tracing_struct_bss;
    pub type tracing_struct_many_args_bss;
    pub type tracing_struct_int128_bss;
}

extern "C" {
    static mut t1_a_a: i64;
    static mut t1_a_b: i64;
    static mut t1_b: i64;
    static mut t1_c: i64;
    static mut t1_nregs: i64;
    static mut t1_reg0: i64;
    static mut t1_reg1: i64;
    static mut t1_reg2: i64;
    static mut t1_reg3: i64;
    static mut t1_ret: i64;
    static mut t2_a: i64;
    static mut t2_b_a: i64;
    static mut t2_b_b: i64;
    static mut t2_c: i64;
    static mut t2_ret: i64;
    static mut t3_a: i64;
    static mut t3_b: i64;
    static mut t3_c_a: i64;
    static mut t3_c_b: i64;
    static mut t3_ret: i64;
    static mut t4_a_a: i64;
    static mut t4_b: i64;
    static mut t4_c: i64;
    static mut t4_d: i64;
    static mut t4_e_a: i64;
    static mut t4_e_b: i64;
    static mut t4_ret: i64;
    static mut t5_ret: i64;
    static mut t6: i64;

    static mut t7_a: i64;
    static mut t7_b: i64;
    static mut t7_c: i64;
    static mut t7_d: i64;
    static mut t7_e: i64;
    static mut t7_f_a: i64;
    static mut t7_f_b: i64;
    static mut t7_ret: i64;
    static mut t8_a: i64;
    static mut t8_b: i64;
    static mut t8_c: i64;
    static mut t8_d: i64;
    static mut t8_e: i64;
    static mut t8_f_a: i64;
    static mut t8_f_b: i64;
    static mut t8_g: i64;
    static mut t8_ret: i64;
    static mut t9_a: i64;
    static mut t9_b: i64;
    static mut t9_c: i64;
    static mut t9_d: i64;
    static mut t9_e: i64;
    static mut t9_f: i64;
    static mut t9_g: i64;
    static mut t9_h_a: i64;
    static mut t9_h_b: i64;
    static mut t9_h_c: i64;
    static mut t9_h_d: i64;
    static mut t9_i: i64;
    static mut t9_ret: i64;

    static mut t_b: i64;
    static mut t_c: i64;
    static mut t_ret: i64;

    static mut ut1_a_a: i64;
    static mut ut1_b: i64;
    static mut ut1_c: i64;
    static mut ut2_a: i64;
    static mut ut2_b_a: i64;
    static mut ut2_b_b: i64;
}

unsafe fn test_struct_args() {
    let skel: *mut tracing_struct;
    let mut err: c_int;

    skel = tracing_struct__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast::<core::ffi::c_void>(),
        c"tracing_struct__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = tracing_struct__attach(skel);
    if !ASSERT_OK(err, c"tracing_struct__attach".as_ptr()) {
        tracing_struct__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ(t1_a_a, 2, c"t1:a.a".as_ptr());
    ASSERT_EQ(t1_a_b, 3, c"t1:a.b".as_ptr());
    ASSERT_EQ(t1_b, 1, c"t1:b".as_ptr());
    ASSERT_EQ(t1_c, 4, c"t1:c".as_ptr());

    ASSERT_EQ(t1_nregs, 4, c"t1 nregs".as_ptr());
    ASSERT_EQ(t1_reg0, 2, c"t1 reg0".as_ptr());
    ASSERT_EQ(t1_reg1, 3, c"t1 reg1".as_ptr());
    ASSERT_EQ(t1_reg2, 1, c"t1 reg2".as_ptr());
    ASSERT_EQ(t1_reg3, 4, c"t1 reg3".as_ptr());
    ASSERT_EQ(t1_ret, 10, c"t1 ret".as_ptr());

    ASSERT_EQ(t2_a, 1, c"t2:a".as_ptr());
    ASSERT_EQ(t2_b_a, 2, c"t2:b.a".as_ptr());
    ASSERT_EQ(t2_b_b, 3, c"t2:b.b".as_ptr());
    ASSERT_EQ(t2_c, 4, c"t2:c".as_ptr());
    ASSERT_EQ(t2_ret, 10, c"t2 ret".as_ptr());

    ASSERT_EQ(t3_a, 1, c"t3:a".as_ptr());
    ASSERT_EQ(t3_b, 4, c"t3:b".as_ptr());
    ASSERT_EQ(t3_c_a, 2, c"t3:c.a".as_ptr());
    ASSERT_EQ(t3_c_b, 3, c"t3:c.b".as_ptr());
    ASSERT_EQ(t3_ret, 10, c"t3 ret".as_ptr());

    ASSERT_EQ(t4_a_a, 10, c"t4:a.a".as_ptr());
    ASSERT_EQ(t4_b, 1, c"t4:b".as_ptr());
    ASSERT_EQ(t4_c, 2, c"t4:c".as_ptr());
    ASSERT_EQ(t4_d, 3, c"t4:d".as_ptr());
    ASSERT_EQ(t4_e_a, 2, c"t4:e.a".as_ptr());
    ASSERT_EQ(t4_e_b, 3, c"t4:e.b".as_ptr());
    ASSERT_EQ(t4_ret, 21, c"t4 ret".as_ptr());

    ASSERT_EQ(t5_ret, 1, c"t5 ret".as_ptr());

    ASSERT_EQ(t6, 1, c"t6 ret".as_ptr());

    tracing_struct__destroy(skel);
}

unsafe fn test_struct_many_args() {
    let skel: *mut tracing_struct_many_args;
    let mut err: c_int;

    skel = tracing_struct_many_args__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast::<core::ffi::c_void>(),
        c"tracing_struct_many_args__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = tracing_struct_many_args__attach(skel);
    if !ASSERT_OK(err, c"tracing_struct_many_args__attach".as_ptr()) {
        tracing_struct_many_args__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ(t7_a, 16, c"t7:a".as_ptr());
    ASSERT_EQ(t7_b, 17, c"t7:b".as_ptr());
    ASSERT_EQ(t7_c, 18, c"t7:c".as_ptr());
    ASSERT_EQ(t7_d, 19, c"t7:d".as_ptr());
    ASSERT_EQ(t7_e, 20, c"t7:e".as_ptr());
    ASSERT_EQ(t7_f_a, 21, c"t7:f.a".as_ptr());
    ASSERT_EQ(t7_f_b, 22, c"t7:f.b".as_ptr());
    ASSERT_EQ(t7_ret, 133, c"t7 ret".as_ptr());

    ASSERT_EQ(t8_a, 16, c"t8:a".as_ptr());
    ASSERT_EQ(t8_b, 17, c"t8:b".as_ptr());
    ASSERT_EQ(t8_c, 18, c"t8:c".as_ptr());
    ASSERT_EQ(t8_d, 19, c"t8:d".as_ptr());
    ASSERT_EQ(t8_e, 20, c"t8:e".as_ptr());
    ASSERT_EQ(t8_f_a, 21, c"t8:f.a".as_ptr());
    ASSERT_EQ(t8_f_b, 22, c"t8:f.b".as_ptr());
    ASSERT_EQ(t8_g, 23, c"t8:g".as_ptr());
    ASSERT_EQ(t8_ret, 156, c"t8 ret".as_ptr());

    ASSERT_EQ(t9_a, 16, c"t9:a".as_ptr());
    ASSERT_EQ(t9_b, 17, c"t9:b".as_ptr());
    ASSERT_EQ(t9_c, 18, c"t9:c".as_ptr());
    ASSERT_EQ(t9_d, 19, c"t9:d".as_ptr());
    ASSERT_EQ(t9_e, 20, c"t9:e".as_ptr());
    ASSERT_EQ(t9_f, 21, c"t9:f".as_ptr());
    ASSERT_EQ(t9_g, 22, c"t9:f".as_ptr());
    ASSERT_EQ(t9_h_a, 23, c"t9:h.a".as_ptr());
    ASSERT_EQ(t9_h_b, 24, c"t9:h.b".as_ptr());
    ASSERT_EQ(t9_h_c, 25, c"t9:h.c".as_ptr());
    ASSERT_EQ(t9_h_d, 26, c"t9:h.d".as_ptr());
    ASSERT_EQ(t9_i, 27, c"t9:i".as_ptr());
    ASSERT_EQ(t9_ret, 258, c"t9 ret".as_ptr());

    tracing_struct_many_args__destroy(skel);
}

unsafe fn test_int128_args() {
    /*
     * __int128 arguments are passed in a register pair on x86_64 and
     * arm64, which the trampoline packs into two context slots. Other
     * architectures pass a __int128 differently (e.g. s390x passes larger
     * arguments by reference), so only exercise this on x86_64 and arm64.
     */
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        let skel: *mut tracing_struct_int128;
        let mut err: c_int;

        skel = tracing_struct_int128__open_and_load();
        if !ASSERT_OK_PTR(
            skel.cast::<core::ffi::c_void>(),
            c"tracing_struct_int128__open_and_load".as_ptr(),
        ) {
            return;
        }

        err = tracing_struct_int128__attach(skel);
        if !ASSERT_OK(err, c"tracing_struct_int128__attach".as_ptr()) {
            tracing_struct_int128__destroy(skel);
            return;
        }

        ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

        ASSERT_EQ(t_b, 2, c"t:b".as_ptr());
        ASSERT_EQ(t_c, 3, c"t:c".as_ptr());
        ASSERT_EQ(t_ret, 6, c"t ret".as_ptr());

        tracing_struct_int128__destroy(skel);
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        test__skip();
    }
}

unsafe fn test_union_args() {
    let skel: *mut tracing_struct;
    let mut err: c_int;

    skel = tracing_struct__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast::<core::ffi::c_void>(),
        c"tracing_struct__open_and_load".as_ptr(),
    ) {
        return;
    }

    err = tracing_struct__attach(skel);
    if !ASSERT_OK(err, c"tracing_struct__attach".as_ptr()) {
        tracing_struct__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ(ut1_a_a, 1, c"ut1:a.arg.a".as_ptr());
    ASSERT_EQ(ut1_b, 4, c"ut1:b".as_ptr());
    ASSERT_EQ(ut1_c, 5, c"ut1:c".as_ptr());

    ASSERT_EQ(ut2_a, 6, c"ut2:a".as_ptr());
    ASSERT_EQ(ut2_b_a, 2, c"ut2:b.arg.a".as_ptr());
    ASSERT_EQ(ut2_b_b, 3, c"ut2:b.arg.b".as_ptr());

    tracing_struct__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_tracing_struct() {
    if test__start_subtest(c"struct_args".as_ptr()) {
        test_struct_args();
    }
    if test__start_subtest(c"struct_many_args".as_ptr()) {
        test_struct_many_args();
    }
    if test__start_subtest(c"int128_args".as_ptr()) {
        test_int128_args();
    }
    if test__start_subtest(c"union_args".as_ptr()) {
        test_union_args();
    }
}
