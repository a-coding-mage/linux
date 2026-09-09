/* SPDX-License-Identifier: GPL-2.0 */

/* Linux and Ceph dependencies are supplied by other translated units. */

#[repr(C)]
pub enum ceph_feature_type {
    CEPHFS_FEATURE_MIMIC = 8,
    CEPHFS_FEATURE_REPLY_ENCODING,
    CEPHFS_FEATURE_RECLAIM_CLIENT,
    CEPHFS_FEATURE_LAZY_CAP_WANTED,
    CEPHFS_FEATURE_MULTI_RECONNECT,
    CEPHFS_FEATURE_DELEG_INO,
    CEPHFS_FEATURE_METRIC_COLLECT,
    CEPHFS_FEATURE_ALTERNATE_NAME,
    CEPHFS_FEATURE_NOTIFY_SESSION_STATE,
    CEPHFS_FEATURE_OP_GETVXATTR,
    CEPHFS_FEATURE_32BITS_RETRY_FWD,
    CEPHFS_FEATURE_NEW_SNAPREALM_INFO,
    CEPHFS_FEATURE_HAS_OWNER_UIDGID,
    CEPHFS_FEATURE_MDS_AUTH_CAPS_CHECK,
    CEPHFS_FEATURE_SUBVOLUME_METRICS,
}

pub const CEPHFS_FEATURE_MAX: ceph_feature_type = ceph_feature_type::CEPHFS_FEATURE_SUBVOLUME_METRICS;
pub const CEPHFS_FEATURES_CLIENT_SUPPORTED: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];

pub const MDS_AUTH_UID_ANY: i32 = -1;
pub const CEPH_GET_CAPS_WAIT_TIMEOUT: u32 = 5 * HZ;
pub const CEPH_CAP_FLUSH_WAIT_TIMEOUT_SEC: u32 = 60;
pub const CEPH_CAP_FLUSH_MAX_DUMP_ENTRIES: u32 = 5;
pub const CEPH_CAP_FLUSH_MAX_DUMP_ITERS: u32 = 5;
pub const CEPH_CLIENT_RESET_REASON_LEN: usize = 64;
pub const CEPH_CLIENT_RESET_DRAIN_SEC: u32 = 30;
pub const CEPH_CLIENT_RESET_CLOSE_GRACE_MS: u32 = 100;
pub const CEPH_CLIENT_RESET_WAIT_TIMEOUT_SEC: u32 = 120;

#[repr(C)]
pub enum ceph_client_reset_phase { CEPH_CLIENT_RESET_IDLE = 0, CEPH_CLIENT_RESET_QUIESCING, CEPH_CLIENT_RESET_DRAINING, CEPH_CLIENT_RESET_TEARDOWN }

#[repr(C)]
pub struct ceph_client_reset_state {
    pub lock: spinlock_t, pub trigger_count: u64, pub success_count: u64, pub failure_count: u64,
    pub last_start: c_ulong, pub last_finish: c_ulong, pub last_errno: i32,
    pub phase: ceph_client_reset_phase, pub drain_timed_out: bool, pub shutdown: bool,
    pub sessions_reset: i32, pub last_reason: [c_char; CEPH_CLIENT_RESET_REASON_LEN],
    pub blocked_wq: wait_queue_head_t, pub blocked_requests: atomic_t,
}

#[inline] pub unsafe fn ceph_reset_is_idle(st: *const ceph_client_reset_state) -> bool { READ_ONCE((*st).phase) == ceph_client_reset_phase::CEPH_CLIENT_RESET_IDLE }

#[repr(C)] pub struct ceph_mds_cap_match { pub uid: i64, pub num_gids: u32, pub gids: *mut u32, pub path: *mut c_char, pub fs_name: *mut c_char, pub root_squash: bool }
#[repr(C)] pub struct ceph_mds_cap_auth { pub r#match: ceph_mds_cap_match, pub readable: bool, pub writeable: bool }
#[repr(C)] pub struct ceph_mds_reply_info_in { pub r#in: *mut ceph_mds_reply_inode, pub dir_layout: ceph_dir_layout, pub symlink_len: u32, pub symlink: *mut c_char, pub xattr_len: u32, pub xattr_data: *mut c_char, pub inline_version: u64, pub inline_len: u32, pub inline_data: *mut c_char, pub pool_ns_len: u32, pub pool_ns_data: *mut c_char, pub max_bytes: u64, pub max_files: u64, pub dir_pin: i32, pub btime: ceph_timespec, pub snap_btime: ceph_timespec, pub fscrypt_auth: *mut u8, pub fscrypt_file: *mut u8, pub fscrypt_auth_len: u32, pub fscrypt_file_len: u32, pub rsnaps: u64, pub change_attr: u64, pub subvolume_id: u64 }
#[repr(C)] pub struct ceph_mds_reply_dir_entry { pub is_nokey: bool, pub name: *mut c_char, pub name_len: u32, pub raw_hash: u32, pub lease: *mut ceph_mds_reply_lease, pub inode: ceph_mds_reply_info_in, pub offset: loff_t }
#[repr(C)] pub struct ceph_mds_reply_xattr { pub xattr_value: *mut c_char, pub xattr_value_len: size_t }

#[repr(C)] pub union ceph_mds_reply_info_parsed_extra { pub filelock_reply: *mut ceph_filelock, pub readdir: ceph_mds_reply_info_parsed_readdir, pub create: ceph_mds_reply_info_parsed_create }
#[repr(C)] pub struct ceph_mds_reply_info_parsed_readdir { pub dir_dir: *mut ceph_mds_reply_dirfrag, pub dir_buf_size: size_t, pub dir_nr: i32, pub dir_end: bool, pub dir_complete: bool, pub hash_order: bool, pub offset_hash: bool, pub dir_entries: *mut ceph_mds_reply_dir_entry }
#[repr(C)] pub struct ceph_mds_reply_info_parsed_create { pub has_create_ino: bool, pub ino: u64 }
#[repr(C)] pub struct ceph_mds_reply_info_parsed { pub head: *mut ceph_mds_reply_head, pub diri: ceph_mds_reply_info_in, pub targeti: ceph_mds_reply_info_in, pub dirfrag: *mut ceph_mds_reply_dirfrag, pub dname: *mut c_char, pub altname: *mut u8, pub dname_len: u32, pub altname_len: u32, pub dlease: *mut ceph_mds_reply_lease, pub xattr_info: ceph_mds_reply_xattr, pub extra: ceph_mds_reply_info_parsed_extra, pub snapblob: *mut c_void, pub snapblob_len: i32 }

pub const CEPH_MDS_SESSION_NEW: i32 = 1; pub const CEPH_MDS_SESSION_OPENING: i32 = 2; pub const CEPH_MDS_SESSION_OPEN: i32 = 3; pub const CEPH_MDS_SESSION_HUNG: i32 = 4; pub const CEPH_MDS_SESSION_RESTARTING: i32 = 5; pub const CEPH_MDS_SESSION_RECONNECTING: i32 = 6; pub const CEPH_MDS_SESSION_CLOSING: i32 = 7; pub const CEPH_MDS_SESSION_CLOSED: i32 = 8; pub const CEPH_MDS_SESSION_REJECTED: i32 = 9;
pub const USE_ANY_MDS: i32 = 0; pub const USE_RANDOM_MDS: i32 = 1; pub const USE_AUTH_MDS: i32 = 2;

#[repr(C)] pub struct ceph_mds_session { pub s_mdsc: *mut ceph_mds_client, pub s_mds: i32, pub s_state: i32, pub s_ttl: c_ulong, pub s_features: c_ulong, pub s_seq: u64, pub s_mutex: mutex, pub s_con: ceph_connection, pub s_auth: ceph_auth_handshake, pub s_cap_gen: atomic_t, pub s_cap_ttl: c_ulong, pub s_cap_lock: spinlock_t, pub s_ref: refcount_t, pub s_caps: list_head, pub s_cap_iterator: *mut ceph_cap, pub s_nr_caps: i32, pub s_num_cap_releases: i32, pub s_cap_reconnect: i32, pub s_readonly: i32, pub s_cap_releases: list_head, pub s_cap_release_work: work_struct, pub s_cap_dirty: list_head, pub s_cap_flushing: list_head, pub s_renew_requested: c_ulong, pub s_renew_seq: u64, pub s_waiting: list_head, pub s_unsafe: list_head, pub s_delegated_inos: xarray, pub s_num_deleg_inos: atomic_t }

pub type ceph_mds_request_callback_t = unsafe extern "C" fn(*mut ceph_mds_client, *mut ceph_mds_request);
pub type ceph_mds_request_wait_callback_t = unsafe extern "C" fn(*mut ceph_mds_client, *mut ceph_mds_request) -> i32;

#[repr(C)] pub struct ceph_mds_request { pub r_tid: u64, pub r_node: rb_node, pub r_mdsc: *mut ceph_mds_client, pub r_kref: kref, pub r_op: i32, pub r_inode: *mut inode, pub r_dentry: *mut dentry, pub r_old_dentry: *mut dentry, pub r_old_dentry_dir: *mut inode, pub r_path1: *mut c_char, pub r_path2: *mut c_char, pub r_ino1: ceph_vino, pub r_ino2: ceph_vino, pub r_parent: *mut inode, pub r_target_inode: *mut inode, pub r_new_inode: *mut inode, pub r_dname: *const qstr, pub r_req_flags: c_ulong, pub r_fill_mutex: mutex, pub r_args: ceph_mds_request_args, pub r_fscrypt_auth: *mut ceph_fscrypt_auth, pub r_fscrypt_file: u64, pub r_altname: *mut u8, pub r_altname_len: u32, pub r_fmode: i32, pub r_request_release_offset: i32, pub r_cred: *const cred, pub r_mnt_idmap: *mut mnt_idmap, pub r_stamp: timespec64, pub r_direct_mode: i32, pub r_direct_hash: u32, pub r_pagelist: *mut ceph_pagelist, pub r_inode_drop: i32, pub r_inode_unless: i32, pub r_dentry_drop: i32, pub r_dentry_unless: i32, pub r_old_dentry_drop: i32, pub r_old_dentry_unless: i32, pub r_old_inode: *mut inode, pub r_old_inode_drop: i32, pub r_old_inode_unless: i32, pub r_request: *mut ceph_msg, pub r_reply: *mut ceph_msg, pub r_reply_info: ceph_mds_reply_info_parsed, pub r_err: i32, pub r_readdir_offset: u32, pub r_locked_page: *mut page, pub r_dir_caps: i32, pub r_num_caps: i32, pub r_timeout: c_ulong, pub r_started: c_ulong, pub r_start_latency: c_ulong, pub r_end_latency: c_ulong, pub r_request_started: c_ulong, pub r_unsafe_dir: *mut inode, pub r_unsafe_dir_item: list_head, pub r_unsafe_target_item: list_head, pub r_session: *mut ceph_mds_session, pub r_attempts: i32, pub r_num_fwd: i32, pub r_resend_mds: i32, pub r_sent_on_mseq: u32, pub r_deleg_ino: u64, pub r_wait: list_head, pub r_completion: completion, pub r_safe_completion: completion, pub r_callback: Option<ceph_mds_request_callback_t>, pub r_unsafe_item: list_head, pub r_dir_release_cnt: i64, pub r_dir_ordered_cnt: i64, pub r_readdir_cache_idx: i32, pub r_feature_needed: i32, pub r_caps_reservation: ceph_cap_reservation }

#[repr(C)] pub struct ceph_pool_perm { pub node: rb_node, pub perm: i32, pub pool: i64, pub pool_ns_len: size_t, pub pool_ns: [c_char; 0] }
#[repr(C)] pub struct ceph_snapid_map { pub node: rb_node, pub lru: list_head, pub r#ref: atomic_t, pub dev: dev_t, pub snap: u64, pub last_used: c_ulong }
#[repr(C)] pub struct ceph_quotarealm_inode { pub node: rb_node, pub ino: u64, pub timeout: c_ulong, pub mutex: mutex, pub inode: *mut inode }

pub const CEPH_MDSC_STOPPING_BEGIN: i32 = 1;
pub const CEPH_MDSC_STOPPING_FLUSHING: i32 = 2;
pub const CEPH_MDSC_STOPPING_FLUSHED: i32 = 3;
#[repr(C)] pub struct ceph_mds_client {
    pub fsc: *mut ceph_fs_client, pub mutex: mutex, pub mdsmap: *mut ceph_mdsmap,
    pub safe_umount_waiters: completion, pub session_close_wq: wait_queue_head_t,
    pub waiting_for_map: list_head, pub mdsmap_err: i32, pub sessions: *mut *mut ceph_mds_session,
    pub num_sessions: atomic_t, pub max_sessions: i32, pub stopping_lock: spinlock_t, pub stopping: i32,
    pub stopping_blockers: atomic_t, pub stopping_waiter: completion, pub dirty_folios: atomic64_t,
    pub flush_end_wq: wait_queue_head_t, pub quotarealms_count: atomic64_t, pub quotarealms_inodes: rb_root,
    pub quotarealms_inodes_mutex: mutex, pub last_snap_seq: u64, pub snap_rwsem: rw_semaphore,
    pub snap_realms: rb_root, pub snap_empty: list_head, pub num_snap_realms: i32, pub snap_empty_lock: spinlock_t,
    pub last_tid: u64, pub oldest_tid: u64, pub request_tree: rb_root, pub delayed_work: delayed_work,
    pub last_renew_caps: c_ulong, pub cap_delay_list: list_head, pub cap_unlink_delay_list: list_head,
    pub cap_delay_lock: spinlock_t, pub snap_flush_list: list_head, pub snap_flush_lock: spinlock_t,
    pub last_cap_flush_tid: u64, pub cap_flush_list: list_head, pub cap_dirty_migrating: list_head,
    pub num_cap_flushing: i32, pub cap_dirty_lock: spinlock_t, pub cap_flushing_wq: wait_queue_head_t,
    pub cap_reclaim_work: work_struct, pub cap_reclaim_pending: atomic_t, pub cap_unlink_work: work_struct,
    pub caps_list_lock: spinlock_t, pub caps_list: list_head, pub caps_total_count: i32, pub caps_use_count: i32,
    pub caps_use_max: i32, pub caps_reserve_count: i32, pub caps_avail_count: i32, pub caps_min_count: i32,
    pub dentry_list_lock: spinlock_t, pub dentry_leases: list_head, pub dentry_dir_leases: list_head,
    pub metric: ceph_client_metric, pub reset_work: work_struct, pub reset_state: ceph_client_reset_state,
    pub subvol_metrics: ceph_subvolume_metrics_tracker, pub subvol_metrics_last_mutex: mutex,
    pub subvol_metrics_last: *mut ceph_subvol_metric_snapshot, pub subvol_metrics_last_nr: u32,
    pub subvol_metrics_sent: u64, pub subvol_metrics_nonzero_sends: u64, pub snapid_map_lock: spinlock_t,
    pub snapid_map_tree: rb_root, pub snapid_map_lru: list_head, pub pool_perm_rwsem: rw_semaphore,
    pub pool_perm_tree: rb_root, pub s_cap_auths_num: u32, pub s_cap_auths: *mut ceph_mds_cap_auth,
    pub nodename: [c_char; NEW_UTS_LEN + 1],
}

#[repr(C)] pub struct ceph_path_info { pub path: *const c_char, pub pathlen: i32, pub vino: ceph_vino, pub freepath: bool }

extern "C" {
    pub fn ceph_mds_op_name(op: i32) -> *const c_char;
    pub fn check_session_state(s: *mut ceph_mds_session) -> bool;
    pub fn inc_session_sequence(s: *mut ceph_mds_session);
    pub fn __ceph_lookup_mds_session(mdsc: *mut ceph_mds_client, mds: i32) -> *mut ceph_mds_session;
    pub fn ceph_session_state_name(s: i32) -> *const c_char;
    pub fn ceph_reset_phase_name(phase: ceph_client_reset_phase) -> *const c_char;
    pub fn ceph_get_mds_session(s: *mut ceph_mds_session) -> *mut ceph_mds_session;
    pub fn ceph_put_mds_session(s: *mut ceph_mds_session);
    pub fn ceph_mdsc_schedule_reset(mdsc: *mut ceph_mds_client, reason: *const c_char) -> i32;
    pub fn ceph_mdsc_wait_for_reset(mdsc: *mut ceph_mds_client) -> i32;
    pub fn ceph_mdsc_init(fsc: *mut ceph_fs_client) -> i32;
    pub fn ceph_mdsc_close_sessions(mdsc: *mut ceph_mds_client);
    pub fn ceph_mdsc_force_umount(mdsc: *mut ceph_mds_client);
    pub fn ceph_mdsc_destroy(fsc: *mut ceph_fs_client);
    pub fn ceph_mdsc_sync(mdsc: *mut ceph_mds_client);
    pub fn ceph_invalidate_dir_request(req: *mut ceph_mds_request);
    pub fn ceph_alloc_readdir_reply_buffer(req: *mut ceph_mds_request, dir: *mut inode) -> i32;
    pub fn ceph_mdsc_create_request(mdsc: *mut ceph_mds_client, op: i32, mode: i32) -> *mut ceph_mds_request;
    pub fn ceph_mdsc_submit_request(mdsc: *mut ceph_mds_client, dir: *mut inode, req: *mut ceph_mds_request) -> i32;
    pub fn ceph_mdsc_wait_request(mdsc: *mut ceph_mds_client, req: *mut ceph_mds_request, wait_func: ceph_mds_request_wait_callback_t) -> i32;
    pub fn ceph_mdsc_do_request(mdsc: *mut ceph_mds_client, dir: *mut inode, req: *mut ceph_mds_request) -> i32;
    pub fn ceph_mdsc_release_dir_caps(req: *mut ceph_mds_request);
    pub fn ceph_mdsc_release_dir_caps_async(req: *mut ceph_mds_request);
    pub fn ceph_mdsc_release_request(kref: *mut kref);
    pub fn send_flush_mdlog(s: *mut ceph_mds_session);
    pub fn ceph_mdsc_pre_umount(mdsc: *mut ceph_mds_client);
    pub fn ceph_mdsc_build_path(mdsc: *mut ceph_mds_client, dentry: *mut dentry, path_info: *mut ceph_path_info, for_wire: i32) -> *mut c_char;
    pub fn __ceph_mdsc_drop_dentry_lease(dentry: *mut dentry);
    pub fn ceph_mdsc_lease_send_msg(session: *mut ceph_mds_session, dentry: *mut dentry, action: c_char, seq: u32);
    pub fn ceph_mdsc_handle_mdsmap(mdsc: *mut ceph_mds_client, msg: *mut ceph_msg);
    pub fn ceph_mdsc_handle_fsmap(mdsc: *mut ceph_mds_client, msg: *mut ceph_msg);
    pub fn ceph_mdsc_open_export_target_session(mdsc: *mut ceph_mds_client, target: i32) -> *mut ceph_mds_session;
    pub fn ceph_trim_caps(mdsc: *mut ceph_mds_client, session: *mut ceph_mds_session, max_caps: i32) -> i32;
    pub fn ceph_wait_on_conflict_unlink(dentry: *mut dentry) -> i32;
    pub fn ceph_get_deleg_ino(session: *mut ceph_mds_session) -> u64;
    pub fn ceph_restore_deleg_ino(session: *mut ceph_mds_session, ino: u64) -> i32;
}

extern "C" { pub static mut enable_unsafe_idmap: bool; }

#[inline] pub unsafe fn ceph_mdsc_get_request(req: *mut ceph_mds_request) { kref_get(&mut (*req).r_kref); }
#[inline] pub unsafe fn ceph_mdsc_put_request(req: *mut ceph_mds_request) { kref_put(&mut (*req).r_kref, ceph_mdsc_release_request); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
