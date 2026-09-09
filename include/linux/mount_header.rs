/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for mount interface. This describes the in the kernel build
 * linkedlist with mounted filesystems.
 *
 * Author: Marco van Wieringen <mvw@planets.elm.net>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_system_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fs_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct path {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum mount_flags {
    MNT_NOSUID = 0x01,
    MNT_NODEV = 0x02,
    MNT_NOEXEC = 0x04,
    MNT_NOATIME = 0x08,
    MNT_NODIRATIME = 0x10,
    MNT_RELATIME = 0x20,
    MNT_READONLY = 0x40,
    MNT_NOSYMFOLLOW = 0x80,
    MNT_SHRINKABLE = 0x100,
    MNT_INTERNAL = 0x4000,
    MNT_LOCK_ATIME = 0x040000,
    MNT_LOCK_NOEXEC = 0x080000,
    MNT_LOCK_NOSUID = 0x100000,
    MNT_LOCK_NODEV = 0x200000,
    MNT_LOCK_READONLY = 0x400000,
    MNT_LOCKED = 0x800000,
    MNT_DOOMED = 0x1000000,
    MNT_SYNC_UMOUNT = 0x2000000,
    MNT_UMOUNT = 0x8000000,
    MNT_USER_SETTABLE_MASK = 0x01 | 0x02 | 0x04 | 0x08 | 0x10 | 0x20 | 0x40 | 0x80,
    MNT_ATIME_MASK = 0x08 | 0x10 | 0x20,
    MNT_INTERNAL_FLAGS = 0x4000 | 0x1000000 | 0x2000000 | 0x800000,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_root: *mut dentry,
    pub mnt_sb: *mut super_block,
    pub mnt_flags: c_int,
    pub mnt_idmap: *mut mnt_idmap,
}

#[inline]
pub unsafe fn mnt_idmap(mnt: *const vfsmount) -> *mut mnt_idmap {
    /* Pairs with smp_store_release() in do_idmap_mount(). */
    core::ptr::read_volatile(core::ptr::addr_of!((*mnt).mnt_idmap))
}

unsafe extern "C" {
    pub fn mnt_want_write(mnt: *mut vfsmount) -> c_int;
    pub fn mnt_want_write_file(file: *mut file) -> c_int;
    pub fn mnt_drop_write(mnt: *mut vfsmount);
    pub fn mnt_drop_write_file(file: *mut file);
    pub fn mntput(mnt: *mut vfsmount);
    pub fn mntget(mnt: *mut vfsmount) -> *mut vfsmount;
    pub fn mnt_make_shortterm(mnt: *mut vfsmount);
    pub fn mnt_clone_internal(path: *const path) -> *mut vfsmount;
    pub fn __mnt_is_readonly(mnt: *const vfsmount) -> bool;
    pub fn mnt_may_suid(mnt: *mut vfsmount) -> bool;
    pub fn clone_private_mount(path: *const path) -> *mut vfsmount;
    pub fn mnt_get_write_access(mnt: *mut vfsmount) -> c_int;
    pub fn mnt_put_write_access(mnt: *mut vfsmount);
    pub fn fc_mount(fc: *mut fs_context) -> *mut vfsmount;
    pub fn fc_mount_longterm(fc: *mut fs_context) -> *mut vfsmount;
    pub fn vfs_create_mount(fc: *mut fs_context) -> *mut vfsmount;
    pub fn vfs_kern_mount(type_: *mut file_system_type, flags: c_int, name: *const c_char, data: *mut c_void) -> *mut vfsmount;
    pub fn mnt_set_expiry(mnt: *mut vfsmount, expiry_list: *mut list_head);
    pub fn mark_mounts_for_expiry(mounts: *mut list_head);
    pub fn path_is_mountpoint(path: *const path) -> bool;
    pub fn our_mnt(mnt: *mut vfsmount) -> bool;
    pub fn kern_mount(type_: *mut file_system_type) -> *mut vfsmount;
    pub fn kern_unmount(mnt: *mut vfsmount);
    pub fn may_umount_tree(mnt: *mut vfsmount) -> c_int;
    pub fn may_umount(mnt: *mut vfsmount) -> c_int;
    pub fn do_mount(dev_name: *const c_char, dir_name: *const c_char, type_: *const c_char, flags: c_ulong, data: *mut c_void) -> c_int;
    pub fn collect_paths(path: *const path, p: *mut path, count: c_uint) -> *const path;
    pub fn drop_collected_paths(path: *const path, p: *const path);
    pub fn kern_unmount_array(mnt: *mut *mut vfsmount, num: c_uint);
    pub fn cifs_root_data(dev: *mut *mut c_char, opts: *mut *mut c_char) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
