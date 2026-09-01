/*
 * Strictly speaking, this is not a test. But it can report during test
 * runs so relative performance can be measured.
 */
#![no_main]

// C dependencies: assert.h, err.h, limits.h, sched.h, stdbool.h, stddef.h,
// stdio.h, stdlib.h, time.h, unistd.h, linux/filter.h, linux/seccomp.h,
// sys/param.h, sys/prctl.h, sys/syscall.h, sys/types.h, and "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type clockid_t = c_int;
type pid_t = c_int;
type ulong = c_ulong;

const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
const INT_MAX: u64 = 2147483647;
const PR_SET_NO_NEW_PRIVS: c_int = 38;
const PR_SET_SECCOMP: c_int = 22;
const SECCOMP_MODE_FILTER: c_ulong = 2;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
const __NR_getpid: c_long = 39;
const _SC_NPROCESSORS_CONF: c_int = 83;
const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_RET: u16 = 0x06;
const BPF_K: u16 = 0x00;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
struct seccomp_data {
    nr: c_int,
    arch: u32,
    instruction_pointer: u64,
    args: [u64; 6],
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 16],
}

extern "C" {
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;

    fn getpid() -> pid_t;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn prctl(option: c_int, ...) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn CPU_ALLOC(count: c_ulong) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_ulong) -> c_ulong;
    fn CPU_ZERO_S(setsize: c_ulong, cpuset: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_long, setsize: c_ulong, cpuset: *mut cpu_set_t);
    fn CPU_FREE(cpuset: *mut cpu_set_t);
    fn sched_setaffinity(pid: pid_t, cpusetsize: c_ulong, mask: *const cpu_set_t) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result(result: bool, format: *const c_char, ...);
    fn ksft_finished();
}

type c_uint = u32;

fn bpf_stmt(code: u16, k: u32) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

fn max_double(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn timing(clk_id: clockid_t, samples: u64) -> u64 {
    let mut start: timespec = mem::zeroed();
    let mut finish: timespec = mem::zeroed();
    let mut i: u64;
    let pid: pid_t;
    let mut ret: pid_t;

    pid = getpid();
    assert!(clock_gettime(clk_id, &mut start) == 0);
    i = 0;
    while i < samples {
        ret = syscall(__NR_getpid) as pid_t;
        assert!(pid == ret);
        i += 1;
    }
    assert!(clock_gettime(clk_id, &mut finish) == 0);

    i = (finish.tv_sec - start.tv_sec) as u64;
    i = i.wrapping_mul(1000000000u64);
    i = i.wrapping_add((finish.tv_nsec - start.tv_nsec) as u64);

    ksft_print_msg(
        b"%lu.%09lu - %lu.%09lu = %llu (%.1fs)\n\0".as_ptr() as *const c_char,
        finish.tv_sec as c_ulong,
        finish.tv_nsec as c_ulong,
        start.tv_sec as c_ulong,
        start.tv_nsec as c_ulong,
        i,
        i as f64 / 1000000000.0f64,
    );

    i
}

unsafe fn calibrate() -> u64 {
    let mut start: timespec = mem::zeroed();
    let mut finish: timespec = mem::zeroed();
    let mut i: u64;
    let mut samples: u64;
    let step: u64 = 9973;
    let pid: pid_t;
    let mut ret: pid_t;
    let seconds: c_int = 15;

    ksft_print_msg(
        b"Calibrating sample size for %d seconds worth of syscalls ...\n\0".as_ptr()
            as *const c_char,
        seconds,
    );

    samples = 0;
    pid = getpid();
    assert!(clock_gettime(CLOCK_MONOTONIC, &mut start) == 0);
    loop {
        i = 0;
        while i < step {
            ret = syscall(__NR_getpid) as pid_t;
            assert!(pid == ret);
            i += 1;
        }
        assert!(clock_gettime(CLOCK_MONOTONIC, &mut finish) == 0);

        samples = samples.wrapping_add(step);
        i = (finish.tv_sec - start.tv_sec) as u64;
        i = i.wrapping_mul(1000000000u64);
        i = i.wrapping_add((finish.tv_nsec - start.tv_nsec) as u64);
        if !(i < 1000000000u64) {
            break;
        }
    }

    samples.wrapping_mul(seconds as u64)
}

fn approx(i_one: c_int, i_two: c_int) -> bool {
    /*
     * This continues to be a noisy test. Instead of a 1% comparison
     * go with 10%.
     */
    let one: f64 = i_one as f64;
    let mut one_bump: f64 = one * 0.1f64;
    let two: f64 = i_two as f64;
    let mut two_bump: f64 = two * 0.1f64;

    one_bump = one + max_double(one_bump, 2.0f64);
    two_bump = two + max_double(two_bump, 2.0f64);

    /* Equal to, or within 1% or 2 digits */
    if one == two || (one > two && one <= two_bump) || (two > one && two <= one_bump) {
        return true;
    }
    false
}

fn le(i_one: c_int, i_two: c_int) -> bool {
    if i_one <= i_two {
        return true;
    }
    false
}

unsafe fn compare(
    name_one: *const c_char,
    name_eval: *const c_char,
    name_two: *const c_char,
    one: u64,
    eval: fn(c_int, c_int) -> bool,
    two: u64,
    skip: bool,
) -> c_long {
    let good: bool;

    if skip {
        ksft_test_result_skip(
            b"%s %s %s\n\0".as_ptr() as *const c_char,
            name_one,
            name_eval,
            name_two,
        );
        return 0;
    }

    ksft_print_msg(
        b"\t%s %s %s (%lld %s %lld): \0".as_ptr() as *const c_char,
        name_one,
        name_eval,
        name_two,
        one as i64,
        name_eval,
        two as i64,
    );
    if one > INT_MAX {
        ksft_print_msg(
            b"Miscalculation! Measurement went negative: %lld\n\0".as_ptr() as *const c_char,
            one as i64,
        );
        good = false;
    } else if two > INT_MAX {
        ksft_print_msg(
            b"Miscalculation! Measurement went negative: %lld\n\0".as_ptr() as *const c_char,
            two as i64,
        );
        good = false;
    } else {
        good = eval(one as c_int, two as c_int);
        printf(
            b"%s\n\0".as_ptr() as *const c_char,
            if good {
                "✔️\0".as_ptr() as *const c_char
            } else {
                "❌\0".as_ptr() as *const c_char
            },
        );
    }

    ksft_test_result(
        good,
        b"%s %s %s\n\0".as_ptr() as *const c_char,
        name_one,
        name_eval,
        name_two,
    );

    if good {
        0
    } else {
        1
    }
}

/* Pin to a single CPU so the benchmark won't bounce around the system. */
unsafe fn affinity() {
    let mut cpu: c_long;
    let ncores: ulong = sysconf(_SC_NPROCESSORS_CONF) as ulong;
    let setp: *mut cpu_set_t = CPU_ALLOC(ncores);
    let setsz: ulong = CPU_ALLOC_SIZE(ncores);

    /*
     * Totally unscientific way to avoid CPUs that might be busier:
     * choose the highest CPU instead of the lowest.
     */
    cpu = ncores as c_long - 1;
    while cpu >= 0 {
        CPU_ZERO_S(setsz, setp);
        CPU_SET_S(cpu, setsz, setp);
        if sched_setaffinity(getpid(), setsz, setp) == -1 {
            cpu -= 1;
            continue;
        }
        printf(
            b"Pinned to CPU %lu of %lu\n\0".as_ptr() as *const c_char,
            (cpu + 1) as c_ulong,
            ncores,
        );
        CPU_FREE(setp);
        return;
    }
    fprintf(
        stderr,
        b"Could not set CPU affinity -- calibration may not work well\0".as_ptr() as *const c_char,
    );

    CPU_FREE(setp);
}

unsafe fn estimate(fmt: *const c_char, var: &mut u64, what: u64, skip: &mut bool, ret: &mut c_long) {
    *var = what;
    ksft_print_msg(
        b"Estimated %s: %llu ns\n\0".as_ptr() as *const c_char,
        fmt,
        *var,
    );
    if *var > INT_MAX {
        *skip = true;
        *ret |= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut bitmap_filter: [sock_filter; 2] = [
        bpf_stmt(
            BPF_LD | BPF_W | BPF_ABS,
            core::mem::offset_of!(seccomp_data, nr) as u32,
        ),
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];
    let mut bitmap_prog: sock_fprog = sock_fprog {
        len: bitmap_filter.len() as u16,
        filter: bitmap_filter.as_mut_ptr(),
    };
    let mut filter: [sock_filter; 2] = [
        bpf_stmt(
            BPF_LD | BPF_W | BPF_ABS,
            core::mem::offset_of!(seccomp_data, args) as u32,
        ),
        bpf_stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    ];
    let mut prog: sock_fprog = sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };

    let mut ret: c_long;
    let mut bits: c_long;
    let samples: u64;
    let mut calc: u64 = 0;
    let native: u64;
    let filter1: u64;
    let filter2: u64;
    let bitmap1: u64;
    let bitmap2: u64;
    let mut entry: u64 = 0;
    let mut per_filter1: u64 = 0;
    let mut per_filter2: u64 = 0;
    let mut skip: bool = false;

    setbuf(stdout, ptr::null_mut());

    ksft_print_header();
    ksft_set_plan(7);

    ksft_print_msg(b"Running on:\n\0".as_ptr() as *const c_char);
    ksft_print_msg(b"%s\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char);
    system(b"uname -a\0".as_ptr() as *const c_char);

    ksft_print_msg(b"Current BPF sysctl settings:\n\0".as_ptr() as *const c_char);
    /* Avoid using "sysctl" which may not be installed. */
    ksft_print_msg(b"%s\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char);
    system(b"grep -H . /proc/sys/net/core/bpf_jit_enable\0".as_ptr() as *const c_char);
    ksft_print_msg(b"%s\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char);
    system(b"grep -H . /proc/sys/net/core/bpf_jit_harden\0".as_ptr() as *const c_char);

    affinity();

    if argc > 1 {
        samples = strtoull(*argv.offset(1), ptr::null_mut(), 0);
    } else {
        samples = calibrate();
    }

    ksft_print_msg(
        b"Benchmarking %llu syscalls...\n\0".as_ptr() as *const c_char,
        samples,
    );

    /* Native call */
    native = timing(CLOCK_PROCESS_CPUTIME_ID, samples) / samples;
    ksft_print_msg(
        b"getpid native: %llu ns\n\0".as_ptr() as *const c_char,
        native,
    );

    ret = prctl(PR_SET_NO_NEW_PRIVS, 1 as c_ulong, 0 as c_ulong, 0 as c_ulong, 0 as c_ulong) as c_long;
    assert!(ret == 0);

    /* One filter resulting in a bitmap */
    ret = prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut bitmap_prog as *mut sock_fprog) as c_long;
    assert!(ret == 0);

    bitmap1 = timing(CLOCK_PROCESS_CPUTIME_ID, samples) / samples;
    ksft_print_msg(
        b"getpid RET_ALLOW 1 filter (bitmap): %llu ns\n\0".as_ptr() as *const c_char,
        bitmap1,
    );

    /* Second filter resulting in a bitmap */
    ret = prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut bitmap_prog as *mut sock_fprog) as c_long;
    assert!(ret == 0);

    bitmap2 = timing(CLOCK_PROCESS_CPUTIME_ID, samples) / samples;
    ksft_print_msg(
        b"getpid RET_ALLOW 2 filters (bitmap): %llu ns\n\0".as_ptr() as *const c_char,
        bitmap2,
    );

    /* Third filter, can no longer be converted to bitmap */
    ret = prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut prog as *mut sock_fprog) as c_long;
    assert!(ret == 0);

    filter1 = timing(CLOCK_PROCESS_CPUTIME_ID, samples) / samples;
    ksft_print_msg(
        b"getpid RET_ALLOW 3 filters (full): %llu ns\n\0".as_ptr() as *const c_char,
        filter1,
    );

    /* Fourth filter, can not be converted to bitmap because of filter 3 */
    ret = prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &mut bitmap_prog as *mut sock_fprog) as c_long;
    assert!(ret == 0);

    filter2 = timing(CLOCK_PROCESS_CPUTIME_ID, samples) / samples;
    ksft_print_msg(
        b"getpid RET_ALLOW 4 filters (full): %llu ns\n\0".as_ptr() as *const c_char,
        filter2,
    );

    /* Estimations */
    estimate(
        b"total seccomp overhead for 1 bitmapped filter\0".as_ptr() as *const c_char,
        &mut calc,
        bitmap1.wrapping_sub(native),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"total seccomp overhead for 2 bitmapped filters\0".as_ptr() as *const c_char,
        &mut calc,
        bitmap2.wrapping_sub(native),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"total seccomp overhead for 3 full filters\0".as_ptr() as *const c_char,
        &mut calc,
        filter1.wrapping_sub(native),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"total seccomp overhead for 4 full filters\0".as_ptr() as *const c_char,
        &mut calc,
        filter2.wrapping_sub(native),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"seccomp entry overhead\0".as_ptr() as *const c_char,
        &mut entry,
        bitmap1
            .wrapping_sub(native)
            .wrapping_sub(bitmap2.wrapping_sub(bitmap1)),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"seccomp per-filter overhead (last 2 diff)\0".as_ptr() as *const c_char,
        &mut per_filter1,
        filter2.wrapping_sub(filter1),
        &mut skip,
        &mut ret,
    );
    estimate(
        b"seccomp per-filter overhead (filters / 4)\0".as_ptr() as *const c_char,
        &mut per_filter2,
        filter2.wrapping_sub(native).wrapping_sub(entry) / 4,
        &mut skip,
        &mut ret,
    );

    ksft_print_msg(b"Expectations:\n\0".as_ptr() as *const c_char);
    ret |= compare(
        b"native\0".as_ptr() as *const c_char,
        "≤\0".as_ptr() as *const c_char,
        b"1 bitmap\0".as_ptr() as *const c_char,
        native,
        le,
        bitmap1,
        skip,
    );
    bits = compare(
        b"native\0".as_ptr() as *const c_char,
        "≤\0".as_ptr() as *const c_char,
        b"1 filter\0".as_ptr() as *const c_char,
        native,
        le,
        filter1,
        skip,
    );
    if bits != 0 {
        skip = true;
    }

    ret |= compare(
        b"per-filter (last 2 diff)\0".as_ptr() as *const c_char,
        "≈\0".as_ptr() as *const c_char,
        b"per-filter (filters / 4)\0".as_ptr() as *const c_char,
        per_filter1,
        approx,
        per_filter2,
        skip,
    );

    bits = compare(
        b"1 bitmapped\0".as_ptr() as *const c_char,
        "≈\0".as_ptr() as *const c_char,
        b"2 bitmapped\0".as_ptr() as *const c_char,
        bitmap1.wrapping_sub(native),
        approx,
        bitmap2.wrapping_sub(native),
        skip,
    );
    if bits != 0 {
        ksft_print_msg(
            b"Skipping constant action bitmap expectations: they appear unsupported.\n\0".as_ptr()
                as *const c_char,
        );
        skip = true;
    }

    ret |= compare(
        b"entry\0".as_ptr() as *const c_char,
        "≈\0".as_ptr() as *const c_char,
        b"1 bitmapped\0".as_ptr() as *const c_char,
        entry,
        approx,
        bitmap1.wrapping_sub(native),
        skip,
    );
    ret |= compare(
        b"entry\0".as_ptr() as *const c_char,
        "≈\0".as_ptr() as *const c_char,
        b"2 bitmapped\0".as_ptr() as *const c_char,
        entry,
        approx,
        bitmap2.wrapping_sub(native),
        skip,
    );
    ret |= compare(
        b"native + entry + (per filter * 4)\0".as_ptr() as *const c_char,
        "≈\0".as_ptr() as *const c_char,
        b"4 filters total\0".as_ptr() as *const c_char,
        entry.wrapping_add(per_filter1.wrapping_mul(4)).wrapping_add(native),
        approx,
        filter2,
        skip,
    );

    if ret != 0 {
        ksft_print_msg(
            b"Saw unexpected benchmark result. Try running again with more samples?\n\0".as_ptr()
                as *const c_char,
        );
    }

    ksft_finished();

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
