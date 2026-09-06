// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_iint.c (translated to Rust)
 *	- implements the IMA hook: ima_inode_free
 *	- cache integrity information in the inode security blob
 */

use std::ptr;
use std::mem;

// External types from ima.h and linux/fs.h
// Defined elsewhere; treated as opaque types
extern "C" {
    type kmem_cache;
    type inode;
    type ima_iint_cache;
    type ima_blob_sizes_type;
}

// External functions from ima.h and linux/slab.h
extern "C" {
    fn IS_IMA(inode: *const inode) -> bool;
    fn ima_inode_get_iint(inode: *const inode) -> *mut ima_iint_cache;
    fn ima_inode_set_iint(inode: *mut inode, iint: *mut ima_iint_cache);
    fn mutex_init(lock: *mut u8) -> i32;
    fn mutex_destroy(lock: *mut u8);
    fn lockdep_set_class(lock: *mut u8, key: *const u8);
    fn kmem_cache_alloc(s: *mut kmem_cache, flags: u32) -> *mut u8;
    fn kmem_cache_free(s: *mut kmem_cache, x: *mut u8);
    fn kmem_cache_create(
        name: *const u8,
        size: usize,
        align: usize,
        flags: u32,
        ctor: Option<extern "C" fn(*mut u8)>,
    ) -> *mut kmem_cache;
    fn kfree(x: *mut u8);

    // External globals
    static ima_blob_sizes: ima_blob_sizes_type;
}

// Static cache pointer - initialized to NULL
// Kernel attribute: __ro_after_init means read-only after initialization
static mut ima_iint_cache: *mut kmem_cache = ptr::null_mut();

// Macro: IMA_MAX_NESTING = FILESYSTEM_MAX_STACK_DEPTH + 1
// FILESYSTEM_MAX_STACK_DEPTH is defined in linux/fs.h (typically 2-3)
const IMA_MAX_NESTING: usize = 5; // Approximate: FILESYSTEM_MAX_STACK_DEPTH + 1

/**
 * ima_iint_find - Return the iint associated with an inode
 * @inode: Pointer to the inode
 *
 * Return the IMA integrity information (iint) associated with an inode, if the
 * inode was processed by IMA.
 *
 * Return: Found iint or NULL.
 */
pub unsafe fn ima_iint_find(inode: *const inode) -> *mut ima_iint_cache {
    if !IS_IMA(inode) {
        return ptr::null_mut();
    }

    ima_inode_get_iint(inode)
}

/*
 * It is not clear that IMA should be nested at all, but as long is it measures
 * files both on overlayfs and on underlying fs, we need to annotate the iint
 * mutex to avoid lockdep false positives related to IMA + overlayfs.
 * See ovl_lockdep_annotate_inode_mutex_key() for more details.
 */
#[inline]
unsafe fn ima_iint_lockdep_annotate(iint: *mut ima_iint_cache, inode: *const inode) {
    // Compiled only when CONFIG_LOCKDEP is enabled in kernel
    #[cfg(all(target_os = "linux", feature = "lockdep"))]
    {
        static mut ima_iint_mutex_key: [u8; IMA_MAX_NESTING] = [0; IMA_MAX_NESTING];

        let depth = (*(*inode).i_sb).s_stack_depth;

        let depth = if depth < 0 || depth >= IMA_MAX_NESTING as i32 {
            0
        } else {
            depth as usize
        };

        lockdep_set_class(
            &mut (*iint).mutex as *mut _ as *mut u8,
            &ima_iint_mutex_key[depth] as *const _ as *const u8,
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

/**
 * ima_inode_get - Find or allocate an iint associated with an inode
 * @inode: Pointer to the inode
 *
 * Find an iint associated with an inode, and allocate a new one if not found.
 * Caller must lock i_mutex.
 *
 * Return: An iint on success, NULL on error.
 */
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

/**
 * ima_inode_free_rcu - Called to free an inode via a RCU callback
 * @inode_security: The inode->i_security pointer
 *
 * Free the IMA data associated with an inode.
 */
pub unsafe extern "C" fn ima_inode_free_rcu(inode_security: *mut u8) {
    let iint_p = (inode_security as *mut *mut ima_iint_cache).add(ima_blob_sizes.lbs_inode);

    /* *iint_p should be NULL if !IS_IMA(inode) */
    if !(*iint_p).is_null() {
        ima_iint_free(*iint_p);
    }
}

unsafe extern "C" fn ima_iint_init_once(foo: *mut u8) {
    let iint = foo as *mut ima_iint_cache;

    ptr::write_bytes(iint as *mut u8, 0, mem::size_of::<ima_iint_cache>());
}

// Kernel __init attribute: function only called at initialization
pub unsafe fn ima_iintcache_init() {
    ima_iint_cache = kmem_cache_create(
        b"ima_iint_cache\0".as_ptr(),
        mem::size_of::<ima_iint_cache>(),
        0,
        SLAB_PANIC,
        Some(ima_iint_init_once),
    );
}

// External constants and macros from ima.h and linux/*.h
extern "C" {
    static INTEGRITY_UNKNOWN: u32;
    static GFP_NOFS: u32;
    static SLAB_PANIC: u32;
    static S_IMA: u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
