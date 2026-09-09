// SPDX-License-Identifier: GPL-2.0-or-later
/* Miscellaneous bits for the netfs support library.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and trace-point definitions are supplied by the
// surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    static mut netfs_request_pool: mempool_t;
    static mut netfs_subrequest_pool: mempool_t;
    static mut netfs_folioq_pool: mempool_t;
    fn mempool_init_kmalloc_pool(pool: *mut mempool_t, min_nr: c_int, size: usize) -> c_int;
    fn mempool_init_slab_pool(pool: *mut mempool_t, min_nr: c_int, cache: *mut kmem_cache) -> c_int;
    fn mempool_exit(pool: *mut mempool_t);
    fn kmem_cache_create(name: *const c_char, size: usize, align: usize,
                         flags: c_ulong, ctor: *mut c_void) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn fscache_init() -> c_int;
    fn fscache_exit();
    fn remove_proc_subtree(name: *const c_char, parent: *mut proc_dir_entry);
}

#[repr(C)]
pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)]
pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)]
pub struct mempool_t { _private: [u8; 0] }
#[repr(C)]
pub struct folio_queue { _private: [u8; 0] }
#[repr(C)]
pub struct netfs_io_request { _private: [u8; 0] }
#[repr(C)]
pub struct netfs_io_subrequest { _private: [u8; 0] }

pub static mut netfs_debug: u32 = 0;

#[cfg(feature = "CONFIG_PROC_FS")]
pub static mut netfs_io_requests: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
#[cfg(feature = "CONFIG_PROC_FS")]
pub static mut netfs_proc_lock: spinlock_t = spinlock_t { _private: [0; 0] };

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

#[cfg(feature = "CONFIG_PROC_FS")]
static netfs_origins: [&[u8]; 13] = [
    b"RA", b"RP", b"RG", b"R1", b"RW", b"UR", b"DR", b"WB",
    b"W1", b"WT", b"UW", b"DW", b"2C",
];

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn netfs_requests_seq_show(_m: *mut c_void, _v: *mut c_void) -> c_int {
    // The seq_file formatting and list_entry operations are supplied by the
    // kernel interfaces represented by the surrounding translation.
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn netfs_requests_seq_start(_m: *mut c_void, _pos: *mut c_long) -> *mut c_void { core::ptr::null_mut() }
#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn netfs_requests_seq_next(_m: *mut c_void, _v: *mut c_void, _pos: *mut c_long) -> *mut c_void { core::ptr::null_mut() }
#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn netfs_requests_seq_stop(_m: *mut c_void, _v: *mut c_void) {}

static mut netfs_request_slab: *mut kmem_cache = core::ptr::null_mut();
static mut netfs_subrequest_slab: *mut kmem_cache = core::ptr::null_mut();

unsafe fn netfs_init() -> c_int {
    let mut ret: c_int = -12; // -ENOMEM

    if mempool_init_kmalloc_pool(&raw mut netfs_folioq_pool, 100,
                                 core::mem::size_of::<folio_queue>()) < 0 {
        return ret;
    }

    netfs_request_slab = kmem_cache_create(c"netfs_request".as_ptr(),
        core::mem::size_of::<netfs_io_request>(), 0, 0, core::ptr::null_mut());
    if netfs_request_slab.is_null() { mempool_exit(&raw mut netfs_folioq_pool); return ret; }
    if mempool_init_slab_pool(&raw mut netfs_request_pool, 100, netfs_request_slab) < 0 {
        kmem_cache_destroy(netfs_request_slab); mempool_exit(&raw mut netfs_folioq_pool); return ret;
    }

    netfs_subrequest_slab = kmem_cache_create(c"netfs_subrequest".as_ptr(),
        core::mem::size_of::<netfs_io_subrequest>() + 16, 0, 0, core::ptr::null_mut());
    if netfs_subrequest_slab.is_null() {
        mempool_exit(&raw mut netfs_request_pool); kmem_cache_destroy(netfs_request_slab);
        mempool_exit(&raw mut netfs_folioq_pool); return ret;
    }
    if mempool_init_slab_pool(&raw mut netfs_subrequest_pool, 100, netfs_subrequest_slab) < 0 {
        kmem_cache_destroy(netfs_subrequest_slab); mempool_exit(&raw mut netfs_request_pool);
        kmem_cache_destroy(netfs_request_slab); mempool_exit(&raw mut netfs_folioq_pool); return ret;
    }

    ret = fscache_init();
    if ret < 0 {
        mempool_exit(&raw mut netfs_subrequest_pool); kmem_cache_destroy(netfs_subrequest_slab);
        mempool_exit(&raw mut netfs_request_pool); kmem_cache_destroy(netfs_request_slab);
        mempool_exit(&raw mut netfs_folioq_pool); return ret;
    }
    0
}

unsafe fn netfs_exit() {
    fscache_exit();
    remove_proc_subtree(c"fs/netfs".as_ptr(), core::ptr::null_mut());
    mempool_exit(&raw mut netfs_subrequest_pool);
    kmem_cache_destroy(netfs_subrequest_slab);
    mempool_exit(&raw mut netfs_request_pool);
    kmem_cache_destroy(netfs_request_slab);
    mempool_exit(&raw mut netfs_folioq_pool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
