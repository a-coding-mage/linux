/* SPDX-License-Identifier: GPL-2.0 */

/* pr_fmt(fmt) KBUILD_MODNAME ": " fmt */

pub const UFS_MAX_GROUP_LOADED: usize = 8;
pub const UFS_CGNO_EMPTY: u32 = u32::MAX;

pub struct ufs_sb_private_info;
pub struct ufs_cg_private_info;
pub struct ufs_csum;

#[repr(C)]
pub struct ufs_sb_info {
    pub s_uspi: *mut ufs_sb_private_info,
    pub s_csp: *mut ufs_csum,
    pub s_bytesex: u32,
    pub s_flags: u32,
    pub s_ucg: *mut *mut buffer_head,
    pub s_ucpi: [*mut ufs_cg_private_info; UFS_MAX_GROUP_LOADED],
    pub s_cgno: [u32; UFS_MAX_GROUP_LOADED],
    pub s_cg_loaded: u16,
    pub s_flavour: u32,
    pub s_on_err: u32,
    pub sb: *mut super_block,
    pub work_queued: i32, /* non-zero if the delayed work is queued */
    pub sync_work: delayed_work, /* FS sync delayed work */
    pub work_lock: spinlock_t, /* protects sync_work and work_queued */
    pub s_lock: mutex,
}

#[repr(C)]
pub union ufs_inode_info_i_u1 {
    pub i_data: [__fs32; 15],
    pub i_symlink: [__u8; 2 * 4 * 15],
    pub u2_i_data: [__fs64; 15],
}

#[repr(C)]
pub struct ufs_inode_info {
    pub i_u1: ufs_inode_info_i_u1,
    pub i_flags: __u32,
    pub i_shadow: __u32,
    pub i_unused1: __u32,
    pub i_unused2: __u32,
    pub i_oeftflag: __u32,
    pub i_osync: __u16,
    pub i_lastfrag: __u64,
    pub meta_lock: seqlock_t,
    pub truncate_mutex: mutex,
    pub i_dir_start_lookup: __u32,
    pub vfs_inode: inode,
}

pub const UFS_MOUNT_ONERROR_PANIC: u32 = 0x00000001;
pub const UFS_MOUNT_ONERROR_LOCK: u32 = 0x00000002;
pub const UFS_MOUNT_ONERROR_UMOUNT: u32 = 0x00000004;
pub const UFS_MOUNT_ONERROR_REPAIR: u32 = 0x00000008;
pub const UFS_MOUNT_UFSTYPE_OLD: u32 = 0x00000010;
pub const UFS_MOUNT_UFSTYPE_44BSD: u32 = 0x00000020;
pub const UFS_MOUNT_UFSTYPE_SUN: u32 = 0x00000040;
pub const UFS_MOUNT_UFSTYPE_NEXTSTEP: u32 = 0x00000080;
pub const UFS_MOUNT_UFSTYPE_NEXTSTEP_CD: u32 = 0x00000100;
pub const UFS_MOUNT_UFSTYPE_OPENSTEP: u32 = 0x00000200;
pub const UFS_MOUNT_UFSTYPE_SUNx86: u32 = 0x00000400;
pub const UFS_MOUNT_UFSTYPE_HP: u32 = 0x00000800;
pub const UFS_MOUNT_UFSTYPE_UFS2: u32 = 0x00001000;
pub const UFS_MOUNT_UFSTYPE_SUNOS: u32 = 0x00002000;

/* Debug code: CONFIG_UFS_DEBUG controls the original UFSD variadic macro. */

unsafe extern "C" {
    pub fn ufs_free_fragments(arg1: *mut inode, fragment: u64, count: u32);
    pub fn ufs_free_blocks(arg1: *mut inode, fragment: u64, count: u32);
    pub fn ufs_new_fragments(arg1: *mut inode, arg2: *mut core::ffi::c_void,
        fragment: u64, goal: u64, count: u32, err: *mut i32, folio: *mut folio) -> u64;
    pub fn ufs_load_cylinder(sb: *mut super_block, index: u32) -> *mut ufs_cg_private_info;
    pub fn ufs_put_cylinder(sb: *mut super_block, index: u32);
    pub static ufs_dir_inode_operations: inode_operations;
    pub fn ufs_add_link(dentry: *mut dentry, inode: *mut inode) -> i32;
    pub fn ufs_inode_by_name(inode: *mut inode, qstr: *const qstr) -> ino_t;
    pub fn ufs_make_empty(inode: *mut inode, dir: *mut inode) -> i32;
    pub fn ufs_find_entry(inode: *mut inode, qstr: *const qstr, folio: *mut *mut folio) -> *mut ufs_dir_entry;
    pub fn ufs_delete_entry(inode: *mut inode, de: *mut ufs_dir_entry, folio: *mut folio) -> i32;
    pub fn ufs_empty_dir(inode: *mut inode) -> i32;
    pub fn ufs_dotdot(inode: *mut inode, folio: *mut *mut folio) -> *mut ufs_dir_entry;
    pub fn ufs_set_link(dir: *mut inode, de: *mut ufs_dir_entry, folio: *mut folio, inode: *mut inode, update_times: bool) -> i32;
    pub static ufs_file_inode_operations: inode_operations;
    pub static ufs_file_operations: file_operations;
    pub static ufs_aops: address_space_operations;
    pub fn ufs_free_inode(inode: *mut inode);
    pub fn ufs_new_inode(inode: *mut inode, mode: umode_t) -> *mut inode;
    pub fn ufs_iget(sb: *mut super_block, ino: libc::c_ulong) -> *mut inode;
    pub fn ufs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32;
    pub fn ufs_sync_inode(inode: *mut inode) -> i32;
    pub fn ufs_evict_inode(inode: *mut inode);
    pub fn ufs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32;
    pub static ufs_dir_operations: file_operations;
    pub fn ufs_warning(sb: *mut super_block, fmt: *const core::ffi::c_char, ...);
    pub fn ufs_error(sb: *mut super_block, fmt: *const core::ffi::c_char, ...);
    pub fn ufs_panic(sb: *mut super_block, fmt: *const core::ffi::c_char, ...);
    pub fn ufs_mark_sb_dirty(sb: *mut super_block);
}

#[inline]
pub unsafe fn UFS_SB(sb: *mut super_block) -> *mut ufs_sb_info {
    (*sb).s_fs_info as *mut ufs_sb_info
}

#[inline]
pub unsafe fn UFS_I(inode: *mut inode) -> *mut ufs_inode_info {
    container_of(inode, core::mem::offset_of!(ufs_inode_info, vfs_inode))
}

/* Give cylinder group number for a file system block. */
#[inline]
pub unsafe fn ufs_dtog(uspi: *mut ufs_sb_private_info, mut b: u64) -> u64 {
    b /= (*uspi).s_fpg;
    b
}

/* Give cylinder group block number for a file system block. */
#[inline]
pub unsafe fn ufs_dtogd(uspi: *mut ufs_sb_private_info, mut b: u64) -> u32 {
    let remainder = b % (*uspi).s_fpg;
    b /= (*uspi).s_fpg;
    remainder as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
