// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */
// C dependencies:
// #include <test_progs.h>
// #include "dummy_st_ops_success.skel.h"
// #include "dummy_st_ops_fail.skel.h"
// #include "trace_dummy_st_ops.skel.h"

use core::ffi::{c_char, c_int, c_long, c_void};

type __u64 = u64;

const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;

/* Need to keep consistent with definition in include/linux/bpf.h */
#[repr(C)]
pub struct bpf_dummy_ops_state {
    pub val: c_int,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct dummy_st_ops_success {
    pub maps: dummy_st_ops_success_maps,
    pub progs: dummy_st_ops_success_progs,
    pub bss: *mut dummy_st_ops_success_bss,
}

#[repr(C)]
pub struct dummy_st_ops_success_maps {
    pub dummy_1: *mut bpf_map,
}

#[repr(C)]
pub struct dummy_st_ops_success_progs {
    pub test_1: *mut bpf_program,
    pub test_2: *mut bpf_program,
    pub test_sleepable: *mut bpf_program,
}

#[repr(C)]
pub struct dummy_st_ops_success_bss {
    pub test_2_args: [__u64; 5],
}

#[repr(C)]
pub struct trace_dummy_st_ops {
    pub progs: trace_dummy_st_ops_progs,
    pub bss: *mut trace_dummy_st_ops_bss,
}

#[repr(C)]
pub struct trace_dummy_st_ops_progs {
    pub fentry_test_1: *mut bpf_program,
}

#[repr(C)]
pub struct trace_dummy_st_ops_bss {
    pub val: c_int,
}

unsafe extern "C" {
    fn dummy_st_ops_success__open_and_load() -> *mut dummy_st_ops_success;
    fn dummy_st_ops_success__destroy(obj: *mut dummy_st_ops_success);
    fn dummy_st_ops_fail();

    fn trace_dummy_st_ops__open() -> *mut trace_dummy_st_ops;
    fn trace_dummy_st_ops__load(obj: *mut trace_dummy_st_ops) -> c_int;
    fn trace_dummy_st_ops__attach(obj: *mut trace_dummy_st_ops) -> c_int;
    fn trace_dummy_st_ops__destroy(obj: *mut trace_dummy_st_ops);

    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn RUN_TESTS(test_object: unsafe extern "C" fn());
}

unsafe fn test_dummy_st_ops_attach() {
    let skel: *mut dummy_st_ops_success;
    let link: *mut bpf_link;

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.dummy_1);
    ASSERT_EQ(
        libbpf_get_error(link as *const c_void),
        -(EOPNOTSUPP as c_long),
        c"dummy_st_ops_attach".as_ptr(),
    );

    dummy_st_ops_success__destroy(skel);
}

unsafe fn test_dummy_init_ret_value() {
    let mut args: [__u64; 1] = [0];
    let mut attr = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        retval: 0,
    };
    let skel: *mut dummy_st_ops_success;
    let fd: c_int;
    let err: c_int;

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    fd = bpf_program__fd((*skel).progs.test_1);
    err = bpf_prog_test_run_opts(fd, &mut attr);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(attr.retval as c_long, 0xf2f3f4f5_u32 as c_long, c"test_ret".as_ptr());

    dummy_st_ops_success__destroy(skel);
}

unsafe fn test_dummy_init_ptr_arg() {
    let exp_retval: c_int = 0xbeef;
    let mut in_state = bpf_dummy_ops_state { val: exp_retval };
    let mut args: [__u64; 1] = [&mut in_state as *mut bpf_dummy_ops_state as usize as __u64];
    let mut attr = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        retval: 0,
    };
    let trace_skel: *mut trace_dummy_st_ops;
    let skel: *mut dummy_st_ops_success;
    let fd: c_int;
    let mut err: c_int;

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    fd = bpf_program__fd((*skel).progs.test_1);

    trace_skel = trace_dummy_st_ops__open();
    if !ASSERT_OK_PTR(trace_skel as *const c_void, c"trace_dummy_st_ops__open".as_ptr()) {
        dummy_st_ops_success__destroy(skel);
        trace_dummy_st_ops__destroy(trace_skel);
        return;
    }

    err = bpf_program__set_attach_target(
        (*trace_skel).progs.fentry_test_1,
        fd,
        c"test_1".as_ptr(),
    );
    if !ASSERT_OK(err, c"set_attach_target(fentry_test_1)".as_ptr()) {
        dummy_st_ops_success__destroy(skel);
        trace_dummy_st_ops__destroy(trace_skel);
        return;
    }

    err = trace_dummy_st_ops__load(trace_skel);
    if !ASSERT_OK(err, c"load(trace_skel)".as_ptr()) {
        dummy_st_ops_success__destroy(skel);
        trace_dummy_st_ops__destroy(trace_skel);
        return;
    }

    err = trace_dummy_st_ops__attach(trace_skel);
    if !ASSERT_OK(err, c"attach(trace_skel)".as_ptr()) {
        dummy_st_ops_success__destroy(skel);
        trace_dummy_st_ops__destroy(trace_skel);
        return;
    }

    err = bpf_prog_test_run_opts(fd, &mut attr);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(in_state.val as c_long, 0x5a, c"test_ptr_ret".as_ptr());
    ASSERT_EQ(attr.retval as c_long, exp_retval as c_long, c"test_ret".as_ptr());
    ASSERT_EQ((*(*trace_skel).bss).val as c_long, exp_retval as c_long, c"fentry_val".as_ptr());

    dummy_st_ops_success__destroy(skel);
    trace_dummy_st_ops__destroy(trace_skel);
}

unsafe fn test_dummy_multiple_args() {
    let mut st = bpf_dummy_ops_state { val: 7 };
    let mut args: [__u64; 5] = [
        &mut st as *mut bpf_dummy_ops_state as usize as __u64,
        (-100_i64) as __u64,
        0x8a5f,
        b'c' as __u64,
        0x1234567887654321_u64,
    ];
    let mut attr = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        retval: 0,
    };
    let skel: *mut dummy_st_ops_success;
    let fd: c_int;
    let err: c_int;
    let mut i: usize;
    let mut name: [c_char; 8] = [0; 8];

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    fd = bpf_program__fd((*skel).progs.test_2);
    err = bpf_prog_test_run_opts(fd, &mut attr);
    ASSERT_OK(err, c"test_run".as_ptr());
    args[0] = 7;
    i = 0;
    while i < args.len() {
        snprintf(
            name.as_mut_ptr(),
            name.len(),
            c"arg %zu".as_ptr(),
            i,
        );
        ASSERT_EQ((*(*skel).bss).test_2_args[i] as c_long, args[i] as c_long, name.as_ptr());
        i += 1;
    }

    dummy_st_ops_success__destroy(skel);
}

unsafe fn test_dummy_sleepable() {
    let mut st = core::mem::MaybeUninit::<bpf_dummy_ops_state>::uninit();
    let mut args: [__u64; 1] = [st.as_mut_ptr() as usize as __u64];
    let mut attr = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        retval: 0,
    };
    let skel: *mut dummy_st_ops_success;
    let fd: c_int;
    let err: c_int;

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    fd = bpf_program__fd((*skel).progs.test_sleepable);
    err = bpf_prog_test_run_opts(fd, &mut attr);
    ASSERT_OK(err, c"test_run".as_ptr());

    dummy_st_ops_success__destroy(skel);
}

/* dummy_st_ops.test_sleepable() parameter is not marked as nullable,
 * thus bpf_prog_test_run_opts() below should be rejected as it tries
 * to pass NULL for this parameter.
 */
unsafe fn test_dummy_sleepable_reject_null() {
    let mut args: [__u64; 1] = [0];
    let mut attr = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        ctx_in: args.as_mut_ptr() as *mut c_void,
        ctx_size_in: core::mem::size_of_val(&args) as u32,
        retval: 0,
    };
    let skel: *mut dummy_st_ops_success;
    let fd: c_int;
    let err: c_int;

    skel = dummy_st_ops_success__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"dummy_st_ops_load".as_ptr()) {
        return;
    }

    fd = bpf_program__fd((*skel).progs.test_sleepable);
    err = bpf_prog_test_run_opts(fd, &mut attr);
    ASSERT_EQ(err as c_long, -(EINVAL as c_long), c"test_run".as_ptr());

    dummy_st_ops_success__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_dummy_st_ops() {
    if test__start_subtest(c"dummy_st_ops_attach".as_ptr()) {
        test_dummy_st_ops_attach();
    }
    if test__start_subtest(c"dummy_init_ret_value".as_ptr()) {
        test_dummy_init_ret_value();
    }
    if test__start_subtest(c"dummy_init_ptr_arg".as_ptr()) {
        test_dummy_init_ptr_arg();
    }
    if test__start_subtest(c"dummy_multiple_args".as_ptr()) {
        test_dummy_multiple_args();
    }
    if test__start_subtest(c"dummy_sleepable".as_ptr()) {
        test_dummy_sleepable();
    }
    if test__start_subtest(c"dummy_sleepable_reject_null".as_ptr()) {
        test_dummy_sleepable_reject_null();
    }

    RUN_TESTS(dummy_st_ops_fail);
}
