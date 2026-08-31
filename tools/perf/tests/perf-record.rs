// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/perf-record.c.
// C includes removed; referenced perf/libc symbols are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;
type size_t = usize;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub desc: *const c_char,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct target {
    pub uses_mmap: bool,
}

#[repr(C)]
pub struct record_opts {
    pub target: target,
    pub no_buffering: bool,
    pub mmap_pages: c_int,
}

#[repr(C)]
pub struct cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
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
pub struct perf_sample {
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub cpu: u32,
}

#[repr(C)]
pub struct evlist_core {
    pub nr_mmaps: c_int,
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
pub struct perf_event_comm {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub comm: [c_char; 0],
}

#[repr(C)]
pub struct perf_event_mmap {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub filename: [c_char; 0],
}

#[repr(C)]
pub struct perf_event_mmap2 {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub filename: [c_char; 0],
}

#[repr(C)]
pub union perf_event {
    pub header: core::mem::ManuallyDrop<perf_event_header>,
    pub comm: core::mem::ManuallyDrop<perf_event_comm>,
    pub mmap: core::mem::ManuallyDrop<perf_event_mmap>,
    pub mmap2: core::mem::ManuallyDrop<perf_event_mmap2>,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut stderr: *mut c_void;

    fn cpu__max_cpu() -> cpu;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_FREE(cpusetp: *mut cpu_set_t);
    fn CPU_ISSET_S(cpu: c_int, setsize: size_t, cpusetp: *const cpu_set_t) -> c_int;
    fn CPU_CLR_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn sched_getaffinity(pid: pid_t, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn perror(s: *const c_char);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sleep(seconds: c_int) -> c_int;

    fn evlist__new_dummy() -> *mut evlist;
    fn evlist__new_default(target: *const target, sample_callchains: bool) -> *mut evlist;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__prepare_workload(
        evlist: *mut evlist,
        target: *mut target,
        argv: *const *const c_char,
        pipe_output: bool,
        output: *mut c_void,
    ) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__workload_pid(evlist: *mut evlist) -> pid_t;
    fn evlist__cancel_workload(evlist: *mut evlist);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_int) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__start_workload(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int;
    fn evlist__put(evlist: *mut evlist);

    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_mmap__read_init(map: *mut perf_mmap) -> c_int;
    fn perf_mmap__read_event(map: *mut perf_mmap) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut perf_mmap);
    fn perf_mmap__read_done(map: *mut perf_mmap);
    fn perf_event__name(type_: u32) -> *const c_char;
    fn perf_event__fprintf(event: *mut perf_event, attr: *mut c_void, fp: *mut c_void);

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const STRERR_BUFSIZE: usize = 128;
const TEST_SKIP: c_int = 2;
const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;

const CPU: c_int = 0;
const TID: c_int = 1;
const TIME: c_int = 2;

const PERF_RECORD_MMAP: u32 = 1;
const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_EXIT: u32 = 4;
const PERF_RECORD_FORK: u32 = 7;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MMAP2: u32 = 10;
const PERF_RECORD_MAX: usize = 64;

unsafe fn sched__get_first_possible_cpu(pid: pid_t, maskp: *mut cpu_set_t) -> c_int {
    let mut i: c_int;
    let mut cpu: c_int = -1;
    let mut nrcpus: c_int = cpu__max_cpu().cpu;
    let mut size: size_t = CPU_ALLOC_SIZE(nrcpus);

    loop {
        CPU_ZERO_S(size, maskp);

        if sched_getaffinity(pid, size, maskp) == -1 {
            if errno == EINVAL && nrcpus < (cpu__max_cpu().cpu << 8) {
                nrcpus = nrcpus << 2;
                size = CPU_ALLOC_SIZE(nrcpus);
                continue;
            }
            perror(c"sched_getaffinity".as_ptr());
            return -1;
        }

        break;
    }

    i = 0;
    while i < nrcpus {
        if CPU_ISSET_S(i, size, maskp) != 0 {
            if cpu == -1 {
                cpu = i;
            } else {
                CPU_CLR_S(i, size, maskp);
            }
        }
        i += 1;
    }

    cpu
}

unsafe extern "C" fn test__PERF_RECORD(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut opts = record_opts {
        target: target { uses_mmap: true },
        no_buffering: true,
        mmap_pages: 256,
    };
    let nrcpus: c_int = cpu__max_cpu().cpu;
    let mut cpu_mask: *mut cpu_set_t;
    let cpu_mask_size: size_t;
    let mut evlist: *mut evlist = evlist__new_dummy();
    let mut evsel: *mut evsel;
    let mut sample: perf_sample = core::mem::zeroed();
    let cmd: *const c_char = c"sleep".as_ptr();
    let argv: [*const c_char; 3] = [cmd, c"1".as_ptr(), core::ptr::null()];
    let mut bname: *mut c_char;
    let mut mmap_filename: *mut c_char;
    let mut prev_time: u64 = 0;
    let mut found_cmd_mmap: bool = false;
    let mut found_coreutils_mmap: bool = false;
    let mut found_libc_mmap: bool = false;
    let mut found_vdso_mmap: bool = false;
    let mut found_ld_mmap: bool = false;
    let mut err: c_int = -1;
    let mut errs: c_int = 0;
    let mut i: c_int;
    let mut wakeups: c_int = 0;
    let cpu: u32;
    let mut total_events: c_int = 0;
    let mut nr_events: [c_int; PERF_RECORD_MAX] = [0; PERF_RECORD_MAX];
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];

    cpu_mask = CPU_ALLOC(nrcpus);
    if cpu_mask.is_null() {
        pr_debug(c"failed to create cpumask\n".as_ptr());
        goto_out(&mut sample, evlist, err, errs);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    cpu_mask_size = CPU_ALLOC_SIZE(nrcpus);
    CPU_ZERO_S(cpu_mask_size, cpu_mask);

    perf_sample__init(&mut sample, false);
    if evlist.is_null() {
        /* Fallback for kernels lacking PERF_COUNT_SW_DUMMY */
        let target: target = core::mem::zeroed();

        evlist = evlist__new_default(&target, false);
    }

    if evlist.is_null() {
        pr_debug(c"Not enough memory to create evlist\n".as_ptr());
        CPU_FREE(cpu_mask);
        goto_out(&mut sample, evlist, err, errs);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * Create maps of threads and cpus to monitor. In this case
     * we start with all threads and cpus (-1, -1) but then in
     * evlist__prepare_workload we'll fill in the only thread
     * we're monitoring, the one forked there.
     */
    err = evlist__create_maps(evlist, &mut opts.target);
    if err < 0 {
        pr_debug(c"Not enough memory to create thread/cpu maps\n".as_ptr());
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * Prepare the workload in argv[] to run, it'll fork it, and then wait
     * for evlist__start_workload() to exec it. This is done this way
     * so that we have time to open the evlist (calling sys_perf_event_open
     * on all the fds) and then mmap them.
     */
    err = evlist__prepare_workload(evlist, &mut opts.target, argv.as_ptr(), false, core::ptr::null_mut());
    if err < 0 {
        pr_debug(c"Couldn't run the workload!\n".as_ptr());
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * Config the evsels, setting attr->comm on the first one, etc.
     */
    evsel = evlist__first(evlist);
    evsel__set_sample_bit(evsel, CPU);
    evsel__set_sample_bit(evsel, TID);
    evsel__set_sample_bit(evsel, TIME);
    evlist__config(evlist, &mut opts, core::ptr::null_mut());

    err = sched__get_first_possible_cpu(evlist__workload_pid(evlist), cpu_mask);
    if err < 0 {
        pr_debug(
            c"sched__get_first_possible_cpu: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        evlist__cancel_workload(evlist);
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    cpu = err as u32;

    /*
     * So that we can check perf_sample.cpu on all the samples.
     */
    if sched_setaffinity(evlist__workload_pid(evlist), cpu_mask_size, cpu_mask) < 0 {
        pr_debug(
            c"sched_setaffinity: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        evlist__cancel_workload(evlist);
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * Call sys_perf_event_open on all the fds on all the evsels,
     * grouping them if asked to.
     */
    err = evlist__open(evlist);
    if err < 0 {
        pr_debug(
            c"perf_evlist__open: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        evlist__cancel_workload(evlist);
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * mmap the first fd on a given CPU and ask for events for the other
     * fds in the same CPU to be injected in the same mmap ring buffer
     * (using ioctl(PERF_EVENT_IOC_SET_OUTPUT)).
     */
    err = evlist__do_mmap(evlist, opts.mmap_pages);
    if err < 0 {
        pr_debug(
            c"evlist__mmap: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        evlist__cancel_workload(evlist);
        CPU_FREE(cpu_mask);
        perf_sample__exit(&mut sample);
        evlist__put(evlist);
        return if err == -EACCES {
            TEST_SKIP
        } else if err < 0 || errs != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
    }

    /*
     * Now that all is properly set up, enable the events, they will
     * count just on workload.pid, which will start...
     */
    evlist__enable(evlist);

    /*
     * Now!
     */
    evlist__start_workload(evlist);

    'read_events: loop {
        let before: c_int = total_events;

        i = 0;
        while i < (*evlist__core(evlist)).nr_mmaps {
            let mut event: *mut perf_event;
            let md: *mut mmap;

            md = evlist__mmap(evlist).offset(i as isize);
            if perf_mmap__read_init(&mut (*md).core) < 0 {
                i += 1;
                continue;
            }

            loop {
                event = perf_mmap__read_event(&mut (*md).core);
                if event.is_null() {
                    break;
                }

                let type_: u32 = (*event).header.type_;
                let name: *const c_char = perf_event__name(type_);

                total_events += 1;
                if (type_ as usize) < PERF_RECORD_MAX {
                    nr_events[type_ as usize] += 1;
                }

                err = evlist__parse_sample(evlist, event, &mut sample);
                if err < 0 {
                    if verbose > 0 {
                        perf_event__fprintf(event, core::ptr::null_mut(), stderr);
                    }
                    pr_debug(c"Couldn't parse sample\n".as_ptr());
                    CPU_FREE(cpu_mask);
                    perf_sample__exit(&mut sample);
                    evlist__put(evlist);
                    return if err == -EACCES {
                        TEST_SKIP
                    } else if err < 0 || errs != 0 {
                        TEST_FAIL
                    } else {
                        TEST_OK
                    };
                }

                if verbose > 0 {
                    pr_info(c"%llu %d ".as_ptr(), sample.time, sample.cpu);
                    perf_event__fprintf(event, core::ptr::null_mut(), stderr);
                }

                if prev_time > sample.time {
                    pr_debug(
                        c"%s going backwards in time, prev=%llu, curr=%llu\n".as_ptr(),
                        name,
                        prev_time,
                        sample.time,
                    );
                    errs += 1;
                }

                prev_time = sample.time;

                if sample.cpu != cpu {
                    pr_debug(
                        c"%s with unexpected cpu, expected %d, got %d\n".as_ptr(),
                        name,
                        cpu,
                        sample.cpu,
                    );
                    errs += 1;
                }

                if sample.pid as pid_t != evlist__workload_pid(evlist) {
                    pr_debug(
                        c"%s with unexpected pid, expected %d, got %d\n".as_ptr(),
                        name,
                        evlist__workload_pid(evlist),
                        sample.pid,
                    );
                    errs += 1;
                }

                if sample.tid as pid_t != evlist__workload_pid(evlist) {
                    pr_debug(
                        c"%s with unexpected tid, expected %d, got %d\n".as_ptr(),
                        name,
                        evlist__workload_pid(evlist),
                        sample.tid,
                    );
                    errs += 1;
                }

                if (type_ == PERF_RECORD_COMM
                    || type_ == PERF_RECORD_MMAP
                    || type_ == PERF_RECORD_MMAP2
                    || type_ == PERF_RECORD_FORK
                    || type_ == PERF_RECORD_EXIT)
                    && (*event).comm.pid as pid_t != evlist__workload_pid(evlist)
                {
                    pr_debug(c"%s with unexpected pid/tid\n".as_ptr(), name);
                    errs += 1;
                }

                if (type_ == PERF_RECORD_COMM || type_ == PERF_RECORD_MMAP || type_ == PERF_RECORD_MMAP2)
                    && (*event).comm.pid != (*event).comm.tid
                {
                    pr_debug(c"%s with different pid/tid!\n".as_ptr(), name);
                    errs += 1;
                }

                match type_ {
                    PERF_RECORD_COMM => {
                        if strcmp((*event).comm.comm.as_ptr(), cmd) != 0 {
                            pr_debug(c"%s with unexpected comm!\n".as_ptr(), name);
                            errs += 1;
                        }
                    }
                    PERF_RECORD_EXIT => {
                        perf_mmap__consume(&mut (*md).core);
                        perf_sample__exit(&mut sample);
                        perf_mmap__read_done(&mut (*md).core);
                        break 'read_events;
                    }
                    PERF_RECORD_MMAP => {
                        mmap_filename = (*event).mmap.filename.as_ptr() as *mut c_char;
                        bname = strrchr(mmap_filename, '/' as c_int);
                        if !bname.is_null() {
                            if !found_cmd_mmap {
                                found_cmd_mmap = strcmp(bname.offset(1), cmd) == 0;
                            }
                            if !found_coreutils_mmap {
                                found_coreutils_mmap = strcmp(bname.offset(1), c"coreutils".as_ptr()) == 0;
                            }
                            if !found_libc_mmap {
                                found_libc_mmap = strncmp(bname.offset(1), c"libc".as_ptr(), 4) == 0;
                            }
                            if !found_ld_mmap {
                                found_ld_mmap = strncmp(bname.offset(1), c"ld".as_ptr(), 2) == 0;
                            }
                        } else if !found_vdso_mmap {
                            found_vdso_mmap = strcmp(mmap_filename, c"[vdso]".as_ptr()) == 0;
                        }
                    }
                    PERF_RECORD_MMAP2 => {
                        mmap_filename = (*event).mmap2.filename.as_ptr() as *mut c_char;
                        bname = strrchr(mmap_filename, '/' as c_int);
                        if !bname.is_null() {
                            if !found_cmd_mmap {
                                found_cmd_mmap = strcmp(bname.offset(1), cmd) == 0;
                            }
                            if !found_coreutils_mmap {
                                found_coreutils_mmap = strcmp(bname.offset(1), c"coreutils".as_ptr()) == 0;
                            }
                            if !found_libc_mmap {
                                found_libc_mmap = strncmp(bname.offset(1), c"libc".as_ptr(), 4) == 0;
                            }
                            if !found_ld_mmap {
                                found_ld_mmap = strncmp(bname.offset(1), c"ld".as_ptr(), 2) == 0;
                            }
                        } else if !found_vdso_mmap {
                            found_vdso_mmap = strcmp(mmap_filename, c"[vdso]".as_ptr()) == 0;
                        }
                    }
                    PERF_RECORD_SAMPLE => {
                        /* Just ignore samples for now */
                    }
                    _ => {
                        pr_debug(c"Unexpected perf_event->header.type %d!\n".as_ptr(), type_);
                        errs += 1;
                    }
                }

                perf_mmap__consume(&mut (*md).core);
                perf_sample__exit(&mut sample);
            }
            perf_mmap__read_done(&mut (*md).core);
            i += 1;
        }

        /*
         * We don't use poll here because at least at 3.1 times the
         * PERF_RECORD_{!SAMPLE} events don't honour
         * perf_event_attr.wakeup_events, just PERF_EVENT_SAMPLE does.
         */
        if total_events == before && false {
            evlist__poll(evlist, -1);
        }

        sleep(1);
        wakeups += 1;
        if wakeups > 5 {
            pr_debug(c"No PERF_RECORD_EXIT event!\n".as_ptr());
            break;
        }
    }

    if nr_events[PERF_RECORD_COMM as usize] > 1 + (found_coreutils_mmap as c_int) {
        pr_debug(c"Excessive number of PERF_RECORD_COMM events!\n".as_ptr());
        errs += 1;
    }

    if nr_events[PERF_RECORD_COMM as usize] == 0 {
        pr_debug(c"Missing PERF_RECORD_COMM for %s!\n".as_ptr(), cmd);
        errs += 1;
    }

    if !found_cmd_mmap && !found_coreutils_mmap {
        pr_debug(c"PERF_RECORD_MMAP for %s missing!\n".as_ptr(), cmd);
        errs += 1;
    }

    if !found_libc_mmap {
        pr_debug(c"PERF_RECORD_MMAP for %s missing!\n".as_ptr(), c"libc".as_ptr());
        errs += 1;
    }

    if !found_ld_mmap {
        pr_debug(c"PERF_RECORD_MMAP for %s missing!\n".as_ptr(), c"ld".as_ptr());
        errs += 1;
    }

    if !found_vdso_mmap {
        pr_debug(c"PERF_RECORD_MMAP for %s missing!\n".as_ptr(), c"[vdso]".as_ptr());
        errs += 1;
    }

    CPU_FREE(cpu_mask);
    perf_sample__exit(&mut sample);
    evlist__put(evlist);
    if err == -EACCES {
        return TEST_SKIP;
    }
    if err < 0 || errs != 0 {
        return TEST_FAIL;
    }
    TEST_OK
}

unsafe fn goto_out(sample: *mut perf_sample, evlist: *mut evlist, _err: c_int, _errs: c_int) {
    perf_sample__exit(sample);
    evlist__put(evlist);
}

static mut tests__PERF_RECORD: [test_case; 2] = [
    test_case {
        name: c"PERF_RECORD_* events & perf_sample fields".as_ptr(),
        run_case: Some(test__PERF_RECORD),
        desc: core::ptr::null(),
        reason: c"permissions".as_ptr(),
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
        desc: core::ptr::null(),
        reason: core::ptr::null(),
    },
];

#[repr(C)]
pub struct test_suite_PERF_RECORD {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[unsafe(no_mangle)]
pub static mut suite__PERF_RECORD: test_suite_PERF_RECORD = test_suite_PERF_RECORD {
    desc: c"PERF_RECORD_* events & perf_sample fields".as_ptr(),
    test_cases: unsafe { tests__PERF_RECORD.as_mut_ptr() },
};
