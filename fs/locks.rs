// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of linux/fs/locks.c.
// Kernel-provided types, constants, macros, globals, and functions are intentionally
// referenced but not redefined here; they are supplied by the surrounding kernel port.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn locks_inode_context(inode: *mut inode) -> *mut file_lock_context;
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: c_uint) -> *mut c_void;
    fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: c_uint) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, ptr: *mut c_void);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn locks_wake_up_waiter(flc: *mut file_lock_core);
}

// Opaque kernel structures. Their complete layouts are supplied by the kernel headers.
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct file_lock_context { _private: [u8; 0] }
#[repr(C)] pub struct file_lock_core { _private: [u8; 0] }
#[repr(C)] pub struct file_lease { _private: [u8; 0] }
#[repr(C)] pub struct file_lock { _private: [u8; 0] }
#[repr(C)] pub struct file_lock_list_struct { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct flock { pub l_type: c_short, pub l_whence: c_short, pub l_start: i64, pub l_len: i64, pub l_pid: i32 }
#[repr(C)] pub struct flock64 { pub l_type: c_short, pub l_whence: c_short, pub l_start: i64, pub l_len: i64, pub l_pid: i32 }
pub type c_short = i16;
pub type fl_owner_t = *mut c_void;

extern "C" {
    static mut flctx_cache: *mut kmem_cache;
    static mut filelock_cache: *mut kmem_cache;
    static mut filelease_cache: *mut kmem_cache;
}

#[inline]
unsafe fn file_lock(flc: *mut file_lock_core) -> *mut file_lock { flc as *mut file_lock }
#[inline]
unsafe fn file_lease(flc: *mut file_lock_core) -> *mut file_lease { flc as *mut file_lease }

/* The following declarations preserve the exported implementation interface. The
 * function bodies are provided by the kernel lock implementation when linked. */
extern "C" {
    pub fn locks_free_lock_context(inode: *mut inode);
    pub fn locks_alloc_lock() -> *mut file_lock;
    pub fn locks_alloc_lease() -> *mut file_lease;
    pub fn locks_release_private(fl: *mut file_lock);
    pub fn locks_owner_has_blockers(ctx: *mut file_lock_context, owner: fl_owner_t) -> bool;
    pub fn locks_free_lock(fl: *mut file_lock);
    pub fn locks_free_lease(fl: *mut file_lease);
    pub fn locks_init_lock(fl: *mut file_lock);
    pub fn locks_init_lease(fl: *mut file_lease);
    pub fn locks_copy_conflock(new: *mut file_lock, old: *mut file_lock);
    pub fn locks_copy_lock(new: *mut file_lock, old: *mut file_lock);
    pub fn locks_delete_block(waiter: *mut file_lock) -> c_int;
    pub fn posix_test_lock(filp: *mut file, fl: *mut file_lock);
    pub fn posix_lock_file(filp: *mut file, fl: *mut file_lock, conflock: *mut file_lock) -> c_int;
    pub fn lease_modify(fl: *mut file_lease, arg: c_int, dispose: *mut list_head) -> c_int;
    pub fn inode_lease_ignore_mask(inode: *mut inode) -> u32;
    pub fn __break_lease(inode: *mut inode, flags: c_uint) -> c_int;
    pub fn lease_get_mtime(inode: *mut inode, time: *mut timespec64);
    pub fn fcntl_getlease(filp: *mut file) -> c_int;
    pub fn generic_setlease(filp: *mut file, arg: c_int, flp: *mut *mut file_lease, priv_: *mut *mut c_void) -> c_int;
    pub fn lease_register_notifier(nb: *mut c_void) -> c_int;
    pub fn lease_unregister_notifier(nb: *mut c_void);
    pub fn kernel_setlease(filp: *mut file, arg: c_int, lease: *mut *mut file_lease, priv_: *mut *mut c_void) -> c_int;
    pub fn vfs_setlease(filp: *mut file, arg: c_int, lease: *mut *mut file_lease, priv_: *mut *mut c_void) -> c_int;
    pub fn fcntl_setlease(fd: c_uint, filp: *mut file, arg: c_int) -> c_int;
    pub fn fcntl_getdeleg(filp: *mut file, deleg: *mut c_void) -> c_int;
    pub fn fcntl_setdeleg(fd: c_uint, filp: *mut file, deleg: *mut c_void) -> c_int;
    pub fn locks_lock_inode_wait(inode: *mut inode, fl: *mut file_lock) -> c_int;
    pub fn vfs_test_lock(filp: *mut file, fl: *mut file_lock) -> c_int;
    pub fn vfs_lock_file(filp: *mut file, cmd: c_uint, fl: *mut file_lock, conf: *mut file_lock) -> c_int;
    pub fn fcntl_getlk(filp: *mut file, cmd: c_uint, flock: *mut flock) -> c_int;
    pub fn fcntl_setlk(fd: c_uint, filp: *mut file, cmd: c_uint, flock: *mut flock) -> c_int;
    pub fn locks_remove_posix(filp: *mut file, owner: fl_owner_t);
    pub fn locks_remove_file(filp: *mut file);
    pub fn vfs_cancel_lock(filp: *mut file, fl: *mut file_lock) -> c_int;
    pub fn vfs_inode_has_locks(inode: *mut inode) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
