// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/perf-time-to-tsc.c.
// C includes map to external declarations supplied by the surrounding perf tree.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type u64 = u64;
type pid_t = c_int;

const UINT_MAX: u32 = u32::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const PR_SET_NAME: c_int = 15;
const PERF_RECORD_COMM: u32 = 3;

// Except x86_64/i386 and Arm64, other archs don't support TSC in perf.  Just
// enable the test for x86_64/i386 and Arm64 archs.
#[cfg(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))]
const TSC_IS_SUPPORTED: c_int = 1;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
const TSC_IS_SUPPORTED: c_int = 0;

const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
}

#[repr(C)]
pub struct record_opts {
    pub mmap_pages: u32,
    pub user_freq: u32,
    pub user_interval: u64,
    pub target: target,
    pub sample_time: bool,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
}

#[repr(C)]
pub struct perf_evsel {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub comm: u64,
    pub disabled: u64,
    pub enable_on_exec: u64,
}

#[repr(C)]
pub struct perf_evlist {
    pub nr_mmaps: c_int,
}

#[repr(C)]
pub struct perf_tsc_conversion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_mmap_page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmap {
    pub core: perf_mmap,
}

#[repr(C)]
pub struct perf_mmap {
    pub base: *mut perf_event_mmap_page,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_comm {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub comm: perf_record_comm,
}

#[repr(C)]
pub struct perf_sample {
    pub time: u64,
}

unsafe extern "C" {
    fn getpid() -> pid_t;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);

    fn thread_map__new_by_tid(tid: pid_t) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist;
    fn perf_evlist__set_maps(
        evlist: *mut perf_evlist,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: u32) -> c_int;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;

    fn perf_read_tsc_conversion(
        pc: *mut perf_event_mmap_page,
        tc: *mut perf_tsc_conversion,
    ) -> c_int;
    fn rdtsc() -> u64;
    fn tsc_to_perf_time(tsc: u64, tc: *mut perf_tsc_conversion) -> u64;
    fn perf_time_to_tsc(time: u64, tc: *mut perf_tsc_conversion) -> u64;

    fn perf_mmap__read_init(md: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(md: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(md: *mut perf_mmap);
    fn perf_mmap__read_done(md: *mut perf_mmap);

    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evsel__parse_sample(
        evsel: *mut evsel,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;
}

macro_rules! CHECK__ {
    ($x:expr, $err_label:block) => {{
        while $x < 0 {
            pr_debug(concat!(stringify!($x), " failed!\n\0").as_ptr() as *const c_char);
            $err_label
        }
    }};
}

macro_rules! CHECK_NOT_NULL__ {
    ($x:expr, $err_label:block) => {{
        while ($x).is_null() {
            pr_debug(concat!(stringify!($x), " failed!\n\0").as_ptr() as *const c_char);
            $err_label
        }
    }};
}

unsafe extern "C" fn test__tsc_is_supported(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    if TSC_IS_SUPPORTED == 0 {
        pr_debug(c"Test not supported on this architecture\n".as_ptr());
        return TEST_SKIP;
    }

    TEST_OK
}

/**
 * test__perf_time_to_tsc - test converting perf time to TSC.
 *
 * This function implements a test that checks that the conversion of perf time
 * to and from TSC is consistent with the order of events.  If the test passes
 * %0 is returned, otherwise %-1 is returned.  If TSC conversion is not
 * supported then the test passes but " (not supported)" is printed.
 */
unsafe extern "C" fn test__perf_time_to_tsc(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut opts = record_opts {
        mmap_pages: UINT_MAX,
        user_freq: UINT_MAX,
        user_interval: ULLONG_MAX,
        target: target { uses_mmap: true },
        sample_time: true,
    };
    let mut threads: *mut perf_thread_map = ptr::null_mut();
    let mut cpus: *mut perf_cpu_map = ptr::null_mut();
    let mut evlist: *mut evlist = ptr::null_mut();
    let mut evsel: *mut evsel = ptr::null_mut();
    let mut err: c_int = TEST_FAIL;
    let mut ret: c_int;
    let mut i: c_int;
    let comm1: *const c_char;
    let comm2: *const c_char;
    let mut tc = core::mem::MaybeUninit::<perf_tsc_conversion>::uninit();
    let pc: *mut perf_event_mmap_page;
    let mut event: *mut perf_event;
    let test_tsc: u64;
    let comm1_tsc: u64;
    let comm2_tsc: u64;
    let test_time: u64;
    let mut comm1_time: u64 = 0;
    let mut comm2_time: u64 = 0;
    let mut md: *mut mmap;

    macro_rules! out_err {
        () => {{
            evlist__put(evlist);
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            return err;
        }};
    }

    threads = thread_map__new_by_tid(getpid());
    CHECK_NOT_NULL__!(threads, { out_err!() });

    cpus = perf_cpu_map__new_online_cpus();
    CHECK_NOT_NULL__!(cpus, { out_err!() });

    evlist = evlist__new();
    CHECK_NOT_NULL__!(evlist, { out_err!() });

    perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

    CHECK__!(parse_event(evlist, c"cpu-cycles:u".as_ptr()), { out_err!() });

    evlist__config(evlist, &mut opts, ptr::null_mut());

    /* For hybrid "cpu-cycles:u", it creates two events */
    // evlist__for_each_entry(evlist, evsel)
    // The iterator macro is provided by the surrounding C perf tree; this is
    // represented as direct pointer iteration in translated Rust.
    evsel = evlist__first_entry(evlist);
    while !evsel.is_null() {
        (*evsel).core.attr.comm = 1;
        (*evsel).core.attr.disabled = 1;
        (*evsel).core.attr.enable_on_exec = 0;
        evsel = evlist__next_entry(evlist, evsel);
    }

    ret = evlist__open(evlist);
    if ret < 0 {
        if ret == -ENOENT {
            err = TEST_SKIP;
        } else {
            pr_debug(c"evlist__open() failed\n".as_ptr());
        }
        out_err!();
    }

    CHECK__!(evlist__do_mmap(evlist, UINT_MAX), { out_err!() });

    pc = (*evlist__mmap(evlist).add(0)).core.base;
    ret = perf_read_tsc_conversion(pc, tc.as_mut_ptr());
    if ret != 0 {
        if ret == -EOPNOTSUPP {
            pr_debug(c"perf_read_tsc_conversion is not supported in current kernel\n".as_ptr());
            err = TEST_SKIP;
        }
        out_err!();
    }

    evlist__enable(evlist);

    comm1 = c"Test COMM 1".as_ptr();
    CHECK__!(
        prctl(PR_SET_NAME, comm1 as c_ulong, 0, 0, 0),
        { out_err!() }
    );

    test_tsc = rdtsc();

    comm2 = c"Test COMM 2".as_ptr();
    CHECK__!(
        prctl(PR_SET_NAME, comm2 as c_ulong, 0, 0, 0),
        { out_err!() }
    );

    evlist__disable(evlist);

    i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        md = evlist__mmap(evlist).add(i as usize);
        if perf_mmap__read_init(&mut (*md).core) < 0 {
            i += 1;
            continue;
        }

        loop {
            event = perf_mmap__read_event(&mut (*md).core);
            if event.is_null() {
                break;
            }

            let mut sample = core::mem::MaybeUninit::<perf_sample>::uninit();

            perf_sample__init(sample.as_mut_ptr(), false);
            if (*event).header.type_ != PERF_RECORD_COMM
                || (*event).comm.pid as pid_t != getpid()
                || (*event).comm.tid as pid_t != getpid()
            {
                perf_mmap__consume(&mut (*md).core);
                perf_sample__exit(sample.as_mut_ptr());
                continue;
            }

            if strcmp((*event).comm.comm.as_ptr(), comm1) == 0 {
                evsel = evlist__event2evsel(evlist, event);
                CHECK_NOT_NULL__!(evsel, {
                    perf_sample__exit(sample.as_mut_ptr());
                    out_err!()
                });
                CHECK__!(evsel__parse_sample(evsel, event, sample.as_mut_ptr()), {
                    perf_sample__exit(sample.as_mut_ptr());
                    out_err!()
                });
                comm1_time = (*sample.as_ptr()).time;
            }
            if strcmp((*event).comm.comm.as_ptr(), comm2) == 0 {
                evsel = evlist__event2evsel(evlist, event);
                CHECK_NOT_NULL__!(evsel, {
                    perf_sample__exit(sample.as_mut_ptr());
                    out_err!()
                });
                CHECK__!(evsel__parse_sample(evsel, event, sample.as_mut_ptr()), {
                    perf_sample__exit(sample.as_mut_ptr());
                    out_err!()
                });
                comm2_time = (*sample.as_ptr()).time;
            }

            perf_mmap__consume(&mut (*md).core);
            perf_sample__exit(sample.as_mut_ptr());
        }
        perf_mmap__read_done(&mut (*md).core);
        i += 1;
    }

    if comm1_time == 0 || comm2_time == 0 {
        out_err!();
    }

    test_time = tsc_to_perf_time(test_tsc, tc.as_mut_ptr());
    comm1_tsc = perf_time_to_tsc(comm1_time, tc.as_mut_ptr());
    comm2_tsc = perf_time_to_tsc(comm2_time, tc.as_mut_ptr());

    pr_debug(
        c"1st event perf time %llu tsc %llu\n".as_ptr(),
        comm1_time,
        comm1_tsc,
    );
    pr_debug(
        c"rdtsc          time %llu tsc %llu\n".as_ptr(),
        test_time,
        test_tsc,
    );
    pr_debug(
        c"2nd event perf time %llu tsc %llu\n".as_ptr(),
        comm2_time,
        comm2_tsc,
    );

    if test_time <= comm1_time || test_time >= comm2_time {
        out_err!();
    }

    if test_tsc <= comm1_tsc || test_tsc >= comm2_tsc {
        out_err!();
    }

    err = TEST_OK;

    out_err!();
}

unsafe extern "C" {
    // Rust representation of evlist__for_each_entry() iteration supplied by
    // the surrounding perf support code.
    fn evlist__first_entry(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next_entry(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

#[unsafe(no_mangle)]
pub static mut time_to_tsc_tests: [test_case; 3] = [
    test_case {
        name: c"TSC support".as_ptr(),
        run_case: Some(test__tsc_is_supported),
        reason: c"This architecture does not support".as_ptr(),
    },
    test_case {
        name: c"Perf time to TSC".as_ptr(),
        run_case: Some(test__perf_time_to_tsc),
        reason: c"perf_read_tsc_conversion is not supported".as_ptr(),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
        reason: ptr::null(),
    },
];

#[unsafe(no_mangle)]
pub static mut suite__perf_time_to_tsc: test_suite = test_suite {
    desc: c"Convert perf time to TSC".as_ptr(),
    test_cases: unsafe { time_to_tsc_tests.as_mut_ptr() },
};
