// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE for sched.h to get sched_[gs]etaffinity and CPU_(ZERO,SET).
// C includes removed; the declarations below are supplied by external perf/libc bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const EVENT_NUM: usize = 15;
const WAIT_COUNT: c_ulong = 100000000;
const PATH_MAX: usize = 4096;

type va_list = *mut c_void;
type size_t = usize;
type pid_t = c_int;
type s8 = i8;

const PERF_TYPE_HARDWARE: c_uint = 0;
const PERF_TYPE_SOFTWARE: c_uint = 1;
const PERF_TYPE_TRACEPOINT: c_uint = 2;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const PERF_COUNT_SW_TASK_CLOCK: u64 = 1;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
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
pub struct perf_evsel {
    pub leader: *mut perf_evsel,
}

#[repr(C)]
pub struct perf_mmap {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: u64,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events: u32,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u64; 16],
}

impl perf_event_attr {
    const fn zeroed() -> Self {
        Self {
            type_: 0,
            size: 0,
            config: 0,
            sample_period: 0,
            sample_type: 0,
            read_format: 0,
            flags: 0,
            wakeup_events: 0,
        }
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled {
            self.flags |= 1;
        } else {
            self.flags &= !1;
        }
    }

    fn set_wakeup_watermark(&mut self, wakeup_watermark: u32) {
        self.flags |= 1 << 14;
        self.wakeup_events = wakeup_watermark;
    }
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut tests_failed: c_int;

    fn vfprintf(stream: *mut c_void, fmt: *const c_char, ap: va_list) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn sched_getaffinity(pid: pid_t, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);

    fn sysfs__mountpoint() -> *const c_char;
    fn filename__read_int(filename: *const c_char, value: *mut c_int) -> c_int;

    fn libperf_init(print_fn: Option<unsafe extern "C" fn(libperf_print_level, *const c_char, va_list) -> c_int>);

    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;

    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: pid_t);
    fn perf_thread_map__put(threads: *mut perf_thread_map);

    fn perf_evlist__new() -> *mut perf_evlist;
    fn perf_evlist__add(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    fn perf_evlist__set_leader(evlist: *mut perf_evlist);
    fn perf_evlist__set_maps(evlist: *mut perf_evlist, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);
    fn perf_evlist__open(evlist: *mut perf_evlist) -> c_int;
    fn perf_evlist__close(evlist: *mut perf_evlist);
    fn perf_evlist__delete(evlist: *mut perf_evlist);
    fn perf_evlist__enable(evlist: *mut perf_evlist);
    fn perf_evlist__disable(evlist: *mut perf_evlist);
    fn perf_evlist__mmap(evlist: *mut perf_evlist, pages: c_int) -> c_int;
    fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel;
    fn perf_evlist__next(evlist: *mut perf_evlist, evsel: *mut perf_evsel) -> *mut perf_evsel;
    fn perf_evlist__first_mmap(evlist: *mut perf_evlist, overwrite: bool) -> *mut perf_mmap;
    fn perf_evlist__next_mmap(evlist: *mut perf_evlist, map: *mut perf_mmap, overwrite: bool) -> *mut perf_mmap;

    fn perf_evsel__new(attr: *const perf_event_attr) -> *mut perf_evsel;
    fn perf_evsel__cpus(evsel: *mut perf_evsel) -> *mut perf_cpu_map;
    fn perf_evsel__read(evsel: *mut perf_evsel, cpu: c_int, thread: c_int, counts: *mut perf_counts_values) -> c_int;
    fn perf_evsel__open(evsel: *mut perf_evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> c_int;
    fn perf_evsel__enable(evsel: *mut perf_evsel) -> c_int;
    fn perf_evsel__disable(evsel: *mut perf_evsel) -> c_int;
    fn perf_evsel__close(evsel: *mut perf_evsel);
    fn perf_evsel__delete(evsel: *mut perf_evsel);

    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);

    fn perf_counts_values__scale(counts: *mut perf_counts_values, scale: bool, scaled: *mut s8);
}

#[repr(C)]
pub enum libperf_print_level {
    LIBPERF_ERR = 0,
    LIBPERF_WARN = 1,
    LIBPERF_INFO = 2,
    LIBPERF_DEBUG = 3,
    LIBPERF_DEBUG2 = 4,
    LIBPERF_DEBUG3 = 5,
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn __T(_msg: *const c_char, cond: bool) {
    if !cond {
        tests_failed += 1;
    }
}

unsafe fn __T_VERBOSE(fmt: *const c_char, args: impl FnOnce(*const c_char)) {
    args(fmt);
}

unsafe extern "C" fn libperf_print(_level: libperf_print_level, fmt: *const c_char, ap: va_list) -> c_int {
    vfprintf(stderr, fmt, ap)
}

unsafe fn test_stat_cpu() -> c_int {
    let mut cpus: *mut perf_cpu_map;
    let evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let leader: *mut perf_evsel;
    let mut attr1 = perf_event_attr::zeroed();
    attr1.type_ = PERF_TYPE_SOFTWARE;
    attr1.config = PERF_COUNT_SW_CPU_CLOCK;
    let mut attr2 = perf_event_attr::zeroed();
    attr2.type_ = PERF_TYPE_SOFTWARE;
    attr2.config = PERF_COUNT_SW_TASK_CLOCK;
    let err: c_int;

    cpus = perf_cpu_map__new_online_cpus();
    __T(c_str!("failed to create cpus"), !cpus.is_null());

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    evsel = perf_evsel__new(&attr1);
    leader = evsel;
    __T(c_str!("failed to create evsel1"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    evsel = perf_evsel__new(&attr2);
    __T(c_str!("failed to create evsel2"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    perf_evlist__set_leader(evlist);
    __T(c_str!("failed to set leader"), (*leader).leader == leader);
    __T(c_str!("failed to set leader"), (*evsel).leader == leader);

    perf_evlist__set_maps(evlist, cpus, core::ptr::null_mut());

    err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        cpus = perf_evsel__cpus(evsel);

        let mut idx = 0;
        while idx < perf_cpu_map__nr(cpus) {
            let mut counts = perf_counts_values { val: 0, ena: 0, run: 0 };

            perf_evsel__read(evsel, idx, 0, &mut counts);
            __T(c_str!("failed to read value for evsel"), counts.val != 0);
            idx += 1;
        }

        evsel = perf_evlist__next(evlist, evsel);
    }

    perf_evlist__close(evlist);
    perf_evlist__delete(evlist);

    perf_cpu_map__put(cpus);
    0
}

unsafe fn test_stat_thread() -> c_int {
    let mut counts = perf_counts_values { val: 0, ena: 0, run: 0 };
    let threads: *mut perf_thread_map;
    let evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let leader: *mut perf_evsel;
    let mut attr1 = perf_event_attr::zeroed();
    attr1.type_ = PERF_TYPE_SOFTWARE;
    attr1.config = PERF_COUNT_SW_CPU_CLOCK;
    let mut attr2 = perf_event_attr::zeroed();
    attr2.type_ = PERF_TYPE_SOFTWARE;
    attr2.config = PERF_COUNT_SW_TASK_CLOCK;
    let err: c_int;

    threads = perf_thread_map__new_dummy();
    __T(c_str!("failed to create threads"), !threads.is_null());

    perf_thread_map__set_pid(threads, 0, 0);

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    evsel = perf_evsel__new(&attr1);
    leader = evsel;
    __T(c_str!("failed to create evsel1"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    evsel = perf_evsel__new(&attr2);
    __T(c_str!("failed to create evsel2"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    perf_evlist__set_leader(evlist);
    __T(c_str!("failed to set leader"), (*leader).leader == leader);
    __T(c_str!("failed to set leader"), (*evsel).leader == leader);

    perf_evlist__set_maps(evlist, core::ptr::null_mut(), threads);

    err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        perf_evsel__read(evsel, 0, 0, &mut counts);
        __T(c_str!("failed to read value for evsel"), counts.val != 0);
        evsel = perf_evlist__next(evlist, evsel);
    }

    perf_evlist__close(evlist);
    perf_evlist__delete(evlist);

    perf_thread_map__put(threads);
    0
}

unsafe fn test_stat_thread_enable() -> c_int {
    let mut counts = perf_counts_values { val: 0, ena: 0, run: 0 };
    let threads: *mut perf_thread_map;
    let evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let leader: *mut perf_evsel;
    let mut attr1 = perf_event_attr::zeroed();
    attr1.type_ = PERF_TYPE_SOFTWARE;
    attr1.config = PERF_COUNT_SW_CPU_CLOCK;
    attr1.set_disabled(true);
    let mut attr2 = perf_event_attr::zeroed();
    attr2.type_ = PERF_TYPE_SOFTWARE;
    attr2.config = PERF_COUNT_SW_TASK_CLOCK;
    attr2.set_disabled(true);
    let err: c_int;

    threads = perf_thread_map__new_dummy();
    __T(c_str!("failed to create threads"), !threads.is_null());

    perf_thread_map__set_pid(threads, 0, 0);

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    evsel = perf_evsel__new(&attr1);
    leader = evsel;
    __T(c_str!("failed to create evsel1"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    evsel = perf_evsel__new(&attr2);
    __T(c_str!("failed to create evsel2"), !evsel.is_null());

    perf_evlist__add(evlist, evsel);

    perf_evlist__set_leader(evlist);
    __T(c_str!("failed to set leader"), (*leader).leader == leader);
    __T(c_str!("failed to set leader"), (*evsel).leader == leader);

    perf_evlist__set_maps(evlist, core::ptr::null_mut(), threads);

    err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        perf_evsel__read(evsel, 0, 0, &mut counts);
        __T(c_str!("failed to read value for evsel"), counts.val == 0);
        evsel = perf_evlist__next(evlist, evsel);
    }

    perf_evlist__enable(evlist);

    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        perf_evsel__read(evsel, 0, 0, &mut counts);
        __T(c_str!("failed to read value for evsel"), counts.val != 0);
        evsel = perf_evlist__next(evlist, evsel);
    }

    perf_evlist__disable(evlist);

    perf_evlist__close(evlist);
    perf_evlist__delete(evlist);

    perf_thread_map__put(threads);
    0
}

unsafe fn test_mmap_thread() -> c_int {
    let evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let mut map: *mut perf_mmap;
    let cpus: *mut perf_cpu_map;
    let threads: *mut perf_thread_map;
    let mut attr = perf_event_attr::zeroed();
    attr.type_ = PERF_TYPE_TRACEPOINT;
    attr.sample_period = 1;
    attr.set_wakeup_watermark(1);
    attr.set_disabled(true);
    let mut path = [0 as c_char; PATH_MAX];
    let mut id: c_int = 0;
    let mut go_pipe = [0 as c_int; 2];
    let mut event: *mut perf_event;
    let mut count: c_int = 0;

    snprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c_str!("%s/kernel/debug/tracing/events/syscalls/sys_enter_prctl/id"),
        sysfs__mountpoint(),
    );

    if filename__read_int(path.as_ptr(), &mut id) != 0 {
        tests_failed += 1;
        fprintf(stderr, c_str!("error: failed to get tracepoint id: %s\n"), path.as_ptr());
        return -1;
    }

    attr.config = id as u64;

    let mut err = pipe(go_pipe.as_mut_ptr());
    __T(c_str!("failed to create pipe"), err == 0);

    fflush(core::ptr::null_mut());

    let pid = fork();
    if pid == 0 {
        let mut i: c_int = 0;
        let mut bf: c_char = 0;

        read(go_pipe[0], &mut bf as *mut _ as *mut c_void, 1);

        /* Generate 100 prctl calls. */
        while i < 100 {
            prctl(0, 0, 0, 0, 0);
            i += 1;
        }

        exit(0);
    }

    threads = perf_thread_map__new_dummy();
    __T(c_str!("failed to create threads"), !threads.is_null());

    cpus = perf_cpu_map__new_any_cpu();
    __T(c_str!("failed to create cpus"), !cpus.is_null());

    perf_thread_map__set_pid(threads, 0, pid);

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    evsel = perf_evsel__new(&attr);
    __T(c_str!("failed to create evsel1"), !evsel.is_null());
    __T(c_str!("failed to set leader"), (*evsel).leader == evsel);

    perf_evlist__add(evlist, evsel);

    perf_evlist__set_maps(evlist, cpus, threads);

    err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    err = perf_evlist__mmap(evlist, 4);
    __T(c_str!("failed to mmap evlist"), err == 0);

    perf_evlist__enable(evlist);

    /* kick the child and wait for it to finish */
    write(go_pipe[1], c_str!("A") as *const c_void, 1);
    waitpid(pid, core::ptr::null_mut(), 0);

    /*
     * There's no need to call perf_evlist__disable,
     * monitored process is dead now.
     */

    map = perf_evlist__first_mmap(evlist, false);
    while !map.is_null() {
        if perf_mmap__read_init(map) < 0 {
            map = perf_evlist__next_mmap(evlist, map, false);
            continue;
        }

        loop {
            event = perf_mmap__read_event(map);
            if event.is_null() {
                break;
            }
            count += 1;
            perf_mmap__consume(map);
        }

        perf_mmap__read_done(map);
        map = perf_evlist__next_mmap(evlist, map, false);
    }

    /* calls perf_evlist__munmap/perf_evlist__close */
    perf_evlist__delete(evlist);

    perf_thread_map__put(threads);
    perf_cpu_map__put(cpus);

    /*
     * The generated prctl calls should match the
     * number of events in the buffer.
     */
    __T(c_str!("failed count"), count == 100);

    0
}

unsafe fn test_mmap_cpus() -> c_int {
    let evlist: *mut perf_evlist;
    let evsel: *mut perf_evsel;
    let mut map: *mut perf_mmap;
    let cpus: *mut perf_cpu_map;
    let mut attr = perf_event_attr::zeroed();
    attr.type_ = PERF_TYPE_TRACEPOINT;
    attr.sample_period = 1;
    attr.set_wakeup_watermark(1);
    attr.set_disabled(true);
    let mut saved_mask: cpu_set_t = core::mem::zeroed();
    let mut path = [0 as c_char; PATH_MAX];
    let mut id: c_int = 0;
    let mut tmp: c_int;
    let mut cpu: perf_cpu;
    let mut event: *mut perf_event;
    let mut count: c_int = 0;

    snprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c_str!("%s/kernel/debug/tracing/events/syscalls/sys_enter_prctl/id"),
        sysfs__mountpoint(),
    );

    if filename__read_int(path.as_ptr(), &mut id) != 0 {
        fprintf(stderr, c_str!("error: failed to get tracepoint id: %s\n"), path.as_ptr());
        return -1;
    }

    attr.config = id as u64;

    cpus = perf_cpu_map__new_online_cpus();
    __T(c_str!("failed to create cpus"), !cpus.is_null());

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    evsel = perf_evsel__new(&attr);
    __T(c_str!("failed to create evsel1"), !evsel.is_null());
    __T(c_str!("failed to set leader"), (*evsel).leader == evsel);

    perf_evlist__add(evlist, evsel);

    perf_evlist__set_maps(evlist, cpus, core::ptr::null_mut());

    let mut err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    err = perf_evlist__mmap(evlist, 4);
    __T(c_str!("failed to mmap evlist"), err == 0);

    perf_evlist__enable(evlist);

    err = sched_getaffinity(0, core::mem::size_of::<cpu_set_t>(), &mut saved_mask);
    __T(c_str!("sched_getaffinity failed"), err == 0);

    tmp = 0;
    while tmp < perf_cpu_map__nr(cpus) {
        let mut mask: cpu_set_t = core::mem::zeroed();

        cpu = perf_cpu_map__cpu(cpus, tmp);
        CPU_ZERO(&mut mask);
        CPU_SET(cpu.cpu, &mut mask);

        err = sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &mask);
        __T(c_str!("sched_setaffinity failed"), err == 0);

        prctl(0, 0, 0, 0, 0);
        tmp += 1;
    }

    err = sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &saved_mask);
    __T(c_str!("sched_setaffinity failed"), err == 0);

    perf_evlist__disable(evlist);

    map = perf_evlist__first_mmap(evlist, false);
    while !map.is_null() {
        if perf_mmap__read_init(map) < 0 {
            map = perf_evlist__next_mmap(evlist, map, false);
            continue;
        }

        loop {
            event = perf_mmap__read_event(map);
            if event.is_null() {
                break;
            }
            count += 1;
            perf_mmap__consume(map);
        }

        perf_mmap__read_done(map);
        map = perf_evlist__next_mmap(evlist, map, false);
    }

    /* calls perf_evlist__munmap/perf_evlist__close */
    perf_evlist__delete(evlist);

    /*
     * The generated prctl events should match the
     * number of cpus or be bigger (we are system-wide).
     */
    __T(c_str!("failed count"), count >= perf_cpu_map__nr(cpus));

    perf_cpu_map__put(cpus);

    0
}

unsafe fn display_error(average: i64, high: i64, low: i64, expected: i64) -> f64 {
    let error: f64;

    error = (((average as f64) - expected as f64) / expected as f64) * 100.0;

    __T_VERBOSE(c_str!("   Expected: %lld\n"), |fmt| {
        fprintf(stderr, fmt, expected);
    });
    __T_VERBOSE(c_str!("   High: %lld   Low:  %lld   Average:  %lld\n"), |fmt| {
        fprintf(stderr, fmt, high, low, average);
    });

    __T_VERBOSE(c_str!("   Average Error = %.2f%%\n"), |fmt| {
        fprintf(stderr, fmt, error);
    });

    error
}

unsafe fn test_stat_multiplexing() -> c_int {
    let mut expected_counts = perf_counts_values { val: 0, ena: 0, run: 0 };
    let mut counts = [perf_counts_values { val: 0, ena: 0, run: 0 }; EVENT_NUM];
    let mut threads: *mut perf_thread_map;
    let evlist: *mut perf_evlist;
    let mut evsel: *mut perf_evsel;
    let mut attr = perf_event_attr::zeroed();
    attr.type_ = PERF_TYPE_HARDWARE;
    attr.config = PERF_COUNT_HW_INSTRUCTIONS;
    attr.read_format = PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
    attr.set_disabled(true);
    let mut err: c_int;
    let mut i: c_int;
    let mut nonzero: c_int = 0;
    let mut count: c_ulong;
    let mut max: i64 = 0;
    let mut min: i64 = 0;
    let mut avg: i64 = 0;
    let error: f64;
    let mut scaled: s8 = 0;

    /* read for non-multiplexing event count */
    threads = perf_thread_map__new_dummy();
    __T(c_str!("failed to create threads"), !threads.is_null());

    perf_thread_map__set_pid(threads, 0, 0);

    evsel = perf_evsel__new(&attr);
    __T(c_str!("failed to create evsel"), !evsel.is_null());

    err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
    __T(c_str!("failed to open evsel"), err == 0);

    err = perf_evsel__enable(evsel);
    __T(c_str!("failed to enable evsel"), err == 0);

    /* wait loop */
    count = WAIT_COUNT;
    while count != 0 {
        count -= 1;
    }

    perf_evsel__read(evsel, 0, 0, &mut expected_counts);
    __T(c_str!("failed to read value for evsel"), expected_counts.val != 0);
    __T(
        c_str!("failed to read non-multiplexing event count"),
        expected_counts.ena == expected_counts.run,
    );

    err = perf_evsel__disable(evsel);
    __T(c_str!("failed to enable evsel"), err == 0);

    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);

    perf_thread_map__put(threads);

    /* read for multiplexing event count */
    threads = perf_thread_map__new_dummy();
    __T(c_str!("failed to create threads"), !threads.is_null());

    perf_thread_map__set_pid(threads, 0, 0);

    evlist = perf_evlist__new();
    __T(c_str!("failed to create evlist"), !evlist.is_null());

    i = 0;
    while i < EVENT_NUM as c_int {
        evsel = perf_evsel__new(&attr);
        __T(c_str!("failed to create evsel"), !evsel.is_null());

        perf_evlist__add(evlist, evsel);
        i += 1;
    }
    perf_evlist__set_maps(evlist, core::ptr::null_mut(), threads);

    err = perf_evlist__open(evlist);
    __T(c_str!("failed to open evlist"), err == 0);

    perf_evlist__enable(evlist);

    /* wait loop */
    count = WAIT_COUNT;
    while count != 0 {
        count -= 1;
    }

    i = 0;
    evsel = perf_evlist__first(evlist);
    while !evsel.is_null() {
        perf_evsel__read(evsel, 0, 0, &mut counts[i as usize]);
        __T(c_str!("failed to read value for evsel"), counts[i as usize].val != 0);
        i += 1;
        evsel = perf_evlist__next(evlist, evsel);
    }

    perf_evlist__disable(evlist);

    min = counts[0].val as i64;
    i = 0;
    while i < EVENT_NUM as c_int {
        __T_VERBOSE(
            c_str!("Event %2d -- Raw count = %lu, run = %lu, enable = %lu\n"),
            |fmt| {
                fprintf(
                    stderr,
                    fmt,
                    i,
                    counts[i as usize].val,
                    counts[i as usize].run,
                    counts[i as usize].ena,
                );
            },
        );

        perf_counts_values__scale(&mut counts[i as usize], true, &mut scaled);
        if scaled == 1 {
            __T_VERBOSE(
                c_str!("\t Scaled count = %lu (%.2lf%%, %lu/%lu)\n"),
                |fmt| {
                    fprintf(
                        stderr,
                        fmt,
                        counts[i as usize].val,
                        (counts[i as usize].run as f64) / (counts[i as usize].ena as f64) * 100.0,
                        counts[i as usize].run,
                        counts[i as usize].ena,
                    );
                },
            );
        } else if scaled == -1 {
            __T_VERBOSE(c_str!("\t Not Running\n"), |fmt| {
                fprintf(stderr, fmt);
            });
        } else {
            __T_VERBOSE(c_str!("\t Not Scaling\n"), |fmt| {
                fprintf(stderr, fmt);
            });
        }

        if (counts[i as usize].val as i64) > max {
            max = counts[i as usize].val as i64;
        }

        if (counts[i as usize].val as i64) < min {
            min = counts[i as usize].val as i64;
        }

        avg += counts[i as usize].val as i64;

        if counts[i as usize].val != 0 {
            nonzero += 1;
        }
        i += 1;
    }

    if nonzero != 0 {
        avg = avg / nonzero as i64;
    } else {
        avg = 0;
    }

    error = display_error(avg, max, min, expected_counts.val as i64);

    __T(c_str!("Error out of range!"), error <= 1.0 && error >= -1.0);

    perf_evlist__close(evlist);
    perf_evlist__delete(evlist);

    perf_thread_map__put(threads);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_evlist(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    tests_failed = 0;

    libperf_init(Some(libperf_print));

    test_stat_cpu();
    test_stat_thread();
    test_stat_thread_enable();
    test_mmap_thread();
    test_mmap_cpus();
    test_stat_multiplexing();

    if tests_failed == 0 { 0 } else { -1 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
