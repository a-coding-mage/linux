// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/mmap-basic.c.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type pid_t = c_int;
type ssize_t = isize;
type u64 = u64;
type __u64 = u64;

const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;
const EACCES: c_int = 13;
const O_RDWR: c_int = 0o2;
const STRERR_BUFSIZE: usize = 128;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_PMU_TYPE_SHIFT: u64 = 32;
const nsyscalls: usize = 3;

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
    pub exclusive: bool_,
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
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
}

#[repr(C)]
pub struct perf_evsel {
    pub attr: perf_event_attr,
    pub idx: c_uint,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
    pub config1: u64,
    pub wakeup_events: u32,
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
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub id: u64,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub type_: c_uint,
    pub cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub cap_user_rdpmc: u32,
    pub index: u32,
    pub pmc_width: u16,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
}

#[repr(C)]
pub struct cpu_set_t {
    bits: [c_ulong; 16],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn getpid() -> pid_t;
    fn getsid(pid: pid_t) -> pid_t;
    fn getppid() -> pid_t;
    fn getpgid(pid: pid_t) -> pid_t;
    fn rand() -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;

    fn thread_map__new_by_tid(tid: pid_t) -> *mut perf_thread_map;
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: pid_t);
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn cpu_map__set_affinity(cpus: *mut perf_cpu_map);

    fn evlist__new() -> *mut evlist;
    fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist;
    fn perf_evlist__set_maps(evlist: *mut perf_evlist, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);
    fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel;
    fn evsel__set_sample_id(evsel: *mut evsel, can_sample_identifier: bool_);
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__open(evsel: *mut evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_int) -> c_int;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn evlist__put(evlist: *mut evlist);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;

    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);
    fn perf_event__name(type_: u32) -> *const c_char;
    fn perf_sample__init(sample: *mut perf_sample, all: bool_);
    fn perf_sample__exit(sample: *mut perf_sample);

    fn perf_pmu__event_source_devices_fd() -> c_int;
    fn perf_pmu__pathname_fd(events_fd: c_int, name: *const c_char, filename: *const c_char, flags: c_int) -> c_int;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__supports_extended_type() -> bool_;
    fn perf_evsel__new(attr: *const perf_event_attr) -> *mut perf_evsel;
    fn perf_evsel__open(evsel: *mut perf_evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> c_int;
    fn perf_evsel__mmap(evsel: *mut perf_evsel, pages: c_int) -> c_int;
    fn perf_evsel__mmap_base(evsel: *mut perf_evsel, cpu_map_idx: c_int, thread: c_int) -> *mut perf_event_mmap_page;
    fn perf_evsel__read(evsel: *mut perf_evsel, cpu_map_idx: c_int, thread: c_int, counts: *mut perf_counts_values) -> c_int;
    fn perf_evsel__munmap(evsel: *mut perf_evsel);
    fn perf_evsel__close(evsel: *mut perf_evsel);
    fn perf_evsel__delete(evsel: *mut perf_evsel);

    fn pr_debug(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> isize;
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    (*set).bits = [0; 16];
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let bit = cpu as usize;
    (*set).bits[bit / (8 * core::mem::size_of::<c_ulong>())] |=
        (1 as c_ulong) << (bit % (8 * core::mem::size_of::<c_ulong>()));
}

unsafe extern "C" fn syscall_getsid() -> pid_t {
    getsid(0)
}

unsafe extern "C" fn syscall_getpgid() -> pid_t {
    getpgid(0)
}

/*
 * This test will generate random numbers of calls to some getpid syscalls,
 * then establish an mmap for a group of events that are created to monitor
 * the syscalls.
 *
 * It will receive the events, using mmap, use its PERF_SAMPLE_ID generated
 * sample.id field to map back to its respective perf_evsel instance.
 *
 * Then it checks if the number of syscalls reported as perf events by
 * the kernel corresponds to the number of syscalls made.
 */
unsafe extern "C" fn test__basic_mmap(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut err: c_int = TEST_FAIL;
    let mut event: *mut perf_event;
    let threads: *mut perf_thread_map;
    let cpus: *mut perf_cpu_map;
    let evlist: *mut evlist;
    let mut cpu_set = cpu_set_t { bits: [0; 16] };
    let syscall_names: [*const c_char; nsyscalls] = [c"getsid".as_ptr(), c"getppid".as_ptr(), c"getpgid".as_ptr()];
    let syscalls: [unsafe extern "C" fn() -> pid_t; nsyscalls] = [syscall_getsid, getppid, syscall_getpgid];
    let mut nr_events: [c_uint; nsyscalls] = [0; nsyscalls];
    let mut expected_nr_events: [c_uint; nsyscalls] = [0; nsyscalls];
    let mut i: c_uint;
    let mut j: c_uint;
    let mut evsels: [*mut evsel; nsyscalls] = [core::ptr::null_mut(); nsyscalls];
    let mut evsel: *mut evsel;
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
    let md: *mut mmap;

    threads = thread_map__new_by_tid(getpid());
    if threads.is_null() {
        pr_debug(c"thread_map__new\n".as_ptr());
        return -1;
    }

    cpus = perf_cpu_map__new_online_cpus();
    if cpus.is_null() {
        pr_debug(c"perf_cpu_map__new\n".as_ptr());
        perf_thread_map__put(threads);
        return err;
    }

    'out_free_cpus: {
        CPU_ZERO(&mut cpu_set);
        CPU_SET(perf_cpu_map__cpu(cpus, 0).cpu, &mut cpu_set);
        sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &cpu_set);
        if sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &cpu_set) < 0 {
            pr_debug(
                c"sched_setaffinity() failed on CPU %d: %s ".as_ptr(),
                perf_cpu_map__cpu(cpus, 0).cpu,
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            break 'out_free_cpus;
        }

        evlist = evlist__new();
        if evlist.is_null() {
            pr_debug(c"evlist__new\n".as_ptr());
            break 'out_free_cpus;
        }

        'out_put_evlist: {
            perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

            i = 0;
            while (i as usize) < nsyscalls {
                let mut name: [c_char; 64] = [0; 64];

                snprintf(name.as_mut_ptr(), name.len(), c"sys_enter_%s".as_ptr(), syscall_names[i as usize]);
                evsels[i as usize] = evsel__newtp(c"syscalls".as_ptr(), name.as_ptr());
                if IS_ERR(evsels[i as usize] as *const c_void) {
                    pr_debug(c"evsel__new(%s)\n".as_ptr(), name.as_ptr());
                    if PTR_ERR(evsels[i as usize] as *const c_void) == -(EACCES as isize) {
                        /* Permissions failure, flag the failure as a skip. */
                        err = TEST_SKIP;
                    }
                    break 'out_put_evlist;
                }

                (*evsels[i as usize]).core.attr.wakeup_events = 1;
                evsel__set_sample_id(evsels[i as usize], false);

                evlist__add(evlist, evsels[i as usize]);

                if evsel__open(evsels[i as usize], cpus, threads) < 0 {
                    pr_debug(
                        c"failed to open counter: %s, tweak /proc/sys/kernel/perf_event_paranoid?\n".as_ptr(),
                        str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
                    );
                    break 'out_put_evlist;
                }

                nr_events[i as usize] = 0;
                expected_nr_events[i as usize] = (1 + rand() % 127) as c_uint;
                i += 1;
            }

            if evlist__do_mmap(evlist, 128) < 0 {
                pr_debug(
                    c"failed to mmap events: %d (%s)\n".as_ptr(),
                    errno,
                    str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
                );
                break 'out_put_evlist;
            }

            i = 0;
            while (i as usize) < nsyscalls {
                j = 0;
                while j < expected_nr_events[i as usize] {
                    syscalls[i as usize]();
                    j += 1;
                }
                i += 1;
            }

            md = evlist__mmap(evlist).add(0);
            if perf_mmap__read_init(&mut (*md).core) >= 0 {
                loop {
                    event = perf_mmap__read_event(&mut (*md).core);
                    if event.is_null() {
                        break;
                    }
                    let mut sample = core::mem::MaybeUninit::<perf_sample>::uninit();

                    if (*event).header.type_ != PERF_RECORD_SAMPLE {
                        pr_debug(
                            c"unexpected %s event\n".as_ptr(),
                            perf_event__name((*event).header.type_),
                        );
                        break 'out_put_evlist;
                    }

                    perf_sample__init(sample.as_mut_ptr(), false);
                    let mut sample = sample.assume_init();
                    err = evlist__parse_sample(evlist, event, &mut sample);
                    if err != 0 {
                        pr_err(c"Can't parse sample, err = %d\n".as_ptr(), err);
                        perf_sample__exit(&mut sample);
                        break 'out_put_evlist;
                    }

                    err = -1;
                    evsel = sample.evsel;
                    if evsel.is_null() {
                        evsel = evlist__id2evsel(evlist, sample.id);
                    }
                    perf_sample__exit(&mut sample);
                    if evsel.is_null() {
                        pr_debug(
                            c"event with id %llu doesn't map to an evsel\n".as_ptr(),
                            sample.id,
                        );
                        break 'out_put_evlist;
                    }
                    nr_events[(*evsel).core.idx as usize] += 1;
                    perf_mmap__consume(&mut (*md).core);
                }
                perf_mmap__read_done(&mut (*md).core);
            }

            err = 0;
            i = 0;
            while (i as usize) < nsyscalls {
                evsel = evsels[i as usize];
                if !evsel.is_null()
                    && nr_events[(*evsel).core.idx as usize]
                        != expected_nr_events[(*evsel).core.idx as usize]
                {
                    pr_debug(
                        c"expected %d %s events, got %d\n".as_ptr(),
                        expected_nr_events[(*evsel).core.idx as usize],
                        evsel__name(evsel),
                        nr_events[(*evsel).core.idx as usize],
                    );
                    err = -1;
                    break 'out_put_evlist;
                }
                i += 1;
            }
        }
        evlist__put(evlist);
    }
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
    err
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum user_read_state {
    USER_READ_ENABLED,
    USER_READ_DISABLED,
    USER_READ_UNKNOWN,
}

unsafe fn set_user_read(pmu: *mut perf_pmu, enabled: user_read_state) -> user_read_state {
    let mut buf: [c_char; 2] = [0, b'\n' as c_char];
    let mut len: ssize_t;
    let events_fd: c_int;
    let rdpmc_fd: c_int;
    let mut old_user_read = user_read_state::USER_READ_UNKNOWN;

    if enabled == user_read_state::USER_READ_UNKNOWN {
        return user_read_state::USER_READ_UNKNOWN;
    }

    events_fd = perf_pmu__event_source_devices_fd();
    if events_fd < 0 {
        return user_read_state::USER_READ_UNKNOWN;
    }

    rdpmc_fd = perf_pmu__pathname_fd(events_fd, (*pmu).name, c"rdpmc".as_ptr(), O_RDWR);
    if rdpmc_fd < 0 {
        close(events_fd);
        return user_read_state::USER_READ_UNKNOWN;
    }

    len = read(rdpmc_fd, buf.as_mut_ptr() as *mut c_void, buf.len());
    if len != buf.len() as ssize_t {
        pr_debug(c"%s read failed\n".as_ptr(), c"set_user_read".as_ptr());
    }

    // Note, on Intel hybrid disabling on 1 PMU will implicitly disable on
    // all the core PMUs.
    old_user_read = if buf[0] == b'1' as c_char {
        user_read_state::USER_READ_ENABLED
    } else {
        user_read_state::USER_READ_DISABLED
    };

    if enabled != old_user_read {
        buf[0] = if enabled == user_read_state::USER_READ_ENABLED {
            b'1' as c_char
        } else {
            b'0' as c_char
        };
        len = write(rdpmc_fd, buf.as_ptr() as *const c_void, buf.len());
        if len != buf.len() as ssize_t {
            pr_debug(c"%s write failed\n".as_ptr(), c"set_user_read".as_ptr());
        }
    }
    close(rdpmc_fd);
    close(events_fd);
    old_user_read
}

unsafe fn test_stat_user_read(event: u64, enabled: user_read_state) -> c_int {
    let mut pmu: *mut perf_pmu = core::ptr::null_mut();
    let threads: *mut perf_thread_map = perf_thread_map__new_dummy();
    let mut ret: c_int = TEST_OK;

    pr_err(c"User space counter reading %llu\n".as_ptr(), event);
    if threads.is_null() {
        pr_err(c"User space counter reading [Failed to create threads]\n".as_ptr());
        return TEST_FAIL;
    }
    perf_thread_map__set_pid(threads, 0, 0);

    loop {
        pmu = perf_pmus__scan_core(pmu);
        if pmu.is_null() {
            break;
        }
        let saved_user_read_state = set_user_read(pmu, enabled);
        let mut attr = perf_event_attr {
            type_: PERF_TYPE_HARDWARE,
            config: if perf_pmus__supports_extended_type() {
                event | (((*pmu).type_ as u64) << PERF_PMU_TYPE_SHIFT)
            } else {
                event
            },
            // __aarch64__ C build sets config1 = 0x2 to request user access.
            #[cfg(target_arch = "aarch64")]
            config1: 0x2,
            #[cfg(not(target_arch = "aarch64"))]
            config1: 0,
            wakeup_events: 0,
        };
        let mut evsel: *mut perf_evsel = core::ptr::null_mut();
        let mut err: c_int;
        let pc: *mut perf_event_mmap_page;
        let mut mapped = false;
        let mut opened = false;
        let rdpmc_supported: bool_;
        let mut counts = perf_counts_values { val: 0 };

        pr_debug(c"User space counter reading for PMU %s\n".as_ptr(), (*pmu).name);
        /*
         * Restrict scheduling to only use the rdpmc on the CPUs the
         * event can be on. If the test doesn't run on the CPU of the
         * event then the event will be disabled and the pc->index test
         * will fail.
         */
        if !(*pmu).cpus.is_null() {
            cpu_map__set_affinity((*pmu).cpus);
        }

        /* Make the evsel. */
        evsel = perf_evsel__new(&attr);
        if evsel.is_null() {
            pr_err(
                c"User space counter reading for PMU %s [Failed to allocate evsel]\n".as_ptr(),
                (*pmu).name,
            );
            ret = TEST_FAIL;
        } else {
            'cleanup: {
                err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
                if err != 0 {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed to open evsel]\n".as_ptr(),
                        (*pmu).name,
                    );
                    ret = TEST_SKIP;
                    break 'cleanup;
                }
                opened = true;
                err = perf_evsel__mmap(evsel, 0);
                if err != 0 {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed to mmap evsel]\n".as_ptr(),
                        (*pmu).name,
                    );
                    ret = TEST_FAIL;
                    break 'cleanup;
                }
                mapped = true;

                pc = perf_evsel__mmap_base(evsel, 0, 0);
                if pc.is_null() {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed to get mmaped address]\n".as_ptr(),
                        (*pmu).name,
                    );
                    ret = TEST_FAIL;
                    break 'cleanup;
                }

                if saved_user_read_state == user_read_state::USER_READ_UNKNOWN {
                    rdpmc_supported = (*pc).cap_user_rdpmc != 0 && (*pc).index != 0;
                } else {
                    rdpmc_supported = enabled == user_read_state::USER_READ_ENABLED;
                }

                if rdpmc_supported && ((*pc).cap_user_rdpmc == 0 || (*pc).index == 0) {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed unexpected supported counter access %d %d]\n".as_ptr(),
                        (*pmu).name,
                        (*pc).cap_user_rdpmc,
                        (*pc).index,
                    );
                    ret = TEST_FAIL;
                    break 'cleanup;
                }

                if !rdpmc_supported && (*pc).cap_user_rdpmc != 0 {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed unexpected unsupported counter access %d]\n".as_ptr(),
                        (*pmu).name,
                        (*pc).cap_user_rdpmc,
                    );
                    ret = TEST_FAIL;
                    break 'cleanup;
                }

                if rdpmc_supported && (*pc).pmc_width < 32 {
                    pr_err(
                        c"User space counter reading for PMU %s [Failed width not set %d]\n".as_ptr(),
                        (*pmu).name,
                        (*pc).pmc_width as c_int,
                    );
                    ret = TEST_FAIL;
                    break 'cleanup;
                }

                perf_evsel__read(evsel, 0, 0, &mut counts);
                if rdpmc_supported && counts.val == 0 {
                    pr_err(c"User space counter reading for PMU %s [Failed read]\n".as_ptr(), (*pmu).name);
                    ret = TEST_FAIL;
                    break 'cleanup;
                }

                let mut i: c_int = 0;
                while i < 5 {
                    let mut count: c_int = 0x10000_i32 << i;
                    let start: __u64;
                    let end: __u64;
                    let mut last: __u64 = 0;

                    pr_debug(c"\tloop = %u, ".as_ptr(), count);

                    perf_evsel__read(evsel, 0, 0, &mut counts);
                    start = counts.val;

                    while {
                        let old = count;
                        count -= 1;
                        old != 0
                    } {}

                    perf_evsel__read(evsel, 0, 0, &mut counts);
                    end = counts.val;

                    if end.wrapping_sub(start) < last {
                        pr_err(
                            c"User space counter reading for PMU %s [Failed invalid counter data: end=%llu start=%llu last= %llu]\n".as_ptr(),
                            (*pmu).name,
                            end,
                            start,
                            last,
                        );
                        ret = TEST_FAIL;
                        break 'cleanup;
                    }
                    last = end.wrapping_sub(start);
                    pr_debug(c"count = %llu\n".as_ptr(), last);
                    i += 1;
                }
                pr_debug(c"User space counter reading for PMU %s [Success]\n".as_ptr(), (*pmu).name);
            }
        }
        if mapped {
            perf_evsel__munmap(evsel);
        }
        if opened {
            perf_evsel__close(evsel);
        }
        perf_evsel__delete(evsel);

        /* If the affinity was changed, then put it back to all CPUs. */
        if !(*pmu).cpus.is_null() {
            let cpus = cpu_map__online();

            cpu_map__set_affinity(cpus);
            perf_cpu_map__put(cpus);
        }
        set_user_read(pmu, saved_user_read_state);
    }
    perf_thread_map__put(threads);
    ret
}

unsafe extern "C" fn test__mmap_user_read_instr(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test_stat_user_read(PERF_COUNT_HW_INSTRUCTIONS, user_read_state::USER_READ_ENABLED)
}

unsafe extern "C" fn test__mmap_user_read_cycles(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test_stat_user_read(PERF_COUNT_HW_CPU_CYCLES, user_read_state::USER_READ_ENABLED)
}

unsafe extern "C" fn test__mmap_user_read_instr_disabled(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test_stat_user_read(PERF_COUNT_HW_INSTRUCTIONS, user_read_state::USER_READ_DISABLED)
}

unsafe extern "C" fn test__mmap_user_read_cycles_disabled(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test_stat_user_read(PERF_COUNT_HW_CPU_CYCLES, user_read_state::USER_READ_DISABLED)
}

// C preprocessor selected "permissions" on i386, x86_64, aarch64, and riscv64;
// otherwise it selected "unsupported".
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", all(target_arch = "riscv64", target_pointer_width = "64")))]
const USER_READ_REASON: *const c_char = c"permissions".as_ptr();
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", all(target_arch = "riscv64", target_pointer_width = "64"))))]
const USER_READ_REASON: *const c_char = c"unsupported".as_ptr();

#[unsafe(no_mangle)]
pub static mut tests__basic_mmap: [test_case; 6] = [
    test_case {
        name: c"Read samples using the mmap interface".as_ptr(),
        run_case: Some(test__basic_mmap),
        reason: c"permissions".as_ptr(),
        exclusive: false,
    },
    test_case {
        name: c"User space counter reading of instructions".as_ptr(),
        run_case: Some(test__mmap_user_read_instr),
        reason: USER_READ_REASON,
        exclusive: true,
    },
    test_case {
        name: c"User space counter reading of cycles".as_ptr(),
        run_case: Some(test__mmap_user_read_cycles),
        reason: USER_READ_REASON,
        exclusive: true,
    },
    test_case {
        name: c"User space counter disabling instructions".as_ptr(),
        run_case: Some(test__mmap_user_read_instr_disabled),
        reason: USER_READ_REASON,
        exclusive: true,
    },
    test_case {
        name: c"User space counter disabling cycles".as_ptr(),
        run_case: Some(test__mmap_user_read_cycles_disabled),
        reason: USER_READ_REASON,
        exclusive: true,
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
        reason: core::ptr::null(),
        exclusive: false,
    },
];

#[unsafe(no_mangle)]
pub static mut suite__basic_mmap: test_suite = test_suite {
    desc: c"mmap interface tests".as_ptr(),
    test_cases: unsafe { tests__basic_mmap.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
