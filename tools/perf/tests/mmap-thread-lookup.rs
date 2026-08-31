// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/mmap-thread-lookup.c.
// External perf, pthread, syscall, mmap, and test-suite symbols are provided by
// the surrounding repository/build in the same way the C includes provided them.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type ssize_t = isize;
type size_t = usize;
type synth_cb = unsafe extern "C" fn(machine: *mut machine) -> c_int;

const THREADS: usize = 4;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const SYS_gettid: c_long = 186;
const PERF_RECORD_MISC_USER: c_uint = 2;

#[repr(C)]
struct thread_data {
    pt: pthread_t,
    tid: pid_t,
    map: *mut c_void,
    ready: [c_int; 2],
}

#[repr(C)]
struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
struct machine {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
struct thread {
    _private: [u8; 0],
}

#[repr(C)]
struct map {
    _private: [u8; 0],
}

#[repr(C)]
struct addr_location {
    map: *mut map,
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

static mut go_away: c_int = 0;
static mut threads: [thread_data; THREADS] = [
    thread_data {
        pt: 0,
        tid: 0,
        map: ptr::null_mut(),
        ready: [0; 2],
    },
    thread_data {
        pt: 0,
        tid: 0,
        map: ptr::null_mut(),
        ready: [0; 2],
    },
    thread_data {
        pt: 0,
        tid: 0,
        map: ptr::null_mut(),
        ready: [0; 2],
    },
    thread_data {
        pt: 0,
        tid: 0,
        map: ptr::null_mut(),
        ready: [0; 2],
    },
];

extern "C" {
    static page_size: size_t;
    static mut dump_trace: c_int;
    static verbose: c_int;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn perror(s: *const c_char);
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> pid_t;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__delete(machine: *mut machine);
    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn thread_map__new_by_pid(pid: pid_t) -> *mut perf_thread_map;
    fn perf_thread_map__put(map: *mut perf_thread_map);
    fn perf_event__process(machine: *mut machine, event: *mut c_void, sample: *mut c_void) -> c_int;
    fn perf_event__synthesize_threads(
        tool: *mut c_void,
        process: unsafe extern "C" fn(*mut machine, *mut c_void, *mut c_void) -> c_int,
        machine: *mut machine,
        needs_mmap: c_int,
        data_mmap: c_int,
        proc_map_timeout: c_int,
    ) -> c_int;
    fn perf_event__synthesize_thread_map(
        tool: *mut c_void,
        threads: *mut perf_thread_map,
        process: unsafe extern "C" fn(*mut machine, *mut c_void, *mut c_void) -> c_int,
        machine: *mut machine,
        needs_mmap: c_int,
        data_mmap: c_int,
    ) -> c_int;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_map(
        thread: *mut thread,
        cpumode: c_uint,
        addr: c_ulong,
        al: *mut addr_location,
    );
    fn thread__put(thread: *mut thread);
    fn map__start(map: *mut map) -> u64;
    fn test_assert_val(file: *const c_char, line: c_int, desc: *const c_char, expr: c_int) -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! TEST_ASSERT_VAL {
    ($desc:literal, $expr:expr) => {{
        if !$expr {
            return test_assert_val(c_str!(file!()), line!() as c_int, c_str!($desc), 0);
        }
    }};
}

unsafe fn map_failed(ptr: *mut c_void) -> bool {
    ptr == (-1isize as *mut c_void)
}

unsafe extern "C" fn thread_init(td: *mut thread_data) -> c_int {
    let map = mmap(
        ptr::null_mut(),
        page_size,
        PROT_READ | PROT_WRITE | PROT_EXEC,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );

    if map_failed(map) {
        perror(c_str!("mmap failed"));
        return -1;
    }

    (*td).map = map;
    (*td).tid = syscall(SYS_gettid) as pid_t;

    pr_debug(c_str!("tid = %d, map = %p\n"), (*td).tid, map);
    0
}

unsafe extern "C" fn thread_fn(arg: *mut c_void) -> *mut c_void {
    let td = arg as *mut thread_data;
    let mut go: c_int = 0;

    if thread_init(td) != 0 {
        return ptr::null_mut();
    }

    /* Signal thread_create thread is initialized. */
    let ret = write(
        (*td).ready[1],
        &mut go as *mut c_int as *const c_void,
        core::mem::size_of::<c_int>(),
    );
    if ret != core::mem::size_of::<c_int>() as ssize_t {
        pr_err(c_str!("failed to notify\n"));
        return ptr::null_mut();
    }

    while go_away == 0 {
        /* Waiting for main thread to kill us. */
        usleep(100);
    }

    munmap((*td).map, page_size);
    ptr::null_mut()
}

unsafe extern "C" fn thread_create(i: c_int) -> c_int {
    let td = &mut threads[i as usize] as *mut thread_data;
    let mut go: c_int = 0;

    if pipe((*td).ready.as_mut_ptr()) != 0 {
        return -1;
    }

    let mut err = pthread_create(&mut (*td).pt, ptr::null(), thread_fn, td as *mut c_void);
    if err == 0 {
        /* Wait for thread initialization. */
        let ret = read(
            (*td).ready[0],
            &mut go as *mut c_int as *mut c_void,
            core::mem::size_of::<c_int>(),
        );
        err = (ret != core::mem::size_of::<c_int>() as ssize_t) as c_int;
    }

    close((*td).ready[0]);
    close((*td).ready[1]);
    err
}

unsafe extern "C" fn threads_create() -> c_int {
    let td0 = &mut threads[0] as *mut thread_data;
    let mut i: c_int;
    let mut err: c_int = 0;

    go_away = 0;

    /* 0 is main thread */
    if thread_init(td0) != 0 {
        return -1;
    }

    i = 1;
    while err == 0 && i < THREADS as c_int {
        err = thread_create(i);
        i += 1;
    }

    err
}

unsafe extern "C" fn threads_destroy() -> c_int {
    let td0 = &mut threads[0] as *mut thread_data;
    let mut i: c_int;
    let mut err: c_int = 0;

    /* cleanup the main thread */
    munmap((*td0).map, page_size);

    go_away = 1;

    i = 1;
    while err == 0 && i < THREADS as c_int {
        err = pthread_join(threads[i as usize].pt, ptr::null_mut());
        i += 1;
    }

    err
}

unsafe extern "C" fn synth_all(machine: *mut machine) -> c_int {
    perf_event__synthesize_threads(ptr::null_mut(), perf_event__process, machine, 1, 0, 1)
}

unsafe extern "C" fn synth_process(machine: *mut machine) -> c_int {
    let map: *mut perf_thread_map;
    let err: c_int;

    map = thread_map__new_by_pid(getpid());

    err = perf_event__synthesize_thread_map(
        ptr::null_mut(),
        map,
        perf_event__process,
        machine,
        1,
        0,
    );

    perf_thread_map__put(map);
    err
}

unsafe extern "C" fn mmap_events(synth: synth_cb) -> c_int {
    let mut host_env = core::mem::MaybeUninit::<perf_env>::uninit();
    let machine: *mut machine;
    let mut err: c_int;
    let mut i: c_int;

    /*
     * The threads_create will not return before all threads
     * are spawned and all created memory map.
     *
     * They will loop until threads_destroy is called, so we
     * can safely run synthesizing function.
     */
    TEST_ASSERT_VAL!("failed to create threads", threads_create() == 0);

    perf_env__init(host_env.as_mut_ptr());
    machine = machine__new_host(host_env.as_mut_ptr());

    dump_trace = if verbose > 1 { 1 } else { 0 };

    err = synth(machine);

    dump_trace = 0;

    TEST_ASSERT_VAL!("failed to destroy threads", threads_destroy() == 0);
    TEST_ASSERT_VAL!("failed to synthesize maps", err == 0);

    /*
     * All data is synthesized, try to find map for each
     * thread object.
     */
    i = 0;
    while i < THREADS as c_int {
        let td = &mut threads[i as usize] as *mut thread_data;
        let mut al = core::mem::MaybeUninit::<addr_location>::uninit();
        let thread: *mut thread;

        addr_location__init(al.as_mut_ptr());
        thread = machine__findnew_thread(machine, getpid(), (*td).tid);

        pr_debug(c_str!("looking for map %p\n"), (*td).map);

        thread__find_map(
            thread,
            PERF_RECORD_MISC_USER,
            ((*td).map as *mut u8).add(1) as c_ulong,
            al.as_mut_ptr(),
        );

        thread__put(thread);

        if (*al.as_ptr()).map.is_null() {
            pr_debug(c_str!("failed, couldn't find map\n"));
            err = -1;
            addr_location__exit(al.as_mut_ptr());
            break;
        }

        pr_debug(
            c_str!("map %p, addr %llx\n"),
            (*al.as_ptr()).map,
            map__start((*al.as_ptr()).map),
        );
        addr_location__exit(al.as_mut_ptr());

        i += 1;
    }

    machine__delete(machine);
    perf_env__exit(host_env.as_mut_ptr());
    err
}

/*
 * This test creates 'THREADS' number of threads (including
 * main thread) and each thread creates memory map.
 *
 * When threads are created, we synthesize them with both
 * (separate tests):
 *   perf_event__synthesize_thread_map (process based)
 *   perf_event__synthesize_threads    (global)
 *
 * We test we can find all memory maps via:
 *   thread__find_map
 *
 * by using all thread objects.
 */
unsafe extern "C" fn test__mmap_thread_lookup(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    /* perf_event__synthesize_threads synthesize */
    TEST_ASSERT_VAL!(
        "failed with synthesizing all",
        mmap_events(synth_all) == 0
    );

    /* perf_event__synthesize_thread_map synthesize */
    TEST_ASSERT_VAL!(
        "failed with synthesizing process",
        mmap_events(synth_process) == 0
    );

    0
}

// C source ends with: DEFINE_SUITE("Lookup mmap thread", mmap_thread_lookup);
// The surrounding Rust perf test harness should provide the direct equivalent.
