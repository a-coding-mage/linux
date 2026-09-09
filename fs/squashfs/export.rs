// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * export.c
 */

/*
 * This file implements code to make Squashfs filesystems exportable (NFS etc.)
 *
 * The export code uses an inode lookup table to map inode numbers passed in
 * filehandles to an inode location on disk.  This table is stored compressed
 * into metadata blocks.  A second index table is used to locate these.
 */

use core::ffi::c_void;

pub type __le64 = u64;
pub type u64_ = u64;

#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut c_void,
}
#[repr(C)]
pub struct squashfs_sb_info {
    pub inodes: u32,
    pub inode_lookup_table: *mut __le64,
}
#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct fid {
    pub i32: fid_i32,
}
#[repr(C)]
pub struct fid_i32 {
    pub ino: u32,
    pub parent_ino: u32,
}

#[repr(C)]
pub struct export_operations {
    pub encode_fh: Option<unsafe extern "C" fn()>,
    pub fh_to_dentry: Option<unsafe extern "C" fn(*mut super_block, *mut fid, i32, i32) -> *mut dentry>,
    pub fh_to_parent: Option<unsafe extern "C" fn(*mut super_block, *mut fid, i32, i32) -> *mut dentry>,
    pub get_parent: Option<unsafe extern "C" fn(*mut dentry) -> *mut dentry>,
}

extern "C" {
    fn squashfs_read_metadata(sb: *mut super_block, buffer: *mut __le64,
        start: *mut u64, offset: *mut i32, length: usize) -> i32;
    fn d_obtain_alias(inode: *mut inode) -> *mut dentry;
    fn squashfs_iget(sb: *mut super_block, ino: i64, ino_num: u32) -> *mut inode;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn squashfs_i(inode: *mut inode) -> *mut squashfs_inode_info;
    fn squashfs_read_table(sb: *mut super_block, start: u64, length: u32) -> *mut __le64;
    fn kfree(pointer: *mut __le64);
    fn generic_encode_ino32_fh();
}

#[repr(C)]
pub struct squashfs_inode_info {
    pub parent: u32,
}

const EINVAL: i64 = 22;
const ENOENT: i64 = 2;
const FILEID_INO32_GEN: i32 = 1;
const FILEID_INO32_GEN_PARENT: i32 = 2;
const SQUASHFS_METADATA_SIZE: u64 = 8192;
const SQUASHFS_BLOCK_OFFSET: u64 = 8191;

#[inline]
unsafe fn squashfs_lookup_block(value: i32) -> usize { (value as usize) / 128 }
#[inline]
unsafe fn squashfs_lookup_block_offset(value: i32) -> i32 { (value & 127) * 8 }
#[inline]
unsafe fn squashfs_lookup_block_bytes(inodes: u32) -> u32 { ((inodes + 127) / 128) * 8 }
#[inline]
unsafe fn squashfs_lookup_blocks(inodes: u32) -> u32 { (inodes + 127) / 128 }

static mut SQUASHFS_EXPORT_OPS: export_operations = export_operations {
    encode_fh: Some(generic_encode_ino32_fh),
    fh_to_dentry: Some(squashfs_fh_to_dentry),
    fh_to_parent: Some(squashfs_fh_to_parent),
    get_parent: Some(squashfs_get_parent),
};

unsafe fn squashfs_inode_lookup(sb: *mut super_block, ino_num: i32) -> i64 {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let blk = squashfs_lookup_block(ino_num - 1);
    let mut offset = squashfs_lookup_block_offset(ino_num - 1);
    let mut start = (*msblk).inode_lookup_table.add(blk).read();
    let mut ino: __le64 = 0;

    if ino_num == 0 || (ino_num - 1) as u32 >= (*msblk).inodes { return -EINVAL; }
    let err = squashfs_read_metadata(sb, &mut ino, &mut start, &mut offset, core::mem::size_of::<__le64>());
    if err < 0 { return err as i64; }
    ino as i64
}

unsafe fn squashfs_export_iget(sb: *mut super_block, ino_num: u32) -> *mut dentry {
    let ino = squashfs_inode_lookup(sb, ino_num as i32);
    if ino >= 0 { d_obtain_alias(squashfs_iget(sb, ino, ino_num)) } else { (-ENOENT) as *mut dentry }
}

unsafe extern "C" fn squashfs_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if (fh_type != FILEID_INO32_GEN && fh_type != FILEID_INO32_GEN_PARENT) || fh_len < 2 { return core::ptr::null_mut(); }
    squashfs_export_iget(sb, (*fid).i32.ino)
}

unsafe extern "C" fn squashfs_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if fh_type != FILEID_INO32_GEN_PARENT || fh_len < 4 { return core::ptr::null_mut(); }
    squashfs_export_iget(sb, (*fid).i32.parent_ino)
}

unsafe extern "C" fn squashfs_get_parent(child: *mut dentry) -> *mut dentry {
    let inode = d_inode(child);
    let parent_ino = (*squashfs_i(inode)).parent;
    squashfs_export_iget((*inode).i_sb, parent_ino)
}

pub unsafe extern "C" fn squashfs_read_inode_lookup_table(sb: *mut super_block,
    lookup_table_start: u64, next_table: u64, inodes: u32) -> *mut __le64 {
    let length = squashfs_lookup_block_bytes(inodes);
    let indexes = squashfs_lookup_blocks(inodes);
    if inodes == 0 || length as u64 != next_table - lookup_table_start { return (-EINVAL) as *mut __le64; }
    let table = squashfs_read_table(sb, lookup_table_start, length);
    if table.is_null() { return table; }
    for n in 0..(indexes - 1) {
        let start = table.add(n as usize).read();
        let end = table.add((n + 1) as usize).read();
        if start >= end || end - start > SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET { kfree(table); return (-EINVAL) as *mut __le64; }
    }
    let start = table.add((indexes - 1) as usize).read();
    if start >= lookup_table_start || lookup_table_start - start > SQUASHFS_METADATA_SIZE + SQUASHFS_BLOCK_OFFSET { kfree(table); return (-EINVAL) as *mut __le64; }
    table
}

pub static squashfs_export_ops: export_operations = export_operations {
    encode_fh: Some(generic_encode_ino32_fh),
    fh_to_dentry: Some(squashfs_fh_to_dentry),
    fh_to_parent: Some(squashfs_fh_to_parent),
    get_parent: Some(squashfs_get_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
