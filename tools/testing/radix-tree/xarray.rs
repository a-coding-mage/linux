// SPDX-License-Identifier: GPL-2.0+
/*
 * xarray.c: Userspace shim for XArray test-suite
 * Copyright (c) 2018 Matthew Wilcox <willy@infradead.org>
 */

// Dependencies from:
// #include "xarray-shared.h"
// #include "test.h"
//
// XA_DEBUG is undefined before including "../../../lib/test_xarray.c" in the
// original C source; the included implementation is an external dependency for
// this isolated translation.

unsafe extern "C" {
    fn xarray_checks();
    fn xarray_exit();
    fn rcu_register_thread();
    fn radix_tree_init();
    fn radix_tree_cpu_dead(cpu: i32);
    fn rcu_barrier();
    fn printf(format: *const ::std::os::raw::c_char, ...) -> i32;
    fn rcu_unregister_thread();

    static mut nr_allocated: i32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xarray_tests() {
    unsafe {
        xarray_checks();
        xarray_exit();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    unsafe {
        rcu_register_thread();
        radix_tree_init();
        xarray_tests();
        radix_tree_cpu_dead(1);
        rcu_barrier();
        if nr_allocated != 0 {
            printf(c"nr_allocated = %d\n".as_ptr(), nr_allocated);
        }
        rcu_unregister_thread();
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
