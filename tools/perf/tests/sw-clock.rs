// SPDX-License-Identifier: GPL-2.0

// Translated from includes:
// errno.h, inttypes.h, unistd.h, stdlib.h, signal.h, sys/mman.h,
// linux/compiler.h, linux/string.h, tests.h, util/debug.h, util/evsel.h,
// util/evlist.h, util/cpumap.h, util/mmap.h, util/sample.h,
// util/thread_map.h, perf/evlist.h, perf/mmap.h.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const NR_LOOPS: c_int = 10000000;

type u64 = u64;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
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
pub struct perf_evlist {
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
    pub r#type: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_sample {
    pub period: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub r#type: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    // C bitfields in struct perf_event_attr. The surrounding bindings are
    // expected to provide the exact layout for real builds.
    pub flags: u64,
}

impl perf_event_attr {
    unsafe fn set_sample_freq(&mut self, value: u64) {
        self.sample_period_or_freq = value;
    }
}

type perf_sw_ids = c_uint;

const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_CPU_CLOCK: perf_sw_ids = 0;
const PERF_COUNT_SW_TASK_CLOCK: perf_sw_ids = 1;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_RECORD_SAMPLE: u32 = 9;
const STRERR_BUFSIZE: usize = 128;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    static mut errno: c_int;

    fn getpid() -> c_int;

    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;

    fn evlist__new() -> *mut evlist;
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist;
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_int) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__parse_sample(
        evlist: *mut evlist,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;
    fn evlist__put(evlist: *mut evlist);

    fn evsel__new(attr: *const perf_event_attr) -> *mut evsel;

    fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);

    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);

    fn perf_evlist__set_maps(
        evlist: *mut perf_evlist,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );

    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);

    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
}

/*
 * This test will open software clock events (cpu-clock, task-clock)
 * then check their frequency -> period conversion has no artifact of
 * setting period to 1 forcefully.
 */
unsafe fn __test__sw_clock_freq(clock_id: perf_sw_ids) -> c_int {
    let mut i: c_int;
    let mut err: c_int = -1;
    let mut tmp: c_int = 0;
    let mut total_periods: u64 = 0;
    let mut nr_samples: c_int = 0;
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let mut event: *mut perf_event;
    let evsel: *mut evsel;
    let evlist: *mut evlist;
    let mut attr = perf_event_attr {
        r#type: PERF_TYPE_SOFTWARE,
        size: 0,
        config: clock_id as u64,
        sample_period_or_freq: 0,
        sample_type: PERF_SAMPLE_PERIOD,
        read_format: 0,
        flags: (1 << 5) | (1 << 0) | (1 << 10), // exclude_kernel, disabled, freq
    };
    let mut cpus: *mut perf_cpu_map = core::ptr::null_mut();
    let mut threads: *mut perf_thread_map = core::ptr::null_mut();
    let md: *mut mmap;

    attr.set_sample_freq(500);

    evlist = evlist__new();
    if evlist.is_null() {
        pr_debug(c"evlist__new\n".as_ptr());
        return -1;
    }

    evsel = evsel__new(&attr);
    if evsel.is_null() {
        pr_debug(c"evsel__new\n".as_ptr());
        goto_out_put_evlist(evlist, cpus, threads);
        return err;
    }
    evlist__add(evlist, evsel);

    cpus = perf_cpu_map__new_any_cpu();
    threads = thread_map__new_by_tid(getpid());
    if cpus.is_null() || threads.is_null() {
        err = -ENOMEM;
        pr_debug(c"Not enough memory to create thread/cpu maps\n".as_ptr());
        goto_out_put_evlist(evlist, cpus, threads);
        return err;
    }

    perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

    if evlist__open(evlist) != 0 {
        let knob = c"/proc/sys/kernel/perf_event_max_sample_rate";

        err = -errno;
        pr_debug(
            c"Couldn't open evlist: %s\nHint: check %s, using %llu in this test.\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            knob.as_ptr(),
            attr.sample_period_or_freq as c_ulonglong,
        );
        goto_out_put_evlist(evlist, cpus, threads);
        return err;
    }

    err = evlist__do_mmap(evlist, 128);
    if err < 0 {
        pr_debug(
            c"failed to mmap event: %d (%s)\n".as_ptr(),
            errno,
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        goto_out_put_evlist(evlist, cpus, threads);
        return err;
    }

    evlist__enable(evlist);

    /* collect samples */
    i = 0;
    while i < NR_LOOPS {
        core::ptr::write_volatile(&mut tmp, core::ptr::read_volatile(&tmp).wrapping_add(1));
        i += 1;
    }

    evlist__disable(evlist);

    md = evlist__mmap(evlist).add(0);
    if perf_mmap__read_init(&mut (*md).core) < 0 {
        if nr_samples as u64 == total_periods {
            pr_debug(
                c"All (%d) samples have period value of 1!\n".as_ptr(),
                nr_samples,
            );
            err = -1;
        }
        goto_out_put_evlist(evlist, cpus, threads);
        return err;
    }

    loop {
        event = perf_mmap__read_event(&mut (*md).core);
        if event.is_null() {
            break;
        }

        let mut sample = core::mem::MaybeUninit::<perf_sample>::uninit();

        perf_sample__init(sample.as_mut_ptr(), false);
        if (*event).header.r#type == PERF_RECORD_SAMPLE {
            err = evlist__parse_sample(evlist, event, sample.as_mut_ptr());
            if err < 0 {
                pr_debug(c"Error during parse sample\n".as_ptr());
                perf_sample__exit(sample.as_mut_ptr());
                goto_out_put_evlist(evlist, cpus, threads);
                return err;
            }

            total_periods = total_periods.wrapping_add((*sample.as_ptr()).period);
            nr_samples += 1;
        }

        perf_mmap__consume(&mut (*md).core);
        perf_sample__exit(sample.as_mut_ptr());
    }
    perf_mmap__read_done(&mut (*md).core);

    if nr_samples as u64 == total_periods {
        pr_debug(
            c"All (%d) samples have period value of 1!\n".as_ptr(),
            nr_samples,
        );
        err = -1;
    }

    goto_out_put_evlist(evlist, cpus, threads);
    err
}

unsafe fn goto_out_put_evlist(
    evlist: *mut evlist,
    cpus: *mut perf_cpu_map,
    threads: *mut perf_thread_map,
) {
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
    evlist__put(evlist);
}

unsafe fn test__sw_clock_freq(test: *mut test_suite, subtest: c_int) -> c_int {
    let mut ret: c_int;

    let _ = test;
    let _ = subtest;

    ret = __test__sw_clock_freq(PERF_COUNT_SW_CPU_CLOCK);
    if ret == 0 {
        ret = __test__sw_clock_freq(PERF_COUNT_SW_TASK_CLOCK);
    }

    ret
}

// DEFINE_SUITE("Software clock events period values", sw_clock_freq);
