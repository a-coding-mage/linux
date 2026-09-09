/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for NTFS Linux kernel driver.
 *
 * Copyright (c) 2001-2014 Anton Altaparmakov and Tuxera Inc.
 * Copyright (C) 2002 Richard Russon
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// C header dependencies are supplied by the surrounding translation unit.

pub const NTFS_DEF_PREALLOC_SIZE: usize = 65536;
pub const STANDARD_COMPRESSION_UNIT: u32 = 4;
pub const MAX_COMPRESSION_CLUSTER_SIZE: usize = 4096;

macro_rules! NTFS_B_TO_CLU {
    ($vol:expr, $b:expr) => { ($b) >> ($vol).cluster_size_bits };
}
macro_rules! NTFS_CLU_TO_B {
    ($vol:expr, $clu:expr) => { (($clu as u64) << ($vol).cluster_size_bits) };
}
macro_rules! NTFS_B_TO_CLU_OFS {
    ($vol:expr, $clu:expr) => { (($clu as u64) & ($vol).cluster_size_mask) };
}
macro_rules! NTFS_MFT_NR_TO_CLU {
    ($vol:expr, $mft_no:expr) => {
        ((($mft_no as u64) << ($vol).mft_record_size_bits) >> ($vol).cluster_size_bits)
    };
}
macro_rules! NTFS_MFT_NR_TO_PIDX {
    ($vol:expr, $mft_no:expr) => {
        ($mft_no >> (PAGE_SHIFT - ($vol).mft_record_size_bits))
    };
}
macro_rules! NTFS_MFT_NR_TO_POFS {
    ($vol:expr, $mft_no:expr) => {
        ((($mft_no as u64) << ($vol).mft_record_size_bits) & !PAGE_MASK)
    };
}
macro_rules! NTFS_PIDX_TO_BLK {
    ($vol:expr, $idx:expr) => {
        ((($idx as u64) << PAGE_SHIFT) >> (($vol).sb).s_blocksize_bits)
    };
}
macro_rules! NTFS_PIDX_TO_CLU {
    ($vol:expr, $idx:expr) => {
        ((($idx as u64) << PAGE_SHIFT) >> ($vol).cluster_size_bits)
    };
}
macro_rules! NTFS_CLU_TO_PIDX {
    ($vol:expr, $clu:expr) => {
        ((($clu as u64) << ($vol).cluster_size_bits) >> PAGE_SHIFT)
    };
}
macro_rules! NTFS_CLU_TO_POFS {
    ($vol:expr, $clu:expr) => {
        ((($clu as u64) << ($vol).cluster_size_bits) & !PAGE_MASK)
    };
}
macro_rules! NTFS_B_TO_SECTOR {
    ($vol:expr, $b:expr) => { ($b) >> (($vol).sb).s_blocksize_bits };
}

pub const NTFS_BLOCK_SIZE: u32 = 512;
pub const NTFS_BLOCK_SIZE_BITS: u32 = 9;
pub const NTFS_SB_MAGIC: u32 = 0x5346_544e;
pub const NTFS_MAX_NAME_LEN: u32 = 255;
pub const NTFS_MAX_LABEL_LEN: u32 = 128;

pub const CASE_SENSITIVE: u32 = 0;
pub const IGNORE_CASE: u32 = 1;

/* Conversion helpers for NTFS units. */

/* Convert bytes to cluster count */
#[inline]
pub unsafe fn ntfs_bytes_to_cluster(vol: *const ntfs_volume, bytes: s64) -> u64 {
    (bytes >> (*vol).cluster_size_bits) as u64
}

/* Convert cluster count to bytes */
#[inline]
pub unsafe fn ntfs_cluster_to_bytes(vol: *const ntfs_volume, clusters: u64) -> u64 {
    clusters << (*vol).cluster_size_bits
}

/* Get the byte offset within a cluster from a linear byte address */
#[inline]
pub unsafe fn ntfs_bytes_to_cluster_off(vol: *const ntfs_volume, bytes: u64) -> u64 {
    bytes & (*vol).cluster_size_mask
}

/* Calculate the physical cluster number containing a specific MFT record. */
#[inline]
pub unsafe fn ntfs_mft_no_to_cluster(vol: *const ntfs_volume, mft_no: c_ulong) -> u64 {
    ((mft_no as u64) << (*vol).mft_record_size_bits) >> (*vol).cluster_size_bits
}

/* Calculate the folio index where the MFT record resides. */
#[inline]
pub unsafe fn ntfs_mft_no_to_pidx(vol: *const ntfs_volume, mft_no: c_ulong) -> pgoff_t {
    mft_no >> (PAGE_SHIFT - (*vol).mft_record_size_bits)
}

/* Calculate the byte offset within a folio for an MFT record. */
#[inline]
pub unsafe fn ntfs_mft_no_to_poff(vol: *const ntfs_volume, mft_no: c_ulong) -> u64 {
    ((mft_no as u64) << (*vol).mft_record_size_bits) & !PAGE_MASK
}

/* Convert folio index to cluster number. */
#[inline]
pub unsafe fn ntfs_pidx_to_cluster(vol: *const ntfs_volume, idx: pgoff_t) -> u64 {
    ((idx as u64) << PAGE_SHIFT) >> (*vol).cluster_size_bits
}

/* Convert cluster number to folio index. */
#[inline]
pub unsafe fn ntfs_cluster_to_pidx(vol: *const ntfs_volume, clu: u64) -> pgoff_t {
    (clu << (*vol).cluster_size_bits) >> PAGE_SHIFT
}

/* Get the byte offset within a folio from a cluster number */
#[inline]
pub unsafe fn ntfs_cluster_to_poff(vol: *const ntfs_volume, clu: u64) -> u64 {
    (clu << (*vol).cluster_size_bits) & !PAGE_MASK
}

/* Convert byte offset to sector (block) number. */
#[inline]
pub unsafe fn ntfs_bytes_to_sector(vol: *const ntfs_volume, bytes: u64) -> sector_t {
    bytes >> (*(*vol).sb).s_blocksize_bits
}

/* Global variables. */

/* Slab caches (from super.c). */
extern "C" {
    pub static mut ntfs_name_cache: *mut kmem_cache;
    pub static mut ntfs_inode_cache: *mut kmem_cache;
    pub static mut ntfs_big_inode_cache: *mut kmem_cache;
    pub static mut ntfs_attr_ctx_cache: *mut kmem_cache;
    pub static mut ntfs_index_ctx_cache: *mut kmem_cache;

    pub static ntfs_aops: address_space_operations;
    pub static ntfs_mft_aops: address_space_operations;
    pub static ntfs_file_ops: file_operations;
    pub static ntfs_file_inode_ops: inode_operations;
    pub static ntfs_symlink_inode_operations: inode_operations;
    pub static ntfs_special_inode_operations: inode_operations;
    pub static ntfs_dir_ops: file_operations;
    pub static ntfs_dir_inode_ops: inode_operations;
    pub static ntfs_empty_file_ops: file_operations;
    pub static ntfs_empty_inode_ops: inode_operations;
    pub static ntfs_export_ops: export_operations;
}

#[inline]
pub unsafe fn NTFS_SB(sb: *mut super_block) -> *mut ntfs_volume {
    (*sb).s_fs_info as *mut ntfs_volume
}

pub const default_upcase_len: u32 = 0x10000;

extern "C" {
    pub static mut ntfs_lock: mutex;
    pub static on_errors_arr: option_t;
}

#[repr(C)]
pub struct option_t {
    pub val: c_int,
    pub str: *mut c_char,
}

extern "C" {
    pub fn ntfs_read_compressed_block(folio: *mut folio) -> c_int;
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub fn ntfs_read_wof_compressed_block(folio: *mut folio) -> c_int;
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub fn ntfs_wof_free_workspaces();
    pub fn allocate_compression_buffers() -> c_int;
    pub fn free_compression_buffers();
    pub fn ntfs_compress_write(
        ni: *mut ntfs_inode,
        pos: loff_t,
        count: size_t,
        from: *mut iov_iter,
    ) -> c_int;

    pub fn ntfs_set_volume_flags(vol: *mut ntfs_volume, flags: __le16) -> c_int;
    pub fn ntfs_clear_volume_flags(vol: *mut ntfs_volume, flags: __le16) -> c_int;
    pub fn ntfs_write_volume_label(vol: *mut ntfs_volume, label: *mut c_char) -> c_int;

    pub fn post_read_mst_fixup(b: *mut ntfs_record, size: u32) -> c_int;
    pub fn pre_write_mst_fixup(b: *mut ntfs_record, size: u32) -> c_int;
    pub fn post_write_mst_fixup(b: *mut ntfs_record);

    pub fn ntfs_are_names_equal(
        s1: *const __le16,
        s1_len: size_t,
        s2: *const __le16,
        s2_len: size_t,
        ic: u32,
        upcase: *const __le16,
        upcase_size: u32,
    ) -> bool;
    pub fn ntfs_collate_names(
        name1: *const __le16,
        name1_len: u32,
        name2: *const __le16,
        name2_len: u32,
        err_val: c_int,
        ic: u32,
        upcase: *const __le16,
        upcase_len: u32,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
