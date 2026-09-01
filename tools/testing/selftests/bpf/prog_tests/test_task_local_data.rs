// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original dependencies:
// - <pthread.h>
// - <bpf/btf.h>
// - <test_progs.h>
// - "task_local_data.h"
// - "test_task_local_data.skel.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

type __u16 = u16;
type __u64 = u64;
type intptr_t = isize;
type pthread_t = usize;
type pthread_mutex_t = c_void;
type tld_key_t = isize;

const E2BIG: c_int = 7;
const EEXIST: c_int = 17;
const ENOSPC: c_int = 28;

extern "C" {
    fn getpagesize() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;

    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;

    fn sys_gettid() -> c_int;

    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn test_task_local_data__open_and_load() -> *mut test_task_local_data;
    fn test_task_local_data__destroy(skel: *mut test_task_local_data);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn tld_get_data(fd: c_int, key: tld_key_t) -> *mut c_void;
    fn tld_create_key(name: *const c_char, size: usize) -> tld_key_t;
    fn tld_key_is_err(key: tld_key_t) -> bool;
    fn tld_key_err_or_zero(key: tld_key_t) -> c_int;
    fn tld_free();

    static mut tld_meta_p: *mut tld_meta;
    static value0_key: tld_key_t;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn CHECK_FAIL(condition: bool) -> bool;
}

#[repr(C)]
struct bpf_test_run_opts {
    retval: c_int,
}

#[repr(C)]
struct tld_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct tld_data_u {
    _private: [u8; 0],
}

#[repr(C)]
struct tld_meta {
    cnt: __u16,
    size: __u16,
    metadata: [tld_metadata; TLD_MAX_DATA_CNT],
}

#[repr(C)]
struct test_task_local_data_maps {
    tld_data_map: *mut c_void,
}

#[repr(C)]
struct test_task_local_data_progs {
    task_main: *mut c_void,
}

#[repr(C)]
struct test_task_local_data_bss {
    test_value0: c_int,
    test_value1: c_int,
    test_value2: test_tld_struct,
}

#[repr(C)]
struct test_task_local_data {
    maps: test_task_local_data_maps,
    progs: test_task_local_data_progs,
    bss: *mut test_task_local_data_bss,
}

/*
 * Only a page is pinned to kernel, so the maximum amount of dynamic data
 * allowed is page_size - sizeof(struct tld_data_u) - static TLD fields.
 */
unsafe fn TLD_DYN_DATA_SIZE_MAX() -> __u16 {
    (getpagesize() as usize - size_of::<tld_data_u>() - 8) as __u16
}

// #define TLD_FREE_DATA_ON_THREAD_EXIT
// #define TLD_DYN_DATA_SIZE TLD_DYN_DATA_SIZE_MAX

const TLD_MAX_DATA_CNT: usize = 64;
const TLD_NAME_LEN: usize = 32;
const TLD_PAGE_SIZE: usize = 4096;

fn TLD_ROUND_UP(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[repr(C)]
struct test_tld_struct {
    a: __u64,
    b: __u64,
    c: __u64,
    d: __u64,
}

// TLD_DEFINE_KEY(value0_key, "value0", sizeof(int));

/*
 * Reset task local data between subtests by clearing metadata other
 * than the statically defined value0. This is safe as subtests run
 * sequentially. Users of task local data library should not touch
 * library internal.
 */
unsafe fn reset_tld(dyn_data_size: __u16) {
    if !tld_meta_p.is_null() {
        /* Remove TLDs created by tld_create_key() */
        (*tld_meta_p).cnt = 1;
        (*tld_meta_p).size = dyn_data_size + 8;
        memset(
            (*tld_meta_p).metadata.as_mut_ptr().add(1) as *mut c_void,
            0,
            (TLD_MAX_DATA_CNT - 1) * size_of::<tld_metadata>(),
        );
    }
}

/* Serialize access to bpf program's global variables */
static mut global_mutex: MaybeUninit<pthread_mutex_t> = MaybeUninit::uninit();

static mut tld_keys: *mut tld_key_t = ptr::null_mut();

const TEST_BASIC_THREAD_NUM: usize = 32;

unsafe extern "C" fn test_task_local_data_basic_thread(arg: *mut c_void) -> *mut c_void {
    let mut opts: bpf_test_run_opts = MaybeUninit::zeroed().assume_init();
    let skel = arg as *mut test_task_local_data;
    let fd: c_int;
    let mut err: c_int;
    let tid: c_int;
    let value0: *mut c_int;
    let value1: *mut c_int;
    let value2: *mut test_tld_struct;

    fd = bpf_map__fd((*skel).maps.tld_data_map);

    value0 = tld_get_data(fd, value0_key) as *mut c_int;
    if !ASSERT_OK_PTR(value0 as *const c_void, b"tld_get_data\0".as_ptr() as *const c_char) {
        goto_out_basic_thread();
    }

    value1 = tld_get_data(fd, *tld_keys.add(1)) as *mut c_int;
    if !ASSERT_OK_PTR(value1 as *const c_void, b"tld_get_data\0".as_ptr() as *const c_char) {
        goto_out_basic_thread();
    }

    value2 = tld_get_data(fd, *tld_keys.add(2)) as *mut test_tld_struct;
    if !ASSERT_OK_PTR(value2 as *const c_void, b"tld_get_data\0".as_ptr() as *const c_char) {
        goto_out_basic_thread();
    }

    tid = sys_gettid();

    *value0 = tid + 0;
    *value1 = tid + 1;
    (*value2).a = (tid + 2) as __u64;
    (*value2).b = (tid + 3) as __u64;
    (*value2).c = (tid + 4) as __u64;
    (*value2).d = (tid + 5) as __u64;

    pthread_mutex_lock(global_mutex.as_mut_ptr());
    /* Run task_main that read task local data and save to global variables */
    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.task_main), &mut opts);
    ASSERT_OK(err, b"run task_main\0".as_ptr() as *const c_char);
    ASSERT_OK(opts.retval, b"task_main retval\0".as_ptr() as *const c_char);

    ASSERT_EQ((*(*skel).bss).test_value0 as i64, (tid + 0) as i64, b"tld_get_data value0\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value1 as i64, (tid + 1) as i64, b"tld_get_data value1\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.a as i64, (tid + 2) as i64, b"tld_get_data value2.a\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.b as i64, (tid + 3) as i64, b"tld_get_data value2.b\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.c as i64, (tid + 4) as i64, b"tld_get_data value2.c\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.d as i64, (tid + 5) as i64, b"tld_get_data value2.d\0".as_ptr() as *const c_char);
    pthread_mutex_unlock(global_mutex.as_mut_ptr());

    /* Make sure valueX are indeed local to threads */
    ASSERT_EQ(*value0 as i64, (tid + 0) as i64, b"value0\0".as_ptr() as *const c_char);
    ASSERT_EQ(*value1 as i64, (tid + 1) as i64, b"value1\0".as_ptr() as *const c_char);
    ASSERT_EQ((*value2).a as i64, (tid + 2) as i64, b"value2.a\0".as_ptr() as *const c_char);
    ASSERT_EQ((*value2).b as i64, (tid + 3) as i64, b"value2.b\0".as_ptr() as *const c_char);
    ASSERT_EQ((*value2).c as i64, (tid + 4) as i64, b"value2.c\0".as_ptr() as *const c_char);
    ASSERT_EQ((*value2).d as i64, (tid + 5) as i64, b"value2.d\0".as_ptr() as *const c_char);

    *value0 = tid + 5;
    *value1 = tid + 4;
    (*value2).a = (tid + 3) as __u64;
    (*value2).b = (tid + 2) as __u64;
    (*value2).c = (tid + 1) as __u64;
    (*value2).d = (tid + 0) as __u64;

    /* Run task_main again */
    pthread_mutex_lock(global_mutex.as_mut_ptr());
    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.task_main), &mut opts);
    ASSERT_OK(err, b"run task_main\0".as_ptr() as *const c_char);
    ASSERT_OK(opts.retval, b"task_main retval\0".as_ptr() as *const c_char);

    ASSERT_EQ((*(*skel).bss).test_value0 as i64, (tid + 5) as i64, b"tld_get_data value0\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value1 as i64, (tid + 4) as i64, b"tld_get_data value1\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.a as i64, (tid + 3) as i64, b"tld_get_data value2.a\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.b as i64, (tid + 2) as i64, b"tld_get_data value2.b\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.c as i64, (tid + 1) as i64, b"tld_get_data value2.c\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value2.d as i64, (tid + 0) as i64, b"tld_get_data value2.d\0".as_ptr() as *const c_char);
    pthread_mutex_unlock(global_mutex.as_mut_ptr());

    pthread_exit(ptr::null_mut());
}

unsafe fn goto_out_basic_thread() -> ! {
    pthread_exit(ptr::null_mut());
}

unsafe fn test_task_local_data_basic() {
    let mut skel: *mut test_task_local_data;
    let mut thread: [pthread_t; TEST_BASIC_THREAD_NUM] = [0; TEST_BASIC_THREAD_NUM];
    let mut dummy_key_name: [c_char; TLD_NAME_LEN] = [0; TLD_NAME_LEN];
    let mut key: tld_key_t;
    let mut i: c_int;
    let mut err: c_int;

    reset_tld(TLD_DYN_DATA_SIZE_MAX());

    ASSERT_OK(
        pthread_mutex_init(global_mutex.as_mut_ptr(), ptr::null()),
        b"pthread_mutex_init\0".as_ptr() as *const c_char,
    );

    skel = test_task_local_data__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    tld_keys = calloc(TLD_MAX_DATA_CNT, size_of::<tld_key_t>()) as *mut tld_key_t;
    if !ASSERT_OK_PTR(tld_keys as *const c_void, b"calloc tld_keys\0".as_ptr() as *const c_char) {
        goto_out_basic(skel, &mut thread);
        return;
    }

    ASSERT_FALSE(tld_key_is_err(value0_key), b"TLD_DEFINE_KEY\0".as_ptr() as *const c_char);
    *tld_keys.add(1) = tld_create_key(b"value1\0".as_ptr() as *const c_char, size_of::<c_int>());
    ASSERT_FALSE(tld_key_is_err(*tld_keys.add(1)), b"tld_create_key\0".as_ptr() as *const c_char);
    *tld_keys.add(2) = tld_create_key(b"value2\0".as_ptr() as *const c_char, size_of::<test_tld_struct>());
    ASSERT_FALSE(tld_key_is_err(*tld_keys.add(2)), b"tld_create_key\0".as_ptr() as *const c_char);

    /*
     * Shouldn't be able to store data exceed a page. Create a TLD just big
     * enough to exceed a page. Data already contains struct tld_data_u,
     * value0 and value1 of int type, and value 2 of struct test_tld_struct.
     */
    key = tld_create_key(
        b"value_not_exist\0".as_ptr() as *const c_char,
        TLD_PAGE_SIZE + 1
            - size_of::<tld_data_u>()
            - TLD_ROUND_UP(size_of::<c_int>(), 8) * 2
            - TLD_ROUND_UP(size_of::<test_tld_struct>(), 8),
    );
    ASSERT_EQ(tld_key_err_or_zero(key) as i64, (-E2BIG) as i64, b"tld_create_key\0".as_ptr() as *const c_char);

    key = tld_create_key(b"value2\0".as_ptr() as *const c_char, size_of::<test_tld_struct>());
    ASSERT_EQ(tld_key_err_or_zero(key) as i64, (-EEXIST) as i64, b"tld_create_key\0".as_ptr() as *const c_char);

    /* Shouldn't be able to create the (TLD_MAX_DATA_CNT+1)-th TLD */
    i = 3;
    while (i as usize) < TLD_MAX_DATA_CNT {
        snprintf(
            dummy_key_name.as_mut_ptr(),
            TLD_NAME_LEN,
            b"dummy_value%d\0".as_ptr() as *const c_char,
            i,
        );
        *tld_keys.add(i as usize) = tld_create_key(dummy_key_name.as_ptr(), size_of::<c_int>());
        ASSERT_FALSE(tld_key_is_err(*tld_keys.add(i as usize)), b"tld_create_key\0".as_ptr() as *const c_char);
        i += 1;
    }
    key = tld_create_key(b"value_not_exist\0".as_ptr() as *const c_char, size_of::<test_tld_struct>());
    ASSERT_EQ(tld_key_err_or_zero(key) as i64, (-ENOSPC) as i64, b"tld_create_key\0".as_ptr() as *const c_char);

    /* Access TLDs from multiple threads and check if they are thread-specific */
    i = 0;
    while (i as usize) < TEST_BASIC_THREAD_NUM {
        err = pthread_create(
            &mut thread[i as usize],
            ptr::null(),
            test_task_local_data_basic_thread,
            skel as *mut c_void,
        );
        if !ASSERT_OK(err, b"pthread_create\0".as_ptr() as *const c_char) {
            break;
        }
        i += 1;
    }

    goto_out_basic(skel, &mut thread);
}

unsafe fn goto_out_basic(skel: *mut test_task_local_data, thread: &mut [pthread_t; TEST_BASIC_THREAD_NUM]) {
    let mut i: c_int = 0;
    while (i as usize) < TEST_BASIC_THREAD_NUM {
        pthread_join(thread[i as usize], ptr::null_mut());
        i += 1;
    }

    if !tld_keys.is_null() {
        free(tld_keys as *mut c_void);
        tld_keys = ptr::null_mut();
    }
    tld_free();
    test_task_local_data__destroy(skel);
}

const TEST_RACE_THREAD_NUM: usize = TLD_MAX_DATA_CNT - 3;

unsafe extern "C" fn test_task_local_data_race_thread(arg: *mut c_void) -> *mut c_void {
    let mut err: c_int = 0;
    let id: c_int = arg as intptr_t as c_int;
    let mut key_name: [c_char; 32] = [0; 32];
    let mut key: tld_key_t;

    key = tld_create_key(b"value_not_exist\0".as_ptr() as *const c_char, TLD_PAGE_SIZE + 1);
    if tld_key_err_or_zero(key) != -E2BIG {
        err = 1;
        return err as intptr_t as *mut c_void;
    }

    /* Only one thread will succeed in creating value1 */
    key = tld_create_key(b"value1\0".as_ptr() as *const c_char, size_of::<c_int>());
    if !tld_key_is_err(key) {
        *tld_keys.add(1) = key;
    }

    /* Only one thread will succeed in creating value2 */
    key = tld_create_key(b"value2\0".as_ptr() as *const c_char, size_of::<test_tld_struct>());
    if !tld_key_is_err(key) {
        *tld_keys.add(2) = key;
    }

    snprintf(key_name.as_mut_ptr(), 32, b"thread_%d\0".as_ptr() as *const c_char, id);
    *tld_keys.add(id as usize) = tld_create_key(key_name.as_ptr(), size_of::<c_int>());
    if tld_key_is_err(*tld_keys.add(id as usize)) {
        err = 2;
    }
    err as intptr_t as *mut c_void
}

unsafe fn test_task_local_data_race() {
    let mut opts: bpf_test_run_opts = MaybeUninit::zeroed().assume_init();
    let mut thread: [pthread_t; TEST_RACE_THREAD_NUM] = [0; TEST_RACE_THREAD_NUM];
    let mut skel: *mut test_task_local_data;
    let fd: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut err: c_int;
    let mut data: *mut c_int;
    let mut ret: *mut c_void = ptr::null_mut();

    skel = test_task_local_data__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    tld_keys = calloc(TLD_MAX_DATA_CNT, size_of::<tld_key_t>()) as *mut tld_key_t;
    if !ASSERT_OK_PTR(tld_keys as *const c_void, b"calloc tld_keys\0".as_ptr() as *const c_char) {
        goto_out_race(skel);
        return;
    }

    fd = bpf_map__fd((*skel).maps.tld_data_map);

    ASSERT_FALSE(tld_key_is_err(value0_key), b"TLD_DEFINE_KEY\0".as_ptr() as *const c_char);
    *tld_keys.add(0) = value0_key;

    j = 0;
    while j < 100 {
        reset_tld(TLD_DYN_DATA_SIZE_MAX());

        i = 0;
        while (i as usize) < TEST_RACE_THREAD_NUM {
            /*
             * Try to make tld_create_key() race with each other. Call
             * tld_create_key(), both valid and invalid, from different threads.
             */
            err = pthread_create(
                &mut thread[i as usize],
                ptr::null(),
                test_task_local_data_race_thread,
                (i + 3) as intptr_t as *mut c_void,
            );
            if CHECK_FAIL(err != 0) {
                break;
            }
            i += 1;
        }

        /* Wait for all tld_create_key() to return */
        i = 0;
        while (i as usize) < TEST_RACE_THREAD_NUM {
            pthread_join(thread[i as usize], &mut ret);
            if CHECK_FAIL(!ret.is_null()) {
                break;
            }
            i += 1;
        }

        /* Write a unique number to each TLD */
        i = 0;
        while (i as usize) < TLD_MAX_DATA_CNT {
            data = tld_get_data(fd, *tld_keys.add(i as usize)) as *mut c_int;
            if CHECK_FAIL(data.is_null()) {
                break;
            }
            *data = i;
            i += 1;
        }

        /* Read TLDs and check the value to see if any address collides with another */
        i = 0;
        while (i as usize) < TLD_MAX_DATA_CNT {
            data = tld_get_data(fd, *tld_keys.add(i as usize)) as *mut c_int;
            if CHECK_FAIL(*data != i) {
                break;
            }
            i += 1;
        }

        /* Run task_main to make sure no invalid TLDs are added */
        err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.task_main), &mut opts);
        ASSERT_OK(err, b"run task_main\0".as_ptr() as *const c_char);
        ASSERT_OK(opts.retval, b"task_main retval\0".as_ptr() as *const c_char);

        j += 1;
    }
    goto_out_race(skel);
}

unsafe fn goto_out_race(skel: *mut test_task_local_data) {
    if !tld_keys.is_null() {
        free(tld_keys as *mut c_void);
        tld_keys = ptr::null_mut();
    }
    tld_free();
    test_task_local_data__destroy(skel);
}

unsafe fn test_task_local_data_dyn_size(dyn_data_size: __u16) {
    let mut opts: bpf_test_run_opts = MaybeUninit::zeroed().assume_init();
    let mut skel: *mut test_task_local_data;
    let max_keys: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let fd: c_int;
    let mut data: *mut c_int;
    let mut name: [c_char; TLD_NAME_LEN] = [0; TLD_NAME_LEN];
    let mut key: tld_key_t;

    reset_tld(dyn_data_size);

    skel = test_task_local_data__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    tld_keys = calloc(TLD_MAX_DATA_CNT, size_of::<tld_key_t>()) as *mut tld_key_t;
    if !ASSERT_OK_PTR(tld_keys as *const c_void, b"calloc tld_keys\0".as_ptr() as *const c_char) {
        goto_out_dyn_size(skel);
        return;
    }

    fd = bpf_map__fd((*skel).maps.tld_data_map);

    /* Create as many int-sized TLDs as the dynamic data size allows */
    max_keys = (dyn_data_size as usize / TLD_ROUND_UP(size_of::<c_int>(), 8)) as c_int;
    i = 0;
    while i < max_keys {
        snprintf(name.as_mut_ptr(), TLD_NAME_LEN, b"value_%d\0".as_ptr() as *const c_char, i);
        *tld_keys.add(i as usize) = tld_create_key(name.as_ptr(), size_of::<c_int>());
        if !ASSERT_FALSE(tld_key_is_err(*tld_keys.add(i as usize)), b"tld_create_key\0".as_ptr() as *const c_char) {
            goto_out_dyn_size(skel);
            return;
        }

        data = tld_get_data(fd, *tld_keys.add(i as usize)) as *mut c_int;
        if !ASSERT_OK_PTR(data as *const c_void, b"tld_get_data\0".as_ptr() as *const c_char) {
            goto_out_dyn_size(skel);
            return;
        }
        *data = i;
        i += 1;
    }

    /* The next key should fail with E2BIG */
    key = tld_create_key(b"overflow\0".as_ptr() as *const c_char, size_of::<c_int>());
    ASSERT_EQ(tld_key_err_or_zero(key) as i64, (-E2BIG) as i64, b"tld_create_key overflow\0".as_ptr() as *const c_char);

    /* Verify data for value_i do not overlap */
    i = 0;
    while i < max_keys {
        data = tld_get_data(fd, *tld_keys.add(i as usize)) as *mut c_int;
        if !ASSERT_OK_PTR(data as *const c_void, b"tld_get_data\0".as_ptr() as *const c_char) {
            goto_out_dyn_size(skel);
            return;
        }

        ASSERT_EQ(*data as i64, i as i64, b"tld_get_data value_i\0".as_ptr() as *const c_char);
        i += 1;
    }

    /* Verify BPF side can still read the static key */
    data = tld_get_data(fd, value0_key) as *mut c_int;
    if !ASSERT_OK_PTR(data as *const c_void, b"tld_get_data value0\0".as_ptr() as *const c_char) {
        goto_out_dyn_size(skel);
        return;
    }
    *data = 0xdeadbeefu32 as c_int;

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.task_main), &mut opts);
    ASSERT_OK(err, b"run task_main\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).test_value0 as i64, 0xdeadbeefu32 as c_int as i64, b"tld_get_data value0\0".as_ptr() as *const c_char);

    goto_out_dyn_size(skel);
}

unsafe fn goto_out_dyn_size(skel: *mut test_task_local_data) {
    if !tld_keys.is_null() {
        free(tld_keys as *mut c_void);
        tld_keys = ptr::null_mut();
    }
    tld_free();
    test_task_local_data__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_task_local_data() {
    if test__start_subtest(b"task_local_data_basic\0".as_ptr() as *const c_char) {
        test_task_local_data_basic();
    }
    if test__start_subtest(b"task_local_data_race\0".as_ptr() as *const c_char) {
        test_task_local_data_race();
    }
    if test__start_subtest(b"task_local_data_dyn_size_small\0".as_ptr() as *const c_char) {
        test_task_local_data_dyn_size(64);
    }
    if test__start_subtest(b"task_local_data_dyn_size_zero\0".as_ptr() as *const c_char) {
        test_task_local_data_dyn_size(0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
