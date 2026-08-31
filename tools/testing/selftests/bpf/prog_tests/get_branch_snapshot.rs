// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Original C dependencies:
 * #include <test_progs.h>
 * #include "get_branch_snapshot.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_event_attr {
    size: u32,
    type_: u32,
    config: u64,
    sample_type: u64,
    branch_sample_type: u64,
}

#[repr(C)]
struct get_branch_snapshot_bss {
    address_low: u64,
    address_high: u64,
    total_entries: i32,
    test1_hits: i32,
    wasted_entries: i32,
}

#[repr(C)]
struct get_branch_snapshot {
    bss: *mut get_branch_snapshot_bss,
}

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_BRANCH_USER: u64 = 1 << 0;
const PERF_SAMPLE_BRANCH_KERNEL: u64 = 1 << 1;
const PERF_SAMPLE_BRANCH_ANY: u64 = 1 << 3;
const PERF_FLAG_FD_CLOEXEC: u64 = 1 << 3;
const __NR_PERF_EVENT_OPEN: c_long = 298;

static mut pfd_array: *mut c_int = core::ptr::null_mut();
static mut cpu_cnt: c_int = 0;

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn fclose(stream: *mut FILE) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn syscall(number: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;

    fn libbpf_num_possible_cpus() -> c_int;
    fn test__skip();
    fn get_branch_snapshot__open_and_load() -> *mut get_branch_snapshot;
    fn get_branch_snapshot__attach(skel: *mut get_branch_snapshot) -> c_int;
    fn get_branch_snapshot__destroy(skel: *mut get_branch_snapshot);
    fn kallsyms_find(sym: *const c_char, addr: *mut u64) -> c_int;
    fn trigger_module_test_read(read_sz: c_int);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn is_hypervisor() -> bool {
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut ret = false;
    let mut len: usize = 0;
    let fp: *mut FILE;

    fp = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return false;
    }

    while getline(&mut line, &mut len, fp) != -1 {
        if strncmp(line, c"flags".as_ptr(), 5) == 0 {
            if !strstr(line, c"hypervisor".as_ptr()).is_null() {
                ret = true;
            }
            break;
        }
    }

    free(line as *mut c_void);
    fclose(fp);
    ret
}

unsafe fn create_perf_events() -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut cpu: c_int;

    /* create perf event */
    attr.size = core::mem::size_of_val(&attr) as u32;
    attr.type_ = PERF_TYPE_HARDWARE;
    attr.config = PERF_COUNT_HW_CPU_CYCLES;
    attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    attr.branch_sample_type =
        PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_ANY;

    cpu_cnt = libbpf_num_possible_cpus();
    pfd_array = malloc(core::mem::size_of::<c_int>() * cpu_cnt as usize) as *mut c_int;
    if pfd_array.is_null() {
        cpu_cnt = 0;
        return 1;
    }

    cpu = 0;
    while cpu < cpu_cnt {
        *pfd_array.add(cpu as usize) = syscall(
            __NR_PERF_EVENT_OPEN,
            &mut attr as *mut perf_event_attr,
            -1,
            cpu,
            -1,
            PERF_FLAG_FD_CLOEXEC,
        ) as c_int;
        if *pfd_array.add(cpu as usize) < 0 {
            break;
        }
        cpu += 1;
    }

    (cpu == 0) as c_int
}

unsafe fn close_perf_events() {
    let mut cpu: c_int;
    let mut fd: c_int;

    cpu = 0;
    while cpu < cpu_cnt {
        fd = *pfd_array.add(cpu as usize);
        if fd < 0 {
            break;
        }
        close(fd);
        cpu += 1;
    }
    free(pfd_array as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_get_branch_snapshot() {
    let mut skel: *mut get_branch_snapshot = core::ptr::null_mut();
    let mut err: c_int;

    /* Skip the test before we fix LBR snapshot for hypervisor. */
    if is_hypervisor() {
        test__skip();
        return;
    }

    if create_perf_events() != 0 {
        test__skip(); /* system doesn't support LBR */
        /* goto cleanup; */
        get_branch_snapshot__destroy(skel);
        close_perf_events();
        return;
    }

    skel = get_branch_snapshot__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"get_branch_snapshot__open_and_load".as_ptr()) {
        get_branch_snapshot__destroy(skel);
        close_perf_events();
        return;
    }

    err = kallsyms_find(
        c"bpf_testmod_loop_test".as_ptr(),
        &mut (*(*skel).bss).address_low,
    );
    if !ASSERT_OK(err, c"kallsyms_find".as_ptr()) {
        get_branch_snapshot__destroy(skel);
        close_perf_events();
        return;
    }

    /* Just a guess for the end of this function, as module functions
     * in /proc/kallsyms could come in any order.
     */
    (*(*skel).bss).address_high = (*(*skel).bss).address_low + 128;

    err = get_branch_snapshot__attach(skel);
    if !ASSERT_OK(err, c"get_branch_snapshot__attach".as_ptr()) {
        get_branch_snapshot__destroy(skel);
        close_perf_events();
        return;
    }

    trigger_module_test_read(100);

    if (*(*skel).bss).total_entries < 16 {
        /* too few entries for the hit/waste test */
        test__skip();
        get_branch_snapshot__destroy(skel);
        close_perf_events();
        return;
    }

    ASSERT_GT((*(*skel).bss).test1_hits, 6, c"find_looptest_in_lbr".as_ptr());

    /* Given we stop LBR in software, we will waste a few entries.
     * But we should try to waste as few as possible entries. We are at
     * about 7 on x86_64 systems.
     * Add a check for < 10 so that we get heads-up when something
     * changes and wastes too many entries.
     */
    ASSERT_LT(
        (*(*skel).bss).wasted_entries,
        10,
        c"check_wasted_entries".as_ptr(),
    );

    /* cleanup: */
    get_branch_snapshot__destroy(skel);
    close_perf_events();
}
