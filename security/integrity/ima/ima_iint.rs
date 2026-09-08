// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_iint.c
 *	- implements the IMA hook: ima_inode_free
 *	- cache integrity information in the inode security blob
 */

use std::mem;
use std::ptr;

extern "C" {
    type kmem_cache;
    type inode;
    type ima_iint_cache;
    type ima_blob_sizes_type;
    type lock_class_key;

    fn IS_IMA(inode: *const inode) -> bool;
    fn ima_inode_get_iint(inode: *const inode) -> *mut ima_iint_cache;
    fn ima_inode_set_iint(inode: *mut inode, iint: *mut ima_iint_cache);
    fn mutex_init(lock: *mut u8) -> i32;
    fn mutex_destroy(lock: *mut u8);
    fn lockdep_set_class(lock: *mut u8, key: *const lock_class_key);
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: u32) -> *mut u8;
    fn kmem_cache_free(cache: *mut kmem_cache, object: *mut u8);
    fn kmem_cache_create(
        name: *const u8,
        size: usize,
        align: usize,
        flags: u32,
        ctor: Option<unsafe extern "C" fn(*mut u8)>,
    ) -> *mut kmem_cache;
    fn kfree(object: *mut u8);

    static ima_blob_sizes: ima_blob_sizes_type;
    static FILESYSTEM_MAX_STACK_DEPTH: usize;
    static INTEGRITY_UNKNOWN: u32;
    static GFP_NOFS: u32;
    static SLAB_PANIC: u32;
    static S_IMA: u32;
}

static mut ima_iint_cache: *mut kmem_cache = ptr::null_mut();

// IMA_MAX_NESTING is FILESYSTEM_MAX_STACK_DEPTH + 1.
const IMA_MAX_NESTING: usize = 5; // Build-time filesystem limit is supplied externally.

pub unsafe fn ima_iint_find(inode: *const inode) -> *mut ima_iint_cache {
    if !IS_IMA(inode) {
        return ptr::null_mut();
    }
    ima_inode_get_iint(inode)
}

#[inline]
unsafe fn ima_iint_lockdep_annotate(iint: *mut ima_iint_cache, inode: *const inode) {
    // CONFIG_LOCKDEP conditionally includes this block in the kernel build.
    #[cfg(feature = "lockdep")]
    {
        static mut ima_iint_mutex_key: [lock_class_key; IMA_MAX_NESTING] = [
            // Opaque kernel lock keys are initialized by the kernel.
            unsafe { mem::zeroed() }; IMA_MAX_NESTING
        ];
        let mut depth = (*(*inode).i_sb).s_stack_depth;
        if depth < 0 || depth >= IMA_MAX_NESTING as i32 {
            depth = 0;
        }
        lockdep_set_class(
            &mut (*iint).mutex as *mut _ as *mut u8,
            &ima_iint_mutex_key[depth as usize],
        );
    }
}

unsafe fn ima_iint_init_always(iint: *mut ima_iint_cache, inode: *const inode) {
    (*iint).ima_hash = ptr::null_mut();
    (*iint).real_inode.version = 0;
    (*iint).flags = 0;
    (*iint).atomic_flags = 0;
    (*iint).ima_file_status = INTEGRITY_UNKNOWN;
    (*iint).ima_mmap_status = INTEGRITY_UNKNOWN;
    (*iint).ima_bprm_status = INTEGRITY_UNKNOWN;
    (*iint).ima_read_status = INTEGRITY_UNKNOWN;
    (*iint).ima_creds_status = INTEGRITY_UNKNOWN;
    (*iint).measured_pcrs = 0;
    mutex_init(&mut (*iint).mutex as *mut _ as *mut u8);
    ima_iint_lockdep_annotate(iint, inode);
}

unsafe fn ima_iint_free(iint: *mut ima_iint_cache) {
    kfree((*iint).ima_hash as *mut u8);
    mutex_destroy(&mut (*iint).mutex as *mut _ as *mut u8);
    kmem_cache_free(ima_iint_cache, iint as *mut u8);
}

pub unsafe fn ima_inode_get(inode: *mut inode) -> *mut ima_iint_cache {
    let mut iint = ima_iint_find(inode);
    if !iint.is_null() {
        return iint;
    }
    iint = kmem_cache_alloc(ima_iint_cache, GFP_NOFS) as *mut ima_iint_cache;
    if iint.is_null() {
        return ptr::null_mut();
    }
    ima_iint_init_always(iint, inode);
    (*inode).i_flags |= S_IMA;
    ima_inode_set_iint(inode, iint);
    iint
}

pub unsafe extern "C" fn ima_inode_free_rcu(inode_security: *mut u8) {
    let iint_p = (inode_security as *mut *mut ima_iint_cache).add(ima_blob_sizes.lbs_inode);
    if !(*iint_p).is_null() {
        ima_iint_free(*iint_p);
    }
}

unsafe extern "C" fn ima_iint_init_once(foo: *mut u8) {
    ptr::write_bytes(foo, 0, mem::size_of::<ima_iint_cache>());
}

pub unsafe fn ima_iintcache_init() {
    ima_iint_cache = kmem_cache_create(
        b"ima_iint_cache\0".as_ptr(),
        mem::size_of::<ima_iint_cache>(),
        0,
        SLAB_PANIC,
        Some(ima_iint_init_once),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
