// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from testing/selftests/bpf/prog_tests/user_ringbuf.c.
 * C includes are intentionally not executable Rust; the referenced libbpf,
 * skeleton, kernel, and test harness symbols are declared as external
 * dependencies below.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type __u32 = u32;
type __u64 = u64;
type s32 = i32;
type s64 = i64;

const BPF_RINGBUF_HDR_SZ: c_long = 8;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_SHARED: c_int = 0x01;
const MREMAP_MAYMOVE: c_int = 1;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const __NR_getpgid: c_long = 121;
const __NR_prctl: c_long = 157;
const __NR_prlimit64: c_long = 302;

#[repr(C)]
pub struct sample {
    pub pid: c_int,
    pub seq: __u32,
    pub value: __u32,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct test_msg {
    pub msg_op: test_msg_op,
    pub operand_64: s64,
    pub operand_32: s32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_msg_op {
    TEST_MSG_OP_INC64 = 0,
    TEST_MSG_OP_INC32 = 1,
    TEST_MSG_OP_MUL64 = 2,
    TEST_MSG_OP_MUL32 = 3,
    TEST_MSG_OP_NUM_OPS = 4,
}

const TEST_OP_64: __u64 = 2;
const TEST_OP_32: __u32 = 3;

#[repr(C)]
pub struct user_ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_ringbuf_success_maps {
    pub user_ringbuf: *mut bpf_map,
    pub kernel_ringbuf: *mut bpf_map,
}

#[repr(C)]
pub struct user_ringbuf_success_bss {
    pub pid: c_int,
    pub read: c_long,
    pub err: c_int,
    pub user_mutated: __u64,
    pub kern_mutated: __u64,
}

#[repr(C)]
pub struct user_ringbuf_success {
    pub maps: user_ringbuf_success_maps,
    pub bss: *mut user_ringbuf_success_bss,
}

type ring_buffer_sample_fn =
    Option<unsafe extern "C" fn(ctx: *mut c_void, data: *mut c_void, len: size_t) -> c_int>;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn syscall(num: c_long, ...) -> c_long;
    fn getpid() -> c_int;
    fn getpagesize() -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: size_t,
        new_size: size_t,
        flags: c_int,
        ...
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;

    fn user_ringbuf_success__open() -> *mut user_ringbuf_success;
    fn user_ringbuf_success__load(skel: *mut user_ringbuf_success) -> c_int;
    fn user_ringbuf_success__attach(skel: *mut user_ringbuf_success) -> c_int;
    fn user_ringbuf_success__destroy(skel: *mut user_ringbuf_success);

    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_long) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: ring_buffer_sample_fn,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__free(rb: *mut ring_buffer);
    fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;

    fn user_ring_buffer__new(map_fd: c_int, opts: *mut c_void) -> *mut user_ring_buffer;
    fn user_ring_buffer__free(rb: *mut user_ring_buffer);
    fn user_ring_buffer__reserve(rb: *mut user_ring_buffer, size: size_t) -> *mut c_void;
    fn user_ring_buffer__reserve_blocking(
        rb: *mut user_ring_buffer,
        size: size_t,
        timeout_ms: c_int,
    ) -> *mut c_void;
    fn user_ring_buffer__submit(rb: *mut user_ring_buffer, sample: *mut c_void);
    fn user_ring_buffer__discard(rb: *mut user_ring_buffer, sample: *mut c_void);

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ_LONG(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_EQ_U64(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_EQ_PTR(actual: *const c_void, expected: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GT_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT_LONG(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_GE_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LE_LONG(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_LT_LONG(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn PRINT_FAIL(format: *const c_char, ...);
    fn RUN_TESTS_user_ringbuf_fail();
}

type pthread_t = usize;

static C_SAMPLE_SIZE: c_long = (size_of::<sample>() as c_long) + BPF_RINGBUF_HDR_SZ;
static mut c_ringbuf_size: c_long = 0;
static mut c_max_entries: c_long = 0;

unsafe fn smp_store_release(p: *mut __u64, v: __u64) {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    ptr::write_volatile(p, v);
}

unsafe fn drain_current_samples() {
    syscall(__NR_getpgid);
}

unsafe fn write_samples(ringbuf: *mut user_ring_buffer, num_samples: u32) -> c_int {
    let mut err: c_int = 0;

    /* Write some number of samples to the ring buffer. */
    let mut i: u32 = 0;
    while i < num_samples {
        let entry = user_ring_buffer__reserve(ringbuf, size_of::<sample>()) as *mut sample;
        if entry.is_null() {
            err = -errno;
            goto_done_write_samples(err);
            return err;
        }

        (*entry).pid = getpid();
        (*entry).seq = i;
        (*entry).value = i.wrapping_mul(i);

        let read = snprintf(
            (*entry).comm.as_mut_ptr(),
            (*entry).comm.len(),
            c"%u".as_ptr(),
            i as c_uint,
        );
        if read <= 0 {
            /* Assert on the error path to avoid spamming logs with
             * mostly success messages.
             */
            ASSERT_GT_INT(read, 0, c"snprintf_comm".as_ptr());
            err = read;
            user_ring_buffer__discard(ringbuf, entry as *mut c_void);
            goto_done_write_samples(err);
            return err;
        }

        user_ring_buffer__submit(ringbuf, entry as *mut c_void);
        i += 1;
    }

    drain_current_samples();
    err
}

unsafe fn goto_done_write_samples(err: c_int) {
    drain_current_samples();
    let _ = err;
}

unsafe fn open_load_ringbuf_skel() -> *mut user_ringbuf_success {
    let skel = user_ringbuf_success__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return ptr::null_mut();
    }

    let mut err = bpf_map__set_max_entries((*skel).maps.user_ringbuf, c_ringbuf_size);
    if !ASSERT_OK(err, c"set_max_entries".as_ptr()) {
        user_ringbuf_success__destroy(skel);
        return ptr::null_mut();
    }

    err = bpf_map__set_max_entries((*skel).maps.kernel_ringbuf, c_ringbuf_size);
    if !ASSERT_OK(err, c"set_max_entries".as_ptr()) {
        user_ringbuf_success__destroy(skel);
        return ptr::null_mut();
    }

    err = user_ringbuf_success__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        user_ringbuf_success__destroy(skel);
        return ptr::null_mut();
    }

    skel
}

unsafe fn test_user_ringbuf_mappings() {
    let page_size = getpagesize();
    let skel = open_load_ringbuf_skel();
    if skel.is_null() {
        return;
    }

    let rb_fd = bpf_map__fd((*skel).maps.user_ringbuf);
    /* cons_pos can be mapped R/O, can't add +X with mprotect. */
    let mut mmap_ptr = mmap(ptr::null_mut(), page_size as size_t, PROT_READ, MAP_SHARED, rb_fd, 0);
    ASSERT_OK_PTR(mmap_ptr, c"ro_cons_pos".as_ptr());
    ASSERT_ERR(mprotect(mmap_ptr, page_size as size_t, PROT_WRITE), c"write_cons_pos_protect".as_ptr());
    ASSERT_ERR(mprotect(mmap_ptr, page_size as size_t, PROT_EXEC), c"exec_cons_pos_protect".as_ptr());
    ASSERT_ERR_PTR(mremap(mmap_ptr, 0, (4 * page_size) as size_t, MREMAP_MAYMOVE), c"wr_prod_pos".as_ptr());
    let mut err = -errno;
    ASSERT_ERR(err, c"wr_prod_pos_err".as_ptr());
    ASSERT_OK(munmap(mmap_ptr, page_size as size_t), c"unmap_ro_cons".as_ptr());

    /* prod_pos can be mapped RW, can't add +X with mprotect. */
    mmap_ptr = mmap(
        ptr::null_mut(),
        page_size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        rb_fd,
        page_size as c_long,
    );
    ASSERT_OK_PTR(mmap_ptr, c"rw_prod_pos".as_ptr());
    ASSERT_ERR(mprotect(mmap_ptr, page_size as size_t, PROT_EXEC), c"exec_prod_pos_protect".as_ptr());
    err = -errno;
    ASSERT_ERR(err, c"wr_prod_pos_err".as_ptr());
    ASSERT_OK(munmap(mmap_ptr, page_size as size_t), c"unmap_rw_prod".as_ptr());

    /* data pages can be mapped RW, can't add +X with mprotect. */
    mmap_ptr = mmap(
        ptr::null_mut(),
        page_size as size_t,
        PROT_WRITE,
        MAP_SHARED,
        rb_fd,
        (2 * page_size) as c_long,
    );
    ASSERT_OK_PTR(mmap_ptr, c"rw_data".as_ptr());
    ASSERT_ERR(mprotect(mmap_ptr, page_size as size_t, PROT_EXEC), c"exec_data_protect".as_ptr());
    err = -errno;
    ASSERT_ERR(err, c"exec_data_err".as_ptr());
    ASSERT_OK(munmap(mmap_ptr, page_size as size_t), c"unmap_rw_data".as_ptr());

    user_ringbuf_success__destroy(skel);
}

unsafe fn load_skel_create_ringbufs(
    skel_out: *mut *mut user_ringbuf_success,
    kern_ringbuf_out: *mut *mut ring_buffer,
    callback: ring_buffer_sample_fn,
    user_ringbuf_out: *mut *mut user_ring_buffer,
) -> c_int {
    let mut kern_ringbuf: *mut ring_buffer = ptr::null_mut();
    let mut user_ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let err: c_int = -ENOMEM;

    let skel = open_load_ringbuf_skel();
    if skel.is_null() {
        return err;
    }

    /* only trigger BPF program for current process */
    (*(*skel).bss).pid = getpid();

    if !kern_ringbuf_out.is_null() {
        let rb_fd = bpf_map__fd((*skel).maps.kernel_ringbuf);
        kern_ringbuf = ring_buffer__new(rb_fd, callback, skel as *mut c_void, ptr::null_mut());
        if !ASSERT_OK_PTR(kern_ringbuf as *const c_void, c"kern_ringbuf_create".as_ptr()) {
            goto_cleanup_load(skel, kern_ringbuf_out, user_ringbuf_out, kern_ringbuf, user_ringbuf);
            return err;
        }

        *kern_ringbuf_out = kern_ringbuf;
    }

    if !user_ringbuf_out.is_null() {
        let rb_fd = bpf_map__fd((*skel).maps.user_ringbuf);
        user_ringbuf = user_ring_buffer__new(rb_fd, ptr::null_mut());
        if !ASSERT_OK_PTR(user_ringbuf as *const c_void, c"user_ringbuf_create".as_ptr()) {
            goto_cleanup_load(skel, kern_ringbuf_out, user_ringbuf_out, kern_ringbuf, user_ringbuf);
            return err;
        }

        *user_ringbuf_out = user_ringbuf;
        ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"no_reads_after_load".as_ptr());
    }

    let attach_err = user_ringbuf_success__attach(skel);
    if !ASSERT_OK(attach_err, c"skel_attach".as_ptr()) {
        goto_cleanup_load(skel, kern_ringbuf_out, user_ringbuf_out, kern_ringbuf, user_ringbuf);
        return err;
    }

    *skel_out = skel;
    0
}

unsafe fn goto_cleanup_load(
    skel: *mut user_ringbuf_success,
    kern_ringbuf_out: *mut *mut ring_buffer,
    user_ringbuf_out: *mut *mut user_ring_buffer,
    kern_ringbuf: *mut ring_buffer,
    user_ringbuf: *mut user_ring_buffer,
) {
    if !kern_ringbuf_out.is_null() {
        *kern_ringbuf_out = ptr::null_mut();
    }
    if !user_ringbuf_out.is_null() {
        *user_ringbuf_out = ptr::null_mut();
    }
    ring_buffer__free(kern_ringbuf);
    user_ring_buffer__free(user_ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn load_skel_create_user_ringbuf(
    skel_out: *mut *mut user_ringbuf_success,
    ringbuf_out: *mut *mut user_ring_buffer,
) -> c_int {
    load_skel_create_ringbufs(skel_out, ptr::null_mut(), None, ringbuf_out)
}

unsafe fn manually_write_test_invalid_sample(
    skel: *mut user_ringbuf_success,
    size: __u32,
    producer_pos: __u64,
    err: c_int,
) {
    let page_size = getpagesize();
    let rb_fd = bpf_map__fd((*skel).maps.user_ringbuf);

    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_before_bad_sample".as_ptr());

    /* Map the producer_pos as RW. */
    let producer_pos_ptr = mmap(
        ptr::null_mut(),
        page_size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        rb_fd,
        page_size as c_long,
    ) as *mut __u64;
    ASSERT_OK_PTR(producer_pos_ptr as *const c_void, c"producer_pos_ptr".as_ptr());

    /* Map the data pages as RW. */
    let data_ptr = mmap(
        ptr::null_mut(),
        page_size as size_t,
        PROT_WRITE,
        MAP_SHARED,
        rb_fd,
        (2 * page_size) as c_long,
    );
    ASSERT_OK_PTR(data_ptr, c"rw_data".as_ptr());

    memset(data_ptr, 0, BPF_RINGBUF_HDR_SZ as size_t);
    *(data_ptr as *mut __u32) = size;

    /* Synchronizes with smp_load_acquire() in __bpf_user_ringbuf_peek() in the kernel. */
    smp_store_release(producer_pos_ptr, producer_pos + BPF_RINGBUF_HDR_SZ as __u64);

    drain_current_samples();
    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_after_bad_sample".as_ptr());
    ASSERT_OK((*(*skel).bss).err - err, c"err_after_bad_sample".as_ptr());

    ASSERT_OK(munmap(producer_pos_ptr as *mut c_void, page_size as size_t), c"unmap_producer_pos".as_ptr());
    ASSERT_OK(munmap(data_ptr, page_size as size_t), c"unmap_data_ptr".as_ptr());
}

unsafe fn test_user_ringbuf_post_misaligned() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let size: __u32 = (1 << 5) + 7;

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"misaligned_skel".as_ptr()) {
        return;
    }

    manually_write_test_invalid_sample(skel, size, size as __u64, -EINVAL);
    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_post_producer_wrong_offset() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let size: __u32 = 1 << 5;

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"wrong_offset_skel".as_ptr()) {
        return;
    }

    manually_write_test_invalid_sample(skel, size, (size - 8) as __u64, -EINVAL);
    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_post_larger_than_ringbuf_sz() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let size: __u32 = c_ringbuf_size as __u32;

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"huge_sample_skel".as_ptr()) {
        return;
    }

    manually_write_test_invalid_sample(skel, size, size as __u64, -E2BIG);
    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_basic() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"ringbuf_basic_skel".as_ptr()) {
        return;
    }

    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_read_before".as_ptr());

    let err = write_samples(ringbuf, 2);
    if !ASSERT_OK(err, c"write_samples".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    ASSERT_EQ_LONG((*(*skel).bss).read, 2, c"num_samples_read_after".as_ptr());

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_sample_full_ring_buffer() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"ringbuf_full_sample_skel".as_ptr()) {
        return;
    }

    let sample = user_ring_buffer__reserve(ringbuf, (c_ringbuf_size - BPF_RINGBUF_HDR_SZ) as size_t);
    if !ASSERT_OK_PTR(sample, c"full_sample".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    user_ring_buffer__submit(ringbuf, sample);
    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_read_before".as_ptr());
    drain_current_samples();
    ASSERT_EQ_LONG((*(*skel).bss).read, 1, c"num_samples_read_after".as_ptr());

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_post_alignment_autoadjust() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if !ASSERT_OK(err, c"ringbuf_align_autoadjust_skel".as_ptr()) {
        return;
    }

    /* libbpf should automatically round any sample up to an 8-byte alignment. */
    let sample = user_ring_buffer__reserve(ringbuf, size_of::<sample>() + 1) as *mut sample;
    ASSERT_OK_PTR(sample as *const c_void, c"reserve_autoaligned".as_ptr());
    user_ring_buffer__submit(ringbuf, sample as *mut c_void);

    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_read_before".as_ptr());
    drain_current_samples();
    ASSERT_EQ_LONG((*(*skel).bss).read, 1, c"num_samples_read_after".as_ptr());

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_overfill() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if err != 0 {
        return;
    }

    let err = write_samples(ringbuf, (c_max_entries * 5) as u32);
    ASSERT_ERR(err, c"write_samples".as_ptr());
    ASSERT_EQ_LONG((*(*skel).bss).read, c_max_entries, c"max_entries".as_ptr());

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_discards_properly_ignored() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let mut num_discarded: c_int = 0;

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if err != 0 {
        return;
    }

    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_read_before".as_ptr());

    loop {
        /* Write samples until the buffer is full. */
        let token = user_ring_buffer__reserve(ringbuf, size_of::<__u64>()) as *mut __u64;
        if token.is_null() {
            break;
        }

        user_ring_buffer__discard(ringbuf, token as *mut c_void);
        num_discarded += 1;
    }

    if !ASSERT_GE_INT(num_discarded, 0, c"num_discarded".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    /* Should not read any samples, as they are all discarded. */
    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_pre_kick".as_ptr());
    drain_current_samples();
    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_post_kick".as_ptr());

    /* Now that the ring buffer has been drained, we should be able to
     * reserve another token.
     */
    let token = user_ring_buffer__reserve(ringbuf, size_of::<__u64>()) as *mut __u64;

    if ASSERT_OK_PTR(token as *const c_void, c"new_token".as_ptr()) {
        user_ring_buffer__discard(ringbuf, token as *mut c_void);
    }

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn test_user_ringbuf_loop() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let total_samples: u32 = 8192;
    let mut remaining_samples: u32 = total_samples;

    if !ASSERT_LT_LONG(c_max_entries, total_samples as c_long, c"compare_c_max_entries".as_ptr()) {
        return;
    }

    let mut err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if err != 0 {
        return;
    }

    loop {
        let curr_samples = if (remaining_samples as c_long) > c_max_entries {
            c_max_entries as u32
        } else {
            remaining_samples
        };
        err = write_samples(ringbuf, curr_samples);
        if err != 0 {
            /* Assert inside of if statement to avoid flooding logs
             * on the success path.
             */
            ASSERT_OK(err, c"write_samples".as_ptr());
            user_ring_buffer__free(ringbuf);
            user_ringbuf_success__destroy(skel);
            return;
        }

        remaining_samples -= curr_samples;
        ASSERT_EQ_LONG(
            (*(*skel).bss).read,
            (total_samples - remaining_samples) as c_long,
            c"current_batched_entries".as_ptr(),
        );
        if remaining_samples == 0 {
            break;
        }
    }
    ASSERT_EQ_LONG((*(*skel).bss).read, total_samples as c_long, c"total_batched_entries".as_ptr());

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe fn send_test_message(
    ringbuf: *mut user_ring_buffer,
    op: test_msg_op,
    operand_64: s64,
    operand_32: s32,
) -> c_int {
    let msg = user_ring_buffer__reserve(ringbuf, size_of::<test_msg>()) as *mut test_msg;
    if msg.is_null() {
        /* Assert on the error path to avoid spamming logs with mostly
         * success messages.
         */
        ASSERT_OK_PTR(msg as *const c_void, c"reserve_msg".as_ptr());
        return -ENOMEM;
    }

    (*msg).msg_op = op;

    match op {
        test_msg_op::TEST_MSG_OP_INC64 | test_msg_op::TEST_MSG_OP_MUL64 => {
            (*msg).operand_64 = operand_64;
        }
        test_msg_op::TEST_MSG_OP_INC32 | test_msg_op::TEST_MSG_OP_MUL32 => {
            (*msg).operand_32 = operand_32;
        }
        _ => {
            PRINT_FAIL(c"Invalid operand %d\n".as_ptr(), op as c_int);
            user_ring_buffer__discard(ringbuf, msg as *mut c_void);
            return -EINVAL;
        }
    }

    user_ring_buffer__submit(ringbuf, msg as *mut c_void);

    0
}

unsafe fn kick_kernel_read_messages() {
    syscall(__NR_prctl);
}

unsafe extern "C" fn handle_kernel_msg(ctx: *mut c_void, data: *mut c_void, _len: size_t) -> c_int {
    let skel = ctx as *mut user_ringbuf_success;
    let msg = data as *mut test_msg;

    match (*msg).msg_op {
        test_msg_op::TEST_MSG_OP_INC64 => {
            (*(*skel).bss).user_mutated = (*(*skel).bss).user_mutated.wrapping_add((*msg).operand_64 as __u64);
            0
        }
        test_msg_op::TEST_MSG_OP_INC32 => {
            (*(*skel).bss).user_mutated = (*(*skel).bss).user_mutated.wrapping_add((*msg).operand_32 as __u64);
            0
        }
        test_msg_op::TEST_MSG_OP_MUL64 => {
            (*(*skel).bss).user_mutated = (*(*skel).bss).user_mutated.wrapping_mul((*msg).operand_64 as __u64);
            0
        }
        test_msg_op::TEST_MSG_OP_MUL32 => {
            (*(*skel).bss).user_mutated = (*(*skel).bss).user_mutated.wrapping_mul((*msg).operand_32 as __u64);
            0
        }
        _ => {
            fprintf(stderr, c"Invalid operand %d\n".as_ptr(), (*msg).msg_op as c_int);
            -EINVAL
        }
    }
}

unsafe fn drain_kernel_messages_buffer(kern_ringbuf: *mut ring_buffer, skel: *mut user_ringbuf_success) {
    let cnt = ring_buffer__consume(kern_ringbuf);
    ASSERT_EQ_LONG(cnt as c_long, 8, c"consume_kern_ringbuf".as_ptr());
    ASSERT_OK((*(*skel).bss).err, c"consume_kern_ringbuf_err".as_ptr());
}

unsafe fn test_user_ringbuf_msg_protocol() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut user_ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let mut kern_ringbuf: *mut ring_buffer = ptr::null_mut();
    let mut expected_kern: __u64 = 0;

    let mut err = load_skel_create_ringbufs(
        &mut skel,
        &mut kern_ringbuf,
        Some(handle_kernel_msg),
        &mut user_ringbuf,
    );
    if !ASSERT_OK(err, c"create_ringbufs".as_ptr()) {
        return;
    }

    let mut i: c_int = 0;
    while i < 64 {
        let op = core::mem::transmute::<c_int, test_msg_op>(i % test_msg_op::TEST_MSG_OP_NUM_OPS as c_int);
        let operand_64: __u64 = TEST_OP_64;
        let operand_32: __u32 = TEST_OP_32;

        err = send_test_message(user_ringbuf, op, operand_64 as s64, operand_32 as s32);
        if err != 0 {
            /* Only assert on a failure to avoid spamming success logs. */
            ASSERT_OK(err, c"send_test_message".as_ptr());
            ring_buffer__free(kern_ringbuf);
            user_ring_buffer__free(user_ringbuf);
            user_ringbuf_success__destroy(skel);
            return;
        }

        match op {
            test_msg_op::TEST_MSG_OP_INC64 => expected_kern = expected_kern.wrapping_add(operand_64),
            test_msg_op::TEST_MSG_OP_INC32 => expected_kern = expected_kern.wrapping_add(operand_32 as __u64),
            test_msg_op::TEST_MSG_OP_MUL64 => expected_kern = expected_kern.wrapping_mul(operand_64),
            test_msg_op::TEST_MSG_OP_MUL32 => expected_kern = expected_kern.wrapping_mul(operand_32 as __u64),
            _ => {
                PRINT_FAIL(c"Unexpected op %d\n".as_ptr(), op as c_int);
                ring_buffer__free(kern_ringbuf);
                user_ring_buffer__free(user_ringbuf);
                user_ringbuf_success__destroy(skel);
                return;
            }
        }

        if i % 8 == 0 {
            kick_kernel_read_messages();
            ASSERT_EQ_U64((*(*skel).bss).kern_mutated, expected_kern, c"expected_kern".as_ptr());
            ASSERT_EQ_LONG((*(*skel).bss).err as c_long, 0, c"bpf_prog_err".as_ptr());
            drain_kernel_messages_buffer(kern_ringbuf, skel);
        }

        i += 1;
    }

    ring_buffer__free(kern_ringbuf);
    user_ring_buffer__free(user_ringbuf);
    user_ringbuf_success__destroy(skel);
}

unsafe extern "C" fn kick_kernel_cb(_arg: *mut c_void) -> *mut c_void {
    /* Kick the kernel, causing it to drain the ring buffer and then wake
     * up the test thread waiting on epoll.
     */
    syscall(__NR_prlimit64);

    ptr::null_mut()
}

unsafe fn spawn_kick_thread_for_poll() -> c_int {
    let mut thread: pthread_t = 0;

    pthread_create(&mut thread, ptr::null(), Some(kick_kernel_cb), ptr::null_mut())
}

unsafe fn test_user_ringbuf_blocking_reserve() {
    let mut skel: *mut user_ringbuf_success = ptr::null_mut();
    let mut ringbuf: *mut user_ring_buffer = ptr::null_mut();
    let mut num_written: c_int = 0;

    let err = load_skel_create_user_ringbuf(&mut skel, &mut ringbuf);
    if err != 0 {
        return;
    }

    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_samples_read_before".as_ptr());

    loop {
        /* Write samples until the buffer is full. */
        let token = user_ring_buffer__reserve(ringbuf, size_of::<__u64>()) as *mut __u64;
        if token.is_null() {
            break;
        }

        *token = 0xdeadbeef;

        user_ring_buffer__submit(ringbuf, token as *mut c_void);
        num_written += 1;
    }

    if !ASSERT_GE_INT(num_written, 0, c"num_written".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    /* Should not have read any samples until the kernel is kicked. */
    ASSERT_EQ_LONG((*(*skel).bss).read, 0, c"num_pre_kick".as_ptr());

    /* We correctly time out after 1 second, without a sample. */
    let mut token = user_ring_buffer__reserve_blocking(ringbuf, size_of::<__u64>(), 1000) as *mut __u64;
    if !ASSERT_EQ_PTR(token as *const c_void, ptr::null(), c"pre_kick_timeout_token".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    let err = spawn_kick_thread_for_poll();
    if !ASSERT_EQ_LONG(err as c_long, 0, c"deferred_kick_thread\n".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    /* After spawning another thread that asynchronously kicks the kernel to
     * drain the messages, we're able to block and successfully get a
     * sample once we receive an event notification.
     */
    token = user_ring_buffer__reserve_blocking(ringbuf, size_of::<__u64>(), 10000) as *mut __u64;

    if !ASSERT_OK_PTR(token as *const c_void, c"block_token".as_ptr()) {
        user_ring_buffer__free(ringbuf);
        user_ringbuf_success__destroy(skel);
        return;
    }

    ASSERT_GT_LONG((*(*skel).bss).read, 0, c"num_post_kill".as_ptr());
    ASSERT_LE_LONG((*(*skel).bss).read, num_written as c_long, c"num_post_kill".as_ptr());
    ASSERT_EQ_LONG((*(*skel).bss).err as c_long, 0, c"err_post_poll".as_ptr());
    user_ring_buffer__discard(ringbuf, token as *mut c_void);

    user_ring_buffer__free(ringbuf);
    user_ringbuf_success__destroy(skel);
}

#[repr(C)]
struct success_test {
    test_callback: unsafe fn(),
    test_name: *const c_char,
}

static success_tests: [success_test; 12] = [
    success_test { test_callback: test_user_ringbuf_mappings, test_name: c"test_user_ringbuf_mappings".as_ptr() },
    success_test { test_callback: test_user_ringbuf_post_misaligned, test_name: c"test_user_ringbuf_post_misaligned".as_ptr() },
    success_test { test_callback: test_user_ringbuf_post_producer_wrong_offset, test_name: c"test_user_ringbuf_post_producer_wrong_offset".as_ptr() },
    success_test { test_callback: test_user_ringbuf_post_larger_than_ringbuf_sz, test_name: c"test_user_ringbuf_post_larger_than_ringbuf_sz".as_ptr() },
    success_test { test_callback: test_user_ringbuf_basic, test_name: c"test_user_ringbuf_basic".as_ptr() },
    success_test { test_callback: test_user_ringbuf_sample_full_ring_buffer, test_name: c"test_user_ringbuf_sample_full_ring_buffer".as_ptr() },
    success_test { test_callback: test_user_ringbuf_post_alignment_autoadjust, test_name: c"test_user_ringbuf_post_alignment_autoadjust".as_ptr() },
    success_test { test_callback: test_user_ringbuf_overfill, test_name: c"test_user_ringbuf_overfill".as_ptr() },
    success_test { test_callback: test_user_ringbuf_discards_properly_ignored, test_name: c"test_user_ringbuf_discards_properly_ignored".as_ptr() },
    success_test { test_callback: test_user_ringbuf_loop, test_name: c"test_user_ringbuf_loop".as_ptr() },
    success_test { test_callback: test_user_ringbuf_msg_protocol, test_name: c"test_user_ringbuf_msg_protocol".as_ptr() },
    success_test { test_callback: test_user_ringbuf_blocking_reserve, test_name: c"test_user_ringbuf_blocking_reserve".as_ptr() },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_user_ringbuf() {
    c_ringbuf_size = getpagesize() as c_long; /* 1 page */
    c_max_entries = c_ringbuf_size / C_SAMPLE_SIZE;

    let mut i: usize = 0;
    while i < success_tests.len() {
        if !test__start_subtest(success_tests[i].test_name) {
            i += 1;
            continue;
        }

        (success_tests[i].test_callback)();
        i += 1;
    }

    RUN_TESTS_user_ringbuf_fail();
}
