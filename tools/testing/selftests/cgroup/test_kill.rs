/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;

const ESRCH: c_int = 3;
const PATH_MAX: usize = 4096;
const KSFT_FAIL: c_int = 1;
const KSFT_PASS: c_int = 0;
const KSFT_SKIP: c_int = 4;

unsafe extern "C" {
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn fork() -> pid_t;
    fn getppid() -> pid_t;
    fn usleep(usec: c_int) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished();

    fn wait_for_pid(pid: pid_t) -> c_int;

    fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_wait_for(fd: c_int) -> c_int;
    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
    fn cg_read_strcmp(cgroup: *const c_char, file: *const c_char, expected: *const c_char) -> c_int;
    fn cg_read_strcmp_wait(
        cgroup: *const c_char,
        file: *const c_char,
        expected: *const c_char,
    ) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_find_unified_root(root: *mut c_char, len: usize, mount: *mut c_void) -> c_int;
}

/*
 * Kill the given cgroup and wait for the inotify signal.
 * If there are no events in 10 seconds, treat this as an error.
 * Then check that the cgroup is in the desired state.
 */
unsafe fn cg_kill_wait(cgroup: *const c_char) -> c_int {
    let fd: c_int;
    let mut ret: c_int = -1;

    fd = unsafe { cg_prepare_for_wait(cgroup) };
    if fd < 0 {
        return fd;
    }

    ret = unsafe { cg_write(cgroup, c"cgroup.kill".as_ptr(), c"1".as_ptr()) };
    if ret != 0 {
        unsafe { close(fd) };
        return ret;
    }

    ret = unsafe { cg_wait_for(fd) };
    if ret != 0 {
        unsafe { close(fd) };
        return ret;
    }

    unsafe { close(fd) };
    ret
}

/*
 * A simple process running in a sleep loop until being
 * re-parented.
 */
unsafe extern "C" fn child_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    let ppid: c_int = unsafe { getppid() };

    while unsafe { getppid() } == ppid {
        unsafe { usleep(1000) };
    }

    (unsafe { getppid() } == ppid) as c_int
}

unsafe fn test_cgkill_simple(root: *const c_char) -> c_int {
    let mut pids: [pid_t; 100] = [0; 100];
    let mut ret: c_int = KSFT_FAIL;
    let mut cgroup: *mut c_char = core::ptr::null_mut();
    let mut i: c_int;

    cgroup = unsafe { cg_name(root, c"cg_test_simple".as_ptr()) };
    if cgroup.is_null() {
        i = 0;
        while i < 100 {
            unsafe { wait_for_pid(pids[i as usize]) };
            i += 1;
        }
        unsafe { free(cgroup as *mut c_void) };
        return ret;
    }

    if unsafe { cg_create(cgroup) } != 0 {
        i = 0;
        while i < 100 {
            unsafe { wait_for_pid(pids[i as usize]) };
            i += 1;
        }
        if !cgroup.is_null() {
            unsafe { cg_destroy(cgroup) };
        }
        unsafe { free(cgroup as *mut c_void) };
        return ret;
    }

    i = 0;
    while i < 100 {
        pids[i as usize] =
            unsafe { cg_run_nowait(cgroup, child_fn, core::ptr::null_mut()) };
        i += 1;
    }

    if unsafe { cg_wait_for_proc_count(cgroup, 100) } == 0
        && unsafe { cg_read_strcmp(cgroup, c"cgroup.events".as_ptr(), c"populated 1\n".as_ptr()) }
            == 0
        && unsafe { cg_kill_wait(cgroup) } == 0
    {
        ret = KSFT_PASS;
    }

    i = 0;
    while i < 100 {
        unsafe { wait_for_pid(pids[i as usize]) };
        i += 1;
    }

    if ret == KSFT_PASS
        && unsafe {
            cg_read_strcmp_wait(cgroup, c"cgroup.events".as_ptr(), c"populated 0\n".as_ptr())
        } != 0
    {
        ret = KSFT_FAIL;
    }

    if !cgroup.is_null() {
        unsafe { cg_destroy(cgroup) };
    }
    unsafe { free(cgroup as *mut c_void) };
    ret
}

/*
 * The test creates the following hierarchy:
 *       A
 *    / / \ \
 *   B  E  I K
 *  /\  |
 * C  D F
 *      |
 *      G
 *      |
 *      H
 *
 * with a process in C, H and 3 processes in K.
 * Then it tries to kill the whole tree.
 */
unsafe fn test_cgkill_tree(root: *const c_char) -> c_int {
    let mut pids: [pid_t; 5] = [0; 5];
    let mut cgroup: [*mut c_char; 10] = [core::ptr::null_mut(); 10];
    let mut ret: c_int = KSFT_FAIL;
    let mut i: c_int;

    cgroup[0] = unsafe { cg_name(root, c"cg_test_tree_A".as_ptr()) };
    if !cgroup[0].is_null() {
        cgroup[1] = unsafe { cg_name(cgroup[0], c"B".as_ptr()) };
    }
    if !cgroup[1].is_null() {
        cgroup[2] = unsafe { cg_name(cgroup[1], c"C".as_ptr()) };
    }
    if !cgroup[2].is_null() {
        cgroup[3] = unsafe { cg_name(cgroup[1], c"D".as_ptr()) };
    }
    if !cgroup[3].is_null() {
        cgroup[4] = unsafe { cg_name(cgroup[0], c"E".as_ptr()) };
    }
    if !cgroup[4].is_null() {
        cgroup[5] = unsafe { cg_name(cgroup[4], c"F".as_ptr()) };
    }
    if !cgroup[5].is_null() {
        cgroup[6] = unsafe { cg_name(cgroup[5], c"G".as_ptr()) };
    }
    if !cgroup[6].is_null() {
        cgroup[7] = unsafe { cg_name(cgroup[6], c"H".as_ptr()) };
    }
    if !cgroup[7].is_null() {
        cgroup[8] = unsafe { cg_name(cgroup[0], c"I".as_ptr()) };
    }
    if !cgroup[8].is_null() {
        cgroup[9] = unsafe { cg_name(cgroup[0], c"K".as_ptr()) };
    }

    if !cgroup[9].is_null() {
        i = 0;
        while i < 10 {
            if unsafe { cg_create(cgroup[i as usize]) } != 0 {
                break;
            }
            i += 1;
        }

        if i == 10 {
            pids[0] = unsafe { cg_run_nowait(cgroup[2], child_fn, core::ptr::null_mut()) };
            pids[1] = unsafe { cg_run_nowait(cgroup[7], child_fn, core::ptr::null_mut()) };
            pids[2] = unsafe { cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut()) };
            pids[3] = unsafe { cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut()) };
            pids[4] = unsafe { cg_run_nowait(cgroup[9], child_fn, core::ptr::null_mut()) };

            /*
             * Wait until all child processes will enter
             * corresponding cgroups.
             */

            if !(unsafe { cg_wait_for_proc_count(cgroup[2], 1) } != 0
                || unsafe { cg_wait_for_proc_count(cgroup[7], 1) } != 0
                || unsafe { cg_wait_for_proc_count(cgroup[9], 3) } != 0)
            {
                /*
                 * Kill A and check that we get an empty notification.
                 */
                if unsafe { cg_kill_wait(cgroup[0]) } == 0 {
                    ret = KSFT_PASS;
                }
            }
        }
    }

    i = 0;
    while i < 5 {
        unsafe { wait_for_pid(pids[i as usize]) };
        i += 1;
    }

    if ret == KSFT_PASS
        && unsafe {
            cg_read_strcmp_wait(cgroup[0], c"cgroup.events".as_ptr(), c"populated 0\n".as_ptr())
        } != 0
    {
        ret = KSFT_FAIL;
    }

    i = 9;
    while i >= 0 && !cgroup[i as usize].is_null() {
        unsafe { cg_destroy(cgroup[i as usize]) };
        unsafe { free(cgroup[i as usize] as *mut c_void) };
        i -= 1;
    }

    ret
}

unsafe extern "C" fn forkbomb_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    let ppid: c_int;

    unsafe { fork() };
    unsafe { fork() };

    ppid = unsafe { getppid() };

    while unsafe { getppid() } == ppid {
        unsafe { usleep(1000) };
    }

    (unsafe { getppid() } == ppid) as c_int
}

/*
 * The test runs a fork bomb in a cgroup and tries to kill it.
 */
unsafe fn test_cgkill_forkbomb(root: *const c_char) -> c_int {
    let mut ret: c_int = KSFT_FAIL;
    let mut cgroup: *mut c_char = core::ptr::null_mut();
    let mut pid: pid_t = -ESRCH;

    cgroup = unsafe { cg_name(root, c"cg_forkbomb_test".as_ptr()) };
    if !cgroup.is_null() {
        if unsafe { cg_create(cgroup) } == 0 {
            pid = unsafe { cg_run_nowait(cgroup, forkbomb_fn, core::ptr::null_mut()) };
            if pid >= 0 {
                unsafe { usleep(100000) };

                if unsafe { cg_kill_wait(cgroup) } == 0
                    && unsafe { cg_wait_for_proc_count(cgroup, 0) } == 0
                {
                    ret = KSFT_PASS;
                }
            }
        }
    }

    if pid > 0 {
        unsafe { wait_for_pid(pid) };
    }

    if ret == KSFT_PASS
        && unsafe {
            cg_read_strcmp_wait(cgroup, c"cgroup.events".as_ptr(), c"populated 0\n".as_ptr())
        } != 0
    {
        ret = KSFT_FAIL;
    }

    if !cgroup.is_null() {
        unsafe { cg_destroy(cgroup) };
    }
    unsafe { free(cgroup as *mut c_void) };
    ret
}

#[repr(C)]
struct cgkill_test {
    fn_: unsafe fn(root: *const c_char) -> c_int,
    name: *const c_char,
}

const TESTS: [cgkill_test; 3] = [
    cgkill_test {
        fn_: test_cgkill_simple,
        name: c"test_cgkill_simple".as_ptr(),
    },
    cgkill_test {
        fn_: test_cgkill_tree,
        name: c"test_cgkill_tree".as_ptr(),
    },
    cgkill_test {
        fn_: test_cgkill_forkbomb,
        name: c"test_cgkill_forkbomb".as_ptr(),
    },
];

fn main() {
    let mut root: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut i: c_int;

    unsafe {
        ksft_print_header();
        ksft_set_plan(TESTS.len() as c_int);
        if cg_find_unified_root(root.as_mut_ptr(), core::mem::size_of_val(&root), core::ptr::null_mut())
            != 0
        {
            ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
        }
        i = 0;
        while i < TESTS.len() as c_int {
            match (TESTS[i as usize].fn_)(root.as_mut_ptr()) {
                KSFT_PASS => {
                    ksft_test_result_pass(c"%s\n".as_ptr(), TESTS[i as usize].name);
                }
                KSFT_SKIP => {
                    ksft_test_result_skip(c"%s\n".as_ptr(), TESTS[i as usize].name);
                }
                _ => {
                    ksft_test_result_fail(c"%s\n".as_ptr(), TESTS[i as usize].name);
                }
            }
            i += 1;
        }

        ksft_finished();
    }
}
