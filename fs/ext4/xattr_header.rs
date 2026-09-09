// SPDX-License-Identifier: GPL-2.0
/*
  File: fs/ext4/xattr.h

  On-disk format of extended attributes for the ext4 filesystem.

  (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
*/

// Dependency: linux/xattr.h

/* Magic value in attribute blocks */
pub const EXT4_XATTR_MAGIC: u32 = 0xEA020000;
/* Maximum number of references to one attribute block */
pub const EXT4_XATTR_REFCOUNT_MAX: u32 = 1024;

/* Name indexes */
pub const EXT4_XATTR_INDEX_USER: u32 = 1;
pub const EXT4_XATTR_INDEX_POSIX_ACL_ACCESS: u32 = 2;
pub const EXT4_XATTR_INDEX_POSIX_ACL_DEFAULT: u32 = 3;
pub const EXT4_XATTR_INDEX_TRUSTED: u32 = 4;
pub const EXT4_XATTR_INDEX_LUSTRE: u32 = 5;
pub const EXT4_XATTR_INDEX_SECURITY: u32 = 6;
pub const EXT4_XATTR_INDEX_SYSTEM: u32 = 7;
pub const EXT4_XATTR_INDEX_RICHACL: u32 = 8;
pub const EXT4_XATTR_INDEX_ENCRYPTION: u32 = 9;
pub const EXT4_XATTR_INDEX_HURD: u32 = 10; // Reserved for Hurd

#[repr(C)]
pub struct ext4_xattr_header {
    pub h_magic: __le32,
    pub h_refcount: __le32,
    pub h_blocks: __le32,
    pub h_hash: __le32,
    pub h_checksum: __le32,
    pub h_reserved: [__u32; 3],
}

#[repr(C)]
pub struct ext4_xattr_ibody_header { pub h_magic: __le32 }

#[repr(C)]
pub struct ext4_xattr_entry {
    pub e_name_len: __u8,
    pub e_name_index: __u8,
    pub e_value_offs: __le16,
    pub e_value_inum: __le32,
    pub e_value_size: __le32,
    pub e_hash: __le32,
    pub e_name: [c_char; 0],
}

pub const EXT4_XATTR_PAD_BITS: u32 = 2;
pub const EXT4_XATTR_PAD: usize = 1 << EXT4_XATTR_PAD_BITS;
pub const EXT4_XATTR_ROUND: usize = EXT4_XATTR_PAD - 1;
#[inline]
pub unsafe fn EXT4_XATTR_LEN(name_len: usize) -> usize {
    (name_len + EXT4_XATTR_ROUND + core::mem::size_of::<ext4_xattr_entry>()) & !EXT4_XATTR_ROUND
}
#[inline]
pub unsafe fn EXT4_XATTR_NEXT(entry: *mut ext4_xattr_entry) -> *mut ext4_xattr_entry {
    (entry as *mut u8).add(EXT4_XATTR_LEN((*entry).e_name_len as usize)) as *mut ext4_xattr_entry
}
#[inline]
pub const fn EXT4_XATTR_SIZE(size: usize) -> usize { (size + EXT4_XATTR_ROUND) & !EXT4_XATTR_ROUND }

// IHDR, ITAIL, IFIRST, BHDR, ENTRY, BFIRST, and EXT4_INODE_HAS_XATTR_SPACE
// retain their C pointer/arithmetic semantics and depend on external ext4 definitions.
pub const EXT4_XATTR_SIZE_MAX: usize = 1 << 24;
#[inline]
pub const fn EXT4_XATTR_MIN_LARGE_EA_SIZE(b: usize) -> usize {
    b - EXT4_XATTR_LEN(3) - core::mem::size_of::<ext4_xattr_header>() - 4
}
pub const EXT4_XATTR_NAME_ENCRYPTION_CONTEXT: &[u8] = b"c\0";

#[inline]
pub unsafe fn ext4_write_lock_xattr(inode: *mut inode, save: *mut c_int) {
    down_write(&mut (*EXT4_I(inode)).xattr_sem);
    *save = ext4_test_inode_state(inode, EXT4_STATE_NO_EXPAND);
    ext4_set_inode_state(inode, EXT4_STATE_NO_EXPAND);
}
#[inline]
pub unsafe fn ext4_write_trylock_xattr(inode: *mut inode, save: *mut c_int) -> c_int {
    if down_write_trylock(&mut (*EXT4_I(inode)).xattr_sem) == 0 { return 0; }
    *save = ext4_test_inode_state(inode, EXT4_STATE_NO_EXPAND);
    ext4_set_inode_state(inode, EXT4_STATE_NO_EXPAND);
    1
}
#[inline]
pub unsafe fn ext4_write_unlock_xattr(inode: *mut inode, save: *mut c_int) {
    if *save == 0 { ext4_clear_inode_state(inode, EXT4_STATE_NO_EXPAND); }
    up_write(&mut (*EXT4_I(inode)).xattr_sem);
}
#[inline]
pub unsafe fn xattr_check_inode(inode: *mut inode, header: *mut ext4_xattr_ibody_header, end: *mut core::ffi::c_void) -> c_int {
    __xattr_check_inode(inode, header, end, b"xattr_check_inode\0".as_ptr() as *const c_char, 0)
}

#[repr(C)]
pub struct ext4_xattr_info {
    pub name: *const c_char,
    pub value: *const core::ffi::c_void,
    pub value_len: usize,
    pub name_index: c_int,
    pub in_inode: c_int,
}
#[repr(C)]
pub struct ext4_xattr_search {
    pub first: *mut ext4_xattr_entry,
    pub base: *mut core::ffi::c_void,
    pub end: *mut core::ffi::c_void,
    pub here: *mut ext4_xattr_entry,
    pub not_found: c_int,
}
#[repr(C)]
pub struct ext4_xattr_ibody_find {
    pub s: ext4_xattr_search,
    pub iloc: ext4_iloc,
}

extern "C" {
    pub static ext4_xattr_user_handler: xattr_handler;
    pub static ext4_xattr_trusted_handler: xattr_handler;
    pub static ext4_xattr_security_handler: xattr_handler;
    pub static ext4_xattr_hurd_handler: xattr_handler;

    pub fn ext4_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> ssize_t;
    pub fn ext4_xattr_get(inode: *mut inode, name_index: c_int, name: *const c_char, buffer: *mut core::ffi::c_void, buffer_size: usize) -> c_int;
    pub fn ext4_xattr_set(inode: *mut inode, name_index: c_int, name: *const c_char, value: *const core::ffi::c_void, size: usize, flags: c_int) -> c_int;
    pub fn ext4_xattr_set_handle(handle: *mut handle_t, inode: *mut inode, name_index: c_int, name: *const c_char, value: *const core::ffi::c_void, size: usize, flags: c_int) -> c_int;
    pub fn ext4_xattr_set_credits(inode: *mut inode, value_len: usize, is_create: bool, credits: *mut c_int) -> c_int;
    pub fn __ext4_xattr_set_credits(sb: *mut super_block, inode: *mut inode, block_bh: *mut buffer_head, value_len: usize, is_create: bool) -> c_int;
    pub fn ext4_xattr_delete_inode(handle: *mut handle_t, inode: *mut inode, extra_credits: c_int) -> c_int;
    pub fn ext4_init_ea_inode_work(sbi: *mut ext4_sb_info);
    pub fn ext4_put_ea_inode(inode: *mut inode);
    pub fn ext4_expand_extra_isize_ea(inode: *mut inode, new_extra_isize: c_int, raw_inode: *mut ext4_inode, handle: *mut handle_t) -> c_int;
    pub fn ext4_evict_ea_inode(inode: *mut inode);
    pub static ext4_xattr_handlers: *const *const xattr_handler;
    pub fn ext4_xattr_ibody_find(inode: *mut inode, i: *mut ext4_xattr_info, is: *mut ext4_xattr_ibody_find) -> c_int;
    pub fn ext4_xattr_ibody_get(inode: *mut inode, name_index: c_int, name: *const c_char, buffer: *mut core::ffi::c_void, buffer_size: usize) -> c_int;
    pub fn ext4_xattr_ibody_set(handle: *mut handle_t, inode: *mut inode, i: *mut ext4_xattr_info, is: *mut ext4_xattr_ibody_find) -> c_int;
    pub fn ext4_xattr_create_cache() -> *mut mb_cache;
    pub fn ext4_xattr_destroy_cache(cache: *mut mb_cache);
    pub fn __xattr_check_inode(inode: *mut inode, header: *mut ext4_xattr_ibody_header, end: *mut core::ffi::c_void, function: *const c_char, line: c_uint) -> c_int;
    pub fn ext4_init_security(handle: *mut handle_t, inode: *mut inode, dir: *mut inode, qstr: *const qstr) -> c_int;
    pub fn ext4_xattr_inode_set_class(ea_inode: *mut inode);
    pub fn ext4_get_inode_usage(inode: *mut inode, usage: *mut qsize_t) -> c_int;
}

// CONFIG_EXT4_FS_SECURITY and CONFIG_LOCKDEP conditionals are preserved by
// the external declarations above; configuration-specific inline fallbacks
// require the surrounding kernel definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
