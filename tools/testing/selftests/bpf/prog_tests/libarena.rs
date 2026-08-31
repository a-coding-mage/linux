// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
/*
 * Translated from testing/selftests/bpf/prog_tests/libarena.c.
 * C include dependencies:
 * - <test_progs.h>
 * - <unistd.h>
 * - <libarena/common.h>
 * - <libarena/asan.h>
 * - <libarena/buddy.h>
 * - <libarena/userspace.h>
 * - "libarena/libarena.skel.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

type size_t = usize;
type pthread_t = usize;

const ENOENT: c_int = 2;
const EINTR: c_int = 4;
const ENOMEM: c_int = 12;
const ENAMETOOLONG: c_int = 36;
const EOPNOTSUPP: c_int = 95;
const UINT_MAX: u32 = u32::MAX;

const ARENA_RESERVE_PAGES_DFL: u32 = 256;

/* Max suffix is ceil((lg 2^32) / (lg 10)) + sizeof("__") = 10 + 2 = 12. */
const MAX_PARTEST_SUFFIX: usize = 12;
const MAX_PARTEST_NAME: usize = 1024;
const MAX_PARTEST_PREFIX: usize = MAX_PARTEST_NAME - MAX_PARTEST_SUFFIX;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
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
struct arena_alloc_reserve_args {
    nr_pages: u32,
}

#[repr(C)]
struct libarena_progs {
    arena_buddy_destroy: *mut bpf_program,
    arena_buddy_reset: *mut bpf_program,
    arena_alloc_reserve: *mut bpf_program,
}

#[repr(C)]
struct libarena {
    obj: *mut bpf_object,
    progs: libarena_progs,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;

    fn libarena__open_and_load() -> *mut libarena;
    fn libarena__attach(skel: *mut libarena) -> c_int;
    fn libarena__destroy(skel: *mut libarena);
    fn libarena_run_prog(fd: c_int) -> c_int;
    fn libarena_run_prog_args(fd: c_int, args: *mut c_void, args_sz: size_t) -> c_int;
    fn libarena_is_parallel_test_prog(name: *const c_char) -> bool;
    fn libarena_is_test_prog(name: *const c_char) -> bool;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut libarena, name: *const c_char) -> bool;
    fn ASSERT_LT<T, U>(left: T, right: U, name: *const c_char) -> bool;
    fn ASSERT_TRUE<T>(value: T, name: *const c_char) -> bool;
}

unsafe fn run_libarena_test(
    skel: *mut libarena,
    prog: *mut bpf_program,
    name: *const c_char,
) {
    let mut ret: c_int;

    if !strstr(name, c"test_buddy".as_ptr()).is_null() {
        /* Buddy tests initialize the allocator directly. */
        ret = libarena_run_prog(bpf_program__fd((*skel).progs.arena_buddy_destroy));
        if !ASSERT_OK(ret, c"arena_buddy_destroy".as_ptr()) {
            return;
        }
    } else {
        ret = libarena_run_prog(bpf_program__fd((*skel).progs.arena_buddy_reset));
        if !ASSERT_OK(ret, c"arena_buddy_reset".as_ptr()) {
            return;
        }
    }

    ret = libarena_run_prog(bpf_program__fd(prog));

    ASSERT_OK(ret, name);
}

unsafe extern "C" fn run_libarena_parallel_prog(arg: *mut c_void) -> *mut c_void {
    let prog: *mut bpf_program = arg.cast();

    libarena_run_prog(bpf_program__fd(prog)) as c_long as *mut c_void
}

unsafe fn run_libarena_parallel_fini(
    skel: *mut libarena,
    name: *const c_char,
    prefixlen: size_t,
) -> c_int {
    let mut tdname = [0 as c_char; MAX_PARTEST_NAME];
    let mut fini_prog: *mut bpf_program;
    let mut ret: c_int;

    ret = snprintf(
        tdname.as_mut_ptr(),
        tdname.len(),
        c"%.*s__fini".as_ptr(),
        prefixlen as c_int,
        name,
    );
    if !ASSERT_LT(ret, tdname.len(), c"partest fini name".as_ptr()) {
        return -ENAMETOOLONG;
    }

    fini_prog = bpf_object__find_program_by_name((*skel).obj, tdname.as_ptr());
    if !ASSERT_TRUE(fini_prog, c"partest fini prog".as_ptr()) {
        return -ENOENT;
    }

    ret = libarena_run_prog(bpf_program__fd(fini_prog));
    ASSERT_OK(ret, tdname.as_ptr());

    ret
}

unsafe fn run_libarena_parallel_test_workers(
    skel: *mut libarena,
    name: *const c_char,
    prefixlen: size_t,
) -> c_int {
    let mut threads: *mut pthread_t = ptr::null_mut();
    let mut tmp_threads: *mut pthread_t;
    let mut tdname = [0 as c_char; MAX_PARTEST_NAME];
    let mut tdprog: *mut bpf_program;
    let mut nthreads: u32;
    let mut thread_ret: *mut c_void = ptr::null_mut();
    let mut ret: c_int;
    let mut err: c_int = 0;
    let mut worker_err: c_int;
    let mut i: c_int;

    nthreads = 0;
    while nthreads < UINT_MAX {
        ret = snprintf(
            tdname.as_mut_ptr(),
            tdname.len(),
            c"%.*s__%u".as_ptr(),
            prefixlen as c_int,
            name,
            nthreads,
        );
        if !ASSERT_LT(ret, tdname.len(), c"test worker name".as_ptr()) {
            err = -ENAMETOOLONG;
            break;
        }

        /*
         * We enumerate the worker threads for a given test with __0, __1,
         * and so on. The suffixes always start from 0 and are contiguous,
         * so if we don't find a program with the requested name we have
         * discovered all available worker programs.
         */
        tdprog = bpf_object__find_program_by_name((*skel).obj, tdname.as_ptr());
        if tdprog.is_null() {
            break;
        }

        /* Bump the alloc array to accommodate the new thread. */
        tmp_threads = realloc(
            threads.cast(),
            (nthreads as usize + 1) * std::mem::size_of::<pthread_t>(),
        )
        .cast();
        if !ASSERT_TRUE(tmp_threads, c"realloc".as_ptr()) {
            err = -ENOMEM;
            break;
        }
        threads = tmp_threads;

        ret = pthread_create(
            threads.add(nthreads as usize),
            ptr::null(),
            run_libarena_parallel_prog,
            tdprog.cast(),
        );
        if !ASSERT_OK(ret, c"pthread_create".as_ptr()) {
            err = ret;
            break;
        }

        nthreads = nthreads.wrapping_add(1);
    }

    i = 0;
    while i < nthreads as c_int {
        ret = pthread_join(*threads.add(i as usize), &mut thread_ret);
        if !ASSERT_OK(ret, c"pthread_join".as_ptr()) {
            if err == 0 {
                err = ret;
            }
            i += 1;
            continue;
        }

        worker_err = thread_ret as c_long as c_int;

        /*
         * A worker that bails out because another one already gave up
         * reports -EINTR. It is collateral damage that carries no
         * information, so skip it entirely: never let it become the
         * reported error, and don't log it either.
         */
        if worker_err == 0 || worker_err == -EINTR {
            i += 1;
            continue;
        }

        if err == 0 {
            err = worker_err;
        }

        fprintf(
            stdout,
            c"%.*s__%d returned %d\n".as_ptr(),
            prefixlen as c_int,
            name,
            i,
            worker_err,
        );
        i += 1;
    }

    free(threads.cast());

    err
}

unsafe fn libarena_parallel_test_enabled(
    skel: *mut libarena,
    prefix: *const c_char,
    prefixlen: size_t,
) -> bool {
    let mut prog: *mut bpf_program;
    let mut progname = [0 as c_char; MAX_PARTEST_NAME];
    let mut ret: c_int;

    ret = snprintf(
        progname.as_mut_ptr(),
        progname.len(),
        c"%.*s__enabled".as_ptr(),
        prefixlen as c_int,
        prefix,
    );
    if !ASSERT_LT(ret, progname.len(), c"partest enabled name".as_ptr()) {
        return false;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, progname.as_ptr());
    if prog.is_null() {
        return true;
    }

    ret = libarena_run_prog(bpf_program__fd(prog));
    if ret == -EOPNOTSUPP {
        return false;
    }
    if !ASSERT_OK(ret, progname.as_ptr()) {
        return false;
    }
    true
}

unsafe fn run_libarena_parallel_test(
    skel: *mut libarena,
    prog: *mut bpf_program,
    name: *const c_char,
) {
    let mut testname = [0 as c_char; MAX_PARTEST_NAME];
    let mut prefixlen: size_t;
    let pos: *const c_char;
    let mut ret: c_int;

    /*
     * We annotate the initialization prog with __init. If the current prog does
     * not match, it is one of the parallel threads instead and is ignored.
     *
     * We assume the test writer knows what they are doing and do not add __init
     * randomly in the middle of a test name.
     */
    pos = strstr(name, c"__init".as_ptr());
    if pos.is_null() {
        return;
    }

    prefixlen = pos.offset_from(name) as size_t;
    if !ASSERT_LT(prefixlen, MAX_PARTEST_PREFIX, c"partest prefix too long".as_ptr()) {
        return;
    }

    /* The name of the test without the __init suffix. Looks nicer in the test log. */
    ret = snprintf(
        testname.as_mut_ptr(),
        testname.len(),
        c"%.*s".as_ptr(),
        prefixlen as c_int,
        name,
    );
    if !ASSERT_LT(ret, testname.len(), c"partest test name".as_ptr()) {
        return;
    }

    if !test__start_subtest(testname.as_ptr()) {
        return;
    }

    if !libarena_parallel_test_enabled(skel, testname.as_ptr(), prefixlen) {
        test__skip();
        return;
    }

    ret = libarena_run_prog(bpf_program__fd((*skel).progs.arena_buddy_reset));
    if !ASSERT_OK(ret, c"arena_buddy_reset".as_ptr()) {
        return;
    }

    ret = libarena_run_prog(bpf_program__fd(prog));
    if !ASSERT_OK(ret, testname.as_ptr()) {
        return;
    }

    ret = run_libarena_parallel_test_workers(skel, name, prefixlen);

    ASSERT_OK(ret, testname.as_ptr());

    run_libarena_parallel_fini(skel, name, prefixlen);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_libarena() {
    let mut args: arena_alloc_reserve_args = std::mem::zeroed();
    let mut skel: *mut libarena;
    let mut prog: *mut bpf_program;
    let mut ret: c_int;

    skel = libarena__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) {
        return;
    }

    ret = libarena__attach(skel);
    if !ASSERT_OK(ret, c"attach".as_ptr()) {
        libarena__destroy(skel);
        return;
    }

    args.nr_pages = ARENA_RESERVE_PAGES_DFL;

    ret = libarena_run_prog_args(
        bpf_program__fd((*skel).progs.arena_alloc_reserve),
        (&mut args as *mut arena_alloc_reserve_args).cast(),
        std::mem::size_of_val(&args),
    );
    if !ASSERT_OK(ret, c"arena_alloc_reserve".as_ptr()) {
        libarena__destroy(skel);
        return;
    }

    /*
     * Translation of:
     * bpf_object__for_each_program(prog, skel->obj) { ... }
     *
     * The iterator macro is supplied by external libbpf headers and has no
     * file-local Rust equivalent. Keep the body translated verbatim below for
     * use with the corresponding Rust binding or macro in the final tree.
     */
    bpf_object__for_each_program!(prog, (*skel).obj, {
        let name: *const c_char = bpf_program__name(prog);

        /*
         * Handle parallel test progs separately. For those
         * progs it's not a matter of test/skip, because each
         * parallel test prog includes an initialization prog
         * and a set of progs to be run in parallel. For the
         * latter we do not record them as skipped or run,
         * because we run them all at once when we come across
         * the initialization prog. For more details on how we
         * discover the progs see the comment on
         * run_libarena_parallel_test.
         */
        if libarena_is_parallel_test_prog(name) {
            run_libarena_parallel_test(skel, prog, name);
            continue;
        }

        if !libarena_is_test_prog(name) {
            continue;
        }

        if !test__start_subtest(name) {
            continue;
        }

        run_libarena_test(skel, prog, name);
    });

    libarena__destroy(skel);
}
