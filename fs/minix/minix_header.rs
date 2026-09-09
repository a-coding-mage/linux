/* SPDX-License-Identifier: GPL-2.0 */

// Translated from minix.h. Linux header dependencies are supplied externally.

pub const MINIX_V1: u16 = 0x0001; // original minix fs
pub const MINIX_V2: u16 = 0x0002; // minix V2 fs
pub const MINIX_V3: u16 = 0x0003; // minix V3 fs

#[repr(C)]
pub union MinixInodeInfoData {
    pub i1_data: [u16; 16],
    pub i2_data: [u32; 16],
}

#[repr(C)]
pub struct MinixInodeInfo {
    pub u: MinixInodeInfoData,
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub vfs_inode: inode,
}

#[repr(C)]
pub struct MinixSbInfo {
    pub s_ninodes: c_ulong,
    pub s_nzones: c_ulong,
    pub s_imap_blocks: c_ulong,
    pub s_zmap_blocks: c_ulong,
    pub s_firstdatazone: c_ulong,
    pub s_log_zone_size: c_ulong,
    pub s_dirsize: c_int,
    pub s_namelen: c_int,
    pub s_imap: *mut *mut buffer_head,
    pub s_zmap: *mut *mut buffer_head,
    pub s_sbh: *mut buffer_head,
    pub s_ms: *mut minix_super_block,
    pub s_mount_state: c_ushort,
    pub s_version: c_ushort,
}

extern "C" {
    pub fn __minix_error_inode(
        inode: *mut inode,
        function: *const c_char,
        line: c_uint,
        fmt: *const c_char,
        ...,
    );
    pub fn minix_iget(sb: *mut super_block, ino: c_ulong) -> *mut inode;
    pub fn minix_V1_raw_inode(sb: *mut super_block, ino: ino_t, bh: *mut *mut buffer_head) -> *mut minix_inode;
    pub fn minix_V2_raw_inode(sb: *mut super_block, ino: ino_t, bh: *mut *mut buffer_head) -> *mut minix2_inode;
    pub fn minix_new_inode(dir: *const inode, mode: umode_t) -> *mut inode;
    pub fn minix_free_inode(inode: *mut inode);
    pub fn minix_count_free_inodes(sb: *mut super_block) -> c_ulong;
    pub fn minix_new_block(inode: *mut inode) -> c_int;
    pub fn minix_free_block(inode: *mut inode, block: c_ulong);
    pub fn minix_count_free_blocks(sb: *mut super_block) -> c_ulong;
    pub fn minix_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int;
    pub fn minix_prepare_chunk(folio: *mut folio, pos: loff_t, len: c_uint) -> c_int;
    pub fn minix_get_metadata_bhs(inode: *mut inode) -> *mut mapping_metadata_bhs;
    pub fn V1_minix_truncate(inode: *mut inode);
    pub fn V2_minix_truncate(inode: *mut inode);
    pub fn minix_truncate(inode: *mut inode);
    pub fn minix_set_inode(inode: *mut inode, dev: dev_t);
    pub fn V1_minix_get_block(inode: *mut inode, block: c_long, bh: *mut buffer_head, create: c_int) -> c_int;
    pub fn V2_minix_get_block(inode: *mut inode, block: c_long, bh: *mut buffer_head, create: c_int) -> c_int;
    pub fn V1_minix_blocks(size: loff_t, sb: *mut super_block) -> c_uint;
    pub fn V2_minix_blocks(size: loff_t, sb: *mut super_block) -> c_uint;
    pub fn minix_find_entry(dentry: *mut dentry, res_page: *mut *mut folio) -> *mut minix_dir_entry;
    pub fn minix_add_link(dentry: *mut dentry, inode: *mut inode) -> c_int;
    pub fn minix_delete_entry(de: *mut minix_dir_entry, folio: *mut folio) -> c_int;
    pub fn minix_make_empty(inode: *mut inode, dir: *mut inode) -> c_int;
    pub fn minix_empty_dir(inode: *mut inode) -> c_int;
    pub fn minix_set_link(de: *mut minix_dir_entry, folio: *mut folio, inode: *mut inode) -> c_int;
    pub fn minix_dotdot(inode: *mut inode, res_page: *mut *mut folio) -> *mut minix_dir_entry;
    pub fn minix_inode_by_name(dentry: *mut dentry) -> ino_t;
    pub static minix_file_inode_operations: inode_operations;
    pub static minix_dir_inode_operations: inode_operations;
    pub static minix_file_operations: file_operations;
    pub static minix_dir_operations: file_operations;
}

#[inline]
pub unsafe fn minix_sb(sb: *mut super_block) -> *mut MinixSbInfo {
    (*sb).s_fs_info as *mut MinixSbInfo
}

#[inline]
pub unsafe fn minix_i(inode: *mut inode) -> *mut MinixInodeInfo {
    container_of!(inode, MinixInodeInfo, vfs_inode)
}

#[inline]
pub const fn minix_blocks_needed(bits: c_uint, blocksize: c_uint) -> c_uint {
    (bits + blocksize * 8 - 1) / (blocksize * 8)
}

// The following bitmap operations depend on the build-time endian configuration.
// CONFIG_MINIX_FS_NATIVE_ENDIAN together with CONFIG_MINIX_FS_BIG_ENDIAN_16BIT_INDEXED
// is invalid in the original header. The corresponding external kernel bitmap
// primitives are intentionally referenced by these declarations/macros.

#[macro_export]
macro_rules! minix_error_inode {
    ($inode:expr, $fmt:expr $(, $arg:expr)*) => {
        unsafe { $crate::__minix_error_inode($inode, concat!(module_path!(), "\\0").as_ptr() as *const c_char, line!(), $fmt $(, $arg)*) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
