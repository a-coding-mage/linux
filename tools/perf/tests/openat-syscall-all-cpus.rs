// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source.
// External dependencies correspond to the original C includes:
// errno.h, inttypes.h, sched.h, sys/types.h, sys/stat.h, fcntl.h,
// api/fs/fs.h, linux/err.h, linux/string.h, api/fs/tracing_path.h,
// evsel.h, tests.h, thread_map.h, perf/cpumap.h, debug.h, stat.h,
// util/counts.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const TEST_OK: c_int = 0;
const STRERR_BUFSIZE: usize = 128;
const BUFSIZ: usize = 8192;
const CPU_SETSIZE: c_int = 1024;
const O_RDONLY: c_int = 0;

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
pub struct evsel {
    pub core: perf_evsel,
    pub counts: *mut perf_counts_values,
}

#[repr(C)]
pub struct perf_evsel {
    pub cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
}

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn getpid() -> c_int;
    fn thread_map__new_by_tid(pid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__get(cpus: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel;
    fn evsel__open(evsel: *mut evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> c_int;
    fn evsel__read_on_cpu(evsel: *mut evsel, cpu_map_idx: c_uint, thread: c_int) -> c_int;
    fn evsel__free_counts(evsel: *mut evsel);
    fn evsel__put(evsel: *mut evsel);
    fn perf_evsel__close_fd(evsel: *mut perf_evsel);
    fn perf_counts(counts: *mut perf_counts_values, cpu_map_idx: c_uint, thread: c_int) -> *mut perf_counts_values;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn tracing_path__strerror_open_tp(
        errnum: c_int,
        buf: *mut c_char,
        buflen: usize,
        sys: *const c_char,
        name: *const c_char,
    );
    fn pr_debug(fmt: *const c_char, ...);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        let set = &mut *set;
        let mut i = 0usize;
        while i < set.__bits.len() {
            set.__bits[i] = 0;
            i += 1;
        }
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    unsafe {
        let cpu = cpu as usize;
        let bits_per_word = core::mem::size_of::<c_ulong>() * 8;
        (*set).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

unsafe fn CPU_CLR(cpu: c_int, set: *mut cpu_set_t) {
    unsafe {
        let cpu = cpu as usize;
        let bits_per_word = core::mem::size_of::<c_ulong>() * 8;
        (*set).__bits[cpu / bits_per_word] &= !((1 as c_ulong) << (cpu % bits_per_word));
    }
}

unsafe extern "C" fn test__openat_syscall_event_on_all_cpus(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    unsafe {
        let mut err: c_int = TEST_FAIL;
        let mut fd: c_int;
        let mut idx: c_uint;
        let mut cpu: perf_cpu;
        let mut cpus: *mut perf_cpu_map;
        let evsel: *mut evsel;
        let nr_openat_calls: c_uint = 111;
        let mut i: c_uint;
        let mut cpu_set: cpu_set_t = core::mem::zeroed();
        let threads: *mut perf_thread_map = thread_map__new_by_tid(getpid());
        let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];
        let mut errbuf: [c_char; BUFSIZ] = [0; BUFSIZ];

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

        CPU_ZERO(&mut cpu_set);

        evsel = evsel__newtp(c"syscalls".as_ptr(), c"sys_enter_openat".as_ptr());
        if IS_ERR(evsel as *const c_void) {
            tracing_path__strerror_open_tp(
                errno,
                errbuf.as_mut_ptr(),
                errbuf.len(),
                c"syscalls".as_ptr(),
                c"sys_enter_openat".as_ptr(),
            );
            pr_debug(c"%s\n".as_ptr(), errbuf.as_ptr());
            err = TEST_SKIP;
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            return err;
        }

        if evsel__open(evsel, cpus, threads) < 0 {
            pr_debug(
                c"failed to open counter: %s, tweak /proc/sys/kernel/perf_event_paranoid?\n".as_ptr(),
                str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
            );
            err = TEST_SKIP;
            evsel__put(evsel);
            perf_cpu_map__put(cpus);
            perf_thread_map__put(threads);
            return err;
        }

        idx = 0;
        while (idx as c_int) < perf_cpu_map__nr(cpus) {
            cpu = perf_cpu_map__cpu(cpus, idx as c_int);
            let ncalls: c_uint = nr_openat_calls.wrapping_add(idx);
            /*
             * XXX eventually lift this restriction in a way that
             * keeps perf building on older glibc installations
             * without CPU_ALLOC. 1024 cpus in 2010 still seems
             * a reasonable upper limit tho :-)
             */
            if cpu.cpu >= CPU_SETSIZE {
                pr_debug(c"Ignoring CPU %d\n".as_ptr(), cpu.cpu);
                idx = idx.wrapping_add(1);
                continue;
            }

            CPU_SET(cpu.cpu, &mut cpu_set);
            if sched_setaffinity(0, core::mem::size_of_val(&cpu_set), &cpu_set) < 0 {
                pr_debug(
                    c"sched_setaffinity() failed on CPU %d: %s ".as_ptr(),
                    cpu.cpu,
                    str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
                );
                evsel__free_counts(evsel);
                perf_evsel__close_fd(&mut (*evsel).core);
                evsel__put(evsel);
                perf_cpu_map__put(cpus);
                perf_thread_map__put(threads);
                return err;
            }
            i = 0;
            while i < ncalls {
                fd = openat(0, c"/etc/passwd".as_ptr(), O_RDONLY);
                close(fd);
                i = i.wrapping_add(1);
            }
            CPU_CLR(cpu.cpu, &mut cpu_set);

            idx = idx.wrapping_add(1);
        }

        (*evsel).core.cpus = perf_cpu_map__get(cpus);

        err = TEST_OK;

        idx = 0;
        while (idx as c_int) < perf_cpu_map__nr(cpus) {
            cpu = perf_cpu_map__cpu(cpus, idx as c_int);
            let expected: c_uint;

            if cpu.cpu >= CPU_SETSIZE {
                idx = idx.wrapping_add(1);
                continue;
            }

            if evsel__read_on_cpu(evsel, idx, 0) < 0 {
                pr_debug(c"evsel__read_on_cpu\n".as_ptr());
                err = TEST_FAIL;
                break;
            }

            expected = nr_openat_calls.wrapping_add(idx);
            if (*perf_counts((*evsel).counts, idx, 0)).val != expected as u64 {
                pr_debug(
                    c"evsel__read_on_cpu: expected to intercept %d calls on cpu %d, got %llu\n".as_ptr(),
                    expected,
                    cpu.cpu,
                    (*perf_counts((*evsel).counts, idx, 0)).val,
                );
                err = TEST_FAIL;
            }

            idx = idx.wrapping_add(1);
        }

        evsel__free_counts(evsel);
        perf_evsel__close_fd(&mut (*evsel).core);
        evsel__put(evsel);
        perf_cpu_map__put(cpus);
        perf_thread_map__put(threads);
        err
    }
}

#[unsafe(no_mangle)]
pub static mut tests__openat_syscall_event_on_all_cpus: [test_case; 2] = [
    test_case {
        name: c"Detect openat syscall event on all cpus".as_ptr(),
        run_case: Some(test__openat_syscall_event_on_all_cpus),
        desc: c"permissions".as_ptr(),
    },
    test_case {
        name: core::ptr::null(),
        run_case: None,
        desc: core::ptr::null(),
    },
];

#[unsafe(no_mangle)]
pub static mut suite__openat_syscall_event_on_all_cpus: test_suite = test_suite {
    desc: c"Detect openat syscall event on all cpus".as_ptr(),
    test_cases: unsafe { tests__openat_syscall_event_on_all_cpus.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
