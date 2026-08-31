/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_double, c_int, c_long, c_ulonglong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type pid_t = c_int;
pub type useconds_t = u32;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn labs(j: c_long) -> c_long;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
}

/* #ifndef BUF_SIZE */
pub const BUF_SIZE: c_int = 4096;
/* #endif */

pub const fn MB(x: c_ulonglong) -> c_ulonglong {
    x << 20
}

pub const fn GB(x: c_ulonglong) -> c_ulonglong {
    (x as c_ulonglong) << 30
}

pub const NSEC_PER_USEC: c_long = 1000;
pub const USEC_PER_SEC: c_long = 1000000;
pub const NSEC_PER_SEC: c_long = 1000000000;

pub const TEST_UID: c_int = 65534; /* usually nobody, any !root is fine */

pub const CG_NAMED_NAME: &[u8; 9] = b"selftest\0";

pub unsafe fn CG_THREADS_FILE() -> *const c_char {
    if !cg_test_v1_named {
        c"cgroup.threads".as_ptr()
    } else {
        c"tasks".as_ptr()
    }
}

pub unsafe fn CG_PATH_FORMAT() -> *const c_char {
    if !cg_test_v1_named {
        c"0::%s".as_ptr()
    } else {
        c":name=selftest:%s".as_ptr()
    }
}

pub const DEFAULT_WAIT_INTERVAL_US: c_int = 100 * 1000; /* 100 ms */

/*
 * Checks if two given values differ by less than err% of their sum.
 */
pub unsafe fn values_close(a: c_long, b: c_long, err: c_int) -> c_int {
    (labs(a - b) <= (a + b) / 100 * err as c_long) as c_int
}

/*
 * Checks if two given values differ by less than err% of their sum and assert
 * with detailed debug info if not.
 */
pub unsafe fn values_close_report(a: c_long, b: c_long, err: c_int) -> c_int {
    let diff: c_long = labs(a - b);
    let limit: c_long = (a + b) / 100 * err as c_long;
    let actual_err: c_double = if (a + b) != 0 {
        100.0 * diff as c_double / (a + b) as c_double
    } else {
        0.0
    };
    let close: c_int = (diff <= limit) as c_int;

    if close == 0 {
        fprintf(
            stderr,
            c"[FAIL] actual=%ld expected=%ld | diff=%ld | limit=%ld | tolerance=%d%% | actual_error=%.2f%%\n".as_ptr(),
            a,
            b,
            diff,
            limit,
            err,
            actual_err,
        );
    }

    close
}

unsafe extern "C" {
    pub fn read_text(path: *const c_char, buf: *mut c_char, max_len: size_t) -> ssize_t;
    pub fn write_text(path: *const c_char, buf: *mut c_char, len: ssize_t) -> ssize_t;

    pub fn cg_find_controller_root(
        root: *mut c_char,
        len: size_t,
        controller: *const c_char,
    ) -> c_int;
    pub fn cg_find_unified_root(
        root: *mut c_char,
        len: size_t,
        nsdelegate: *mut bool,
    ) -> c_int;
    pub fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    pub fn cg_name_indexed(root: *const c_char, name: *const c_char, index: c_int) -> *mut c_char;
    pub fn cg_control(cgroup: *const c_char, control: *const c_char) -> *mut c_char;
    pub fn cg_create(cgroup: *const c_char) -> c_int;
    pub fn cg_destroy(cgroup: *const c_char) -> c_int;
    pub fn cg_read(
        cgroup: *const c_char,
        control: *const c_char,
        buf: *mut c_char,
        len: size_t,
    ) -> c_int;
    pub fn cg_read_strcmp(
        cgroup: *const c_char,
        control: *const c_char,
        expected: *const c_char,
    ) -> c_int;
    pub fn cg_read_strcmp_wait(
        cgroup: *const c_char,
        control: *const c_char,
        expected: *const c_char,
    ) -> c_int;
    pub fn cg_read_strstr(
        cgroup: *const c_char,
        control: *const c_char,
        needle: *const c_char,
    ) -> c_int;
    pub fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
    pub fn cg_read_long_fd(fd: c_int) -> c_long;
    pub fn cg_read_key_long(
        cgroup: *const c_char,
        control: *const c_char,
        key: *const c_char,
    ) -> c_long;
    pub fn cg_read_key_long_poll(
        cgroup: *const c_char,
        control: *const c_char,
        key: *const c_char,
        expected: c_long,
        retries: c_int,
        wait_interval_us: useconds_t,
    ) -> c_long;
    pub fn cg_read_lc(cgroup: *const c_char, control: *const c_char) -> c_long;
    pub fn cg_write(cgroup: *const c_char, control: *const c_char, buf: *mut c_char) -> c_int;
    pub fn cg_open(cgroup: *const c_char, control: *const c_char, flags: c_int) -> c_int;
    pub fn cg_write_numeric(cgroup: *const c_char, control: *const c_char, value: c_long) -> c_int;
    pub fn cg_run(
        cgroup: *const c_char,
        fn_: Option<unsafe extern "C" fn(cgroup: *const c_char, arg: *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn cg_enter(cgroup: *const c_char, pid: c_int) -> c_int;
    pub fn cg_enter_current(cgroup: *const c_char) -> c_int;
    pub fn cg_enter_current_thread(cgroup: *const c_char) -> c_int;
    pub fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: Option<unsafe extern "C" fn(cgroup: *const c_char, arg: *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
    pub fn cg_killall(cgroup: *const c_char) -> c_int;
    pub fn proc_mount_contains(option: *const c_char) -> c_int;
    pub fn cgroup_feature(feature: *const c_char) -> c_int;
    pub fn proc_read_text(
        pid: c_int,
        thread: bool,
        item: *const c_char,
        buf: *mut c_char,
        size: size_t,
    ) -> ssize_t;
    pub fn proc_read_strstr(
        pid: c_int,
        thread: bool,
        item: *const c_char,
        needle: *const c_char,
    ) -> c_int;
    pub fn clone_into_cgroup(cgroup_fd: c_int) -> pid_t;
    pub fn clone_reap(pid: pid_t, options: c_int) -> c_int;
    pub fn clone_into_cgroup_run_wait(cgroup: *const c_char) -> c_int;
    pub fn dirfd_open_opath(dir: *const c_char) -> c_int;
    pub fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int;
    pub fn memcg_prepare_for_wait(cgroup: *const c_char) -> c_int;
    pub fn cg_wait_for(fd: c_int) -> c_int;
    pub static mut cg_test_v1_named: bool;
}
