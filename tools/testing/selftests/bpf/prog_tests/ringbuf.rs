// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/ringbuf.c.
// C includes are external dependencies supplied by the surrounding selftest tree:
// linux/compiler.h, asm/barrier.h, test_progs.h, sys/mman.h, sys/epoll.h,
// time.h, sched.h, signal.h, pthread.h, sys/sysinfo.h, linux/perf_event.h,
// linux/ring_buffer.h, and the generated *.lskel.h skeletons.

use core::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

const EDONE: c_int = 7777;

static mut duration: c_int = 0;

#[repr(C)]
struct sample {
    pid: c_int,
    seq: c_int,
    value: c_long,
    comm: [i8; 16],
}

static sample_cnt: AtomicI32 = AtomicI32::new(0);

unsafe fn atomic_inc(cnt: *mut c_int) {
    (*(cnt as *mut AtomicI32)).fetch_add(1, Ordering::SeqCst);
}

unsafe fn atomic_xchg(cnt: *mut c_int, val: c_int) -> c_int {
    (*(cnt as *mut AtomicI32)).swap(val, Ordering::SeqCst)
}

unsafe extern "C" {
    static mut errno: c_int;

    fn getpagesize() -> c_int;
    fn getpid() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: usize,
        new_size: usize,
        flags: c_int,
    ) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_tryjoin_np(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> c_int,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__free(rb: *mut ring_buffer);
    fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int;
    fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;
    fn ring_buffer__consume_n(rb: *mut ring_buffer, n: c_ulong) -> c_int;
    fn ring_buffer__ring(rb: *mut ring_buffer, idx: c_int) -> *mut ring;
    fn ring__map_fd(r: *mut ring) -> c_int;
    fn ring__avail_data_size(r: *mut ring) -> c_ulong;
    fn ring__size(r: *mut ring) -> c_ulong;
    fn ring__consumer_pos(r: *mut ring) -> c_ulong;
    fn ring__producer_pos(r: *mut ring) -> c_ulong;
    fn ring__consume(r: *mut ring) -> c_int;

    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn test__start_subtest(name: *const i8) -> bool;

    fn test_ringbuf_lskel__open() -> *mut test_ringbuf_lskel;
    fn test_ringbuf_lskel__load(skel: *mut test_ringbuf_lskel) -> c_int;
    fn test_ringbuf_lskel__attach(skel: *mut test_ringbuf_lskel) -> c_int;
    fn test_ringbuf_lskel__detach(skel: *mut test_ringbuf_lskel);
    fn test_ringbuf_lskel__destroy(skel: *mut test_ringbuf_lskel);

    fn test_ringbuf_n_lskel__open() -> *mut test_ringbuf_n_lskel;
    fn test_ringbuf_n_lskel__load(skel: *mut test_ringbuf_n_lskel) -> c_int;
    fn test_ringbuf_n_lskel__attach(skel: *mut test_ringbuf_n_lskel) -> c_int;
    fn test_ringbuf_n_lskel__destroy(skel: *mut test_ringbuf_n_lskel);

    fn test_ringbuf_map_key_lskel__open() -> *mut test_ringbuf_map_key_lskel;
    fn test_ringbuf_map_key_lskel__load(skel: *mut test_ringbuf_map_key_lskel) -> c_int;
    fn test_ringbuf_map_key_lskel__attach(skel: *mut test_ringbuf_map_key_lskel) -> c_int;
    fn test_ringbuf_map_key_lskel__destroy(skel: *mut test_ringbuf_map_key_lskel);

    fn test_ringbuf_write_lskel__open() -> *mut test_ringbuf_write_lskel;
    fn test_ringbuf_write_lskel__load(skel: *mut test_ringbuf_write_lskel) -> c_int;
    fn test_ringbuf_write_lskel__attach(skel: *mut test_ringbuf_write_lskel) -> c_int;
    fn test_ringbuf_write_lskel__detach(skel: *mut test_ringbuf_write_lskel);
    fn test_ringbuf_write_lskel__destroy(skel: *mut test_ringbuf_write_lskel);

    fn test_ringbuf_overwrite_lskel__open() -> *mut test_ringbuf_overwrite_lskel;
    fn test_ringbuf_overwrite_lskel__load(skel: *mut test_ringbuf_overwrite_lskel) -> c_int;
    fn test_ringbuf_overwrite_lskel__attach(skel: *mut test_ringbuf_overwrite_lskel) -> c_int;
    fn test_ringbuf_overwrite_lskel__detach(skel: *mut test_ringbuf_overwrite_lskel);
    fn test_ringbuf_overwrite_lskel__destroy(skel: *mut test_ringbuf_overwrite_lskel);
}

type c_uint = u32;
type pthread_t = c_ulong;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_SHARED: c_int = 0x01;
const MREMAP_MAYMOVE: c_int = 1;
const EPERM: c_int = 1;
const EBUSY: c_int = 16;
const __NR_getpgid: c_long = 121;
const BPF_RINGBUF_HDR_SZ: usize = 8;
const BPF_RB_NO_WAKEUP: c_ulong = 1;
const BPF_RB_FORCE_WAKEUP: c_ulong = 2;

#[repr(C)]
struct ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct ring {
    _private: [u8; 0],
}

#[repr(C)]
struct map_desc {
    map_fd: c_int,
    max_entries: c_ulong,
}

#[repr(C)]
struct test_ringbuf_maps {
    ringbuf: map_desc,
}

#[repr(C)]
struct test_ringbuf_bss {
    dropped: c_long,
    total: c_long,
    discarded: c_long,
    value: c_long,
    pid: c_int,
    flags: c_ulong,
    avail_data: c_ulong,
    ring_size: c_ulong,
    cons_pos: c_ulong,
    prod_pos: c_ulong,
}

#[repr(C)]
struct test_ringbuf_lskel {
    maps: test_ringbuf_maps,
    bss: *mut test_ringbuf_bss,
}

#[repr(C)]
struct test_ringbuf_n_bss {
    pid: c_int,
    value: c_long,
}

#[repr(C)]
struct test_ringbuf_n_lskel {
    maps: test_ringbuf_maps,
    bss: *mut test_ringbuf_n_bss,
}

#[repr(C)]
struct test_ringbuf_map_key_maps {
    ringbuf: map_desc,
    hash_map: map_desc,
}

#[repr(C)]
struct test_ringbuf_map_key_bss {
    pid: c_int,
    seq: c_int,
}

#[repr(C)]
struct test_ringbuf_map_key_lskel {
    maps: test_ringbuf_map_key_maps,
    bss: *mut test_ringbuf_map_key_bss,
}

#[repr(C)]
struct test_ringbuf_write_bss {
    pid: c_int,
    discarded: c_long,
    passed: c_long,
}

#[repr(C)]
struct test_ringbuf_write_lskel {
    maps: test_ringbuf_maps,
    bss: *mut test_ringbuf_write_bss,
}

#[repr(C)]
struct test_ringbuf_overwrite_bss {
    pid: c_int,
    reserve1_fail: c_int,
    reserve2_fail: c_int,
    reserve3_fail: c_int,
    reserve4_fail: c_int,
    reserve5_fail: c_int,
    ring_size: c_ulong,
    avail_data: c_ulong,
    cons_pos: c_ulong,
    prod_pos: c_ulong,
    over_pos: c_ulong,
}

#[repr(C)]
struct test_ringbuf_overwrite_rodata {
    LEN1: c_ulong,
    LEN2: c_ulong,
    LEN3: c_ulong,
    LEN4: c_ulong,
    LEN5: c_ulong,
}

#[repr(C)]
struct test_ringbuf_overwrite_lskel {
    maps: test_ringbuf_maps,
    bss: *mut test_ringbuf_overwrite_bss,
    rodata: *mut test_ringbuf_overwrite_rodata,
}

static mut skel_map_key: *mut test_ringbuf_map_key_lskel = ptr::null_mut();
static mut skel: *mut test_ringbuf_lskel = ptr::null_mut();
static mut ringbuf: *mut ring_buffer = ptr::null_mut();

unsafe extern "C" fn process_sample(_ctx: *mut c_void, data: *mut c_void, _len: usize) -> c_int {
    let s = data as *mut sample;

    atomic_inc(&sample_cnt as *const AtomicI32 as *mut c_int);

    match (*s).seq {
        0 => {
            CHECK!(
                (*s).value != 333,
                "sample1_value\0".as_ptr() as *const i8,
                "exp %ld, got %ld\n\0".as_ptr() as *const i8,
                333 as c_long,
                (*s).value
            );
            0
        }
        1 => {
            CHECK!(
                (*s).value != 777,
                "sample2_value\0".as_ptr() as *const i8,
                "exp %ld, got %ld\n\0".as_ptr() as *const i8,
                777 as c_long,
                (*s).value
            );
            -EDONE
        }
        _ => {
            /* we don't care about the rest */
            0
        }
    }
}

unsafe fn trigger_samples() {
    (*(*skel).bss).dropped = 0;
    (*(*skel).bss).total = 0;
    (*(*skel).bss).discarded = 0;

    /* trigger exactly two samples */
    (*(*skel).bss).value = 333;
    syscall(__NR_getpgid);
    (*(*skel).bss).value = 777;
    syscall(__NR_getpgid);
}

unsafe extern "C" fn poll_thread(input: *mut c_void) -> *mut c_void {
    let timeout = input as c_long;

    ring_buffer__poll(ringbuf, timeout as c_int) as c_long as *mut c_void
}

unsafe fn ringbuf_write_subtest() {
    let mut skel: *mut test_ringbuf_write_lskel;
    let page_size: c_int = getpagesize();
    let mut mmap_ptr: *mut usize;
    let mut err: c_int;
    let rb_fd: c_int;

    skel = test_ringbuf_write_lskel__open();
    if !ASSERT_OK_PTR!(skel, "skel_open\0".as_ptr() as *const i8) {
        return;
    }

    (*skel).maps.ringbuf.max_entries = 0x40000;

    err = test_ringbuf_write_lskel__load(skel);
    if !ASSERT_OK!(err, "skel_load\0".as_ptr() as *const i8) {
        test_ringbuf_write_lskel__destroy(skel);
        return;
    }

    rb_fd = (*skel).maps.ringbuf.map_fd;

    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_READ | PROT_WRITE, MAP_SHARED, rb_fd, 0) as *mut usize;
    if !ASSERT_OK_PTR!(mmap_ptr, "rw_cons_pos\0".as_ptr() as *const i8) {
        test_ringbuf_write_lskel__destroy(skel);
        return;
    }
    *mmap_ptr = 0x30000;
    ASSERT_OK!(munmap(mmap_ptr as *mut c_void, page_size as usize), "unmap_rw\0".as_ptr() as *const i8);

    (*(*skel).bss).pid = getpid();

    ringbuf = ring_buffer__new(rb_fd, process_sample, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_OK_PTR!(ringbuf, "ringbuf_new\0".as_ptr() as *const i8) {
        test_ringbuf_write_lskel__destroy(skel);
        return;
    }

    err = test_ringbuf_write_lskel__attach(skel);
    if !ASSERT_OK!(err, "skel_attach\0".as_ptr() as *const i8) {
        ring_buffer__free(ringbuf);
        test_ringbuf_write_lskel__destroy(skel);
        return;
    }

    (*(*skel).bss).discarded = 0;
    (*(*skel).bss).passed = 0;

    /* trigger exactly two samples */
    syscall(__NR_getpgid);
    syscall(__NR_getpgid);

    ASSERT_EQ!((*(*skel).bss).discarded, 2, "discarded\0".as_ptr() as *const i8);
    ASSERT_EQ!((*(*skel).bss).passed, 0, "passed\0".as_ptr() as *const i8);

    test_ringbuf_write_lskel__detach(skel);
    ring_buffer__free(ringbuf);
    test_ringbuf_write_lskel__destroy(skel);
}

unsafe fn ringbuf_subtest() {
    let rec_sz: usize = BPF_RINGBUF_HDR_SZ + size_of::<sample>();
    let mut thread: pthread_t = 0;
    let mut bg_ret: c_long = -1;
    let mut err: c_int;
    let mut cnt: c_int;
    let rb_fd: c_int;
    let page_size: c_int = getpagesize();
    let mut mmap_ptr: *mut c_void;
    let mut tmp_ptr: *mut c_void;
    let ring: *mut ring;
    let map_fd: c_int;
    let mut avail_data: c_ulong;
    let mut ring_size: c_ulong;
    let mut cons_pos: c_ulong;
    let mut prod_pos: c_ulong;

    skel = test_ringbuf_lskel__open();
    if CHECK!(!skel.is_null(), "skel_open\0".as_ptr() as *const i8, "skeleton open failed\n\0".as_ptr() as *const i8) {
        return;
    }

    (*skel).maps.ringbuf.max_entries = page_size as c_ulong;

    err = test_ringbuf_lskel__load(skel);
    if CHECK!(err != 0, "skel_load\0".as_ptr() as *const i8, "skeleton load failed\n\0".as_ptr() as *const i8) {
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    rb_fd = (*skel).maps.ringbuf.map_fd;
    /* good read/write cons_pos */
    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_READ | PROT_WRITE, MAP_SHARED, rb_fd, 0);
    ASSERT_OK_PTR!(mmap_ptr, "rw_cons_pos\0".as_ptr() as *const i8);
    tmp_ptr = mremap(mmap_ptr, page_size as usize, (2 * page_size) as usize, MREMAP_MAYMOVE);
    if !ASSERT_ERR_PTR!(tmp_ptr, "rw_extend\0".as_ptr() as *const i8) {
        test_ringbuf_lskel__destroy(skel);
        return;
    }
    ASSERT_ERR!(mprotect(mmap_ptr, page_size as usize, PROT_EXEC), "exec_cons_pos_protect\0".as_ptr() as *const i8);
    ASSERT_OK!(munmap(mmap_ptr, page_size as usize), "unmap_rw\0".as_ptr() as *const i8);

    /* bad writeable prod_pos */
    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_WRITE, MAP_SHARED, rb_fd, page_size as isize);
    err = -errno;
    ASSERT_ERR_PTR!(mmap_ptr, "wr_prod_pos\0".as_ptr() as *const i8);
    ASSERT_EQ!(err, -EPERM, "wr_prod_pos_err\0".as_ptr() as *const i8);

    /* bad writeable data pages */
    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_WRITE, MAP_SHARED, rb_fd, (2 * page_size) as isize);
    err = -errno;
    ASSERT_ERR_PTR!(mmap_ptr, "wr_data_page_one\0".as_ptr() as *const i8);
    ASSERT_EQ!(err, -EPERM, "wr_data_page_one_err\0".as_ptr() as *const i8);
    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_WRITE, MAP_SHARED, rb_fd, (3 * page_size) as isize);
    ASSERT_ERR_PTR!(mmap_ptr, "wr_data_page_two\0".as_ptr() as *const i8);
    mmap_ptr = mmap(ptr::null_mut(), (2 * page_size) as usize, PROT_WRITE, MAP_SHARED, rb_fd, (2 * page_size) as isize);
    ASSERT_ERR_PTR!(mmap_ptr, "wr_data_page_all\0".as_ptr() as *const i8);

    /* good read-only pages */
    mmap_ptr = mmap(ptr::null_mut(), (4 * page_size) as usize, PROT_READ, MAP_SHARED, rb_fd, 0);
    if !ASSERT_OK_PTR!(mmap_ptr, "ro_prod_pos\0".as_ptr() as *const i8) {
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    ASSERT_ERR!(mprotect(mmap_ptr, (4 * page_size) as usize, PROT_WRITE), "write_protect\0".as_ptr() as *const i8);
    ASSERT_ERR!(mprotect(mmap_ptr, (4 * page_size) as usize, PROT_EXEC), "exec_protect\0".as_ptr() as *const i8);
    ASSERT_ERR_PTR!(mremap(mmap_ptr, 0, (4 * page_size) as usize, MREMAP_MAYMOVE), "ro_remap\0".as_ptr() as *const i8);
    ASSERT_OK!(munmap(mmap_ptr, (4 * page_size) as usize), "unmap_ro\0".as_ptr() as *const i8);

    /* good read-only pages with initial offset */
    mmap_ptr = mmap(ptr::null_mut(), page_size as usize, PROT_READ, MAP_SHARED, rb_fd, page_size as isize);
    if !ASSERT_OK_PTR!(mmap_ptr, "ro_prod_pos\0".as_ptr() as *const i8) {
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    ASSERT_ERR!(mprotect(mmap_ptr, page_size as usize, PROT_WRITE), "write_protect\0".as_ptr() as *const i8);
    ASSERT_ERR!(mprotect(mmap_ptr, page_size as usize, PROT_EXEC), "exec_protect\0".as_ptr() as *const i8);
    ASSERT_ERR_PTR!(mremap(mmap_ptr, 0, (3 * page_size) as usize, MREMAP_MAYMOVE), "ro_remap\0".as_ptr() as *const i8);
    ASSERT_OK!(munmap(mmap_ptr, page_size as usize), "unmap_ro\0".as_ptr() as *const i8);

    /* only trigger BPF program for current process */
    (*(*skel).bss).pid = getpid();

    ringbuf = ring_buffer__new((*skel).maps.ringbuf.map_fd, process_sample, ptr::null_mut(), ptr::null_mut());
    if CHECK!(ringbuf.is_null(), "ringbuf_create\0".as_ptr() as *const i8, "failed to create ringbuf\n\0".as_ptr() as *const i8) {
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    err = test_ringbuf_lskel__attach(skel);
    if CHECK!(err != 0, "skel_attach\0".as_ptr() as *const i8, "skeleton attachment failed: %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    trigger_samples();

    ring = ring_buffer__ring(ringbuf, 0);
    if !ASSERT_OK_PTR!(ring, "ring_buffer__ring_idx_0\0".as_ptr() as *const i8) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    map_fd = ring__map_fd(ring);
    ASSERT_EQ!(map_fd, (*skel).maps.ringbuf.map_fd, "ring_map_fd\0".as_ptr() as *const i8);

    /* 2 submitted + 1 discarded records */
    CHECK!((*(*skel).bss).avail_data != (3 * rec_sz) as c_ulong, "err_avail_size\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, (3 * rec_sz) as c_long, (*(*skel).bss).avail_data);
    CHECK!((*(*skel).bss).ring_size != page_size as c_ulong, "err_ring_size\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, page_size as c_long, (*(*skel).bss).ring_size);
    CHECK!((*(*skel).bss).cons_pos != 0, "err_cons_pos\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 0 as c_long, (*(*skel).bss).cons_pos);
    CHECK!((*(*skel).bss).prod_pos != (3 * rec_sz) as c_ulong, "err_prod_pos\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, (3 * rec_sz) as c_long, (*(*skel).bss).prod_pos);

    /* verify getting this data directly via the ring object yields the same
     * results
     */
    avail_data = ring__avail_data_size(ring);
    ASSERT_EQ!(avail_data, (3 * rec_sz) as c_ulong, "ring_avail_size\0".as_ptr() as *const i8);
    ring_size = ring__size(ring);
    ASSERT_EQ!(ring_size, page_size as c_ulong, "ring_ring_size\0".as_ptr() as *const i8);
    cons_pos = ring__consumer_pos(ring);
    ASSERT_EQ!(cons_pos, 0, "ring_cons_pos\0".as_ptr() as *const i8);
    prod_pos = ring__producer_pos(ring);
    ASSERT_EQ!(prod_pos, (3 * rec_sz) as c_ulong, "ring_prod_pos\0".as_ptr() as *const i8);

    /* poll for samples */
    err = ring_buffer__poll(ringbuf, -1);

    /* -EDONE is used as an indicator that we are done */
    if CHECK!(err != -EDONE, "err_done\0".as_ptr() as *const i8, "done err: %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 2, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 2, cnt);

    /* we expect extra polling to return nothing */
    err = ring_buffer__poll(ringbuf, 0);
    if CHECK!(err != 0, "extra_samples\0".as_ptr() as *const i8, "poll result: %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 0, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 0, cnt);

    CHECK!((*(*skel).bss).dropped != 0, "err_dropped\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 0 as c_long, (*(*skel).bss).dropped);
    CHECK!((*(*skel).bss).total != 2, "err_total\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 2 as c_long, (*(*skel).bss).total);
    CHECK!((*(*skel).bss).discarded != 1, "err_discarded\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 1 as c_long, (*(*skel).bss).discarded);

    /* now validate consumer position is updated and returned */
    trigger_samples();
    CHECK!((*(*skel).bss).cons_pos != (3 * rec_sz) as c_ulong, "err_cons_pos\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, (3 * rec_sz) as c_long, (*(*skel).bss).cons_pos);
    err = ring_buffer__poll(ringbuf, -1);
    CHECK!(err <= 0, "poll_err\0".as_ptr() as *const i8, "err %d\n\0".as_ptr() as *const i8, err);
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 2, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 2, cnt);

    /* start poll in background w/ long timeout */
    err = pthread_create(&mut thread, ptr::null(), poll_thread, 10000 as c_long as *mut c_void);
    if CHECK!(err != 0, "bg_poll\0".as_ptr() as *const i8, "pthread_create failed: %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    /* turn off notifications now */
    (*(*skel).bss).flags = BPF_RB_NO_WAKEUP;

    /* give background thread a bit of a time */
    usleep(50000);
    trigger_samples();
    /* sleeping arbitrarily is bad, but no better way to know that
     * epoll_wait() **DID NOT** unblock in background thread
     */
    usleep(50000);
    /* background poll should still be blocked */
    err = pthread_tryjoin_np(thread, &mut bg_ret as *mut c_long as *mut *mut c_void);
    if CHECK!(err != EBUSY, "try_join\0".as_ptr() as *const i8, "err %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    /* BPF side did everything right */
    CHECK!((*(*skel).bss).dropped != 0, "err_dropped\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 0 as c_long, (*(*skel).bss).dropped);
    CHECK!((*(*skel).bss).total != 2, "err_total\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 2 as c_long, (*(*skel).bss).total);
    CHECK!((*(*skel).bss).discarded != 1, "err_discarded\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 1 as c_long, (*(*skel).bss).discarded);
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 0, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 0, cnt);

    /* clear flags to return to "adaptive" notification mode */
    (*(*skel).bss).flags = 0;

    /* produce new samples, no notification should be triggered, because
     * consumer is now behind
     */
    trigger_samples();

    /* background poll should still be blocked */
    err = pthread_tryjoin_np(thread, &mut bg_ret as *mut c_long as *mut *mut c_void);
    if CHECK!(err != EBUSY, "try_join\0".as_ptr() as *const i8, "err %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    /* still no samples, because consumer is behind */
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 0, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 0, cnt);

    (*(*skel).bss).dropped = 0;
    (*(*skel).bss).total = 0;
    (*(*skel).bss).discarded = 0;

    (*(*skel).bss).value = 333;
    syscall(__NR_getpgid);
    /* now force notifications */
    (*(*skel).bss).flags = BPF_RB_FORCE_WAKEUP;
    (*(*skel).bss).value = 777;
    syscall(__NR_getpgid);

    /* now we should get a pending notification */
    usleep(50000);
    err = pthread_tryjoin_np(thread, &mut bg_ret as *mut c_long as *mut *mut c_void);
    if CHECK!(err != 0, "join_bg\0".as_ptr() as *const i8, "err %d\n\0".as_ptr() as *const i8, err) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    if CHECK!(bg_ret <= 0, "bg_ret\0".as_ptr() as *const i8, "epoll_wait result: %ld\0".as_ptr() as *const i8, bg_ret) {
        ring_buffer__free(ringbuf);
        test_ringbuf_lskel__destroy(skel);
        return;
    }

    /* due to timing variations, there could still be non-notified
     * samples, so consume them here to collect all the samples
     */
    err = ring_buffer__consume(ringbuf);
    CHECK!(err < 0, "rb_consume\0".as_ptr() as *const i8, "failed: %d\b\0".as_ptr() as *const i8, err);

    /* also consume using ring__consume to make sure it works the same */
    err = ring__consume(ring);
    ASSERT_GE!(err, 0, "ring_consume\0".as_ptr() as *const i8);

    /* 3 rounds, 2 samples each */
    cnt = atomic_xchg(&sample_cnt as *const AtomicI32 as *mut c_int, 0);
    CHECK!(cnt != 6, "cnt\0".as_ptr() as *const i8, "exp %d samples, got %d\n\0".as_ptr() as *const i8, 6, cnt);

    /* BPF side did everything right */
    CHECK!((*(*skel).bss).dropped != 0, "err_dropped\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 0 as c_long, (*(*skel).bss).dropped);
    CHECK!((*(*skel).bss).total != 2, "err_total\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 2 as c_long, (*(*skel).bss).total);
    CHECK!((*(*skel).bss).discarded != 1, "err_discarded\0".as_ptr() as *const i8, "exp %ld, got %ld\n\0".as_ptr() as *const i8, 1 as c_long, (*(*skel).bss).discarded);

    test_ringbuf_lskel__detach(skel);
    ring_buffer__free(ringbuf);
    test_ringbuf_lskel__destroy(skel);
}

/*
 * Test ring_buffer__consume_n() by producing N_TOT_SAMPLES samples in the ring
 * buffer, via getpid(), and consuming them in chunks of N_SAMPLES.
 */
const N_TOT_SAMPLES: c_int = 32;
const N_SAMPLES: c_int = 4;

/* Sample value to verify the callback validity */
const SAMPLE_VALUE: c_long = 42;

unsafe extern "C" fn process_n_sample(_ctx: *mut c_void, data: *mut c_void, _len: usize) -> c_int {
    let s = data as *mut sample;

    ASSERT_EQ!((*s).value, SAMPLE_VALUE, "sample_value\0".as_ptr() as *const i8);

    0
}

unsafe fn ringbuf_n_subtest() {
    let skel_n: *mut test_ringbuf_n_lskel;
    let mut err: c_int;
    let mut i: c_int;

    skel_n = test_ringbuf_n_lskel__open();
    if !ASSERT_OK_PTR!(skel_n, "test_ringbuf_n_lskel__open\0".as_ptr() as *const i8) {
        return;
    }

    (*skel_n).maps.ringbuf.max_entries = getpagesize() as c_ulong;
    (*(*skel_n).bss).pid = getpid();

    err = test_ringbuf_n_lskel__load(skel_n);
    if !ASSERT_OK!(err, "test_ringbuf_n_lskel__load\0".as_ptr() as *const i8) {
        test_ringbuf_n_lskel__destroy(skel_n);
        return;
    }

    ringbuf = ring_buffer__new((*skel_n).maps.ringbuf.map_fd, process_n_sample, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_OK_PTR!(ringbuf, "ring_buffer__new\0".as_ptr() as *const i8) {
        test_ringbuf_n_lskel__destroy(skel_n);
        return;
    }

    err = test_ringbuf_n_lskel__attach(skel_n);
    if !ASSERT_OK!(err, "test_ringbuf_n_lskel__attach\0".as_ptr() as *const i8) {
        ring_buffer__free(ringbuf);
        test_ringbuf_n_lskel__destroy(skel_n);
        return;
    }

    /* Produce N_TOT_SAMPLES samples in the ring buffer by calling getpid() */
    (*(*skel_n).bss).value = SAMPLE_VALUE;
    i = 0;
    while i < N_TOT_SAMPLES {
        syscall(__NR_getpgid);
        i += 1;
    }

    /* Consume all samples from the ring buffer in batches of N_SAMPLES */
    i = 0;
    while i < N_TOT_SAMPLES {
        err = ring_buffer__consume_n(ringbuf, N_SAMPLES as c_ulong);
        if !ASSERT_EQ!(err, N_SAMPLES, "rb_consume\0".as_ptr() as *const i8) {
            ring_buffer__free(ringbuf);
            test_ringbuf_n_lskel__destroy(skel_n);
            return;
        }
        i += err;
    }

    ring_buffer__free(ringbuf);
    test_ringbuf_n_lskel__destroy(skel_n);
}

unsafe extern "C" fn process_map_key_sample(_ctx: *mut c_void, data: *mut c_void, _len: usize) -> c_int {
    let mut s: *mut sample;
    let mut err: c_int;
    let mut val: c_int = 0;

    s = data as *mut sample;
    match (*s).seq {
        1 => {
            ASSERT_EQ!((*s).value, 42, "sample_value\0".as_ptr() as *const i8);
            err = bpf_map_lookup_elem((*skel_map_key).maps.hash_map.map_fd, s as *const c_void, &mut val as *mut c_int as *mut c_void);
            ASSERT_OK!(err, "hash_map bpf_map_lookup_elem\0".as_ptr() as *const i8);
            ASSERT_EQ!(val, 1, "hash_map val\0".as_ptr() as *const i8);
            -EDONE
        }
        _ => 0,
    }
}

unsafe fn ringbuf_map_key_subtest() {
    let mut err: c_int;

    skel_map_key = test_ringbuf_map_key_lskel__open();
    if !ASSERT_OK_PTR!(skel_map_key, "test_ringbuf_map_key_lskel__open\0".as_ptr() as *const i8) {
        return;
    }

    (*skel_map_key).maps.ringbuf.max_entries = getpagesize() as c_ulong;
    (*(*skel_map_key).bss).pid = getpid();

    err = test_ringbuf_map_key_lskel__load(skel_map_key);
    if !ASSERT_OK!(err, "test_ringbuf_map_key_lskel__load\0".as_ptr() as *const i8) {
        test_ringbuf_map_key_lskel__destroy(skel_map_key);
        return;
    }

    ringbuf = ring_buffer__new((*skel_map_key).maps.ringbuf.map_fd, process_map_key_sample, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_OK_PTR!(ringbuf, "ring_buffer__new\0".as_ptr() as *const i8) {
        test_ringbuf_map_key_lskel__destroy(skel_map_key);
        return;
    }

    err = test_ringbuf_map_key_lskel__attach(skel_map_key);
    if !ASSERT_OK!(err, "test_ringbuf_map_key_lskel__attach\0".as_ptr() as *const i8) {
        ring_buffer__free(ringbuf);
        test_ringbuf_map_key_lskel__destroy(skel_map_key);
        return;
    }

    syscall(__NR_getpgid);
    ASSERT_EQ!((*(*skel_map_key).bss).seq, 1, "skel_map_key->bss->seq\0".as_ptr() as *const i8);
    err = ring_buffer__poll(ringbuf, -1);
    ASSERT_EQ!(err, -EDONE, "ring_buffer__poll\0".as_ptr() as *const i8);

    ring_buffer__free(ringbuf);
    test_ringbuf_map_key_lskel__destroy(skel_map_key);
}

unsafe fn ringbuf_overwrite_mode_subtest() {
    let mut size: c_ulong;
    let mut len1: c_ulong;
    let mut len2: c_ulong;
    let mut len3: c_ulong;
    let mut len4: c_ulong;
    let mut len5: c_ulong;
    let mut expect_avail_data: c_ulong;
    let mut expect_prod_pos: c_ulong;
    let mut expect_over_pos: c_ulong;
    let skel: *mut test_ringbuf_overwrite_lskel;
    let page_size: c_int = getpagesize();
    let mut err: c_int;

    skel = test_ringbuf_overwrite_lskel__open();
    if !ASSERT_OK_PTR!(skel, "skel_open\0".as_ptr() as *const i8) {
        return;
    }

    size = page_size as c_ulong;
    len1 = (page_size / 2) as c_ulong;
    len2 = (page_size / 4) as c_ulong;
    len3 = size - len1 - len2 - (BPF_RINGBUF_HDR_SZ * 3) as c_ulong;
    len4 = len3 - 8;
    len5 = len3; /* retry with len3 */

    (*skel).maps.ringbuf.max_entries = size;
    (*(*skel).rodata).LEN1 = len1;
    (*(*skel).rodata).LEN2 = len2;
    (*(*skel).rodata).LEN3 = len3;
    (*(*skel).rodata).LEN4 = len4;
    (*(*skel).rodata).LEN5 = len5;

    (*(*skel).bss).pid = getpid();

    err = test_ringbuf_overwrite_lskel__load(skel);
    if !ASSERT_OK!(err, "skel_load\0".as_ptr() as *const i8) {
        test_ringbuf_overwrite_lskel__destroy(skel);
        return;
    }

    err = test_ringbuf_overwrite_lskel__attach(skel);
    if !ASSERT_OK!(err, "skel_attach\0".as_ptr() as *const i8) {
        test_ringbuf_overwrite_lskel__destroy(skel);
        return;
    }

    syscall(__NR_getpgid);

    ASSERT_EQ!((*(*skel).bss).reserve1_fail, 0, "reserve 1\0".as_ptr() as *const i8);
    ASSERT_EQ!((*(*skel).bss).reserve2_fail, 0, "reserve 2\0".as_ptr() as *const i8);
    ASSERT_EQ!((*(*skel).bss).reserve3_fail, 1, "reserve 3\0".as_ptr() as *const i8);
    ASSERT_EQ!((*(*skel).bss).reserve4_fail, 0, "reserve 4\0".as_ptr() as *const i8);
    ASSERT_EQ!((*(*skel).bss).reserve5_fail, 0, "reserve 5\0".as_ptr() as *const i8);

    ASSERT_EQ!((*(*skel).bss).ring_size, size, "check_ring_size\0".as_ptr() as *const i8);

    expect_avail_data = len2 + len4 + len5 + (3 * BPF_RINGBUF_HDR_SZ) as c_ulong;
    ASSERT_EQ!((*(*skel).bss).avail_data, expect_avail_data, "check_avail_size\0".as_ptr() as *const i8);

    ASSERT_EQ!((*(*skel).bss).cons_pos, 0, "check_cons_pos\0".as_ptr() as *const i8);

    expect_prod_pos = len1 + len2 + len4 + len5 + (4 * BPF_RINGBUF_HDR_SZ) as c_ulong;
    ASSERT_EQ!((*(*skel).bss).prod_pos, expect_prod_pos, "check_prod_pos\0".as_ptr() as *const i8);

    expect_over_pos = len1 + BPF_RINGBUF_HDR_SZ as c_ulong;
    ASSERT_EQ!((*(*skel).bss).over_pos, expect_over_pos, "check_over_pos\0".as_ptr() as *const i8);

    test_ringbuf_overwrite_lskel__detach(skel);
    test_ringbuf_overwrite_lskel__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_ringbuf() {
    if test__start_subtest("ringbuf\0".as_ptr() as *const i8) {
        ringbuf_subtest();
    }
    if test__start_subtest("ringbuf_n\0".as_ptr() as *const i8) {
        ringbuf_n_subtest();
    }
    if test__start_subtest("ringbuf_map_key\0".as_ptr() as *const i8) {
        ringbuf_map_key_subtest();
    }
    if test__start_subtest("ringbuf_write\0".as_ptr() as *const i8) {
        ringbuf_write_subtest();
    }
    if test__start_subtest("ringbuf_overwrite_mode\0".as_ptr() as *const i8) {
        ringbuf_overwrite_mode_subtest();
    }
}
