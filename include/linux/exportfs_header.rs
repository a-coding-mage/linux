/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external types and declarations.

pub const MAX_HANDLE_SZ: usize = 128;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum fid_type {
    FILEID_ROOT = 0,
    FILEID_INO32_GEN = 1,
    FILEID_INO32_GEN_PARENT = 2,
    FILEID_BTRFS_WITHOUT_PARENT = 0x4d,
    FILEID_BTRFS_WITH_PARENT = 0x4e,
    FILEID_BTRFS_WITH_PARENT_ROOT = 0x4f,
    FILEID_UDF_WITHOUT_PARENT = 0x51,
    FILEID_UDF_WITH_PARENT = 0x52,
    FILEID_NILFS_WITHOUT_PARENT = 0x61,
    FILEID_NILFS_WITH_PARENT = 0x62,
    FILEID_FAT_WITHOUT_PARENT = 0x71,
    FILEID_FAT_WITH_PARENT = 0x72,
    FILEID_INO64_GEN = 0x81,
    FILEID_INO64_GEN_PARENT = 0x82,
    FILEID_LUSTRE = 0x97,
    FILEID_BCACHEFS_WITHOUT_PARENT = 0xb1,
    FILEID_BCACHEFS_WITH_PARENT = 0xb2,
    FILEID_NSFS = 0xf1,
    FILEID_KERNFS = 0xfe,
    FILEID_INVALID = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fid {
    pub i32: fid_i32,
    pub i64: fid_i64,
    pub udf: fid_udf,
    pub raw: [u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fid_i32 {
    pub ino: u32,
    pub gen: u32,
    pub parent_ino: u32,
    pub parent_gen: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fid_i64 {
    pub ino: u64,
    pub gen: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fid_udf {
    pub block: u32,
    pub partref: u16,
    pub parent_partref: u16,
    pub generation: u32,
    pub parent_block: u32,
    pub parent_generation: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum handle_to_path_flags {
    HANDLE_CHECK_PERMS = 1 << 0,
    HANDLE_CHECK_SUBTREE = 1 << 1,
}

#[repr(C)]
pub struct handle_to_path_ctx {
    pub root: path,
    pub flags: handle_to_path_flags,
    pub fh_flags: u32,
}

pub const EXPORT_FH_CONNECTABLE: i32 = 0x1;
pub const EXPORT_FH_FID: i32 = 0x2;
pub const EXPORT_FH_DIR_ONLY: i32 = 0x4;
pub const FILEID_USER_FLAGS_MASK: u32 = 0xffff0000;
pub const FILEID_IS_CONNECTABLE: u32 = 0x10000;
pub const FILEID_IS_DIR: u32 = 0x20000;
pub const FILEID_VALID_USER_FLAGS: u32 = FILEID_IS_CONNECTABLE | FILEID_IS_DIR;

#[inline]
pub const fn FILEID_USER_FLAGS(type_: u32) -> u32 {
    type_ & FILEID_USER_FLAGS_MASK
}

#[repr(C)]
pub struct export_operations {
    pub encode_fh: Option<unsafe extern "C" fn(*mut inode, *mut u32, *mut i32, *mut inode) -> i32>,
    pub fh_to_dentry: Option<unsafe extern "C" fn(*mut super_block, *mut fid, i32, i32) -> *mut dentry>,
    pub fh_to_parent: Option<unsafe extern "C" fn(*mut super_block, *mut fid, i32, i32) -> *mut dentry>,
    pub get_name: Option<unsafe extern "C" fn(*mut dentry, *mut i8, *mut dentry) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut dentry) -> *mut dentry>,
    pub commit_metadata: Option<unsafe extern "C" fn(*mut inode) -> i32>,
    pub permission: Option<unsafe extern "C" fn(*mut handle_to_path_ctx, u32) -> i32>,
    pub open: Option<unsafe extern "C" fn(*const path, u32) -> *mut file>,
    pub flags: libc::c_ulong,
    pub block_ops: *const exportfs_block_ops,
}

pub const EXPORT_OP_NOWCC: libc::c_ulong = 0x1;
pub const EXPORT_OP_NOSUBTREECHK: libc::c_ulong = 0x2;
pub const EXPORT_OP_CLOSE_BEFORE_UNLINK: libc::c_ulong = 0x4;
pub const EXPORT_OP_REMOTE_FS: libc::c_ulong = 0x8;
pub const EXPORT_OP_NOATOMIC_ATTR: libc::c_ulong = 0x10;
pub const EXPORT_OP_FLUSH_ON_CLOSE: libc::c_ulong = 0x20;
pub const EXPORT_OP_NOLOCKS: libc::c_ulong = 0x40;

#[inline]
pub unsafe fn exportfs_cannot_lock(export_ops: *const export_operations) -> bool {
    (*export_ops).flags & EXPORT_OP_NOLOCKS != 0
}

extern "C" {
    pub fn exportfs_encode_inode_fh(inode: *mut inode, fid: *mut fid, max_len: *mut i32,
                                    parent: *mut inode, flags: i32) -> i32;
    pub fn exportfs_encode_fh(dentry: *mut dentry, fid: *mut fid, max_len: *mut i32,
                              flags: i32) -> i32;
    pub fn exportfs_decode_fh_raw(mnt: *mut vfsmount, fid: *mut fid, fh_len: i32,
                                  fileid_type: i32, flags: u32,
                                  acceptable: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut dentry) -> i32>,
                                  context: *mut core::ffi::c_void) -> *mut dentry;
    pub fn exportfs_decode_fh(mnt: *mut vfsmount, fid: *mut fid, fh_len: i32,
                              fileid_type: i32,
                              acceptable: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut dentry) -> i32>,
                              context: *mut core::ffi::c_void) -> *mut dentry;
    pub fn generic_encode_ino32_fh(inode: *mut inode, fh: *mut u32, max_len: *mut i32,
                                   parent: *mut inode) -> i32;
    pub fn generic_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32,
                                fh_type: i32,
                                get_inode: Option<unsafe extern "C" fn(*mut super_block, u64, u32) -> *mut inode>) -> *mut dentry;
    pub fn generic_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: i32,
                                fh_type: i32,
                                get_inode: Option<unsafe extern "C" fn(*mut super_block, u64, u32) -> *mut inode>) -> *mut dentry;
}

#[inline]
pub unsafe fn exportfs_can_encode_fid(nop: *const export_operations) -> bool {
    nop.is_null() || (*nop).encode_fh.is_some()
}

#[inline]
pub unsafe fn exportfs_can_decode_fh(nop: *const export_operations) -> bool {
    !nop.is_null() && (*nop).fh_to_dentry.is_some()
}

#[inline]
pub unsafe fn exportfs_may_export(nop: *const export_operations) -> bool {
    exportfs_can_decode_fh(nop) && (*nop).open.is_none() && (*nop).permission.is_none()
}

#[inline]
pub unsafe fn exportfs_can_encode_fh(nop: *const export_operations, fh_flags: i32) -> bool {
    if fh_flags & EXPORT_FH_FID != 0 { return exportfs_can_encode_fid(nop); }
    if nop.is_null() { return false; }
    if fh_flags & EXPORT_FH_CONNECTABLE != 0 && (*nop).fh_to_parent.is_none() { return false; }
    exportfs_can_decode_fh(nop)
}

#[inline]
pub unsafe fn exportfs_encode_fid(inode: *mut inode, fid: *mut fid, max_len: *mut i32) -> i32 {
    exportfs_encode_inode_fh(inode, fid, max_len, core::ptr::null_mut(), EXPORT_FH_FID)
}

// Opaque types supplied by dependent translations.
pub enum dentry {}
pub enum exportfs_block_ops {}
pub enum inode {}
pub enum super_block {}
pub enum vfsmount {}
pub enum file {}
pub struct path { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
