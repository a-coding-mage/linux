/* SPDX-License-Identifier: GPL-2.0 */
// Translated from nfs_fs_sb.h. Included kernel types and declarations are
// supplied by other translation units.

pub enum nfs4_session {}
pub enum nfs_iostats {}
pub enum nlm_host {}
pub enum nfs4_sequence_args {}
pub enum nfs4_sequence_res {}
pub enum nfs4_minor_version_ops {}
pub enum nfs41_server_scope {}
pub enum nfs41_impl_id {}

pub const NFS_CS_READY: i32 = 0;
pub const NFS_CS_INITING: i32 = 1;
pub const NFS_CS_SESSION_INITING: i32 = 2;
pub const NFS_CS_CALLBACK: u32 = 1;
pub const NFS_CS_IDMAP: u32 = 2;
pub const NFS_CS_RENEWD: u32 = 3;
pub const NFS_CS_STOP_RENEW: u32 = 4;
pub const NFS_CS_CHECK_LEASE_TIME: u32 = 5;
pub const NFS_CS_NORESVPORT: u32 = 0;
pub const NFS_CS_DISCRTRY: u32 = 1;
pub const NFS_CS_MIGRATION: u32 = 2;
pub const NFS_CS_INFINITE_SLOTS: u32 = 3;
pub const NFS_CS_NO_RETRANS_TIMEOUT: u32 = 4;
pub const NFS_CS_TSM_POSSIBLE: u32 = 5;
pub const NFS_CS_NOPING: u32 = 6;
pub const NFS_CS_DS: u32 = 7;
pub const NFS_CS_REUSEPORT: u32 = 8;
pub const NFS_CS_PNFS: u32 = 9;
pub const NFS_CS_NETUNREACH_FATAL: u32 = 10;

#[repr(C)]
pub struct nfs_client {
    pub cl_count: refcount_t,
    pub cl_mds_count: atomic_t,
    pub cl_cons_state: i32,
    pub cl_res_state: c_ulong,
    pub cl_flags: c_ulong,
    pub cl_addr: sockaddr_storage,
    pub cl_addrlen: usize,
    pub cl_hostname: *mut c_char,
    pub cl_acceptor: *mut c_char,
    pub cl_share_link: list_head,
    pub cl_superblocks: list_head,
    pub cl_rpcclient: *mut rpc_clnt,
    pub rpc_ops: *const nfs_rpc_ops,
    pub cl_proto: i32,
    pub cl_nfs_mod: *mut nfs_subversion,
    pub cl_minorversion: u32,
    pub cl_nconnect: c_uint,
    pub cl_max_connect: c_uint,
    pub cl_principal: *const c_char,
    pub cl_xprtsec: xprtsec_parms,
    // IS_ENABLED(CONFIG_NFS_V4)
    pub cl_ds_clients: list_head,
    pub cl_clientid: u64,
    pub cl_confirm: nfs4_verifier,
    pub cl_state: c_ulong,
    pub cl_lock: spinlock_t,
    pub cl_lease_time: c_ulong,
    pub cl_last_renewal: c_ulong,
    pub cl_renewd: delayed_work,
    pub cl_rpcwaitq: rpc_wait_queue,
    pub cl_idmap: *mut idmap,
    pub cl_owner_id: *const c_char,
    pub cl_cb_ident: u32,
    pub cl_mvops: *const nfs4_minor_version_ops,
    pub cl_mig_gen: c_ulong,
    pub cl_slot_tbl: *mut nfs4_slot_table,
    pub cl_seqid: u32,
    pub cl_exchange_flags: u32,
    pub cl_session: *mut nfs4_session,
    pub cl_preserve_clid: bool,
    pub cl_serverowner: *mut nfs41_server_owner,
    pub cl_serverscope: *mut nfs41_server_scope,
    pub cl_implid: *mut nfs41_impl_id,
    pub cl_sp4_flags: c_ulong,
    pub cl_lock_waitq: wait_queue_head_t,
    pub cl_ipaddr: [c_char; 48],
    pub cl_net: *mut net,
    pub cl_ns_tracker: netns_tracker,
    pub pending_cb_stateids: list_head,
    pub rcu: rcu_head,
    // IS_ENABLED(CONFIG_NFS_LOCALIO)
    pub cl_nfssvc_boot: timespec64,
    pub cl_boot_lock: seqlock_t,
    pub cl_uuid: nfs_uuid_t,
    pub cl_local_probe_work: work_struct,
}

pub const NFS_SP4_MACH_CRED_MINIMAL: u32 = 1;
pub const NFS_SP4_MACH_CRED_CLEANUP: u32 = 2;
pub const NFS_SP4_MACH_CRED_SECINFO: u32 = 3;
pub const NFS_SP4_MACH_CRED_STATEID: u32 = 4;
pub const NFS_SP4_MACH_CRED_WRITE: u32 = 5;
pub const NFS_SP4_MACH_CRED_COMMIT: u32 = 6;
pub const NFS_SP4_MACH_CRED_PNFS_CLEANUP: u32 = 7;

#[repr(C)]
pub struct nfs_server {
    pub nfs_client: *mut nfs_client,
    pub client_link: list_head,
    pub master_link: list_head,
    pub client: *mut rpc_clnt,
    pub client_acl: *mut rpc_clnt,
    pub nlm_host: *mut nlm_host,
    pub io_stats: *mut nfs_iostats,
    pub write_congestion_wait: wait_queue_head_t,
    pub writeback: atomic_long_t,
    pub write_congested: c_uint,
    pub flags: c_uint,
    pub automount_inherit: c_uint,
    pub caps: c_uint,
    pub fattr_valid: u64,
    pub rsize: c_uint,
    pub rpages: c_uint,
    pub wsize: c_uint,
    pub wtmult: c_uint,
    pub dtsize: c_uint,
    pub port: c_ushort,
    pub bsize: c_uint,
    // CONFIG_NFS_V4_2 fields
    pub gxasize: c_uint,
    pub sxasize: c_uint,
    pub lxasize: c_uint,
    pub acregmin: c_uint,
    pub acregmax: c_uint,
    pub acdirmin: c_uint,
    pub acdirmax: c_uint,
    pub namelen: c_uint,
    pub options: c_uint,
    pub clone_blksize: c_uint,
    pub change_attr_type: nfs4_change_attr_type,
    pub fsid: nfs_fsid,
    pub s_sysfs_id: i32,
    pub maxfilesize: u64,
    pub mount_time: c_ulong,
    pub super_: *mut super_block,
    pub s_dev: dev_t,
    pub auth_info: nfs_auth_info,
    // CONFIG_NFS_FSCACHE fields
    pub fscache: *mut fscache_volume,
    pub fscache_uniq: *mut c_char,
    pub fh_expire_type: u32,
    pub pnfs_blksize: u32,
    // IS_ENABLED(CONFIG_NFS_V4)
    pub attr_bitmask: [u32; 3],
    pub attr_bitmask_nl: [u32; 3],
    pub exclcreat_bitmask: [u32; 3],
    pub cache_consistency_bitmask: [u32; 3],
    pub acl_bitmask: u32,
    pub pnfs_curr_ld: *mut pnfs_layoutdriver_type,
    pub roc_rpcwaitq: rpc_wait_queue,
    pub state_owners: rb_root,
    pub owner_ctr: atomic64_t,
    pub state_owners_lru: list_head,
    pub layouts: list_head,
    pub delegations: list_head,
    pub delegations_lock: spinlock_t,
    pub delegations_return: list_head,
    pub delegations_lru: list_head,
    pub delegations_delayed: list_head,
    pub nr_active_delegations: atomic_long_t,
    pub delegation_hash_mask: c_uint,
    pub delegation_hash_table: *mut hlist_head,
    pub ss_copies: list_head,
    pub ss_src_copies: list_head,
    pub delegation_flags: c_ulong,
    pub delegation_gen: c_ulong,
    pub mig_gen: c_ulong,
    pub mig_status: c_ulong,
    pub destroy: Option<unsafe extern "C" fn(*mut nfs_server)>,
    pub active: atomic_t,
    pub mountd_address: sockaddr_storage,
    pub mountd_addrlen: usize,
    pub mountd_version: u32,
    pub mountd_port: c_ushort,
    pub mountd_protocol: c_ushort,
    pub uoc_rpcwaitq: rpc_wait_queue,
    pub read_hdrsize: c_uint,
    pub cred: *const cred,
    pub has_sec_mnt_opts: bool,
    pub kobj: kobject,
    pub rcu: rcu_head,
}

pub const NFS_MOUNT_LOOKUP_CACHE_NONEG: u32 = 0x10000;
pub const NFS_MOUNT_LOOKUP_CACHE_NONE: u32 = 0x20000;
pub const NFS_MOUNT_NORESVPORT: u32 = 0x40000;
pub const NFS_MOUNT_LEGACY_INTERFACE: u32 = 0x80000;
pub const NFS_MOUNT_LOCAL_FLOCK: u32 = 0x100000;
pub const NFS_MOUNT_LOCAL_FCNTL: u32 = 0x200000;
pub const NFS_MOUNT_SOFTERR: u32 = 0x400000;
pub const NFS_MOUNT_SOFTREVAL: u32 = 0x800000;
pub const NFS_MOUNT_WRITE_EAGER: u32 = 0x01000000;
pub const NFS_MOUNT_WRITE_WAIT: u32 = 0x02000000;
pub const NFS_MOUNT_TRUNK_DISCOVERY: u32 = 0x04000000;
pub const NFS_MOUNT_SHUTDOWN: u32 = 0x08000000;
pub const NFS_MOUNT_NO_ALIGNWRITE: u32 = 0x10000000;
pub const NFS_MOUNT_FORCE_RDIRPLUS: u32 = 0x20000000;
pub const NFS_MOUNT_NETUNREACH_FATAL: u32 = 0x40000000;
pub const NFS_AUTOMOUNT_INHERIT_BSIZE: u32 = 1;
pub const NFS_AUTOMOUNT_INHERIT_RSIZE: u32 = 2;
pub const NFS_AUTOMOUNT_INHERIT_WSIZE: u32 = 4;
pub const NFS_OPTION_FSCACHE: u32 = 1;
pub const NFS_OPTION_MIGRATION: u32 = 2;
pub const NFS_FH_NOEXPIRE_WITH_OPEN: u32 = 1;
pub const NFS_FH_VOLATILE_ANY: u32 = 2;
pub const NFS_FH_VOL_MIGRATION: u32 = 4;
pub const NFS_FH_VOL_RENAME: u32 = 8;
pub const NFS_FH_RENAME_UNSAFE: u32 = NFS_FH_VOLATILE_ANY | NFS_FH_VOL_RENAME;
pub const NFS4SERV_DELEGATION_EXPIRED: u32 = 1;
pub const NFS_MIG_IN_TRANSITION: u32 = 1;
pub const NFS_MIG_FAILED: u32 = 2;
pub const NFS_MIG_TSM_POSSIBLE: u32 = 3;

pub const NFS_CAP_READDIRPLUS: u32 = 1 << 0;
pub const NFS_CAP_HARDLINKS: u32 = 1 << 1;
pub const NFS_CAP_SYMLINKS: u32 = 1 << 2;
pub const NFS_CAP_ACLS: u32 = 1 << 3;
pub const NFS_CAP_ATOMIC_OPEN: u32 = 1 << 4;
pub const NFS_CAP_LGOPEN: u32 = 1 << 5;
pub const NFS_CAP_CASE_INSENSITIVE: u32 = 1 << 6;
pub const NFS_CAP_CASE_NONPRESERVING: u32 = 1 << 7;
pub const NFS_CAP_REBOOT_LAYOUTRETURN: u32 = 1 << 8;
pub const NFS_CAP_OFFLOAD_STATUS: u32 = 1 << 9;
pub const NFS_CAP_ZERO_RANGE: u32 = 1 << 10;
pub const NFS_CAP_DIR_DELEG: u32 = 1 << 11;
pub const NFS_CAP_OPEN_XOR: u32 = 1 << 12;
pub const NFS_CAP_DELEGTIME: u32 = 1 << 13;
pub const NFS_CAP_POSIX_LOCK: u32 = 1 << 14;
pub const NFS_CAP_UIDGID_NOMAP: u32 = 1 << 15;
pub const NFS_CAP_STATEID_NFSV41: u32 = 1 << 16;
pub const NFS_CAP_ATOMIC_OPEN_V1: u32 = 1 << 17;
pub const NFS_CAP_SECURITY_LABEL: u32 = 1 << 18;
pub const NFS_CAP_SEEK: u32 = 1 << 19;
pub const NFS_CAP_ALLOCATE: u32 = 1 << 20;
pub const NFS_CAP_DEALLOCATE: u32 = 1 << 21;
pub const NFS_CAP_LAYOUTSTATS: u32 = 1 << 22;
pub const NFS_CAP_CLONE: u32 = 1 << 23;
pub const NFS_CAP_COPY: u32 = 1 << 24;
pub const NFS_CAP_OFFLOAD_CANCEL: u32 = 1 << 25;
pub const NFS_CAP_LAYOUTERROR: u32 = 1 << 26;
pub const NFS_CAP_COPY_NOTIFY: u32 = 1 << 27;
pub const NFS_CAP_XATTR: u32 = 1 << 28;
pub const NFS_CAP_READ_PLUS: u32 = 1 << 29;
pub const NFS_CAP_FS_LOCATIONS: u32 = 1 << 30;
pub const NFS_CAP_MOVEABLE: u32 = 1 << 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
