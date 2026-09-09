// SPDX-License-Identifier: GPL-2.0
/*
 * Routines that mimic syscalls, but don't use the user address space or file
 * descriptors. Only for init/ and related early init code.
 *
 * The declarations referenced below are supplied by the surrounding kernel
 * translation units.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn kern_path(name: *const c_char, flags: c_int, path: *mut path) -> c_int;
    fn path_pivot_root(new_path: *const path, old_path: *const path) -> c_int;
    fn path_mount(dev_name: *const c_char, path: *const path, type_page: *const c_char,
                  flags: c_ulong, data_page: *mut c_void) -> c_int;
    fn path_put(path: *const path);
    fn path_umount(path: *const path, flags: c_int) -> c_int;
    fn path_permission(path: *const path, mask: c_int) -> c_int;
    fn set_fs_pwd(fs: *mut fs_struct, path: *const path);
    fn current_user_ns() -> *mut user_namespace;
    fn ns_capable(ns: *mut user_namespace, cap: c_int) -> bool;
    fn security_path_chroot(path: *const path) -> c_int;
    fn set_fs_root(fs: *mut fs_struct, path: *const path);
    fn mnt_want_write(mnt: *mut mount) -> c_int;
    fn mnt_drop_write(mnt: *mut mount);
    fn chown_common(path: *const path, user: uid_t, group: gid_t) -> c_int;
    fn chmod_common(path: *const path, mode: umode_t) -> c_int;
    fn vfs_getattr(path: *const path, stat: *mut kstat, request_mask: c_uint,
                   query_flags: c_int) -> c_int;
    fn filename_mknodat(dfd: c_int, name: *mut filename, mode: umode_t, dev: c_uint) -> c_int;
    fn filename_linkat(oldfd: c_int, old: *mut filename, newfd: c_int,
                       new: *mut filename, flags: c_int) -> c_int;
    fn filename_symlinkat(old: *mut filename, newfd: c_int, new: *mut filename) -> c_int;
    fn filename_unlinkat(dfd: c_int, name: *mut filename) -> c_int;
    fn filename_mkdirat(dfd: c_int, name: *mut filename, mode: umode_t) -> c_int;
    fn filename_rmdir(dfd: c_int, name: *mut filename) -> c_int;
    fn vfs_utimes(path: *const path, ts: *mut timespec64) -> c_int;
    fn get_unused_fd_flags(flags: c_uint) -> c_int;
    fn get_file(file: *mut file) -> *mut file;
    fn fd_install(fd: c_int, file: *mut file);
}

type uid_t = c_uint;
type gid_t = c_uint;
type umode_t = c_uint;

#[repr(C)] pub struct path { pub mnt: *mut mount, pub dentry: *mut c_void }
#[repr(C)] pub struct mount { _private: [u8; 0] }
#[repr(C)] pub struct fs_struct { _private: [u8; 0] }
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct filename { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }

extern "C" {
    static mut current: *mut task_struct;
}
#[repr(C)] pub struct task_struct { pub fs: *mut fs_struct }

const LOOKUP_FOLLOW: c_int = 0x0001;
const LOOKUP_DIRECTORY: c_int = 0x0002;
const LOOKUP_MOUNTPOINT: c_int = 0x0004;
const UMOUNT_NOFOLLOW: c_int = 0x0001;
const AT_SYMLINK_NOFOLLOW: c_int = 0x0100;
const AT_NO_AUTOMOUNT: c_int = 0x0800;
const AT_FDCWD: c_int = -100;
const MAY_EXEC: c_int = 1 << 2;
const MAY_CHDIR: c_int = 1 << 9;
const MAY_ACCESS: c_int = 1 << 9;
const CAP_SYS_CHROOT: c_int = 18;
const EPERM: c_int = 1;
const STATX_BASIC_STATS: c_uint = 0x07ff;

pub unsafe fn init_pivot_root(new_root: *const c_char, put_old: *const c_char) -> c_int {
    let mut new_path = core::mem::zeroed::<path>();
    let mut old_path = core::mem::zeroed::<path>();
    let mut ret = kern_path(new_root, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &mut new_path);
    if ret != 0 { return ret; }
    ret = kern_path(put_old, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &mut old_path);
    if ret != 0 { path_put(&old_path); path_put(&new_path); return ret; }
    ret = path_pivot_root(&new_path, &old_path);
    path_put(&old_path); path_put(&new_path); ret
}

pub unsafe fn init_mount(dev_name: *const c_char, dir_name: *const c_char,
                         type_page: *const c_char, flags: c_ulong,
                         data_page: *mut c_void) -> c_int {
    let mut path = core::mem::zeroed::<path>();
    let ret = kern_path(dir_name, LOOKUP_FOLLOW, &mut path);
    if ret != 0 { return ret; }
    let ret = path_mount(dev_name, &path, type_page, flags, data_page);
    path_put(&path); ret
}

pub unsafe fn init_umount(name: *const c_char, flags: c_int) -> c_int {
    let mut lookup_flags = LOOKUP_MOUNTPOINT;
    if flags & UMOUNT_NOFOLLOW == 0 { lookup_flags |= LOOKUP_FOLLOW; }
    let mut path = core::mem::zeroed::<path>();
    let ret = kern_path(name, lookup_flags, &mut path);
    if ret != 0 { return ret; }
    path_umount(&path, flags)
}

pub unsafe fn init_chdir(filename: *const c_char) -> c_int {
    let mut path = core::mem::zeroed::<path>();
    let mut error = kern_path(filename, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &mut path);
    if error == 0 { error = path_permission(&path, MAY_EXEC | MAY_CHDIR); }
    if error == 0 { set_fs_pwd((*current).fs, &path); }
    path_put(&path); error
}

pub unsafe fn init_chroot(filename: *const c_char) -> c_int {
    let mut path = core::mem::zeroed::<path>();
    let mut error = kern_path(filename, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &mut path);
    if error != 0 { return error; }
    error = path_permission(&path, MAY_EXEC | MAY_CHDIR);
    if error == 0 { error = -EPERM; }
    if error == -EPERM && !ns_capable(current_user_ns(), CAP_SYS_CHROOT) { path_put(&path); return error; }
    error = security_path_chroot(&path);
    if error == 0 { set_fs_root((*current).fs, &path); }
    path_put(&path); error
}

pub unsafe fn init_chown(filename: *const c_char, user: uid_t, group: gid_t, flags: c_int) -> c_int {
    let lookup_flags = if flags & AT_SYMLINK_NOFOLLOW != 0 { 0 } else { LOOKUP_FOLLOW };
    let mut path = core::mem::zeroed::<path>();
    let mut error = kern_path(filename, lookup_flags, &mut path);
    if error != 0 { return error; }
    error = mnt_want_write(path.mnt);
    if error == 0 { error = chown_common(&path, user, group); mnt_drop_write(path.mnt); }
    path_put(&path); error
}

pub unsafe fn init_chmod(filename: *const c_char, mode: umode_t) -> c_int {
    let mut path = core::mem::zeroed::<path>(); let mut error = kern_path(filename, LOOKUP_FOLLOW, &mut path);
    if error == 0 { error = chmod_common(&path, mode); } path_put(&path); error
}

pub unsafe fn init_eaccess(filename: *const c_char) -> c_int {
    let mut path = core::mem::zeroed::<path>(); let mut error = kern_path(filename, LOOKUP_FOLLOW, &mut path);
    if error == 0 { error = path_permission(&path, MAY_ACCESS); } path_put(&path); error
}

pub unsafe fn init_stat(filename: *const c_char, stat: *mut kstat, flags: c_int) -> c_int {
    let lookup_flags = if flags & AT_SYMLINK_NOFOLLOW != 0 { 0 } else { LOOKUP_FOLLOW };
    let mut path = core::mem::zeroed::<path>(); let mut error = kern_path(filename, lookup_flags, &mut path);
    if error == 0 { error = vfs_getattr(&path, stat, STATX_BASIC_STATS, flags | AT_NO_AUTOMOUNT); }
    path_put(&path); error
}

// `filename_kernel` is the kernel filename wrapper used by the C source.
extern "C" { fn filename_kernel_init(name: *const c_char) -> *mut filename; }

pub unsafe fn init_mknod(filename: *const c_char, mode: umode_t, dev: c_uint) -> c_int {
    filename_mknodat(AT_FDCWD, filename_kernel_init(filename), mode, dev)
}
pub unsafe fn init_link(oldname: *const c_char, newname: *const c_char) -> c_int {
    filename_linkat(AT_FDCWD, filename_kernel_init(oldname), AT_FDCWD, filename_kernel_init(newname), 0)
}
pub unsafe fn init_symlink(oldname: *const c_char, newname: *const c_char) -> c_int {
    filename_symlinkat(filename_kernel_init(oldname), AT_FDCWD, filename_kernel_init(newname))
}
pub unsafe fn init_unlink(pathname: *const c_char) -> c_int {
    filename_unlinkat(AT_FDCWD, filename_kernel_init(pathname))
}
pub unsafe fn init_mkdir(pathname: *const c_char, mode: umode_t) -> c_int {
    filename_mkdirat(AT_FDCWD, filename_kernel_init(pathname), mode)
}
pub unsafe fn init_rmdir(pathname: *const c_char) -> c_int {
    filename_rmdir(AT_FDCWD, filename_kernel_init(pathname))
}

pub unsafe fn init_utimes(filename: *mut c_char, ts: *mut timespec64) -> c_int {
    let mut path = core::mem::zeroed::<path>(); let mut error = kern_path(filename, 0, &mut path);
    if error == 0 { error = vfs_utimes(&path, ts); } path_put(&path); error
}

pub unsafe fn init_dup(file: *mut file) -> c_int {
    let fd = get_unused_fd_flags(0);
    if fd < 0 { return fd; }
    fd_install(fd, get_file(file)); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
