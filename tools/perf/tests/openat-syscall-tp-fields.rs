// SPDX-License-Identifier: GPL-2.0
// C dependencies: stdbool.h, linux/err.h, linux/string.h, sys/types.h,
// sys/stat.h, fcntl.h, evlist.h, evsel.h, thread_map.h, record.h, tests.h,
// debug.h, util/mmap.h, errno.h, perf/mmap.h, util/sample.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const O_DIRECTORY: c_int = 0o0200000;
const AT_FDCWD: c_int = -100;

const UINT_MAX: c_uint = c_uint::MAX;
const TEST_FAIL: c_int = 1;
const TEST_SKIP: c_int = 2;
const TEST_OK: c_int = 0;
const EACCES: c_int = 13;
const O_RDONLY: c_int = 0;
const O_ACCMODE: c_int = 0o00000003;
const STRERR_BUFSIZE: usize = 128;
const PERF_RECORD_SAMPLE: u32 = 9;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub func: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct record_opts {
    pub target: target,
    pub no_buffering: bool,
    pub freq: c_int,
    pub mmap_pages: c_int,
    pub raw_samples: bool,
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub threads: *mut perf_thread_map,
    pub nr_mmaps: c_int,
}

#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
}

#[repr(C)]
pub struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__poll(evlist: *mut evlist, timeout_ms: c_int) -> c_int;

    fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel;
    fn evsel__config(evsel: *mut evsel, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evsel__parse_sample(
        evsel: *mut evsel,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;

    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: c_int);
    fn getpid() -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *const c_char;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int;

    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);

    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
}

unsafe extern "C" fn test__syscall_openat_tp_fields(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut opts = record_opts {
        target: target { uses_mmap: true },
        no_buffering: true,
        freq: 1,
        mmap_pages: 256,
        raw_samples: true,
    };
    let filename = c"/etc/passwd".as_ptr();
    let flags: c_int = O_RDONLY | O_DIRECTORY;
    let evlist = unsafe { evlist__new() };
    let evsel: *mut evsel;
    let mut ret: c_int = TEST_FAIL;
    let mut err: c_int;
    let mut i: c_int;
    let mut nr_events: c_int = 0;
    let mut nr_polls: c_int = 0;
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    if evlist.is_null() {
        unsafe {
            pr_debug(c"%s: evlist__new\n".as_ptr(), c"test__syscall_openat_tp_fields".as_ptr());
        }
        return ret;
    }

    evsel = unsafe { evsel__newtp(c"syscalls".as_ptr(), c"sys_enter_openat".as_ptr()) };
    if unsafe { IS_ERR(evsel as *const c_void) } {
        unsafe {
            pr_debug(c"%s: evsel__newtp\n".as_ptr(), c"test__syscall_openat_tp_fields".as_ptr());
        }
        ret = if unsafe { PTR_ERR(evsel as *const c_void) } == -(EACCES as isize) {
            TEST_SKIP
        } else {
            TEST_FAIL
        };
        unsafe {
            evlist__put(evlist);
        }
        return ret;
    }

    unsafe {
        evlist__add(evlist, evsel);
    }

    err = unsafe { evlist__create_maps(evlist, &mut opts.target) };
    if err < 0 {
        unsafe {
            pr_debug(c"%s: evlist__create_maps\n".as_ptr(), c"test__syscall_openat_tp_fields".as_ptr());
            evlist__put(evlist);
        }
        return ret;
    }

    unsafe {
        evsel__config(evsel, &mut opts, ptr::null_mut());
        perf_thread_map__set_pid((*evlist__core(evlist)).threads, 0, getpid());
    }

    err = unsafe { evlist__open(evlist) };
    if err < 0 {
        unsafe {
            pr_debug(
                c"perf_evlist__open: %s\n".as_ptr(),
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            evlist__put(evlist);
        }
        return ret;
    }

    err = unsafe { evlist__do_mmap(evlist, UINT_MAX) };
    if err < 0 {
        unsafe {
            pr_debug(
                c"evlist__mmap: %s\n".as_ptr(),
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            evlist__put(evlist);
        }
        return ret;
    }

    unsafe {
        evlist__enable(evlist);
    }

    /*
     * Generate the event:
     */
    unsafe {
        openat(AT_FDCWD, filename, flags);
    }

    loop {
        let before = nr_events;

        i = 0;
        while i < unsafe { (*evlist__core(evlist)).nr_mmaps } {
            let mut event: *mut perf_event;
            let md: *mut mmap;

            md = unsafe { evlist__mmap(evlist).add(i as usize) };
            if unsafe { perf_mmap__read_init(&mut (*md).core) } < 0 {
                i += 1;
                continue;
            }

            loop {
                event = unsafe { perf_mmap__read_event(&mut (*md).core) };
                if event.is_null() {
                    break;
                }

                let type_ = unsafe { (*event).header.type_ };
                let tp_flags: c_int;
                let mut sample = perf_sample { _private: [] };

                nr_events += 1;

                if type_ != PERF_RECORD_SAMPLE {
                    unsafe {
                        perf_mmap__consume(&mut (*md).core);
                    }
                    continue;
                }

                unsafe {
                    perf_sample__init(&mut sample, false);
                }
                err = unsafe { evsel__parse_sample(evsel, event, &mut sample) };
                if err != 0 {
                    unsafe {
                        pr_debug(c"Can't parse sample, err = %d\n".as_ptr(), err);
                        perf_sample__exit(&mut sample);
                        evlist__put(evlist);
                    }
                    return ret;
                }

                tp_flags = unsafe { perf_sample__intval(&mut sample, c"flags".as_ptr()) };
                unsafe {
                    perf_sample__exit(&mut sample);
                }
                /* C library wrapper may set additional flags,
                   access mode must be unchanged */
                if (tp_flags & O_ACCMODE) != (flags & O_ACCMODE) || (tp_flags & flags) != flags {
                    unsafe {
                        pr_debug(
                            c"%s: Expected flags=%#x, got %#x\n".as_ptr(),
                            c"test__syscall_openat_tp_fields".as_ptr(),
                            flags,
                            tp_flags,
                        );
                        evlist__put(evlist);
                    }
                    return ret;
                }

                ret = TEST_OK;
                unsafe {
                    evlist__put(evlist);
                }
                return ret;
            }
            unsafe {
                perf_mmap__read_done(&mut (*md).core);
            }
            i += 1;
        }

        if nr_events == before {
            unsafe {
                evlist__poll(evlist, 10);
            }
        }

        nr_polls += 1;
        if nr_polls > 5 {
            unsafe {
                pr_debug(c"%s: no events!\n".as_ptr(), c"test__syscall_openat_tp_fields".as_ptr());
                evlist__put(evlist);
            }
            return ret;
        }
    }
}

static mut TESTS__SYSCALL_OPENAT_TP_FIELDS: [test_case; 2] = [
    test_case {
        name: c"syscalls:sys_enter_openat event fields".as_ptr(),
        func: Some(test__syscall_openat_tp_fields),
        reason: c"permissions".as_ptr(),
    },
    test_case {
        name: ptr::null(),
        func: None,
        reason: ptr::null(),
    },
];

#[unsafe(no_mangle)]
pub static mut suite__syscall_openat_tp_fields: test_suite = test_suite {
    desc: c"syscalls:sys_enter_openat event fields".as_ptr(),
    test_cases: unsafe { TESTS__SYSCALL_OPENAT_TP_FIELDS.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
