/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Helper routines for the NFS client caches
 *
 * Copyright (c) 2009 Trond Myklebust <Trond.Myklebust@netapp.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_char;

/*
 * Deferred request handling
 */
#[repr(C)]
pub struct nfs_cache_defer_req {
    pub req: cache_req,
    pub deferred_req: cache_deferred_req,
    pub completion: completion,
    pub count: refcount_t,
}

extern "C" {
    pub fn nfs_cache_upcall(cd: *mut cache_detail, entry_name: *mut c_char) -> i32;
    pub fn nfs_cache_defer_req_alloc() -> *mut nfs_cache_defer_req;
    pub fn nfs_cache_defer_req_put(dreq: *mut nfs_cache_defer_req);
    pub fn nfs_cache_wait_for_upcall(dreq: *mut nfs_cache_defer_req) -> i32;

    pub fn nfs_cache_register_net(net: *mut net, cd: *mut cache_detail) -> i32;
    pub fn nfs_cache_unregister_net(net: *mut net, cd: *mut cache_detail);
    pub fn nfs_cache_register_sb(sb: *mut super_block, cd: *mut cache_detail) -> i32;
    pub fn nfs_cache_unregister_sb(sb: *mut super_block, cd: *mut cache_detail);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
