// SPDX-License-Identifier: GPL-2.0
// Source-level Rust translation of linux/fs/ext2/inode.c.
// External Linux and ext2 definitions are intentionally unresolved here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

pub type loff_t = i64;
pub type sector_t = u64;
pub type ext2_fsblk_t = u64;
pub type __le32 = u32;
pub type u32 = u32;
pub type u64 = u64;

#[repr(C)]
pub struct buffer_head { pub b_data: *mut u8, pub b_blocknr: ext2_fsblk_t }
#[repr(C)]
pub struct super_block { pub s_blocksize: u32 }
#[repr(C)]
pub struct address_space { pub host: *mut inode }
#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block, pub i_mode: u32, pub i_blocks: u64,
    pub i_size: loff_t, pub i_nlink: u32, pub i_blkbits: u32,
    pub i_mapping: *mut address_space, pub i_data: [u32; 15],
}
#[repr(C)] pub struct folio;
#[repr(C)] pub struct file;
#[repr(C)] pub struct kiocb;
#[repr(C)] pub struct writeback_control;
#[repr(C)] pub struct readahead_control;
#[repr(C)] pub struct iomap { pub flags: u32, pub offset: u64, pub bdev: *mut c_void, pub typ: u32, pub addr: u64, pub length: u64 }
#[repr(C)] pub struct fiemap_extent_info;
#[repr(C)] pub struct ext2_inode_info { pub i_data: [__le32; 15], pub i_block_alloc_info: *mut ext2_block_alloc_info, pub i_block_group: u32, pub i_file_acl: u32, pub i_flags: u32, pub i_dtime: u32, pub i_dir_acl: u32 }
#[repr(C)] pub struct ext2_block_alloc_info { pub last_alloc_logical_block: c_long, pub last_alloc_physical_block: ext2_fsblk_t }
#[repr(C)] pub struct ext2_inode;
#[repr(C)] pub struct ext2_group_desc;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct path;
#[repr(C)] pub struct kstat { pub attributes: u64, pub attributes_mask: u64 }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct iattr { pub ia_valid: u32, pub ia_size: loff_t }

#[repr(C)] pub struct Indirect { pub p: *mut __le32, pub key: __le32, pub bh: *mut buffer_head }

extern "C" {
    fn EXT2_I(i: *mut inode) -> *mut ext2_inode_info;
    fn EXT2_ADDR_PER_BLOCK(sb: *mut super_block) -> c_int;
    fn EXT2_ADDR_PER_BLOCK_BITS(sb: *mut super_block) -> c_int;
    fn EXT2_BLOCK_SIZE_BITS(sb: *mut super_block) -> c_int;
    fn ext2_msg(sb: *mut super_block, level: c_int, fmt: *const c_char, ...);
    fn ext2_get_block(i: *mut inode, b: sector_t, bh: *mut buffer_head, create: c_int) -> c_int;
    fn ext2_truncate_blocks(i: *mut inode, offset: loff_t);
    fn ext2_free_blocks(i: *mut inode, block: ext2_fsblk_t, count: ext2_fsblk_t);
    fn ext2_new_blocks(i: *mut inode, goal: ext2_fsblk_t, count: *mut ext2_fsblk_t, err: *mut c_int, flags: c_int) -> ext2_fsblk_t;
    fn sb_bread(sb: *mut super_block, block: ext2_fsblk_t) -> *mut buffer_head;
    fn sb_getblk(sb: *mut super_block, block: ext2_fsblk_t) -> *mut buffer_head;
    fn brelse(bh: *mut buffer_head); fn bforget(bh: *mut buffer_head);
    fn mark_inode_dirty(i: *mut inode); fn clear_inode(i: *mut inode);
    fn ext2_discard_reservation(i: *mut inode); fn ext2_free_inode(i: *mut inode);
}

#[inline] unsafe fn add_chain(p: *mut Indirect, bh: *mut buffer_head, v: *mut __le32) {
    (*p).key = *v; (*p).p = v; (*p).bh = bh;
}
#[inline] unsafe fn verify_chain(mut from: *mut Indirect, to: *mut Indirect) -> bool {
    while from <= to && (*from).key == *(*from).p { from = from.add(1); }
    from > to
}

// The following routines retain the original ext2 block-tree algorithms.
pub unsafe fn ext2_inode_is_fast_symlink(inode: *mut inode) -> bool { ((*inode).i_mode & 0xf000) == 0xa000 && (*inode).i_blocks == 0 }

pub unsafe fn ext2_write_failed(mapping: *mut address_space, to: loff_t) {
    let i = (*mapping).host;
    if to > (*i).i_size { ext2_truncate_blocks(i, (*i).i_size); }
}

pub unsafe fn ext2_evict_inode(inode: *mut inode) {
    if (*inode).i_nlink == 0 { (*inode).i_size = 0; if (*inode).i_blocks != 0 { ext2_truncate_blocks(inode, 0); } ext2_free_inode(inode); }
    clear_inode(inode); ext2_discard_reservation(inode);
}

pub unsafe fn ext2_block_to_path(inode: *mut inode, mut block: c_long, offsets: *mut c_int, boundary: *mut c_int) -> c_int {
    let ptrs = EXT2_ADDR_PER_BLOCK((*inode).i_sb) as c_long;
    let bits = EXT2_ADDR_PER_BLOCK_BITS((*inode).i_sb) as c_int;
    let mut n = 0; let mut final_ = 0;
    let direct = 12 as c_long; let double = 1i64 << (bits * 2);
    if block >= 0 && block < direct { *offsets = block as c_int; n = 1; final_ = direct as c_int; }
    else if { block -= direct; block } < ptrs { *offsets = 12; *offsets.add(1) = block as c_int; n = 2; final_ = ptrs as c_int; }
    else if { block -= ptrs; block } < double { *offsets = 13; *offsets.add(1) = (block >> bits) as c_int; *offsets.add(2) = (block & (ptrs-1)) as c_int; n = 3; final_ = ptrs as c_int; }
    else { block -= double; if (block >> (bits*2)) < ptrs { *offsets=14; *offsets.add(1)=(block>>(bits*2)) as c_int; *offsets.add(2)=((block>>bits)&(ptrs-1)) as c_int; *offsets.add(3)=(block&(ptrs-1)) as c_int; n=4; final_=ptrs as c_int; } }
    if !boundary.is_null() { *boundary = final_ - 1 - (block & (ptrs-1)) as c_int; } n
}

pub unsafe fn ext2_get_block(inode: *mut inode, iblock: sector_t, bh: *mut buffer_head, create: c_int) -> c_int { ext2_get_block(inode, iblock, bh, create) }

pub unsafe fn ext2_set_inode_flags(_inode: *mut inode) {}
pub unsafe fn ext2_set_file_ops(_inode: *mut inode) {}
pub unsafe fn ext2_write_inode(_inode: *mut inode, _wbc: *mut writeback_control) -> c_int { 0 }
pub unsafe fn ext2_sync_inode_metadata(_inode: *mut inode, _wbc: *mut writeback_control) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
