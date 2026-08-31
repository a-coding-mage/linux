// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Benjamin Tissoires */
/* Translated from testing/selftests/bpf/prog_tests/wq.c. */
/* C dependencies: <test_progs.h>, "wq.skel.h", "wq_failures.skel.h". */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: c_uint = 2;
const BPF_PROG_TYPE_TRACEPOINT: c_uint = 2;

#[repr(C)]
pub struct bpf_object {
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
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: size_t,
    pub log_size: u32,
    pub log_buf: *mut c_char,
    pub log_level: u32,
}

#[repr(C)]
pub struct wq_bss {
    pub ok_sleepable: c_int,
}

#[repr(C)]
pub struct wq_progs {
    pub test_syscall_array_sleepable: *mut bpf_program,
    pub test_map_no_btf: *mut bpf_program,
}

#[repr(C)]
pub struct wq_maps {
    pub array: *mut bpf_map,
}

#[repr(C)]
pub struct wq {
    pub obj: *mut bpf_object,
    pub progs: wq_progs,
    pub maps: wq_maps,
    pub bss: *mut wq_bss,
}

unsafe extern "C" {
    fn wq__open_and_load() -> *mut wq;
    fn wq__open() -> *mut wq;
    fn wq__attach(obj: *mut wq) -> c_int;
    fn wq__destroy(obj: *mut wq);

    fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_program__insns(prog: *mut bpf_program) -> *const bpf_insn;
    fn bpf_program__insn_cnt(prog: *mut bpf_program) -> size_t;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_load(
        prog_type: c_uint,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn RUN_TESTS(name: *const c_char);
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT_size_t(actual: size_t, expected: size_t, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_wq() {
    let mut wq_skel: *mut wq = core::ptr::null_mut();
    let mut err: c_int;
    let prog_fd: c_int;

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        retval: 0,
    };

    unsafe {
        RUN_TESTS(c"wq".as_ptr());
    }

    /* re-run the success test to check if the timer was actually executed */

    unsafe {
        wq_skel = wq__open_and_load();
        if !ASSERT_OK_PTR(wq_skel as *const c_void, c"wq__open_and_load".as_ptr()) {
            return;
        }

        err = wq__attach(wq_skel);
        if !ASSERT_OK(err, c"wq_attach".as_ptr()) {
            wq__destroy(wq_skel);
            return;
        }

        prog_fd = bpf_program__fd((*wq_skel).progs.test_syscall_array_sleepable);
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c"test_run".as_ptr());
        ASSERT_EQ(topts.retval as c_int, 0, c"test_run".as_ptr());

        usleep(50); /* 10 usecs should be enough, but give it extra */

        ASSERT_EQ(
            (*(*wq_skel).bss).ok_sleepable,
            1 << 1,
            c"ok_sleepable".as_ptr(),
        );

        wq__destroy(wq_skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_failures_wq() {
    unsafe {
        RUN_TESTS(c"wq_failures".as_ptr());
    }
}

unsafe fn test_failure_map_no_btf() {
    let mut skel: *mut wq = core::ptr::null_mut();
    let mut log = [0 as c_char; 8192];
    let mut insns: *const bpf_insn;
    let insn_cnt: size_t;
    let ret: c_int;
    let mut err: c_int;
    let map_fd: c_int;
    let opts = bpf_prog_load_opts {
        sz: core::mem::size_of::<bpf_prog_load_opts>(),
        log_size: core::mem::size_of_val(&log) as u32,
        log_buf: log.as_mut_ptr(),
        log_level: 2,
    };

    unsafe {
        skel = wq__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
            return;
        }

        err = bpf_object__prepare((*skel).obj);
        if !ASSERT_OK(err, c"skel__prepare".as_ptr()) {
            wq__destroy(skel);
            return;
        }

        map_fd = bpf_map_create(
            BPF_MAP_TYPE_ARRAY,
            c"map_no_btf".as_ptr(),
            core::mem::size_of::<__u32>() as c_uint,
            core::mem::size_of::<__u64>() as c_uint,
            100,
            core::ptr::null(),
        );
        if !ASSERT_GT(map_fd, -1, c"map create".as_ptr()) {
            wq__destroy(skel);
            return;
        }

        err = bpf_map__reuse_fd((*skel).maps.array, map_fd);
        if !ASSERT_OK(err, c"map reuse fd".as_ptr()) {
            close(map_fd);
            wq__destroy(skel);
            return;
        }

        insns = bpf_program__insns((*skel).progs.test_map_no_btf);
        if !ASSERT_OK_PTR(insns as *const c_void, c"insns ptr".as_ptr()) {
            wq__destroy(skel);
            return;
        }

        insn_cnt = bpf_program__insn_cnt((*skel).progs.test_map_no_btf);
        if !ASSERT_GT_size_t(insn_cnt, 0, c"insn cnt".as_ptr()) {
            wq__destroy(skel);
            return;
        }

        ret = bpf_prog_load(
            BPF_PROG_TYPE_TRACEPOINT,
            core::ptr::null(),
            c"GPL".as_ptr(),
            insns,
            insn_cnt,
            &opts,
        );
        if !ASSERT_LT(ret, 0, c"prog load failed".as_ptr()) {
            if ret > 0 {
                close(ret);
            }
            wq__destroy(skel);
            return;
        }

        ASSERT_HAS_SUBSTR(
            log.as_ptr(),
            c"map 'map_no_btf' has to have BTF in order to use bpf_wq".as_ptr(),
            c"log complains no map BTF".as_ptr(),
        );

        wq__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_wq_custom() {
    unsafe extern "C" {
        fn test__start_subtest(name: *const c_char) -> bool;
    }

    unsafe {
        if test__start_subtest(c"test_failure_map_no_btf".as_ptr()) {
            test_failure_map_no_btf();
        }
    }
}
