/* SPDX-License-Identifier: GPL-2.0 */
/* Wrapper functions for accessing the file_struct fd array. */

/* Dependencies supplied by the surrounding kernel translation unit. */

use core::ffi::c_char;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn fput(file: *mut file);
    pub fn alloc_file_pseudo(
        inode: *mut inode, mnt: *mut vfsmount, name: *const c_char,
        flags: core::ffi::c_int, fops: *const file_operations,
    ) -> *mut file;
    pub fn alloc_file_pseudo_noaccount(
        inode: *mut inode, mnt: *mut vfsmount, name: *const c_char,
        flags: core::ffi::c_int, fops: *const file_operations,
    ) -> *mut file;
    pub fn alloc_file_clone(
        file: *mut file, flags: core::ffi::c_int, fops: *const file_operations,
    ) -> *mut file;
    pub fn fget(fd: u32) -> *mut file;
    pub fn fget_raw(fd: u32) -> *mut file;
    pub fn fget_task(task: *mut task_struct, fd: u32) -> *mut file;
    pub fn fget_task_next(task: *mut task_struct, fd: *mut u32) -> *mut file;
    pub fn __f_unlock_pos(file: *mut file);
    pub fn fdget(fd: u32) -> fd;
    pub fn fdget_raw(fd: u32) -> fd;
    pub fn fdget_pos(fd: u32) -> fd;
    pub fn f_dupfd(from: u32, file: *mut file, flags: u32) -> core::ffi::c_int;
    pub fn replace_fd(fd: u32, file: *mut file, flags: u32) -> core::ffi::c_int;
    pub fn set_close_on_exec(fd: u32, flag: core::ffi::c_int);
    pub fn get_close_on_exec(fd: u32) -> bool;
    pub fn __get_unused_fd_flags(flags: u32, nofile: usize) -> core::ffi::c_int;
    pub fn get_unused_fd_flags(flags: u32) -> core::ffi::c_int;
    pub fn put_unused_fd(fd: u32);
    pub fn fd_install(fd: u32, file: *mut file);
    pub fn receive_fd(file: *mut file, ufd: *mut core::ffi::c_int, o_flags: u32) -> core::ffi::c_int;
    pub fn receive_fd_replace(new_fd: core::ffi::c_int, file: *mut file, o_flags: u32) -> core::ffi::c_int;
    pub fn flush_delayed_fput();
    pub fn __fput_sync(file: *mut file);
}

pub static mut sysctl_nr_open_min: u32 = 0;
pub static mut sysctl_nr_open_max: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd {
    pub word: usize,
}

pub const FDPUT_FPUT: usize = 1;
pub const FDPUT_POS_UNLOCK: usize = 2;

#[inline]
pub unsafe fn fd_file(f: fd) -> *mut file {
    (f.word & !(FDPUT_FPUT | FDPUT_POS_UNLOCK)) as *mut file
}

#[inline]
pub fn fd_empty(f: fd) -> bool { f.word == 0 }

pub const EMPTY_FD: fd = fd { word: 0 };

#[inline]
pub fn BORROWED_FD(f: *mut file) -> fd { fd { word: f as usize } }

#[inline]
pub fn CLONED_FD(f: *mut file) -> fd { fd { word: f as usize | FDPUT_FPUT } }

#[inline]
pub unsafe fn fdput(value: fd) {
    if value.word & FDPUT_FPUT != 0 { fput(fd_file(value)); }
}

#[inline]
pub unsafe fn fdput_pos(value: fd) {
    if value.word & FDPUT_POS_UNLOCK != 0 { __f_unlock_pos(fd_file(value)); }
    fdput(value);
}

/* DEFINE_CLASS(fd, fd_raw, and fd_pos) are represented by their constructors
 * and cleanup functions above; the surrounding cleanup framework supplies the
 * automatic-scope behavior. */

#[repr(C)]
pub struct fd_prepare {
    pub err: i32,
    pub __fd: i32,
    pub __file: *mut file,
}

pub type class_fd_prepare_t = fd_prepare;

#[inline]
pub fn fd_prepare_fd(fdf: &fd_prepare) -> i32 { fdf.__fd }
#[inline]
pub fn fd_prepare_file(fdf: &fd_prepare) -> *mut file { fdf.__file }

#[inline]
pub unsafe fn class_fd_prepare_destructor(fdf: *const fd_prepare) {
    if (*fdf).__fd >= 0 { put_unused_fd((*fdf).__fd as u32); }
    if !(*fdf).__file.is_null() { fput((*fdf).__file); }
}

#[inline]
pub unsafe fn class_fd_prepare_lock_err(fdf: *const fd_prepare) -> i32 {
    if (*fdf).err != 0 { return (*fdf).err; }
    if (*fdf).__fd < 0 { return (*fdf).__fd; }
    if (*fdf).__file.is_null() { return -12; }
    0
}

/* take_fd(), FD_PREPARE(), fd_publish(), __FD_ADD(), and FD_ADD() retain the
 * cleanup-framework macro semantics and are intentionally expressed by the
 * surrounding translation's cleanup support. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
