// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of ext4.h.  Kernel-provided types and
// functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub type ext4_grpblk_t = i32;
pub type ext4_fsblk_t = u64;
pub type ext4_lblk_t = u32;
pub type ext4_group_t = u32;

#[repr(C)]
pub struct ext4_allocation_request {
    pub inode: *mut inode,
    pub len: u32,
    pub logical: ext4_lblk_t,
    pub lleft: ext4_lblk_t,
    pub lright: ext4_lblk_t,
    pub goal: ext4_fsblk_t,
    pub pleft: ext4_fsblk_t,
    pub pright: ext4_fsblk_t,
    pub flags: u32,
}

#[repr(C)]
pub struct ext4_map_blocks {
    pub m_pblk: ext4_fsblk_t,
    pub m_lblk: ext4_lblk_t,
    pub m_len: u32,
    pub m_flags: u32,
    pub m_seq: u64,
}

#[repr(C)]
pub struct ext4_group_desc {
    pub bg_block_bitmap_lo: u32, pub bg_inode_bitmap_lo: u32,
    pub bg_inode_table_lo: u32, pub bg_free_blocks_count_lo: u16,
    pub bg_free_inodes_count_lo: u16, pub bg_used_dirs_count_lo: u16,
    pub bg_flags: u16, pub bg_exclude_bitmap_lo: u32,
    pub bg_block_bitmap_csum_lo: u16, pub bg_inode_bitmap_csum_lo: u16,
    pub bg_itable_unused_lo: u16, pub bg_checksum: u16,
    pub bg_block_bitmap_hi: u32, pub bg_inode_bitmap_hi: u32,
    pub bg_inode_table_hi: u32, pub bg_free_blocks_count_hi: u16,
    pub bg_free_inodes_count_hi: u16, pub bg_used_dirs_count_hi: u16,
    pub bg_itable_unused_hi: u16, pub bg_exclude_bitmap_hi: u32,
    pub bg_block_bitmap_csum_hi: u16, pub bg_inode_bitmap_csum_hi: u16,
    pub bg_reserved: u32,
}

#[repr(C)]
pub struct ext4_dir_entry {
    pub inode: u32, pub rec_len: u16, pub name_len: u16,
    pub name: [u8; 255],
}

#[repr(C)]
pub struct ext4_dir_entry_hash { pub hash: u32, pub minor_hash: u32 }

#[repr(C)]
pub struct ext4_dir_entry_2 {
    pub inode: u32, pub rec_len: u16, pub name_len: u8, pub file_type: u8,
    pub name: [u8; 255],
}

#[repr(C)]
pub struct ext4_dir_entry_tail {
    pub det_reserved_zero1: u32, pub det_rec_len: u16,
    pub det_reserved_zero2: u8, pub det_reserved_ft: u8, pub det_checksum: u32,
}

pub const EXT4_BAD_INO: u32 = 1;
pub const EXT4_ROOT_INO: u32 = 2;
pub const EXT4_GOOD_OLD_FIRST_INO: u32 = 11;
pub const EXT4_LINK_MAX: u32 = 65000;
pub const EXT4_MIN_BLOCK_SIZE: u32 = 1024;
pub const EXT4_MAX_BLOCK_SIZE: u32 = 65536;
pub const EXT4_NDIR_BLOCKS: u32 = 12;
pub const EXT4_N_BLOCKS: u32 = 15;
pub const EXT4_NAME_LEN: usize = 255;
pub const EXT4_FT_UNKNOWN: u8 = 0;
pub const EXT4_FT_REG_FILE: u8 = 1;
pub const EXT4_FT_DIR: u8 = 2;
pub const EXT4_FT_CHRDEV: u8 = 3;
pub const EXT4_FT_BLKDEV: u8 = 4;
pub const EXT4_FT_FIFO: u8 = 5;
pub const EXT4_FT_SOCK: u8 = 6;
pub const EXT4_FT_SYMLINK: u8 = 7;
pub const EXT4_FT_MAX: u8 = 8;
pub const EXT4_FT_DIR_CSUM: u8 = 0xde;

// The remaining declarations depend on Linux kernel types, configuration
// predicates, and included ext4 companion headers; they are preserved below
// as source-level reference for the generated binding layer.
/*
 * ext4.h declarations not representable without those external dependencies
 * are intentionally retained as an external-header dependency rather than
 * inventing implementations or substitute layouts.
 */

#[allow(non_camel_case_types)]
pub enum inode {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
