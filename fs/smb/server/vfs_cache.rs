// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful Rust-facing translation of the kernel VFS cache implementation.
// Kernel/project types and operations are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const S_DEL_PENDING: u32 = 1;
pub const S_DEL_ON_CLS: u32 = 2;
pub const S_DEL_ON_CLS_STREAM: u32 = 8;
pub const OPLOCK_NONE: i32 = 0;
pub const OPLOCK_EXCLUSIVE: i32 = 1;
pub const OPLOCK_BATCH: i32 = 2;
pub const OPLOCK_READ: i32 = 3;
pub const OPEN_ID_TYPE_VOLATILE_ID: i32 = 0;
pub const OPEN_ID_TYPE_PERSISTENT_ID: i32 = 1;

/* These opaque declarations correspond to the structures supplied by the
 * Linux kernel and ksmbd headers included by the original implementation. */
#[repr(C)] pub struct ksmbd_file { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_file_table { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_work { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_session { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_tree_connect { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_share_config { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_user { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }

/* External declarations retain the original externally visible interface. */
extern "C" {
    pub fn ksmbd_inode_lookup_lock(d: *mut dentry) -> *mut ksmbd_inode;
    pub fn ksmbd_query_inode_status(dentry: *mut dentry) -> i32;
    pub fn ksmbd_inode_put(ci: *mut ksmbd_inode);
    pub fn ksmbd_set_fd_limit(limit: usize);
    pub fn ksmbd_durable_scavenger_active() -> bool;
    pub fn ksmbd_open_fd(work: *mut ksmbd_work, filp: *mut file) -> *mut ksmbd_file;
    pub fn ksmbd_file_get(fp: *mut ksmbd_file) -> *mut ksmbd_file;
    pub fn ksmbd_fd_put(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    pub fn ksmbd_close_fd(work: *mut ksmbd_work, id: u64) -> i32;
    pub fn ksmbd_lookup_foreign_fd(work: *mut ksmbd_work, id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_fd_fast(work: *mut ksmbd_work, id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_fd_slow(work: *mut ksmbd_work, id: u64, pid: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_global_fd(id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_durable_fd(id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_put_durable_fd(fp: *mut ksmbd_file);
    pub fn ksmbd_lookup_fd_inode(dentry: *mut dentry) -> *mut ksmbd_file;
    pub fn ksmbd_open_durable_fd(fp: *mut ksmbd_file) -> u32;
    pub fn ksmbd_reopen_durable_fd(work: *mut ksmbd_work, fp: *mut ksmbd_file) -> i32;
    pub fn ksmbd_close_tree_conn_fds(work: *mut ksmbd_work);
    pub fn ksmbd_close_session_fds(work: *mut ksmbd_work);
    pub fn ksmbd_init_global_file_table() -> i32;
    pub fn ksmbd_free_global_file_table();
    pub fn ksmbd_init_file_table(ft: *mut ksmbd_file_table) -> i32;
    pub fn ksmbd_destroy_file_table(sess: *mut ksmbd_session);
    pub fn ksmbd_init_file_cache() -> i32;
    pub fn ksmbd_exit_file_cache();
}

#[repr(C)] pub struct ksmbd_inode { _private: [u8; 0] }

pub unsafe fn ksmbd_inode_pending_delete(_fp: *mut ksmbd_file) -> bool { unimplemented!() }
pub unsafe fn ksmbd_set_inode_pending_delete(_fp: *mut ksmbd_file) { unimplemented!() }
pub unsafe fn ksmbd_clear_inode_pending_delete(_fp: *mut ksmbd_file) { unimplemented!() }
pub unsafe fn ksmbd_has_stream_without_delete_share(_fp: *mut ksmbd_file) -> bool { unimplemented!() }
pub unsafe fn ksmbd_fd_set_delete_on_close(_fp: *mut ksmbd_file, _file_info: i32) { unimplemented!() }
pub unsafe fn ksmbd_fd_set_delete_pending(_fp: *mut ksmbd_file) { unimplemented!() }
pub unsafe fn ksmbd_fd_clear_delete_pending(_fp: *mut ksmbd_file) { unimplemented!() }
pub unsafe fn ksmbd_has_other_active_fd(_fp: *mut ksmbd_file) -> bool { unimplemented!() }
pub unsafe fn ksmbd_has_other_nonposix_open(_dentry: *mut dentry) -> bool { unimplemented!() }
pub unsafe fn ksmbd_has_nonposix_open_child(_fp: *mut ksmbd_file) -> bool { unimplemented!() }
pub unsafe fn ksmbd_close_disconnected_durable_delete_on_close(_dentry: *mut dentry) -> bool { unimplemented!() }
pub unsafe fn ksmbd_invalidate_durable_fd(_id: u64) -> i32 { unimplemented!() }
pub unsafe fn ksmbd_validate_name_reconnect(_share: *mut ksmbd_share_config, _fp: *mut ksmbd_file, _name: *mut i8) -> i32 { unimplemented!() }
pub unsafe fn ksmbd_vfs_set_durable_owner(_fp: *mut ksmbd_file, _user: *mut ksmbd_user) -> i32 { unimplemented!() }
pub unsafe fn ksmbd_vfs_compare_durable_owner(_fp: *mut ksmbd_file, _user: *mut ksmbd_user) -> bool { unimplemented!() }
pub unsafe fn ksmbd_launch_ksmbd_durable_scavenger() { unimplemented!() }
pub unsafe fn ksmbd_stop_durable_scavenger() { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
