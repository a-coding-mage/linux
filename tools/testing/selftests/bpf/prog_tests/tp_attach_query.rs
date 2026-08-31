// SPDX-License-Identifier: GPL-2.0
// Translated from C. External test, libc, kernel, and libbpf symbols are
// expected to be supplied by the surrounding selftests build.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

unsafe extern "C" {
    static mut errno: c_int;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn bzero(s: *mut c_void, n: usize);
    fn syscall(number: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_get_info_by_fd(
        bpf_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

// Layout is provided by linux/perf_event.h in the original C build.
#[repr(C)]
#[derive(Default)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events: c_uint,
}

// Layout is provided by linux/bpf.h in the original C build.
#[repr(C)]
#[derive(Default)]
pub struct bpf_prog_info {
    pub type_: __u32,
    pub id: __u32,
    pub tag: [u8; 8],
    pub jited_prog_len: __u32,
    pub xlated_prog_len: __u32,
    pub jited_prog_insns: u64,
    pub xlated_prog_insns: u64,
    pub load_time: u64,
    pub created_by_uid: __u32,
    pub nr_map_ids: __u32,
}

#[repr(C)]
pub struct perf_event_query_bpf {
    pub ids_len: __u32,
    pub prog_cnt: __u32,
    pub ids: [__u32; 0],
}

unsafe fn query_id(query: *mut perf_event_query_bpf, index: c_int) -> __u32 {
    unsafe { *(*query).ids.as_ptr().add(index as usize) }
}

pub unsafe fn serial_test_tp_attach_query() {
    const NUM_PROGS: usize = 3;
    let mut i: c_int;
    let mut j: c_int;
    let mut bytes: c_int;
    let mut efd: c_int;
    let mut err: c_int;
    let mut prog_fd: [c_int; NUM_PROGS] = [0; NUM_PROGS];
    let mut pmu_fd: [c_int; NUM_PROGS] = [0; NUM_PROGS];
    let mut info_len: __u32;
    let mut saved_prog_ids: [__u32; NUM_PROGS] = [0; NUM_PROGS];
    let file = c"./test_tracepoint.bpf.o".as_ptr();
    let mut query: *mut perf_event_query_bpf;
    let mut attr: perf_event_attr = perf_event_attr::default();
    let mut obj: [*mut bpf_object; NUM_PROGS] = [ptr::null_mut(); NUM_PROGS];
    let mut prog_info: bpf_prog_info = bpf_prog_info::default();
    let mut buf: [c_char; 256] = [0; 256];

    i = 0;
    while i < NUM_PROGS as c_int {
        obj[i as usize] = ptr::null_mut();
        i += 1;
    }

    unsafe {
        if access(c"/sys/kernel/tracing/trace".as_ptr(), F_OK) == 0 {
            snprintf(
                buf.as_mut_ptr(),
                size_of_val(&buf),
                c"/sys/kernel/tracing/events/sched/sched_switch/id".as_ptr(),
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                size_of_val(&buf),
                c"/sys/kernel/debug/tracing/events/sched/sched_switch/id".as_ptr(),
            );
        }
        efd = open(buf.as_ptr(), O_RDONLY, 0);
        if CHECK(
            efd < 0,
            c"open".as_ptr(),
            c"err %d errno %d\n".as_ptr(),
            efd,
            errno,
        ) {
            return;
        }
        bytes = read(efd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_int;
        close(efd);
        if CHECK(
            bytes <= 0 || bytes >= size_of_val(&buf) as c_int,
            c"read".as_ptr(),
            c"bytes %d errno %d\n".as_ptr(),
            bytes,
            errno,
        ) {
            return;
        }

        attr.config = strtol(buf.as_ptr(), ptr::null_mut(), 0) as u64;
        attr.type_ = PERF_TYPE_TRACEPOINT;
        attr.sample_type = PERF_SAMPLE_RAW | PERF_SAMPLE_CALLCHAIN;
        attr.sample_period = 1;
        attr.wakeup_events = 1;

        query = malloc(size_of::<perf_event_query_bpf>() + size_of::<__u32>() * NUM_PROGS)
            as *mut perf_event_query_bpf;

        i = 0;
        while i < NUM_PROGS as c_int {
            let cleanup_level: c_int;

            err = bpf_prog_test_load(
                file,
                BPF_PROG_TYPE_TRACEPOINT,
                &mut obj[i as usize],
                &mut prog_fd[i as usize],
            );
            if CHECK(
                err != 0,
                c"prog_load".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            ) {
                cleanup_level = 1;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }

            bzero(
                &mut prog_info as *mut bpf_prog_info as *mut c_void,
                size_of::<bpf_prog_info>(),
            );
            prog_info.jited_prog_len = 0;
            prog_info.xlated_prog_len = 0;
            prog_info.nr_map_ids = 0;
            info_len = size_of::<bpf_prog_info>() as __u32;
            err = bpf_prog_get_info_by_fd(prog_fd[i as usize], &mut prog_info, &mut info_len);
            if CHECK(
                err != 0,
                c"bpf_prog_get_info_by_fd".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            ) {
                cleanup_level = 1;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }
            saved_prog_ids[i as usize] = prog_info.id;

            pmu_fd[i as usize] = syscall(
                __NR_perf_event_open,
                &mut attr as *mut perf_event_attr,
                -1 as c_int, /* pid */
                0 as c_int,  /* cpu 0 */
                -1 as c_int, /* group id */
                0 as c_int,  /* flags */
            ) as c_int;
            if CHECK(
                pmu_fd[i as usize] < 0,
                c"perf_event_open".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                pmu_fd[i as usize],
                errno,
            ) {
                cleanup_level = 2;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }
            err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_ENABLE, 0);
            if CHECK(
                err != 0,
                c"perf_event_ioc_enable".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            ) {
                cleanup_level = 3;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }

            if i == 0 {
                /* check NULL prog array query */
                (*query).ids_len = NUM_PROGS as __u32;
                err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_QUERY_BPF, query);
                if CHECK(
                    err != 0 || (*query).prog_cnt != 0,
                    c"perf_event_ioc_query_bpf".as_ptr(),
                    c"err %d errno %d query->prog_cnt %u\n".as_ptr(),
                    err,
                    errno,
                    (*query).prog_cnt,
                ) {
                    cleanup_level = 3;
                    cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                    free(query as *mut c_void);
                    return;
                }
            }

            err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_SET_BPF, prog_fd[i as usize]);
            if CHECK(
                err != 0,
                c"perf_event_ioc_set_bpf".as_ptr(),
                c"err %d errno %d\n".as_ptr(),
                err,
                errno,
            ) {
                cleanup_level = 3;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }

            if i == 1 {
                /* try to get # of programs only */
                (*query).ids_len = 0;
                err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_QUERY_BPF, query);
                if CHECK(
                    err != 0 || (*query).prog_cnt != 2,
                    c"perf_event_ioc_query_bpf".as_ptr(),
                    c"err %d errno %d query->prog_cnt %u\n".as_ptr(),
                    err,
                    errno,
                    (*query).prog_cnt,
                ) {
                    cleanup_level = 3;
                    cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                    free(query as *mut c_void);
                    return;
                }

                /* try a few negative tests */
                /* invalid query pointer */
                err = ioctl(
                    pmu_fd[i as usize],
                    PERF_EVENT_IOC_QUERY_BPF,
                    0x1usize as *mut perf_event_query_bpf,
                );
                if CHECK(
                    err == 0 || errno != EFAULT,
                    c"perf_event_ioc_query_bpf".as_ptr(),
                    c"err %d errno %d\n".as_ptr(),
                    err,
                    errno,
                ) {
                    cleanup_level = 3;
                    cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                    free(query as *mut c_void);
                    return;
                }

                /* no enough space */
                (*query).ids_len = 1;
                err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_QUERY_BPF, query);
                if CHECK(
                    err == 0 || errno != ENOSPC || (*query).prog_cnt != 2,
                    c"perf_event_ioc_query_bpf".as_ptr(),
                    c"err %d errno %d query->prog_cnt %u\n".as_ptr(),
                    err,
                    errno,
                    (*query).prog_cnt,
                ) {
                    cleanup_level = 3;
                    cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                    free(query as *mut c_void);
                    return;
                }
            }

            (*query).ids_len = NUM_PROGS as __u32;
            err = ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_QUERY_BPF, query);
            if CHECK(
                err != 0 || (*query).prog_cnt != (i + 1) as __u32,
                c"perf_event_ioc_query_bpf".as_ptr(),
                c"err %d errno %d query->prog_cnt %u\n".as_ptr(),
                err,
                errno,
                (*query).prog_cnt,
            ) {
                cleanup_level = 3;
                cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                free(query as *mut c_void);
                return;
            }
            j = 0;
            while j < i + 1 {
                if CHECK(
                    saved_prog_ids[j as usize] != query_id(query, j),
                    c"perf_event_ioc_query_bpf".as_ptr(),
                    c"#%d saved_prog_id %x query prog_id %x\n".as_ptr(),
                    j,
                    saved_prog_ids[j as usize],
                    query_id(query, j),
                ) {
                    cleanup_level = 3;
                    cleanup_from(i, cleanup_level, &mut pmu_fd, &mut obj);
                    free(query as *mut c_void);
                    return;
                }
                j += 1;
            }

            i += 1;
        }

        i = NUM_PROGS as c_int - 1;
        while i >= 0 {
            cleanup_from(i, 3, &mut pmu_fd, &mut obj);
            i -= 1;
        }
        free(query as *mut c_void);
    }
}

unsafe fn cleanup_from(
    mut i: c_int,
    cleanup_level: c_int,
    pmu_fd: &mut [c_int; 3],
    obj: &mut [*mut bpf_object; 3],
) {
    unsafe {
        loop {
            if cleanup_level >= 3 {
                ioctl(pmu_fd[i as usize], PERF_EVENT_IOC_DISABLE);
            }
            if cleanup_level >= 2 {
                close(pmu_fd[i as usize]);
            }
            if cleanup_level >= 1 {
                bpf_object__close(obj[i as usize]);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}

