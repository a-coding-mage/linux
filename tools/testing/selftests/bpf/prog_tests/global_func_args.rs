// SPDX-License-Identifier: GPL-2.0
// Translated from global_func_args.c.
// Dependencies from "test_progs.h" and "network_helpers.h" are declared as
// external symbols where this file references them.

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = c_uint;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub retval: u32,
    pub repeat: c_int,
}

const BPF_PROG_TYPE_CGROUP_SKB: c_int = 8;

static mut duration: __u32 = 0;

unsafe extern "C" {
    static pkt_v4: [u8; 0];
    static mut errno: c_int;

    fn strerror(errnum: c_int) -> *mut c_char;
    fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn CHECK(
        condition: bool,
        tag: *const c_char,
        fmt: *const c_char,
        ...
    ) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char);
}

unsafe fn test_global_func_args0(obj: *mut bpf_object) {
    let mut err: c_int;
    let mut i: c_int;
    let map_fd: c_int;
    let mut actual_value: c_int = 0;
    let map_name: *const c_char = c"values".as_ptr();

    map_fd = bpf_find_map(c"test_global_func_args0".as_ptr(), obj, map_name);
    if CHECK(
        map_fd < 0,
        c"bpf_find_map".as_ptr(),
        c"cannot find BPF map %s: %s\n".as_ptr(),
        map_name,
        strerror(errno),
    ) {
        return;
    }

    #[repr(C)]
    struct Test {
        descr: *const c_char,
        expected_value: c_int,
    }

    let tests = [
        Test {
            descr: c"passing NULL pointer".as_ptr(),
            expected_value: 0,
        },
        Test {
            descr: c"returning value".as_ptr(),
            expected_value: 1,
        },
        Test {
            descr: c"reading local variable".as_ptr(),
            expected_value: 100,
        },
        Test {
            descr: c"writing local variable".as_ptr(),
            expected_value: 101,
        },
        Test {
            descr: c"reading global variable".as_ptr(),
            expected_value: 42,
        },
        Test {
            descr: c"writing global variable".as_ptr(),
            expected_value: 43,
        },
        Test {
            descr: c"writing to pointer-to-pointer".as_ptr(),
            expected_value: 1,
        },
    ];

    i = 0;
    while (i as usize) < tests.len() {
        let expected_value: c_int = tests[i as usize].expected_value;

        err = bpf_map_lookup_elem(
            map_fd,
            &i as *const c_int as *const c_void,
            &mut actual_value as *mut c_int as *mut c_void,
        );

        CHECK(
            err != 0 || actual_value != expected_value,
            tests[i as usize].descr,
            c"err %d result %d expected %d\n".as_ptr(),
            err,
            actual_value,
            expected_value,
        );

        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_global_func_args() {
    let file: *const c_char = c"./test_global_func_args.bpf.o".as_ptr();
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    // LIBBPF_OPTS(bpf_test_run_opts, topts, ...).
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: pkt_v4.as_ptr() as *const c_void,
        data_out: core::ptr::null_mut(),
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        data_size_out: 0,
        retval: 0,
        repeat: 1,
    };

    err = bpf_prog_test_load(
        file,
        BPF_PROG_TYPE_CGROUP_SKB,
        &mut obj as *mut *mut bpf_object,
        &mut prog_fd as *mut c_int,
    );
    if CHECK(
        err != 0,
        c"load program".as_ptr(),
        c"error %d loading %s\n".as_ptr(),
        err,
        file,
    ) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts as *mut bpf_test_run_opts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"test_run retval".as_ptr());

    test_global_func_args0(obj);

    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
