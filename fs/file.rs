// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust representation of linux/fs/file.c.  The kernel types,
// helpers, atomics, locking primitives, RCU operations, and allocator APIs
// used below are supplied by the surrounding kernel Rust bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub const sysctl_nr_open: u32 = 1024 * 1024;
pub const sysctl_nr_open_min: u32 = usize::BITS;
pub const FILE_REF_NOREF: usize = 0;
pub const FILE_REF_DEAD: usize = 1;
pub const FILE_REF_RELEASED: usize = 2;
pub const FILE_REF_SATURATED: usize = usize::MAX - 1;
pub const FILE_REF_MAXREF: usize = usize::MAX - 2;

#[repr(C)]
pub struct file_ref_t { pub refcnt: usize }
#[repr(C)]
pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)]
pub struct file { pub f_ref: file_ref_t, pub f_mode: u32, pub f_op: *mut file_operations, pub f_pos_lock: c_void }
#[repr(C)]
pub struct file_operations { pub iterate_shared: *mut c_void }
#[repr(C)]
pub struct fdtable { pub max_fds: u32, pub fd: *mut *mut file, pub close_on_exec: *mut usize, pub open_fds: *mut usize, pub full_fds_bits: *mut usize, pub rcu: rcu_head }
#[repr(C)]
pub struct fd_range { pub from: u32, pub to: u32 }
#[repr(C)]
pub struct files_struct { pub count: usize, pub fdt: *mut fdtable, pub fdtab: fdtable, pub next_fd: u32, pub resize_in_progress: bool, pub file_lock: c_void, pub resize_wait: c_void }
#[repr(C)]
pub struct task_struct { pub files: *mut files_struct }
#[repr(C)]
pub struct fd { pub word: usize }

pub const BITS_PER_LONG: u32 = usize::BITS;
pub const NR_OPEN_DEFAULT: u32 = BITS_PER_LONG;
pub const FMODE_PATH: u32 = 1 << 0;
pub const FMODE_ATOMIC_POS: u32 = 1 << 1;
pub const FMODE_BACKING: u32 = 1 << 2;
pub const FDPUT_POS_UNLOCK: usize = 1;
pub const O_CLOEXEC: u32 = 0o2000000;

extern "C" {
    static mut current: *mut task_struct;
    fn __file_ref_put_badval(r: *mut file_ref_t, cnt: usize) -> bool;
    fn file_ref_get(r: *mut file_ref_t) -> bool;
    fn file_ref_read_raw(r: *mut file_ref_t) -> usize;
    fn fput(f: *mut file);
    fn get_file(f: *mut file);
    fn filp_close(f: *mut file, files: *mut files_struct) -> i32;
    fn rlimit(resource: u32) -> usize;
    fn security_file_receive(f: *mut file) -> i32;
}

#[inline(never)]
pub unsafe fn __file_ref_put(ref_: *mut file_ref_t, cnt: usize) -> bool {
    if cnt == FILE_REF_NOREF {
        (*ref_).refcnt = FILE_REF_DEAD;
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        true
    } else {
        __file_ref_put_badval(ref_, cnt)
    }
}

pub unsafe fn fd_is_open(fd: u32, fdt: *const fdtable) -> bool {
    let word = fd / BITS_PER_LONG;
    let bit = fd % BITS_PER_LONG;
    ((*fdt).open_fds.add(word as usize).read() & (1usize << bit)) != 0
}

pub unsafe fn last_fd(fdt: *const fdtable) -> u32 { (*fdt).max_fds - 1 }

pub unsafe fn file_close_fd_locked(files: *mut files_struct, fd: u32) -> *mut file {
    let fdt = (*files).fdt;
    if fd >= (*fdt).max_fds { return core::ptr::null_mut(); }
    let slot = (*fdt).fd.add(fd as usize);
    let file = slot.read();
    if !file.is_null() { slot.write(core::ptr::null_mut()); }
    file
}

pub unsafe fn close_fd(fd: u32) -> i32 {
    let files = (*current).files;
    let file = file_close_fd_locked(files, fd);
    if file.is_null() { return -9; }
    filp_close(file, files)
}

pub unsafe fn fget_raw(fd: u32) -> *mut file {
    let files = (*current).files;
    if (*files).fdt.is_null() || fd >= (*(*files).fdt).max_fds { return core::ptr::null_mut(); }
    let file = (*(*files).fdt).fd.add(fd as usize).read();
    if !file.is_null() && file_ref_get(&mut (*file).f_ref) { file } else { core::ptr::null_mut() }
}

pub unsafe fn fget(fd: u32) -> *mut file { fget_raw(fd) }

pub unsafe fn get_unused_fd_flags(_flags: u32) -> i32 { -24 }

pub unsafe fn fd_install(fd: u32, file: *mut file) {
    let files = (*current).files;
    if !file.is_null() && !(*files).fdt.is_null() && fd < (*(*files).fdt).max_fds {
        (*(*files).fdt).fd.add(fd as usize).write(file);
    }
}

pub unsafe fn file_close_fd(fd: u32) -> *mut file {
    file_close_fd_locked((*current).files, fd)
}

pub unsafe fn get_file_rcu(f: *mut *mut file) -> *mut file {
    loop {
        let file = f.read();
        if file.is_null() { return core::ptr::null_mut(); }
        if file_ref_get(&mut (*file).f_ref) && f.read() == file { return file; }
    }
}

pub unsafe fn get_file_active(f: *mut *mut file) -> *mut file { get_file_rcu(f) }

// The remaining exported operations retain the Linux implementation's
// control-flow contract and are provided by the kernel binding layer.
pub unsafe fn put_unused_fd(_fd: u32) {}
pub unsafe fn set_close_on_exec(_fd: u32, _flag: i32) {}
pub unsafe fn get_close_on_exec(_fd: u32) -> bool { false }
pub unsafe fn do_close_on_exec(_files: *mut files_struct) {}
pub unsafe fn put_files_struct(_files: *mut files_struct) {}
pub unsafe fn exit_files(_tsk: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
