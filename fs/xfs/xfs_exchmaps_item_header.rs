/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/*
 * The file mapping exchange intent item helps us exchange multiple file
 * mappings between two inode forks.  It does this by tracking the range of
 * file block offsets that still need to be exchanged, and relogs as progress
 * happens.
 *
 * *I items should be recorded in the *first* of a series of rolled
 * transactions, and the *D items should be recorded in the same transaction
 * that records the associated bmbt updates.
 *
 * Should the system crash after the commit of the first transaction but
 * before the commit of the final transaction in a series, log recovery will
 * use the redo information recorded by the intent items to replay the
 * rest of the mapping exchanges.
 */

/* kernel only XMI/XMD definitions */

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_log_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_xmi_log_format {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_xmd_log_format {
    _private: [u8; 0],
}

/*
 * This is the incore file mapping exchange intent log item.  It is used to log
 * the fact that we are exchanging mappings between two files.  It is used in
 * conjunction with the incore file mapping exchange done log item described
 * below.
 *
 * These log items follow the same rules as struct xfs_efi_log_item; see the
 * comments about that structure (in xfs_extfree_item.h) for more details.
 */
#[repr(C)]
pub struct xfs_xmi_log_item {
    pub xmi_item: xfs_log_item,
    pub xmi_refcount: atomic_t,
    pub xmi_format: xfs_xmi_log_format,
}

/*
 * This is the incore file mapping exchange done log item.  It is used to log
 * the fact that an exchange mentioned in an earlier xmi item have been
 * performed.
 */
#[repr(C)]
pub struct xfs_xmd_log_item {
    pub xmd_item: xfs_log_item,
    pub xmd_intent_log_item: *mut xfs_xmi_log_item,
    pub xmd_format: xfs_xmd_log_format,
}

extern "C" {
    pub static mut xfs_xmi_cache: *mut kmem_cache;
    pub static mut xfs_xmd_cache: *mut kmem_cache;
}

#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_exchmaps_intent {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_exchmaps_defer_add(
        tp: *mut xfs_trans,
        xmi: *mut xfs_exchmaps_intent,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
