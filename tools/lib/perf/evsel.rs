// SPDX-License-Identifier: GPL-2.0
//
// Translated from lib/perf/evsel.c. C include dependencies are intentionally
// left as external Rust items expected from the surrounding translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type u64 = u64;

extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn __errno_location() -> *mut c_int;

    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;

    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;

    fn xyarray__new(xlen: c_int, ylen: c_int, entry_size: size_t) -> *mut xyarray;
    fn xyarray__delete(xy: *mut xyarray);
    fn xyarray__entry(xy: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn xyarray__max_x(xy: *const xyarray) -> c_int;
    fn xyarray__max_y(xy: *const xyarray) -> c_int;

    fn perf_mmap__init(
        map: *mut perf_mmap,
        prev: *mut c_void,
        overwrite: bool,
        ops: *mut c_void,
    );
    fn perf_mmap__mmap(
        map: *mut perf_mmap,
        mp: *mut perf_mmap_param,
        fd: c_int,
        cpu: perf_cpu,
    ) -> c_int;
    fn perf_mmap__munmap(map: *mut perf_mmap);
    fn perf_mmap__read_self(
        map: *mut perf_mmap,
        count: *mut perf_counts_values,
    ) -> bool;
}

type c_long = i64;

extern "C" {
    static page_size: c_int;
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBADF: c_int = 9;
const ENOTCONN: c_int = 107;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;

const __NR_perf_event_open: c_long = 298;

const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_LOST: u64 = 1 << 4;

const PERF_EVENT_IOC_ENABLE: c_int = 0x2400;
const PERF_EVENT_IOC_DISABLE: c_int = 0x2401;
const PERF_EVENT_IOC_SET_FILTER: c_int = 0x40082406;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub read_format: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread {
    pub pid: pid_t,
}

#[repr(C)]
pub struct perf_thread_map {
    pub nr: c_int,
    pub map: [perf_thread; 0],
}

#[repr(C)]
pub struct xyarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_mmap {
    pub base: *mut c_void,
}

#[repr(C)]
pub struct perf_mmap_param {
    pub prot: c_int,
    pub mask: c_ulong,
}

#[repr(C)]
pub union perf_counts_values_union {
    pub values: [u64; 5],
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
    pub id: u64,
    pub lost: u64,
}

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
    pub per_stream_periods: list_head,
    pub attr: perf_event_attr,
    pub idx: c_int,
    pub leader: *mut perf_evsel,
    pub fd: *mut xyarray,
    pub mmap: *mut xyarray,
    pub sample_id: *mut c_void,
    pub cpus: *mut perf_cpu_map,
    pub pmu_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub nr_members: c_int,
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

#[inline]
unsafe fn FD(evsel: *mut perf_evsel, cpu_map_idx: c_int, thread: c_int) -> *mut c_int {
    xyarray__entry((*evsel).fd, cpu_map_idx, thread) as *mut c_int
}

#[inline]
unsafe fn MMAP(evsel: *mut perf_evsel, cpu_map_idx: c_int, thread: c_int) -> *mut perf_mmap {
    if !(*evsel).mmap.is_null() {
        xyarray__entry((*evsel).mmap, cpu_map_idx, thread) as *mut perf_mmap
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__init(
    evsel: *mut perf_evsel,
    attr: *mut perf_event_attr,
    idx: c_int,
) {
    INIT_LIST_HEAD(&mut (*evsel).node);
    INIT_LIST_HEAD(&mut (*evsel).per_stream_periods);
    (*evsel).attr = ptr::read(attr);
    (*evsel).idx = idx;
    (*evsel).leader = evsel;
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__new(attr: *mut perf_event_attr) -> *mut perf_evsel {
    let evsel = zalloc(size_of::<perf_evsel>()) as *mut perf_evsel;

    if !evsel.is_null() {
        perf_evsel__init(evsel, attr, 0);
    }

    evsel
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__exit(evsel: *mut perf_evsel) {
    assert!((*evsel).fd.is_null()); /* If not fds were not closed. */
    assert!((*evsel).mmap.is_null()); /* If not munmap wasn't called. */
    assert!((*evsel).sample_id.is_null()); /* If not free_id wasn't called. */
    perf_cpu_map__put((*evsel).cpus);
    perf_cpu_map__put((*evsel).pmu_cpus);
    perf_thread_map__put((*evsel).threads);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__delete(evsel: *mut perf_evsel) {
    perf_evsel__exit(evsel);
    free(evsel as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__alloc_fd(
    evsel: *mut perf_evsel,
    ncpus: c_int,
    nthreads: c_int,
) -> c_int {
    (*evsel).fd = xyarray__new(ncpus, nthreads, size_of::<c_int>());

    if !(*evsel).fd.is_null() {
        let mut idx = 0;
        while idx < ncpus {
            let mut thread = 0;
            while thread < nthreads {
                let fd = FD(evsel, idx, thread);

                if !fd.is_null() {
                    *fd = -1;
                }
                thread += 1;
            }
            idx += 1;
        }
    }

    if !(*evsel).fd.is_null() { 0 } else { -ENOMEM }
}

unsafe fn perf_evsel__alloc_mmap(
    evsel: *mut perf_evsel,
    ncpus: c_int,
    nthreads: c_int,
) -> c_int {
    (*evsel).mmap = xyarray__new(ncpus, nthreads, size_of::<perf_mmap>());

    if !(*evsel).mmap.is_null() { 0 } else { -ENOMEM }
}

unsafe fn sys_perf_event_open(
    attr: *mut perf_event_attr,
    pid: pid_t,
    cpu: perf_cpu,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    syscall(__NR_perf_event_open, attr, pid, cpu.cpu, group_fd, flags) as c_int
}

unsafe fn get_group_fd(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
    thread: c_int,
    group_fd: *mut c_int,
) -> c_int {
    let leader = (*evsel).leader;
    let fd: *mut c_int;

    if evsel == leader {
        *group_fd = -1;
        return 0;
    }

    /*
     * Leader must be already processed/open,
     * if not it's a bug.
     */
    if (*leader).fd.is_null() {
        return -ENOTCONN;
    }

    fd = FD(leader, cpu_map_idx, thread);
    if fd.is_null() || *fd == -1 {
        return -EBADF;
    }

    *group_fd = *fd;

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__open(
    evsel: *mut perf_evsel,
    mut cpus: *mut perf_cpu_map,
    mut threads: *mut perf_thread_map,
) -> c_int {
    static mut empty_cpu_map: *mut perf_cpu_map = ptr::null_mut();
    static mut empty_thread_map: *mut perf_thread_map = ptr::null_mut();
    let mut err = 0;

    if cpus.is_null() {
        if empty_cpu_map.is_null() {
            empty_cpu_map = perf_cpu_map__new_any_cpu();
            if empty_cpu_map.is_null() {
                return -ENOMEM;
            }
        }

        cpus = empty_cpu_map;
    }

    if threads.is_null() {
        if empty_thread_map.is_null() {
            empty_thread_map = perf_thread_map__new_dummy();
            if empty_thread_map.is_null() {
                return -ENOMEM;
            }
        }

        threads = empty_thread_map;
    }

    if (*evsel).fd.is_null()
        && perf_evsel__alloc_fd(evsel, perf_cpu_map__nr(cpus), (*threads).nr) < 0
    {
        return -ENOMEM;
    }

    let mut idx: c_int = 0;
    while idx < perf_cpu_map__nr(cpus) {
        let cpu = perf_cpu_map__cpu(cpus, idx);
        let mut thread = 0;
        while thread < (*threads).nr {
            let mut group_fd: c_int = 0;

            let evsel_fd = FD(evsel, idx, thread);
            if evsel_fd.is_null() {
                err = -EINVAL;
                break;
            }

            err = get_group_fd(evsel, idx, thread, &mut group_fd);
            if err < 0 {
                break;
            }

            let fd = sys_perf_event_open(
                &mut (*evsel).attr,
                (*(*threads).map.as_ptr().offset(thread as isize)).pid,
                cpu,
                group_fd,
                0,
            );

            if fd < 0 {
                err = -errno();
                break;
            }

            *evsel_fd = fd;
            thread += 1;
        }
        if err < 0 {
            break;
        }
        idx += 1;
    }

    if err != 0 {
        perf_evsel__close(evsel);
    }

    err
}

unsafe fn perf_evsel__close_fd_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) {
    let mut thread = 0;

    while thread < xyarray__max_y((*evsel).fd) {
        let fd = FD(evsel, cpu_map_idx, thread);

        if !fd.is_null() && *fd >= 0 {
            close(*fd);
            *fd = -1;
        }
        thread += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__close_fd(evsel: *mut perf_evsel) {
    let mut idx = 0;
    while idx < xyarray__max_x((*evsel).fd) {
        perf_evsel__close_fd_cpu(evsel, idx);
        idx += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__free_fd(evsel: *mut perf_evsel) {
    xyarray__delete((*evsel).fd);
    (*evsel).fd = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__close(evsel: *mut perf_evsel) {
    if (*evsel).fd.is_null() {
        return;
    }

    perf_evsel__close_fd(evsel);
    perf_evsel__free_fd(evsel);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__close_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) {
    if (*evsel).fd.is_null() {
        return;
    }

    perf_evsel__close_fd_cpu(evsel, cpu_map_idx);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__munmap(evsel: *mut perf_evsel) {
    if (*evsel).fd.is_null() || (*evsel).mmap.is_null() {
        return;
    }

    let mut idx = 0;
    while idx < xyarray__max_x((*evsel).fd) {
        let mut thread = 0;
        while thread < xyarray__max_y((*evsel).fd) {
            let fd = FD(evsel, idx, thread);

            if fd.is_null() || *fd < 0 {
                thread += 1;
                continue;
            }

            perf_mmap__munmap(MMAP(evsel, idx, thread));
            thread += 1;
        }
        idx += 1;
    }

    xyarray__delete((*evsel).mmap);
    (*evsel).mmap = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__mmap(evsel: *mut perf_evsel, pages: c_int) -> c_int {
    let mut mp = perf_mmap_param {
        prot: PROT_READ | PROT_WRITE,
        mask: ((pages * page_size) - 1) as c_ulong,
    };

    if (*evsel).fd.is_null() || !(*evsel).mmap.is_null() {
        return -EINVAL;
    }

    if perf_evsel__alloc_mmap(evsel, xyarray__max_x((*evsel).fd), xyarray__max_y((*evsel).fd)) < 0 {
        return -ENOMEM;
    }

    let mut idx = 0;
    while idx < xyarray__max_x((*evsel).fd) {
        let mut thread = 0;
        while thread < xyarray__max_y((*evsel).fd) {
            let fd = FD(evsel, idx, thread);
            let cpu = perf_cpu_map__cpu((*evsel).cpus, idx);

            if fd.is_null() || *fd < 0 {
                thread += 1;
                continue;
            }

            let map = MMAP(evsel, idx, thread);
            perf_mmap__init(map, ptr::null_mut(), false, ptr::null_mut());

            let ret = perf_mmap__mmap(map, &mut mp, *fd, cpu);
            if ret != 0 {
                perf_evsel__munmap(evsel);
                return ret;
            }
            thread += 1;
        }
        idx += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__mmap_base(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> *mut c_void {
    let fd = FD(evsel, cpu_map_idx, thread);

    if fd.is_null() || *fd < 0 || MMAP(evsel, cpu_map_idx, thread).is_null() {
        return ptr::null_mut();
    }

    (*MMAP(evsel, cpu_map_idx, thread)).base
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__read_size(evsel: *mut perf_evsel) -> c_int {
    let read_format = (*evsel).attr.read_format;
    let mut entry = size_of::<u64>() as c_int; /* value */
    let mut size = 0;
    let mut nr = 1;

    if read_format & PERF_FORMAT_TOTAL_TIME_ENABLED != 0 {
        size += size_of::<u64>() as c_int;
    }

    if read_format & PERF_FORMAT_TOTAL_TIME_RUNNING != 0 {
        size += size_of::<u64>() as c_int;
    }

    if read_format & PERF_FORMAT_ID != 0 {
        entry += size_of::<u64>() as c_int;
    }

    if read_format & PERF_FORMAT_LOST != 0 {
        entry += size_of::<u64>() as c_int;
    }

    if read_format & PERF_FORMAT_GROUP != 0 {
        nr = (*evsel).nr_members;
        size += size_of::<u64>() as c_int;
    }

    size += entry * nr;
    size
}

/* This only reads values for the leader */
unsafe fn perf_evsel__read_group(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
    thread: c_int,
    count: *mut perf_counts_values,
) -> c_int {
    let size = perf_evsel__read_size(evsel) as size_t;
    let fd = FD(evsel, cpu_map_idx, thread);
    let read_format = (*evsel).attr.read_format;
    let mut idx: isize = 1;

    if fd.is_null() || *fd < 0 {
        return -EINVAL;
    }

    let data = calloc(1, size) as *mut u64;
    if data.is_null() {
        return -ENOMEM;
    }

    if readn(*fd, data as *mut c_void, size) <= 0 {
        free(data as *mut c_void);
        return -errno();
    }

    /*
     * This reads only the leader event intentionally since we don't have
     * perf counts values for sibling events.
     */
    if read_format & PERF_FORMAT_TOTAL_TIME_ENABLED != 0 {
        (*count).ena = *data.offset(idx);
        idx += 1;
    }
    if read_format & PERF_FORMAT_TOTAL_TIME_RUNNING != 0 {
        (*count).run = *data.offset(idx);
        idx += 1;
    }

    /* value is always available */
    (*count).val = *data.offset(idx);
    idx += 1;
    if read_format & PERF_FORMAT_ID != 0 {
        (*count).id = *data.offset(idx);
        idx += 1;
    }
    if read_format & PERF_FORMAT_LOST != 0 {
        (*count).lost = *data.offset(idx);
    }

    free(data as *mut c_void);
    0
}

/*
 * The perf read format is very flexible.  It needs to set the proper
 * values according to the read format.
 */
unsafe fn perf_evsel__adjust_values(
    evsel: *mut perf_evsel,
    buf: *mut u64,
    count: *mut perf_counts_values,
) {
    let read_format = (*evsel).attr.read_format;
    let mut n: isize = 0;

    (*count).val = *buf.offset(n);
    n += 1;

    if read_format & PERF_FORMAT_TOTAL_TIME_ENABLED != 0 {
        (*count).ena = *buf.offset(n);
        n += 1;
    }

    if read_format & PERF_FORMAT_TOTAL_TIME_RUNNING != 0 {
        (*count).run = *buf.offset(n);
        n += 1;
    }

    if read_format & PERF_FORMAT_ID != 0 {
        (*count).id = *buf.offset(n);
        n += 1;
    }

    if read_format & PERF_FORMAT_LOST != 0 {
        (*count).lost = *buf.offset(n);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__read(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
    thread: c_int,
    count: *mut perf_counts_values,
) -> c_int {
    let size = perf_evsel__read_size(evsel) as size_t;
    let fd = FD(evsel, cpu_map_idx, thread);
    let read_format = (*evsel).attr.read_format;
    let mut buf = perf_counts_values {
        val: 0,
        ena: 0,
        run: 0,
        id: 0,
        lost: 0,
    };

    memset(
        count as *mut c_void,
        0,
        size_of::<perf_counts_values>(),
    );

    if fd.is_null() || *fd < 0 {
        return -EINVAL;
    }

    if read_format & PERF_FORMAT_GROUP != 0 {
        return perf_evsel__read_group(evsel, cpu_map_idx, thread, count);
    }

    if !MMAP(evsel, cpu_map_idx, thread).is_null()
        && read_format & (PERF_FORMAT_ID | PERF_FORMAT_LOST) == 0
        && !perf_mmap__read_self(MMAP(evsel, cpu_map_idx, thread), count)
    {
        return 0;
    }

    if readn(*fd, &mut buf as *mut perf_counts_values as *mut c_void, size) <= 0 {
        return -errno();
    }

    perf_evsel__adjust_values(evsel, &mut buf as *mut perf_counts_values as *mut u64, count);
    0
}

unsafe fn perf_evsel__ioctl(
    evsel: *mut perf_evsel,
    ioc: c_int,
    arg: *mut c_void,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    let fd = FD(evsel, cpu_map_idx, thread);

    if fd.is_null() || *fd < 0 {
        return -1;
    }

    ioctl(*fd, ioc, arg)
}

unsafe fn perf_evsel__run_ioctl(
    evsel: *mut perf_evsel,
    ioc: c_int,
    arg: *mut c_void,
    cpu_map_idx: c_int,
) -> c_int {
    let mut thread = 0;

    while thread < xyarray__max_y((*evsel).fd) {
        let err = perf_evsel__ioctl(evsel, ioc, arg, cpu_map_idx, thread);

        if err != 0 {
            return err;
        }
        thread += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__enable_cpu(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
) -> c_int {
    perf_evsel__run_ioctl(evsel, PERF_EVENT_IOC_ENABLE, ptr::null_mut(), cpu_map_idx)
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__enable_thread(
    evsel: *mut perf_evsel,
    thread: c_int,
) -> c_int {
    let mut idx: c_int = 0;

    while idx < perf_cpu_map__nr((*evsel).cpus) {
        let err = perf_evsel__ioctl(evsel, PERF_EVENT_IOC_ENABLE, ptr::null_mut(), idx, thread);
        if err != 0 {
            return err;
        }
        idx += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__enable(evsel: *mut perf_evsel) -> c_int {
    let mut i = 0;
    let mut err = 0;

    while i < xyarray__max_x((*evsel).fd) && err == 0 {
        err = perf_evsel__run_ioctl(evsel, PERF_EVENT_IOC_ENABLE, ptr::null_mut(), i);
        i += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__disable_cpu(
    evsel: *mut perf_evsel,
    cpu_map_idx: c_int,
) -> c_int {
    perf_evsel__run_ioctl(evsel, PERF_EVENT_IOC_DISABLE, ptr::null_mut(), cpu_map_idx)
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__disable(evsel: *mut perf_evsel) -> c_int {
    let mut i = 0;
    let mut err = 0;

    while i < xyarray__max_x((*evsel).fd) && err == 0 {
        err = perf_evsel__run_ioctl(evsel, PERF_EVENT_IOC_DISABLE, ptr::null_mut(), i);
        i += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__apply_filter(
    evsel: *mut perf_evsel,
    filter: *const c_char,
) -> c_int {
    let mut err = 0;
    let mut i: c_int = 0;

    while i < perf_cpu_map__nr((*evsel).cpus) && err == 0 {
        err = perf_evsel__run_ioctl(
            evsel,
            PERF_EVENT_IOC_SET_FILTER,
            filter as *mut c_void,
            i,
        );
        i += 1;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__cpus(evsel: *mut perf_evsel) -> *mut perf_cpu_map {
    (*evsel).cpus
}

#[no_mangle]
pub unsafe extern "C" fn perf_evsel__threads(evsel: *mut perf_evsel) -> *mut perf_thread_map {
    (*evsel).threads
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
