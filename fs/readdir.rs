// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/readdir.c. Kernel-provided types and operations
 * are intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};

#[repr(C)] pub struct file { pub f_op: *mut file_operations, pub f_pos: loff_t }
#[repr(C)] pub struct file_operations { pub iterate_shared: Option<unsafe extern "C" fn(*mut file, *mut dir_context) -> c_int> }
#[repr(C)] pub struct inode { pub i_rwsem: rw_semaphore }
#[repr(C)] pub struct rw_semaphore;
#[repr(C)] pub struct dir_context { pub actor: Option<unsafe extern "C" fn(*mut dir_context, *const c_char, c_int, loff_t, u64, c_uint) -> bool>, pub pos: loff_t, pub count: usize, pub dt_flags_mask: c_uint }
pub type loff_t = i64;
pub type u64 = u64;
pub type compat_ulong_t = c_uint;
pub type compat_long_t = c_int;

extern "C" {
    fn file_inode(f: *mut file) -> *mut inode;
    fn up_read(s: *mut rw_semaphore); fn down_write(s: *mut rw_semaphore);
    fn downgrade_write(s: *mut rw_semaphore); fn down_read_killable(s: *mut rw_semaphore) -> c_int;
    fn inode_unlock_shared(i: *mut inode);
    fn security_file_permission(f: *mut file, mask: c_uint) -> c_int;
    fn fsnotify_file_perm(f: *mut file, mask: c_uint) -> c_int;
    fn fsnotify_access(f: *mut file); fn file_accessed(f: *mut file);
    fn is_deaddir(i: *mut inode) -> bool;
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn signal_pending(task: *mut c_void) -> bool;
    fn put_user<T>(v: T, p: *mut T) -> c_int;
    fn fd_empty(f: *mut c_void) -> bool; fn fd_file(f: *mut c_void) -> *mut file;
}

const ENOENT: c_int = 2; const ENOTDIR: c_int = 20; const EIO: c_int = 5;
const EFAULT: c_int = 14; const EBADF: c_int = 9; const EINVAL: c_int = 22;
const EOVERFLOW: c_int = 75; const MAY_READ: c_uint = 4; const PATH_MAX: c_int = 4096;
const FILLDIR_FLAG_NOINTR: c_uint = 1; const S_DT_MASK: c_uint = 0xf;

#[repr(C)] pub struct old_linux_dirent { pub d_ino: c_ulong, pub d_offset: c_ulong, pub d_namlen: c_ushort, pub d_name: [c_char; 0] }
#[repr(C)] pub struct linux_dirent { pub d_ino: c_ulong, pub d_off: c_ulong, pub d_reclen: c_ushort, pub d_name: [c_char; 0] }
#[repr(C)] pub struct linux_dirent64 { pub d_ino: u64, pub d_off: i64, pub d_reclen: c_ushort, pub d_type: u8, pub d_name: [c_char; 0] }
#[repr(C)] pub struct readdir_callback { pub ctx: dir_context, pub dirent: *mut old_linux_dirent, pub result: c_int }
#[repr(C)] pub struct getdents_callback { pub ctx: dir_context, pub current_dir: *mut linux_dirent, pub prev_reclen: c_int, pub error: c_int }
#[repr(C)] pub struct getdents_callback64 { pub ctx: dir_context, pub current_dir: *mut linux_dirent64, pub prev_reclen: c_int, pub error: c_int }

unsafe fn verify_dirent_name(name: *const c_char, len: c_int) -> c_int { if len <= 0 || len >= PATH_MAX || !memchr(name as *const c_void, b'/' as c_int, len as usize).is_null() { -EIO } else { 0 } }

pub unsafe extern "C" fn wrap_directory_iterator(file: *mut file, ctx: *mut dir_context, iter: unsafe extern "C" fn(*mut file, *mut dir_context) -> c_int) -> c_int {
    let inode = file_inode(file); up_read(&mut (*inode).i_rwsem); down_write(&mut (*inode).i_rwsem);
    let ret = if !is_deaddir(inode) { iter(file, ctx) } else { -ENOENT }; downgrade_write(&mut (*inode).i_rwsem); ret
}

pub unsafe extern "C" fn iterate_dir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file); let mut res = -ENOTDIR;
    let ops = (*file).f_op;
    if (*ops).iterate_shared.is_none() { return res; }
    res = security_file_permission(file, MAY_READ); if res != 0 { return res; }
    res = fsnotify_file_perm(file, MAY_READ); if res != 0 { return res; }
    res = down_read_killable(&mut (*inode).i_rwsem); if res != 0 { return res; }
    res = -ENOENT;
    if !is_deaddir(inode) { (*ctx).pos = (*file).f_pos; res = ((*ops).iterate_shared.unwrap())(file, ctx); (*file).f_pos = (*ctx).pos; fsnotify_access(file); file_accessed(file); }
    inode_unlock_shared(inode); res
}

unsafe fn dirent_size<T>(_: *mut T, len: usize) -> usize { core::mem::size_of::<T>() + len }
unsafe fn copy_name(dst: *mut c_char, src: *const c_char, len: usize) { core::ptr::copy_nonoverlapping(src, dst, len); *dst.add(len) = 0; }

unsafe extern "C" fn fillonedir(ctx: *mut dir_context, name: *const c_char, namlen: c_int, offset: loff_t, ino: u64, _d_type: c_uint) -> bool {
    let buf = ctx as *mut readdir_callback; if (*buf).result != 0 { return false; } (*buf).result = verify_dirent_name(name, namlen); if (*buf).result != 0 { return false; }
    let d_ino = ino as c_ulong; if core::mem::size_of::<c_ulong>() < core::mem::size_of::<u64>() && d_ino as u64 != ino { (*buf).result = -EOVERFLOW; return false; }
    (*buf).result += 1; let d = (*buf).dirent; (*d).d_ino = d_ino; (*d).d_offset = offset as c_ulong; (*d).d_namlen = namlen as c_ushort; copy_name((*d).d_name.as_mut_ptr(), name, namlen as usize); true
}

pub unsafe extern "C" fn old_readdir(_fd: c_uint, _dirent: *mut old_linux_dirent, _count: c_uint) -> c_int { -ENOTDIR }

unsafe extern "C" fn filldir(ctx: *mut dir_context, name: *const c_char, namlen: c_int, offset: loff_t, ino: u64, mut d_type: c_uint) -> bool {
    let buf = ctx as *mut getdents_callback; let reclen = ((core::mem::size_of::<linux_dirent>() + namlen as usize + 2 + core::mem::size_of::<c_ulong>() - 1) / core::mem::size_of::<c_ulong>() * core::mem::size_of::<c_ulong>()) as c_int;
    let flags = d_type; d_type &= S_DT_MASK; (*buf).error = verify_dirent_name(name, namlen); if (*buf).error != 0 { return false; } (*buf).error = -EINVAL; if reclen as usize > (*ctx).count { return false; }
    let d = (*buf).current_dir; let prev = (d as *mut u8).offset(-(*buf).prev_reclen as isize) as *mut linux_dirent; if flags & FILLDIR_FLAG_NOINTR == 0 && (*buf).prev_reclen != 0 && signal_pending(core::ptr::null_mut()) { return false; }
    (*prev).d_off = offset as c_ulong; (*d).d_ino = ino as c_ulong; (*d).d_reclen = reclen as c_ushort; *((d as *mut u8).add(reclen as usize - 1)) = d_type as u8; copy_name((*d).d_name.as_mut_ptr(), name, namlen as usize);
    (*buf).current_dir = (d as *mut u8).add(reclen as usize) as *mut linux_dirent; (*buf).prev_reclen = reclen; (*ctx).count -= reclen as usize; true
}
unsafe extern "C" fn filldir64(ctx: *mut dir_context, name: *const c_char, namlen: c_int, offset: loff_t, ino: u64, mut d_type: c_uint) -> bool {
    let buf = ctx as *mut getdents_callback64; let reclen = ((core::mem::size_of::<linux_dirent64>() + namlen as usize + 1 + 7) / 8 * 8) as c_int; d_type &= S_DT_MASK;
    (*buf).error = verify_dirent_name(name, namlen); if (*buf).error != 0 { return false; } (*buf).error = -EINVAL; if reclen as usize > (*ctx).count { return false; }
    let d = (*buf).current_dir; let prev = (d as *mut u8).offset(-(*buf).prev_reclen as isize) as *mut linux_dirent64; (*prev).d_off = offset; (*d).d_ino = ino; (*d).d_reclen = reclen as c_ushort; (*d).d_type = d_type as u8; copy_name((*d).d_name.as_mut_ptr(), name, namlen as usize);
    (*buf).prev_reclen = reclen; (*buf).current_dir = (d as *mut u8).add(reclen as usize) as *mut linux_dirent64; (*ctx).count -= reclen as usize; true
}
pub unsafe extern "C" fn getdents(_fd: c_uint, _dirent: *mut linux_dirent, _count: c_uint) -> c_int { -ENOTDIR }
pub unsafe extern "C" fn getdents64(_fd: c_uint, _dirent: *mut linux_dirent64, _count: c_uint) -> c_int { -ENOTDIR }

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct compat_old_linux_dirent { pub d_ino: compat_ulong_t, pub d_offset: compat_ulong_t, pub d_namlen: c_ushort, pub d_name: [c_char; 0] }
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct compat_linux_dirent { pub d_ino: compat_ulong_t, pub d_off: compat_ulong_t, pub d_reclen: c_ushort, pub d_name: [c_char; 0] }
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct compat_readdir_callback { pub ctx: dir_context, pub dirent: *mut compat_old_linux_dirent, pub result: c_int }
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct compat_getdents_callback { pub ctx: dir_context, pub current_dir: *mut compat_linux_dirent, pub prev_reclen: c_int, pub error: c_int }

#[cfg(feature = "CONFIG_COMPAT")]
unsafe extern "C" fn compat_fillonedir(ctx: *mut dir_context, name: *const c_char, namlen: c_int, offset: loff_t, ino: u64, _d_type: c_uint) -> bool {
    let b = ctx as *mut compat_readdir_callback; if (*b).result != 0 { return false; } (*b).result = verify_dirent_name(name, namlen); if (*b).result != 0 { return false; }
    let d = (*b).dirent; (*d).d_ino = ino as compat_ulong_t; (*d).d_offset = offset as compat_ulong_t; (*d).d_namlen = namlen as c_ushort; copy_name((*d).d_name.as_mut_ptr(), name, namlen as usize); (*b).result += 1; true
}
#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe extern "C" fn compat_old_readdir(_fd: c_uint, _dirent: *mut compat_old_linux_dirent, _count: c_uint) -> c_int { -ENOTDIR }
#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe extern "C" fn compat_getdents(_fd: c_uint, _dirent: *mut compat_linux_dirent, _count: c_uint) -> c_int { -ENOTDIR }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
