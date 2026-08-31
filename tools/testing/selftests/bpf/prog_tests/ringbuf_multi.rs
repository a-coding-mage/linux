// SPDX-License-Identifier: GPL-2.0
// C dependencies in the original: test_progs.h, sys/epoll.h,
// test_ringbuf_multi.skel.h.

use core::ffi::{c_char, c_int, c_long, c_void};

static mut duration: c_int = 0;

#[repr(C)]
struct sample {
    pid: c_int,
    seq: c_int,
    value: c_long,
    comm: [c_char; 16],
}

unsafe extern "C" {
    fn getpagesize() -> c_int;
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;

    fn test_ringbuf_multi__open() -> *mut test_ringbuf_multi;
    fn test_ringbuf_multi__load(skel: *mut test_ringbuf_multi) -> c_int;
    fn test_ringbuf_multi__attach(skel: *mut test_ringbuf_multi) -> c_int;
    fn test_ringbuf_multi__destroy(skel: *mut test_ringbuf_multi);

    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_int) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: c_int,
        value_size: c_int,
        max_entries: c_int,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map__set_inner_map_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> c_int,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__ring(ringbuf: *mut ring_buffer, idx: c_int) -> *mut ring;
    fn ring_buffer__add(
        ringbuf: *mut ring_buffer,
        map_fd: c_int,
        sample_cb: unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> c_int,
        ctx: *mut c_void,
    ) -> c_int;
    fn ring_buffer__poll(ringbuf: *mut ring_buffer, timeout_ms: c_int) -> c_int;
    fn ring_buffer__free(ringbuf: *mut ring_buffer);
}

#[repr(C)]
struct test_ringbuf_multi {
    maps: test_ringbuf_multi_maps,
    bss: *mut test_ringbuf_multi_bss,
}

#[repr(C)]
struct test_ringbuf_multi_maps {
    ringbuf1: *mut bpf_map,
    ringbuf2: *mut bpf_map,
    ringbuf_hash: *mut bpf_map,
}

#[repr(C)]
struct test_ringbuf_multi_bss {
    pid: c_int,
    target_ring: c_int,
    value: c_long,
    dropped: c_long,
    skipped: c_long,
    total: c_long,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct ring {
    _private: [u8; 0],
}

const BPF_MAP_TYPE_RINGBUF: c_int = 27;
const __NR_getpgid: c_long = 121;

unsafe extern "C" fn process_sample(ctx: *mut c_void, data: *mut c_void, _len: usize) -> c_int {
    let ring: c_int = ctx as usize as c_int;
    let s: *mut sample = data as *mut sample;

    match unsafe { (*s).seq } {
        0 => {
            CHECK!(
                ring != 1,
                "sample1_ring",
                "exp %d, got %d\n",
                1,
                ring
            );
            CHECK!(
                unsafe { (*s).value } != 333,
                "sample1_value",
                "exp %ld, got %ld\n",
                333 as c_long,
                unsafe { (*s).value }
            );
        }
        1 => {
            CHECK!(
                ring != 2,
                "sample2_ring",
                "exp %d, got %d\n",
                2,
                ring
            );
            CHECK!(
                unsafe { (*s).value } != 777,
                "sample2_value",
                "exp %ld, got %ld\n",
                777 as c_long,
                unsafe { (*s).value }
            );
        }
        _ => {
            CHECK!(
                true,
                "extra_sample",
                "unexpected sample seq %d, val %ld\n",
                unsafe { (*s).seq },
                unsafe { (*s).value }
            );
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn test_ringbuf_multi() {
    let mut skel: *mut test_ringbuf_multi;
    let mut ringbuf: *mut ring_buffer = core::ptr::null_mut();
    let mut ring_old: *mut ring;
    let mut ring: *mut ring;
    let mut err: c_int;
    let page_size: c_int = unsafe { getpagesize() };
    let mut proto_fd: c_int = -1;

    skel = unsafe { test_ringbuf_multi__open() };
    if CHECK!(!skel.is_null(), "skel_open", "skeleton open failed\n") {
        return;
    }

    /* validate ringbuf size adjustment logic */
    ASSERT_EQ!(
        unsafe { bpf_map__max_entries((*skel).maps.ringbuf1) },
        page_size,
        "rb1_size_before"
    );
    ASSERT_OK!(
        unsafe { bpf_map__set_max_entries((*skel).maps.ringbuf1, page_size + 1) },
        "rb1_resize"
    );
    ASSERT_EQ!(
        unsafe { bpf_map__max_entries((*skel).maps.ringbuf1) },
        2 * page_size,
        "rb1_size_after"
    );
    ASSERT_OK!(
        unsafe { bpf_map__set_max_entries((*skel).maps.ringbuf1, page_size) },
        "rb1_reset"
    );
    ASSERT_EQ!(
        unsafe { bpf_map__max_entries((*skel).maps.ringbuf1) },
        page_size,
        "rb1_size_final"
    );

    proto_fd = unsafe {
        bpf_map_create(
            BPF_MAP_TYPE_RINGBUF,
            core::ptr::null(),
            0,
            0,
            page_size,
            core::ptr::null(),
        )
    };
    if CHECK!(
        proto_fd < 0,
        "bpf_map_create",
        "bpf_map_create failed\n"
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    err = unsafe { bpf_map__set_inner_map_fd((*skel).maps.ringbuf_hash, proto_fd) };
    if CHECK!(
        err != 0,
        "bpf_map__set_inner_map_fd",
        "bpf_map__set_inner_map_fd failed\n"
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    err = unsafe { test_ringbuf_multi__load(skel) };
    if CHECK!(err != 0, "skel_load", "skeleton load failed\n") {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    unsafe {
        close(proto_fd);
    }
    proto_fd = -1;

    /* make sure we can't resize ringbuf after object load */
    if !ASSERT_ERR!(
        unsafe { bpf_map__set_max_entries((*skel).maps.ringbuf1, 3 * page_size) },
        "rb1_resize_after_load"
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    /* only trigger BPF program for current process */
    unsafe {
        (*(*skel).bss).pid = getpid();
    }

    ringbuf = unsafe {
        ring_buffer__new(
            bpf_map__fd((*skel).maps.ringbuf1),
            process_sample,
            1isize as *mut c_void,
            core::ptr::null(),
        )
    };
    if CHECK!(
        ringbuf.is_null(),
        "ringbuf_create",
        "failed to create ringbuf\n"
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    /* verify ring_buffer__ring returns expected results */
    ring = unsafe { ring_buffer__ring(ringbuf, 0) };
    if !ASSERT_OK_PTR!(ring, "ring_buffer__ring_idx_0") {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }
    ring_old = ring;
    ring = unsafe { ring_buffer__ring(ringbuf, 1) };
    ASSERT_ERR_PTR!(ring, "ring_buffer__ring_idx_1");

    err = unsafe {
        ring_buffer__add(
            ringbuf,
            bpf_map__fd((*skel).maps.ringbuf2),
            process_sample,
            2isize as *mut c_void,
        )
    };
    if CHECK!(err != 0, "ringbuf_add", "failed to add another ring\n") {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    /* verify adding a new ring didn't invalidate our older pointer */
    ring = unsafe { ring_buffer__ring(ringbuf, 0) };
    if !ASSERT_EQ!(ring, ring_old, "ring_buffer__ring_again") {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    err = unsafe { test_ringbuf_multi__attach(skel) };
    if CHECK!(
        err != 0,
        "skel_attach",
        "skeleton attachment failed: %d\n",
        err
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    /* trigger few samples, some will be skipped */
    unsafe {
        (*(*skel).bss).target_ring = 0;
        (*(*skel).bss).value = 333;
        syscall(__NR_getpgid);
    }

    /* skipped, no ringbuf in slot 1 */
    unsafe {
        (*(*skel).bss).target_ring = 1;
        (*(*skel).bss).value = 555;
        syscall(__NR_getpgid);
    }

    unsafe {
        (*(*skel).bss).target_ring = 2;
        (*(*skel).bss).value = 777;
        syscall(__NR_getpgid);
    }

    /* poll for samples, should get 2 ringbufs back */
    err = unsafe { ring_buffer__poll(ringbuf, -1) };
    if CHECK!(
        err != 2,
        "poll_res",
        "expected 2 records, got %d\n",
        err
    ) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    /* expect extra polling to return nothing */
    err = unsafe { ring_buffer__poll(ringbuf, 0) };
    if CHECK!(err < 0, "extra_samples", "poll result: %d\n", err) {
        goto_cleanup(skel, ringbuf, proto_fd);
        return;
    }

    CHECK!(
        unsafe { (*(*skel).bss).dropped } != 0,
        "err_dropped",
        "exp %ld, got %ld\n",
        0 as c_long,
        unsafe { (*(*skel).bss).dropped }
    );
    CHECK!(
        unsafe { (*(*skel).bss).skipped } != 1,
        "err_skipped",
        "exp %ld, got %ld\n",
        1 as c_long,
        unsafe { (*(*skel).bss).skipped }
    );
    CHECK!(
        unsafe { (*(*skel).bss).total } != 2,
        "err_total",
        "exp %ld, got %ld\n",
        2 as c_long,
        unsafe { (*(*skel).bss).total }
    );

    goto_cleanup(skel, ringbuf, proto_fd);
}

unsafe fn goto_cleanup(
    skel: *mut test_ringbuf_multi,
    ringbuf: *mut ring_buffer,
    proto_fd: c_int,
) {
    if proto_fd >= 0 {
        unsafe {
            close(proto_fd);
        }
    }
    unsafe {
        ring_buffer__free(ringbuf);
        test_ringbuf_multi__destroy(skel);
    }
}
