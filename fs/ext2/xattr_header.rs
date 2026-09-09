/* SPDX-License-Identifier: GPL-2.0 */
/*
  File: linux/ext2_xattr.h

  On-disk format of extended attributes for the ext2 filesystem.

  (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
*/

// The original header includes linux/init.h and linux/xattr.h.

/* Magic value in attribute blocks */
pub const EXT2_XATTR_MAGIC: u32 = 0xEA020000;

/* Maximum number of references to one attribute block */
pub const EXT2_XATTR_REFCOUNT_MAX: u32 = 1024;

/* Name indexes */
pub const EXT2_XATTR_INDEX_USER: u32 = 1;
pub const EXT2_XATTR_INDEX_POSIX_ACL_ACCESS: u32 = 2;
pub const EXT2_XATTR_INDEX_POSIX_ACL_DEFAULT: u32 = 3;
pub const EXT2_XATTR_INDEX_TRUSTED: u32 = 4;
pub const EXT2_XATTR_INDEX_LUSTRE: u32 = 5;
pub const EXT2_XATTR_INDEX_SECURITY: u32 = 6;

#[repr(C)]
pub struct ext2_xattr_header {
    pub h_magic: u32,       /* magic number for identification */
    pub h_refcount: u32,    /* reference count */
    pub h_blocks: u32,      /* number of disk blocks used */
    pub h_hash: u32,        /* hash value of all attributes */
    pub h_reserved: [u32; 4], /* zero right now */
}

#[repr(C)]
pub struct ext2_xattr_entry {
    pub e_name_len: u8,     /* length of name */
    pub e_name_index: u8,   /* attribute name index */
    pub e_value_offs: u16,  /* offset in disk block of value */
    pub e_value_block: u32, /* disk block attribute is stored on (n/i) */
    pub e_value_size: u32,  /* size of attribute value */
    pub e_hash: u32,        /* hash value of name and value */
    pub e_name: [core::ffi::c_char; 0], /* attribute name */
}

pub const EXT2_XATTR_PAD_BITS: usize = 2;
pub const EXT2_XATTR_PAD: usize = 1 << EXT2_XATTR_PAD_BITS;
pub const EXT2_XATTR_ROUND: usize = EXT2_XATTR_PAD - 1;

#[inline]
pub const fn EXT2_XATTR_LEN(name_len: usize) -> usize {
    (name_len + EXT2_XATTR_ROUND + core::mem::size_of::<ext2_xattr_entry>())
        & !EXT2_XATTR_ROUND
}

#[inline]
pub unsafe fn EXT2_XATTR_NEXT(entry: *mut ext2_xattr_entry) -> *mut ext2_xattr_entry {
    (entry as *mut u8).add(EXT2_XATTR_LEN((*entry).e_name_len as usize))
        as *mut ext2_xattr_entry
}

#[inline]
pub const fn EXT2_XATTR_SIZE(size: usize) -> usize {
    (size + EXT2_XATTR_ROUND) & !EXT2_XATTR_ROUND
}

pub struct mb_cache;

// CONFIG_EXT2_FS_XATTR controls whether the following external declarations
// or the inline no-support implementations are selected.
#[cfg(feature = "CONFIG_EXT2_FS_XATTR")]
extern "C" {
    pub static ext2_xattr_user_handler: xattr_handler;
    pub static ext2_xattr_trusted_handler: xattr_handler;
    pub static ext2_xattr_security_handler: xattr_handler;
    pub fn ext2_listxattr(dentry: *mut dentry, buffer: *mut core::ffi::c_char, size: usize) -> isize;
    pub fn ext2_xattr_get(inode: *mut inode, name_index: i32, name: *const core::ffi::c_char, buffer: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn ext2_xattr_set(inode: *mut inode, name_index: i32, name: *const core::ffi::c_char, value: *const core::ffi::c_void, size: usize, flags: i32) -> i32;
    pub fn ext2_xattr_delete_inode(inode: *mut inode);
    pub fn ext2_xattr_create_cache() -> *mut mb_cache;
    pub fn ext2_xattr_destroy_cache(cache: *mut mb_cache);
    pub static ext2_xattr_handlers: *const *const xattr_handler;
}

#[cfg(not(feature = "CONFIG_EXT2_FS_XATTR"))]
pub unsafe fn ext2_xattr_get(_inode: *mut inode, _name_index: i32, _name: *const core::ffi::c_char, _buffer: *mut core::ffi::c_void, _size: usize) -> i32 { -libc::EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_EXT2_FS_XATTR"))]
pub unsafe fn ext2_xattr_set(_inode: *mut inode, _name_index: i32, _name: *const core::ffi::c_char, _value: *const core::ffi::c_void, _size: usize, _flags: i32) -> i32 { -libc::EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_EXT2_FS_XATTR"))]
pub unsafe fn ext2_xattr_delete_inode(_inode: *mut inode) {}

#[cfg(not(feature = "CONFIG_EXT2_FS_XATTR"))]
pub const ext2_xattr_handlers: *const *const xattr_handler = core::ptr::null();

#[cfg(not(feature = "CONFIG_EXT2_FS_XATTR"))]
pub const ext2_listxattr: Option<unsafe extern "C" fn()> = None;

#[cfg(feature = "CONFIG_EXT2_FS_SECURITY")]
extern "C" {
    pub fn ext2_init_security(inode: *mut inode, dir: *mut inode, qstr: *const qstr) -> i32;
}

#[cfg(not(feature = "CONFIG_EXT2_FS_SECURITY"))]
pub unsafe fn ext2_init_security(_inode: *mut inode, _dir: *mut inode, _qstr: *const qstr) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
