// SPDX-License-Identifier: GPL-2.0-only
/*
 * snapshot.c    Ceph snapshot context utility routines (part of libceph)
 *
 * Copyright (C) 2013 Inktank Storage, Inc.
 */

use core::ffi::c_void;

pub type u32 = core::ffi::c_uint;
pub type gfp_t = usize;

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ceph_snap_context {
    pub nref: refcount_t,
    pub num_snaps: u32,
    pub seq: u64,
    pub snaps: [u64; 0],
}

extern "C" {
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn refcount_set(r: *mut refcount_t, value: i32);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
}

/*
 * Ceph snapshot contexts are reference counted objects, and the
 * returned structure holds a single reference.  Acquire additional
 * references with ceph_get_snap_context(), and release them with
 * ceph_put_snap_context().  When the reference count reaches zero
 * the entire structure is freed.
 */

/*
 * Create a new ceph snapshot context large enough to hold the
 * indicated number of snapshot ids (which can be 0).  Caller has
 * to fill in snapc->seq and snapc->snaps[0..snap_count-1].
 *
 * Returns a null pointer if an error occurs.
 */
#[no_mangle]
pub unsafe extern "C" fn ceph_create_snap_context(
    snap_count: u32,
    gfp_flags: gfp_t,
) -> *mut ceph_snap_context {
    let mut snapc: *mut ceph_snap_context;
    let mut size: usize;

    size = core::mem::size_of::<ceph_snap_context>();
    size += (snap_count as usize) * core::mem::size_of::<u64>();
    snapc = kzalloc(size, gfp_flags) as *mut ceph_snap_context;
    if snapc.is_null() {
        return core::ptr::null_mut();
    }

    refcount_set(&mut (*snapc).nref, 1);
    (*snapc).num_snaps = snap_count;

    snapc
}

#[no_mangle]
pub unsafe extern "C" fn ceph_get_snap_context(
    sc: *mut ceph_snap_context,
) -> *mut ceph_snap_context {
    if !sc.is_null() {
        refcount_inc(&mut (*sc).nref);
    }
    sc
}

#[no_mangle]
pub unsafe extern "C" fn ceph_put_snap_context(sc: *mut ceph_snap_context) {
    if sc.is_null() {
        return;
    }
    if refcount_dec_and_test(&mut (*sc).nref) {
        /*printk(" deleting snap_context %p\n", sc);*/
        kfree(sc as *mut c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
