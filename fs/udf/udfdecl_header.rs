/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding UDF and kernel translation units.
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

pub const UDF_DEFAULT_PREALLOC_BLOCKS: u32 = 8;
pub const UDF_EXTENT_LENGTH_MASK: u32 = 0x3fff_ffff;
pub const UDF_EXTENT_FLAG_MASK: u32 = 0xc000_0000;
pub const UDF_INVALID_ID: u32 = u32::MAX;
pub const UDF_NAME_PAD: usize = 4;
pub const UDF_NAME_LEN: usize = 254;
pub const UDF_NAME_LEN_CS0: usize = 255;

pub type udf_pblk_t = u32;
pub type loff_t = i64;
pub type sector_t = u64;
pub type umode_t = u16;
pub type __le32 = u32;

#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { pub b_data: *mut c_void }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct tag { _private: [u8; 0] }
#[repr(C)] pub struct kernel_lb_addr { pub logicalBlockNum: u32, pub partitionReferenceNum: u16 }
#[repr(C)] pub struct extent_position { _private: [u8; 0] }
#[repr(C)] pub struct fileIdentDesc { pub lengthOfImpUse: u16, pub lengthFileIdent: u8 }
#[repr(C)] pub struct logicalVolIntegrityDesc { pub integrityType: __le32 }
#[repr(C)] pub struct timestamp { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i32 }
#[repr(C)] pub struct unallocSpaceEntry { _private: [u8; 0] }
#[repr(C)] pub struct extendedFileEntry { _private: [u8; 0] }
#[repr(C)] pub struct fileEntry { _private: [u8; 0] }
#[repr(C)] pub struct udf_inode_info { pub i_use: bool, pub i_efe: bool, pub i_lenEAttr: u32, pub i_alloc_type: u32 }
#[repr(C)] pub struct genericFormat { _private: [u8; 0] }
#[repr(C)] pub struct long_ad { _private: [u8; 0] }
#[repr(C)] pub struct short_ad { _private: [u8; 0] }
#[repr(C)] pub struct export_operations { _private: [u8; 0] }
#[repr(C)] pub struct inode_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct address_space_operations { _private: [u8; 0] }

#[repr(C)]
pub struct udf_fileident_iter {
    pub dir: *mut inode,
    pub pos: loff_t,
    pub bh: [*mut buffer_head; 2],
    pub eloc: kernel_lb_addr,
    pub elen: u32,
    pub loffset: sector_t,
    pub epos: extent_position,
    pub fi: fileIdentDesc,
    pub name: *mut u8,
    pub namebuf: *mut u8,
}

#[repr(C)] pub struct udf_vds_record { pub block: u32, pub volDescSeqNum: u32 }
#[repr(C)] pub struct generic_desc { pub descTag: tag, pub volDescSeqNum: __le32 }

// The following accessors/macros are supplied by the surrounding UDF headers.
extern "C" {
    pub fn UDF_I(inode: *mut inode) -> *mut udf_inode_info;
    pub fn UDF_SB(sb: *mut super_block) -> *mut c_void;
    pub fn udf_tag_checksum(t: *const tag) -> u8;
    pub fn udf_get_pblock(sb: *mut super_block, block: u32, partition: u16, offset: u32) -> u32;
    pub fn __udf_iget(sb: *mut super_block, ino: *mut kernel_lb_addr, hidden_inode: bool) -> *mut inode;
}

#[inline]
pub unsafe fn udf_file_entry_alloc_offset(i: *mut inode) -> usize {
    let iinfo = UDF_I(i);
    if (*iinfo).i_use { core::mem::size_of::<unallocSpaceEntry>() }
    else if (*iinfo).i_efe { core::mem::size_of::<extendedFileEntry>() + (*iinfo).i_lenEAttr as usize }
    else { core::mem::size_of::<fileEntry>() + (*iinfo).i_lenEAttr as usize }
}

#[inline]
pub unsafe fn udf_ext0_offset(i: *mut inode) -> usize {
    if (*UDF_I(i)).i_alloc_type == 0x0000_0010 { udf_file_entry_alloc_offset(i) } else { 0 }
}

#[inline]
pub unsafe fn udf_iget_special(sb: *mut super_block, ino: *mut kernel_lb_addr) -> *mut inode { __udf_iget(sb, ino, true) }
#[inline]
pub unsafe fn udf_iget(sb: *mut super_block, ino: *mut kernel_lb_addr) -> *mut inode { __udf_iget(sb, ino, false) }

#[inline]
pub unsafe fn udf_dir_entry_len(cfi: *mut fileIdentDesc) -> usize {
    (core::mem::size_of::<fileIdentDesc>() + (*cfi).lengthOfImpUse as usize + (*cfi).lengthFileIdent as usize + 3) & !3
}

#[inline]
pub unsafe fn udf_get_lb_pblock(sb: *mut super_block, loc: *mut kernel_lb_addr, offset: u32) -> u32 {
    udf_get_pblock(sb, (*loc).logicalBlockNum, (*loc).partitionReferenceNum, offset)
}

extern "C" {
    pub static udf_export_ops: export_operations;
    pub static udf_dir_inode_operations: inode_operations;
    pub static udf_dir_operations: file_operations;
    pub static udf_file_inode_operations: inode_operations;
    pub static udf_file_operations: file_operations;
    pub static udf_symlink_inode_operations: inode_operations;
    pub static udf_aops: address_space_operations;
    pub static udf_symlink_aops: address_space_operations;
    pub fn _udf_err(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn _udf_warn(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn lvid_get_unique_id(sb: *mut super_block) -> u64;
    pub fn udf_find_metadata_inode_efe(sb: *mut super_block, meta_file_loc: u32, partition_num: u32) -> *mut inode;
    pub fn udf_ioctl(f: *mut file, cmd: u32, arg: c_ulong) -> c_long;
    pub fn udf_expand_file_adinicb(i: *mut inode) -> c_int;
    pub fn udf_bread(i: *mut inode, block: udf_pblk_t, create: c_int, err: *mut c_int) -> *mut buffer_head;
    pub fn udf_setsize(i: *mut inode, size: loff_t) -> c_int;
    pub fn udf_evict_inode(i: *mut inode);
    pub fn udf_write_inode(i: *mut inode, wbc: *mut writeback_control) -> c_int;
    pub fn udf_sync_inode_metadata(i: *mut inode, wbc: *mut writeback_control) -> c_int;
    pub fn inode_bmap(i: *mut inode, block: sector_t, pos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: *mut u32, offset: *mut sector_t, etype: *mut i8) -> c_int;
    pub fn udf_get_block(i: *mut inode, block: sector_t, bh: *mut buffer_head, create: c_int) -> c_int;
    pub fn udf_setup_indirect_aext(i: *mut inode, block: udf_pblk_t, epos: *mut extent_position) -> c_int;
    pub fn __udf_add_aext(i: *mut inode, epos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: u32, inc: c_int) -> c_int;
    pub fn udf_add_aext(i: *mut inode, epos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: u32, inc: c_int) -> c_int;
    pub fn udf_write_aext(i: *mut inode, epos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: u32, inc: c_int);
    pub fn udf_delete_aext(i: *mut inode, epos: extent_position, eloc: *mut kernel_lb_addr) -> i8;
    pub fn udf_next_aext(i: *mut inode, epos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: *mut u32, etype: *mut i8, inc: c_int) -> i8;
    pub fn udf_current_aext(i: *mut inode, epos: *mut extent_position, eloc: *mut kernel_lb_addr, elen: *mut u32, etype: *mut i8, inc: c_int) -> i8;
    pub fn udf_update_extra_perms(i: *mut inode, mode: umode_t);
    pub fn udf_add_extendedattr(i: *mut inode, t: u32, l: u32, ty: u8) -> *mut genericFormat;
    pub fn udf_get_extendedattr(i: *mut inode, t: u32, ty: u8) -> *mut genericFormat;
    pub fn udf_read_tagged(sb: *mut super_block, block: u32, location: u32, ident: *mut u16) -> *mut buffer_head;
    pub fn udf_read_ptagged(sb: *mut super_block, loc: *mut kernel_lb_addr, block: u32, ident: *mut u16) -> *mut buffer_head;
    pub fn udf_update_tag(data: *mut c_char, length: c_int);
    pub fn udf_new_tag(data: *mut c_char, ident: u16, version: u16, serial: u16, location: u32, length: c_int);
    pub fn udf_get_last_session(sb: *mut super_block) -> u32;
    pub fn udf_get_last_block(sb: *mut super_block) -> udf_pblk_t;
    pub fn udf_get_pblock_virt15(sb: *mut super_block, block: u32, part: u16, off: u32) -> u32;
    pub fn udf_get_pblock_virt20(sb: *mut super_block, block: u32, part: u16, off: u32) -> u32;
    pub fn udf_get_pblock_spar15(sb: *mut super_block, block: u32, part: u16, off: u32) -> u32;
    pub fn udf_get_pblock_meta25(sb: *mut super_block, block: u32, part: u16, off: u32) -> u32;
    pub fn udf_relocate_blocks(sb: *mut super_block, block: c_long, count: *mut c_long) -> c_int;
    pub fn udf_get_filename(sb: *mut super_block, src: *const u8, slen: c_int, dst: *mut u8, dlen: c_int) -> c_int;
    pub fn udf_put_filename(sb: *mut super_block, src: *const u8, slen: c_int, dst: *mut u8, dlen: c_int) -> c_int;
    pub fn udf_dstrCS0toChar(sb: *mut super_block, dst: *mut u8, dlen: c_int, src: *const u8, slen: c_int) -> c_int;
    pub fn udf_free_inode(i: *mut inode);
    pub fn udf_new_inode(dir: *mut inode, mode: umode_t) -> *mut inode;
    pub fn udf_truncate_tail_extent(i: *mut inode);
    pub fn udf_discard_prealloc(i: *mut inode);
    pub fn udf_truncate_extents(i: *mut inode) -> c_int;
    pub fn udf_free_blocks(sb: *mut super_block, i: *mut inode, loc: *mut kernel_lb_addr, blocks: u32, count: u32);
    pub fn udf_prealloc_blocks(sb: *mut super_block, i: *mut inode, partition: u16, goal: u32, count: u32) -> c_int;
    pub fn udf_new_block(sb: *mut super_block, i: *mut inode, partition: u16, goal: u32, err: *mut c_int) -> udf_pblk_t;
    pub fn udf_fiiter_init(iter: *mut udf_fileident_iter, dir: *mut inode, pos: loff_t) -> c_int;
    pub fn udf_fiiter_advance(iter: *mut udf_fileident_iter) -> c_int;
    pub fn udf_fiiter_release(iter: *mut udf_fileident_iter);
    pub fn udf_fiiter_write_fi(iter: *mut udf_fileident_iter, impuse: *mut u8);
    pub fn udf_fiiter_update_elen(iter: *mut udf_fileident_iter, new_elen: u32);
    pub fn udf_fiiter_append_blk(iter: *mut udf_fileident_iter) -> c_int;
    pub fn udf_get_filelongad(data: *mut u8, offset: c_int, elen: *mut u32, inc: c_int) -> *mut long_ad;
    pub fn udf_get_fileshortad(data: *mut u8, offset: c_int, elen: *mut u32, inc: c_int) -> *mut short_ad;
    pub fn udf_disk_stamp_to_time(dest: *mut timespec64, src: timestamp);
    pub fn udf_time_to_disk_stamp(dest: *mut timestamp, src: timespec64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
