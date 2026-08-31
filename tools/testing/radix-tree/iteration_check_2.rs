// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * iteration_check_2.c: Check that deleting a tagged entry doesn't cause
 * an RCU walker to finish early.
 * Copyright (c) 2020 Oracle
 * Author: Matthew Wilcox <willy@infradead.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type pthread_t = c_ulong;

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xa_state {
    pub xa_index: c_ulong,
}

pub const ULONG_MAX: c_ulong = c_ulong::MAX;

unsafe extern "C" {
    static XA_MARK_0: c_uint;
    static GFP_KERNEL: c_uint;

    fn rcu_register_thread();
    fn rcu_unregister_thread();
    fn rcu_read_lock();
    fn rcu_read_unlock();

    fn xas_set(xas: *mut xa_state, index: c_ulong);
    fn xas_for_each_marked(
        xas: *mut xa_state,
        entry: *mut *mut c_void,
        max: c_ulong,
        mark: c_uint,
    ) -> bool;

    fn xa_store(xa: *mut xarray, index: c_ulong, entry: *mut c_void, gfp: c_uint) -> *mut c_void;
    fn xa_set_mark(xa: *mut xarray, index: c_ulong, mark: c_uint);
    fn xa_erase(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_destroy(xa: *mut xarray);
    fn xa_mk_value(entry: c_ulong) -> *mut c_void;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn sleep(seconds: c_uint) -> c_uint;
    fn printv(level: c_int, format: *const c_char, ...);
}

static mut test_complete: bool = false;

unsafe extern "C" fn iterator(arg: *mut c_void) -> *mut c_void {
    /* C dependency: XA_STATE(xas, arg, 0). */
    let mut xas = xa_state { xa_index: 0 };
    let mut entry: *mut c_void = ptr::null_mut();

    unsafe {
        rcu_register_thread();

        while !ptr::addr_of!(test_complete).read_volatile() {
            xas_set(&mut xas, 0);
            rcu_read_lock();
            while xas_for_each_marked(&mut xas, &mut entry, ULONG_MAX, XA_MARK_0) {}
            rcu_read_unlock();
            assert!(xas.xa_index >= 100);
        }

        rcu_unregister_thread();
    }
    ptr::null_mut()
}

unsafe extern "C" fn throbber(arg: *mut c_void) -> *mut c_void {
    let xa = arg as *mut xarray;

    unsafe {
        rcu_register_thread();

        while !ptr::addr_of!(test_complete).read_volatile() {
            let mut i: c_int;

            i = 0;
            while i < 100 {
                xa_store(xa, i as c_ulong, xa_mk_value(i as c_ulong), GFP_KERNEL);
                xa_set_mark(xa, i as c_ulong, XA_MARK_0);
                i += 1;
            }
            i = 0;
            while i < 100 {
                xa_erase(xa, i as c_ulong);
                i += 1;
            }
        }

        rcu_unregister_thread();
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iteration_test2(test_duration: c_uint) {
    let mut threads: [pthread_t; 2] = [0; 2];
    /* C dependency: DEFINE_XARRAY(array). */
    let mut array: xarray = xarray { _private: [] };
    let mut i: c_int;

    unsafe {
        printv(
            1,
            c"Running iteration test 2 for %d seconds\n".as_ptr(),
            test_duration,
        );

        ptr::addr_of_mut!(test_complete).write_volatile(false);

        xa_store(
            &mut array,
            100,
            xa_mk_value(100),
            GFP_KERNEL,
        );
        xa_set_mark(&mut array, 100, XA_MARK_0);

        if pthread_create(
            &mut threads[0],
            ptr::null(),
            iterator,
            &mut array as *mut xarray as *mut c_void,
        ) != 0
        {
            perror(c"create iterator thread".as_ptr());
            exit(1);
        }
        if pthread_create(
            &mut threads[1],
            ptr::null(),
            throbber,
            &mut array as *mut xarray as *mut c_void,
        ) != 0
        {
            perror(c"create throbber thread".as_ptr());
            exit(1);
        }

        sleep(test_duration);
        ptr::addr_of_mut!(test_complete).write_volatile(true);

        i = 0;
        while i < 2 {
            if pthread_join(threads[i as usize], ptr::null_mut()) != 0 {
                perror(c"pthread_join".as_ptr());
                exit(1);
            }
            i += 1;
        }

        xa_destroy(&mut array);
    }
}
