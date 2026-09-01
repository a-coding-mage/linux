// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
/* Translated from C implementation source: test_task_work.c. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const PERF_TYPE_HARDWARE: __u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: __u64 = 0;
const __NR_perf_event_open: c_long = 298;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;

#[repr(C)]
struct perf_event_attr {
    type_: __u32,
    size: __u32,
    config: __u64,
    sample_period: __u64,
}

#[repr(C)]
struct bpf_task_work {
    _opaque: [u8; 0],
}

#[repr(C)]
struct elem {
    data: [c_char; 128],
    tw: bpf_task_work,
}

#[repr(C)]
struct bpf_map {
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
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct task_work_bss {
    user_ptr: *mut c_char,
}

#[repr(C)]
struct task_work {
    obj: *mut bpf_object,
    bss: *mut task_work_bss,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(number: c_long, ...) -> c_long;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn time(tloc: *mut c_long) -> c_long;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;

    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *mut c_void,
        value_sz: usize,
        flags: c_ulong,
    ) -> c_int;
    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;

    fn task_work__open() -> *mut task_work;
    fn task_work__load(skel: *mut task_work) -> c_int;
    fn task_work__destroy(skel: *mut task_work);

    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach_perf_event(
        prog: *mut bpf_program,
        pfd: c_int,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__find_map_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_map;

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn task_work_fail__run_tests();

    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn perf_event_open(type_: __u32, config: __u64, pid: c_int) -> c_int {
    let attr = perf_event_attr {
        type_,
        config,
        size: size_of::<perf_event_attr>() as __u32,
        sample_period: 100000,
    };

    syscall(__NR_perf_event_open, &attr, pid, -1, -1, 0) as c_int
}

unsafe fn verify_map(map: *mut bpf_map, expected_data: *const c_char) -> c_int {
    let mut err: c_int;
    let mut value: elem = core::mem::zeroed();
    let mut processed_values: c_int = 0;
    let mut k: c_int;
    let sz: c_int;

    sz = bpf_map__max_entries(map);
    k = 0;
    while k < sz {
        err = bpf_map__lookup_elem(
            map,
            &k as *const c_int as *const c_void,
            size_of::<c_int>(),
            &mut value as *mut elem as *mut c_void,
            size_of::<elem>(),
            0,
        );
        if err != 0 {
            k += 1;
            continue;
        }
        if !ASSERT_EQ(
            strcmp(expected_data, value.data.as_ptr()),
            0,
            c"map data".as_ptr(),
        ) {
            fprintf(
                stderr,
                c"expected '%s', found '%s' in %s map".as_ptr(),
                expected_data,
                value.data.as_ptr(),
                bpf_map__name(map),
            );
            return 2;
        }
        processed_values += 1;
        k += 1;
    }

    (processed_values == 0) as c_int
}

unsafe fn task_work_run(prog_name: *const c_char, map_name: *const c_char) {
    let mut skel: *mut task_work;
    let mut prog: *mut bpf_program;
    let mut map: *mut bpf_map;
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut err: c_int;
    let mut pe_fd: c_int = -1;
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut pipefd: [c_int; 2] = [0; 2];
    let mut user_string = *b"hello world\0";

    if !ASSERT_NEQ(pipe(pipefd.as_mut_ptr()), -1, c"pipe".as_ptr()) {
        return;
    }

    pid = fork();
    if pid == 0 {
        let mut num: __u64 = 1;
        let mut i: c_int;
        let mut buf: c_char = 0;

        close(pipefd[1]);
        read(
            pipefd[0],
            &mut buf as *mut c_char as *mut c_void,
            size_of::<c_char>(),
        );
        close(pipefd[0]);

        i = 0;
        while i < 10000 {
            num = num.wrapping_mul((time(ptr::null_mut()) % 7) as __u64);
            i += 1;
        }
        let _ = num;
        exit(0);
    }
    if !ASSERT_GT(pid, 0, c"fork() failed".as_ptr()) {
        close(pipefd[0]);
        close(pipefd[1]);
        return;
    }

    skel = task_work__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"task_work__open".as_ptr()) {
        return;
    }

    prog = bpf_object__next_program((*skel).obj, ptr::null_mut());
    while !prog.is_null() {
        bpf_program__set_autoload(prog, false);
        prog = bpf_object__next_program((*skel).obj, prog);
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c"prog_name".as_ptr()) {
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }
    bpf_program__set_autoload(prog, true);
    (*(*skel).bss).user_ptr = user_string.as_mut_ptr() as *mut c_char;

    err = task_work__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }

    pe_fd = perf_event_open(PERF_TYPE_HARDWARE, PERF_COUNT_HW_CPU_CYCLES, pid);
    if pe_fd == -1 && (errno == ENOENT || errno == EOPNOTSUPP) {
        printf(
            c"%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n".as_ptr(),
            c"task_work_run".as_ptr(),
        );
        test__skip();
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }
    if !ASSERT_NEQ(pe_fd, -1, c"pe_fd".as_ptr()) {
        fprintf(
            stderr,
            c"perf_event_open errno: %d, pid: %d\n".as_ptr(),
            errno,
            pid,
        );
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }

    link = bpf_program__attach_perf_event(prog, pe_fd);
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_perf_event".as_ptr()) {
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }

    /* perf event fd ownership is passed to bpf_link */
    pe_fd = -1;
    close(pipefd[0]);
    write(pipefd[1], user_string.as_ptr() as *const c_void, 1);
    close(pipefd[1]);
    /* Wait to collect some samples */
    waitpid(pid, &mut status, 0);
    pid = 0;
    map = bpf_object__find_map_by_name((*skel).obj, map_name);
    if !ASSERT_OK_PTR(map as *const c_void, c"find map_name".as_ptr()) {
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }
    if !ASSERT_OK(verify_map(map, user_string.as_ptr()), c"verify map".as_ptr()) {
        goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
        return;
    }
    goto_cleanup(skel, link, pe_fd, pid, pipefd, user_string.as_ptr(), &mut status);
}

unsafe fn goto_cleanup(
    skel: *mut task_work,
    link: *mut bpf_link,
    pe_fd: c_int,
    pid: c_int,
    pipefd: [c_int; 2],
    user_string: *const c_char,
    status: *mut c_int,
) {
    if pe_fd >= 0 {
        close(pe_fd);
    }
    bpf_link__destroy(link);
    task_work__destroy(skel);
    if pid > 0 {
        close(pipefd[0]);
        write(pipefd[1], user_string as *const c_void, 1);
        close(pipefd[1]);
        waitpid(pid, status, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_work() {
    if test__start_subtest(c"test_task_work_hash_map".as_ptr()) {
        task_work_run(c"oncpu_hash_map".as_ptr(), c"hmap".as_ptr());
    }

    if test__start_subtest(c"test_task_work_array_map".as_ptr()) {
        task_work_run(c"oncpu_array_map".as_ptr(), c"arrmap".as_ptr());
    }

    if test__start_subtest(c"test_task_work_lru_map".as_ptr()) {
        task_work_run(c"oncpu_lru_map".as_ptr(), c"lrumap".as_ptr());
    }

    task_work_fail__run_tests();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
