/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for runlist handling in NTFS Linux kernel driver.
 *
 * Copyright (c) 2001-2005 Anton Altaparmakov
 * Copyright (c) 2002 Richard Russon
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

/* Dependency declarations supplied by the corresponding volume header. */
#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ntfs_volume {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attr_record {
    _private: [u8; 0],
}

extern "C" {
    pub fn init_rwsem(sem: *mut rw_semaphore);
}

/*
 * runlist_element - in memory vcn to lcn mapping array element
 * @vcn: starting vcn of the current array element
 * @lcn: starting lcn of the current array element
 * @length: length in clusters of the current array element
 *
 * The last vcn (in fact the last vcn + 1) is reached when length == 0.
 *
 * When lcn == -1 this means that the count vcns starting at vcn are not
 * physically allocated (i.e. this is a hole / data is sparse).
 *
 * In memory vcn to lcn mapping structure element.
 * @vcn: vcn = Starting virtual cluster number.
 * @lcn: lcn = Starting logical cluster number.
 * @length: Run length in clusters.
 */
#[repr(C)]
pub struct runlist_element {
    pub vcn: i64,
    pub lcn: i64,
    pub length: i64,
}

/*
 * runlist - in memory vcn to lcn mapping array including a read/write lock
 * @rl: pointer to an array of runlist elements
 * @lock: read/write spinlock for serializing access to @rl
 * @rl_hint: hint/cache pointing to the last accessed runlist element
 */
#[repr(C)]
pub struct runlist {
    pub rl: *mut runlist_element,
    pub lock: rw_semaphore,
    pub count: usize,
    pub rl_hint: i32,
}

pub unsafe fn ntfs_init_runlist(rl: *mut runlist) {
    (*rl).rl = core::ptr::null_mut();
    init_rwsem(&mut (*rl).lock);
    (*rl).count = 0;
    (*rl).rl_hint = -1;
}

pub const LCN_DELALLOC: i64 = -1;
pub const LCN_HOLE: i64 = -2;
pub const LCN_RL_NOT_MAPPED: i64 = -3;
pub const LCN_ENOENT: i64 = -4;
pub const LCN_ENOMEM: i64 = -5;
pub const LCN_EIO: i64 = -6;
pub const LCN_EINVAL: i64 = -7;

extern "C" {
    pub fn ntfs_runlists_merge(
        d_runlist: *mut runlist,
        srl: *mut runlist_element,
        s_rl_count: usize,
        new_rl_count: *mut usize,
    ) -> *mut runlist_element;
    pub fn ntfs_mapping_pairs_decompress(
        vol: *const ntfs_volume,
        attr: *const attr_record,
        old_runlist: *mut runlist,
        new_rl_count: *mut usize,
    ) -> *mut runlist_element;
    pub fn ntfs_rl_vcn_to_lcn(rl: *const runlist_element, vcn: i64) -> i64;
    pub fn ntfs_rl_find_vcn_nolock(rl: *mut runlist_element, vcn: i64) -> *mut runlist_element;
    pub fn ntfs_get_size_for_mapping_pairs(
        vol: *const ntfs_volume,
        rl: *const runlist_element,
        first_vcn: i64,
        last_vcn: i64,
        max_mp_size: i32,
    ) -> i32;
    pub fn ntfs_mapping_pairs_build(
        vol: *const ntfs_volume,
        dst: *mut i8,
        dst_len: i32,
        rl: *const runlist_element,
        first_vcn: i64,
        last_vcn: i64,
        stop_vcn: *mut i64,
        stop_rl: *mut *mut runlist_element,
        de_cluster_count: *mut u32,
    ) -> i32;
    pub fn ntfs_rl_truncate_nolock(
        vol: *const ntfs_volume,
        runlist: *mut runlist,
        new_length: i64,
    ) -> i32;
    pub fn ntfs_rl_sparse(rl: *mut runlist_element) -> i32;
    pub fn ntfs_rl_get_compressed_size(
        vol: *mut ntfs_volume,
        rl: *mut runlist_element,
    ) -> i64;
    pub fn ntfs_rl_insert_range(
        dst_rl: *mut runlist_element,
        dst_cnt: i32,
        src_rl: *mut runlist_element,
        src_cnt: i32,
        new_cnt: *mut usize,
    ) -> *mut runlist_element;
    pub fn ntfs_rl_punch_hole(
        dst_rl: *mut runlist_element,
        dst_cnt: i32,
        start_vcn: i64,
        len: i64,
        punch_rl: *mut *mut runlist_element,
        new_rl_cnt: *mut usize,
    ) -> *mut runlist_element;
    pub fn ntfs_rl_collapse_range(
        dst_rl: *mut runlist_element,
        dst_cnt: i32,
        start_vcn: i64,
        len: i64,
        punch_rl: *mut *mut runlist_element,
        new_rl_cnt: *mut usize,
    ) -> *mut runlist_element;
    pub fn ntfs_rl_realloc(
        rl: *mut runlist_element,
        old_size: i32,
        new_size: i32,
    ) -> *mut runlist_element;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
