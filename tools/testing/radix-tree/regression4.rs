// SPDX-License-Identifier: GPL-2.0
// Translated from testing/radix-tree/regression4.c.
// C header dependencies:
// linux/kernel.h, linux/gfp.h, linux/slab.h, linux/radix-tree.h,
// linux/rcupdate.h, stdlib.h, pthread.h, stdio.h, assert.h, regression.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [u8; 0],
}

pub type pthread_t = c_ulong;
pub type gfp_t = c_uint;

#[repr(C)]
pub struct radix_tree_root {
    pub height: c_uint,
    pub gfp_mask: gfp_t,
    pub rnode: *mut c_void,
}

// Equivalent intent to C's static RADIX_TREE(mt_tree, GFP_KERNEL).
// The exact GFP_KERNEL value is supplied by the translated linux/gfp.h dependency.
extern "C" {
    static GFP_KERNEL: gfp_t;
}

static mut worker_barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };
static mut obj0: c_int = 0;
static mut obj1: c_int = 0;
static mut mt_tree: radix_tree_root = radix_tree_root {
    height: 0,
    gfp_mask: 0,
    rnode: core::ptr::null_mut(),
};

extern "C" {
    fn rcu_register_thread();
    fn rcu_unregister_thread();
    fn rcu_read_lock();
    fn rcu_read_unlock();

    fn radix_tree_lookup(root: *mut radix_tree_root, index: c_ulong) -> *mut c_void;
    fn radix_tree_insert(root: *mut radix_tree_root, index: c_ulong, item: *mut c_void) -> c_int;
    fn radix_tree_delete(root: *mut radix_tree_root, index: c_ulong) -> *mut c_void;

    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn abort() -> !;
    fn exit(status: c_int) -> !;
    fn printv(level: c_int, format: *const c_char, ...) -> c_int;
}

unsafe extern "C" fn reader_fn(_arg: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let mut entry: *mut c_void;

    rcu_register_thread();
    pthread_barrier_wait(&mut worker_barrier);

    i = 0;
    while i < 1000000 {
        rcu_read_lock();
        entry = radix_tree_lookup(&mut mt_tree, 0);
        rcu_read_unlock();
        if entry != (&mut obj0 as *mut c_int).cast::<c_void>() {
            printf(
                b"iteration %d bad entry = %p\n\0".as_ptr().cast::<c_char>(),
                i,
                entry,
            );
            abort();
        }
        i += 1;
    }

    rcu_unregister_thread();

    core::ptr::null_mut()
}

unsafe extern "C" fn writer_fn(_arg: *mut c_void) -> *mut c_void {
    let mut i: c_int;

    rcu_register_thread();
    pthread_barrier_wait(&mut worker_barrier);

    i = 0;
    while i < 1000000 {
        radix_tree_insert(&mut mt_tree, 1, (&mut obj1 as *mut c_int).cast::<c_void>());
        radix_tree_delete(&mut mt_tree, 1);
        i += 1;
    }

    rcu_unregister_thread();

    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn regression4_test() {
    let mut reader: pthread_t = 0;
    let mut writer: pthread_t = 0;

    printv(1, b"regression test 4 starting\n\0".as_ptr().cast::<c_char>());

    mt_tree.gfp_mask = GFP_KERNEL;
    radix_tree_insert(&mut mt_tree, 0, (&mut obj0 as *mut c_int).cast::<c_void>());
    pthread_barrier_init(&mut worker_barrier, core::ptr::null(), 2);

    if pthread_create(
        &mut reader,
        core::ptr::null(),
        reader_fn,
        core::ptr::null_mut(),
    ) != 0
        || pthread_create(
            &mut writer,
            core::ptr::null(),
            writer_fn,
            core::ptr::null_mut(),
        ) != 0
    {
        perror(b"pthread_create\0".as_ptr().cast::<c_char>());
        exit(1);
    }

    if pthread_join(reader, core::ptr::null_mut()) != 0
        || pthread_join(writer, core::ptr::null_mut()) != 0
    {
        perror(b"pthread_join\0".as_ptr().cast::<c_char>());
        exit(1);
    }

    printv(1, b"regression test 4 passed\n\0".as_ptr().cast::<c_char>());
}
