// SPDX-License-Identifier: GPL-2.0-only
/*
 * Helper functions for handling target threads/cpus
 *
 * Copyright (C) 2012, LG Electronics, Namhyung Kim <namhyung.kim@lge.com>
 */

// Translated from perf/util/target.c. C include dependencies are expected to be
// provided by the surrounding crate/bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
use core::ptr;

pub type size_t = c_ulong;
pub type uid_t = c_uint;

pub const UINT_MAX: uid_t = c_uint::MAX;

pub const TARGET_ERRNO__SUCCESS: target_errno = 0;
pub const __TARGET_ERRNO__START: c_int = -100_000;
pub const TARGET_ERRNO__PID_OVERRIDE_CPU: target_errno = __TARGET_ERRNO__START;
pub const TARGET_ERRNO__PID_OVERRIDE_SYSTEM: target_errno = __TARGET_ERRNO__START + 1;
pub const TARGET_ERRNO__SYSTEM_OVERRIDE_THREAD: target_errno = __TARGET_ERRNO__START + 2;
pub const TARGET_ERRNO__BPF_OVERRIDE_CPU: target_errno = __TARGET_ERRNO__START + 3;
pub const TARGET_ERRNO__BPF_OVERRIDE_PID: target_errno = __TARGET_ERRNO__START + 4;
pub const TARGET_ERRNO__BPF_OVERRIDE_THREAD: target_errno = __TARGET_ERRNO__START + 5;
pub const __TARGET_ERRNO__END: c_int = __TARGET_ERRNO__START + 6;

pub type target_errno = c_int;

#[repr(C)]
pub struct target {
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub cpu_list: *const c_char,
    pub system_wide: bool,
    pub bpf_str: *const c_char,
    pub per_thread: bool,
}

#[repr(C)]
pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: uid_t,
    pub pw_gid: c_uint,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

unsafe extern "C" {
    fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
}

macro_rules! BUG_ON {
    ($cond:expr) => {
        if $cond {
            panic!("BUG_ON({})", stringify!($cond));
        }
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn target__validate(target: *mut target) -> target_errno {
    let mut ret: target_errno = TARGET_ERRNO__SUCCESS;

    if !(*target).pid.is_null() {
        (*target).tid = (*target).pid;
    }

    /* CPU and PID are mutually exclusive */
    if !(*target).tid.is_null() && !(*target).cpu_list.is_null() {
        (*target).cpu_list = ptr::null();
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__PID_OVERRIDE_CPU;
        }
    }

    /* PID and SYSTEM are mutually exclusive */
    if !(*target).tid.is_null() && (*target).system_wide {
        (*target).system_wide = false;
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__PID_OVERRIDE_SYSTEM;
        }
    }

    /* BPF and CPU are mutually exclusive */
    if !(*target).bpf_str.is_null() && !(*target).cpu_list.is_null() {
        (*target).cpu_list = ptr::null();
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__BPF_OVERRIDE_CPU;
        }
    }

    /* BPF and PID/TID are mutually exclusive */
    if !(*target).bpf_str.is_null() && !(*target).tid.is_null() {
        (*target).tid = ptr::null();
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__BPF_OVERRIDE_PID;
        }
    }

    /* BPF and THREADS are mutually exclusive */
    if !(*target).bpf_str.is_null() && (*target).per_thread {
        (*target).per_thread = false;
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__BPF_OVERRIDE_THREAD;
        }
    }

    /* THREAD and SYSTEM/CPU are mutually exclusive */
    if (*target).per_thread && ((*target).system_wide || !(*target).cpu_list.is_null()) {
        (*target).per_thread = false;
        if ret == TARGET_ERRNO__SUCCESS {
            ret = TARGET_ERRNO__SYSTEM_OVERRIDE_THREAD;
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_uid(str_: *const c_char) -> uid_t {
    let mut pwd: passwd = core::mem::zeroed();
    let mut result: *mut passwd = ptr::null_mut();
    let mut buf = [0 as c_char; 1024];

    if str_.is_null() {
        return UINT_MAX;
    }

    /* Try user name first */
    getpwnam_r(
        str_,
        &mut pwd,
        buf.as_mut_ptr(),
        buf.len() as size_t,
        &mut result,
    );

    if result.is_null() {
        /*
         * The user name not found. Maybe it's a UID number.
         */
        let mut endptr: *mut c_char = ptr::null_mut();
        let uid: c_int = strtol(str_, &mut endptr, 10) as c_int;

        if *endptr != b'\0' as c_char {
            return UINT_MAX;
        }

        getpwuid_r(
            uid as uid_t,
            &mut pwd,
            buf.as_mut_ptr(),
            buf.len() as size_t,
            &mut result,
        );

        if result.is_null() {
            return UINT_MAX;
        }
    }

    (*result).pw_uid
}

/*
 * This must have a same ordering as the enum target_errno.
 */
static target__error_str: [*const c_char; 6] = [
    b"PID/TID switch overriding CPU\0".as_ptr() as *const c_char,
    b"PID/TID switch overriding SYSTEM\0".as_ptr() as *const c_char,
    b"SYSTEM/CPU switch overriding PER-THREAD\0".as_ptr() as *const c_char,
    b"BPF switch overriding CPU\0".as_ptr() as *const c_char,
    b"BPF switch overriding PID/TID\0".as_ptr() as *const c_char,
    b"BPF switch overriding THREAD\0".as_ptr() as *const c_char,
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn target__strerror(
    _target: *mut target,
    errnum: c_int,
    buf: *mut c_char,
    buflen: size_t,
) -> c_int {
    let idx: c_int;
    let msg: *const c_char;

    BUG_ON!(buflen == 0);

    if errnum >= 0 {
        str_error_r(errnum, buf, buflen);
        return 0;
    }

    if errnum < __TARGET_ERRNO__START || errnum >= __TARGET_ERRNO__END {
        return -1;
    }

    idx = errnum - __TARGET_ERRNO__START;
    msg = target__error_str[idx as usize];

    match errnum {
        TARGET_ERRNO__PID_OVERRIDE_CPU..=TARGET_ERRNO__BPF_OVERRIDE_THREAD => {
            snprintf(buf, buflen, b"%s\0".as_ptr() as *const c_char, msg);
        }

        _ => {
            /* cannot reach here */
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
