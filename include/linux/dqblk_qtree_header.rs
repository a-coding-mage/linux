/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Definitions of structures and functions for quota formats using trie
 */

use core::ffi::c_void;

/* Numbers of blocks needed for updates - we count with the smallest
 * possible block size (1024) */
pub const QTREE_INIT_ALLOC: i32 = 4;
pub const QTREE_INIT_REWRITE: i32 = 2;
pub const QTREE_DEL_ALLOC: i32 = 0;
pub const QTREE_DEL_REWRITE: i32 = 6;

#[repr(C)]
pub struct dquot {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kqid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

/* Operations */
#[repr(C)]
pub struct qtree_fmt_operations {
    /* Convert given entry from in memory format to disk one */
    pub mem2disk_dqblk: Option<unsafe extern "C" fn(disk: *mut c_void, dquot: *mut dquot)>,
    /* Convert given entry from disk format to in memory one */
    pub disk2mem_dqblk: Option<unsafe extern "C" fn(dquot: *mut dquot, disk: *mut c_void)>,
    /* Is this structure for given id? */
    pub is_id: Option<unsafe extern "C" fn(disk: *mut c_void, dquot: *mut dquot) -> i32>,
}

/* Inmemory copy of version specific information */
#[repr(C)]
pub struct qtree_mem_dqinfo {
    /* Sb quota is on */
    pub dqi_sb: *mut super_block,
    /* Quota type */
    pub dqi_type: i32,
    /* # of blocks in quota file */
    pub dqi_blocks: u32,
    /* First block in list of free blocks */
    pub dqi_free_blk: u32,
    /* First block with free entry */
    pub dqi_free_entry: u32,
    /* Block size of quota file */
    pub dqi_blocksize_bits: u32,
    /* Size of quota entry in quota file */
    pub dqi_entry_size: u32,
    /* Space usable in block for quota data */
    pub dqi_usable_bs: u32,
    /* Precomputed depth of quota tree */
    pub dqi_qtree_depth: u32,
    /* Operations for entry manipulation */
    pub dqi_ops: *const qtree_fmt_operations,
}

extern "C" {
    pub fn qtree_write_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> i32;
    pub fn qtree_read_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> i32;
    pub fn qtree_delete_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> i32;
    pub fn qtree_release_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> i32;
    pub fn qtree_entry_unused(info: *mut qtree_mem_dqinfo, disk: *mut core::ffi::c_char) -> i32;
}

pub unsafe fn qtree_depth(info: *mut qtree_mem_dqinfo) -> i32 {
    let epb: u32 = (*info).dqi_usable_bs >> 2;
    let mut entries: u64 = epb as u64;
    let mut i: i32 = 1;

    while entries < (1u64 << 32) {
        i += 1;
        entries = entries.wrapping_mul(epb as u64);
    }
    i
}

extern "C" {
    pub fn qtree_get_next_id(info: *mut qtree_mem_dqinfo, qid: *mut kqid) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
