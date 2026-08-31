// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/task-exit.c.
// Dependencies originally included: debug.h, evlist.h, evsel.h, target.h,
// thread_map.h, tests.h, util/mmap.h, errno.h, signal.h, linux/string.h,
// perf/cpumap.h, perf/evlist.h, perf/mmap.h.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const STRERR_BUFSIZE: usize = 128;
const ENOMEM: c_int = 12;
const SIGCHLD: c_int = 17;
const PERF_RECORD_EXIT: u32 = 4;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub exclusive: bool,
}

#[repr(C)]
pub struct perf_event_attr {
    pub task: u64,
    pub sample_freq: u64,
    pub inherit: u64,
    pub watermark: u64,
    pub wakeup_events: u32,
    pub exclude_kernel: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
}

#[repr(C)]
pub union perf_event {
    pub header: core::mem::ManuallyDrop<perf_event_header>,
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;

    fn evlist__new_dummy() -> *mut evlist;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;
    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_evlist__set_maps(
        evlist: *mut evlist_core,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );
    fn evlist__prepare_workload(
        evlist: *mut evlist,
        target: *mut target,
        argv: *const *const c_char,
        pipe_output: bool,
        exec_error: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    ) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_int) -> c_int;
    fn evlist__start_workload(evlist: *mut evlist);
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn evlist__put(evlist: *mut evlist);
}

static mut EXITED: c_int = 0;
static mut NR_EXIT: c_int = 0;

unsafe extern "C" fn sig_handler(_sig: c_int) {
    unsafe {
        EXITED = 1;
    }
}

/*
 * evlist__prepare_workload will send a SIGUSR1 if the fork fails, since
 * we asked by setting its exec_error to this handler.
 */
unsafe extern "C" fn workload_exec_failed_signal(
    _signo: c_int,
    _info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    unsafe {
        EXITED = 1;
        NR_EXIT = -1;
    }
}

/*
 * This test will start a workload that does nothing then it checks
 * if the number of exit event reported by the kernel is 1 or not
 * in order to check the kernel returns correct number of event.
 */
unsafe extern "C" fn test__task_exit(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut err: c_int = -1;
    let mut event: *mut perf_event;
    let mut evsel: *mut evsel;
    let evlist: *mut evlist;
    let mut target = target { uses_mmap: true };
    let argv: [*const c_char; 2] = [c"true".as_ptr(), ptr::null()];
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let cpus: *mut perf_cpu_map;
    let threads: *mut perf_thread_map;
    let mut md: *mut mmap;
    let mut retry_count: c_int = 0;

    unsafe {
        signal(SIGCHLD, sig_handler);

        evlist = evlist__new_dummy();
        if evlist.is_null() {
            pr_debug(c"evlist__new_dummy\n".as_ptr());
            return -1;
        }

        /*
         * Create maps of threads and cpus to monitor. In this case
         * we start with all threads and cpus (-1, -1) but then in
         * evlist__prepare_workload we'll fill in the only thread
         * we're monitoring, the one forked there.
         */
        cpus = perf_cpu_map__new_any_cpu();
        threads = thread_map__new_by_tid(-1);
        if cpus.is_null() || threads.is_null() {
            err = -ENOMEM;
            pr_debug(c"Not enough memory to create thread/cpu maps\n".as_ptr());
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            evlist__put(evlist);
            return err;
        }

        perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

        err = evlist__prepare_workload(
            evlist,
            &mut target,
            argv.as_ptr(),
            false,
            workload_exec_failed_signal,
        );
        if err < 0 {
            pr_debug(c"Couldn't run the workload!\n".as_ptr());
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            evlist__put(evlist);
            return err;
        }

        evsel = evlist__first(evlist);
        (*evsel).core.attr.task = 1;
        // C conditional preserved: on __s390x__, sample_freq is 1000000; otherwise 1.
        #[cfg(target_arch = "s390x")]
        {
            (*evsel).core.attr.sample_freq = 1000000;
        }
        #[cfg(not(target_arch = "s390x"))]
        {
            (*evsel).core.attr.sample_freq = 1;
        }
        (*evsel).core.attr.inherit = 0;
        (*evsel).core.attr.watermark = 0;
        (*evsel).core.attr.wakeup_events = 1;
        (*evsel).core.attr.exclude_kernel = 1;

        err = evlist__open(evlist);
        if err < 0 {
            pr_debug(
                c"Couldn't open the evlist: %s\n".as_ptr(),
                str_error_r(-err, sbuf.as_mut_ptr(), sbuf.len()),
            );
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            evlist__put(evlist);
            return err;
        }

        if evlist__do_mmap(evlist, 128) < 0 {
            pr_debug(
                c"failed to mmap events: %d (%s)\n".as_ptr(),
                errno,
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            err = -1;
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            evlist__put(evlist);
            return err;
        }

        evlist__start_workload(evlist);

        loop {
            md = evlist__mmap(evlist).add(0);
            if perf_mmap__read_init(&mut (*md).core) >= 0 {
                loop {
                    event = perf_mmap__read_event(&mut (*md).core);
                    if event.is_null() {
                        break;
                    }

                    if (*event).header.type_ == PERF_RECORD_EXIT {
                        NR_EXIT += 1;
                    }

                    perf_mmap__consume(&mut (*md).core);
                }
                perf_mmap__read_done(&mut (*md).core);
            }

            if EXITED != 0 && NR_EXIT != 0 {
                break;
            }

            evlist__poll(evlist, -1);

            if retry_count > 1000 {
                retry_count += 1;
                pr_debug(c"Failed after retrying 1000 times\n".as_ptr());
                err = -1;
                perf_cpu_map__put(cpus);
                perf_thread_map__put(threads);
                evlist__put(evlist);
                return err;
            }
            retry_count += 1;
        }

        if NR_EXIT != 1 {
            pr_debug(c"received %d EXIT records\n".as_ptr(), NR_EXIT);
            err = -1;
        }

        perf_cpu_map__put(cpus);
        perf_thread_map__put(threads);
        evlist__put(evlist);
        err
    }
}

#[unsafe(no_mangle)]
pub static mut tests__task_exit: [test_case; 2] = [
    test_case {
        name: c"Number of exit events of a simple workload".as_ptr(),
        run_case: Some(test__task_exit),
        exclusive: true,
    },
    test_case {
        name: ptr::null(),
        run_case: None,
        exclusive: false,
    },
];

#[unsafe(no_mangle)]
pub static mut suite__task_exit: test_suite = test_suite {
    desc: c"Number of exit events of a simple workload".as_ptr(),
    test_cases: unsafe { tests__task_exit.as_mut_ptr() },
};
