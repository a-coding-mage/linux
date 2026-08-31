// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source:
// testing/selftests/bpf/prog_tests/spin_lock.c
//
// C dependencies preserved as external declarations:
// <regex.h>, <test_progs.h>, <network_helpers.h>,
// "test_spin_lock.skel.h", and "test_spin_lock_fail.skel.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u32 = c_uint;
type size_t = usize;
type pthread_t = usize;

const REG_NOSUB: c_int = 1;

#[repr(C)]
struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object_open_opts {
    kernel_log_buf: *mut c_char,
    kernel_log_size: size_t,
    kernel_log_level: c_int,
}

#[repr(C)]
struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: u32,
    repeat: u32,
    retval: u32,
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct test_spin_lock_fail {
    obj: *mut bpf_object,
}

#[repr(C)]
struct test_spin_lock__progs {
    bpf_spin_lock_test: *mut bpf_program,
}

#[repr(C)]
struct test_spin_lock {
    progs: test_spin_lock__progs,
}

#[repr(C)]
struct spin_lock_fail_test {
    prog_name: *const c_char,
    err_msg: *const c_char,
}

static mut log_buf: [c_char; 1024 * 1024] = [0; 1024 * 1024];

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static spin_lock_fail_tests: [spin_lock_fail_test; 26] = [
    spin_lock_fail_test {
        prog_name: cstr!("lock_id_kptr_preserve"),
        err_msg: cstr!(
            "[0-9]\\+: (bf) r1 = r0                       ; R0=ptr_foo(id=2) R1=ptr_foo(id=2) refs=2\n\
[0-9]\\+: (85) call bpf_this_cpu_ptr#154\n\
R1 type=ptr_ expected=percpu_ptr_"
        ),
    },
    spin_lock_fail_test {
        prog_name: cstr!("lock_id_global_zero"),
        err_msg: cstr!(
            "; R1=map_value(map=.data.A,ks=4,vs=4)\n2: (85) call bpf_this_cpu_ptr#154\n\
R1 type=map_value expected=percpu_ptr_"
        ),
    },
    spin_lock_fail_test {
        prog_name: cstr!("lock_id_mapval_preserve"),
        err_msg: cstr!(
            "[0-9]\\+: (bf) r1 = r0                       ; R0=map_value(id=1,map=array_map,ks=4,vs=8) R1=map_value(id=1,map=array_map,ks=4,vs=8)\n\
[0-9]\\+: (85) call bpf_this_cpu_ptr#154\n\
R1 type=map_value expected=percpu_ptr_"
        ),
    },
    spin_lock_fail_test {
        prog_name: cstr!("lock_id_innermapval_preserve"),
        err_msg: cstr!(
            "[0-9]\\+: (bf) r1 = r0                      ; R0=map_value(id=2,ks=4,vs=8) R1=map_value(id=2,ks=4,vs=8)\n\
[0-9]\\+: (85) call bpf_this_cpu_ptr#154\n\
R1 type=map_value expected=percpu_ptr_"
        ),
    },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_kptr_kptr"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_kptr_global"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_kptr_mapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_kptr_innermapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_global_global"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_global_kptr"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_global_mapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_global_innermapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_mapval_mapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_mapval_kptr"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_mapval_global"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_mapval_innermapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_innermapval_innermapval1"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_innermapval_innermapval2"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_innermapval_kptr"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_innermapval_global"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_id_mismatch_innermapval_mapval"), err_msg: cstr!("bpf_spin_unlock of different lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_global_subprog_call1"), err_msg: cstr!("global function calls are not allowed while holding a lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_global_subprog_call2"), err_msg: cstr!("global function calls are not allowed while holding a lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_global_sleepable_helper_subprog"), err_msg: cstr!("global function calls are not allowed while holding a lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_global_sleepable_kfunc_subprog"), err_msg: cstr!("global function calls are not allowed while holding a lock") },
    spin_lock_fail_test { prog_name: cstr!("lock_global_sleepable_subprog_indirect"), err_msg: cstr!("global function calls are not allowed while holding a lock") },
];

unsafe extern "C" {
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut c_void,
        eflags: c_int,
    ) -> c_int;
    fn regerror(errcode: c_int, preg: *const regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t;
    fn regfree(preg: *mut regex_t);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn PRINT_FAIL(format: *const c_char, ...);
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: *mut c_void, expected: *mut c_void, name: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn test_spin_lock_fail__open_opts(opts: *const bpf_object_open_opts) -> *mut test_spin_lock_fail;
    fn test_spin_lock_fail__load(skel: *mut test_spin_lock_fail) -> c_int;
    fn test_spin_lock_fail__destroy(skel: *mut test_spin_lock_fail);
    fn test_spin_lock__open_and_load() -> *mut test_spin_lock;
    fn test_spin_lock__destroy(skel: *mut test_spin_lock);

    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    static pkt_v4: [u8; 0];
}

unsafe fn match_regex(pattern: *const c_char, string: *const c_char) -> c_int {
    let mut err: c_int;
    let rc: c_int;
    let mut re: regex_t = mem::zeroed();

    err = regcomp(&mut re, pattern, REG_NOSUB);
    if err != 0 {
        let mut errbuf: [c_char; 512] = [0; 512];

        regerror(err, &re, errbuf.as_mut_ptr(), mem::size_of_val(&errbuf));
        PRINT_FAIL(cstr!("Can't compile regex: %s\n"), errbuf.as_ptr());
        return -1;
    }
    rc = regexec(&re, string, 0, ptr::null_mut(), 0);
    regfree(&mut re);
    if rc == 0 { 1 } else { 0 }
}

unsafe fn test_spin_lock_fail_prog(prog_name: *const c_char, err_msg: *const c_char) {
    let opts = bpf_object_open_opts {
        kernel_log_buf: log_buf.as_mut_ptr(),
        kernel_log_size: mem::size_of_val(&log_buf),
        kernel_log_level: 1,
    };
    let skel: *mut test_spin_lock_fail;
    let prog: *mut bpf_program;
    let mut ret: c_int;

    skel = test_spin_lock_fail__open_opts(&opts);
    if !ASSERT_OK_PTR(skel as *const c_void, cstr!("test_spin_lock_fail__open_opts")) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, cstr!("bpf_object__find_program_by_name")) {
        test_spin_lock_fail__destroy(skel);
        return;
    }

    bpf_program__set_autoload(prog, true);

    ret = test_spin_lock_fail__load(skel);
    if !ASSERT_ERR(ret, cstr!("test_spin_lock_fail__load must fail")) {
        test_spin_lock_fail__destroy(skel);
        return;
    }

    /* Skip check if JIT does not support kfuncs */
    if !strstr(
        log_buf.as_ptr(),
        cstr!("JIT does not support calling kernel function"),
    )
    .is_null()
    {
        test__skip();
        test_spin_lock_fail__destroy(skel);
        return;
    }

    ret = match_regex(err_msg, log_buf.as_ptr());
    if !ASSERT_GE(ret, 0, cstr!("match_regex")) {
        test_spin_lock_fail__destroy(skel);
        return;
    }

    if !ASSERT_TRUE(ret, cstr!("no match for expected error message")) {
        fprintf(stderr, cstr!("Expected: %s\n"), err_msg);
        fprintf(stderr, cstr!("Verifier: %s\n"), log_buf.as_ptr());
    }

    test_spin_lock_fail__destroy(skel);
}

unsafe extern "C" fn spin_lock_thread(arg: *mut c_void) -> *mut c_void {
    let prog_fd: c_int = *(arg as *mut u32) as c_int;
    let mut topts = bpf_test_run_opts {
        data_in: pkt_v4.as_ptr() as *const c_void,
        data_size_in: mem::size_of_val(&pkt_v4) as u32,
        repeat: 10000,
        retval: 0,
    };

    let err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, cstr!("test_run"));
    ASSERT_OK(topts.retval as c_int, cstr!("test_run retval"));
    pthread_exit(arg);
}

#[no_mangle]
pub unsafe extern "C" fn test_spin_lock_success() {
    let skel: *mut test_spin_lock;
    let mut thread_id: [pthread_t; 4] = [0; 4];
    let mut prog_fd: c_int;
    let mut i: c_int;
    let mut ret: *mut c_void = ptr::null_mut();

    skel = test_spin_lock__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, cstr!("test_spin_lock__open_and_load")) {
        return;
    }
    prog_fd = bpf_program__fd((*skel).progs.bpf_spin_lock_test);
    i = 0;
    while i < 4 {
        let err: c_int;

        err = pthread_create(
            &mut thread_id[i as usize],
            ptr::null(),
            Some(spin_lock_thread),
            &mut prog_fd as *mut c_int as *mut c_void,
        );
        if !ASSERT_OK(err, cstr!("pthread_create")) {
            test_spin_lock__destroy(skel);
            return;
        }
        i += 1;
    }

    i = 0;
    while i < 4 {
        if !ASSERT_OK(
            pthread_join(thread_id[i as usize], &mut ret),
            cstr!("pthread_join"),
        ) {
            test_spin_lock__destroy(skel);
            return;
        }
        if !ASSERT_EQ(
            ret,
            &mut prog_fd as *mut c_int as *mut c_void,
            cstr!("ret == prog_fd"),
        ) {
            test_spin_lock__destroy(skel);
            return;
        }
        i += 1;
    }

    test_spin_lock__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_spin_lock() {
    let mut i: c_int;

    test_spin_lock_success();

    i = 0;
    while (i as usize) < spin_lock_fail_tests.len() {
        if !test__start_subtest(spin_lock_fail_tests[i as usize].prog_name) {
            i += 1;
            continue;
        }
        test_spin_lock_fail_prog(
            spin_lock_fail_tests[i as usize].prog_name,
            spin_lock_fail_tests[i as usize].err_msg,
        );
        i += 1;
    }
}
