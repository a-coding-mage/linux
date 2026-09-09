/* SPDX-License-Identifier: GPL-2.0-or-later */
/* General filesystem caching backing cache interface
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * NOTE!!! See:
 *
 * Documentation/filesystems/caching/backend-api.rst
 *
 * for a description of the cache backend interface declared here.
 */

// Dependency: linux/fscache.h

pub enum fscache_cache_trace {}
pub enum fscache_cookie_trace {}
pub enum fscache_access_trace {}
pub enum fscache_volume_trace {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_cache_state {
    FSCACHE_CACHE_IS_NOT_PRESENT,
    FSCACHE_CACHE_IS_PREPARING,
    FSCACHE_CACHE_IS_ACTIVE,
    FSCACHE_CACHE_GOT_IOERROR,
    FSCACHE_CACHE_IS_WITHDRAWN,
}

pub const NR__FSCACHE_CACHE_STATE: usize = fscache_cache_state::FSCACHE_CACHE_IS_WITHDRAWN as usize + 1;

/* Cache cookie. */
#[repr(C)]
pub struct fscache_cache {
    pub ops: *const fscache_cache_ops,
    pub cache_link: list_head,
    pub cache_priv: *mut core::ffi::c_void,
    pub ref_: refcount_t,
    pub n_volumes: atomic_t,
    pub n_accesses: atomic_t,
    pub object_count: atomic_t,
    pub debug_id: core::ffi::c_uint,
    pub state: fscache_cache_state,
    pub name: *mut core::ffi::c_char,
}

/* cache operations */
#[repr(C)]
pub struct fscache_cache_ops {
    pub name: *const core::ffi::c_char,
    pub acquire_volume: Option<unsafe extern "C" fn(volume: *mut fscache_volume)>,
    pub free_volume: Option<unsafe extern "C" fn(volume: *mut fscache_volume)>,
    pub lookup_cookie: Option<unsafe extern "C" fn(cookie: *mut fscache_cookie) -> bool>,
    pub withdraw_cookie: Option<unsafe extern "C" fn(cookie: *mut fscache_cookie)>,
    pub resize_cookie: Option<unsafe extern "C" fn(cres: *mut netfs_cache_resources, new_size: loff_t)>,
    pub invalidate_cookie: Option<unsafe extern "C" fn(cookie: *mut fscache_cookie) -> bool>,
    pub begin_operation: Option<unsafe extern "C" fn(cres: *mut netfs_cache_resources, want_state: fscache_want_state) -> bool>,
    pub prepare_to_write: Option<unsafe extern "C" fn(cookie: *mut fscache_cookie)>,
}

extern "C" {
    pub static mut fscache_wq: *mut workqueue_struct;
    pub static mut fscache_clearance_waiters: wait_queue_head_t;
    pub static mut fscache_addremove_sem: rw_semaphore;

    pub fn fscache_acquire_cache(name: *const core::ffi::c_char) -> *mut fscache_cache;
    pub fn fscache_relinquish_cache(cache: *mut fscache_cache);
    pub fn fscache_add_cache(cache: *mut fscache_cache, ops: *const fscache_cache_ops, cache_priv: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn fscache_withdraw_cache(cache: *mut fscache_cache);
    pub fn fscache_withdraw_volume(volume: *mut fscache_volume);
    pub fn fscache_withdraw_cookie(cookie: *mut fscache_cookie);
    pub fn fscache_io_error(cache: *mut fscache_cache);
    pub fn fscache_try_get_volume(volume: *mut fscache_volume, where_: fscache_volume_trace) -> *mut fscache_volume;
    pub fn fscache_put_volume(volume: *mut fscache_volume, where_: fscache_volume_trace);
    pub fn fscache_end_volume_access(volume: *mut fscache_volume, cookie: *mut fscache_cookie, why: fscache_access_trace);
    pub fn fscache_get_cookie(cookie: *mut fscache_cookie, where_: fscache_cookie_trace) -> *mut fscache_cookie;
    pub fn fscache_put_cookie(cookie: *mut fscache_cookie, where_: fscache_cookie_trace);
    pub fn fscache_end_cookie_access(cookie: *mut fscache_cookie, why: fscache_access_trace);
    pub fn fscache_cookie_lookup_negative(cookie: *mut fscache_cookie);
    pub fn fscache_resume_after_invalidation(cookie: *mut fscache_cookie);
    pub fn fscache_caching_failed(cookie: *mut fscache_cookie);
    pub fn fscache_wait_for_operation(cred: *mut netfs_cache_resources, state: fscache_want_state) -> bool;
}

pub unsafe fn fscache_cookie_state(cookie: *mut fscache_cookie) -> fscache_cookie_state {
    smp_load_acquire(&(*cookie).state)
}

pub unsafe fn fscache_get_key(cookie: *mut fscache_cookie) -> *mut core::ffi::c_void {
    if (*cookie).key_len <= core::mem::size_of_val(&(*cookie).inline_key) {
        (*cookie).inline_key.as_mut_ptr() as *mut core::ffi::c_void
    } else {
        (*cookie).key
    }
}

pub unsafe fn fscache_cres_cookie(cres: *mut netfs_cache_resources) -> *mut fscache_cookie {
    (*cres).cache_priv as *mut fscache_cookie
}

pub unsafe fn fscache_count_object(cache: *mut fscache_cache) { atomic_inc(&mut (*cache).object_count); }

pub unsafe fn fscache_uncount_object(cache: *mut fscache_cache) {
    if atomic_dec_and_test(&mut (*cache).object_count) { wake_up_all(&mut fscache_clearance_waiters); }
}

pub unsafe fn fscache_wait_for_objects(cache: *mut fscache_cache) {
    wait_event(&mut fscache_clearance_waiters, atomic_read(&(*cache).object_count) == 0);
}

#[cfg(CONFIG_FSCACHE_STATS)]
extern "C" {
    pub static mut fscache_n_read: atomic_t;
    pub static mut fscache_n_write: atomic_t;
    pub static mut fscache_n_no_write_space: atomic_t;
    pub static mut fscache_n_no_create_space: atomic_t;
    pub static mut fscache_n_culled: atomic_t;
    pub static mut fscache_n_dio_misfit: atomic_t;
}

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_read { () => { unsafe { atomic_inc(&mut fscache_n_read) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_read { () => {}; }

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_write { () => { unsafe { atomic_inc(&mut fscache_n_write) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_write { () => {}; }

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_no_write_space { () => { unsafe { atomic_inc(&mut fscache_n_no_write_space) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_no_write_space { () => {}; }

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_no_create_space { () => { unsafe { atomic_inc(&mut fscache_n_no_create_space) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_no_create_space { () => {}; }

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_culled { () => { unsafe { atomic_inc(&mut fscache_n_culled) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_culled { () => {}; }

#[cfg(CONFIG_FSCACHE_STATS)]
#[macro_export] macro_rules! fscache_count_dio_misfit { () => { unsafe { atomic_inc(&mut fscache_n_dio_misfit) } }; }
#[cfg(not(CONFIG_FSCACHE_STATS))]
#[macro_export] macro_rules! fscache_count_dio_misfit { () => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
