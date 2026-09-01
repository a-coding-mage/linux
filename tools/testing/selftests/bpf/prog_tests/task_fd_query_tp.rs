// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/task_fd_query_tp.c
// Dependencies originally provided by <test_progs.h> and system headers.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: __u32,
    pub size: __u32,
    pub config: __u64,
    pub sample_period: __u64,
    pub sample_type: __u64,
    pub read_format: __u64,
    pub flags: __u64,
    pub wakeup_events: __u32,
    pub bp_type: __u32,
    pub bp_addr: __u64,
    pub bp_len: __u64,
}

pub type __u32 = u32;
pub type __u64 = u64;

unsafe extern "C" {
    static mut errno: c_int;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn syscall(number: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getpid() -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_task_fd_query(
        pid: c_int,
        fd: c_int,
        flags: __u32,
        buf: *mut c_char,
        buf_len: *mut __u32,
        prog_id: *mut __u32,
        fd_type: *mut __u32,
        probe_offset: *mut __u64,
        probe_addr: *mut __u64,
    ) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...) -> bool;
}

const BPF_PROG_TYPE_TRACEPOINT: c_int = 2;
const BPF_FD_TYPE_TRACEPOINT: __u32 = 2;
const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const PERF_TYPE_TRACEPOINT: __u32 = 2;
const PERF_SAMPLE_RAW: __u64 = 1 << 10;
const PERF_EVENT_IOC_ENABLE: c_ulong = 0x2400;
const PERF_EVENT_IOC_SET_BPF: c_ulong = 0x40042408;
const __NR_perf_event_open: c_long = 298;

unsafe fn test_task_fd_query_tp_core(probe_name: *const c_char, tp_name: *const c_char) {
    let file: *const c_char = b"./test_tracepoint.bpf.o\0".as_ptr() as *const c_char;
    let mut err: c_int;
    let mut bytes: c_int;
    let mut efd: c_int;
    let mut prog_fd: c_int = 0;
    let mut pmu_fd: c_int = 0;
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut probe_offset: __u64 = 0;
    let mut probe_addr: __u64 = 0;
    let mut len: __u32;
    let mut prog_id: __u32 = 0;
    let mut fd_type: __u32 = 0;
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut duration: __u32 = 0;
    let mut buf: [c_char; 256] = [0; 256];

    err = bpf_prog_test_load(
        file,
        BPF_PROG_TYPE_TRACEPOINT,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK(
        err != 0,
        b"bpf_prog_test_load\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_prog(obj);
        return;
    }

    if access(b"/sys/kernel/tracing/trace\0".as_ptr() as *const c_char, F_OK) == 0 {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 256]>(),
            b"/sys/kernel/tracing/events/%s/id\0".as_ptr() as *const c_char,
            probe_name,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            size_of::<[c_char; 256]>(),
            b"/sys/kernel/debug/tracing/events/%s/id\0".as_ptr() as *const c_char,
            probe_name,
        );
    }
    efd = open(buf.as_ptr(), O_RDONLY, 0);
    if CHECK(
        efd < 0,
        b"open\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        efd,
        errno,
    ) {
        goto_close_prog(obj);
        return;
    }
    bytes = read(efd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 256]>()) as c_int;
    close(efd);
    if CHECK(
        bytes <= 0 || bytes >= size_of::<[c_char; 256]>() as c_int,
        b"read\0".as_ptr() as *const c_char,
        b"bytes %d errno %d\n\0".as_ptr() as *const c_char,
        bytes,
        errno,
    ) {
        goto_close_prog(obj);
        return;
    }

    attr.config = strtol(buf.as_ptr(), ptr::null_mut(), 0) as __u64;
    attr.type_ = PERF_TYPE_TRACEPOINT;
    attr.sample_type = PERF_SAMPLE_RAW;
    attr.sample_period = 1;
    attr.wakeup_events = 1;
    pmu_fd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1, /* pid */
        0,  /* cpu 0 */
        -1, /* group id */
        0,  /* flags */
    ) as c_int;
    if CHECK(
        err != 0,
        b"perf_event_open\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_pmu(pmu_fd, obj);
        return;
    }

    err = ioctl(pmu_fd, PERF_EVENT_IOC_ENABLE, 0);
    if CHECK(
        err != 0,
        b"perf_event_ioc_enable\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_pmu(pmu_fd, obj);
        return;
    }

    err = ioctl(pmu_fd, PERF_EVENT_IOC_SET_BPF, prog_fd);
    if CHECK(
        err != 0,
        b"perf_event_ioc_set_bpf\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_pmu(pmu_fd, obj);
        return;
    }

    /* query (getpid(), pmu_fd) */
    len = size_of::<[c_char; 256]>() as __u32;
    err = bpf_task_fd_query(
        getpid(),
        pmu_fd,
        0,
        buf.as_mut_ptr(),
        &mut len,
        &mut prog_id,
        &mut fd_type,
        &mut probe_offset,
        &mut probe_addr,
    );
    if CHECK(
        err < 0,
        b"bpf_task_fd_query\0".as_ptr() as *const c_char,
        b"err %d errno %d\n\0".as_ptr() as *const c_char,
        err,
        errno,
    ) {
        goto_close_pmu(pmu_fd, obj);
        return;
    }

    err = ((fd_type == BPF_FD_TYPE_TRACEPOINT) && strcmp(buf.as_ptr(), tp_name) == 0) as c_int;
    if CHECK(
        err == 0,
        b"check_results\0".as_ptr() as *const c_char,
        b"fd_type %d tp_name %s\n\0".as_ptr() as *const c_char,
        fd_type,
        buf.as_ptr(),
    ) {
        goto_close_pmu(pmu_fd, obj);
        return;
    }

    goto_close_pmu(pmu_fd, obj);
}

unsafe fn goto_close_pmu(pmu_fd: c_int, obj: *mut bpf_object) {
    close(pmu_fd);
    goto_close_prog(obj);
}

unsafe fn goto_close_prog(obj: *mut bpf_object) {
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_task_fd_query_tp() {
    test_task_fd_query_tp_core(
        b"sched/sched_switch\0".as_ptr() as *const c_char,
        b"sched_switch\0".as_ptr() as *const c_char,
    );
    test_task_fd_query_tp_core(
        b"syscalls/sys_enter_read\0".as_ptr() as *const c_char,
        b"sys_enter_read\0".as_ptr() as *const c_char,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
