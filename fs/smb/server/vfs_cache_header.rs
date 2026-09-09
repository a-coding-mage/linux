/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2019 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const FILE_GENERIC_ALL: u32 = 0x1F01FF;
pub const FILE_GENERIC_READ: u32 = 0x120089;
pub const FILE_GENERIC_WRITE: u32 = 0x120116;
pub const FILE_GENERIC_EXECUTE: u32 = 0x1200a0;

pub const KSMBD_START_FID: i32 = 1;
pub const KSMBD_NO_FID: i32 = i32::MAX;
pub const SMB2_NO_FID: u64 = 0xFFFF_FFFF_FFFF_FFFF;

#[repr(C)] pub struct ksmbd_conn { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_session { _private: [u8; 0] }
#[repr(C)] pub struct file_lock { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct oplock_info { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_tree_connect { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_readdir_data { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_work { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_user { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_share_config { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { pub actor: Option<unsafe extern "C" fn()>, }
pub type filldir_t = unsafe extern "C" fn();
pub type ssize_t = isize;
pub type loff_t = i64;
pub type __le16 = u16;
pub type __le32 = u32;
pub type __u64 = u64;
pub type u8 = std::os::raw::c_uchar;

#[repr(C)] pub struct ksmbd_lock {
    pub fl: *mut file_lock, pub conn: *mut ksmbd_conn,
    pub clist: list_head, pub flist: list_head, pub llist: list_head,
    pub flags: u32, pub cmd: i32, pub zero_len: i32,
    pub start: u64, pub end: u64,
}
#[repr(C)] pub struct stream { pub name: *mut i8, pub size: ssize_t, pub pos: loff_t }
#[repr(C)] pub struct ksmbd_inode {
    pub m_lock: rw_semaphore, pub m_count: atomic_t, pub op_count: atomic_t,
    pub sop_count: atomic_t, pub m_de: *mut dentry, pub m_flags: u32,
    pub m_hash: hlist_node, pub m_fp_list: list_head, pub m_op_list: list_head,
    pub m_opinfo: *mut oplock_info, pub m_fattr: __le32,
}
pub const FP_NEW: i32 = 0;
pub const FP_INITED: i32 = 1;
pub const FP_CLOSED: i32 = 2;
#[repr(C)] pub struct durable_owner { pub uid: u32, pub gid: u32, pub name: *mut i8 }
pub const KSMBD_LOCK_SEQ_ARRAY_SIZE: usize = 64;
#[repr(C)] pub struct ksmbd_lock_sequence { pub valid: bool, pub sequence: u8 }

#[repr(C)] pub struct ksmbd_file {
    pub filp: *mut file, pub persistent_id: u64, pub volatile_id: u64,
    pub durable_volatile_id: u64, pub f_lock: spinlock_t,
    pub f_ci: *mut ksmbd_inode, pub f_parent_ci: *mut ksmbd_inode,
    pub f_opinfo: *mut oplock_info, pub conn: *mut ksmbd_conn,
    pub tcon: *mut ksmbd_tree_connect, pub refcount: atomic_t,
    pub daccess: __le32, pub saccess: __le32, pub coption: __le32,
    pub cdoption: __le32, pub create_file_attributes: __le32,
    pub create_time: __u64, pub change_time: __u64, pub allocation_size: __u64,
    pub itime: __u64, pub open_mtime: __u64, pub is_nt_open: bool,
    pub attrib_only: bool, pub allocation_size_set: bool,
    pub client_guid: [i8; 16], pub create_guid: [i8; 16], pub app_instance_id: [i8; 16],
    pub stream: stream, pub node: list_head, pub blocked_works: list_head,
    pub lock_list: list_head, pub stream_del_pending: bool,
    pub durable_timeout: u32, pub durable_scavenger_timeout: u32,
    pub create_action: __le32, pub readdir_data: ksmbd_readdir_data,
    pub readdir_lock: mutex, pub dot_dotdot: [i32; 2], pub f_state: u32,
    pub reserve_lease_break: bool, pub is_durable: bool, pub is_persistent: bool,
    pub is_resilient: bool, pub has_app_instance_id: bool,
    pub app_instance_version_valid: bool, pub app_instance_version_high: u64,
    pub app_instance_version_low: u64, pub durable_reconnect_disabled: bool,
    pub durable_replay_consumed: bool, pub is_posix_ctxt: bool,
    pub owner: durable_owner, pub channel_sequence: __le16,
    pub outstanding_requests: u32, pub outstanding_pre_requests: u32,
    pub lock_seq: [ksmbd_lock_sequence; KSMBD_LOCK_SEQ_ARRAY_SIZE],
    pub notify_pendings: list_head,
}

pub unsafe fn set_ctx_actor(ctx: *mut dir_context, actor: filldir_t) { (*ctx).actor = Some(actor); }
pub const KSMBD_NR_OPEN_DEFAULT: usize = usize::BITS as usize;
#[repr(C)] pub struct ksmbd_file_table { pub lock: rwlock_t, pub idr: *mut idr }
pub unsafe fn has_file_id(id: u64) -> bool { id < KSMBD_NO_FID as u64 }
pub unsafe fn ksmbd_stream_fd(fp: *mut ksmbd_file) -> bool { !(*fp).stream.name.is_null() }

extern "C" {
    pub fn ksmbd_init_file_table(ft: *mut ksmbd_file_table) -> i32;
    pub fn ksmbd_destroy_file_table(sess: *mut ksmbd_session);
    pub fn ksmbd_close_fd(work: *mut ksmbd_work, id: u64) -> i32;
    pub fn ksmbd_lookup_fd_fast(work: *mut ksmbd_work, id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_foreign_fd(work: *mut ksmbd_work, id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_fd_slow(work: *mut ksmbd_work, id: u64, pid: u64) -> *mut ksmbd_file;
    pub fn ksmbd_vfs_set_durable_owner(fp: *mut ksmbd_file, user: *mut ksmbd_user) -> i32;
    pub fn ksmbd_file_get(fp: *mut ksmbd_file) -> *mut ksmbd_file;
    pub fn ksmbd_fd_put(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    pub fn ksmbd_inode_lookup_lock(d: *mut dentry) -> *mut ksmbd_inode;
    pub fn ksmbd_inode_put(ci: *mut ksmbd_inode);
    pub fn ksmbd_close_disconnected_durable_delete_on_close(dentry: *mut dentry) -> bool;
    pub fn ksmbd_lookup_global_fd(id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_durable_fd(id: u64) -> *mut ksmbd_file;
    pub fn ksmbd_put_durable_fd(fp: *mut ksmbd_file);
    pub fn ksmbd_invalidate_durable_fd(id: u64) -> i32;
    pub fn ksmbd_has_other_active_fd(fp: *mut ksmbd_file) -> bool;
    pub fn ksmbd_has_stream_without_delete_share(fp: *mut ksmbd_file) -> bool;
    pub fn ksmbd_lookup_fd_app_instance_id(id: *mut i8) -> *mut ksmbd_file;
    pub fn ksmbd_close_fd_app_instance_id(id: *mut i8) -> i32;
    pub fn ksmbd_lookup_fd_cguid(id: *mut i8) -> *mut ksmbd_file;
    pub fn ksmbd_lookup_fd_inode(dentry: *mut dentry) -> *mut ksmbd_file;
    pub fn ksmbd_has_other_nonposix_open(dentry: *mut dentry) -> bool;
    pub fn ksmbd_has_nonposix_open_child(old_fp: *mut ksmbd_file) -> bool;
    pub fn ksmbd_open_durable_fd(fp: *mut ksmbd_file) -> u32;
    pub fn ksmbd_open_fd(work: *mut ksmbd_work, filp: *mut file) -> *mut ksmbd_file;
    pub fn ksmbd_launch_ksmbd_durable_scavenger(); pub fn ksmbd_stop_durable_scavenger();
    pub fn ksmbd_durable_scavenger_active() -> bool;
    pub fn ksmbd_close_tree_conn_fds(work: *mut ksmbd_work); pub fn ksmbd_close_session_fds(work: *mut ksmbd_work);
    pub fn ksmbd_close_inode_fds(work: *mut ksmbd_work, inode: *mut inode) -> i32;
    pub fn ksmbd_init_global_file_table() -> i32; pub fn ksmbd_free_global_file_table();
    pub fn ksmbd_set_fd_limit(limit: usize);
    pub fn ksmbd_update_fstate(ft: *mut ksmbd_file_table, fp: *mut ksmbd_file, state: u32) -> i32;
    pub fn ksmbd_vfs_compare_durable_owner(fp: *mut ksmbd_file, user: *mut ksmbd_user) -> bool;
    pub fn ksmbd_inode_hash_init() -> i32; pub fn ksmbd_release_inode_hash();
}

#[repr(C)] pub enum KSMBD_INODE_STATUS { KSMBD_INODE_STATUS_OK, KSMBD_INODE_STATUS_UNKNOWN, KSMBD_INODE_STATUS_PENDING_DELETE }
extern "C" {
    pub fn ksmbd_query_inode_status(dentry: *mut dentry) -> i32;
    pub fn ksmbd_inode_pending_delete(fp: *mut ksmbd_file) -> bool;
    pub fn ksmbd_set_inode_pending_delete(fp: *mut ksmbd_file);
    pub fn ksmbd_clear_inode_pending_delete(fp: *mut ksmbd_file);
    pub fn ksmbd_fd_set_delete_on_close(fp: *mut ksmbd_file, file_info: i32);
    pub fn ksmbd_fd_set_delete_pending(fp: *mut ksmbd_file);
    pub fn ksmbd_fd_clear_delete_pending(fp: *mut ksmbd_file);
    pub fn ksmbd_reopen_durable_fd(work: *mut ksmbd_work, fp: *mut ksmbd_file) -> i32;
    pub fn ksmbd_validate_name_reconnect(share: *mut ksmbd_share_config, fp: *mut ksmbd_file, name: *mut i8) -> i32;
    pub fn ksmbd_init_file_cache() -> i32; pub fn ksmbd_exit_file_cache();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
