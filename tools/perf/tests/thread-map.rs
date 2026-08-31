// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/thread-map.c.
// Original C includes provided declarations from:
// stdlib.h, string.h, sys/types.h, unistd.h, sys/prctl.h, tests.h,
// thread_map.h, debug.h, event.h, util/synthetic-events.h, perf/event.h,
// internal/threadmap.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    pub nr: c_int,
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct perf_record_thread_map_entry {
    pub pid: u64,
    pub comm: [c_char; 0],
}

#[repr(C)]
pub struct perf_record_thread_map {
    pub nr: u64,
    pub entries: [perf_record_thread_map_entry; 0],
}

#[repr(C)]
pub union perf_event {
    pub thread_map: core::mem::ManuallyDrop<perf_record_thread_map>,
}

const PR_SET_NAME: c_int = 15;
const NAME: *const c_char = b"perf\0".as_ptr() as *const c_char;

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !($cond) {
            return -1;
        }
    };
}

extern "C" {
    static mut verbose: c_int;
    static mut stderr: *mut c_void;

    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn getpid() -> c_int;
    fn getppid() -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);

    fn thread_map__new_by_pid(pid: c_int) -> *mut perf_thread_map;
    fn thread_map__read_comms(map: *mut perf_thread_map);
    fn perf_thread_map__pid(map: *mut perf_thread_map, idx: c_int) -> c_int;
    fn perf_thread_map__comm(map: *mut perf_thread_map, idx: c_int) -> *const c_char;
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn perf_thread_map__put(map: *mut perf_thread_map);
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn thread_map__new_event(event: *const perf_record_thread_map) -> *mut perf_thread_map;
    fn perf_event__synthesize_thread_map2(
        tool: *mut perf_tool,
        threads: *mut perf_thread_map,
        process: unsafe extern "C" fn(
            *const perf_tool,
            *mut perf_event,
            *mut perf_sample,
            *mut machine,
        ) -> c_int,
        machine: *mut machine,
    ) -> c_int;
    fn thread_map__new_str(
        str_: *const c_char,
        tid: *const c_void,
        all_threads: bool,
    ) -> *mut perf_thread_map;
    fn thread_map__fprintf(threads: *mut perf_thread_map, fp: *mut c_void);
    fn thread_map__remove(threads: *mut perf_thread_map, idx: c_int) -> c_int;
}

unsafe extern "C" fn test__thread_map(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut map: *mut perf_thread_map;

    TEST_ASSERT_VAL!(
        "failed to set process name",
        prctl(PR_SET_NAME, NAME as c_ulong, 0, 0, 0) == 0
    );

    /* test map on current pid */
    map = thread_map__new_by_pid(getpid());
    TEST_ASSERT_VAL!("failed to alloc map", !map.is_null());

    thread_map__read_comms(map);

    TEST_ASSERT_VAL!("wrong nr", (*map).nr == 1);
    TEST_ASSERT_VAL!("wrong pid", perf_thread_map__pid(map, 0) == getpid());
    TEST_ASSERT_VAL!(
        "wrong comm",
        !perf_thread_map__comm(map, 0).is_null()
            && strcmp(perf_thread_map__comm(map, 0), NAME) == 0
    );
    TEST_ASSERT_VAL!("wrong refcnt", refcount_read(&(*map).refcnt) == 1);
    perf_thread_map__put(map);

    /* test dummy pid */
    map = perf_thread_map__new_dummy();
    TEST_ASSERT_VAL!("failed to alloc map", !map.is_null());

    thread_map__read_comms(map);

    TEST_ASSERT_VAL!("wrong nr", (*map).nr == 1);
    TEST_ASSERT_VAL!("wrong pid", perf_thread_map__pid(map, 0) == -1);
    TEST_ASSERT_VAL!(
        "wrong comm",
        !perf_thread_map__comm(map, 0).is_null()
            && strcmp(perf_thread_map__comm(map, 0), b"dummy\0".as_ptr() as *const c_char) == 0
    );
    TEST_ASSERT_VAL!("wrong refcnt", refcount_read(&(*map).refcnt) == 1);
    perf_thread_map__put(map);
    0
}

unsafe extern "C" fn process_event(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let map: *mut perf_record_thread_map = &mut (*event).thread_map as *mut _ as *mut perf_record_thread_map;
    let mut threads: *mut perf_thread_map;

    TEST_ASSERT_VAL!("wrong nr", (*map).nr == 1);
    TEST_ASSERT_VAL!("wrong pid", (*(*map).entries.as_ptr()).pid == getpid() as u64);
    TEST_ASSERT_VAL!(
        "wrong comm",
        strcmp((*(*map).entries.as_ptr()).comm.as_ptr(), NAME) == 0
    );

    threads = thread_map__new_event(&(*event).thread_map as *const _ as *const perf_record_thread_map);
    TEST_ASSERT_VAL!("failed to alloc map", !threads.is_null());

    TEST_ASSERT_VAL!("wrong nr", (*threads).nr == 1);
    TEST_ASSERT_VAL!("wrong pid", perf_thread_map__pid(threads, 0) == getpid());
    TEST_ASSERT_VAL!(
        "wrong comm",
        !perf_thread_map__comm(threads, 0).is_null()
            && strcmp(perf_thread_map__comm(threads, 0), NAME) == 0
    );
    TEST_ASSERT_VAL!("wrong refcnt", refcount_read(&(*threads).refcnt) == 1);
    perf_thread_map__put(threads);
    0
}

unsafe extern "C" fn test__thread_map_synthesize(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let threads: *mut perf_thread_map;

    TEST_ASSERT_VAL!(
        "failed to set process name",
        prctl(PR_SET_NAME, NAME as c_ulong, 0, 0, 0) == 0
    );

    /* test map on current pid */
    threads = thread_map__new_by_pid(getpid());
    TEST_ASSERT_VAL!("failed to alloc map", !threads.is_null());

    thread_map__read_comms(threads);

    TEST_ASSERT_VAL!(
        "failed to synthesize map",
        perf_event__synthesize_thread_map2(
            core::ptr::null_mut(),
            threads,
            process_event,
            core::ptr::null_mut()
        ) == 0
    );

    perf_thread_map__put(threads);
    0
}

unsafe extern "C" fn test__thread_map_remove(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let threads: *mut perf_thread_map;
    let mut str_: *mut c_char = core::ptr::null_mut();

    TEST_ASSERT_VAL!(
        "failed to allocate map string",
        asprintf(
            &mut str_,
            b"%d,%d\0".as_ptr() as *const c_char,
            getpid(),
            getppid()
        ) >= 0
    );

    threads = thread_map__new_str(str_, core::ptr::null(), false);
    free(str_ as *mut c_void);

    TEST_ASSERT_VAL!("failed to allocate thread_map", !threads.is_null());

    if verbose > 0 {
        thread_map__fprintf(threads, stderr);
    }

    TEST_ASSERT_VAL!("failed to remove thread", thread_map__remove(threads, 0) == 0);

    TEST_ASSERT_VAL!("thread_map count != 1", (*threads).nr == 1);

    if verbose > 0 {
        thread_map__fprintf(threads, stderr);
    }

    TEST_ASSERT_VAL!("failed to remove thread", thread_map__remove(threads, 0) == 0);

    TEST_ASSERT_VAL!("thread_map count != 0", (*threads).nr == 0);

    if verbose > 0 {
        thread_map__fprintf(threads, stderr);
    }

    TEST_ASSERT_VAL!("failed to not remove thread", thread_map__remove(threads, 0) != 0);

    perf_thread_map__put(threads);
    0
}

// DEFINE_SUITE("Thread map", thread_map);
// DEFINE_SUITE("Synthesize thread map", thread_map_synthesize);
// DEFINE_SUITE("Remove thread map", thread_map_remove);
