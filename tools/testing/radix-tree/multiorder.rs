// SPDX-License-Identifier: GPL-2.0-only
/*
 * multiorder.c: Multi-order radix tree entry testing
 * Copyright (c) 2016 Intel Corporation
 * Author: Ross Zwisler <ross.zwisler@linux.intel.com>
 * Author: Matthew Wilcox <matthew.r.wilcox@intel.com>
 */
/* Dependencies from:
 * <linux/radix-tree.h>
 * <linux/slab.h>
 * <linux/errno.h>
 * <pthread.h>
 * "test.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct radix_tree_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xa_node {
    pub shift: c_uint,
}

#[repr(C)]
pub struct xa_state {
    pub xa: *mut xarray,
    pub xa_index: c_ulong,
    pub xa_node: *mut xa_node,
}

#[repr(C)]
pub struct item {
    pub index: c_ulong,
    pub order: c_uint,
}

pub type pthread_t = c_ulong;

const GFP_KERNEL: c_uint = 0;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const XA_MARK_0: c_uint = 0;
const XA_MARK_1: c_uint = 1;
const XA_MARK_2: c_uint = 2;
const XA_CHUNK_SHIFT: c_int = 6;
const RADIX_TREE_MAP_SHIFT: c_uint = 6;
const _SC_NPROCESSORS_ONLN: c_int = 84;

unsafe extern "C" {
    static mut test_verbose: c_int;

    fn item_create(index: c_ulong, order: c_uint) -> *mut item;
    fn free(ptr: *mut c_void);
    fn printv(level: c_int, fmt: *const c_char, ...);

    fn xas_lock(xas: *mut xa_state);
    fn xas_store(xas: *mut xa_state, entry: *mut c_void);
    fn xas_unlock(xas: *mut xa_state);
    fn xas_nomem(xas: *mut xa_state, gfp: c_uint) -> bool;
    fn xas_error(xas: *mut xa_state) -> c_int;
    fn xas_set(xas: *mut xa_state, index: c_ulong);
    fn xas_next(xas: *mut xa_state, max: c_ulong) -> *mut c_void;
    fn xas_next_marked(xas: *mut xa_state, max: c_ulong, mark: c_uint) -> *mut c_void;
    fn xas_retry(xas: *mut xa_state, entry: *mut c_void) -> bool;

    fn item_kill_tree(xa: *mut xarray);
    fn radix_tree_is_internal_node(ptr: *mut c_void) -> bool;
    fn xa_is_internal(ptr: *mut c_void) -> bool;
    fn xa_marked(xa: *mut xarray, mark: c_uint) -> bool;
    fn xa_set_mark(xa: *mut xarray, index: c_ulong, mark: c_uint);
    fn tag_tagged_items(
        xa: *mut xarray,
        start: c_ulong,
        end: c_ulong,
        nr: c_int,
        iftag: c_uint,
        thentag: c_uint,
    ) -> c_int;
    fn item_delete_rcu(xa: *mut xarray, index: c_ulong);
    fn item_sanity(item: *mut item, index: c_ulong);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_register_thread();
    fn rcu_unregister_thread();
    fn sysconf(name: c_int) -> c_long;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn xa_load(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_find(xa: *mut xarray, indexp: *mut c_ulong, max: c_ulong, mark: c_uint) -> *mut c_void;
    fn radix_tree_cpu_dead(cpu: c_uint);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn radix_tree_init();
}

pub type c_long = i64;

static mut stop_iteration: bool = false;

static mut array: xarray = xarray { _private: [] };

unsafe fn XA_STATE_ORDER(xa: *mut xarray, index: c_ulong, _order: c_uint) -> xa_state {
    xa_state {
        xa,
        xa_index: index,
        xa_node: ptr::null_mut(),
    }
}

unsafe fn XA_STATE(xa: *mut xarray, index: c_ulong) -> xa_state {
    xa_state {
        xa,
        xa_index: index,
        xa_node: ptr::null_mut(),
    }
}

unsafe fn item_insert_order(xa: *mut xarray, index: c_ulong, order: c_uint) -> c_int {
    let mut xas = XA_STATE_ORDER(xa, index, order);
    let item = item_create(index, order);

    loop {
        xas_lock(&mut xas);
        xas_store(&mut xas, item as *mut c_void);
        xas_unlock(&mut xas);
        if !xas_nomem(&mut xas, GFP_KERNEL) {
            break;
        }
    }

    if xas_error(&mut xas) == 0 {
        return 0;
    }

    free(item as *mut c_void);
    xas_error(&mut xas)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiorder_iteration(xa: *mut xarray) {
    let mut xas = XA_STATE(xa, 0);
    let mut item_ptr: *mut item;
    let mut i: c_int;
    let mut j: c_int;
    let mut err: c_int;

    const NUM_ENTRIES: usize = 11;
    let index: [c_int; NUM_ENTRIES] = [0, 2, 4, 8, 16, 32, 34, 36, 64, 72, 128];
    let order: [c_int; NUM_ENTRIES] = [1, 1, 2, 3, 4, 1, 0, 1, 3, 0, 7];

    printv(1, c"Multiorder iteration test\n".as_ptr());

    i = 0;
    while i < NUM_ENTRIES as c_int {
        err = item_insert_order(xa, index[i as usize] as c_ulong, order[i as usize] as c_uint);
        assert!(err == 0);
        i += 1;
    }

    j = 0;
    while j < 256 {
        i = 0;
        while i < NUM_ENTRIES as c_int {
            if j <= (index[i as usize] | ((1 << order[i as usize]) - 1)) {
                break;
            }
            i += 1;
        }

        xas_set(&mut xas, j as c_ulong);
        loop {
            item_ptr = xas_next(&mut xas, ULONG_MAX) as *mut item;
            if item_ptr.is_null() {
                break;
            }
            let height: c_int = order[i as usize] / XA_CHUNK_SHIFT;
            let shift: c_int = height * XA_CHUNK_SHIFT;
            let mask: c_ulong = ((1 as c_ulong) << order[i as usize]) - 1;

            assert!((xas.xa_index | mask) == ((index[i as usize] as c_ulong) | mask));
            assert!((*xas.xa_node).shift == shift as c_uint);
            assert!(!radix_tree_is_internal_node(item_ptr as *mut c_void));
            assert!(((*item_ptr).index | mask) == ((index[i as usize] as c_ulong) | mask));
            assert!((*item_ptr).order == order[i as usize] as c_uint);
            i += 1;
        }
        j += 1;
    }

    item_kill_tree(xa);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiorder_tagged_iteration(xa: *mut xarray) {
    let mut xas = XA_STATE(xa, 0);
    let mut item_ptr: *mut item;
    let mut i: c_int;
    let mut j: c_int;

    const MT_NUM_ENTRIES: usize = 9;
    let index: [c_int; MT_NUM_ENTRIES] = [0, 2, 4, 16, 32, 40, 64, 72, 128];
    let order: [c_int; MT_NUM_ENTRIES] = [1, 0, 2, 4, 3, 1, 3, 0, 7];

    const TAG_ENTRIES: usize = 7;
    let tag_index: [c_int; TAG_ENTRIES] = [0, 4, 16, 40, 64, 72, 128];

    printv(1, c"Multiorder tagged iteration test\n".as_ptr());

    i = 0;
    while i < MT_NUM_ENTRIES as c_int {
        assert!(item_insert_order(xa, index[i as usize] as c_ulong, order[i as usize] as c_uint) == 0);
        i += 1;
    }

    assert!(!xa_marked(xa, XA_MARK_1));

    i = 0;
    while i < TAG_ENTRIES as c_int {
        xa_set_mark(xa, tag_index[i as usize] as c_ulong, XA_MARK_1);
        i += 1;
    }

    j = 0;
    while j < 256 {
        let mut k: c_int;

        i = 0;
        while i < TAG_ENTRIES as c_int {
            k = i;
            while index[k as usize] < tag_index[i as usize] {
                k += 1;
            }
            if j <= (index[k as usize] | ((1 << order[k as usize]) - 1)) {
                break;
            }
            i += 1;
        }

        xas_set(&mut xas, j as c_ulong);
        loop {
            item_ptr = xas_next_marked(&mut xas, ULONG_MAX, XA_MARK_1) as *mut item;
            if item_ptr.is_null() {
                break;
            }
            k = i;
            while index[k as usize] < tag_index[i as usize] {
                k += 1;
            }
            let mask: c_ulong = ((1 as c_ulong) << order[k as usize]) - 1;

            assert!((xas.xa_index | mask) == ((tag_index[i as usize] as c_ulong) | mask));
            assert!(!xa_is_internal(item_ptr as *mut c_void));
            assert!(((*item_ptr).index | mask) == ((tag_index[i as usize] as c_ulong) | mask));
            assert!((*item_ptr).order == order[k as usize] as c_uint);
            i += 1;
        }
        j += 1;
    }

    assert!(
        tag_tagged_items(
            xa,
            0,
            ULONG_MAX,
            TAG_ENTRIES as c_int,
            XA_MARK_1,
            XA_MARK_2,
        ) == TAG_ENTRIES as c_int
    );

    j = 0;
    while j < 256 {
        let mut mask: c_int;
        let mut k: c_int;

        i = 0;
        while i < TAG_ENTRIES as c_int {
            k = i;
            while index[k as usize] < tag_index[i as usize] {
                k += 1;
            }
            if j <= (index[k as usize] | ((1 << order[k as usize]) - 1)) {
                break;
            }
            i += 1;
        }

        xas_set(&mut xas, j as c_ulong);
        loop {
            item_ptr = xas_next_marked(&mut xas, ULONG_MAX, XA_MARK_2) as *mut item;
            if item_ptr.is_null() {
                break;
            }
            k = i;
            while index[k as usize] < tag_index[i as usize] {
                k += 1;
            }
            mask = (1 << order[k as usize]) - 1;

            assert!((xas.xa_index | mask as c_ulong) == ((tag_index[i as usize] as c_ulong) | mask as c_ulong));
            assert!(!xa_is_internal(item_ptr as *mut c_void));
            assert!(((*item_ptr).index | mask as c_ulong) == ((tag_index[i as usize] as c_ulong) | mask as c_ulong));
            assert!((*item_ptr).order == order[k as usize] as c_uint);
            i += 1;
        }
        j += 1;
    }

    assert!(
        tag_tagged_items(
            xa,
            1,
            ULONG_MAX,
            (MT_NUM_ENTRIES * 2) as c_int,
            XA_MARK_1,
            XA_MARK_0,
        ) == TAG_ENTRIES as c_int
    );
    i = 0;
    xas_set(&mut xas, 0);
    loop {
        item_ptr = xas_next_marked(&mut xas, ULONG_MAX, XA_MARK_0) as *mut item;
        if item_ptr.is_null() {
            break;
        }
        assert!(xas.xa_index == tag_index[i as usize] as c_ulong);
        i += 1;
    }
    assert!(i == TAG_ENTRIES as c_int);

    item_kill_tree(xa);
}

unsafe extern "C" fn creator_func(ptr: *mut c_void) -> *mut c_void {
    /* 'order' is set up to ensure we have sibling entries */
    let order: c_uint = RADIX_TREE_MAP_SHIFT - 1;
    let tree = ptr as *mut xarray;
    let mut i: c_int;

    i = 0;
    while i < 10000 {
        item_insert_order(tree, 0, order);
        item_delete_rcu(tree, 0);
        i += 1;
    }

    stop_iteration = true;
    ptr::null_mut()
}

unsafe extern "C" fn iterator_func(ptr: *mut c_void) -> *mut c_void {
    let mut xas = XA_STATE(ptr as *mut xarray, 0);
    let mut item_ptr: *mut item;

    while !stop_iteration {
        rcu_read_lock();
        loop {
            item_ptr = xas_next(&mut xas, ULONG_MAX) as *mut item;
            if item_ptr.is_null() {
                break;
            }
            if xas_retry(&mut xas, item_ptr as *mut c_void) {
                continue;
            }

            item_sanity(item_ptr, xas.xa_index);
        }
        rcu_read_unlock();
    }
    ptr::null_mut()
}

unsafe fn multiorder_iteration_race(xa: *mut xarray) {
    let num_threads: c_int = sysconf(_SC_NPROCESSORS_ONLN) as c_int;
    let mut worker_thread: Vec<pthread_t> = vec![0; num_threads as usize];
    let mut i: c_int;

    stop_iteration = false;
    pthread_create(&mut worker_thread[0], ptr::null(), Some(creator_func), xa as *mut c_void);
    i = 1;
    while i < num_threads {
        pthread_create(
            &mut worker_thread[i as usize],
            ptr::null(),
            Some(iterator_func),
            xa as *mut c_void,
        );
        i += 1;
    }

    i = 0;
    while i < num_threads {
        pthread_join(worker_thread[i as usize], ptr::null_mut());
        i += 1;
    }

    item_kill_tree(xa);
}

unsafe extern "C" fn load_creator(ptr: *mut c_void) -> *mut c_void {
    /* 'order' is set up to ensure we have sibling entries */
    let mut order: c_uint;
    let tree = ptr as *mut xarray;
    let mut i: c_int;

    rcu_register_thread();
    item_insert_order(tree, (3 << RADIX_TREE_MAP_SHIFT) as c_ulong, 0);
    item_insert_order(tree, (2 << RADIX_TREE_MAP_SHIFT) as c_ulong, 0);
    i = 0;
    while i < 10000 {
        order = 1;
        while order < RADIX_TREE_MAP_SHIFT {
            let index: c_ulong = ((3 << RADIX_TREE_MAP_SHIFT) - (1 << order)) as c_ulong;
            item_insert_order(tree, index, order);
            xa_set_mark(tree, index, XA_MARK_1);
            item_delete_rcu(tree, index);
            order += 1;
        }
        i += 1;
    }
    rcu_unregister_thread();

    stop_iteration = true;
    ptr::null_mut()
}

unsafe extern "C" fn load_worker(ptr: *mut c_void) -> *mut c_void {
    let index: c_ulong = ((3 << RADIX_TREE_MAP_SHIFT) - 1) as c_ulong;

    rcu_register_thread();
    while !stop_iteration {
        let mut find_index: c_ulong = ((2 << RADIX_TREE_MAP_SHIFT) + 1) as c_ulong;
        let mut item_ptr = xa_load(ptr as *mut xarray, index) as *mut item;
        assert!(!xa_is_internal(item_ptr as *mut c_void));
        item_ptr = xa_find(ptr as *mut xarray, &mut find_index, index, XA_MARK_1) as *mut item;
        assert!(!xa_is_internal(item_ptr as *mut c_void));
    }
    rcu_unregister_thread();

    ptr::null_mut()
}

unsafe fn load_race(xa: *mut xarray) {
    let num_threads: c_int = (sysconf(_SC_NPROCESSORS_ONLN) * 4) as c_int;
    let mut worker_thread: Vec<pthread_t> = vec![0; num_threads as usize];
    let mut i: c_int;

    stop_iteration = false;
    pthread_create(&mut worker_thread[0], ptr::null(), Some(load_creator), xa as *mut c_void);
    i = 1;
    while i < num_threads {
        pthread_create(
            &mut worker_thread[i as usize],
            ptr::null(),
            Some(load_worker),
            xa as *mut c_void,
        );
        i += 1;
    }

    i = 0;
    while i < num_threads {
        pthread_join(worker_thread[i as usize], ptr::null_mut());
        i += 1;
    }

    item_kill_tree(xa);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn multiorder_checks() {
    let array_ptr = ptr::addr_of_mut!(array);
    multiorder_iteration(array_ptr);
    multiorder_tagged_iteration(array_ptr);
    multiorder_iteration_race(array_ptr);
    load_race(array_ptr);

    radix_tree_cpu_dead(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;

    loop {
        opt = getopt(argc, argv, c"ls:v".as_ptr());
        if opt == -1 {
            break;
        }
        if opt == 'v' as c_int {
            test_verbose += 1;
        }
    }

    rcu_register_thread();
    radix_tree_init();
    multiorder_checks();
    rcu_unregister_thread();
    0
}
