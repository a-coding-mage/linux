/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs/f2fs/xattr.h
 *
 * Copyright (c) 2012 Samsung Electronics Co., Ltd.
 *             http://www.samsung.com/
 *
 * Portions of this code from linux/fs/ext2/xattr.h
 *
 * On-disk format of extended attributes for the ext2 filesystem.
 *
 * (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
 */

// Dependencies supplied by the surrounding translation unit.

pub const F2FS_XATTR_MAGIC: u32 = 0xF2F5_2011;
pub const F2FS_XATTR_REFCOUNT_MAX: u32 = 1024;

pub const F2FS_SYSTEM_ADVISE_NAME: &[u8] = b"system.advise\0";
pub const F2FS_XATTR_INDEX_USER: u32 = 1;
pub const F2FS_XATTR_INDEX_POSIX_ACL_ACCESS: u32 = 2;
pub const F2FS_XATTR_INDEX_POSIX_ACL_DEFAULT: u32 = 3;
pub const F2FS_XATTR_INDEX_TRUSTED: u32 = 4;
pub const F2FS_XATTR_INDEX_LUSTRE: u32 = 5;
pub const F2FS_XATTR_INDEX_SECURITY: u32 = 6;
pub const F2FS_XATTR_INDEX_ADVISE: u32 = 7;
/* Should be same as EXT4_XATTR_INDEX_ENCRYPTION */
pub const F2FS_XATTR_INDEX_ENCRYPTION: u32 = 9;
pub const F2FS_XATTR_INDEX_VERITY: u32 = 11;

pub const F2FS_XATTR_NAME_ENCRYPTION_CONTEXT: &[u8] = b"c\0";
pub const F2FS_XATTR_NAME_VERITY: &[u8] = b"v\0";

#[repr(C)]
pub struct f2fs_xattr_header {
    pub h_magic: __le32,       /* magic number for identification */
    pub h_refcount: __le32,    /* reference count */
    pub h_reserved: [__u32; 4], /* zero right now */
}

#[repr(C)]
pub struct f2fs_xattr_entry {
    pub e_name_index: __u8,
    pub e_name_len: __u8,
    pub e_value_size: __le16,  /* size of attribute value */
    pub e_name: [c_char; 0],   /* attribute name */
}

pub const XATTR_ROUND: usize = 3;

#[inline]
pub unsafe fn XATTR_HDR(ptr: *mut c_void) -> *mut f2fs_xattr_header {
    ptr as *mut f2fs_xattr_header
}

#[inline]
pub unsafe fn XATTR_ENTRY(ptr: *mut c_void) -> *mut f2fs_xattr_entry {
    ptr as *mut f2fs_xattr_entry
}

#[inline]
pub unsafe fn XATTR_FIRST_ENTRY(ptr: *mut c_void) -> *mut f2fs_xattr_entry {
    XATTR_ENTRY(XATTR_HDR(ptr).add(1) as *mut c_void)
}

#[inline]
pub const fn XATTR_ALIGN(size: usize) -> usize {
    (size + XATTR_ROUND) & !XATTR_ROUND
}

#[inline]
pub unsafe fn ENTRY_SIZE(entry: *const f2fs_xattr_entry) -> usize {
    XATTR_ALIGN(core::mem::size_of::<f2fs_xattr_entry>()
        + (*entry).e_name_len as usize
        + le16_to_cpu((*entry).e_value_size) as usize)
}

#[inline]
pub unsafe fn XATTR_NEXT_ENTRY(entry: *mut f2fs_xattr_entry) -> *mut f2fs_xattr_entry {
    (entry as *mut c_char).add(ENTRY_SIZE(entry)) as *mut f2fs_xattr_entry
}

#[inline]
pub unsafe fn IS_XATTR_LAST_ENTRY(entry: *const f2fs_xattr_entry) -> bool {
    *(entry as *const __u32) == 0
}

pub const XATTR_PADDING_SIZE: usize = core::mem::size_of::<__u32>();

#[inline]
pub unsafe fn VALID_XATTR_BLOCK_SIZE() -> usize {
    PAGE_SIZE - core::mem::size_of::<node_footer>()
}

#[inline]
pub unsafe fn XATTR_SIZE(i: *mut f2fs_sb_info) -> usize {
    (if (*F2FS_I(i)).i_xattr_nid != 0 { VALID_XATTR_BLOCK_SIZE() } else { 0 })
        + inline_xattr_size(i)
}

#[inline]
pub unsafe fn MIN_OFFSET(i: *mut f2fs_sb_info) -> usize {
    XATTR_ALIGN(inline_xattr_size(i) + VALID_XATTR_BLOCK_SIZE())
}

#[inline]
pub unsafe fn MAX_VALUE_LEN(i: *mut f2fs_sb_info) -> usize {
    MIN_OFFSET(i) - core::mem::size_of::<f2fs_xattr_header>()
        - core::mem::size_of::<f2fs_xattr_entry>()
}

pub const MIN_INLINE_XATTR_SIZE: usize =
    core::mem::size_of::<f2fs_xattr_header>() / core::mem::size_of::<__le32>();
pub const MAX_INLINE_XATTR_SIZE: usize = DEF_ADDRS_PER_INODE
    - F2FS_TOTAL_EXTRA_ATTR_SIZE / core::mem::size_of::<__le32>()
    - DEF_INLINE_RESERVED_SIZE
    - MIN_INLINE_DENTRY_SIZE / core::mem::size_of::<__le32>();
pub const DEFAULT_XATTR_SLAB_SIZE: usize = DEFAULT_INLINE_XATTR_ADDRS
    * core::mem::size_of::<__le32>() + XATTR_PADDING_SIZE;

/*
 * On-disk structure of f2fs_xattr
 * We use inline xattrs space + 1 block for xattr.
 */

#[cfg(CONFIG_F2FS_FS_XATTR)]
extern "C" {
    pub static f2fs_xattr_user_handler: xattr_handler;
    pub static f2fs_xattr_trusted_handler: xattr_handler;
    pub static f2fs_xattr_advise_handler: xattr_handler;
    pub static f2fs_xattr_security_handler: xattr_handler;
    pub static f2fs_xattr_handlers: *const *const xattr_handler;

    pub fn f2fs_setxattr(inode: *mut inode, index: c_int, name: *const c_char,
        value: *const c_void, size: size_t, folio: *mut folio, flags: c_int) -> c_int;
    pub fn f2fs_getxattr(inode: *mut inode, index: c_int, name: *const c_char,
        buffer: *mut c_void, size: size_t, folio: *mut folio) -> c_int;
    pub fn f2fs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: size_t) -> ssize_t;
    pub fn f2fs_init_xattr_cache() -> c_int;
    pub fn f2fs_destroy_xattr_cache();
}

#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub const f2fs_xattr_handlers: *const xattr_handler = core::ptr::null();

#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub unsafe fn f2fs_setxattr(_: *mut inode, _: c_int, _: *const c_char,
    _: *const c_void, _: size_t, _: *mut folio, _: c_int) -> c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub unsafe fn f2fs_getxattr(_: *mut inode, _: c_int, _: *const c_char,
    _: *mut c_void, _: size_t, _: *mut folio) -> c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub const f2fs_listxattr: Option<unsafe extern "C" fn(*mut dentry, *mut c_char, size_t) -> ssize_t> = None;
#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub unsafe fn f2fs_init_xattr_cache() -> c_int { 0 }
#[cfg(not(CONFIG_F2FS_FS_XATTR))]
pub unsafe fn f2fs_destroy_xattr_cache() {}

#[cfg(CONFIG_F2FS_FS_SECURITY)]
extern "C" { pub fn f2fs_init_security(inode: *mut inode, dir: *mut inode,
    qstr: *const qstr, ifolio: *mut folio) -> c_int; }
#[cfg(not(CONFIG_F2FS_FS_SECURITY))]
pub unsafe fn f2fs_init_security(_: *mut inode, _: *mut inode,
    _: *const qstr, _: *mut folio) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
