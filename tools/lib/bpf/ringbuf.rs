// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/*
 * Ring buffer operations.
 *
 * Copyright (C) 2020 Facebook, Inc.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type uintptr_t = usize;
type int64_t = i64;
type __u32 = u32;
type __u64 = u64;

type ring_buffer_sample_fn =
    Option<unsafe extern "C" fn(ctx: *mut c_void, data: *mut c_void, size: __u32) -> c_int>;

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[repr(C)]
pub struct epoll_data {
    pub fd: c_int,
}

#[repr(C)]
pub struct epoll_event {
    pub events: __u32,
    pub data: epoll_data,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct ring_buffer_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_ring_buffer_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ring {
    sample_cb: ring_buffer_sample_fn,
    ctx: *mut c_void,
    data: *mut c_void,
    consumer_pos: *mut c_ulong,
    producer_pos: *mut c_ulong,
    mask: c_ulong,
    map_fd: c_int,
}

#[repr(C)]
pub struct ring_buffer {
    events: *mut epoll_event,
    rings: *mut *mut ring,
    page_size: size_t,
    epoll_fd: c_int,
    ring_cnt: c_int,
}

#[repr(C)]
pub struct user_ring_buffer {
    event: epoll_event,
    consumer_pos: *mut c_ulong,
    producer_pos: *mut c_ulong,
    data: *mut c_void,
    mask: c_ulong,
    page_size: size_t,
    map_fd: c_int,
    epoll_fd: c_int,
}

/* 8-byte ring buffer header structure */
#[repr(C)]
struct ringbuf_hdr {
    len: __u32,
    pad: __u32,
}

const BPF_MAP_TYPE_RINGBUF: __u32 = 27;
const BPF_MAP_TYPE_USER_RINGBUF: __u32 = 31;
const BPF_RINGBUF_BUSY_BIT: __u32 = 1 << 31;
const BPF_RINGBUF_DISCARD_BIT: __u32 = 1 << 30;
const BPF_RINGBUF_HDR_SZ: __u32 = 8;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const E2BIG: c_int = 7;
const ENOSPC: c_int = 28;
const INT_MAX: c_int = c_int::MAX;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const EPOLLIN: __u32 = 0x001;
const EPOLLOUT: __u32 = 0x004;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CLOEXEC: c_int = 0o2000000;
const CLOCK_MONOTONIC: c_int = 1;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    static mut errno: c_int;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;

    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *__u32) -> c_int;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_err(err: c_int) -> c_int;
    fn errstr(err: c_int) -> *const c_char;
    fn pr_warn(fmt: *const c_char, ...);
}

unsafe fn OPTS_VALID<T>(_opts: *const T, _name: &str) -> bool {
    true
}

unsafe fn smp_load_acquire<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

unsafe fn smp_store_release<T>(p: *mut T, v: T) {
    core::ptr::write_volatile(p, v);
}

unsafe fn atomic_exchange_n_u32(p: *mut __u32, v: __u32) -> __u32 {
    core::ptr::replace(p, v)
}

unsafe fn ringbuf_free_ring(rb: *mut ring_buffer, r: *mut ring) {
    if !(*r).consumer_pos.is_null() {
        munmap((*r).consumer_pos as *mut c_void, (*rb).page_size);
        (*r).consumer_pos = ptr::null_mut();
    }
    if !(*r).producer_pos.is_null() {
        munmap(
            (*r).producer_pos as *mut c_void,
            (*rb).page_size + 2 * ((*r).mask + 1) as size_t,
        );
        (*r).producer_pos = ptr::null_mut();
    }

    free(r as *mut c_void);
}

/* Add extra RINGBUF maps to this ring buffer manager */
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__add(
    rb: *mut ring_buffer,
    map_fd: c_int,
    sample_cb: ring_buffer_sample_fn,
    ctx: *mut c_void,
) -> c_int {
    let mut info: bpf_map_info = mem::zeroed();
    let mut len: __u32 = mem::size_of::<bpf_map_info>() as __u32;
    let mut err: c_int;

    memset(
        &mut info as *mut bpf_map_info as *mut c_void,
        0,
        mem::size_of::<bpf_map_info>(),
    );

    err = bpf_map_get_info_by_fd(map_fd, &mut info, &mut len);
    if err != 0 {
        err = -errno;
        pr_warn(
            b"ringbuf: failed to get map info for fd=%d: %s\n\0".as_ptr() as *const c_char,
            map_fd,
            errstr(err),
        );
        return libbpf_err(err);
    }

    if info.type_ != BPF_MAP_TYPE_RINGBUF {
        pr_warn(
            b"ringbuf: map fd=%d is not BPF_MAP_TYPE_RINGBUF\n\0".as_ptr() as *const c_char,
            map_fd,
        );
        return libbpf_err(-EINVAL);
    }

    let mut tmp = libbpf_reallocarray(
        (*rb).rings as *mut c_void,
        ((*rb).ring_cnt + 1) as size_t,
        mem::size_of::<*mut ring>(),
    );
    if tmp.is_null() {
        return libbpf_err(-ENOMEM);
    }
    (*rb).rings = tmp as *mut *mut ring;

    tmp = libbpf_reallocarray(
        (*rb).events as *mut c_void,
        ((*rb).ring_cnt + 1) as size_t,
        mem::size_of::<epoll_event>(),
    );
    if tmp.is_null() {
        return libbpf_err(-ENOMEM);
    }
    (*rb).events = tmp as *mut epoll_event;

    let r = calloc(1, mem::size_of::<ring>()) as *mut ring;
    if r.is_null() {
        return libbpf_err(-ENOMEM);
    }
    *(*rb).rings.add((*rb).ring_cnt as usize) = r;

    (*r).map_fd = map_fd;
    (*r).sample_cb = sample_cb;
    (*r).ctx = ctx;
    (*r).mask = (info.max_entries - 1) as c_ulong;

    /* Map writable consumer page */
    tmp = mmap(
        ptr::null_mut(),
        (*rb).page_size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        map_fd,
        0,
    );
    if tmp == MAP_FAILED {
        err = -errno;
        pr_warn(
            b"ringbuf: failed to mmap consumer page for map fd=%d: %s\n\0".as_ptr()
                as *const c_char,
            map_fd,
            errstr(err),
        );
        ringbuf_free_ring(rb, r);
        return libbpf_err(err);
    }
    (*r).consumer_pos = tmp as *mut c_ulong;

    /* Map read-only producer page and data pages. We map twice as big
     * data size to allow simple reading of samples that wrap around the
     * end of a ring buffer. See kernel implementation for details.
     */
    let mmap_sz: __u64 = (*rb).page_size as __u64 + 2 * info.max_entries as __u64;
    if mmap_sz != mmap_sz as size_t as __u64 {
        err = -E2BIG;
        pr_warn(
            b"ringbuf: ring buffer size (%u) is too big\n\0".as_ptr() as *const c_char,
            info.max_entries,
        );
        ringbuf_free_ring(rb, r);
        return libbpf_err(err);
    }
    tmp = mmap(
        ptr::null_mut(),
        mmap_sz as size_t,
        PROT_READ,
        MAP_SHARED,
        map_fd,
        (*rb).page_size as isize,
    );
    if tmp == MAP_FAILED {
        err = -errno;
        pr_warn(
            b"ringbuf: failed to mmap data pages for map fd=%d: %s\n\0".as_ptr() as *const c_char,
            map_fd,
            errstr(err),
        );
        ringbuf_free_ring(rb, r);
        return libbpf_err(err);
    }
    (*r).producer_pos = tmp as *mut c_ulong;
    (*r).data = (tmp as *mut u8).add((*rb).page_size) as *mut c_void;

    let e = (*rb).events.add((*rb).ring_cnt as usize);
    memset(e as *mut c_void, 0, mem::size_of::<epoll_event>());

    (*e).events = EPOLLIN;
    (*e).data.fd = (*rb).ring_cnt;
    if epoll_ctl((*rb).epoll_fd, EPOLL_CTL_ADD, map_fd, e) < 0 {
        err = -errno;
        pr_warn(
            b"ringbuf: failed to epoll add map fd=%d: %s\n\0".as_ptr() as *const c_char,
            map_fd,
            errstr(err),
        );
        ringbuf_free_ring(rb, r);
        return libbpf_err(err);
    }

    (*rb).ring_cnt += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn ring_buffer__free(rb: *mut ring_buffer) {
    if rb.is_null() {
        return;
    }

    let mut i = 0;
    while i < (*rb).ring_cnt {
        ringbuf_free_ring(rb, *(*rb).rings.add(i as usize));
        i += 1;
    }
    if (*rb).epoll_fd >= 0 {
        close((*rb).epoll_fd);
    }

    free((*rb).events as *mut c_void);
    free((*rb).rings as *mut c_void);
    free(rb as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ring_buffer__new(
    map_fd: c_int,
    sample_cb: ring_buffer_sample_fn,
    ctx: *mut c_void,
    opts: *const ring_buffer_opts,
) -> *mut ring_buffer {
    let mut err: c_int;

    if !OPTS_VALID(opts, "ring_buffer_opts") {
        errno = EINVAL;
        return ptr::null_mut();
    }

    let rb = calloc(1, mem::size_of::<ring_buffer>()) as *mut ring_buffer;
    if rb.is_null() {
        errno = ENOMEM;
        return ptr::null_mut();
    }

    (*rb).page_size = getpagesize() as size_t;

    (*rb).epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if (*rb).epoll_fd < 0 {
        err = -errno;
        pr_warn(
            b"ringbuf: failed to create epoll instance: %s\n\0".as_ptr() as *const c_char,
            errstr(err),
        );
        ring_buffer__free(rb);
        errno = -err;
        return ptr::null_mut();
    }

    err = ring_buffer__add(rb, map_fd, sample_cb, ctx);
    if err != 0 {
        ring_buffer__free(rb);
        errno = -err;
        return ptr::null_mut();
    }

    rb
}

#[inline]
unsafe fn roundup_len(mut len: __u32) -> c_int {
    /* clear out top 2 bits (discard and busy, if set) */
    len <<= 2;
    len >>= 2;
    /* add length prefix */
    len = len.wrapping_add(BPF_RINGBUF_HDR_SZ);
    /* round up to 8 byte alignment */
    (((len + 7) / 8) * 8) as c_int
}

unsafe fn ringbuf_process_ring(r: *mut ring, n: size_t) -> int64_t {
    let mut cnt: int64_t = 0;
    let mut cons_pos: c_ulong;
    let mut prod_pos: c_ulong;
    let mut got_new_data: bool;

    cons_pos = smp_load_acquire((*r).consumer_pos);
    loop {
        got_new_data = false;
        prod_pos = smp_load_acquire((*r).producer_pos);
        while prod_pos.wrapping_sub(cons_pos) > 0 {
            let len_ptr = ((*r).data as *mut u8).add((cons_pos & (*r).mask) as usize) as *mut c_int;
            let len = smp_load_acquire(len_ptr);

            /* sample not committed yet, bail out for now */
            if (len as __u32) & BPF_RINGBUF_BUSY_BIT != 0 {
                return cnt;
            }

            got_new_data = true;
            cons_pos = cons_pos.wrapping_add(roundup_len(len as __u32) as c_ulong);

            if (len as __u32) & BPF_RINGBUF_DISCARD_BIT == 0 {
                let sample = (len_ptr as *mut u8).add(BPF_RINGBUF_HDR_SZ as usize) as *mut c_void;
                let err = ((*r).sample_cb.unwrap())((*r).ctx, sample, len as __u32);
                if err < 0 {
                    /* update consumer pos and bail out */
                    smp_store_release((*r).consumer_pos, cons_pos);
                    return err as int64_t;
                }
                cnt += 1;
            }

            smp_store_release((*r).consumer_pos, cons_pos);

            if cnt >= n as int64_t {
                return cnt;
            }
        }
        if !got_new_data {
            break;
        }
    }
    cnt
}

/* Consume available ring buffer(s) data without event polling, up to n
 * records.
 *
 * Returns number of records consumed across all registered ring buffers (or
 * n, whichever is less), or negative number if any of the callbacks return
 * error.
 */
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__consume_n(rb: *mut ring_buffer, mut n: size_t) -> c_int {
    let mut res: int64_t = 0;
    let mut i = 0;

    while i < (*rb).ring_cnt {
        let ring = *(*rb).rings.add(i as usize);

        let err = ringbuf_process_ring(ring, n);
        if err < 0 {
            return libbpf_err(err as c_int);
        }
        res += err;
        n -= err as size_t;

        if n == 0 {
            break;
        }
        i += 1;
    }
    if res > INT_MAX as int64_t {
        INT_MAX
    } else {
        res as c_int
    }
}

/* Consume available ring buffer(s) data without event polling.
 * Returns number of records consumed across all registered ring buffers (or
 * INT_MAX, whichever is less), or negative number if any of the callbacks
 * return error.
 */
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int {
    let mut res: int64_t = 0;
    let mut i = 0;

    while i < (*rb).ring_cnt {
        let ring = *(*rb).rings.add(i as usize);

        let err = ringbuf_process_ring(ring, INT_MAX as size_t);
        if err < 0 {
            return libbpf_err(err as c_int);
        }
        res += err;
        if res > INT_MAX as int64_t {
            res = INT_MAX as int64_t;
            break;
        }
        i += 1;
    }
    res as c_int
}

/* Poll for available data and consume records, if any are available.
 * Returns number of records consumed (or INT_MAX, whichever is less), or
 * negative number, if any of the registered callbacks returned error.
 */
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int {
    let mut res: int64_t = 0;

    let cnt = epoll_wait((*rb).epoll_fd, (*rb).events, (*rb).ring_cnt, timeout_ms);
    if cnt < 0 {
        return libbpf_err(-errno);
    }

    let mut i = 0;
    while i < cnt {
        let ring_id: __u32 = (*(*rb).events.add(i as usize)).data.fd as __u32;
        let ring = *(*rb).rings.add(ring_id as usize);

        let err = ringbuf_process_ring(ring, INT_MAX as size_t);
        if err < 0 {
            return libbpf_err(err as c_int);
        }
        res += err;
        i += 1;
    }
    if res > INT_MAX as int64_t {
        res = INT_MAX as int64_t;
    }
    res as c_int
}

/* Get an fd that can be used to sleep until data is available in the ring(s) */
#[no_mangle]
pub unsafe extern "C" fn ring_buffer__epoll_fd(rb: *const ring_buffer) -> c_int {
    (*rb).epoll_fd
}

#[no_mangle]
pub unsafe extern "C" fn ring_buffer__ring(rb: *mut ring_buffer, idx: c_uint) -> *mut ring {
    if idx >= (*rb).ring_cnt as c_uint {
        errno = ERANGE;
        return ptr::null_mut();
    }

    *(*rb).rings.add(idx as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ring__consumer_pos(r: *const ring) -> c_ulong {
    /* Synchronizes with smp_store_release() in ringbuf_process_ring(). */
    smp_load_acquire((*r).consumer_pos)
}

#[no_mangle]
pub unsafe extern "C" fn ring__producer_pos(r: *const ring) -> c_ulong {
    /* Synchronizes with smp_store_release() in __bpf_ringbuf_reserve() in
     * the kernel.
     */
    smp_load_acquire((*r).producer_pos)
}

#[no_mangle]
pub unsafe extern "C" fn ring__avail_data_size(r: *const ring) -> size_t {
    let cons_pos = ring__consumer_pos(r);
    let prod_pos = ring__producer_pos(r);
    prod_pos.wrapping_sub(cons_pos) as size_t
}

#[no_mangle]
pub unsafe extern "C" fn ring__size(r: *const ring) -> size_t {
    ((*r).mask + 1) as size_t
}

#[no_mangle]
pub unsafe extern "C" fn ring__map_fd(r: *const ring) -> c_int {
    (*r).map_fd
}

#[no_mangle]
pub unsafe extern "C" fn ring__consume_n(r: *mut ring, n: size_t) -> c_int {
    let res = ringbuf_process_ring(r, n);
    if res < 0 {
        return libbpf_err(res as c_int);
    }

    if res > INT_MAX as int64_t {
        INT_MAX
    } else {
        res as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn ring__consume(r: *mut ring) -> c_int {
    ring__consume_n(r, INT_MAX as size_t)
}

unsafe fn user_ringbuf_unmap_ring(rb: *mut user_ring_buffer) {
    if !(*rb).consumer_pos.is_null() {
        munmap((*rb).consumer_pos as *mut c_void, (*rb).page_size);
        (*rb).consumer_pos = ptr::null_mut();
    }
    if !(*rb).producer_pos.is_null() {
        munmap(
            (*rb).producer_pos as *mut c_void,
            (*rb).page_size + 2 * ((*rb).mask + 1) as size_t,
        );
        (*rb).producer_pos = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__free(rb: *mut user_ring_buffer) {
    if rb.is_null() {
        return;
    }

    user_ringbuf_unmap_ring(rb);

    if (*rb).epoll_fd >= 0 {
        close((*rb).epoll_fd);
    }

    free(rb as *mut c_void);
}

unsafe fn user_ringbuf_map(rb: *mut user_ring_buffer, map_fd: c_int) -> c_int {
    let mut info: bpf_map_info = mem::zeroed();
    let mut len: __u32 = mem::size_of::<bpf_map_info>() as __u32;
    let mut err: c_int;

    memset(
        &mut info as *mut bpf_map_info as *mut c_void,
        0,
        mem::size_of::<bpf_map_info>(),
    );

    err = bpf_map_get_info_by_fd(map_fd, &mut info, &mut len);
    if err != 0 {
        err = -errno;
        pr_warn(
            b"user ringbuf: failed to get map info for fd=%d: %s\n\0".as_ptr() as *const c_char,
            map_fd,
            errstr(err),
        );
        return err;
    }

    if info.type_ != BPF_MAP_TYPE_USER_RINGBUF {
        pr_warn(
            b"user ringbuf: map fd=%d is not BPF_MAP_TYPE_USER_RINGBUF\n\0".as_ptr()
                as *const c_char,
            map_fd,
        );
        return -EINVAL;
    }

    (*rb).map_fd = map_fd;
    (*rb).mask = (info.max_entries - 1) as c_ulong;

    /* Map read-only consumer page */
    let mut tmp = mmap(ptr::null_mut(), (*rb).page_size, PROT_READ, MAP_SHARED, map_fd, 0);
    if tmp == MAP_FAILED {
        err = -errno;
        pr_warn(
            b"user ringbuf: failed to mmap consumer page for map fd=%d: %s\n\0".as_ptr()
                as *const c_char,
            map_fd,
            errstr(err),
        );
        return err;
    }
    (*rb).consumer_pos = tmp as *mut c_ulong;

    /* Map read-write the producer page and data pages. We map the data
     * region as twice the total size of the ring buffer to allow the
     * simple reading and writing of samples that wrap around the end of
     * the buffer.  See the kernel implementation for details.
     */
    let mmap_sz: __u64 = (*rb).page_size as __u64 + 2 * info.max_entries as __u64;
    if mmap_sz != mmap_sz as size_t as __u64 {
        pr_warn(
            b"user ringbuf: ring buf size (%u) is too big\n\0".as_ptr() as *const c_char,
            info.max_entries,
        );
        return -E2BIG;
    }
    tmp = mmap(
        ptr::null_mut(),
        mmap_sz as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        map_fd,
        (*rb).page_size as isize,
    );
    if tmp == MAP_FAILED {
        err = -errno;
        pr_warn(
            b"user ringbuf: failed to mmap data pages for map fd=%d: %s\n\0".as_ptr()
                as *const c_char,
            map_fd,
            errstr(err),
        );
        return err;
    }

    (*rb).producer_pos = tmp as *mut c_ulong;
    (*rb).data = (tmp as *mut u8).add((*rb).page_size) as *mut c_void;

    let rb_epoll = &mut (*rb).event as *mut epoll_event;
    (*rb_epoll).events = EPOLLOUT;
    if epoll_ctl((*rb).epoll_fd, EPOLL_CTL_ADD, map_fd, rb_epoll) < 0 {
        err = -errno;
        pr_warn(
            b"user ringbuf: failed to epoll add map fd=%d: %s\n\0".as_ptr() as *const c_char,
            map_fd,
            errstr(err),
        );
        return err;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__new(
    map_fd: c_int,
    opts: *const user_ring_buffer_opts,
) -> *mut user_ring_buffer {
    let mut err: c_int;

    if !OPTS_VALID(opts, "user_ring_buffer_opts") {
        errno = EINVAL;
        return ptr::null_mut();
    }

    let rb = calloc(1, mem::size_of::<user_ring_buffer>()) as *mut user_ring_buffer;
    if rb.is_null() {
        errno = ENOMEM;
        return ptr::null_mut();
    }

    (*rb).page_size = getpagesize() as size_t;

    (*rb).epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if (*rb).epoll_fd < 0 {
        err = -errno;
        pr_warn(
            b"user ringbuf: failed to create epoll instance: %s\n\0".as_ptr() as *const c_char,
            errstr(err),
        );
        user_ring_buffer__free(rb);
        errno = -err;
        return ptr::null_mut();
    }

    err = user_ringbuf_map(rb, map_fd);
    if err != 0 {
        user_ring_buffer__free(rb);
        errno = -err;
        return ptr::null_mut();
    }

    rb
}

unsafe fn user_ringbuf_commit(rb: *mut user_ring_buffer, sample: *mut c_void, discard: bool) {
    let hdr_offset: uintptr_t =
        (*rb).mask as uintptr_t + 1 + (sample as uintptr_t - (*rb).data as uintptr_t)
            - BPF_RINGBUF_HDR_SZ as uintptr_t;
    let hdr = ((*rb).data as *mut u8).add(hdr_offset & (*rb).mask as uintptr_t) as *mut ringbuf_hdr;

    let mut new_len = (*hdr).len & !BPF_RINGBUF_BUSY_BIT;
    if discard {
        new_len |= BPF_RINGBUF_DISCARD_BIT;
    }

    /* Synchronizes with smp_load_acquire() in __bpf_user_ringbuf_peek() in
     * the kernel.
     */
    atomic_exchange_n_u32(&mut (*hdr).len, new_len);
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__discard(rb: *mut user_ring_buffer, sample: *mut c_void) {
    user_ringbuf_commit(rb, sample, true);
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__submit(rb: *mut user_ring_buffer, sample: *mut c_void) {
    user_ringbuf_commit(rb, sample, false);
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__reserve(
    rb: *mut user_ring_buffer,
    size: __u32,
) -> *mut c_void {
    let avail_size: __u32;
    let total_size: __u32;
    let max_size: __u32;
    /* 64-bit to avoid overflow in case of extreme application behavior */
    let cons_pos: __u64;
    let prod_pos: __u64;
    let hdr: *mut ringbuf_hdr;

    /* The top two bits are used as special flags */
    if size & (BPF_RINGBUF_BUSY_BIT | BPF_RINGBUF_DISCARD_BIT) != 0 {
        errno = E2BIG;
        return ptr::null_mut();
    }

    /* Synchronizes with smp_store_release() in __bpf_user_ringbuf_peek() in
     * the kernel.
     */
    cons_pos = smp_load_acquire((*rb).consumer_pos) as __u64;
    /* Synchronizes with smp_store_release() in user_ringbuf_commit() */
    prod_pos = smp_load_acquire((*rb).producer_pos) as __u64;

    max_size = ((*rb).mask + 1) as __u32;
    avail_size = max_size.wrapping_sub(prod_pos.wrapping_sub(cons_pos) as __u32);
    /* Round up total size to a multiple of 8. */
    total_size = ((size + BPF_RINGBUF_HDR_SZ + 7) / 8) * 8;

    if total_size > max_size {
        errno = E2BIG;
        return ptr::null_mut();
    }

    if avail_size < total_size {
        errno = ENOSPC;
        return ptr::null_mut();
    }

    hdr = ((*rb).data as *mut u8).add((prod_pos as c_ulong & (*rb).mask) as usize) as *mut ringbuf_hdr;
    (*hdr).len = size | BPF_RINGBUF_BUSY_BIT;
    (*hdr).pad = 0;

    /* Synchronizes with smp_load_acquire() in __bpf_user_ringbuf_peek() in
     * the kernel.
     */
    smp_store_release((*rb).producer_pos, (prod_pos + total_size as __u64) as c_ulong);

    ((*rb).data as *mut u8).add(((prod_pos + BPF_RINGBUF_HDR_SZ as __u64) as c_ulong & (*rb).mask) as usize)
        as *mut c_void
}

unsafe fn ns_elapsed_timespec(start: *const timespec, end: *const timespec) -> __u64 {
    let ns_per_s: __u64 = 1000000000;

    let start_ns: __u64 = (*start).tv_sec as __u64 * ns_per_s + (*start).tv_nsec as __u64;
    let end_ns: __u64 = (*end).tv_sec as __u64 * ns_per_s + (*end).tv_nsec as __u64;

    end_ns - start_ns
}

#[no_mangle]
pub unsafe extern "C" fn user_ring_buffer__reserve_blocking(
    rb: *mut user_ring_buffer,
    size: __u32,
    timeout_ms: c_int,
) -> *mut c_void {
    let mut sample: *mut c_void;
    let mut err: c_int;
    let mut ms_remaining: c_int = timeout_ms;
    let mut start: timespec = mem::zeroed();

    if timeout_ms < 0 && timeout_ms != -1 {
        errno = EINVAL;
        return ptr::null_mut();
    }

    if timeout_ms != -1 {
        err = clock_gettime(CLOCK_MONOTONIC, &mut start);
        if err != 0 {
            return ptr::null_mut();
        }
    }

    loop {
        let cnt: c_int;
        let ms_elapsed: c_int;
        let mut curr: timespec = mem::zeroed();
        let ns_per_ms: __u64 = 1000000;

        sample = user_ring_buffer__reserve(rb, size);
        if !sample.is_null() {
            return sample;
        } else if errno != ENOSPC {
            return ptr::null_mut();
        }

        /* The kernel guarantees at least one event notification
         * delivery whenever at least one sample is drained from the
         * ring buffer in an invocation to bpf_ringbuf_drain(). Other
         * additional events may be delivered at any time, but only one
         * event is guaranteed per bpf_ringbuf_drain() invocation,
         * provided that a sample is drained, and the BPF program did
         * not pass BPF_RB_NO_WAKEUP to bpf_ringbuf_drain(). If
         * BPF_RB_FORCE_WAKEUP is passed to bpf_ringbuf_drain(), a
         * wakeup event will be delivered even if no samples are
         * drained.
         */
        cnt = epoll_wait((*rb).epoll_fd, &mut (*rb).event, 1, ms_remaining);
        if cnt < 0 {
            return ptr::null_mut();
        }

        if timeout_ms == -1 {
            continue;
        }

        err = clock_gettime(CLOCK_MONOTONIC, &mut curr);
        if err != 0 {
            return ptr::null_mut();
        }

        ms_elapsed = (ns_elapsed_timespec(&start, &curr) / ns_per_ms) as c_int;
        ms_remaining = timeout_ms - ms_elapsed;
        if ms_remaining <= 0 {
            break;
        }
    }

    /* Try one more time to reserve a sample after the specified timeout has elapsed. */
    user_ring_buffer__reserve(rb, size)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
