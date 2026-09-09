/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dlmglue.h
 *
 * description here
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Dependency supplied by the surrounding translation unit: dcache.h.

pub const OCFS2_LVB_VERSION: u32 = 5;

#[repr(C)]
pub struct ocfs2_meta_lvb {
    pub lvb_version: u8,
    pub lvb_reserved0: u8,
    pub lvb_idynfeatures: u16,
    pub lvb_iclusters: u32,
    pub lvb_iuid: u32,
    pub lvb_igid: u32,
    pub lvb_iatime_packed: u64,
    pub lvb_ictime_packed: u64,
    pub lvb_imtime_packed: u64,
    pub lvb_isize: u64,
    pub lvb_imode: u16,
    pub lvb_inlink: u16,
    pub lvb_iattr: u32,
    pub lvb_igeneration: u32,
    pub lvb_reserved2: u32,
}

pub const OCFS2_QINFO_LVB_VERSION: u32 = 1;
#[repr(C)]
pub struct ocfs2_qinfo_lvb {
    pub lvb_version: u8,
    pub lvb_reserved: [u8; 3],
    pub lvb_bgrace: u32,
    pub lvb_igrace: u32,
    pub lvb_syncms: u32,
    pub lvb_blocks: u32,
    pub lvb_free_blk: u32,
    pub lvb_free_entry: u32,
}

pub const OCFS2_ORPHAN_LVB_VERSION: u32 = 1;
#[repr(C)]
pub struct ocfs2_orphan_scan_lvb {
    pub lvb_version: u8,
    pub lvb_reserved: [u8; 3],
    pub lvb_os_seqno: u32,
}

pub const OCFS2_TRIMFS_LVB_VERSION: u32 = 1;
#[repr(C)]
pub struct ocfs2_trim_fs_lvb {
    pub lvb_version: u8,
    pub lvb_success: u8,
    pub lvb_reserved: [u8; 2],
    pub lvb_nodenum: u32,
    pub lvb_start: u64,
    pub lvb_len: u64,
    pub lvb_minlen: u64,
    pub lvb_trimlen: u64,
}

#[repr(C)]
pub struct ocfs2_trim_fs_info {
    pub tf_valid: u8,   // lvb is valid, or not
    pub tf_success: u8, // trim is successful, or not
    pub tf_nodenum: u32,
    pub tf_start: u64,
    pub tf_len: u64,
    pub tf_minlen: u64,
    pub tf_trimlen: u64,
}

#[repr(C)]
pub struct ocfs2_lock_holder {
    pub oh_list: list_head,
    pub oh_owner_pid: *mut pid,
    pub oh_ex: i32,
}

// Opaque types supplied by other translated headers.
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
pub enum ocfs2_super {}
pub enum ocfs2_lock_res {}
pub enum ocfs2_dentry_lock {}
pub enum inode {}
pub enum ocfs2_file_private {}
pub enum ocfs2_mem_dqinfo {}
pub enum buffer_head {}
pub enum folio {}
pub enum vfsmount {}
pub enum dentry {}
pub enum file {}
pub enum ocfs2_refcount_tree {}
pub enum ocfs2_dlm_debug {}

pub const OCFS2_META_LOCK_RECOVERY: i32 = 0x01;
pub const OCFS2_META_LOCK_NOQUEUE: i32 = 0x02;
pub const OCFS2_LOCK_NONBLOCK: i32 = 0x04;
pub const OCFS2_META_LOCK_GETBH: i32 = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ocfs2_lock_type { }

pub const OI_LS_NORMAL: i32 = 0;
pub const OI_LS_PARENT: i32 = 1;
pub const OI_LS_RENAME1: i32 = 2;
pub const OI_LS_RENAME2: i32 = 3;
pub const OI_LS_REFLINK_TARGET: i32 = 4;

extern "C" {
    pub fn ocfs2_dlm_init(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_dlm_shutdown(osb: *mut ocfs2_super, hangup_pending: i32);
    pub fn ocfs2_lock_res_init_once(res: *mut ocfs2_lock_res);
    pub fn ocfs2_inode_lock_res_init(res: *mut ocfs2_lock_res, type_: ocfs2_lock_type, generation: u32, inode: *mut inode);
    pub fn ocfs2_dentry_lock_res_init(dl: *mut ocfs2_dentry_lock, parent: u64, inode: *mut inode);
    pub fn ocfs2_file_lock_res_init(lockres: *mut ocfs2_lock_res, fp: *mut ocfs2_file_private);
    pub fn ocfs2_qinfo_lock_res_init(lockres: *mut ocfs2_lock_res, info: *mut ocfs2_mem_dqinfo);
    pub fn ocfs2_refcount_lock_res_init(lockres: *mut ocfs2_lock_res, osb: *mut ocfs2_super, ref_blkno: u64, generation: u32);
    pub fn ocfs2_lock_res_free(res: *mut ocfs2_lock_res);
    pub fn ocfs2_create_new_inode_locks(inode: *mut inode) -> i32;
    pub fn ocfs2_drop_inode_locks(inode: *mut inode) -> i32;
    pub fn ocfs2_rw_lock(inode: *mut inode, write: i32) -> i32;
    pub fn ocfs2_try_rw_lock(inode: *mut inode, write: i32) -> i32;
    pub fn ocfs2_rw_unlock(inode: *mut inode, write: i32);
    pub fn ocfs2_open_lock(inode: *mut inode) -> i32;
    pub fn ocfs2_try_open_lock(inode: *mut inode, write: i32) -> i32;
    pub fn ocfs2_open_unlock(inode: *mut inode);
    pub fn ocfs2_inode_lock_atime(inode: *mut inode, vfsmnt: *mut vfsmount, level: *mut i32, wait: i32) -> i32;
    pub fn ocfs2_inode_lock_full_nested(inode: *mut inode, ret_bh: *mut *mut buffer_head, ex: i32, arg_flags: i32, subclass: i32) -> i32;
    pub fn ocfs2_inode_lock_with_folio(inode: *mut inode, ret_bh: *mut *mut buffer_head, ex: i32, folio: *mut folio) -> i32;
    pub fn ocfs2_inode_unlock(inode: *mut inode, ex: i32);
    pub fn ocfs2_super_lock(osb: *mut ocfs2_super, ex: i32) -> i32;
    pub fn ocfs2_super_unlock(osb: *mut ocfs2_super, ex: i32);
    pub fn ocfs2_orphan_scan_lock(osb: *mut ocfs2_super, seqno: *mut u32) -> i32;
    pub fn ocfs2_orphan_scan_unlock(osb: *mut ocfs2_super, seqno: u32);
    pub fn ocfs2_rename_lock(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_rename_unlock(osb: *mut ocfs2_super);
    pub fn ocfs2_nfs_sync_lock(osb: *mut ocfs2_super, ex: i32) -> i32;
    pub fn ocfs2_nfs_sync_unlock(osb: *mut ocfs2_super, ex: i32);
    pub fn ocfs2_trim_fs_lock_res_init(osb: *mut ocfs2_super);
    pub fn ocfs2_trim_fs_lock_res_uninit(osb: *mut ocfs2_super);
    pub fn ocfs2_trim_fs_lock(osb: *mut ocfs2_super, info: *mut ocfs2_trim_fs_info, trylock: i32) -> i32;
    pub fn ocfs2_trim_fs_unlock(osb: *mut ocfs2_super, info: *mut ocfs2_trim_fs_info);
    pub fn ocfs2_dentry_lock(dentry: *mut dentry, ex: i32) -> i32;
    pub fn ocfs2_dentry_unlock(dentry: *mut dentry, ex: i32);
    pub fn ocfs2_file_lock(file: *mut file, ex: i32, trylock: i32) -> i32;
    pub fn ocfs2_file_unlock(file: *mut file);
    pub fn ocfs2_qinfo_lock(oinfo: *mut ocfs2_mem_dqinfo, ex: i32) -> i32;
    pub fn ocfs2_qinfo_unlock(oinfo: *mut ocfs2_mem_dqinfo, ex: i32);
    pub fn ocfs2_refcount_lock(ref_tree: *mut ocfs2_refcount_tree, ex: i32) -> i32;
    pub fn ocfs2_refcount_unlock(ref_tree: *mut ocfs2_refcount_tree, ex: i32);
    pub fn ocfs2_mark_lockres_freeing(osb: *mut ocfs2_super, lockres: *mut ocfs2_lock_res);
    pub fn ocfs2_simple_drop_lockres(osb: *mut ocfs2_super, lockres: *mut ocfs2_lock_res);
    pub fn ocfs2_wake_downconvert_thread(osb: *mut ocfs2_super);
    pub fn ocfs2_new_dlm_debug() -> *mut ocfs2_dlm_debug;
    pub fn ocfs2_put_dlm_debug(dlm_debug: *mut ocfs2_dlm_debug);
    pub fn ocfs2_set_locking_protocol();
    pub fn ocfs2_inode_lock_tracker(inode: *mut inode, ret_bh: *mut *mut buffer_head, ex: i32, oh: *mut ocfs2_lock_holder) -> i32;
    pub fn ocfs2_inode_unlock_tracker(inode: *mut inode, ex: i32, oh: *mut ocfs2_lock_holder, had_lock: i32);
}

#[inline]
pub unsafe fn ocfs2_inode_lock_full(i: *mut inode, r: *mut *mut buffer_head, e: i32, f: i32) -> i32 {
    ocfs2_inode_lock_full_nested(i, r, e, f, OI_LS_NORMAL)
}
#[inline]
pub unsafe fn ocfs2_inode_lock_nested(i: *mut inode, b: *mut *mut buffer_head, e: i32, s: i32) -> i32 {
    ocfs2_inode_lock_full_nested(i, b, e, 0, s)
}
#[inline]
pub unsafe fn ocfs2_inode_lock(i: *mut inode, b: *mut *mut buffer_head, e: i32) -> i32 {
    ocfs2_inode_lock_full_nested(i, b, e, 0, OI_LS_NORMAL)
}
#[inline]
pub unsafe fn ocfs2_try_inode_lock(i: *mut inode, b: *mut *mut buffer_head, e: i32) -> i32 {
    ocfs2_inode_lock_full_nested(i, b, e, OCFS2_META_LOCK_NOQUEUE, OI_LS_NORMAL)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
