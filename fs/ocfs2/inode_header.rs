/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * inode.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// C dependency: extent_map.h

/* OCFS2 Inode Private Data */
#[repr(C)]
pub struct ocfs2_inode_info {
    pub ip_blkno: u64,
    pub ip_rw_lockres: ocfs2_lock_res,
    pub ip_inode_lockres: ocfs2_lock_res,
    pub ip_open_lockres: ocfs2_lock_res,
    pub ip_alloc_sem: rw_semaphore,
    pub ip_xattr_sem: rw_semaphore,
    pub ip_lock: spinlock_t,
    pub ip_open_count: u32,
    pub ip_io_markers: list_head,
    pub ip_clusters: u32,
    pub ip_dyn_features: u16,
    pub ip_io_mutex: mutex,
    pub ip_flags: u32,
    pub ip_attr: u32,
    pub ip_unwritten_list: list_head,
    pub ip_next_orphan: *mut inode,
    pub ip_metadata_cache: ocfs2_caching_info,
    pub ip_extent_map: ocfs2_extent_map,
    pub vfs_inode: inode,
    pub ip_jinode: jbd2_inode,
    pub ip_dir_start_lookup: u32,
    pub ip_last_used_slot: u32,
    pub ip_last_used_group: u64,
    pub ip_dir_lock_gen: u32,
    pub ip_la_data_resv: ocfs2_alloc_reservation,
    pub i_sync_tid: tid_t,
    pub i_datasync_tid: tid_t,
    pub i_dquot: [*mut dquot; MAXQUOTAS],
}

pub const OCFS2_INODE_SYSTEM_FILE: u32 = 0x00000001;
pub const OCFS2_INODE_JOURNAL: u32 = 0x00000002;
pub const OCFS2_INODE_BITMAP: u32 = 0x00000004;
pub const OCFS2_INODE_DELETED: u32 = 0x00000008;
pub const OCFS2_INODE_MAYBE_ORPHANED: u32 = 0x00000010;
pub const OCFS2_INODE_OPEN_DIRECT: u32 = 0x00000020;
pub const OCFS2_INODE_SKIP_ORPHAN_DIR: u32 = 0x00000040;
pub const OCFS2_INODE_DIO_ORPHAN_ENTRY: u32 = 0x00000080;

#[inline]
pub unsafe fn OCFS2_I(inode: *mut inode) -> *mut ocfs2_inode_info {
    container_of!(inode, ocfs2_inode_info, vfs_inode)
}

#[inline]
pub unsafe fn INODE_JOURNAL(i: *mut inode) -> bool {
    ((*OCFS2_I(i)).ip_flags & OCFS2_INODE_JOURNAL) != 0
}

#[inline]
pub unsafe fn SET_INODE_JOURNAL(i: *mut inode) {
    (*OCFS2_I(i)).ip_flags |= OCFS2_INODE_JOURNAL;
}

extern "C" {
    pub static ocfs2_aops: address_space_operations;
    pub static ocfs2_inode_caching_ops: ocfs2_caching_operations;
}

#[inline]
pub unsafe fn INODE_CACHE(inode: *mut inode) -> *mut ocfs2_caching_info {
    &mut (*OCFS2_I(inode)).ip_metadata_cache
}

extern "C" {
    pub fn ocfs2_evict_inode(inode: *mut inode);
}

pub const OCFS2_FI_FLAG_SYSFILE: u32 = 0x1;
pub const OCFS2_FI_FLAG_ORPHAN_RECOVERY: u32 = 0x2;
pub const OCFS2_FI_FLAG_FILECHECK_CHK: u32 = 0x4;
pub const OCFS2_FI_FLAG_FILECHECK_FIX: u32 = 0x8;

extern "C" {
    pub fn ocfs2_ilookup(sb: *mut super_block, feoff: u64) -> *mut inode;
    pub fn ocfs2_iget(osb: *mut ocfs2_super, feoff: u64, flags: u32, sysfile_type: i32) -> *mut inode;
    pub fn ocfs2_inode_revalidate(dentry: *mut dentry) -> i32;
    pub fn ocfs2_populate_inode(inode: *mut inode, fe: *mut ocfs2_dinode, create_ino: i32);
    pub fn ocfs2_sync_blockdev(sb: *mut super_block);
    pub fn ocfs2_refresh_inode(inode: *mut inode, fe: *mut ocfs2_dinode);
    pub fn ocfs2_mark_inode_dirty(handle: *mut handle_t, inode: *mut inode, bh: *mut buffer_head) -> i32;
    pub fn ocfs2_set_inode_flags(inode: *mut inode);
    pub fn ocfs2_get_inode_flags(oi: *mut ocfs2_inode_info);
}

#[inline]
pub unsafe fn ocfs2_inode_sector_count(inode: *mut inode) -> blkcnt_t {
    let c_to_s_bits = (*OCFS2_SB((*inode).i_sb)).s_clustersize_bits - 9;
    ((*OCFS2_I(inode)).ip_clusters as blkcnt_t) << c_to_s_bits
}

extern "C" {
    pub fn ocfs2_validate_inode_block(sb: *mut super_block, bh: *mut buffer_head) -> i32;
    pub fn ocfs2_read_inode_block(inode: *mut inode, bh: *mut *mut buffer_head) -> i32;
    pub fn ocfs2_read_inode_block_full(inode: *mut inode, bh: *mut *mut buffer_head, flags: i32) -> i32;
}

#[inline]
pub unsafe fn cache_info_to_inode(ci: *mut ocfs2_caching_info) -> *mut ocfs2_inode_info {
    container_of!(ci, ocfs2_inode_info, ip_metadata_cache)
}

/* Does this inode have the reflink flag set? */
#[inline]
pub unsafe fn ocfs2_is_refcount_inode(inode: *mut inode) -> bool {
    ((*OCFS2_I(inode)).ip_dyn_features & OCFS2_HAS_REFCOUNT_FL) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
