/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * uptodate.h
 *
 * Cluster uptodate tracking
 *
 * Copyright (C) 2002, 2004, 2005 Oracle.  All rights reserved.
 */

/*
 * The caching code relies on locking provided by the user of
 * struct ocfs2_caching_info. These operations connect that up.
 */
#[repr(C)]
pub struct ocfs2_caching_operations {
    /* A u64 representing the owning structure. Usually this is the block
     * number (i_blkno or whatnot). This is used so that caching log messages
     * can identify the owning structure.
     */
    pub co_owner: Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info) -> u64>,

    /* The superblock is needed during I/O. */
    pub co_get_super:
        Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info) -> *mut super_block>,

    /* Lock and unlock the caching data. These will not sleep, and should
     * probably be spinlocks.
     */
    pub co_cache_lock: Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info)>,
    pub co_cache_unlock: Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info)>,

    /* Lock and unlock for disk I/O. These will sleep, and should be mutexes.
     */
    pub co_io_lock: Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info)>,
    pub co_io_unlock: Option<unsafe extern "C" fn(ci: *mut ocfs2_caching_info)>,
}

/* __init */
unsafe extern "C" {
    pub fn init_ocfs2_uptodate_cache() -> ::core::ffi::c_int;
    pub fn exit_ocfs2_uptodate_cache();

    pub fn ocfs2_metadata_cache_init(
        ci: *mut ocfs2_caching_info,
        ops: *const ocfs2_caching_operations,
    );
    pub fn ocfs2_metadata_cache_purge(ci: *mut ocfs2_caching_info);
    pub fn ocfs2_metadata_cache_exit(ci: *mut ocfs2_caching_info);

    pub fn ocfs2_metadata_cache_owner(ci: *mut ocfs2_caching_info) -> u64;
    pub fn ocfs2_metadata_cache_io_lock(ci: *mut ocfs2_caching_info);
    pub fn ocfs2_metadata_cache_io_unlock(ci: *mut ocfs2_caching_info);

    pub fn ocfs2_buffer_uptodate(
        ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_set_buffer_uptodate(
        ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head,
    );
    pub fn ocfs2_set_new_buffer_uptodate(
        ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head,
    );
    pub fn ocfs2_remove_from_cache(
        ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head,
    );
    pub fn ocfs2_remove_xattr_clusters_from_cache(
        ci: *mut ocfs2_caching_info,
        block: sector_t,
        c_len: u32,
    );
    pub fn ocfs2_buffer_read_ahead(
        ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
