// SPDX-License-Identifier: GPL-2.0-only
/*
 * idr-test.c: Test the IDR API
 * Copyright (c) 2016 Matthew Wilcox <willy@infradead.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type u32 = u32;
type time_t = i64;
type pthread_t = c_ulong;

const GFP_KERNEL: c_uint = 0;
const GFP_NOWAIT: c_uint = 0;
const ENOSPC: c_int = 28;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const INT_MAX: c_ulong = 0x7fffffff;
const RADIX_TREE_MAP_SIZE: c_ulong = 64;
const IDA_BITMAP_BITS: c_ulong = 1024;
const BITS_PER_XA_VALUE: c_ulong = 63;
const DUMMY_PTR: *mut c_void = 0x10 as *mut c_void;

#[repr(C)]
pub struct idr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct item {
    pub index: c_ulong,
}

unsafe extern "C" {
    static mut find_idr: idr;
    static mut nr_allocated: c_int;

    fn free(ptr: *mut c_void);
    fn abort() -> !;
    fn exit(status: c_int) -> !;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn rand() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn time(tloc: *mut time_t) -> time_t;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn rcu_register_thread();
    fn rcu_unregister_thread();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_barrier();
    fn radix_tree_init();
    fn radix_tree_cpu_dead(cpu: c_int);

    fn item_create(index: c_ulong, order: c_uint) -> *mut item;
    fn idr_alloc(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_uint) -> c_int;
    fn idr_alloc_cyclic(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: c_uint) -> c_int;
    fn idr_alloc_u32(idr: *mut idr, ptr: *mut c_void, nextid: *mut u32, max: u32, gfp: c_uint) -> c_int;
    fn idr_destroy(idr: *mut idr);
    fn idr_find(idr: *mut idr, id: c_int) -> *mut c_void;
    fn idr_for_each(
        idr: *mut idr,
        fn_: unsafe extern "C" fn(c_int, *mut c_void, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn idr_get_next(idr: *mut idr, nextid: *mut c_int) -> *mut c_void;
    fn idr_init_base(idr: *mut idr, base: c_int);
    fn idr_is_empty(idr: *mut idr) -> bool_;
    fn idr_remove(idr: *mut idr, id: c_int) -> *mut c_void;
    fn idr_replace(idr: *mut idr, ptr: *mut c_void, id: c_int) -> *mut c_void;
    fn idr_set_cursor(idr: *mut idr, cursor: c_uint);

    fn ida_alloc(ida: *mut ida, gfp: c_uint) -> c_int;
    fn ida_alloc_max(ida: *mut ida, max: c_uint, gfp: c_uint) -> c_int;
    fn ida_alloc_min(ida: *mut ida, min: c_uint, gfp: c_uint) -> c_int;
    fn ida_alloc_range(ida: *mut ida, min: c_uint, max: c_uint, gfp: c_uint) -> c_int;
    fn ida_checks();
    fn ida_destroy(ida: *mut ida);
    fn ida_dump(ida: *mut ida);
    fn ida_exit();
    fn ida_free(ida: *mut ida, id: c_uint);
    fn ida_is_empty(ida: *mut ida) -> bool_;

    fn test_bit(bit: c_int, bitmap: *mut c_ulong) -> bool_;
    fn __clear_bit(bit: c_int, bitmap: *mut c_ulong);
    fn __set_bit(bit: c_int, bitmap: *mut c_ulong);
}

unsafe fn ERR_PTR(err: c_int) -> *mut c_void {
    err as isize as *mut c_void
}

unsafe fn xa_mk_value(id: c_int) -> *mut c_void {
    (((id as usize) << 1) | 1) as *mut c_void
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        abort();
    }
}

unsafe fn IDA_BUG_ON(_ida: *mut ida, cond: bool) {
    BUG_ON(cond);
}

#[inline]
unsafe fn new_idr() -> idr {
    core::mem::zeroed()
}

#[inline]
unsafe fn new_ida() -> ida {
    core::mem::zeroed()
}

pub unsafe extern "C" fn item_idr_free(id: c_int, p: *mut c_void, _data: *mut c_void) -> c_int {
    let item = p as *mut item;
    assert!((*item).index == id as c_ulong);
    free(p);

    0
}

pub unsafe extern "C" fn item_idr_remove(idr: *mut idr, id: c_int) {
    let item = idr_find(idr, id) as *mut item;
    assert!((*item).index == id as c_ulong);
    idr_remove(idr, id);
    free(item as *mut c_void);
}

pub unsafe extern "C" fn idr_alloc_test() {
    let mut i: c_ulong;
    let mut idr = new_idr();

    assert!(idr_alloc_cyclic(&mut idr, DUMMY_PTR, 0, 0x4000, GFP_KERNEL) == 0);
    assert!(idr_alloc_cyclic(&mut idr, DUMMY_PTR, 0x3ffd, 0x4000, GFP_KERNEL) == 0x3ffd);
    idr_remove(&mut idr, 0x3ffd);
    idr_remove(&mut idr, 0);

    i = 0x3ffe;
    while i < 0x4003 {
        let id: c_int;
        let item: *mut item;

        if i < 0x4000 {
            item = item_create(i, 0);
        } else {
            item = item_create(i - 0x3fff, 0);
        }

        id = idr_alloc_cyclic(&mut idr, item as *mut c_void, 1, 0x4000, GFP_KERNEL);
        assert!(id as c_ulong == (*item).index);
        i += 1;
    }

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);
}

pub unsafe extern "C" fn idr_alloc2_test() {
    let mut id: c_int;
    let mut idr = new_idr();
    idr_init_base(&mut idr, 1);

    id = idr_alloc(&mut idr, idr_alloc2_test as *mut c_void, 0, 1, GFP_KERNEL);
    assert!(id == -ENOSPC);

    id = idr_alloc(&mut idr, idr_alloc2_test as *mut c_void, 1, 2, GFP_KERNEL);
    assert!(id == 1);

    id = idr_alloc(&mut idr, idr_alloc2_test as *mut c_void, 0, 1, GFP_KERNEL);
    assert!(id == -ENOSPC);

    id = idr_alloc(&mut idr, idr_alloc2_test as *mut c_void, 0, 2, GFP_KERNEL);
    assert!(id == -ENOSPC);

    idr_destroy(&mut idr);
}

pub unsafe extern "C" fn idr_replace_test() {
    let mut idr = new_idr();

    idr_alloc(&mut idr, -1isize as *mut c_void, 10, 11, GFP_KERNEL);
    idr_replace(&mut idr, &mut idr as *mut _ as *mut c_void, 10);

    idr_destroy(&mut idr);
}

/*
 * Unlike the radix tree, you can put a NULL pointer -- with care -- into
 * the IDR.  Some interfaces, like idr_find() do not distinguish between
 * "present, value is NULL" and "not present", but that's exactly what some
 * users want.
 */
pub unsafe extern "C" fn idr_null_test() {
    let mut i: c_int;
    let mut idr = new_idr();

    assert!(idr_is_empty(&mut idr));

    assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert!(!idr_is_empty(&mut idr));
    idr_remove(&mut idr, 0);
    assert!(idr_is_empty(&mut idr));

    assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert!(!idr_is_empty(&mut idr));
    idr_destroy(&mut idr);
    assert!(idr_is_empty(&mut idr));

    i = 0;
    while i < 10 {
        assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == i);
        i += 1;
    }

    assert!(idr_replace(&mut idr, DUMMY_PTR, 3).is_null());
    assert!(idr_replace(&mut idr, DUMMY_PTR, 4).is_null());
    assert!(idr_replace(&mut idr, core::ptr::null_mut(), 4) == DUMMY_PTR);
    assert!(idr_replace(&mut idr, DUMMY_PTR, 11) == ERR_PTR(-ENOENT));
    idr_remove(&mut idr, 5);
    assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 5);
    idr_remove(&mut idr, 5);

    i = 0;
    while i < 9 {
        idr_remove(&mut idr, i);
        assert!(!idr_is_empty(&mut idr));
        i += 1;
    }
    idr_remove(&mut idr, 8);
    assert!(!idr_is_empty(&mut idr));
    idr_remove(&mut idr, 9);
    assert!(idr_is_empty(&mut idr));

    assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) == 0);
    assert!(idr_replace(&mut idr, DUMMY_PTR, 3) == ERR_PTR(-ENOENT));
    assert!(idr_replace(&mut idr, DUMMY_PTR, 0).is_null());
    assert!(idr_replace(&mut idr, core::ptr::null_mut(), 0) == DUMMY_PTR);

    idr_destroy(&mut idr);
    assert!(idr_is_empty(&mut idr));

    i = 1;
    while i < 10 {
        assert!(idr_alloc(&mut idr, core::ptr::null_mut(), 1, 0, GFP_KERNEL) == i);
        i += 1;
    }

    idr_destroy(&mut idr);
    assert!(idr_is_empty(&mut idr));
}

pub unsafe extern "C" fn idr_nowait_test() {
    let mut i: c_uint;
    let mut idr = new_idr();

    idr_preload(GFP_KERNEL);

    i = 0;
    while i < 3 {
        let item = item_create(i as c_ulong, 0);
        assert!(idr_alloc(&mut idr, item as *mut c_void, i as c_int, (i + 1) as c_int, GFP_NOWAIT) == i as c_int);
        i += 1;
    }

    idr_preload_end();

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);
}

unsafe extern "C" {
    fn idr_preload(gfp: c_uint);
    fn idr_preload_end();
}

pub unsafe extern "C" fn idr_get_next_test(base: c_int) {
    let mut i: c_ulong;
    let mut nextid: c_int;
    let mut idr = new_idr();
    idr_init_base(&mut idr, base);

    let indices: [c_int; 9] = [4, 7, 9, 15, 65, 128, 1000, 99999, 0];

    i = 0;
    while indices[i as usize] != 0 {
        let item = item_create(indices[i as usize] as c_ulong, 0);
        assert!(idr_alloc(&mut idr, item as *mut c_void, indices[i as usize], indices[i as usize + 1], GFP_KERNEL) == indices[i as usize]);
        i += 1;
    }

    i = 0;
    nextid = 0;
    while indices[i as usize] != 0 {
        idr_get_next(&mut idr, &mut nextid);
        assert!(nextid == indices[i as usize]);
        nextid += 1;
        i += 1;
    }

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);
}

pub unsafe extern "C" fn idr_u32_cb(id: c_int, ptr: *mut c_void, _data: *mut c_void) -> c_int {
    BUG_ON(id < 0);
    BUG_ON(ptr != DUMMY_PTR);
    0
}

pub unsafe extern "C" fn idr_u32_test1(idr: *mut idr, handle: u32) {
    static mut warned: bool_ = false;
    let mut id: u32 = handle;
    let mut sid: c_int = 0;
    let ptr: *mut c_void;

    BUG_ON(idr_alloc_u32(idr, DUMMY_PTR, &mut id, id, GFP_KERNEL) != 0);
    BUG_ON(id != handle);
    BUG_ON(idr_alloc_u32(idr, DUMMY_PTR, &mut id, id, GFP_KERNEL) != -ENOSPC);
    BUG_ON(id != handle);
    if !warned && id > INT_MAX as u32 {
        printk(c"vvv Ignore these warnings\n".as_ptr());
    }
    ptr = idr_get_next(idr, &mut sid);
    if id > INT_MAX as u32 {
        BUG_ON(!ptr.is_null());
        BUG_ON(sid != 0);
    } else {
        BUG_ON(ptr != DUMMY_PTR);
        BUG_ON(sid != id as c_int);
    }
    idr_for_each(idr, idr_u32_cb, core::ptr::null_mut());
    if !warned && id > INT_MAX as u32 {
        printk(c"^^^ Warnings over\n".as_ptr());
        warned = true;
    }
    BUG_ON(idr_remove(idr, id as c_int) != DUMMY_PTR);
    BUG_ON(!idr_is_empty(idr));
}

pub unsafe extern "C" fn idr_u32_test(base: c_int) {
    let mut idr = new_idr();
    idr_init_base(&mut idr, base);
    idr_u32_test1(&mut idr, 10);
    idr_u32_test1(&mut idr, 0x7fffffff);
    idr_u32_test1(&mut idr, 0x80000000);
    idr_u32_test1(&mut idr, 0x80000001);
    idr_u32_test1(&mut idr, 0xffe00000);
    idr_u32_test1(&mut idr, 0xffffffff);
}

unsafe extern "C" fn idr_align_test(idr: *mut idr) {
    let mut name = *b"Motorola 68000\0";
    let mut i: c_int;
    let mut id: c_int = 0;
    let mut entry: *mut c_void = core::ptr::null_mut();

    i = 0;
    while i < 9 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != i);
        idr_for_each_entry(idr, &mut entry, &mut id);
        i += 1;
    }
    idr_destroy(idr);

    i = 1;
    while i < 10 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != i - 1);
        idr_for_each_entry(idr, &mut entry, &mut id);
        i += 1;
    }
    idr_destroy(idr);

    i = 2;
    while i < 11 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != i - 2);
        idr_for_each_entry(idr, &mut entry, &mut id);
        i += 1;
    }
    idr_destroy(idr);

    i = 3;
    while i < 12 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != i - 3);
        idr_for_each_entry(idr, &mut entry, &mut id);
        i += 1;
    }
    idr_destroy(idr);

    i = 0;
    while i < 8 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != 0);
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add((i + 1) as usize) as *mut c_void, 0, 0, GFP_KERNEL) != 1);
        idr_for_each_entry(idr, &mut entry, &mut id);
        idr_remove(idr, 1);
        idr_for_each_entry(idr, &mut entry, &mut id);
        idr_remove(idr, 0);
        BUG_ON(!idr_is_empty(idr));
        i += 1;
    }

    i = 0;
    while i < 8 {
        BUG_ON(idr_alloc(idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) != 0);
        idr_for_each_entry(idr, &mut entry, &mut id);
        idr_replace(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0);
        idr_for_each_entry(idr, &mut entry, &mut id);
        BUG_ON(idr_find(idr, 0) != name.as_mut_ptr().add(i as usize) as *mut c_void);
        idr_remove(idr, 0);
        i += 1;
    }

    i = 0;
    while i < 8 {
        BUG_ON(idr_alloc(idr, name.as_mut_ptr().add(i as usize) as *mut c_void, 0, 0, GFP_KERNEL) != 0);
        BUG_ON(idr_alloc(idr, core::ptr::null_mut(), 0, 0, GFP_KERNEL) != 1);
        idr_remove(idr, 1);
        idr_for_each_entry(idr, &mut entry, &mut id);
        idr_replace(idr, name.as_mut_ptr().add((i + 1) as usize) as *mut c_void, 0);
        idr_for_each_entry(idr, &mut entry, &mut id);
        idr_remove(idr, 0);
        i += 1;
    }
}

unsafe fn idr_for_each_entry(idr: *mut idr, _entry: *mut *mut c_void, _id: *mut c_int) {
    /* Macro-only iteration side effects are supplied by the future dependency. */
    let _ = idr;
}

unsafe extern "C" fn idr_throbber(arg: *mut c_void) -> *mut c_void {
    let start: time_t = time(core::ptr::null_mut());
    let id: c_int = *(arg as *mut c_int);

    rcu_register_thread();
    loop {
        idr_alloc(&mut find_idr, xa_mk_value(id), id, id + 1, GFP_KERNEL);
        idr_remove(&mut find_idr, id);
        if !(time(core::ptr::null_mut()) < start + 10) {
            break;
        }
    }
    rcu_unregister_thread();

    core::ptr::null_mut()
}

/*
 * There are always either 1 or 2 objects in the IDR.  If we find nothing,
 * or we find something at an ID we didn't expect, that's a bug.
 */
pub unsafe extern "C" fn idr_find_test_1(anchor_id: c_int, mut throbber_id: c_int) {
    let mut throbber: pthread_t = 0;
    let start: time_t = time(core::ptr::null_mut());

    BUG_ON(idr_alloc(&mut find_idr, xa_mk_value(anchor_id), anchor_id, anchor_id + 1, GFP_KERNEL) != anchor_id);

    pthread_create(&mut throbber, core::ptr::null(), idr_throbber, &mut throbber_id as *mut _ as *mut c_void);

    rcu_read_lock();
    loop {
        let mut id: c_int = 0;
        let entry = idr_get_next(&mut find_idr, &mut id);
        rcu_read_unlock();
        if (id != anchor_id && id != throbber_id) || entry != xa_mk_value(id) {
            printf(c"%s(%d, %d): %p at %d\n".as_ptr(), c"idr_find_test_1".as_ptr(), anchor_id, throbber_id, entry, id);
            abort();
        }
        rcu_read_lock();
        if !(time(core::ptr::null_mut()) < start + 11) {
            break;
        }
    }
    rcu_read_unlock();

    pthread_join(throbber, core::ptr::null_mut());

    idr_remove(&mut find_idr, anchor_id);
    BUG_ON(!idr_is_empty(&mut find_idr));
}

pub unsafe extern "C" fn idr_find_test() {
    idr_find_test_1(100000, 0);
    idr_find_test_1(0, 100000);
}

pub unsafe extern "C" fn idr_checks() {
    let mut i: c_ulong;
    let mut idr = new_idr();

    i = 0;
    while i < 10000 {
        let item = item_create(i, 0);
        assert!(idr_alloc(&mut idr, item as *mut c_void, 0, 20000, GFP_KERNEL) == i as c_int);
        i += 1;
    }

    assert!(idr_alloc(&mut idr, DUMMY_PTR, 5, 30, GFP_KERNEL) < 0);

    i = 0;
    while i < 5000 {
        item_idr_remove(&mut idr, i as c_int);
        i += 1;
    }

    idr_remove(&mut idr, 3);

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);

    assert!(idr_is_empty(&mut idr));

    idr_remove(&mut idr, 3);
    idr_remove(&mut idr, 0);

    assert!(idr_alloc(&mut idr, DUMMY_PTR, 0, 0, GFP_KERNEL) == 0);
    idr_remove(&mut idr, 1);
    i = 1;
    while i < RADIX_TREE_MAP_SIZE {
        assert!(idr_alloc(&mut idr, DUMMY_PTR, 0, 0, GFP_KERNEL) == i as c_int);
        i += 1;
    }
    idr_remove(&mut idr, 1 << 30);
    idr_destroy(&mut idr);

    i = INT_MAX - 3;
    while i < INT_MAX + 1 {
        let item = item_create(i, 0);
        assert!(idr_alloc(&mut idr, item as *mut c_void, i as c_int, (i + 10) as c_int, GFP_KERNEL) == i as c_int);
        i += 1;
    }
    assert!(idr_alloc(&mut idr, DUMMY_PTR, (i - 2) as c_int, i as c_int, GFP_KERNEL) == -ENOSPC);
    assert!(idr_alloc(&mut idr, DUMMY_PTR, (i - 2) as c_int, (i + 10) as c_int, GFP_KERNEL) == -ENOSPC);

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);
    idr_destroy(&mut idr);

    assert!(idr_is_empty(&mut idr));

    idr_set_cursor(&mut idr, (INT_MAX - 3) as c_uint);
    i = INT_MAX - 3;
    while i < INT_MAX + 3 {
        let item: *mut item;
        let id: c_uint;
        if i <= INT_MAX {
            item = item_create(i, 0);
        } else {
            item = item_create(i - INT_MAX - 1, 0);
        }

        id = idr_alloc_cyclic(&mut idr, item as *mut c_void, 0, 0, GFP_KERNEL) as c_uint;
        assert!(id as c_ulong == (*item).index);
        i += 1;
    }

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);
    assert!(idr_is_empty(&mut idr));

    i = 1;
    while i < 10000 {
        let item = item_create(i, 0);
        assert!(idr_alloc(&mut idr, item as *mut c_void, 1, 20000, GFP_KERNEL) == i as c_int);
        i += 1;
    }

    idr_for_each(&mut idr, item_idr_free, &mut idr as *mut _ as *mut c_void);
    idr_destroy(&mut idr);

    idr_replace_test();
    idr_alloc_test();
    idr_alloc2_test();
    idr_null_test();
    idr_nowait_test();
    idr_get_next_test(0);
    idr_get_next_test(1);
    idr_get_next_test(4);
    idr_u32_test(4);
    idr_u32_test(1);
    idr_u32_test(0);
    idr_align_test(&mut idr);
    idr_find_test();
}

/* module_init/module_exit/MODULE_* macros are C-only no-ops here.
 * dump_stack() mapped to assert(0) in the source.
 * The source includes ../../../lib/test_ida.c; its declarations are represented above.
 */

/*
 * Check that we get the correct error when we run out of memory doing
 * allocations.  In userspace, GFP_NOWAIT will always fail an allocation.
 * The first test is for not having a bitmap available, and the second test
 * is for not being able to allocate a level of the radix tree.
 */
pub unsafe extern "C" fn ida_check_nomem() {
    let mut ida = new_ida();
    let mut id: c_int;

    id = ida_alloc_min(&mut ida, 256, GFP_NOWAIT);
    IDA_BUG_ON(&mut ida, id != -ENOMEM);
    id = ida_alloc_min(&mut ida, (1u64 << 30) as c_uint, GFP_NOWAIT);
    IDA_BUG_ON(&mut ida, id != -ENOMEM);
    IDA_BUG_ON(&mut ida, !ida_is_empty(&mut ida));
}

/*
 * Check handling of conversions between exceptional entries and full bitmaps.
 */
pub unsafe extern "C" fn ida_check_conv_user() {
    let mut ida = new_ida();
    let mut i: c_ulong;

    i = 0;
    while i < 1000000 {
        let mut id = ida_alloc(&mut ida, GFP_NOWAIT);
        if id == -ENOMEM {
            IDA_BUG_ON(&mut ida, ((i % IDA_BITMAP_BITS) != BITS_PER_XA_VALUE) && ((i % IDA_BITMAP_BITS) != 0));
            id = ida_alloc(&mut ida, GFP_KERNEL);
        } else {
            IDA_BUG_ON(&mut ida, (i % IDA_BITMAP_BITS) == BITS_PER_XA_VALUE);
        }
        IDA_BUG_ON(&mut ida, id != i as c_int);
        i += 1;
    }
    ida_destroy(&mut ida);
}

pub unsafe extern "C" fn ida_check_random() {
    let mut ida = new_ida();
    let mut bitmap: [c_ulong; 2048 / (core::mem::size_of::<c_ulong>() * 8)] = [0; 2048 / (core::mem::size_of::<c_ulong>() * 8)];
    let mut i: c_uint;
    let s: time_t = time(core::ptr::null_mut());

    loop {
        memset(bitmap.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&bitmap));
        i = 0;
        while i < 100000 {
            let i_rand: c_int = rand();
            let bit: c_int = i_rand & 2047;
            if test_bit(bit, bitmap.as_mut_ptr()) {
                __clear_bit(bit, bitmap.as_mut_ptr());
                ida_free(&mut ida, bit as c_uint);
            } else {
                __set_bit(bit, bitmap.as_mut_ptr());
                IDA_BUG_ON(&mut ida, ida_alloc_min(&mut ida, bit as c_uint, GFP_KERNEL) != bit);
            }
            i += 1;
        }
        ida_destroy(&mut ida);
        if !(time(core::ptr::null_mut()) < s + 10) {
            break;
        }
    }
}

pub unsafe extern "C" fn ida_alloc_free_test() {
    let mut ida = new_ida();
    let mut i: c_ulong;

    i = 0;
    while i < 10000 {
        assert!(ida_alloc_max(&mut ida, 20000, GFP_KERNEL) == i as c_int);
        i += 1;
    }
    assert!(ida_alloc_range(&mut ida, 5, 30, GFP_KERNEL) < 0);

    i = 0;
    while i < 10000 {
        ida_free(&mut ida, i as c_uint);
        i += 1;
    }
    assert!(ida_is_empty(&mut ida));

    ida_destroy(&mut ida);
}

pub unsafe extern "C" fn user_ida_checks() {
    radix_tree_cpu_dead(1);

    ida_check_nomem();
    ida_check_conv_user();
    ida_check_random();
    ida_alloc_free_test();

    radix_tree_cpu_dead(1);
}

unsafe extern "C" fn ida_random_fn(_arg: *mut c_void) -> *mut c_void {
    rcu_register_thread();
    ida_check_random();
    rcu_unregister_thread();
    core::ptr::null_mut()
}

unsafe extern "C" fn ida_leak_fn(arg: *mut c_void) -> *mut c_void {
    let ida = arg as *mut ida;
    let s: time_t = time(core::ptr::null_mut());
    let mut i: c_int;
    let mut ret: c_int;

    rcu_register_thread();

    loop {
        i = 0;
        while i < 1000 {
            ret = ida_alloc_range(ida, 128, 128, GFP_KERNEL);
            if ret >= 0 {
                ida_free(ida, 128);
            }
            i += 1;
        }
        if !(time(core::ptr::null_mut()) < s + 2) {
            break;
        }
    }

    rcu_unregister_thread();
    core::ptr::null_mut()
}

pub unsafe extern "C" fn ida_thread_tests() {
    let mut ida = new_ida();
    let mut threads: [pthread_t; 20] = [0; 20];
    let mut i: c_int;

    i = 0;
    while i < threads.len() as c_int {
        if pthread_create(&mut threads[i as usize], core::ptr::null(), ida_random_fn, core::ptr::null_mut()) != 0 {
            perror(c"creating ida thread".as_ptr());
            exit(1);
        }
        i += 1;
    }

    while {
        i -= 1;
        i >= 0
    } {
        pthread_join(threads[i as usize], core::ptr::null_mut());
    }

    i = 0;
    while i < threads.len() as c_int {
        if pthread_create(&mut threads[i as usize], core::ptr::null(), ida_leak_fn, &mut ida as *mut _ as *mut c_void) != 0 {
            perror(c"creating ida thread".as_ptr());
            exit(1);
        }
        i += 1;
    }

    while {
        i -= 1;
        i >= 0
    } {
        pthread_join(threads[i as usize], core::ptr::null_mut());
    }
    assert!(ida_is_empty(&mut ida));
}

pub unsafe extern "C" fn ida_tests() {
    user_ida_checks();
    ida_checks();
    ida_exit();
    ida_thread_tests();
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    rcu_register_thread();
    radix_tree_init();
    idr_checks();
    ida_tests();
    radix_tree_cpu_dead(1);
    rcu_barrier();
    if nr_allocated != 0 {
        printf(c"nr_allocated = %d\n".as_ptr(), nr_allocated);
    }
    rcu_unregister_thread();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
