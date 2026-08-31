// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * perf events self profiling example test case for hw breakpoints.
 *
 * This tests perf PERF_TYPE_BREAKPOINT parameters
 * 1) tests all variants of the break on read/write flags
 * 2) tests exclude_user == 0 and 1
 * 3) test array matches (if DAWR is supported))
 * 4) test different numbers of breakpoints matches
 *
 * Configure this breakpoint, then read and write the data a number of
 * times. Then check the output count from perf is as expected.
 *
 * Based on:
 *   http://ozlabs.org/~anton/junkcode/perf_events_example1.c
 *
 * Copyright (C) 2018 Michael Neuling, IBM Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type bool_t = bool;

// Dependencies originally supplied by system and local C headers:
// <unistd.h>, <assert.h>, <sched.h>, <stdio.h>, <stdlib.h>, <signal.h>,
// <string.h>, <sys/ioctl.h>, <sys/wait.h>, <sys/ptrace.h>,
// <sys/resource.h>, <sys/sysinfo.h>, <asm/ptrace.h>, <elf.h>,
// <pthread.h>, <sys/syscall.h>, <linux/perf_event.h>,
// <linux/hw_breakpoint.h>, and "utils.h".

const PPC_DEBUG_FEATURE_DATA_BP_ARCH_31: c_ulong = 0x20;

const MAX_LOOPS: c_int = 10000;

const DAWR_LENGTH_MAX: usize = ((0x3f + 1) * 8);

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const RLIMIT_NOFILE: c_int = 7;
const SIGUSR1: c_int = 10;
const PTRACE_TRACEME: c_int = 0;

extern "C" {
    static mut PERF_TYPE_BREAKPOINT: __u32;
    static mut HW_BREAKPOINT_R: c_int;
    static mut HW_BREAKPOINT_W: c_int;
    static mut HW_BREAKPOINT_RW: c_int;
    static mut PERF_EVENT_IOC_RESET: c_ulong;
    static mut PERF_EVENT_IOC_ENABLE: c_ulong;
    static mut PERF_EVENT_IOC_DISABLE: c_ulong;
    static mut __NR_perf_event_open: c_long;
    static mut PPC_PTRACE_GETHWDBGINFO: c_int;
}

#[repr(C)]
struct perf_event_attr {
    type_: __u32,
    size: __u32,
    bp_type: __u64,
    bp_addr: __u64,
    bp_len: __u64,
    exclude_kernel: __u64,
    exclude_hv: __u64,
    exclude_guest: __u64,
    exclude_user: __u64,
    disabled: __u64,
}

#[repr(C)]
struct rlimit {
    rlim_cur: c_ulong,
    rlim_max: c_ulong,
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
struct ppc_debug_info {
    version: __u32,
    num_instruction_bps: __u32,
    num_data_bps: __u32,
    num_condition_regs: __u32,
    data_bp_alignment: __u32,
    sizeof_condition: __u32,
    features: c_ulong,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn syscall(num: c_long, ...) -> c_long;
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn get_nprocs_conf() -> c_int;
    fn get_nprocs() -> c_int;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_ISSET_S(cpu: c_int, setsize: size_t, cpusetp: *const cpu_set_t) -> c_int;
    fn CPU_FREE(cpusetp: *mut cpu_set_t);
    fn sched_getaffinity(pid: c_int, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn rand() -> c_int;
    fn srand(seed: u32);
    fn time(tloc: *mut c_long) -> c_long;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn fork() -> c_int;
    fn ptrace(request: c_int, ...) -> c_long;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn sleep(seconds: u32) -> u32;
    fn wait(wstatus: *mut c_int) -> c_int;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

extern "C" {
    fn SKIP_IF_MSG(cond: bool_t, msg: *const c_char);
}

static mut nprocs: c_int = 0;

static mut a: c_int = 10;
static mut b: c_int = 10;
#[repr(align(512))]
struct aligned_c([c_char; 512 + 8]);
static mut c: aligned_c = aligned_c([0; 512 + 8]);

unsafe fn perf_event_attr_set(
    attr: *mut perf_event_attr,
    type_: __u32,
    addr: __u64,
    len: __u64,
    exclude_user: bool_t,
) {
    memset(
        attr as *mut c_void,
        0,
        size_of::<perf_event_attr>(),
    );
    (*attr).type_ = PERF_TYPE_BREAKPOINT;
    (*attr).size = size_of::<perf_event_attr>() as __u32;
    (*attr).bp_type = type_ as __u64;
    (*attr).bp_addr = addr;
    (*attr).bp_len = len;
    (*attr).exclude_kernel = 1;
    (*attr).exclude_hv = 1;
    (*attr).exclude_guest = 1;
    (*attr).exclude_user = exclude_user as __u64;
    (*attr).disabled = 1;
}

unsafe fn perf_process_event_open_exclude_user(
    type_: __u32,
    addr: __u64,
    len: __u64,
    exclude_user: bool_t,
) -> c_int {
    let mut attr: perf_event_attr = zeroed();

    perf_event_attr_set(&mut attr, type_, addr, len, exclude_user);
    syscall(__NR_perf_event_open, &mut attr, getpid(), -1, -1, 0) as c_int
}

unsafe fn perf_process_event_open(type_: __u32, addr: __u64, len: __u64) -> c_int {
    let mut attr: perf_event_attr = zeroed();

    perf_event_attr_set(&mut attr, type_, addr, len, false);
    syscall(__NR_perf_event_open, &mut attr, getpid(), -1, -1, 0) as c_int
}

unsafe fn perf_cpu_event_open(cpu: c_long, type_: __u32, addr: __u64, len: __u64) -> c_int {
    let mut attr: perf_event_attr = zeroed();

    perf_event_attr_set(&mut attr, type_, addr, len, false);
    syscall(__NR_perf_event_open, &mut attr, -1, cpu, -1, 0) as c_int
}

unsafe fn close_fds(fd: *mut c_int, n: c_int) {
    let mut i: c_int = 0;

    while i < n {
        close(*fd.offset(i as isize));
        i += 1;
    }
}

unsafe fn read_fds(fd: *mut c_int, n: c_int) -> c_ulong {
    let mut i: c_int = 0;
    let mut c: c_ulong = 0;
    let mut count: c_ulong = 0;
    let mut res: size_t;

    while i < n {
        res = read(
            *fd.offset(i as isize),
            &mut c as *mut _ as *mut c_void,
            size_of::<c_ulong>(),
        ) as size_t;
        assert!(res == size_of::<u64>());
        count = count.wrapping_add(c);
        i += 1;
    }
    count
}

unsafe fn reset_fds(fd: *mut c_int, n: c_int) {
    let mut i: c_int = 0;

    while i < n {
        ioctl(*fd.offset(i as isize), PERF_EVENT_IOC_RESET);
        i += 1;
    }
}

unsafe fn enable_fds(fd: *mut c_int, n: c_int) {
    let mut i: c_int = 0;

    while i < n {
        ioctl(*fd.offset(i as isize), PERF_EVENT_IOC_ENABLE);
        i += 1;
    }
}

unsafe fn disable_fds(fd: *mut c_int, n: c_int) {
    let mut i: c_int = 0;

    while i < n {
        ioctl(*fd.offset(i as isize), PERF_EVENT_IOC_DISABLE);
        i += 1;
    }
}

unsafe fn perf_systemwide_event_open(fd: *mut c_int, type_: __u32, addr: __u64, len: __u64) -> c_int {
    let mut i: c_int;
    let ncpus: c_int;
    let mut cpu: c_int;
    let mut ret: c_int = 0;
    let mut rlim: rlimit = zeroed();
    let mask: *mut cpu_set_t;
    let size: size_t;

    if getrlimit(RLIMIT_NOFILE, &mut rlim) != 0 {
        perror(c"getrlimit".as_ptr());
        return -1;
    }
    rlim.rlim_cur = 65536;
    if setrlimit(RLIMIT_NOFILE, &rlim) != 0 {
        perror(c"setrlimit".as_ptr());
        return -1;
    }

    ncpus = get_nprocs_conf();
    size = CPU_ALLOC_SIZE(ncpus);
    mask = CPU_ALLOC(ncpus);
    if mask.is_null() {
        perror(c"malloc".as_ptr());
        return -1;
    }

    CPU_ZERO_S(size, mask);

    if sched_getaffinity(0, size, mask) != 0 {
        perror(c"sched_getaffinity".as_ptr());
        ret = -1;
        CPU_FREE(mask);
        return ret;
    }

    i = 0;
    cpu = 0;
    while i < nprocs && cpu < ncpus {
        if CPU_ISSET_S(cpu, size, mask) == 0 {
            cpu += 1;
            continue;
        }
        *fd.offset(i as isize) = perf_cpu_event_open(cpu as c_long, type_, addr, len);
        if *fd.offset(i as isize) < 0 {
            perror(c"perf_systemwide_event_open".as_ptr());
            close_fds(fd, i);
            ret = *fd.offset(i as isize);
            CPU_FREE(mask);
            return ret;
        }
        i += 1;
        cpu += 1;
    }

    if i < nprocs {
        printf(
            c"Error: Number of online cpus reduced since start of test: %d < %d\n".as_ptr(),
            i,
            nprocs,
        );
        close_fds(fd, i);
        ret = -1;
    }

    CPU_FREE(mask);
    ret
}

unsafe fn breakpoint_test(len: c_int) -> bool_t {
    let fd: c_int;

    /* bp_addr can point anywhere but needs to be aligned */
    fd = perf_process_event_open(
        HW_BREAKPOINT_R as __u32,
        ((&fd as *const c_int as __u64) & 0xfffffffffffff800),
        len as __u64,
    );
    if fd < 0 {
        return false;
    }
    close(fd);
    true
}

unsafe fn perf_breakpoint_supported() -> bool_t {
    breakpoint_test(4)
}

unsafe fn dawr_supported() -> bool_t {
    breakpoint_test(DAWR_LENGTH_MAX as c_int)
}

unsafe fn runtestsingle(readwriteflag: c_int, exclude_user: c_int, arraytest: c_int) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut res: size_t;
    let mut breaks: u64 = 0;
    let mut needed: u64;
    let mut readint: c_int = 0;
    let mut readintarraybig: [c_int; 2 * DAWR_LENGTH_MAX / size_of::<c_int>()] =
        [0; 2 * DAWR_LENGTH_MAX / size_of::<c_int>()];
    let readintalign: *mut c_int;
    let mut ptr_: *mut c_int;
    let break_fd: c_int;
    let loop_num: c_int = MAX_LOOPS - (rand() % 100); /* provide some variability */
    let mut k: *mut c_int;
    let len: __u64;

    /* align to 0x400 boundary as required by DAWR */
    readintalign = (((readintarraybig.as_mut_ptr() as c_ulong) + 0x7ff)
        & 0xfffffffffffff800) as *mut c_int;

    ptr_ = &mut readint;
    if arraytest != 0 {
        ptr_ = readintalign.offset(0);
    }

    len = if arraytest != 0 {
        DAWR_LENGTH_MAX as __u64
    } else {
        size_of::<c_int>() as __u64
    };
    break_fd = perf_process_event_open_exclude_user(
        readwriteflag as __u32,
        ptr_ as __u64,
        len,
        exclude_user != 0,
    );
    if break_fd < 0 {
        perror(c"perf_process_event_open_exclude_user".as_ptr());
        exit(1);
    }

    /* start counters */
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);

    /* Test a bunch of reads and writes */
    k = &mut readint;
    i = 0;
    while i < loop_num {
        if arraytest != 0 {
            k = readintalign.offset((i % (DAWR_LENGTH_MAX as c_int / size_of::<c_int>() as c_int)) as isize);
        }

        j = ptr::read_volatile(k);
        ptr::write_volatile(k, j);
        i += 1;
    }

    /* stop counters */
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);

    /* read and check counters */
    res = read(
        break_fd,
        &mut breaks as *mut _ as *mut c_void,
        size_of::<u64>(),
    ) as size_t;
    assert!(res == size_of::<u64>());
    /* we read and write each loop, so subtract the ones we are counting */
    needed = 0;
    if (readwriteflag & HW_BREAKPOINT_R) != 0 {
        needed = needed.wrapping_add(loop_num as u64);
    }
    if (readwriteflag & HW_BREAKPOINT_W) != 0 {
        needed = needed.wrapping_add(loop_num as u64);
    }
    needed = needed.wrapping_mul((1 - exclude_user) as u64);
    printf(
        c"TESTED: addr:0x%lx brks:% 8lld loops:% 8i rw:%i !user:%i array:%i\n".as_ptr(),
        ptr_ as c_ulong,
        breaks,
        loop_num,
        readwriteflag,
        exclude_user,
        arraytest,
    );
    if breaks != needed {
        printf(
            c"FAILED: 0x%lx brks:%lld needed:%lli %i %i %i\n\n".as_ptr(),
            ptr_ as c_ulong,
            breaks,
            needed,
            loop_num,
            readwriteflag,
            exclude_user,
        );
        return 1;
    }
    close(break_fd);

    0
}

unsafe fn runtest_dar_outside() -> c_int {
    let target: *mut c_void;
    let mut temp16: __u16;
    let mut temp64: __u64;
    let break_fd: c_int;
    let mut breaks: u64 = 0;
    let mut fail: c_int = 0;
    let mut res: size_t;

    target = malloc(8);
    if target.is_null() {
        perror(c"malloc failed".as_ptr());
        exit(EXIT_FAILURE);
    }

    /* watch middle half of target array */
    break_fd = perf_process_event_open(HW_BREAKPOINT_RW as __u32, (target as *mut u8).offset(2) as __u64, 4);
    if break_fd < 0 {
        free(target);
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    /* Shouldn't hit. */
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = ptr::read_volatile(target as *const __u16);
    ptr::write_volatile(target as *mut __u16, temp16);
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &mut breaks as *mut _ as *mut c_void, size_of::<u64>()) as size_t;
    assert!(res == size_of::<u64>());
    if breaks == 0 {
        printf(c"TESTED: No overlap\n".as_ptr());
    } else {
        printf(c"FAILED: No overlap: %lld != 0\n".as_ptr(), breaks);
        fail = 1;
    }

    /* Hit */
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = ptr::read_volatile((target as *mut u8).offset(1) as *const __u16);
    ptr::write_volatile((target as *mut u8).offset(1) as *mut __u16, temp16);
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &mut breaks as *mut _ as *mut c_void, size_of::<u64>()) as size_t;
    assert!(res == size_of::<u64>());
    if breaks == 2 {
        printf(c"TESTED: Partial overlap\n".as_ptr());
    } else {
        printf(c"FAILED: Partial overlap: %lld != 2\n".as_ptr(), breaks);
        fail = 1;
    }

    /* Hit */
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = ptr::read_volatile((target as *mut u8).offset(5) as *const __u16);
    ptr::write_volatile((target as *mut u8).offset(5) as *mut __u16, temp16);
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &mut breaks as *mut _ as *mut c_void, size_of::<u64>()) as size_t;
    assert!(res == size_of::<u64>());
    if breaks == 2 {
        printf(c"TESTED: Partial overlap\n".as_ptr());
    } else {
        printf(c"FAILED: Partial overlap: %lld != 2\n".as_ptr(), breaks);
        fail = 1;
    }

    /* Shouldn't Hit */
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = ptr::read_volatile((target as *mut u8).offset(6) as *const __u16);
    ptr::write_volatile((target as *mut u8).offset(6) as *mut __u16, temp16);
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &mut breaks as *mut _ as *mut c_void, size_of::<u64>()) as size_t;
    assert!(res == size_of::<u64>());
    if breaks == 0 {
        printf(c"TESTED: No overlap\n".as_ptr());
    } else {
        printf(c"FAILED: No overlap: %lld != 0\n".as_ptr(), breaks);
        fail = 1;
    }

    /* Hit */
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp64 = ptr::read_volatile(target as *const __u64);
    ptr::write_volatile(target as *mut __u64, temp64);
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &mut breaks as *mut _ as *mut c_void, size_of::<u64>()) as size_t;
    assert!(res == size_of::<u64>());
    if breaks == 2 {
        printf(c"TESTED: Full overlap\n".as_ptr());
    } else {
        printf(c"FAILED: Full overlap: %lld != 2\n".as_ptr(), breaks);
        fail = 1;
    }

    free(target);
    close(break_fd);
    fail
}

unsafe fn multi_dawr_workload() {
    ptr::write_volatile(&raw mut a, ptr::read_volatile(&raw const a).wrapping_add(10));
    ptr::write_volatile(&raw mut b, ptr::read_volatile(&raw const b).wrapping_add(10));
    ptr::write_volatile(
        &mut c.0[512 + 1],
        ptr::read_volatile(&c.0[512 + 1]).wrapping_add(b'a' as c_char),
    );
}

unsafe fn test_process_multi_diff_addr() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1: c_int;
    let fd2: c_int;
    let desc = c"Process specific, Two events, diff addr";
    let mut res: size_t;

    fd1 = perf_process_event_open(HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd1 < 0 {
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    fd2 = perf_process_event_open(HW_BREAKPOINT_RW as __u32, &raw const b as __u64, size_of::<c_int>() as __u64);
    if fd2 < 0 {
        close(fd1);
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);

    res = read(fd1, &mut breaks1 as *mut _ as *mut c_void, size_of_val(&breaks1)) as size_t;
    assert!(res == size_of::<u64>());
    res = read(fd2, &mut breaks2 as *mut _ as *mut c_void, size_of_val(&breaks2)) as size_t;
    assert!(res == size_of::<u64>());

    close(fd1);
    close(fd2);

    if breaks1 != 2 || breaks2 != 2 {
        printf(c"FAILED: %s: %lld != 2 || %lld != 2\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_process_multi_same_addr() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1: c_int;
    let fd2: c_int;
    let desc = c"Process specific, Two events, same addr";
    let mut res: size_t;

    fd1 = perf_process_event_open(HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd1 < 0 {
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    fd2 = perf_process_event_open(HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd2 < 0 {
        close(fd1);
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);

    res = read(fd1, &mut breaks1 as *mut _ as *mut c_void, size_of_val(&breaks1)) as size_t;
    assert!(res == size_of::<u64>());
    res = read(fd2, &mut breaks2 as *mut _ as *mut c_void, size_of_val(&breaks2)) as size_t;
    assert!(res == size_of::<u64>());

    close(fd1);
    close(fd2);

    if breaks1 != 2 || breaks2 != 2 {
        printf(c"FAILED: %s: %lld != 2 || %lld != 2\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_process_multi_diff_addr_ro_wo() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1: c_int;
    let fd2: c_int;
    let desc = c"Process specific, Two events, diff addr, one is RO, other is WO";
    let mut res: size_t;

    fd1 = perf_process_event_open(HW_BREAKPOINT_W as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd1 < 0 {
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    fd2 = perf_process_event_open(HW_BREAKPOINT_R as __u32, &raw const b as __u64, size_of::<c_int>() as __u64);
    if fd2 < 0 {
        close(fd1);
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);

    res = read(fd1, &mut breaks1 as *mut _ as *mut c_void, size_of_val(&breaks1)) as size_t;
    assert!(res == size_of::<u64>());
    res = read(fd2, &mut breaks2 as *mut _ as *mut c_void, size_of_val(&breaks2)) as size_t;
    assert!(res == size_of::<u64>());

    close(fd1);
    close(fd2);

    if breaks1 != 1 || breaks2 != 1 {
        printf(c"FAILED: %s: %lld != 1 || %lld != 1\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_process_multi_same_addr_ro_wo() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1: c_int;
    let fd2: c_int;
    let desc = c"Process specific, Two events, same addr, one is RO, other is WO";
    let mut res: size_t;

    fd1 = perf_process_event_open(HW_BREAKPOINT_R as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd1 < 0 {
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    fd2 = perf_process_event_open(HW_BREAKPOINT_W as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if fd2 < 0 {
        close(fd1);
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);

    res = read(fd1, &mut breaks1 as *mut _ as *mut c_void, size_of_val(&breaks1)) as size_t;
    assert!(res == size_of::<u64>());
    res = read(fd2, &mut breaks2 as *mut _ as *mut c_void, size_of_val(&breaks2)) as size_t;
    assert!(res == size_of::<u64>());

    close(fd1);
    close(fd2);

    if breaks1 != 1 || breaks2 != 1 {
        printf(c"FAILED: %s: %lld != 1 || %lld != 1\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_syswide_multi_diff_addr() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let fd2 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let desc = c"Systemwide, Two events, diff addr";
    let mut ret: c_int;

    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        exit(EXIT_FAILURE);
    }

    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_RW as __u32, &raw const b as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        close_fds(fd1, nprocs);
        exit(EXIT_FAILURE);
    }

    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);

    breaks1 = read_fds(fd1, nprocs) as u64;
    breaks2 = read_fds(fd2, nprocs) as u64;

    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);

    free(fd1 as *mut c_void);
    free(fd2 as *mut c_void);

    if breaks1 != 2 || breaks2 != 2 {
        printf(c"FAILED: %s: %lld != 2 || %lld != 2\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_syswide_multi_same_addr() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let fd2 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let desc = c"Systemwide, Two events, same addr";
    let mut ret: c_int;

    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        exit(EXIT_FAILURE);
    }

    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_RW as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        close_fds(fd1, nprocs);
        exit(EXIT_FAILURE);
    }

    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);

    breaks1 = read_fds(fd1, nprocs) as u64;
    breaks2 = read_fds(fd2, nprocs) as u64;

    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);

    free(fd1 as *mut c_void);
    free(fd2 as *mut c_void);

    if breaks1 != 2 || breaks2 != 2 {
        printf(c"FAILED: %s: %lld != 2 || %lld != 2\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_syswide_multi_diff_addr_ro_wo() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let fd2 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let desc = c"Systemwide, Two events, diff addr, one is RO, other is WO";
    let mut ret: c_int;

    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_W as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        exit(EXIT_FAILURE);
    }

    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_R as __u32, &raw const b as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        close_fds(fd1, nprocs);
        exit(EXIT_FAILURE);
    }

    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);

    breaks1 = read_fds(fd1, nprocs) as u64;
    breaks2 = read_fds(fd2, nprocs) as u64;

    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);

    free(fd1 as *mut c_void);
    free(fd2 as *mut c_void);

    if breaks1 != 1 || breaks2 != 1 {
        printf(c"FAILED: %s: %lld != 1 || %lld != 1\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn test_syswide_multi_same_addr_ro_wo() -> c_int {
    let mut breaks1: u64 = 0;
    let mut breaks2: u64 = 0;
    let fd1 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let fd2 = malloc(nprocs as size_t * size_of::<c_int>()) as *mut c_int;
    let desc = c"Systemwide, Two events, same addr, one is RO, other is WO";
    let mut ret: c_int;

    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_W as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        exit(EXIT_FAILURE);
    }

    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_R as __u32, &raw const a as __u64, size_of::<c_int>() as __u64);
    if ret != 0 {
        close_fds(fd1, nprocs);
        exit(EXIT_FAILURE);
    }

    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);

    breaks1 = read_fds(fd1, nprocs) as u64;
    breaks2 = read_fds(fd2, nprocs) as u64;

    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);

    free(fd1 as *mut c_void);
    free(fd2 as *mut c_void);

    if breaks1 != 1 || breaks2 != 1 {
        printf(c"FAILED: %s: %lld != 1 || %lld != 1\n".as_ptr(), desc.as_ptr(), breaks1, breaks2);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

unsafe fn runtest_multi_dawr() -> c_int {
    let mut ret: c_int = 0;

    ret |= test_process_multi_diff_addr();
    ret |= test_process_multi_same_addr();
    ret |= test_process_multi_diff_addr_ro_wo();
    ret |= test_process_multi_same_addr_ro_wo();
    ret |= test_syswide_multi_diff_addr();
    ret |= test_syswide_multi_same_addr();
    ret |= test_syswide_multi_diff_addr_ro_wo();
    ret |= test_syswide_multi_same_addr_ro_wo();

    ret
}

unsafe fn runtest_unaligned_512bytes() -> c_int {
    let mut breaks: u64 = 0;
    let fd: c_int;
    let desc = c"Process specific, 512 bytes, unaligned";
    let addr: __u64 = (&raw const c as __u64).wrapping_add(8);
    let mut res: size_t;

    fd = perf_process_event_open(HW_BREAKPOINT_RW as __u32, addr, 512);
    if fd < 0 {
        perror(c"perf_process_event_open".as_ptr());
        exit(EXIT_FAILURE);
    }

    ioctl(fd, PERF_EVENT_IOC_RESET);
    ioctl(fd, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd, PERF_EVENT_IOC_DISABLE);

    res = read(fd, &mut breaks as *mut _ as *mut c_void, size_of_val(&breaks)) as size_t;
    assert!(res == size_of::<u64>());

    close(fd);

    if breaks != 2 {
        printf(c"FAILED: %s: %lld != 2\n".as_ptr(), desc.as_ptr(), breaks);
        return 1;
    }

    printf(c"TESTED: %s\n".as_ptr(), desc.as_ptr());
    0
}

/* There is no perf api to find number of available watchpoints. Use ptrace. */
unsafe fn get_nr_wps(arch_31: *mut bool_t) -> c_int {
    let mut dbginfo: ppc_debug_info = zeroed();
    let child_pid: c_int;

    child_pid = fork();
    if child_pid == 0 {
        let ret: c_int = ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), 0) as c_int;
        if ret != 0 {
            perror(c"PTRACE_TRACEME failed\n".as_ptr());
            exit(EXIT_FAILURE);
        }
        kill(getpid(), SIGUSR1);

        sleep(1);
        exit(EXIT_SUCCESS);
    }

    wait(ptr::null_mut());
    if ptrace(
        PPC_PTRACE_GETHWDBGINFO,
        child_pid,
        ptr::null_mut::<c_void>(),
        &mut dbginfo as *mut _,
    ) != 0
    {
        perror(c"Can't get breakpoint info".as_ptr());
        exit(EXIT_FAILURE);
    }

    *arch_31 = (dbginfo.features & PPC_DEBUG_FEATURE_DATA_BP_ARCH_31) != 0;
    dbginfo.num_data_bps as c_int
}

unsafe fn runtest() -> c_int {
    let mut rwflag: c_int;
    let mut exclude_user: c_int;
    let mut ret: c_int = 0;
    let dawr: bool_t = dawr_supported();
    let mut arch_31: bool_t = false;
    let nr_wps: c_int = get_nr_wps(&mut arch_31);

    /*
     * perf defines rwflag as two bits read and write and at least
     * one must be set.  So range 1-3.
     */
    rwflag = 1;
    while rwflag < 4 {
        exclude_user = 0;
        while exclude_user < 2 {
            ret = runtestsingle(rwflag, exclude_user, 0);
            if ret != 0 {
                return ret;
            }

            /* if we have the dawr, we can do an array test */
            if !dawr {
                exclude_user += 1;
                continue;
            }
            ret = runtestsingle(rwflag, exclude_user, 1);
            if ret != 0 {
                return ret;
            }
            exclude_user += 1;
        }
        rwflag += 1;
    }

    ret = runtest_dar_outside();
    if ret != 0 {
        return ret;
    }

    if dawr && nr_wps > 1 {
        nprocs = get_nprocs();
        ret = runtest_multi_dawr();
        if ret != 0 {
            return ret;
        }
    }

    if dawr && arch_31 {
        ret = runtest_unaligned_512bytes();
    }

    ret
}

unsafe extern "C" fn perf_hwbreak() -> c_int {
    srand(time(ptr::null_mut()) as u32);

    SKIP_IF_MSG(!perf_breakpoint_supported(), c"Perf breakpoints not supported".as_ptr());

    runtest()
}

unsafe fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char, _envp: *mut *mut c_char) -> c_int {
    test_harness(perf_hwbreak, c"perf_hwbreak".as_ptr())
}
