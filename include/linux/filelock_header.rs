/* SPDX-License-Identifier: GPL-2.0 */

pub const FL_POSIX: u32 = 1 << 0;
pub const FL_FLOCK: u32 = 1 << 1;
pub const FL_DELEG: u32 = 1 << 2;
pub const FL_ACCESS: u32 = 1 << 3;
pub const FL_EXISTS: u32 = 1 << 4;
pub const FL_LEASE: u32 = 1 << 5;
pub const FL_CLOSE: u32 = 1 << 6;
pub const FL_SLEEP: u32 = 1 << 7;
pub const FL_DOWNGRADE_PENDING: u32 = 1 << 8;
pub const FL_UNLOCK_PENDING: u32 = 1 << 9;
pub const FL_OFDLCK: u32 = 1 << 10;
pub const FL_LAYOUT: u32 = 1 << 11;
pub const FL_RECLAIM: u32 = 1 << 12;
pub const FL_IGN_DIR_CREATE: u32 = 1 << 13;
pub const FL_IGN_DIR_DELETE: u32 = 1 << 14;
pub const FL_IGN_DIR_RENAME: u32 = 1 << 15;
pub const FL_CLOSE_POSIX: u32 = FL_POSIX | FL_CLOSE;
pub const FILE_LOCK_DEFERRED: i32 = 1;

pub const LEASE_BREAK_LEASE: u32 = 1 << 0;
pub const LEASE_BREAK_DELEG: u32 = 1 << 1;
pub const LEASE_BREAK_LAYOUT: u32 = 1 << 2;
pub const LEASE_BREAK_NONBLOCK: u32 = 1 << 3;
pub const LEASE_BREAK_OPEN_RDONLY: u32 = 1 << 4;
pub const LEASE_BREAK_DIR_CREATE: u32 = 1 << 5;
pub const LEASE_BREAK_DIR_DELETE: u32 = 1 << 6;
pub const LEASE_BREAK_DIR_RENAME: u32 = 1 << 7;

pub enum file_lock {}
pub enum file_lease {}
pub enum file {}
pub enum inode {}
pub enum net {}
pub enum notifier_block {}
pub enum seq_file {}
pub enum files_struct {}
pub enum delegation {}
pub enum flock {}
pub enum flock64 {}
pub enum timespec64 {}
pub enum fasync_struct {}
pub enum list_head {}
pub enum hlist_node {}
pub enum wait_queue_head_t {}
pub enum spinlock_t {}
pub type fl_owner_t = *mut core::ffi::c_void;
pub type loff_t = i64;
pub type pid_t = i32;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct file_lock_operations {
    pub fl_copy_lock: Option<unsafe extern "C" fn(*mut file_lock, *mut file_lock)>,
    pub fl_release_private: Option<unsafe extern "C" fn(*mut file_lock)>,
}

#[repr(C)]
pub struct lock_manager_operations {
    pub lm_mod_owner: *mut core::ffi::c_void,
    pub lm_get_owner: Option<unsafe extern "C" fn(fl_owner_t) -> fl_owner_t>,
    pub lm_put_owner: Option<unsafe extern "C" fn(fl_owner_t)>,
    pub lm_notify: Option<unsafe extern "C" fn(*mut file_lock)>,
    pub lm_grant: Option<unsafe extern "C" fn(*mut file_lock, i32) -> i32>,
    pub lm_lock_expirable: Option<unsafe extern "C" fn(*mut file_lock) -> bool>,
    pub lm_expire_lock: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct lease_manager_operations {
    pub lm_break: Option<unsafe extern "C" fn(*mut file_lease) -> bool>,
    pub lm_change: Option<unsafe extern "C" fn(*mut file_lease, i32, *mut list_head) -> i32>,
    pub lm_setup: Option<unsafe extern "C" fn(*mut file_lease, *mut *mut core::ffi::c_void)>,
    pub lm_breaker_owns_lease: Option<unsafe extern "C" fn(*mut file_lease) -> bool>,
    pub lm_open_conflict: Option<unsafe extern "C" fn(*mut file, i32) -> i32>,
    pub lm_breaker_timedout: Option<unsafe extern "C" fn(*mut file_lease) -> bool>,
}

#[repr(C)]
pub struct lock_manager {
    pub list: list_head,
    pub block_opens: bool,
}

extern "C" {
    pub fn locks_start_grace(net: *mut net, lm: *mut lock_manager);
    pub fn locks_end_grace(lm: *mut lock_manager);
    pub fn locks_in_grace(net: *mut net) -> bool;
    pub fn opens_in_grace(net: *mut net) -> bool;
}

#[repr(C)]
pub struct file_lock_core {
    pub flc_blocker: *mut file_lock_core,
    pub flc_list: list_head,
    pub flc_link: hlist_node,
    pub flc_blocked_requests: list_head,
    pub flc_blocked_member: list_head,
    pub flc_owner: fl_owner_t,
    pub flc_flags: core::ffi::c_uint,
    pub flc_type: u8,
    pub flc_pid: pid_t,
    pub flc_link_cpu: i32,
    pub flc_wait: wait_queue_head_t,
    pub flc_file: *mut file,
}

#[repr(C)]
pub union file_lock_union {
    pub nfs_fl: nfs_lock_info,
    pub nfs4_fl: nfs4_lock_info,
    pub afs: file_lock_afs,
    pub ceph: file_lock_ceph,
}
#[repr(C)]
pub struct file_lock_afs { pub link: list_head, pub state: i32, pub debug_id: core::ffi::c_uint }
#[repr(C)]
pub struct file_lock_ceph { pub inode: *mut inode }
pub enum nfs_lock_info {}
pub enum nfs4_lock_info {}

#[repr(C)]
pub struct file_lock {
    pub c: file_lock_core,
    pub fl_start: loff_t,
    pub fl_end: loff_t,
    pub fl_ops: *const file_lock_operations,
    pub fl_lmops: *const lock_manager_operations,
    pub fl_u: file_lock_union,
}

#[repr(C)]
pub struct file_lease {
    pub c: file_lock_core,
    pub fl_fasync: *mut fasync_struct,
    pub fl_break_time: core::ffi::c_ulong,
    pub fl_downgrade_time: core::ffi::c_ulong,
    pub fl_lmops: *const lease_manager_operations,
}

#[repr(C)]
pub struct file_lock_context {
    pub flc_lock: spinlock_t,
    pub flc_flock: list_head,
    pub flc_posix: list_head,
    pub flc_lease: list_head,
}

#[cfg(feature = "CONFIG_FILE_LOCKING")]
extern "C" {
    pub fn fcntl_getlk(file: *mut file, cmd: core::ffi::c_uint, user: *mut flock) -> i32;
    pub fn fcntl_setlk(fd: core::ffi::c_uint, file: *mut file, cmd: core::ffi::c_uint, user: *mut flock) -> i32;
    pub fn fcntl_setlease(fd: core::ffi::c_uint, filp: *mut file, arg: i32) -> i32;
    pub fn fcntl_getlease(filp: *mut file) -> i32;
    pub fn fcntl_setdeleg(fd: core::ffi::c_uint, filp: *mut file, deleg: *mut delegation) -> i32;
    pub fn fcntl_getdeleg(filp: *mut file, deleg: *mut delegation) -> i32;
    pub fn locks_free_lock_context(inode: *mut inode);
    pub fn locks_free_lock(fl: *mut file_lock);
    pub fn locks_init_lock(fl: *mut file_lock);
    pub fn locks_alloc_lock() -> *mut file_lock;
    pub fn locks_copy_lock(new: *mut file_lock, old: *mut file_lock);
    pub fn locks_copy_conflock(new: *mut file_lock, old: *mut file_lock);
    pub fn locks_remove_posix(file: *mut file, owner: fl_owner_t);
    pub fn locks_remove_file(file: *mut file);
    pub fn locks_release_private(fl: *mut file_lock);
    pub fn posix_test_lock(file: *mut file, fl: *mut file_lock);
    pub fn posix_lock_file(file: *mut file, fl: *mut file_lock, conflock: *mut file_lock) -> i32;
    pub fn locks_delete_block(waiter: *mut file_lock) -> i32;
    pub fn vfs_test_lock(file: *mut file, fl: *mut file_lock) -> i32;
    pub fn vfs_lock_file(file: *mut file, cmd: core::ffi::c_uint, fl: *mut file_lock, conf: *mut file_lock) -> i32;
    pub fn vfs_cancel_lock(filp: *mut file, fl: *mut file_lock) -> i32;
    pub fn vfs_inode_has_locks(inode: *mut inode) -> bool;
    pub fn locks_lock_inode_wait(inode: *mut inode, fl: *mut file_lock) -> i32;
    pub fn locks_init_lease(fl: *mut file_lease);
    pub fn locks_free_lease(fl: *mut file_lease);
    pub fn locks_alloc_lease() -> *mut file_lease;
    pub fn __break_lease(inode: *mut inode, flags: core::ffi::c_uint) -> i32;
    pub fn lease_get_mtime(inode: *mut inode, time: *mut timespec64);
    pub fn generic_setlease(file: *mut file, arg: i32, flp: *mut *mut file_lease, priv_: *mut *mut core::ffi::c_void) -> i32;
    pub fn kernel_setlease(file: *mut file, arg: i32, lease: *mut *mut file_lease, priv_: *mut *mut core::ffi::c_void) -> i32;
    pub fn vfs_setlease(file: *mut file, arg: i32, lease: *mut *mut file_lease, priv_: *mut *mut core::ffi::c_void) -> i32;
    pub fn lease_modify(fl: *mut file_lease, arg: i32, dispose: *mut list_head) -> i32;
    pub fn inode_lease_ignore_mask(inode: *mut inode) -> u32;
    pub fn lease_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn lease_unregister_notifier(nb: *mut notifier_block);
    pub fn show_fd_locks(f: *mut seq_file, filp: *mut file, files: *mut files_struct);
    pub fn locks_owner_has_blockers(flctx: *mut file_lock_context, owner: fl_owner_t) -> bool;
}

#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
pub unsafe fn fcntl_getlk(_: *mut file, _: core::ffi::c_uint, _: *mut flock) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
pub unsafe fn fcntl_setlk(_: core::ffi::c_uint, _: *mut file, _: core::ffi::c_uint, _: *mut flock) -> i32 { -13 }

pub const F_UNLCK: u8 = 2;
pub const F_RDLCK: u8 = 0;
pub const F_WRLCK: u8 = 1;

#[inline]
pub unsafe fn lock_is_unlock(fl: *mut file_lock) -> bool { (*fl).c.flc_type == F_UNLCK }
#[inline]
pub unsafe fn lock_is_read(fl: *mut file_lock) -> bool { (*fl).c.flc_type == F_RDLCK }
#[inline]
pub unsafe fn lock_is_write(fl: *mut file_lock) -> bool { (*fl).c.flc_type == F_WRLCK }

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn locks_inode_context(_: *const inode) -> *mut file_lock_context { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline]
pub unsafe fn locks_inode_context(_: *const inode) -> *mut file_lock_context { core::ptr::null_mut() }

#[inline]
pub unsafe fn locks_lock_file_wait(_: *mut file, _: *mut file_lock) -> i32 { 0 }

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub fn openmode_to_lease_flags(mode: u32) -> u32 {
    let mut flags = 0;
    if mode & 3 == 0 { flags |= LEASE_BREAK_OPEN_RDONLY; }
    if mode & 0x800 != 0 { flags |= LEASE_BREAK_NONBLOCK; }
    flags
}

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn break_lease(inode: *mut inode, mode: u32) -> i32 {
    let _ = (inode, mode);
    0
}

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn break_deleg(inode: *mut inode, flags: u32) -> i32 { let _ = (inode, flags); 0 }

#[repr(C)]
pub struct delegated_inode { pub di_inode: *mut inode }

#[inline]
pub unsafe fn is_delegated(di: *mut delegated_inode) -> bool { !(*di).di_inode.is_null() }

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn try_break_deleg(inode: *mut inode, flags: u32, di: *mut delegated_inode) -> i32 {
    let ret = break_deleg(inode, flags | LEASE_BREAK_NONBLOCK);
    if ret == -11 && !di.is_null() { (*di).di_inode = inode; }
    ret
}

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn break_deleg_wait(di: *mut delegated_inode) -> i32 { let ret = break_deleg((*di).di_inode, 0); (*di).di_inode = core::ptr::null_mut(); ret }

#[cfg(feature = "CONFIG_FILE_LOCKING")]
#[inline]
pub unsafe fn break_layout(inode: *mut inode, wait: bool) -> i32 { let _ = (inode, wait); 0 }

#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
pub struct delegated_inode;
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn is_delegated(_: *mut delegated_inode) -> bool { false }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn break_lease(_: *mut inode, _: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn break_deleg(_: *mut inode, _: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn try_break_deleg(_: *mut inode, _: u32, _: *mut delegated_inode) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn break_deleg_wait(_: *mut delegated_inode) -> i32 { panic!("BUG") }
#[cfg(not(feature = "CONFIG_FILE_LOCKING"))]
#[inline] pub unsafe fn break_layout(_: *mut inode, _: bool) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
