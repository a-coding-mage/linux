/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust translation of ceph/super.h. Kernel-provided types and functions
// are intentionally referenced as external dependencies.

pub const CEPH_BLOCK_SHIFT: u32 = 22;
pub const CEPH_BLOCK: u32 = 1 << CEPH_BLOCK_SHIFT;
pub const CEPH_4K_BLOCK_SHIFT: u32 = 12;
pub const CEPH_MOUNT_OPT_CLEANRECOVER: u32 = 1 << 1;
pub const CEPH_MOUNT_OPT_DIRSTAT: u32 = 1 << 4;
pub const CEPH_MOUNT_OPT_RBYTES: u32 = 1 << 5;
pub const CEPH_MOUNT_OPT_NOASYNCREADDIR: u32 = 1 << 7;
pub const CEPH_MOUNT_OPT_INO32: u32 = 1 << 8;
pub const CEPH_MOUNT_OPT_DCACHE: u32 = 1 << 9;
pub const CEPH_MOUNT_OPT_FSCACHE: u32 = 1 << 10;
pub const CEPH_MOUNT_OPT_NOPOOLPERM: u32 = 1 << 11;
pub const CEPH_MOUNT_OPT_MOUNTWAIT: u32 = 1 << 12;
pub const CEPH_MOUNT_OPT_NOQUOTADF: u32 = 1 << 13;
pub const CEPH_MOUNT_OPT_NOCOPYFROM: u32 = 1 << 14;
pub const CEPH_MOUNT_OPT_ASYNC_DIROPS: u32 = 1 << 15;
pub const CEPH_MOUNT_OPT_NOPAGECACHE: u32 = 1 << 16;
pub const CEPH_MOUNT_OPT_SPARSEREAD: u32 = 1 << 17;
pub const CEPH_MOUNT_OPT_NEARFULL_SYNC: u32 = 1 << 18;
pub const CEPH_MOUNT_OPT_DEFAULT: u32 = CEPH_MOUNT_OPT_DCACHE | CEPH_MOUNT_OPT_NOCOPYFROM | CEPH_MOUNT_OPT_ASYNC_DIROPS;
pub const CEPH_RASIZE_DEFAULT: u32 = 8192 * 1024;
pub const CEPH_MAX_READDIR_DEFAULT: u32 = 1024;
pub const CEPH_MAX_READDIR_BYTES_DEFAULT: u32 = 512 * 1024;
pub const CEPH_CAPS_WANTED_DELAY_MIN_DEFAULT: u32 = 5;
pub const CEPH_CAPS_WANTED_DELAY_MAX_DEFAULT: u32 = 60;
pub const CEPH_NAMESPACE_WILDCARD: &[u8] = b"*\0";

#[repr(C)] pub struct ceph_mount_options {
    pub flags: u32, pub wsize: u32, pub rsize: u32, pub rasize: u32,
    pub congestion_kb: u32, pub caps_wanted_delay_min: u32, pub caps_wanted_delay_max: u32,
    pub caps_max: i32, pub max_readdir: u32, pub max_readdir_bytes: u32,
    pub new_dev_syntax: bool, pub snapdir_name: *mut i8, pub mds_namespace: *mut i8,
    pub server_path: *mut i8, pub fscache_uniq: *mut i8, pub mon_addr: *mut i8,
    pub dummy_enc_policy: fscrypt_dummy_policy,
}
#[repr(C)] pub struct ceph_fs_client {
    pub sb: *mut super_block, pub metric_wakeup: list_head, pub mount_options: *mut ceph_mount_options,
    pub client: *mut ceph_client, pub mount_state: i32, pub blocklisted: bool, pub have_copy_from2: bool,
    pub filp_gen: u32, pub max_file_size: i64, pub mdsc: *mut ceph_mds_client,
    pub writeback_count: atomic_long_t, pub write_congested: bool,
    pub inode_wq: *mut workqueue_struct, pub cap_wq: *mut workqueue_struct,
    pub async_unlink_conflict: [u8; 0], pub async_unlink_conflict_lock: spinlock_t,
}
#[repr(C)] pub struct ceph_cap { pub ci: *mut ceph_inode_info, pub ci_node: rb_node, pub session: *mut ceph_mds_session, pub session_caps: list_head, pub cap_id: u64, pub issued: i32, pub implemented: i32, pub mds: i32, pub mds_wanted: i32, pub seq: u32, pub issue_seq: u32, pub mseq: u32, pub cap_gen: u32, pub last_used: usize, pub caps_item: list_head }
#[repr(C)] pub struct ceph_cap_flush { pub tid: u64, pub caps: i32, pub wake: bool, pub is_capsnap: bool, pub g_list: list_head, pub i_list: list_head, pub ci: *mut ceph_inode_info }
#[repr(C)] pub struct ceph_cap_snap { pub nref: refcount_t, pub ci_item: list_head, pub cap_flush: ceph_cap_flush, pub follows: u64, pub issued: i32, pub dirty: i32, pub context: *mut ceph_snap_context, pub mode: umode_t, pub uid: kuid_t, pub gid: kgid_t, pub xattr_blob: *mut ceph_buffer, pub xattr_version: u64, pub size: u64, pub change_attr: u64, pub mtime: timespec64, pub atime: timespec64, pub ctime: timespec64, pub btime: timespec64, pub time_warp_seq: u64, pub truncate_size: u64, pub truncate_seq: u32, pub writing: i32, pub dirty_pages: i32, pub inline_data: bool, pub need_flush: bool }
#[repr(C)] pub struct ceph_inode_frag { pub node: rb_node, pub frag: u32, pub split_by: i32, pub mds: i32, pub ndist: i32, pub dist: [i32; 4] }
#[repr(C)] pub struct ceph_inode_xattr { pub node: rb_node, pub name: *const i8, pub name_len: i32, pub val: *const i8, pub val_len: i32, pub dirty: i32, pub should_free_name: i32, pub should_free_val: i32 }
#[repr(C)] pub struct ceph_dentry_info { pub dentry: *mut dentry, pub lease_session: *mut ceph_mds_session, pub lease_list: list_head, pub hnode: hlist_node, pub flags: usize, pub lease_shared_gen: i32, pub lease_gen: u32, pub lease_seq: u32, pub lease_renew_after: usize, pub lease_renew_from: usize, pub time: usize, pub offset: u64 }
#[repr(C)] pub struct ceph_netfs_request_data { pub caps: i32, pub file_ra_pages: u32, pub file_ra_disabled: bool }
#[repr(C)] pub struct ceph_file_info { pub fmode: i16, pub flags: i16, pub rw_contexts_lock: spinlock_t, pub rw_contexts: list_head, pub filp_gen: u32 }
#[repr(C)] pub struct ceph_dir_file_info { pub file_info: ceph_file_info, pub frag: u32, pub last_readdir: *mut ceph_mds_request, pub next_offset: u32, pub last_name: *mut i8, pub dir_release_count: i64, pub dir_ordered_count: i64, pub readdir_cache_idx: i32, pub dir_info: *mut i8, pub dir_info_len: i32 }
#[repr(C)] pub struct ceph_rw_context { pub list: list_head, pub thread: *mut task_struct, pub caps: i32 }
#[repr(C)] pub struct ceph_readdir_cache_control { pub folio: *mut folio, pub dentries: *mut *mut dentry, pub index: i32 }
#[repr(C)] pub struct ceph_snap_realm { pub ino: u64, pub inode: *mut inode, pub nref: atomic_t, pub node: rb_node, pub created: u64, pub seq: u64, pub parent_ino: u64, pub parent_since: u64, pub prior_parent_snaps: *mut u64, pub num_prior_parent_snaps: u32, pub snaps: *mut u64, pub num_snaps: u32, pub parent: *mut ceph_snap_realm, pub children: list_head, pub child_item: list_head, pub empty_item: list_head, pub dirty_item: list_head, pub rebuild_item: list_head, pub cached_context: *mut ceph_snap_context, pub inodes_with_caps: list_head, pub inodes_with_caps_lock: spinlock_t }

pub const CEPH_MOUNT_MOUNTING: i32 = 0; pub const CEPH_MOUNT_MOUNTED: i32 = 1; pub const CEPH_MOUNT_UNMOUNTING: i32 = 2; pub const CEPH_MOUNT_UNMOUNTED: i32 = 3; pub const CEPH_MOUNT_SHUTDOWN: i32 = 4; pub const CEPH_MOUNT_RECOVER: i32 = 5; pub const CEPH_MOUNT_FENCE_IO: i32 = 6;
pub const CEPH_I_DIR_ORDERED_BIT: u32 = 0; pub const CEPH_I_FLUSH_BIT: u32 = 2; pub const CEPH_I_POOL_PERM_BIT: u32 = 3; pub const CEPH_I_POOL_RD_BIT: u32 = 4; pub const CEPH_I_POOL_WR_BIT: u32 = 5; pub const CEPH_I_SEC_INITED_BIT: u32 = 6; pub const CEPH_I_KICK_FLUSH_BIT: u32 = 7; pub const CEPH_I_FLUSH_SNAPS_BIT: u32 = 8; pub const CEPH_I_ODIRECT_BIT: u32 = 11; pub const CEPH_I_ASYNC_CREATE_BIT: u32 = 12; pub const CEPH_I_SHUTDOWN_BIT: u32 = 13; pub const CEPH_I_FLUSH_FORCE_BIT: u32 = 15;
pub const CEPH_I_DIR_ORDERED: usize = 1 << 0; pub const CEPH_I_FLUSH: usize = 1 << 2; pub const CEPH_I_POOL_PERM: usize = 1 << 3; pub const CEPH_I_POOL_RD: usize = 1 << 4; pub const CEPH_I_POOL_WR: usize = 1 << 5; pub const CEPH_I_SEC_INITED: usize = 1 << 6; pub const CEPH_I_KICK_FLUSH: usize = 1 << 7; pub const CEPH_I_FLUSH_SNAPS: usize = 1 << 8; pub const CEPH_I_ODIRECT: usize = 1 << 11; pub const CEPH_I_ASYNC_CREATE: usize = 1 << 12; pub const CEPH_I_SHUTDOWN: usize = 1 << 13; pub const CEPH_I_FLUSH_FORCE: usize = 1 << 15;
pub const CEPH_I_WORK_WRITEBACK: i32 = 0; pub const CEPH_I_WORK_INVALIDATE_PAGES: i32 = 1; pub const CEPH_I_WORK_VMTRUNCATE: i32 = 2; pub const CEPH_I_WORK_CHECK_CAPS: i32 = 3; pub const CEPH_I_WORK_FLUSH_SNAPS: i32 = 4;
pub const CEPH_F_SYNC: i16 = 1; pub const CEPH_F_ATEND: i16 = 2; pub const CEPH_MAX_MDS: u64 = 0x100; pub const CEPH_NUM_STRAY: u64 = 10; pub const CEPH_MDS_INO_MDSDIR_OFFSET: u64 = CEPH_MAX_MDS; pub const CEPH_MDS_INO_LOG_OFFSET: u64 = 2 * CEPH_MAX_MDS; pub const CEPH_INO_SYSTEM_BASE: u64 = 6 * CEPH_MAX_MDS + CEPH_MAX_MDS * CEPH_NUM_STRAY; pub const CEPH_MAX_DELEG_INOS: u32 = 8192;

pub const QUOTA_GET_MAX_FILES: i32 = 0; pub const QUOTA_GET_MAX_BYTES: i32 = 1; pub const QUOTA_GET_ANY: i32 = 2;
extern "C" { pub fn ceph_force_reconnect(sb: *mut super_block) -> i32; pub fn ceph_queue_inode_work(inode: *mut inode, bit: i32); }

// External kernel/Ceph types referenced by this header.
#[allow(non_camel_case_types)] pub type umode_t = u32; pub type kuid_t = u32; pub type kgid_t = u32;
#[repr(C)] pub struct super_block; #[repr(C)] pub struct inode; #[repr(C)] pub struct dentry; #[repr(C)] pub struct ceph_client; #[repr(C)] pub struct ceph_mds_client; #[repr(C)] pub struct ceph_mds_session; #[repr(C)] pub struct ceph_mds_request; #[repr(C)] pub struct ceph_snap_context; #[repr(C)] pub struct ceph_buffer; #[repr(C)] pub struct task_struct; #[repr(C)] pub struct folio; #[repr(C)] pub struct fscrypt_dummy_policy; #[repr(C)] pub struct list_head; #[repr(C)] pub struct rb_node; #[repr(C)] pub struct hlist_node; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct atomic_t; #[repr(C)] pub struct atomic_long_t; #[repr(C)] pub struct refcount_t; #[repr(C)] pub struct timespec64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
