// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

type pid_t = c_int;
type size_t = usize;
type pthread_t = libc::pthread_t;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const USEC_PER_SEC: c_long = 1_000_000;
const NSEC_PER_SEC: c_long = 1_000_000_000;
const NSEC_PER_USEC: c_long = 1_000;
const PATH_MAX: usize = 4096;
const CLOCK_MONOTONIC: libc::clockid_t = libc::CLOCK_MONOTONIC;
const CLOCK_PROCESS_CPUTIME_ID: libc::clockid_t = libc::CLOCK_PROCESS_CPUTIME_ID;
const EINTR: c_int = libc::EINTR;
const EXIT_FAILURE: c_int = libc::EXIT_FAILURE;
const EOF: c_int = -1;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum hog_clock_type {
    // Count elapsed time using the CLOCK_PROCESS_CPUTIME_ID clock.
    CPU_HOG_CLOCK_PROCESS,
    // Count elapsed time using system wallclock time.
    CPU_HOG_CLOCK_WALL,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct cpu_hogger {
    cgroup: *mut c_char,
    pid: pid_t,
    usage: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct cpu_hog_func_param {
    nprocs: c_int,
    ts: libc::timespec,
    clock_type: hog_clock_type,
}

#[repr(C)]
struct cpucg_test {
    fn_: unsafe extern "C" fn(*const c_char) -> c_int,
    name: *const c_char,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_name_indexed(root: *const c_char, name: *const c_char, index: c_int) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_write_numeric(cgroup: *const c_char, control: *const c_char, value: c_long) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
    fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long;
    fn cg_run(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn cg_find_unified_root(root: *mut c_char, len: size_t, mount: *mut c_void) -> c_int;
    fn values_close_report(actual: c_long, expected: c_long, percentage: c_int) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished() -> !;

    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn nice(inc: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn get_nprocs() -> c_int;
    fn clock_gettime(clk_id: libc::clockid_t, tp: *mut libc::timespec) -> c_int;
    fn nanosleep(req: *const libc::timespec, rem: *mut libc::timespec) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut libc::FILE;
    fn pclose(stream: *mut libc::FILE) -> c_int;
    fn fscanf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int {
    N as c_int
}

unsafe fn MIN(a: c_long, b: c_long) -> c_long {
    if a < b { a } else { b }
}

/*
 * This test creates two nested cgroups with and without enabling
 * the cpu controller.
 */
unsafe extern "C" fn test_cpucg_subtree_control(root: *const c_char) -> c_int {
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut child: *mut c_char = core::ptr::null_mut();
    let mut parent2: *mut c_char = core::ptr::null_mut();
    let mut child2: *mut c_char = core::ptr::null_mut();
    let mut ret: c_int = KSFT_FAIL;

    // Create two nested cgroups with the cpu controller enabled.
    parent = cg_name(root, c"cpucg_test_0".as_ptr());
    if parent.is_null() {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_create(parent) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    child = cg_name(parent, c"cpucg_test_child".as_ptr());
    if child.is_null() {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_create(child) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_read_strstr(child, c"cgroup.controllers".as_ptr(), c"cpu".as_ptr()) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    // Create two nested cgroups without enabling the cpu controller.
    parent2 = cg_name(root, c"cpucg_test_1".as_ptr());
    if parent2.is_null() {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_create(parent2) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    child2 = cg_name(parent2, c"cpucg_test_child".as_ptr());
    if child2.is_null() {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_create(child2) != 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    if cg_read_strstr(child2, c"cgroup.controllers".as_ptr(), c"cpu".as_ptr()) == 0 {
        goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
        return ret;
    }

    ret = KSFT_PASS;
    goto_cleanup_cpucg_subtree_control(&mut ret, parent, child, parent2, child2);
    ret
}

unsafe fn goto_cleanup_cpucg_subtree_control(
    _ret: &mut c_int,
    parent: *mut c_char,
    child: *mut c_char,
    parent2: *mut c_char,
    child2: *mut c_char,
) {
    cg_destroy(child);
    free(child as *mut c_void);
    cg_destroy(child2);
    free(child2 as *mut c_void);
    cg_destroy(parent);
    free(parent as *mut c_void);
    cg_destroy(parent2);
    free(parent2 as *mut c_void);
}

unsafe extern "C" fn hog_cpu_thread_func(_arg: *mut c_void) -> *mut c_void {
    loop {}
}

unsafe fn timespec_sub(lhs: *const libc::timespec, rhs: *const libc::timespec) -> libc::timespec {
    let zero = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    let mut ret: libc::timespec;

    if (*lhs).tv_sec < (*rhs).tv_sec {
        return zero;
    }

    ret = libc::timespec {
        tv_sec: (*lhs).tv_sec - (*rhs).tv_sec,
        tv_nsec: 0,
    };

    if (*lhs).tv_nsec < (*rhs).tv_nsec {
        if ret.tv_sec == 0 {
            return zero;
        }

        ret.tv_sec -= 1;
        ret.tv_nsec = NSEC_PER_SEC - (*rhs).tv_nsec + (*lhs).tv_nsec;
    } else {
        ret.tv_nsec = (*lhs).tv_nsec - (*rhs).tv_nsec;
    }

    ret
}

unsafe extern "C" fn hog_cpus_timed(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let param = arg as *const cpu_hog_func_param;
    let ts_run = (*param).ts;
    let mut ts_remaining = ts_run;
    let mut ts_start: libc::timespec = core::mem::zeroed();
    let mut i: c_int;
    let mut ret: c_int;

    ret = clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < (*param).nprocs {
        let mut tid: pthread_t = core::mem::zeroed();

        ret = pthread_create(&mut tid, core::ptr::null(), hog_cpu_thread_func, core::ptr::null_mut());
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    while ts_remaining.tv_sec > 0 || ts_remaining.tv_nsec > 0 {
        let mut ts_total: libc::timespec = core::mem::zeroed();

        ret = nanosleep(&ts_remaining, core::ptr::null_mut());
        if ret != 0 && errno != EINTR {
            return ret;
        }

        if (*param).clock_type == hog_clock_type::CPU_HOG_CLOCK_PROCESS {
            ret = clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut ts_total);
            if ret != 0 {
                return ret;
            }
        } else {
            let mut ts_current: libc::timespec = core::mem::zeroed();

            ret = clock_gettime(CLOCK_MONOTONIC, &mut ts_current);
            if ret != 0 {
                return ret;
            }

            ts_total = timespec_sub(&ts_current, &ts_start);
        }

        ts_remaining = timespec_sub(&ts_run, &ts_total);
    }

    0
}

unsafe extern "C" fn test_cpucg_stats(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut usage_usec: c_long;
    let mut user_usec: c_long;
    let system_usec: c_long;
    let usage_seconds: c_long = 2;
    let expected_usage_usec = usage_seconds * USEC_PER_SEC;
    let cpucg = cg_name(root, c"cpucg_test".as_ptr());

    if cpucg.is_null() {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }
    if cg_create(cpucg) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    usage_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
    user_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"user_usec".as_ptr());
    system_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"system_usec".as_ptr());
    if usage_usec != 0 || user_usec != 0 || system_usec != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    let mut param = cpu_hog_func_param {
        nprocs: 1,
        ts: libc::timespec { tv_sec: usage_seconds, tv_nsec: 0 },
        clock_type: hog_clock_type::CPU_HOG_CLOCK_PROCESS,
    };
    if cg_run(cpucg, hog_cpus_timed, &mut param as *mut _ as *mut c_void) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    usage_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
    user_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"user_usec".as_ptr());
    if user_usec <= 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    if values_close_report(usage_usec, expected_usage_usec, 1) == 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    ret = KSFT_PASS;
    cg_destroy(cpucg);
    free(cpucg as *mut c_void);
    ret
}

unsafe extern "C" fn test_cpucg_nice(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut status: c_int = 0;
    let mut user_usec: c_long;
    let mut nice_usec: c_long;
    let usage_seconds: c_long = 2;
    let expected_nice_usec = usage_seconds * USEC_PER_SEC;
    let cpucg = cg_name(root, c"cpucg_test".as_ptr());
    let pid: pid_t;

    if cpucg.is_null() {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }
    if cg_create(cpucg) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    user_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"user_usec".as_ptr());
    nice_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"nice_usec".as_ptr());
    if nice_usec == -1 {
        ret = KSFT_SKIP;
    }
    if user_usec != 0 || nice_usec != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    /*
     * We fork here to create a new process that can be niced without
     * polluting the nice value of other selftests
     */
    pid = fork();
    if pid < 0 {
    } else if pid == 0 {
        let mut param = cpu_hog_func_param {
            nprocs: 1,
            ts: libc::timespec { tv_sec: usage_seconds, tv_nsec: 0 },
            clock_type: hog_clock_type::CPU_HOG_CLOCK_PROCESS,
        };
        let mut buf = [0 as c_char; 64];
        snprintf(buf.as_mut_ptr(), buf.len(), c"%d".as_ptr(), getpid());
        if cg_write(cpucg, c"cgroup.procs".as_ptr(), buf.as_ptr()) != 0 {
            exit(EXIT_FAILURE);
        }

        /* Try to keep niced CPU usage as constrained to hog_cpu as possible */
        nice(1);
        hog_cpus_timed(cpucg, &mut param as *mut _ as *mut c_void);
        exit(0);
    } else {
        waitpid(pid, &mut status, 0);
        if !WIFEXITED(status) {
            cg_destroy(cpucg);
            free(cpucg as *mut c_void);
            return ret;
        }

        user_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"user_usec".as_ptr());
        nice_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"nice_usec".as_ptr());
        if user_usec <= 0 {
            cg_destroy(cpucg);
            free(cpucg as *mut c_void);
            return ret;
        }
        if values_close_report(nice_usec, expected_nice_usec, 1) == 0 {
            cg_destroy(cpucg);
            free(cpucg as *mut c_void);
            return ret;
        }

        ret = KSFT_PASS;
    }

    cg_destroy(cpucg);
    free(cpucg as *mut c_void);
    ret
}

unsafe fn cleanup_cpu_hoggers(children: &mut [cpu_hogger], parent: *mut c_char) {
    let mut i = 0;
    while i < children.len() {
        cg_destroy(children[i].cgroup);
        free(children[i].cgroup as *mut c_void);
        i += 1;
    }
    cg_destroy(parent);
    free(parent as *mut c_void);
}

unsafe fn run_cpucg_weight_test(
    root: *const c_char,
    spawn_child: unsafe fn(*const cpu_hogger) -> pid_t,
    validate: unsafe fn(*const cpu_hogger, c_int) -> c_int,
) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut i: c_int;
    let mut parent: *mut c_char = core::ptr::null_mut();
    let mut children = [cpu_hogger { cgroup: core::ptr::null_mut(), pid: 0, usage: 0 }; 3];

    parent = cg_name(root, c"cpucg_test_0".as_ptr());
    if parent.is_null() {
        cleanup_cpu_hoggers(&mut children, parent);
        return ret;
    }
    if cg_create(parent) != 0 {
        cleanup_cpu_hoggers(&mut children, parent);
        return ret;
    }
    if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
        cleanup_cpu_hoggers(&mut children, parent);
        return ret;
    }

    i = 0;
    while i < ARRAY_SIZE(&children) {
        children[i as usize].cgroup = cg_name_indexed(parent, c"cpucg_child".as_ptr(), i);
        if children[i as usize].cgroup.is_null() {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        if cg_create(children[i as usize].cgroup) != 0 {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        if cg_write_numeric(children[i as usize].cgroup, c"cpu.weight".as_ptr(), 50 * (i + 1) as c_long) != 0 {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&children) {
        let pid = spawn_child(&children[i as usize]);
        if pid <= 0 {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        children[i as usize].pid = pid;
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&children) {
        let mut retcode: c_int = 0;

        waitpid(children[i as usize].pid, &mut retcode, 0);
        if !WIFEXITED(retcode) {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        if WEXITSTATUS(retcode) != 0 {
            cleanup_cpu_hoggers(&mut children, parent);
            return ret;
        }
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&children) {
        children[i as usize].usage =
            cg_read_key_long(children[i as usize].cgroup, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
        i += 1;
    }

    if validate(children.as_ptr(), ARRAY_SIZE(&children)) != 0 {
        cleanup_cpu_hoggers(&mut children, parent);
        return ret;
    }

    ret = KSFT_PASS;
    cleanup_cpu_hoggers(&mut children, parent);
    ret
}

unsafe fn weight_hog_ncpus(child: *const cpu_hogger, ncpus: c_int) -> pid_t {
    let usage_seconds: c_long = 10;
    let mut param = cpu_hog_func_param {
        nprocs: ncpus,
        ts: libc::timespec { tv_sec: usage_seconds, tv_nsec: 0 },
        clock_type: hog_clock_type::CPU_HOG_CLOCK_WALL,
    };
    cg_run_nowait((*child).cgroup, hog_cpus_timed, &mut param as *mut _ as *mut c_void)
}

unsafe fn weight_hog_all_cpus(child: *const cpu_hogger) -> pid_t {
    weight_hog_ncpus(child, get_nprocs())
}

unsafe fn overprovision_validate(children: *const cpu_hogger, num_children: c_int) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut i = 0;

    while i < num_children - 1 {
        let mut delta: c_long;

        if (*children.add((i + 1) as usize)).usage <= (*children.add(i as usize)).usage {
            return ret;
        }

        delta = (*children.add((i + 1) as usize)).usage - (*children.add(i as usize)).usage;
        if values_close_report(delta, (*children).usage, 35) == 0 {
            return ret;
        }
        i += 1;
    }

    ret = KSFT_PASS;
    ret
}

unsafe extern "C" fn test_cpucg_weight_overprovisioned(root: *const c_char) -> c_int {
    run_cpucg_weight_test(root, weight_hog_all_cpus, overprovision_validate)
}

unsafe fn weight_hog_one_cpu(child: *const cpu_hogger) -> pid_t {
    weight_hog_ncpus(child, 1)
}

unsafe fn underprovision_validate(children: *const cpu_hogger, num_children: c_int) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut i = 0;

    while i < num_children - 1 {
        if values_close_report((*children.add((i + 1) as usize)).usage, (*children).usage, 15) == 0 {
            return ret;
        }
        i += 1;
    }

    ret = KSFT_PASS;
    ret
}

unsafe extern "C" fn test_cpucg_weight_underprovisioned(root: *const c_char) -> c_int {
    // Only run the test if there are enough cores to avoid overprovisioning
    // the system.
    if get_nprocs() < 4 {
        return KSFT_SKIP;
    }

    run_cpucg_weight_test(root, weight_hog_one_cpu, underprovision_validate)
}

unsafe fn cleanup_nested_leaf(leaf: &mut [cpu_hogger], child: *mut c_char, parent: *mut c_char) {
    let mut i = 0;
    while i < leaf.len() {
        cg_destroy(leaf[i].cgroup);
        free(leaf[i].cgroup as *mut c_void);
        i += 1;
    }
    cg_destroy(child);
    free(child as *mut c_void);
    cg_destroy(parent);
    free(parent as *mut c_void);
}

unsafe fn run_cpucg_nested_weight_test(root: *const c_char, overprovisioned: bool) -> c_int {
    let mut ret = KSFT_FAIL;
    let mut i: c_int;
    let mut parent: *mut c_char;
    let mut child: *mut c_char;
    let mut leaf = [cpu_hogger { cgroup: core::ptr::null_mut(), pid: 0, usage: 0 }; 3];
    let nested_leaf_usage: c_long;
    let child_usage: c_long;
    let mut nprocs = get_nprocs();

    if !overprovisioned {
        if nprocs < 4 {
            /*
             * Only run the test if there are enough cores to avoid overprovisioning
             * the system.
             */
            return KSFT_SKIP;
        }
        nprocs /= 4;
    }

    parent = cg_name(root, c"cpucg_test".as_ptr());
    child = cg_name(parent, c"cpucg_child".as_ptr());
    if parent.is_null() || child.is_null() {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }

    if cg_create(parent) != 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }
    if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }

    if cg_create(child) != 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }
    if cg_write(child, c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }
    if cg_write(child, c"cpu.weight".as_ptr(), c"1000".as_ptr()) != 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }

    i = 0;
    while i < ARRAY_SIZE(&leaf) {
        let ancestor: *const c_char;
        let weight: c_long;

        if i == 0 {
            ancestor = parent;
            weight = 1000;
        } else {
            ancestor = child;
            weight = 5000;
        }
        leaf[i as usize].cgroup = cg_name_indexed(ancestor, c"cpucg_leaf".as_ptr(), i);
        if leaf[i as usize].cgroup.is_null() {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }

        if cg_create(leaf[i as usize].cgroup) != 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }

        if cg_write_numeric(leaf[i as usize].cgroup, c"cpu.weight".as_ptr(), weight) != 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&leaf) {
        let pid: pid_t;
        let mut param = cpu_hog_func_param {
            nprocs,
            ts: libc::timespec { tv_sec: 10, tv_nsec: 0 },
            clock_type: hog_clock_type::CPU_HOG_CLOCK_WALL,
        };

        pid = cg_run_nowait(leaf[i as usize].cgroup, hog_cpus_timed, &mut param as *mut _ as *mut c_void);
        if pid <= 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
        leaf[i as usize].pid = pid;
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&leaf) {
        let mut retcode: c_int = 0;

        waitpid(leaf[i as usize].pid, &mut retcode, 0);
        if !WIFEXITED(retcode) {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
        if WEXITSTATUS(retcode) != 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
        i += 1;
    }

    i = 0;
    while i < ARRAY_SIZE(&leaf) {
        leaf[i as usize].usage = cg_read_key_long(leaf[i as usize].cgroup, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
        if leaf[i as usize].usage <= 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
        i += 1;
    }

    nested_leaf_usage = leaf[1].usage + leaf[2].usage;
    if overprovisioned {
        if values_close_report(leaf[0].usage, nested_leaf_usage, 15) == 0 {
            cleanup_nested_leaf(&mut leaf, child, parent);
            return ret;
        }
    } else if values_close_report(leaf[0].usage * 2, nested_leaf_usage, 15) == 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }

    child_usage = cg_read_key_long(child, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
    if child_usage <= 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }
    if values_close_report(child_usage, nested_leaf_usage, 1) == 0 {
        cleanup_nested_leaf(&mut leaf, child, parent);
        return ret;
    }

    ret = KSFT_PASS;
    cleanup_nested_leaf(&mut leaf, child, parent);
    ret
}

unsafe extern "C" fn test_cpucg_nested_weight_overprovisioned(root: *const c_char) -> c_int {
    run_cpucg_nested_weight_test(root, true)
}

unsafe extern "C" fn test_cpucg_nested_weight_underprovisioned(root: *const c_char) -> c_int {
    run_cpucg_nested_weight_test(root, false)
}

/*
 * Best effort attempt to get the kernel's HZ value from the config.
 * Return the HZ value if found otherwise return 1000 (the default) to
 * indicate failure.
 */
unsafe fn get_config_hz() -> c_long {
    let mut hz: c_long = 1000;
    let cmd = c"zcat /proc/config.gz 2>/dev/null | grep '^CONFIG_HZ='";
    let f: *mut libc::FILE;

    f = popen(cmd.as_ptr(), c"r".as_ptr());

    if f.is_null() {
        return hz;
    }

    if fscanf(f, c"CONFIG_HZ=%ld".as_ptr(), &mut hz) == EOF {
    }

    pclose(f);
    hz
}

unsafe extern "C" fn test_cpucg_max(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let hz = get_config_hz();
    let quota_usec: c_long = 1000;
    let default_period_usec: c_long = 100000; /* cpu.max's default period */
    let duration_seconds: c_long = 1;

    let duration_usec: c_long;
    let usage_usec: c_long;
    let n_periods: c_long;
    let remainder_usec: c_long;
    let expected_usage_usec: c_long;
    let cpucg: *mut c_char;
    let mut quota_buf = [0 as c_char; 32];

    duration_usec = duration_seconds * USEC_PER_SEC * 1000 / hz;

    snprintf(quota_buf.as_mut_ptr(), quota_buf.len(), c"%ld".as_ptr(), quota_usec);

    cpucg = cg_name(root, c"cpucg_test".as_ptr());
    if cpucg.is_null() {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    if cg_create(cpucg) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    if cg_write(cpucg, c"cpu.max".as_ptr(), quota_buf.as_ptr()) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    let mut param = cpu_hog_func_param {
        nprocs: 1,
        ts: libc::timespec {
            tv_sec: duration_usec / USEC_PER_SEC,
            tv_nsec: duration_usec % USEC_PER_SEC * NSEC_PER_USEC,
        },
        clock_type: hog_clock_type::CPU_HOG_CLOCK_WALL,
    };
    if cg_run(cpucg, hog_cpus_timed, &mut param as *mut _ as *mut c_void) != 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    usage_usec = cg_read_key_long(cpucg, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
    if usage_usec <= 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    /*
     * The following calculation applies only since
     * the cpu hog is set to run as per wall-clock time
     */
    n_periods = duration_usec / default_period_usec;
    remainder_usec = duration_usec - n_periods * default_period_usec;
    expected_usage_usec = n_periods * quota_usec + MIN(remainder_usec, quota_usec);

    if values_close_report(usage_usec, expected_usage_usec, 10) == 0 {
        cg_destroy(cpucg);
        free(cpucg as *mut c_void);
        return ret;
    }

    ret = KSFT_PASS;

    cg_destroy(cpucg);
    free(cpucg as *mut c_void);
    ret
}

unsafe extern "C" fn test_cpucg_max_nested(root: *const c_char) -> c_int {
    let mut ret = KSFT_FAIL;
    let hz = get_config_hz();
    let quota_usec: c_long = 1000;
    let default_period_usec: c_long = 100000; /* cpu.max's default period */
    let duration_seconds: c_long = 1;

    let duration_usec: c_long;
    let usage_usec: c_long;
    let n_periods: c_long;
    let remainder_usec: c_long;
    let expected_usage_usec: c_long;
    let parent: *mut c_char;
    let child: *mut c_char;
    let mut quota_buf = [0 as c_char; 32];

    duration_usec = duration_seconds * USEC_PER_SEC * 1000 / hz;

    snprintf(quota_buf.as_mut_ptr(), quota_buf.len(), c"%ld".as_ptr(), quota_usec);

    parent = cg_name(root, c"cpucg_parent".as_ptr());
    child = cg_name(parent, c"cpucg_child".as_ptr());
    if parent.is_null() || child.is_null() {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    if cg_create(parent) != 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    if cg_create(child) != 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    if cg_write(parent, c"cpu.max".as_ptr(), quota_buf.as_ptr()) != 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    let mut param = cpu_hog_func_param {
        nprocs: 1,
        ts: libc::timespec {
            tv_sec: duration_usec / USEC_PER_SEC,
            tv_nsec: duration_usec % USEC_PER_SEC * NSEC_PER_USEC,
        },
        clock_type: hog_clock_type::CPU_HOG_CLOCK_WALL,
    };
    if cg_run(child, hog_cpus_timed, &mut param as *mut _ as *mut c_void) != 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    usage_usec = cg_read_key_long(child, c"cpu.stat".as_ptr(), c"usage_usec".as_ptr());
    if usage_usec <= 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    /*
     * The following calculation applies only since
     * the cpu hog is set to run as per wall-clock time
     */
    n_periods = duration_usec / default_period_usec;
    remainder_usec = duration_usec - n_periods * default_period_usec;
    expected_usage_usec = n_periods * quota_usec + MIN(remainder_usec, quota_usec);

    if values_close_report(usage_usec, expected_usage_usec, 10) == 0 {
        cg_destroy(child);
        free(child as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
        return ret;
    }

    ret = KSFT_PASS;

    cg_destroy(child);
    free(child as *mut c_void);
    cg_destroy(parent);
    free(parent as *mut c_void);
    ret
}

static tests: [cpucg_test; 9] = [
    cpucg_test { fn_: test_cpucg_subtree_control, name: c"test_cpucg_subtree_control".as_ptr() },
    cpucg_test { fn_: test_cpucg_stats, name: c"test_cpucg_stats".as_ptr() },
    cpucg_test { fn_: test_cpucg_nice, name: c"test_cpucg_nice".as_ptr() },
    cpucg_test { fn_: test_cpucg_weight_overprovisioned, name: c"test_cpucg_weight_overprovisioned".as_ptr() },
    cpucg_test { fn_: test_cpucg_weight_underprovisioned, name: c"test_cpucg_weight_underprovisioned".as_ptr() },
    cpucg_test { fn_: test_cpucg_nested_weight_overprovisioned, name: c"test_cpucg_nested_weight_overprovisioned".as_ptr() },
    cpucg_test { fn_: test_cpucg_nested_weight_underprovisioned, name: c"test_cpucg_nested_weight_underprovisioned".as_ptr() },
    cpucg_test { fn_: test_cpucg_max, name: c"test_cpucg_max".as_ptr() },
    cpucg_test { fn_: test_cpucg_max_nested, name: c"test_cpucg_max_nested".as_ptr() },
];

unsafe fn real_main(_argc: c_int, _argv: *mut *mut c_char) {
    let mut root = [0 as c_char; PATH_MAX];
    let mut i: c_int;

    ksft_print_header();
    ksft_set_plan(ARRAY_SIZE(&tests));
    if cg_find_unified_root(root.as_mut_ptr(), root.len(), core::ptr::null_mut()) != 0 {
        ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
    }

    if cg_read_strstr(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"cpu".as_ptr()) != 0 {
        if cg_write(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) != 0 {
            ksft_exit_skip(c"Failed to set cpu controller\n".as_ptr());
        }
    }

    i = 0;
    while i < ARRAY_SIZE(&tests) {
        match (tests[i as usize].fn_)(root.as_ptr()) {
            KSFT_PASS => ksft_test_result_pass(c"%s\n".as_ptr(), tests[i as usize].name),
            KSFT_SKIP => ksft_test_result_skip(c"%s\n".as_ptr(), tests[i as usize].name),
            _ => ksft_test_result_fail(c"%s\n".as_ptr(), tests[i as usize].name),
        }
        i += 1;
    }

    ksft_finished();
}

fn main() {
    unsafe {
        real_main(0, core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
