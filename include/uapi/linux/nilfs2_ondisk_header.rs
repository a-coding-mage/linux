/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * nilfs2_ondisk.h - NILFS2 on-disk structures
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation; either version 2.1 of the License, or
 * (at your option) any later version.
 */

pub const NILFS_INODE_BMAP_SIZE: usize = 7;

#[repr(C)]
pub struct nilfs_inode {
    pub i_blocks: __le64,
    pub i_size: __le64,
    pub i_ctime: __le64,
    pub i_mtime: __le64,
    pub i_ctime_nsec: __le32,
    pub i_mtime_nsec: __le32,
    pub i_uid: __le32,
    pub i_gid: __le32,
    pub i_mode: __le16,
    pub i_links_count: __le16,
    pub i_flags: __le32,
    pub i_bmap: [__le64; NILFS_INODE_BMAP_SIZE],
    pub i_xattr: __le64,
    pub i_generation: __le32,
    pub i_pad: __le32,
}

pub const NILFS_MIN_INODE_SIZE: usize = 128;

#[repr(C)]
pub struct nilfs_super_root {
    pub sr_sum: __le32,
    pub sr_bytes: __le16,
    pub sr_flags: __le16,
    pub sr_nongc_ctime: __le64,
    pub sr_dat: nilfs_inode,
    pub sr_cpfile: nilfs_inode,
    pub sr_sufile: nilfs_inode,
}

#[macro_export]
macro_rules! NILFS_SR_MDT_OFFSET {
    ($inode_size:expr, $i:expr) => {
        (core::mem::offset_of!($crate::nilfs_super_root, sr_dat) + ($inode_size) * ($i))
    };
}
#[macro_export]
macro_rules! NILFS_SR_DAT_OFFSET { ($inode_size:expr) => { NILFS_SR_MDT_OFFSET!($inode_size, 0) }; }
#[macro_export]
macro_rules! NILFS_SR_CPFILE_OFFSET { ($inode_size:expr) => { NILFS_SR_MDT_OFFSET!($inode_size, 1) }; }
#[macro_export]
macro_rules! NILFS_SR_SUFILE_OFFSET { ($inode_size:expr) => { NILFS_SR_MDT_OFFSET!($inode_size, 2) }; }
#[macro_export]
macro_rules! NILFS_SR_BYTES { ($inode_size:expr) => { NILFS_SR_MDT_OFFSET!($inode_size, 3) }; }

pub const NILFS_DFL_MAX_MNT_COUNT: u32 = 50;
pub const NILFS_VALID_FS: u32 = 0x0001;
pub const NILFS_ERROR_FS: u32 = 0x0002;
pub const NILFS_RESIZE_FS: u32 = 0x0004;
pub const NILFS_MOUNT_ERROR_MODE: u32 = 0x0070;
pub const NILFS_MOUNT_ERRORS_CONT: u32 = 0x0010;
pub const NILFS_MOUNT_ERRORS_RO: u32 = 0x0020;
pub const NILFS_MOUNT_ERRORS_PANIC: u32 = 0x0040;
pub const NILFS_MOUNT_BARRIER: u32 = 0x1000;
pub const NILFS_MOUNT_STRICT_ORDER: u32 = 0x2000;
pub const NILFS_MOUNT_NORECOVERY: u32 = 0x4000;
pub const NILFS_MOUNT_DISCARD: u32 = 0x8000;

#[repr(C)]
pub struct nilfs_super_block {
    pub s_rev_level: __le32, pub s_minor_rev_level: __le16, pub s_magic: __le16,
    pub s_bytes: __le16, pub s_flags: __le16, pub s_crc_seed: __le32,
    pub s_sum: __le32, pub s_log_block_size: __le32, pub s_nsegments: __le64,
    pub s_dev_size: __le64, pub s_first_data_block: __le64,
    pub s_blocks_per_segment: __le32, pub s_r_segments_percentage: __le32,
    pub s_last_cno: __le64, pub s_last_pseg: __le64, pub s_last_seq: __le64,
    pub s_free_blocks_count: __le64, pub s_ctime: __le64, pub s_mtime: __le64,
    pub s_wtime: __le64, pub s_mnt_count: __le16, pub s_max_mnt_count: __le16,
    pub s_state: __le16, pub s_errors: __le16, pub s_lastcheck: __le64,
    pub s_checkinterval: __le32, pub s_creator_os: __le32, pub s_def_resuid: __le16,
    pub s_def_resgid: __le16, pub s_first_ino: __le32, pub s_inode_size: __le16,
    pub s_dat_entry_size: __le16, pub s_checkpoint_size: __le16,
    pub s_segment_usage_size: __le16, pub s_uuid: [__u8; 16],
    pub s_volume_name: [core::ffi::c_char; 80], pub s_c_interval: __le32,
    pub s_c_block_max: __le32, pub s_feature_compat: __le64,
    pub s_feature_compat_ro: __le64, pub s_feature_incompat: __le64,
    pub s_reserved: [__u32; 186],
}

pub const NILFS_OS_LINUX: u32 = 0;
pub const NILFS_CURRENT_REV: u32 = 2;
pub const NILFS_MINOR_REV: u32 = 0;
pub const NILFS_MIN_SUPP_REV: u32 = 2;
pub const NILFS_FEATURE_COMPAT_RO_BLOCK_COUNT: u64 = 0x00000001;
pub const NILFS_FEATURE_COMPAT_SUPP: u64 = 0;
pub const NILFS_FEATURE_COMPAT_RO_SUPP: u64 = NILFS_FEATURE_COMPAT_RO_BLOCK_COUNT;
pub const NILFS_FEATURE_INCOMPAT_SUPP: u64 = 0;
pub const NILFS_SB_BYTES: usize = core::mem::offset_of!(nilfs_super_block, s_reserved);
pub const NILFS_ROOT_INO: u32 = 2;
pub const NILFS_DAT_INO: u32 = 3;
pub const NILFS_CPFILE_INO: u32 = 4;
pub const NILFS_SUFILE_INO: u32 = 5;
pub const NILFS_IFILE_INO: u32 = 6;
pub const NILFS_ATIME_INO: u32 = 7;
pub const NILFS_XATTR_INO: u32 = 8;
pub const NILFS_SKETCH_INO: u32 = 10;
pub const NILFS_USER_INO: u32 = 11;
pub const NILFS_SB_OFFSET_BYTES: u32 = 1024;
pub const NILFS_SEG_MIN_BLOCKS: u32 = 16;
pub const NILFS_PSEG_MIN_BLOCKS: u32 = 2;
pub const NILFS_MIN_NRSVSEGS: u32 = 8;
#[macro_export]
macro_rules! NILFS_ROOT_METADATA_FILE { ($ino:expr) => { ($ino) >= NILFS_DAT_INO && ($ino) <= NILFS_SUFILE_INO }; }
#[macro_export]
macro_rules! NILFS_SB2_OFFSET_BYTES { ($devsize:expr) => { ((($devsize) >> 12) - 1) << 12 }; }
pub const NILFS_LINK_MAX: u32 = 32000;
pub const NILFS_NAME_LEN: usize = 255;
pub const NILFS_MIN_BLOCK_SIZE: u32 = 1024;
pub const NILFS_MAX_BLOCK_SIZE: u32 = 65536;

#[repr(C)]
pub struct nilfs_dir_entry {
    pub inode: __le64, pub rec_len: __le16, pub name_len: __u8, pub file_type: __u8,
    pub name: [core::ffi::c_char; NILFS_NAME_LEN], pub pad: core::ffi::c_char,
}

pub const NILFS_FT_UNKNOWN: u32 = 0;
pub const NILFS_FT_REG_FILE: u32 = 1;
pub const NILFS_FT_DIR: u32 = 2;
pub const NILFS_FT_CHRDEV: u32 = 3;
pub const NILFS_FT_BLKDEV: u32 = 4;
pub const NILFS_FT_FIFO: u32 = 5;
pub const NILFS_FT_SOCK: u32 = 6;
pub const NILFS_FT_SYMLINK: u32 = 7;
pub const NILFS_FT_MAX: u32 = 8;
pub const NILFS_DIR_PAD: u32 = 8;
pub const NILFS_DIR_ROUND: u32 = NILFS_DIR_PAD - 1;
#[macro_export]
macro_rules! NILFS_DIR_REC_LEN { ($name_len:expr) => { ((($name_len) + 12 + NILFS_DIR_ROUND) & !NILFS_DIR_ROUND) }; }
pub const NILFS_MAX_REC_LEN: u32 = (1 << 16) - 1;

#[repr(C)] pub struct nilfs_finfo { pub fi_ino: __le64, pub fi_cno: __le64, pub fi_nblocks: __le32, pub fi_ndatablk: __le32 }
#[repr(C)] pub struct nilfs_binfo_v { pub bi_vblocknr: __le64, pub bi_blkoff: __le64 }
#[repr(C)] pub struct nilfs_binfo_dat { pub bi_blkoff: __le64, pub bi_level: __u8, pub bi_pad: [__u8; 7] }
#[repr(C)] pub union nilfs_binfo { pub bi_v: nilfs_binfo_v, pub bi_dat: nilfs_binfo_dat }

#[repr(C)]
pub struct nilfs_segment_summary {
    pub ss_datasum: __le32, pub ss_sumsum: __le32, pub ss_magic: __le32,
    pub ss_bytes: __le16, pub ss_flags: __le16, pub ss_seq: __le64,
    pub ss_create: __le64, pub ss_next: __le64, pub ss_nblocks: __le32,
    pub ss_nfinfo: __le32, pub ss_sumbytes: __le32, pub ss_pad: __le32, pub ss_cno: __le64,
}
pub const NILFS_SEGSUM_MAGIC: u32 = 0x1eaffa11;
pub const NILFS_SS_LOGBGN: u32 = 0x0001;
pub const NILFS_SS_LOGEND: u32 = 0x0002;
pub const NILFS_SS_SR: u32 = 0x0004;
pub const NILFS_SS_SYNDT: u32 = 0x0008;
pub const NILFS_SS_GC: u32 = 0x0010;

#[repr(C)] pub struct nilfs_btree_node { pub bn_flags: __u8, pub bn_level: __u8, pub bn_nchildren: __le16, pub bn_pad: __le32 }
pub const NILFS_BTREE_NODE_ROOT: u32 = 0x01;
pub const NILFS_BTREE_LEVEL_DATA: u32 = 0;
pub const NILFS_BTREE_LEVEL_NODE_MIN: u32 = NILFS_BTREE_LEVEL_DATA + 1;
pub const NILFS_BTREE_LEVEL_MAX: u32 = 14;
#[repr(C)] pub struct nilfs_direct_node { pub dn_flags: __u8, pub pad: [__u8; 7] }
#[repr(C)] pub struct nilfs_palloc_group_desc { pub pg_nfrees: __le32 }
#[repr(C)] pub struct nilfs_dat_entry { pub de_blocknr: __le64, pub de_start: __le64, pub de_end: __le64, pub de_rsv: __le64 }
pub const NILFS_MIN_DAT_ENTRY_SIZE: usize = 32;
#[repr(C)] pub struct nilfs_snapshot_list { pub ssl_next: __le64, pub ssl_prev: __le64 }

#[repr(C)]
pub struct nilfs_checkpoint {
    pub cp_flags: __le32, pub cp_checkpoints_count: __le32, pub cp_snapshot_list: nilfs_snapshot_list,
    pub cp_cno: __le64, pub cp_create: __le64, pub cp_nblk_inc: __le64,
    pub cp_inodes_count: __le64, pub cp_blocks_count: __le64, pub cp_ifile_inode: nilfs_inode,
}
pub const NILFS_MIN_CHECKPOINT_SIZE: usize = 64 + NILFS_MIN_INODE_SIZE;
pub const NILFS_CHECKPOINT_SNAPSHOT: u32 = 0;
pub const NILFS_CHECKPOINT_INVALID: u32 = 1;
pub const NILFS_CHECKPOINT_SKETCH: u32 = 2;
pub const NILFS_CHECKPOINT_MINOR: u32 = 3;

#[inline] pub unsafe fn nilfs_checkpoint_set_snapshot(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) | (1u32 << NILFS_CHECKPOINT_SNAPSHOT)); }
#[inline] pub unsafe fn nilfs_checkpoint_clear_snapshot(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) & !(1u32 << NILFS_CHECKPOINT_SNAPSHOT)); }
#[inline] pub unsafe fn nilfs_checkpoint_snapshot(cp: *const nilfs_checkpoint) -> i32 { ((__le32_to_cpu((*cp).cp_flags) & (1u32 << NILFS_CHECKPOINT_SNAPSHOT)) != 0) as i32 }
#[inline] pub unsafe fn nilfs_checkpoint_set_invalid(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) | (1u32 << NILFS_CHECKPOINT_INVALID)); }
#[inline] pub unsafe fn nilfs_checkpoint_clear_invalid(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) & !(1u32 << NILFS_CHECKPOINT_INVALID)); }
#[inline] pub unsafe fn nilfs_checkpoint_invalid(cp: *const nilfs_checkpoint) -> i32 { ((__le32_to_cpu((*cp).cp_flags) & (1u32 << NILFS_CHECKPOINT_INVALID)) != 0) as i32 }
#[inline] pub unsafe fn nilfs_checkpoint_set_minor(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) | (1u32 << NILFS_CHECKPOINT_MINOR)); }
#[inline] pub unsafe fn nilfs_checkpoint_clear_minor(cp: *mut nilfs_checkpoint) { (*cp).cp_flags = __cpu_to_le32(__le32_to_cpu((*cp).cp_flags) & !(1u32 << NILFS_CHECKPOINT_MINOR)); }
#[inline] pub unsafe fn nilfs_checkpoint_minor(cp: *const nilfs_checkpoint) -> i32 { ((__le32_to_cpu((*cp).cp_flags) & (1u32 << NILFS_CHECKPOINT_MINOR)) != 0) as i32 }

#[repr(C)] pub struct nilfs_cpfile_header { pub ch_ncheckpoints: __le64, pub ch_nsnapshots: __le64, pub ch_snapshot_list: nilfs_snapshot_list }
pub const NILFS_CPFILE_FIRST_CHECKPOINT_OFFSET: usize = (core::mem::size_of::<nilfs_cpfile_header>() + core::mem::size_of::<nilfs_checkpoint>() - 1) / core::mem::size_of::<nilfs_checkpoint>();
#[repr(C)] pub struct nilfs_segment_usage { pub su_lastmod: __le64, pub su_nblocks: __le32, pub su_flags: __le32 }
pub const NILFS_MIN_SEGMENT_USAGE_SIZE: usize = 16;
pub const NILFS_SEGMENT_USAGE_ACTIVE: u32 = 0;
pub const NILFS_SEGMENT_USAGE_DIRTY: u32 = 1;
pub const NILFS_SEGMENT_USAGE_ERROR: u32 = 2;
#[inline] pub unsafe fn nilfs_segment_usage_set_active(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) | (1u32 << NILFS_SEGMENT_USAGE_ACTIVE)); }
#[inline] pub unsafe fn nilfs_segment_usage_clear_active(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) & !(1u32 << NILFS_SEGMENT_USAGE_ACTIVE)); }
#[inline] pub unsafe fn nilfs_segment_usage_active(su: *const nilfs_segment_usage) -> i32 { ((__le32_to_cpu((*su).su_flags) & (1u32 << NILFS_SEGMENT_USAGE_ACTIVE)) != 0) as i32 }
#[inline] pub unsafe fn nilfs_segment_usage_set_dirty(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) | (1u32 << NILFS_SEGMENT_USAGE_DIRTY)); }
#[inline] pub unsafe fn nilfs_segment_usage_clear_dirty(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) & !(1u32 << NILFS_SEGMENT_USAGE_DIRTY)); }
#[inline] pub unsafe fn nilfs_segment_usage_dirty(su: *const nilfs_segment_usage) -> i32 { ((__le32_to_cpu((*su).su_flags) & (1u32 << NILFS_SEGMENT_USAGE_DIRTY)) != 0) as i32 }
#[inline] pub unsafe fn nilfs_segment_usage_set_error(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) | (1u32 << NILFS_SEGMENT_USAGE_ERROR)); }
#[inline] pub unsafe fn nilfs_segment_usage_clear_error(su: *mut nilfs_segment_usage) { (*su).su_flags = __cpu_to_le32(__le32_to_cpu((*su).su_flags) & !(1u32 << NILFS_SEGMENT_USAGE_ERROR)); }
#[inline] pub unsafe fn nilfs_segment_usage_error(su: *const nilfs_segment_usage) -> i32 { ((__le32_to_cpu((*su).su_flags) & (1u32 << NILFS_SEGMENT_USAGE_ERROR)) != 0) as i32 }
#[inline] pub unsafe fn nilfs_segment_usage_set_clean(su: *mut nilfs_segment_usage) { (*su).su_lastmod = __cpu_to_le64(0); (*su).su_nblocks = __cpu_to_le32(0); (*su).su_flags = __cpu_to_le32(0); }
#[inline] pub unsafe fn nilfs_segment_usage_clean(su: *const nilfs_segment_usage) -> i32 { (__le32_to_cpu((*su).su_flags) == 0) as i32 }
#[repr(C)] pub struct nilfs_sufile_header { pub sh_ncleansegs: __le64, pub sh_ndirtysegs: __le64, pub sh_last_alloc: __le64 }
pub const NILFS_SUFILE_FIRST_SEGMENT_USAGE_OFFSET: usize = (core::mem::size_of::<nilfs_sufile_header>() + core::mem::size_of::<nilfs_segment_usage>() - 1) / core::mem::size_of::<nilfs_segment_usage>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
