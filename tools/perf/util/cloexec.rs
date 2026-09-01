// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/cloexec.c. C include dependencies are expected to
// provide perf_event_attr, perf constants, WARN_ONCE, str_error_r, and syscall
// wrappers in the surrounding Rust translation.

use core::ffi::{c_char, c_int, c_ulong};

type pid_t = c_int;

extern "C" {
    static mut errno: c_int;

    fn sched_getcpu() -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn str_error_r(err: c_int, buf: *mut c_char, buflen: usize) -> *const c_char;
}

static mut flag: c_ulong = PERF_FLAG_FD_CLOEXEC as c_ulong;
static mut probed: bool = false;

unsafe fn perf_flag_probe() -> c_int {
    /* use 'safest' configuration as used in evsel__fallback() */
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        config: PERF_COUNT_SW_CPU_CLOCK,
        exclude_kernel: 1,
        ..Default::default()
    };
    let mut fd: c_int;
    let mut err: c_int;
    let mut cpu: c_int;
    let mut pid: pid_t = -1;
    let mut sbuf: [c_char; STRERR_BUFSIZE] = [0; STRERR_BUFSIZE];

    cpu = sched_getcpu();
    if cpu < 0 {
        cpu = 0;
    }

    /*
     * Using -1 for the pid is a workaround to avoid gratuitous jump label
     * changes.
     */
    loop {
        /* check cloexec flag */
        fd = sys_perf_event_open(
            &mut attr,
            pid,
            cpu,
            -1,
            PERF_FLAG_FD_CLOEXEC as c_ulong,
        );
        if fd < 0 && pid == -1 && errno == EACCES {
            pid = 0;
            continue;
        }
        break;
    }
    err = errno;

    if fd >= 0 {
        close(fd);
        return 1;
    }

    WARN_ONCE(
        err != EINVAL && err != EBUSY && err != EACCES,
        "perf_event_open(..., PERF_FLAG_FD_CLOEXEC) failed with unexpected error %d (%s)\n",
        err,
        str_error_r(err, sbuf.as_mut_ptr(), sbuf.len()),
    );

    /* not supported, confirm error related to PERF_FLAG_FD_CLOEXEC */
    loop {
        fd = sys_perf_event_open(&mut attr, pid, cpu, -1, 0);
        if fd < 0 && pid == -1 && errno == EACCES {
            pid = 0;
            continue;
        }
        break;
    }
    err = errno;

    if fd >= 0 {
        close(fd);
    }

    if WARN_ONCE(
        fd < 0 && err != EBUSY && err != EACCES,
        "perf_event_open(..., 0) failed unexpectedly with error %d (%s)\n",
        err,
        str_error_r(err, sbuf.as_mut_ptr(), sbuf.len()),
    ) {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_open_cloexec_flag() -> c_ulong {
    if !probed {
        if perf_flag_probe() <= 0 {
            flag = 0;
        }
        probed = true;
    }

    flag
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
