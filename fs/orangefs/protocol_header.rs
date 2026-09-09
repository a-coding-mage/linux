/* SPDX-License-Identifier: GPL-2.0 */
// Linux kernel dependencies corresponding to the original includes are supplied externally.

/* khandle stuff  ***********************************************************/

/*
 * The 2.9 core will put 64 bit handles in here like this:
 *    1234 0000 0000 5678
 * The 3.0 and beyond cores will put 128 bit handles in here like this:
 *    1234 5678 90AB CDEF
 * The kernel module will always use the first four bytes and
 * the last four bytes as an inum.
 */
#[repr(C, align(8))]
pub struct orangefs_khandle {
    pub u: [u8; 16],
}

/* kernel version of an object ref. */
#[repr(C)]
pub struct orangefs_object_kref {
    pub khandle: orangefs_khandle,
    pub fs_id: i32,
    pub __pad1: i32,
}

/* compare 2 khandles assumes little endian thus from large address to small address */
#[inline]
pub unsafe fn ORANGEFS_khandle_cmp(kh1: *const orangefs_khandle, kh2: *const orangefs_khandle) -> i32 {
    let mut i: i32 = 15;
    while i >= 0 {
        if (*kh1).u[i as usize] > (*kh2).u[i as usize] { return 1; }
        if (*kh1).u[i as usize] < (*kh2).u[i as usize] { return -1; }
        i -= 1;
    }
    0
}

#[inline]
pub unsafe fn ORANGEFS_khandle_to(kh: *const orangefs_khandle, p: *mut core::ffi::c_void, size: i32) {
    core::ptr::copy_nonoverlapping((*kh).u.as_ptr(), p as *mut u8, 16);
    core::ptr::write_bytes((p as *mut u8).add(16), 0, (size - 16) as usize);
}

#[inline]
pub unsafe fn ORANGEFS_khandle_from(kh: *mut orangefs_khandle, p: *mut core::ffi::c_void, _size: i32) {
    core::ptr::write_bytes(kh, 0, 1);
    core::ptr::copy_nonoverlapping(p as *const u8, (*kh).u.as_mut_ptr(), 16);
}

/* pvfs2-types.h ************************************************************/
pub const ORANGEFS_SUPER_MAGIC: u32 = 0x20030528;
pub const ORANGEFS_ERROR_BIT: i32 = 1 << 30;
pub const ORANGEFS_NON_ERRNO_ERROR_BIT: i32 = 1 << 29;
pub const ORANGEFS_ERROR_CLASS_BITS: i32 = 0x380;
pub const ORANGEFS_ERROR_NUMBER_BITS: i32 = 0x7f;
pub const ORANGEFS_ECANCEL: i32 = 1 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_EDEVINIT: i32 = 2 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_EDETAIL: i32 = 3 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_EHOSTNTFD: i32 = 4 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_EADDRNTFD: i32 = 5 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_ENORECVR: i32 = 6 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_ETRYAGAIN: i32 = 7 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_ENOTPVFS: i32 = 8 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;
pub const ORANGEFS_ESECURITY: i32 = 9 | ORANGEFS_NON_ERRNO_ERROR_BIT | ORANGEFS_ERROR_BIT;

pub const ORANGEFS_O_EXECUTE: i32 = 1 << 0; pub const ORANGEFS_O_WRITE: i32 = 1 << 1; pub const ORANGEFS_O_READ: i32 = 1 << 2;
pub const ORANGEFS_G_EXECUTE: i32 = 1 << 3; pub const ORANGEFS_G_WRITE: i32 = 1 << 4; pub const ORANGEFS_G_READ: i32 = 1 << 5;
pub const ORANGEFS_U_EXECUTE: i32 = 1 << 6; pub const ORANGEFS_U_WRITE: i32 = 1 << 7; pub const ORANGEFS_U_READ: i32 = 1 << 8;
pub const ORANGEFS_G_SGID: i32 = 1 << 10; pub const ORANGEFS_U_SUID: i32 = 1 << 11;
pub const ORANGEFS_ITERATE_START: i32 = 2147483646; pub const ORANGEFS_ITERATE_END: i32 = 2147483645;
// FS_IMMUTABLE_FL, FS_APPEND_FL, and FS_NOATIME_FL are supplied by Linux dependencies.
pub const ORANGEFS_MIRROR_FL: u64 = 0x01000000;
pub const ORANGEFS_FS_ID_NULL: i32 = 0;
pub const ORANGEFS_ATTR_SYS_UID: i32 = 1 << 0; pub const ORANGEFS_ATTR_SYS_GID: i32 = 1 << 1; pub const ORANGEFS_ATTR_SYS_PERM: i32 = 1 << 2;
pub const ORANGEFS_ATTR_SYS_ATIME: i32 = 1 << 3; pub const ORANGEFS_ATTR_SYS_CTIME: i32 = 1 << 4; pub const ORANGEFS_ATTR_SYS_MTIME: i32 = 1 << 5;
pub const ORANGEFS_ATTR_SYS_TYPE: i32 = 1 << 6; pub const ORANGEFS_ATTR_SYS_ATIME_SET: i32 = 1 << 7; pub const ORANGEFS_ATTR_SYS_MTIME_SET: i32 = 1 << 8;
pub const ORANGEFS_ATTR_SYS_SIZE: i32 = 1 << 20; pub const ORANGEFS_ATTR_SYS_LNK_TARGET: i32 = 1 << 24; pub const ORANGEFS_ATTR_SYS_DFILE_COUNT: i32 = 1 << 25;
pub const ORANGEFS_ATTR_SYS_DIRENT_COUNT: i32 = 1 << 26; pub const ORANGEFS_ATTR_SYS_BLKSIZE: i32 = 1 << 28; pub const ORANGEFS_ATTR_SYS_MIRROR_COPIES_COUNT: i32 = 1 << 29;
pub const ORANGEFS_ATTR_SYS_COMMON_ALL: i32 = ORANGEFS_ATTR_SYS_UID | ORANGEFS_ATTR_SYS_GID | ORANGEFS_ATTR_SYS_PERM | ORANGEFS_ATTR_SYS_ATIME | ORANGEFS_ATTR_SYS_CTIME | ORANGEFS_ATTR_SYS_MTIME | ORANGEFS_ATTR_SYS_TYPE;
pub const ORANGEFS_ATTR_SYS_ALL_SETABLE: i32 = ORANGEFS_ATTR_SYS_COMMON_ALL - ORANGEFS_ATTR_SYS_TYPE;
pub const ORANGEFS_ATTR_SYS_ALL_NOHINT: i32 = ORANGEFS_ATTR_SYS_COMMON_ALL | ORANGEFS_ATTR_SYS_SIZE | ORANGEFS_ATTR_SYS_LNK_TARGET | ORANGEFS_ATTR_SYS_DFILE_COUNT | ORANGEFS_ATTR_SYS_MIRROR_COPIES_COUNT | ORANGEFS_ATTR_SYS_DIRENT_COUNT | ORANGEFS_ATTR_SYS_BLKSIZE;
pub const ORANGEFS_XATTR_REPLACE: i32 = 0x2; pub const ORANGEFS_XATTR_CREATE: i32 = 0x1;
pub const ORANGEFS_MAX_SERVER_ADDR_LEN: usize = 256; pub const ORANGEFS_NAME_MAX: usize = 256;
pub const ORANGEFS_MAX_XATTR_NAMELEN: usize = ORANGEFS_NAME_MAX;
pub const ORANGEFS_MAX_XATTR_VALUELEN: usize = 8192; pub const ORANGEFS_MAX_XATTR_LISTLEN: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ORANGEFS_io_type { ORANGEFS_IO_READ = 1, ORANGEFS_IO_WRITE = 2 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum orangefs_ds_type { ORANGEFS_TYPE_NONE = 0, ORANGEFS_TYPE_METAFILE = 1 << 0, ORANGEFS_TYPE_DATAFILE = 1 << 1, ORANGEFS_TYPE_DIRECTORY = 1 << 2, ORANGEFS_TYPE_SYMLINK = 1 << 3, ORANGEFS_TYPE_DIRDATA = 1 << 4, ORANGEFS_TYPE_INTERNAL = 1 << 5 }

#[repr(C)]
pub struct ORANGEFS_keyval_pair { pub key: [core::ffi::c_char; ORANGEFS_MAX_XATTR_NAMELEN], pub key_sz: i32, pub val_sz: i32, pub val: [core::ffi::c_char; ORANGEFS_MAX_XATTR_VALUELEN] }

#[repr(C)]
pub struct ORANGEFS_sys_attr_s {
    pub owner: u32, pub group: u32, pub perms: u32, pub atime: u64, pub mtime: u64, pub ctime: u64, pub size: i64,
    pub link_target: *mut core::ffi::c_char, pub dfile_count: i32, pub distr_dir_servers_initial: i32, pub distr_dir_servers_max: i32,
    pub distr_dir_split_size: i32, pub mirror_copies_count: u32, pub dist_name: *mut core::ffi::c_char, pub dist_params: *mut core::ffi::c_char,
    pub dirent_count: i64, pub objtype: orangefs_ds_type, pub flags: u64, pub mask: u32, pub blksize: i64,
}

pub const ORANGEFS_LOOKUP_LINK_NO_FOLLOW: i32 = 0;

#[repr(C)]
pub struct dev_mask_info_s { pub mask_type: dev_mask_info_s_mask_type, pub mask_value: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dev_mask_info_s_mask_type { KERNEL_MASK, CLIENT_MASK }
#[repr(C)]
pub struct dev_mask2_info_s { pub mask1_value: u64, pub mask2_value: u64 }

unsafe extern "C" { pub fn ORANGEFS_util_translate_mode(mode: i32) -> i32; }

/* pvfs2-debug.h: orangefs-debug.h is an external dependency. */
// #define llu(x) (unsigned long long)(x)
// #define lld(x) (long long)(x)
pub const ORANGEFS_DEV_MAGIC: u8 = b'k';
pub const ORANGEFS_READDIR_DEFAULT_DESC_COUNT: i32 = 5;
pub const DEV_GET_MAGIC: u32 = 0x1; pub const DEV_GET_MAX_UPSIZE: u32 = 0x2; pub const DEV_GET_MAX_DOWNSIZE: u32 = 0x3; pub const DEV_MAP: u32 = 0x4; pub const DEV_REMOUNT_ALL: u32 = 0x5; pub const DEV_DEBUG: u32 = 0x6; pub const DEV_UPSTREAM: u32 = 0x7; pub const DEV_CLIENT_MASK: u32 = 0x8; pub const DEV_CLIENT_STRING: u32 = 0x9; pub const DEV_MAX_NR: u32 = 0xa;
// Linux _IO, _IOW, and _IOR are external ioctl encoders; these declarations preserve their source expressions.
pub const ORANGEFS_DEV_GET_MAGIC: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_GET_MAGIC, i32);
pub const ORANGEFS_DEV_GET_MAX_UPSIZE: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_GET_MAX_UPSIZE, i32);
pub const ORANGEFS_DEV_GET_MAX_DOWNSIZE: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_GET_MAX_DOWNSIZE, i32);
pub const ORANGEFS_DEV_MAP: u32 = _IO(ORANGEFS_DEV_MAGIC, DEV_MAP);
pub const ORANGEFS_DEV_REMOUNT_ALL: u32 = _IO(ORANGEFS_DEV_MAGIC, DEV_REMOUNT_ALL);
pub const ORANGEFS_DEV_DEBUG: u32 = _IOR(ORANGEFS_DEV_MAGIC, DEV_DEBUG, i32);
pub const ORANGEFS_DEV_UPSTREAM: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_UPSTREAM, i32);
pub const ORANGEFS_DEV_CLIENT_MASK: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_CLIENT_MASK, dev_mask2_info_s);
pub const ORANGEFS_DEV_CLIENT_STRING: u32 = _IOW(ORANGEFS_DEV_MAGIC, DEV_CLIENT_STRING, *mut core::ffi::c_char);
pub const ORANGEFS_DEV_MAXNR: u32 = DEV_MAX_NR;
pub const ORANGEFS_KERNEL_PROTO_VERSION: i32 = 0; pub const ORANGEFS_MINIMUM_USERSPACE_VERSION: i32 = 20903;

#[repr(C)]
pub struct ORANGEFS_dev_map_desc { pub ptr: *mut core::ffi::c_void, pub total_size: i32, pub size: i32, pub count: i32 }
pub static mut orangefs_gossip_debug_mask: u64 = 0;
// gossip_debug and gossip_err depend on kernel printk/pr_err and variadic macro syntax.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
