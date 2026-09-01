// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included errno, linux/limits, signal,
// string, sys/stat, sys/types, unistd, kselftest.h, and cgroup_util.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_int, c_long, c_void};
use std::ptr;

const PATH_MAX: usize = 4096;
const NULL: *mut c_void = ptr::null_mut();

const EAGAIN: c_int = 11;
const SIGINT: c_int = 2;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

unsafe extern "C" {
    fn pause() -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;

    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_read_strcmp(cgroup: *const c_char, file: *const c_char, expected: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, file: *const c_char, buf: *const c_char) -> c_int;
    fn cg_enter_current(cgroup: *const c_char) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: Option<unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    fn cgroup_feature(feature: *const c_char) -> c_int;
    fn cg_read_key_long(cgroup: *const c_char, file: *const c_char, key: *const c_char) -> c_long;
    fn cg_find_unified_root(root: *mut c_char, len: usize, mount_type: *mut c_void) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, file: *const c_char, needle: *const c_char) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished();
}

unsafe extern "C" fn run_success(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    0
}

unsafe extern "C" fn run_pause(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    unsafe { pause() }
}

/*
 * This test checks that pids.max prevents forking new children above the
 * specified limit in the cgroup.
 */
unsafe fn test_pids_max(root: *const c_char) -> c_int {
    let mut ret: c_int = KSFT_FAIL;
    let cg_pids: *mut c_char;
    let pid: c_int;

    cg_pids = unsafe { cg_name(root, c"pids_test".as_ptr()) };
    if cg_pids.is_null() {
        return ret;
    }

    if unsafe { cg_create(cg_pids) } != 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_read_strcmp(cg_pids, c"pids.max".as_ptr(), c"max\n".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_write(cg_pids, c"pids.max".as_ptr(), c"2".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_enter_current(cg_pids) } != 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    pid = unsafe { cg_run_nowait(cg_pids, Some(run_pause), NULL) };
    if pid < 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_run_nowait(cg_pids, Some(run_success), NULL) } != -1
        || unsafe { *__errno_location() } != EAGAIN
    {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    if unsafe { kill(pid, SIGINT) } != 0 {
        unsafe {
            cg_enter_current(root);
            cg_destroy(cg_pids);
            free(cg_pids as *mut c_void);
        }
        return ret;
    }

    ret = KSFT_PASS;

    unsafe {
        cg_enter_current(root);
        cg_destroy(cg_pids);
        free(cg_pids as *mut c_void);
    }

    ret
}

/*
 * This test checks that pids.events are counted in cgroup associated with pids.max
 */
unsafe fn test_pids_events(root: *const c_char) -> c_int {
    let mut ret: c_int = KSFT_FAIL;
    let mut cg_parent: *mut c_char = ptr::null_mut();
    let mut cg_child: *mut c_char = ptr::null_mut();
    let pid: c_int;

    if unsafe { cgroup_feature(c"pids_localevents".as_ptr()) } <= 0 {
        return KSFT_SKIP;
    }

    cg_parent = unsafe { cg_name(root, c"pids_parent".as_ptr()) };
    cg_child = unsafe { cg_name(cg_parent, c"pids_child".as_ptr()) };
    if cg_parent.is_null() || cg_child.is_null() {
        unsafe {
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_create(cg_parent) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }
    if unsafe { cg_write(cg_parent, c"cgroup.subtree_control".as_ptr(), c"+pids".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }
    if unsafe { cg_create(cg_child) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_write(cg_parent, c"pids.max".as_ptr(), c"2".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_read_strcmp(cg_child, c"pids.max".as_ptr(), c"max\n".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_enter_current(cg_child) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    pid = unsafe { cg_run_nowait(cg_child, Some(run_pause), NULL) };
    if pid < 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_run_nowait(cg_child, Some(run_success), NULL) } != -1
        || unsafe { *__errno_location() } != EAGAIN
    {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { kill(pid, SIGINT) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    if unsafe { cg_read_key_long(cg_child, c"pids.events".as_ptr(), c"max ".as_ptr()) } != 0 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }
    if unsafe { cg_read_key_long(cg_parent, c"pids.events".as_ptr(), c"max ".as_ptr()) } != 1 {
        unsafe {
            cg_enter_current(root);
            if !cg_child.is_null() {
                cg_destroy(cg_child);
            }
            if !cg_parent.is_null() {
                cg_destroy(cg_parent);
            }
            free(cg_child as *mut c_void);
            free(cg_parent as *mut c_void);
        }
        return ret;
    }

    ret = KSFT_PASS;

    unsafe {
        cg_enter_current(root);
        if !cg_child.is_null() {
            cg_destroy(cg_child);
        }
        if !cg_parent.is_null() {
            cg_destroy(cg_parent);
        }
        free(cg_child as *mut c_void);
        free(cg_parent as *mut c_void);
    }

    ret
}

#[repr(C)]
struct pids_test {
    fn_: unsafe fn(root: *const c_char) -> c_int,
    name: *const c_char,
}

static mut tests: [pids_test; 2] = [
    pids_test {
        fn_: test_pids_max,
        name: c"test_pids_max".as_ptr(),
    },
    pids_test {
        fn_: test_pids_events,
        name: c"test_pids_events".as_ptr(),
    },
];

fn main() {
    let mut root: [c_char; PATH_MAX] = [0; PATH_MAX];

    unsafe {
        ksft_print_header();
        ksft_set_plan(tests.len() as c_int);
        if cg_find_unified_root(root.as_mut_ptr(), root.len(), NULL) != 0 {
            ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
        }

        /*
         * Check that pids controller is available:
         * pids is listed in cgroup.controllers
         */
        if cg_read_strstr(root.as_ptr(), c"cgroup.controllers".as_ptr(), c"pids".as_ptr()) != 0 {
            ksft_exit_skip(c"pids controller isn't available\n".as_ptr());
        }

        if cg_read_strstr(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"pids".as_ptr()) != 0 {
            if cg_write(root.as_ptr(), c"cgroup.subtree_control".as_ptr(), c"+pids".as_ptr()) != 0 {
                ksft_exit_skip(c"Failed to set pids controller\n".as_ptr());
            }
        }

        let mut i: usize = 0;
        while i < tests.len() {
            match (tests[i].fn_)(root.as_ptr()) {
                KSFT_PASS => {
                    ksft_test_result_pass(c"%s\n".as_ptr(), tests[i].name);
                }
                KSFT_SKIP => {
                    ksft_test_result_skip(c"%s\n".as_ptr(), tests[i].name);
                }
                _ => {
                    ksft_test_result_fail(c"%s\n".as_ptr(), tests[i].name);
                }
            }

            i += 1;
        }

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
