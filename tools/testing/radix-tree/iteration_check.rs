// SPDX-License-Identifier: GPL-2.0-only
/*
 * iteration_check.c: test races having to do with xarray iteration
 * Copyright (c) 2016 Intel Corporation
 * Author: Ross Zwisler <ross.zwisler@linux.intel.com>
 */

use core::ffi::c_void;

const NUM_THREADS: usize = 5;
const MAX_IDX: c_ulong = 100;
const TAG: xa_mark_t = XA_MARK_0;
const NEW_TAG: xa_mark_t = XA_MARK_1;
const GFP_KERNEL: gfp_t = 0;
const ULONG_MAX: c_ulong = c_ulong::MAX;

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type pthread_t = c_ulong;
type xa_mark_t = c_uint;
type gfp_t = c_uint;

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xa_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct item {
    pub order: c_int,
}

const XA_MARK_0: xa_mark_t = 0;
const XA_MARK_1: xa_mark_t = 1;

static mut THREADS: [pthread_t; NUM_THREADS] = [0; NUM_THREADS];
static mut SEEDS: [c_uint; 3] = [0; 3];
static mut ARRAY: xarray = xarray { _private: [] };
static mut TEST_COMPLETE: bool = false;
static mut MAX_ORDER: c_int = 0;

unsafe extern "C" {
    fn item_create(index: c_ulong, order: c_uint) -> *mut item;
    fn item_free(item: *mut item, index: c_ulong);
    fn item_kill_tree(xa: *mut xarray);
    fn tag_tagged_items(
        xa: *mut xarray,
        start: c_ulong,
        end: c_ulong,
        batch: c_ulong,
        old_tag: xa_mark_t,
        new_tag: xa_mark_t,
    );

    fn xa_erase(xa: *mut xarray, index: c_ulong) -> *mut item;

    fn xa_state_init(xas: *mut xa_state, xa: *mut xarray, index: c_ulong);
    fn xas_lock(xas: *mut xa_state);
    fn xas_unlock(xas: *mut xa_state);
    fn xas_set_order(xas: *mut xa_state, index: c_ulong, order: c_uint);
    fn xas_find_conflict(xas: *mut xa_state) -> *mut c_void;
    fn xas_store(xas: *mut xa_state, entry: *mut c_void);
    fn xas_set_mark(xas: *mut xa_state, mark: xa_mark_t);
    fn xas_nomem(xas: *mut xa_state, gfp: gfp_t) -> bool;
    fn xas_set(xas: *mut xa_state, index: c_ulong);
    fn xas_find_marked(xas: *mut xa_state, max: c_ulong, mark: xa_mark_t) -> *mut c_void;
    fn xas_find(xas: *mut xa_state, max: c_ulong) -> *mut c_void;
    fn xas_retry(xas: *mut xa_state, entry: *mut c_void) -> bool;
    fn xas_pause(xas: *mut xa_state);

    fn rcu_register_thread();
    fn rcu_unregister_thread();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_barrier();

    fn rand() -> c_int;
    fn rand_r(seedp: *mut c_uint) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn perror(s: *const i8);
    fn exit(status: c_int) -> !;
    fn printv(level: c_int, fmt: *const i8, ...);
    fn free(ptr: *mut c_void);
}

pub unsafe fn my_item_insert(xa: *mut xarray, index: c_ulong) {
    let mut xas = core::mem::MaybeUninit::<xa_state>::uninit();
    unsafe { xa_state_init(xas.as_mut_ptr(), xa, index) };
    let mut xas = unsafe { xas.assume_init() };
    let item = unsafe { item_create(index, 0) };
    let mut order: c_int;

    loop {
        unsafe { xas_lock(&mut xas) };
        order = unsafe { MAX_ORDER };
        while order >= 0 {
            unsafe { xas_set_order(&mut xas, index, order as c_uint) };
            unsafe {
                (*item).order = order;
            }
            if !unsafe { xas_find_conflict(&mut xas).is_null() } {
                order -= 1;
                continue;
            }
            unsafe { xas_store(&mut xas, item.cast::<c_void>()) };
            unsafe { xas_set_mark(&mut xas, TAG) };
            break;
        }
        unsafe { xas_unlock(&mut xas) };
        if !unsafe { xas_nomem(&mut xas, GFP_KERNEL) } {
            break;
        }
    }
    if order < 0 {
        unsafe { free(item.cast::<c_void>()) };
    }
}

/* relentlessly fill the array with tagged entries */
unsafe extern "C" fn add_entries_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe { rcu_register_thread() };

    while !unsafe { TEST_COMPLETE } {
        let mut pgoff: c_ulong = 0;

        while pgoff < MAX_IDX {
            unsafe { my_item_insert(&raw mut ARRAY, pgoff) };
            pgoff += 1;
        }
    }

    unsafe { rcu_unregister_thread() };

    core::ptr::null_mut()
}

/*
 * Iterate over tagged entries, retrying when we find ourselves in a deleted
 * node and randomly pausing the iteration.
 */
unsafe extern "C" fn tagged_iteration_fn(_arg: *mut c_void) -> *mut c_void {
    let mut xas = core::mem::MaybeUninit::<xa_state>::uninit();
    unsafe { xa_state_init(xas.as_mut_ptr(), &raw mut ARRAY, 0) };
    let mut xas = unsafe { xas.assume_init() };
    let mut entry: *mut c_void;

    unsafe { rcu_register_thread() };

    while !unsafe { TEST_COMPLETE } {
        unsafe { xas_set(&mut xas, 0) };
        unsafe { rcu_read_lock() };
        entry = unsafe { xas_find_marked(&mut xas, ULONG_MAX, TAG) };
        while !entry.is_null() {
            if unsafe { xas_retry(&mut xas, entry) } {
                entry = unsafe { xas_find_marked(&mut xas, ULONG_MAX, TAG) };
                continue;
            }

            if unsafe { rand_r(&raw mut SEEDS[0]) } % 50 == 0 {
                unsafe { xas_pause(&mut xas) };
                unsafe { rcu_read_unlock() };
                unsafe { rcu_barrier() };
                unsafe { rcu_read_lock() };
            }
            entry = unsafe { xas_find_marked(&mut xas, ULONG_MAX, TAG) };
        }
        unsafe { rcu_read_unlock() };
    }

    unsafe { rcu_unregister_thread() };

    core::ptr::null_mut()
}

/*
 * Iterate over the entries, retrying when we find ourselves in a deleted
 * node and randomly pausing the iteration.
 */
unsafe extern "C" fn untagged_iteration_fn(_arg: *mut c_void) -> *mut c_void {
    let mut xas = core::mem::MaybeUninit::<xa_state>::uninit();
    unsafe { xa_state_init(xas.as_mut_ptr(), &raw mut ARRAY, 0) };
    let mut xas = unsafe { xas.assume_init() };
    let mut entry: *mut c_void;

    unsafe { rcu_register_thread() };

    while !unsafe { TEST_COMPLETE } {
        unsafe { xas_set(&mut xas, 0) };
        unsafe { rcu_read_lock() };
        entry = unsafe { xas_find(&mut xas, ULONG_MAX) };
        while !entry.is_null() {
            if unsafe { xas_retry(&mut xas, entry) } {
                entry = unsafe { xas_find(&mut xas, ULONG_MAX) };
                continue;
            }

            if unsafe { rand_r(&raw mut SEEDS[1]) } % 50 == 0 {
                unsafe { xas_pause(&mut xas) };
                unsafe { rcu_read_unlock() };
                unsafe { rcu_barrier() };
                unsafe { rcu_read_lock() };
            }
            entry = unsafe { xas_find(&mut xas, ULONG_MAX) };
        }
        unsafe { rcu_read_unlock() };
    }

    unsafe { rcu_unregister_thread() };

    core::ptr::null_mut()
}

/*
 * Randomly remove entries to help induce retries in the
 * two iteration functions.
 */
unsafe extern "C" fn remove_entries_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe { rcu_register_thread() };

    while !unsafe { TEST_COMPLETE } {
        let pgoff: c_int = unsafe { rand_r(&raw mut SEEDS[2]) } % MAX_IDX as c_int;

        let item = unsafe { xa_erase(&raw mut ARRAY, pgoff as c_ulong) };
        if !item.is_null() {
            unsafe { item_free(item, pgoff as c_ulong) };
        }
    }

    unsafe { rcu_unregister_thread() };

    core::ptr::null_mut()
}

unsafe extern "C" fn tag_entries_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe { rcu_register_thread() };

    while !unsafe { TEST_COMPLETE } {
        unsafe { tag_tagged_items(&raw mut ARRAY, 0, MAX_IDX, 10, TAG, NEW_TAG) };
    }
    unsafe { rcu_unregister_thread() };
    core::ptr::null_mut()
}

/* This is a unit test for a bug found by the syzkaller tester */
pub unsafe fn iteration_test(order: c_uint, test_duration: c_uint) {
    let mut i: c_int;

    unsafe {
        printv(
            1,
            c"Running %siteration tests for %d seconds\n".as_ptr(),
            if order > 0 {
                c"multiorder ".as_ptr()
            } else {
                c"".as_ptr()
            },
            test_duration,
        )
    };

    unsafe {
        MAX_ORDER = order as c_int;
        TEST_COMPLETE = false;
    }

    i = 0;
    while i < 3 {
        unsafe {
            SEEDS[i as usize] = rand() as c_uint;
        }
        i += 1;
    }

    if unsafe {
        pthread_create(
            &raw mut THREADS[0],
            core::ptr::null(),
            tagged_iteration_fn,
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe { perror(c"create tagged iteration thread".as_ptr()) };
        unsafe { exit(1) };
    }
    if unsafe {
        pthread_create(
            &raw mut THREADS[1],
            core::ptr::null(),
            untagged_iteration_fn,
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe { perror(c"create untagged iteration thread".as_ptr()) };
        unsafe { exit(1) };
    }
    if unsafe {
        pthread_create(
            &raw mut THREADS[2],
            core::ptr::null(),
            add_entries_fn,
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe { perror(c"create add entry thread".as_ptr()) };
        unsafe { exit(1) };
    }
    if unsafe {
        pthread_create(
            &raw mut THREADS[3],
            core::ptr::null(),
            remove_entries_fn,
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe { perror(c"create remove entry thread".as_ptr()) };
        unsafe { exit(1) };
    }
    if unsafe {
        pthread_create(
            &raw mut THREADS[4],
            core::ptr::null(),
            tag_entries_fn,
            core::ptr::null_mut(),
        )
    } != 0
    {
        unsafe { perror(c"create tag entry thread".as_ptr()) };
        unsafe { exit(1) };
    }

    unsafe { sleep(test_duration) };
    unsafe {
        TEST_COMPLETE = true;
    }

    i = 0;
    while i < NUM_THREADS as c_int {
        if unsafe { pthread_join(THREADS[i as usize], core::ptr::null_mut()) } != 0 {
            unsafe { perror(c"pthread_join".as_ptr()) };
            unsafe { exit(1) };
        }
        i += 1;
    }

    unsafe { item_kill_tree(&raw mut ARRAY) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
