/* SPDX-License-Identifier: GPL-2.0 */
// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub enum isofs_file_format {
    isofs_file_normal = 0,
    isofs_file_sparse = 1,
    isofs_file_compressed = 2,
}

#[repr(C)]
pub struct iso_inode_info {
    pub i_iget5_block: ::core::primitive::usize,
    pub i_iget5_offset: ::core::primitive::usize,
    pub i_first_extent: u32,
    pub i_file_format: u8,
    pub i_format_parm: [u8; 3],
    pub i_next_section_block: ::core::primitive::usize,
    pub i_next_section_offset: ::core::primitive::usize,
    pub i_section_size: off_t,
    pub vfs_inode: inode,
}

#[repr(C)]
pub struct isofs_sb_info {
    pub s_ninodes: ::core::primitive::usize,
    pub s_nzones: ::core::primitive::usize,
    pub s_firstdatazone: ::core::primitive::usize,
    pub s_log_zone_size: ::core::primitive::usize,
    pub s_max_size: ::core::primitive::usize,
    pub s_rock_offset: i32,
    pub s_sbsector: s32,
    pub s_joliet_level: u8,
    pub s_mapping: u8,
    pub s_check: u8,
    pub s_session: u8,
    // C bit-fields; represented as their containing unsigned word.
    pub s_high_sierra: u32,
    pub s_rock: u32,
    pub s_cruft: u32,
    pub s_nocompress: u32,
    pub s_hide: u32,
    pub s_showassoc: u32,
    pub s_overriderockperm: u32,
    pub s_uid_set: u32,
    pub s_gid_set: u32,
    pub s_fmode: umode_t,
    pub s_dmode: umode_t,
    pub s_gid: kgid_t,
    pub s_uid: kuid_t,
    pub s_nls_iocharset: *mut nls_table,
}

pub const ISOFS_INVALID_MODE: umode_t = -1i32 as umode_t;

#[inline]
pub unsafe fn ISOFS_SB(sb: *mut super_block) -> *mut isofs_sb_info {
    (*sb).s_fs_info
}

#[inline]
pub unsafe fn ISOFS_I(inode_: *mut inode) -> *mut iso_inode_info {
    container_of!(inode_, iso_inode_info, vfs_inode)
}

#[inline]
pub unsafe fn isonum_711(p: *mut u8) -> i32 { *p as i32 }
#[inline]
pub unsafe fn isonum_712(p: *mut s8) -> i32 { *p as i32 }
#[inline]
pub unsafe fn isonum_721(p: *mut u8) -> u32 { get_unaligned_le16(p) }
#[inline]
pub unsafe fn isonum_722(p: *mut u8) -> u32 { get_unaligned_be16(p) }
#[inline]
pub unsafe fn isonum_723(p: *mut u8) -> u32 { get_unaligned_le16(p) }
#[inline]
pub unsafe fn isonum_731(p: *mut u8) -> u32 { get_unaligned_le32(p) }
#[inline]
pub unsafe fn isonum_732(p: *mut u8) -> u32 { get_unaligned_be32(p) }
#[inline]
pub unsafe fn isonum_733(p: *mut u8) -> u32 { get_unaligned_le32(p) }

pub const ISO_DATE_HIGH_SIERRA: i32 = 1 << 0;
pub const ISO_DATE_LONG_FORM: i32 = 1 << 1;
extern "C" { pub fn iso_date(p: *mut u8, flags: i32) -> timespec64; }

extern "C" {
    pub fn parse_rock_ridge_inode(de: *mut iso_directory_record, inode_: *mut inode, relocated: i32) -> i32;
    pub fn get_rock_ridge_filename(de: *mut iso_directory_record, name: *mut i8, inode_: *mut inode) -> i32;
    pub fn isofs_name_translate(de: *mut iso_directory_record, name: *mut i8, inode_: *mut inode) -> i32;
    pub fn isofs_dir_record_valid(de: *mut iso_directory_record, offset: ::core::primitive::usize, bufsize: ::core::primitive::usize) -> bool;
    pub fn get_joliet_filename(de: *mut iso_directory_record, name: *mut u8, inode_: *mut inode) -> i32;
    pub fn get_acorn_filename(de: *mut iso_directory_record, name: *mut i8, inode_: *mut inode) -> i32;
    pub fn isofs_lookup(inode_: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    pub fn isofs_bread(inode_: *mut inode, block: sector_t) -> *mut buffer_head;
    pub fn isofs_get_blocks(inode_: *mut inode, block: sector_t, bh: *mut *mut buffer_head, count: ::core::primitive::usize) -> i32;
    pub fn __isofs_iget(sb: *mut super_block, block: ::core::primitive::usize, offset: ::core::primitive::usize, relocated: i32) -> *mut inode;
}

#[inline]
pub unsafe fn isofs_iget(sb: *mut super_block, block: ::core::primitive::usize, offset: ::core::primitive::usize) -> *mut inode { __isofs_iget(sb, block, offset, 0) }
#[inline]
pub unsafe fn isofs_iget_reloc(sb: *mut super_block, block: ::core::primitive::usize, offset: ::core::primitive::usize) -> *mut inode { __isofs_iget(sb, block, offset, 1) }

#[inline]
pub fn isofs_get_ino(block: ::core::primitive::usize, offset: ::core::primitive::usize, bufbits: ::core::primitive::usize) -> ::core::primitive::usize {
    (block << (bufbits - 5)) | (offset >> 5)
}

#[inline]
pub unsafe fn isofs_normalize_block_and_offset(de: *mut iso_directory_record, block: *mut ::core::primitive::usize, offset: *mut ::core::primitive::usize) {
    if (*de).flags[0] & 2 != 0 {
        *offset = 0;
        *block = isonum_733((*de).extent) as ::core::primitive::usize + isonum_711((*de).ext_attr_length) as ::core::primitive::usize;
    }
}

extern "C" {
    pub fn isofs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32;
    pub static isofs_dir_inode_operations: inode_operations;
    pub static isofs_dir_operations: file_operations;
    pub static isofs_symlink_aops: address_space_operations;
    pub static isofs_export_ops: export_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
