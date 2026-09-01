// Translated from lib/perf/Documentation/examples/sampling.c.
// Dependencies from the original includes:
// linux/perf_event.h, perf/evlist.h, perf/evsel.h, perf/cpumap.h,
// perf/threadmap.h, perf/mmap.h, perf/core.h, perf/event.h, stdio.h, unistd.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;
type va_list = *mut c_void;

const PERF_TYPE_HARDWARE: c_uint = 0;
const PERF_COUNT_HW_CPU_CYCLES: __u64 = 0;
const PERF_SAMPLE_IP: __u64 = 1 << 0;
const PERF_SAMPLE_TID: __u64 = 1 << 1;
const PERF_SAMPLE_PERIOD: __u64 = 1 << 8;
const PERF_SAMPLE_CPU: __u64 = 1 << 7;

#[repr(C)]
pub struct perf_evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_mmap {
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
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: __u64,
    pub sample_period_or_freq: __u64,
    pub sample_type: __u64,
    pub read_format: __u64,
    pub flags: __u64,
}

impl perf_event_attr {
    const fn new_for_sampling() -> Self {
        Self {
            type_: PERF_TYPE_HARDWARE,
            size: 0,
            config: PERF_COUNT_HW_CPU_CYCLES,
            sample_period_or_freq: 10,
            sample_type: PERF_SAMPLE_IP | PERF_SAMPLE_TID | PERF_SAMPLE_CPU | PERF_SAMPLE_PERIOD,
            read_format: 0,
            flags: (1 << 0) | (1 << 10),
        }
    }
}

#[repr(C)]
pub struct perf_event_sample {
    pub array: *mut __u64,
}

#[repr(C)]
pub union perf_event {
    pub sample: core::mem::ManuallyDrop<perf_event_sample>,
}

#[repr(C)]
union u64_swap {
    val64: __u64,
    val32: [__u32; 2],
}

#[repr(C)]
pub enum libperf_print_level {
    LIBPERF_ERR,
    LIBPERF_WARN,
    LIBPERF_INFO,
    LIBPERF_DEBUG,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;

    fn vfprintf(stream: *mut c_void, fmt: *const c_char, ap: va_list) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn libperf_init(print_cb: Option<unsafe extern "C" fn(libperf_print_level, *const c_char, va_list) -> c_int>);

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);

    fn perf_evlist__new() -> *mut perf_evlist;
    fn perf_evlist__delete(evlist: *mut perf_evlist);
    fn perf_evlist__add(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    fn perf_evlist__set_maps(
        evlist: *mut perf_evlist,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    );
    fn perf_evlist__open(evlist: *mut perf_evlist) -> c_int;
    fn perf_evlist__mmap(evlist: *mut perf_evlist, pages: c_int) -> c_int;
    fn perf_evlist__enable(evlist: *mut perf_evlist);
    fn perf_evlist__disable(evlist: *mut perf_evlist);

    fn perf_evsel__new(attr: *const perf_event_attr) -> *mut perf_evsel;

    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);

    fn perf_evlist__mmap_count(evlist: *mut perf_evlist) -> c_int;
    fn perf_evlist__mmap_by_idx(evlist: *mut perf_evlist, idx: c_int) -> *mut perf_mmap;
}

unsafe extern "C" fn libperf_print(
    _level: libperf_print_level,
    fmt: *const c_char,
    ap: va_list,
) -> c_int {
    unsafe { vfprintf(stderr, fmt, ap) }
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let mut map: *mut perf_mmap;
    let cpus: *mut perf_cpu_map;
    let attr = perf_event_attr::new_for_sampling();
    let mut err: c_int = -1;
    let mut event: *mut perf_event;

    unsafe {
        libperf_init(Some(libperf_print));

        cpus = perf_cpu_map__new_online_cpus();
        if cpus.is_null() {
            fprintf(stderr, c"failed to create cpus\n".as_ptr());
            return -1;
        }

        evlist = perf_evlist__new();
        if evlist.is_null() {
            fprintf(stderr, c"failed to create evlist\n".as_ptr());
            perf_cpu_map__put(cpus);
            return err;
        }

        evsel = perf_evsel__new(&attr);
        if evsel.is_null() {
            fprintf(stderr, c"failed to create cycles\n".as_ptr());
            perf_cpu_map__put(cpus);
            return err;
        }

        perf_evlist__add(evlist, evsel);

        perf_evlist__set_maps(evlist, cpus, core::ptr::null_mut());

        err = perf_evlist__open(evlist);
        if err != 0 {
            fprintf(stderr, c"failed to open evlist\n".as_ptr());
            perf_evlist__delete(evlist);
            perf_cpu_map__put(cpus);
            return err;
        }

        err = perf_evlist__mmap(evlist, 4);
        if err != 0 {
            fprintf(stderr, c"failed to mmap evlist\n".as_ptr());
            perf_evlist__delete(evlist);
            perf_cpu_map__put(cpus);
            return err;
        }

        perf_evlist__enable(evlist);
        sleep(3);
        perf_evlist__disable(evlist);

        // Translation of perf_evlist__for_each_mmap(evlist, map, false).
        let mmap_count = perf_evlist__mmap_count(evlist);
        let mut mmap_idx: c_int = 0;
        while mmap_idx < mmap_count {
            map = perf_evlist__mmap_by_idx(evlist, mmap_idx);

            if perf_mmap__read_init(map) < 0 {
                mmap_idx += 1;
                continue;
            }

            event = perf_mmap__read_event(map);
            while !event.is_null() {
                let cpu: c_int;
                let pid: c_int;
                let tid: c_int;
                let ip: __u64;
                let period: __u64;
                let mut array: *mut __u64;
                let mut u: u64_swap;

                array = (*event).sample.array;

                ip = *array;
                array = array.add(1);

                u.val64 = *array;
                pid = u.val32[0] as c_int;
                tid = u.val32[1] as c_int;
                array = array.add(1);

                u.val64 = *array;
                cpu = u.val32[0] as c_int;
                array = array.add(1);

                period = *array;

                fprintf(
                    stdout,
                    c"cpu %3d, pid %6d, tid %6d, ip %20llx, period %20llu\n".as_ptr(),
                    cpu,
                    pid,
                    tid,
                    ip as c_ulong,
                    period as c_ulong,
                );

                perf_mmap__consume(map);
                event = perf_mmap__read_event(map);
            }

            perf_mmap__read_done(map);
            mmap_idx += 1;
        }

        perf_evlist__delete(evlist);
        perf_cpu_map__put(cpus);
    }

    err
}

fn main() {
    let args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    let mut argv = args;
    let code = unsafe { main_impl(argv.len() as c_int, argv.as_mut_ptr()) };
    std::process::exit(code);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
