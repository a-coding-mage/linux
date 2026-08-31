// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust references:
// errno.h, inttypes.h, api/fs/tracing_path.h, linux/err.h, linux/string.h,
// sys/types.h, sys/stat.h, fcntl.h, unistd.h, thread_map.h, evsel.h, debug.h,
// tests.h, util/counts.h

use core::ffi::{c_char, c_int, c_uint, c_ulong};

const O_RDONLY: c_int = 0;

extern "C" {
    static mut errno: c_int;

    fn getpid() -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);

    fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn evsel__open_per_thread(evsel: *mut evsel, threads: *mut perf_thread_map) -> c_int;
    fn evsel__read_on_cpu(evsel: *mut evsel, cpu: c_int, thread: c_int) -> c_int;
    fn evsel__put(evsel: *mut evsel);
    fn perf_evsel__close_fd(core: *mut perf_evsel);

    fn tracing_path__strerror_open_tp(
        errnum: c_int,
        buf: *mut c_char,
        size: usize,
        sys: *const c_char,
        name: *const c_char,
    );
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);

    fn perf_counts(counts: *mut perf_counts_values, cpu: c_int, thread: c_int) -> *mut perf_counts_value;
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub desc: *const c_char,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_value {
    pub val: u64,
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
    pub counts: *mut perf_counts_values,
}

const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;
const STRERR_BUFSIZE: usize = 128;
const BUFSIZ: usize = 8192;

unsafe extern "C" fn test__openat_syscall_event(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    let mut err: c_int = TEST_FAIL;
    let mut fd: c_int;
    let mut evsel: *mut evsel;
    let nr_openat_calls: c_uint = 111;
    let mut i: c_uint;
    let threads: *mut perf_thread_map = thread_map__new_by_tid(getpid());
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let mut errbuf: [c_char; BUFSIZ] = [0; BUFSIZ];

    if threads.is_null() {
        pr_debug(b"thread_map__new\n\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    evsel = evsel__newtp(
        b"syscalls\0".as_ptr() as *const c_char,
        b"sys_enter_openat\0".as_ptr() as *const c_char,
    );
    if IS_ERR(evsel as *const core::ffi::c_void) {
        tracing_path__strerror_open_tp(
            errno,
            errbuf.as_mut_ptr(),
            errbuf.len(),
            b"syscalls\0".as_ptr() as *const c_char,
            b"sys_enter_openat\0".as_ptr() as *const c_char,
        );
        pr_debug(b"%s\n\0".as_ptr() as *const c_char, errbuf.as_mut_ptr());
        err = TEST_SKIP;
        goto_out_thread_map_delete(threads, err)
    } else {
        if evsel__open_per_thread(evsel, threads) < 0 {
            pr_debug(
                b"failed to open counter: %s, tweak /proc/sys/kernel/perf_event_paranoid?\n\0"
                    .as_ptr() as *const c_char,
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            err = TEST_SKIP;
            evsel__put(evsel);
            perf_thread_map__put(threads);
            return err;
        }

        i = 0;
        while i < nr_openat_calls {
            fd = openat(0, b"/etc/passwd\0".as_ptr() as *const c_char, O_RDONLY);
            close(fd);
            i = i.wrapping_add(1);
        }

        if evsel__read_on_cpu(evsel, 0, 0) < 0 {
            pr_debug(b"evsel__read_on_cpu\n\0".as_ptr() as *const c_char);
            perf_evsel__close_fd(&mut (*evsel).core);
            evsel__put(evsel);
            perf_thread_map__put(threads);
            return err;
        }

        if (*perf_counts((*evsel).counts, 0, 0)).val != nr_openat_calls as u64 {
            pr_debug(
                b"evsel__read_on_cpu: expected to intercept %d calls, got %llu\n\0".as_ptr()
                    as *const c_char,
                nr_openat_calls,
                (*perf_counts((*evsel).counts, 0, 0)).val as c_ulong,
            );
            perf_evsel__close_fd(&mut (*evsel).core);
            evsel__put(evsel);
            perf_thread_map__put(threads);
            return err;
        }

        err = TEST_OK;

        perf_evsel__close_fd(&mut (*evsel).core);
        evsel__put(evsel);
        perf_thread_map__put(threads);
        return err;
    }
}

unsafe fn goto_out_thread_map_delete(threads: *mut perf_thread_map, err: c_int) -> c_int {
    perf_thread_map__put(threads);
    err
}

#[no_mangle]
pub static mut tests__openat_syscall_event: [test_case; 2] = [
    test_case {
        name: b"Detect openat syscall event\0".as_ptr() as *const c_char,
        run_case: Some(test__openat_syscall_event),
        desc: b"permissions\0".as_ptr() as *const c_char,
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
        desc: core::ptr::null(),
    },
];

#[no_mangle]
pub static mut suite__openat_syscall_event: test_suite = test_suite {
    desc: b"Detect openat syscall event\0".as_ptr() as *const c_char,
    test_cases: unsafe { tests__openat_syscall_event.as_mut_ptr() },
};
