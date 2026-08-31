// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

/* Translated from C. Dependencies originally came from:
 * <errno.h>, <unistd.h>, <pthread.h>, <bpf/bpf.h>, <bpf/libbpf.h>,
 * <bpf_util.h>, <test_maps.h>, and "map_percpu_stats.skel.h".
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u8 = u8;
type __u32 = u32;
type ssize_t = isize;
type pthread_t = usize;

const MAX_ENTRIES: __u32 = 16384;
const MAX_ENTRIES_HASH_OF_MAPS: __u32 = 64;
const N_THREADS: usize = 8;
const MAX_MAP_KEY_SIZE: usize = 4;
const PCPU_MIN_UNIT_SIZE: c_int = 32768;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const E2BIG: c_int = 7;

const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_PERCPU_HASH: __u32 = 5;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_MAP_TYPE_LRU_HASH: __u32 = 9;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: __u32 = 10;
const BPF_MAP_TYPE_HASH_OF_MAPS: __u32 = 12;

const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_F_NO_COMMON_LRU: __u32 = 2;

const BATCH: bool = true;

#[repr(C)]
struct bpf_map_info {
    type_: __u32,
    id: __u32,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    map_flags: __u32,
}

#[repr(C)]
struct bpf_map_create_opts {
    sz: usize,
    btf_fd: __u32,
    btf_key_type_id: __u32,
    btf_value_type_id: __u32,
    btf_vmlinux_value_type_id: __u32,
    inner_map_fd: c_int,
    map_flags: __u32,
}

#[repr(C)]
struct map_percpu_stats_bss {
    target_id: c_int,
}

#[repr(C)]
struct map_percpu_stats_progs {
    dump_bpf_map: *mut bpf_program,
}

#[repr(C)]
struct map_percpu_stats {
    bss: *mut map_percpu_stats_bss,
    progs: map_percpu_stats_progs,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct upsert_opts {
    map_type: __u32,
    map_fd: c_int,
    n: __u32,
    retry_for_nomem: bool,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn rand() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *mut __u32) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_and_delete_batch(
        fd: c_int,
        in_batch: *mut *mut c_void,
        out_batch: *mut *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_and_delete_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_create(
        map_type: __u32,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn map_update_retriable(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
        attempts: c_int,
        need_retry: unsafe extern "C" fn(c_int) -> bool,
    ) -> c_int;

    fn map_percpu_stats__open() -> *mut map_percpu_stats;
    fn map_percpu_stats__load(skel: *mut map_percpu_stats) -> c_int;
    fn map_percpu_stats__destroy(skel: *mut map_percpu_stats);
    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *const c_void) -> *mut bpf_link;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_link__fd(link: *const bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...);
}

unsafe fn map_info(map_fd: c_int, info: *mut bpf_map_info) {
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let ret: c_int;

    memset(info as *mut c_void, 0, size_of::<bpf_map_info>());

    ret = bpf_obj_get_info_by_fd(map_fd, info as *mut c_void, &mut len);
    CHECK(ret < 0, c"bpf_obj_get_info_by_fd".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));
}

unsafe fn map_type_to_s(type_: __u32) -> *const c_char {
    match type_ {
        BPF_MAP_TYPE_HASH => c"HASH".as_ptr(),
        BPF_MAP_TYPE_PERCPU_HASH => c"PERCPU_HASH".as_ptr(),
        BPF_MAP_TYPE_LRU_HASH => c"LRU_HASH".as_ptr(),
        BPF_MAP_TYPE_LRU_PERCPU_HASH => c"LRU_PERCPU_HASH".as_ptr(),
        BPF_MAP_TYPE_HASH_OF_MAPS => c"BPF_MAP_TYPE_HASH_OF_MAPS".as_ptr(),
        _ => c"<define-me>".as_ptr(),
    }
}

unsafe fn map_count_elements(_type: __u32, map_fd: c_int) -> __u32 {
    let mut key: __u32 = (-1i32) as __u32;
    let mut n: c_int = 0;

    while bpf_map_get_next_key(
        map_fd,
        &key as *const __u32 as *const c_void,
        &mut key as *mut __u32 as *mut c_void,
    ) == 0
    {
        n += 1;
    }
    n as __u32
}

unsafe fn delete_and_lookup_batch(map_fd: c_int, keys: *mut c_void, mut count: __u32) {
    static mut VALUES: [__u8; (8 << 10) * MAX_ENTRIES as usize] =
        [0; (8 << 10) * MAX_ENTRIES as usize];
    let mut in_batch: *mut c_void = null_mut();
    let mut out_batch: *mut c_void = null_mut();
    let save_count: __u32 = count;
    let ret: c_int;

    ret = bpf_map_lookup_and_delete_batch(
        map_fd,
        &mut in_batch,
        &mut out_batch,
        keys,
        VALUES.as_mut_ptr() as *mut c_void,
        &mut count,
        null(),
    );

    /*
     * Despite what uapi header says, lookup_and_delete_batch will return
     * -ENOENT in case we successfully have deleted all elements, so check
     * this separately
     */
    CHECK(
        ret < 0 && (errno != ENOENT || count == 0),
        c"bpf_map_lookup_and_delete_batch".as_ptr(),
        c"error: %s\n".as_ptr(),
        strerror(errno),
    );

    CHECK(
        count != save_count,
        c"bpf_map_lookup_and_delete_batch".as_ptr(),
        c"deleted not all elements: removed=%u expected=%u\n".as_ptr(),
        count,
        save_count,
    );
}

unsafe fn delete_all_elements(type_: __u32, map_fd: c_int, batch: bool) {
    static mut VAL: [__u8; 8 << 10] = [0; 8 << 10]; /* enough for 1024 CPUs */
    let mut key: __u32 = (-1i32) as __u32;
    let keys: *mut c_void;
    let mut i: __u32;
    let mut n: __u32;
    let mut ret: c_int;

    keys = calloc(MAX_MAP_KEY_SIZE, MAX_ENTRIES as usize);
    CHECK(keys.is_null(), c"calloc".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));

    n = 0;
    while bpf_map_get_next_key(
        map_fd,
        &key as *const __u32 as *const c_void,
        &mut key as *mut __u32 as *mut c_void,
    ) == 0
    {
        memcpy(
            (keys as *mut __u8).add(n as usize * MAX_MAP_KEY_SIZE) as *mut c_void,
            &key as *const __u32 as *const c_void,
            MAX_MAP_KEY_SIZE,
        );
        n += 1;
    }

    if batch {
        /* Can't mix delete_batch and delete_and_lookup_batch because
         * they have different semantics in relation to the keys
         * argument. However, delete_batch utilize map_delete_elem,
         * so we actually test it in non-batch scenario */
        delete_and_lookup_batch(map_fd, keys, n);
    } else {
        /* Intentionally mix delete and lookup_and_delete so we can test both */
        i = 0;
        while i < n {
            let keyp = (keys as *mut __u8).add(i as usize * MAX_MAP_KEY_SIZE) as *mut c_void;

            if i % 2 != 0 || type_ == BPF_MAP_TYPE_HASH_OF_MAPS {
                ret = bpf_map_delete_elem(map_fd, keyp);
                CHECK(
                    ret < 0,
                    c"bpf_map_delete_elem".as_ptr(),
                    c"error: key %u: %s\n".as_ptr(),
                    i,
                    strerror(errno),
                );
            } else {
                ret = bpf_map_lookup_and_delete_elem(map_fd, keyp, VAL.as_mut_ptr() as *mut c_void);
                CHECK(
                    ret < 0,
                    c"bpf_map_lookup_and_delete_elem".as_ptr(),
                    c"error: key %u: %s\n".as_ptr(),
                    i,
                    strerror(errno),
                );
            }
            i += 1;
        }
    }

    free(keys);
}

unsafe fn is_lru(map_type: __u32) -> bool {
    map_type == BPF_MAP_TYPE_LRU_HASH || map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH
}

unsafe fn is_percpu(map_type: __u32) -> bool {
    map_type == BPF_MAP_TYPE_PERCPU_HASH || map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH
}

unsafe fn create_small_hash() -> c_int {
    let map_fd: c_int;

    map_fd = bpf_map_create(BPF_MAP_TYPE_HASH, c"small".as_ptr(), 4, 4, 4, null());
    CHECK(
        map_fd < 0,
        c"bpf_map_create()".as_ptr(),
        c"error:%s (name=%s)\n".as_ptr(),
        strerror(errno),
        c"small".as_ptr(),
    );

    map_fd
}

unsafe extern "C" fn retry_for_nomem_fn(err: c_int) -> bool {
    err == ENOMEM
}

unsafe extern "C" fn patch_map_thread(arg: *mut c_void) -> *mut c_void {
    /* 8KB is enough for 1024 CPUs. And it is shared between N_THREADS. */
    static mut BLOB: [__u8; 8 << 10] = [0; 8 << 10];
    let opts = arg as *mut upsert_opts;
    let mut val_ptr: *mut c_void;
    let mut val: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    i = 0;
    while i < (*opts).n as c_int {
        if (*opts).map_type == BPF_MAP_TYPE_HASH_OF_MAPS {
            val = create_small_hash();
            val_ptr = &mut val as *mut c_int as *mut c_void;
        } else if is_percpu((*opts).map_type) {
            val_ptr = BLOB.as_mut_ptr() as *mut c_void;
        } else {
            val = rand();
            val_ptr = &mut val as *mut c_int as *mut c_void;
        }

        /* 2 seconds may be enough ? */
        if (*opts).retry_for_nomem {
            ret = map_update_retriable(
                (*opts).map_fd,
                &i as *const c_int as *const c_void,
                val_ptr as *const c_void,
                0,
                40,
                retry_for_nomem_fn,
            );
        } else {
            ret = bpf_map_update_elem(
                (*opts).map_fd,
                &i as *const c_int as *const c_void,
                val_ptr as *const c_void,
                0,
            );
        }
        CHECK(
            ret < 0,
            c"bpf_map_update_elem".as_ptr(),
            c"key=%d error: %s\n".as_ptr(),
            i,
            strerror(errno),
        );

        if (*opts).map_type == BPF_MAP_TYPE_HASH_OF_MAPS {
            close(val);
        }
        i += 1;
    }
    null_mut()
}

unsafe fn upsert_elements(opts: *mut upsert_opts) {
    let mut threads: [pthread_t; N_THREADS] = [0; N_THREADS];
    let mut ret: c_int;
    let mut i: usize;

    i = 0;
    while i < threads.len() {
        ret = pthread_create(&mut threads[i], null(), patch_map_thread, opts as *mut c_void);
        CHECK(ret != 0, c"pthread_create".as_ptr(), c"error: %s\n".as_ptr(), strerror(ret));
        i += 1;
    }

    i = 0;
    while i < threads.len() {
        ret = pthread_join(threads[i], null_mut());
        CHECK(ret != 0, c"pthread_join".as_ptr(), c"error: %s\n".as_ptr(), strerror(ret));
        i += 1;
    }
}

unsafe fn read_cur_elements(iter_fd: c_int) -> __u32 {
    let mut buf: [c_char; 64] = [0; 64];
    let n: ssize_t;
    let ret: __u32;

    n = read(iter_fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 64]>() - 1);
    CHECK(n <= 0, c"read".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));
    buf[n as usize] = b'\0' as c_char;

    errno = 0;
    ret = strtol(buf.as_ptr(), null_mut(), 10) as __u32;
    CHECK(errno != 0, c"strtol".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));

    ret
}

unsafe fn get_cur_elements(map_id: c_int) -> __u32 {
    let mut skel: *mut map_percpu_stats;
    let link: *mut bpf_link;
    let n_elements: __u32;
    let iter_fd: c_int;
    let ret: c_int;

    skel = map_percpu_stats__open();
    CHECK(skel.is_null(), c"map_percpu_stats__open".as_ptr(), c"error: %s".as_ptr(), strerror(errno));

    (*(*skel).bss).target_id = map_id;

    ret = map_percpu_stats__load(skel);
    CHECK(ret != 0, c"map_percpu_stats__load".as_ptr(), c"error: %s".as_ptr(), strerror(errno));

    link = bpf_program__attach_iter((*skel).progs.dump_bpf_map, null());
    CHECK(link.is_null(), c"bpf_program__attach_iter".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    CHECK(iter_fd < 0, c"bpf_iter_create".as_ptr(), c"error: %s\n".as_ptr(), strerror(errno));

    n_elements = read_cur_elements(iter_fd);

    close(iter_fd);
    bpf_link__destroy(link);
    map_percpu_stats__destroy(skel);

    n_elements
}

unsafe fn check_expected_number_elements(n_inserted: __u32, map_fd: c_int, info: *mut bpf_map_info) {
    let n_real: __u32;
    let n_iter: __u32;

    /* Count the current number of elements in the map by iterating through
     * all the map keys via bpf_get_next_key
     */
    n_real = map_count_elements((*info).type_, map_fd);

    /* The "real" number of elements should be the same as the inserted
     * number of elements in all cases except LRU maps, where some elements
     * may have been evicted
     */
    if n_inserted == 0 || !is_lru((*info).type_) {
        CHECK(
            n_inserted != n_real,
            c"map_count_elements".as_ptr(),
            c"n_real(%u) != n_inserted(%u)\n".as_ptr(),
            n_real,
            n_inserted,
        );
    }

    /* Count the current number of elements in the map using an iterator */
    n_iter = get_cur_elements((*info).id as c_int);

    /* Both counts should be the same, as all updates are over */
    CHECK(
        n_iter != n_real,
        c"get_cur_elements".as_ptr(),
        c"n_iter=%u, expected %u (map_type=%s,map_flags=%08x)\n".as_ptr(),
        n_iter,
        n_real,
        map_type_to_s((*info).type_),
        (*info).map_flags,
    );
}

unsafe fn __test(map_fd: c_int) {
    let mut opts: upsert_opts = upsert_opts {
        map_type: 0,
        map_fd,
        n: 0,
        retry_for_nomem: false,
    };
    let mut info: bpf_map_info = zeroed();

    map_info(map_fd, &mut info);
    opts.map_type = info.type_;
    opts.n = info.max_entries;

    /* Reduce the number of elements we are updating such that we don't
     * bump into -E2BIG from non-preallocated hash maps, but still will
     * have some evictions for LRU maps  */
    if opts.map_type != BPF_MAP_TYPE_HASH_OF_MAPS {
        opts.n -= 512;
    } else {
        opts.n /= 2;
    }

    /* per-cpu bpf memory allocator may not be able to allocate per-cpu
     * pointer successfully and it can not refill free llist timely, and
     * bpf_map_update_elem() will return -ENOMEM. so just retry to mitigate
     * the problem temporarily.
     */
    opts.retry_for_nomem = is_percpu(opts.map_type) && (info.map_flags & BPF_F_NO_PREALLOC) != 0;

    /*
     * Upsert keys [0, n) under some competition: with random values from
     * N_THREADS threads. Check values, then delete all elements and check
     * values again.
     */
    upsert_elements(&mut opts);
    check_expected_number_elements(opts.n, map_fd, &mut info);
    delete_all_elements(info.type_, map_fd, !BATCH);
    check_expected_number_elements(0, map_fd, &mut info);

    /* Now do the same, but using batch delete operations */
    upsert_elements(&mut opts);
    check_expected_number_elements(opts.n, map_fd, &mut info);
    delete_all_elements(info.type_, map_fd, BATCH);
    check_expected_number_elements(0, map_fd, &mut info);

    close(map_fd);
}

unsafe fn map_create_opts(
    type_: __u32,
    name: *const c_char,
    map_opts: *mut bpf_map_create_opts,
    key_size: __u32,
    val_size: __u32,
) -> c_int {
    let max_entries: c_int;
    let map_fd: c_int;

    if type_ == BPF_MAP_TYPE_HASH_OF_MAPS {
        max_entries = MAX_ENTRIES_HASH_OF_MAPS as c_int;
    } else {
        max_entries = MAX_ENTRIES as c_int;
    }

    map_fd = bpf_map_create(type_, name, key_size, val_size, max_entries as __u32, map_opts);
    CHECK(
        map_fd < 0,
        c"bpf_map_create()".as_ptr(),
        c"error:%s (name=%s)\n".as_ptr(),
        strerror(errno),
        name,
    );

    map_fd
}

unsafe fn map_create(type_: __u32, name: *const c_char, map_opts: *mut bpf_map_create_opts) -> c_int {
    map_create_opts(type_, name, map_opts, size_of::<c_int>() as __u32, size_of::<c_int>() as __u32)
}

unsafe fn create_hash() -> c_int {
    let mut map_opts: bpf_map_create_opts = zeroed();
    map_opts.sz = size_of::<bpf_map_create_opts>();
    map_opts.map_flags = BPF_F_NO_PREALLOC;

    map_create(BPF_MAP_TYPE_HASH, c"hash".as_ptr(), &mut map_opts)
}

unsafe fn create_percpu_hash() -> c_int {
    let mut map_opts: bpf_map_create_opts = zeroed();
    map_opts.sz = size_of::<bpf_map_create_opts>();
    map_opts.map_flags = BPF_F_NO_PREALLOC;

    map_create(BPF_MAP_TYPE_PERCPU_HASH, c"percpu_hash".as_ptr(), &mut map_opts)
}

unsafe fn create_hash_prealloc() -> c_int {
    map_create(BPF_MAP_TYPE_HASH, c"hash".as_ptr(), null_mut())
}

unsafe fn create_percpu_hash_prealloc() -> c_int {
    map_create(BPF_MAP_TYPE_PERCPU_HASH, c"percpu_hash_prealloc".as_ptr(), null_mut())
}

unsafe fn create_lru_hash(type_: __u32, map_flags: __u32) -> c_int {
    let mut map_opts: bpf_map_create_opts = zeroed();
    map_opts.sz = size_of::<bpf_map_create_opts>();
    map_opts.map_flags = map_flags;

    map_create(type_, c"lru_hash".as_ptr(), &mut map_opts)
}

unsafe fn create_hash_of_maps() -> c_int {
    let mut map_opts: bpf_map_create_opts = zeroed();
    let ret: c_int;

    map_opts.sz = size_of::<bpf_map_create_opts>();
    map_opts.map_flags = BPF_F_NO_PREALLOC;
    map_opts.inner_map_fd = create_small_hash();

    ret = map_create_opts(
        BPF_MAP_TYPE_HASH_OF_MAPS,
        c"hash_of_maps".as_ptr(),
        &mut map_opts,
        size_of::<c_int>() as __u32,
        size_of::<c_int>() as __u32,
    );
    close(map_opts.inner_map_fd);
    ret
}

unsafe fn map_percpu_stats_hash() {
    __test(create_hash());
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_hash".as_ptr());
}

unsafe fn map_percpu_stats_percpu_hash() {
    __test(create_percpu_hash());
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_percpu_hash".as_ptr());
}

unsafe fn map_percpu_stats_hash_prealloc() {
    __test(create_hash_prealloc());
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_hash_prealloc".as_ptr());
}

unsafe fn map_percpu_stats_percpu_hash_prealloc() {
    __test(create_percpu_hash_prealloc());
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_percpu_hash_prealloc".as_ptr());
}

unsafe fn map_percpu_stats_lru_hash() {
    __test(create_lru_hash(BPF_MAP_TYPE_LRU_HASH, 0));
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_lru_hash".as_ptr());
}

unsafe fn map_percpu_stats_lru_hash_no_common() {
    __test(create_lru_hash(BPF_MAP_TYPE_LRU_HASH, BPF_F_NO_COMMON_LRU));
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_lru_hash_no_common".as_ptr());
}

unsafe fn map_percpu_stats_percpu_lru_hash() {
    __test(create_lru_hash(BPF_MAP_TYPE_LRU_PERCPU_HASH, 0));
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_percpu_lru_hash".as_ptr());
}

unsafe fn map_percpu_stats_percpu_lru_hash_no_common() {
    __test(create_lru_hash(BPF_MAP_TYPE_LRU_PERCPU_HASH, BPF_F_NO_COMMON_LRU));
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_percpu_lru_hash_no_common".as_ptr());
}

unsafe fn map_percpu_stats_hash_of_maps() {
    __test(create_hash_of_maps());
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_hash_of_maps".as_ptr());
}

unsafe fn map_percpu_stats_map_value_size() {
    let mut fd: c_int;
    let value_sz: c_int = PCPU_MIN_UNIT_SIZE + 1;
    let mut opts: bpf_map_create_opts = zeroed();
    let map_types: [__u32; 3] = [
        BPF_MAP_TYPE_PERCPU_ARRAY,
        BPF_MAP_TYPE_PERCPU_HASH,
        BPF_MAP_TYPE_LRU_PERCPU_HASH,
    ];
    opts.sz = size_of::<bpf_map_create_opts>();

    let mut i: usize = 0;
    while i < map_types.len() {
        fd = bpf_map_create(
            map_types[i],
            null(),
            size_of::<__u32>() as __u32,
            value_sz as __u32,
            1,
            &opts,
        );
        CHECK(
            fd < 0 && errno != E2BIG,
            c"percpu map value size".as_ptr(),
            c"error: %s\n".as_ptr(),
            strerror(errno),
        );
        i += 1;
    }
    printf(c"test_%s:PASS\n".as_ptr(), c"map_percpu_stats_map_value_size".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_map_percpu_stats() {
    map_percpu_stats_hash();
    map_percpu_stats_percpu_hash();
    map_percpu_stats_hash_prealloc();
    map_percpu_stats_percpu_hash_prealloc();
    map_percpu_stats_lru_hash();
    map_percpu_stats_lru_hash_no_common();
    map_percpu_stats_percpu_lru_hash();
    map_percpu_stats_percpu_lru_hash_no_common();
    map_percpu_stats_hash_of_maps();
    map_percpu_stats_map_value_size();
}
