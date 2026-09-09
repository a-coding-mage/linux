/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of nfsd.h. */

/* C includes are dependencies supplied by other translation units. */

#[cfg(feature = "config_sunrpc_debug")]
macro_rules! ifdebug { ($flag:ident) => { if nfsd_debug & NFSDDBG_$flag != 0 }; }
#[cfg(not(feature = "config_sunrpc_debug"))]
macro_rules! ifdebug { ($flag:ident) => { if false }; }

pub const NFSD_MINVERS: i32 = 2;
pub const NFSD_MAXVERS: i32 = 4;
pub const NFSD_SUPPORTED_MINOR_VERSION: i32 = 2;
extern "C" { pub fn nfsd_support_version(vers: i32) -> bool; }

pub const NFSSVC_DEFBLKSIZE: usize = 4 * 1024 * 1024;
/* NFSSVC_MAXBLKSIZE = RPCSVC_MAXPAYLOAD (dependency-provided constant). */
pub const NFSD_MAX_OPS_PER_COMPOUND: i32 = 200;

extern "C" {
    pub static mut nfsd_programs: [svc_program; 0];
    pub static nfsd_version2: svc_version;
    pub static nfsd_version3: svc_version;
    pub static nfsd_version4: svc_version;
    pub static mut nfsd_mutex: mutex;
    pub static mut nfsd_th_cnt: atomic_t;
    pub static nfs_exports_op: seq_operations;
}

#[repr(C)] pub struct nfsd_thread_local_info { pub ntli_lease_breaker: *mut *mut nfs4_client, pub ntli_cachetype: i32 }
#[repr(C)] pub struct nfsd_voidargs {}
#[repr(C)] pub struct nfsd_voidres {}
extern "C" {
    pub fn nfssvc_decode_voidarg(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfssvc_encode_voidres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
    pub fn nfsd_svc(n: i32, nservers: *mut i32, net: *mut net, cred: *const cred, scope: *const i8) -> i32;
    pub fn nfsd_dispatch(rqstp: *mut svc_rqst) -> i32;
    pub fn nfsd_nrthreads(net: *mut net) -> i32; pub fn nfsd_nrpools(net: *mut net) -> i32;
    pub fn nfsd_get_nrthreads(n: i32, p: *mut i32, net: *mut net) -> i32;
    pub fn nfsd_set_nrthreads(n: i32, p: *mut i32, net: *mut net) -> i32;
    pub fn nfsd_shutdown_threads(net: *mut net); pub fn nfsd_current_rqst() -> *mut svc_rqst;
}
#[repr(C)] pub struct nfsdfs_client { pub cl_ref: kref, pub cl_release: Option<unsafe extern "C" fn(*mut kref)> }
pub enum nfsd_net {}
extern "C" {
    pub fn get_nfsdfs_client(i: *mut inode) -> *mut nfsdfs_client;
    pub fn nfsd_client_mkdir(nn: *mut nfsd_net, ncl: *mut nfsdfs_client, id: u32, td: *const tree_descr, fd: *mut *mut dentry) -> *mut dentry;
    pub fn nfsd_client_rmdir(dentry: *mut dentry); pub fn nfsd_cache_notify(cd: *mut cache_detail, h: *mut cache_head, cache_type: u32) -> i32;
}
#[cfg(any(feature="config_nfsd_v2_acl", feature="config_nfsd_v3_acl"))]
extern "C" { pub static nfsd_acl_version2: svc_version; pub static nfsd_acl_version3: svc_version; }
#[cfg(feature="config_nfs_localio")] extern "C" { pub static localio_version1: svc_version; }

#[repr(C)] pub enum vers_op { NFSD_SET, NFSD_CLEAR, NFSD_TEST, NFSD_AVAIL }
extern "C" { pub fn nfsd_vers(nn:*mut nfsd_net, vers:i32, change:vers_op)->i32; pub fn nfsd_minorversion(nn:*mut nfsd_net, minorversion:u32, change:vers_op)->i32; pub fn nfsd_reset_versions(nn:*mut nfsd_net); pub fn nfsd_create_serv(net:*mut net)->i32; pub fn nfsd_destroy_serv(net:*mut net); }
#[cfg(feature="config_debug_fs")] extern "C" { pub fn nfsd_debugfs_init(); pub fn nfsd_debugfs_exit(); }
#[cfg(not(feature="config_debug_fs"))] pub unsafe fn nfsd_debugfs_init() {} #[cfg(not(feature="config_debug_fs"))] pub unsafe fn nfsd_debugfs_exit() {}
extern "C" { pub static mut nfsd_disable_splice_read: bool; pub static mut nfsd_delegts_enabled: bool; pub static mut nfsd_io_cache_read:u64; pub static mut nfsd_io_cache_write:u64; pub static mut nfsd_max_blksize:i32; }
#[repr(C)] pub enum nfsd_io { NFSD_IO_BUFFERED, NFSD_IO_DONTCACHE, NFSD_IO_DIRECT }
pub unsafe fn nfsd_v4client(rq: *mut svc_rqst) -> bool { !rq.is_null() && (*rq).rq_prog == NFS_PROGRAM && (*rq).rq_vers == 4 }

#[cfg(feature="config_nfsd_v4")] extern "C" { pub static mut max_delegations: usize; pub fn nfsd4_init_slabs()->i32; pub fn nfsd4_free_slabs(); pub fn nfs4_state_start()->i32; pub fn nfs4_state_start_net(net:*mut net)->i32; pub fn nfs4_state_shutdown(); pub fn nfs4_state_shutdown_net(net:*mut net); pub fn nfs4_reset_recoverydir(p:*mut i8)->i32; pub fn nfs4_recoverydir()->*mut i8; pub fn nfsd4_spo_must_allow(r:*mut svc_rqst)->bool; pub fn nfsd4_create_laundry_wq()->i32; pub fn nfsd4_destroy_laundry_wq(); pub fn nfsd_wait_for_delegreturn(r:*mut svc_rqst,i:*mut inode)->bool; }
#[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_init_slabs()->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_free_slabs(){} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_state_start()->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_state_start_net(_: *mut net)->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_state_shutdown(){} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_state_shutdown_net(_: *mut net){} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_reset_recoverydir(_: *mut i8)->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfs4_recoverydir()->*mut i8{core::ptr::null_mut()} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_spo_must_allow(_: *mut svc_rqst)->bool{false} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_create_laundry_wq()->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_destroy_laundry_wq(){} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd_wait_for_delegreturn(_: *mut svc_rqst,_: *mut inode)->bool{false}
extern "C" { pub fn nfsd_lockd_init(); pub fn nfsd_lockd_shutdown(); }

macro_rules! cpu_to_be32 { ($x:expr) => { $x.to_be() }; }

/* Protocol error aliases retain the original pre-XDR conversion. */
macro_rules! nfs_ok { () => { cpu_to_be32!(NFS_OK) }; }
macro_rules! nfserr_perm { () => { cpu_to_be32!(NFSERR_PERM) }; }
macro_rules! nfserr_noent { () => { cpu_to_be32!(NFSERR_NOENT) }; }
macro_rules! nfserr_io { () => { cpu_to_be32!(NFSERR_IO) }; }
macro_rules! nfserr_nxio { () => { cpu_to_be32!(NFSERR_NXIO) }; }
macro_rules! nfserr_acces { () => { cpu_to_be32!(NFSERR_ACCES) }; }
macro_rules! nfserr_exist { () => { cpu_to_be32!(NFSERR_EXIST) }; }
macro_rules! nfserr_xdev { () => { cpu_to_be32!(NFSERR_XDEV) }; }
macro_rules! nfserr_nodev { () => { cpu_to_be32!(NFSERR_NODEV) }; }
macro_rules! nfserr_notdir { () => { cpu_to_be32!(NFSERR_NOTDIR) }; }
macro_rules! nfserr_isdir { () => { cpu_to_be32!(NFSERR_ISDIR) }; }
macro_rules! nfserr_inval { () => { cpu_to_be32!(NFSERR_INVAL) }; }
macro_rules! nfserr_fbig { () => { cpu_to_be32!(NFSERR_FBIG) }; }
macro_rules! nfserr_nospc { () => { cpu_to_be32!(NFSERR_NOSPC) }; }
macro_rules! nfserr_rofs { () => { cpu_to_be32!(NFSERR_ROFS) }; }
macro_rules! nfserr_mlink { () => { cpu_to_be32!(NFSERR_MLINK) }; }
macro_rules! nfserr_nametoolong { () => { cpu_to_be32!(NFSERR_NAMETOOLONG) }; }
macro_rules! nfserr_notempty { () => { cpu_to_be32!(NFSERR_NOTEMPTY) }; }
macro_rules! nfserr_dquot { () => { cpu_to_be32!(NFSERR_DQUOT) }; }
macro_rules! nfserr_stale { () => { cpu_to_be32!(NFSERR_STALE) }; }
macro_rules! nfserr_remote { () => { cpu_to_be32!(NFSERR_REMOTE) }; }
macro_rules! nfserr_wflush { () => { cpu_to_be32!(NFSERR_WFLUSH) }; }
macro_rules! nfserr_badhandle { () => { cpu_to_be32!(NFSERR_BADHANDLE) }; }
macro_rules! nfserr_notsync { () => { cpu_to_be32!(NFSERR_NOT_SYNC) }; }
macro_rules! nfserr_badcookie { () => { cpu_to_be32!(NFSERR_BAD_COOKIE) }; }
macro_rules! nfserr_notsupp { () => { cpu_to_be32!(NFSERR_NOTSUPP) }; }
macro_rules! nfserr_toosmall { () => { cpu_to_be32!(NFSERR_TOOSMALL) }; }
macro_rules! nfserr_serverfault { () => { cpu_to_be32!(NFSERR_SERVERFAULT) }; }
macro_rules! nfserr_badtype { () => { cpu_to_be32!(NFSERR_BADTYPE) }; }
macro_rules! nfserr_jukebox { () => { cpu_to_be32!(NFSERR_JUKEBOX) }; }
macro_rules! nfserr_denied { () => { cpu_to_be32!(NFSERR_DENIED) }; }
macro_rules! nfserr_deadlock { () => { cpu_to_be32!(NFSERR_DEADLOCK) }; }
macro_rules! nfserr_expired { () => { cpu_to_be32!(NFSERR_EXPIRED) }; }
macro_rules! nfserr_bad_cookie { () => { cpu_to_be32!(NFSERR_BAD_COOKIE) }; }
macro_rules! nfserr_same { () => { cpu_to_be32!(NFSERR_SAME) }; }
macro_rules! nfserr_clid_inuse { () => { cpu_to_be32!(NFSERR_CLID_INUSE) }; }
macro_rules! nfserr_stale_clientid { () => { cpu_to_be32!(NFSERR_STALE_CLIENTID) }; }
macro_rules! nfserr_resource { () => { cpu_to_be32!(NFSERR_RESOURCE) }; }
macro_rules! nfserr_moved { () => { cpu_to_be32!(NFSERR_MOVED) }; }
macro_rules! nfserr_nofilehandle { () => { cpu_to_be32!(NFSERR_NOFILEHANDLE) }; }
macro_rules! nfserr_minor_vers_mismatch { () => { cpu_to_be32!(NFSERR_MINOR_VERS_MISMATCH) }; }
macro_rules! nfserr_share_denied { () => { cpu_to_be32!(NFSERR_SHARE_DENIED) }; }
macro_rules! nfserr_stale_stateid { () => { cpu_to_be32!(NFSERR_STALE_STATEID) }; }
macro_rules! nfserr_old_stateid { () => { cpu_to_be32!(NFSERR_OLD_STATEID) }; }
macro_rules! nfserr_bad_stateid { () => { cpu_to_be32!(NFSERR_BAD_STATEID) }; }
macro_rules! nfserr_bad_seqid { () => { cpu_to_be32!(NFSERR_BAD_SEQID) }; }
macro_rules! nfserr_symlink { () => { cpu_to_be32!(NFSERR_SYMLINK) }; }
macro_rules! nfserr_not_same { () => { cpu_to_be32!(NFSERR_NOT_SAME) }; }
macro_rules! nfserr_lock_range { () => { cpu_to_be32!(NFSERR_LOCK_RANGE) }; }
macro_rules! nfserr_restorefh { () => { cpu_to_be32!(NFSERR_RESTOREFH) }; }
macro_rules! nfserr_attrnotsupp { () => { cpu_to_be32!(NFSERR_ATTRNOTSUPP) }; }
macro_rules! nfserr_bad_xdr { () => { cpu_to_be32!(NFSERR_BAD_XDR) }; }
macro_rules! nfserr_openmode { () => { cpu_to_be32!(NFSERR_OPENMODE) }; }
macro_rules! nfserr_badowner { () => { cpu_to_be32!(NFSERR_BADOWNER) }; }
macro_rules! nfserr_locks_held { () => { cpu_to_be32!(NFSERR_LOCKS_HELD) }; }
macro_rules! nfserr_op_illegal { () => { cpu_to_be32!(NFSERR_OP_ILLEGAL) }; }
macro_rules! nfserr_grace { () => { cpu_to_be32!(NFSERR_GRACE) }; }
macro_rules! nfserr_no_grace { () => { cpu_to_be32!(NFSERR_NO_GRACE) }; }
macro_rules! nfserr_reclaim_bad { () => { cpu_to_be32!(NFSERR_RECLAIM_BAD) }; }
macro_rules! nfserr_badname { () => { cpu_to_be32!(NFSERR_BADNAME) }; }
macro_rules! nfserr_admin_revoked { () => { cpu_to_be32!(NFS4ERR_ADMIN_REVOKED) }; }
macro_rules! nfserr_cb_path_down { () => { cpu_to_be32!(NFSERR_CB_PATH_DOWN) }; }
macro_rules! nfserr_locked { () => { cpu_to_be32!(NFSERR_LOCKED) }; }
macro_rules! nfserr_wrongsec { () => { cpu_to_be32!(NFSERR_WRONGSEC) }; }
macro_rules! nfserr_delay { () => { cpu_to_be32!(NFS4ERR_DELAY) }; }
macro_rules! nfserr_badiomode { () => { cpu_to_be32!(NFS4ERR_BADIOMODE) }; }
macro_rules! nfserr_badlayout { () => { cpu_to_be32!(NFS4ERR_BADLAYOUT) }; }
macro_rules! nfserr_bad_session_digest { () => { cpu_to_be32!(NFS4ERR_BAD_SESSION_DIGEST) }; }
macro_rules! nfserr_badsession { () => { cpu_to_be32!(NFS4ERR_BADSESSION) }; }
macro_rules! nfserr_badslot { () => { cpu_to_be32!(NFS4ERR_BADSLOT) }; }
macro_rules! nfserr_complete_already { () => { cpu_to_be32!(NFS4ERR_COMPLETE_ALREADY) }; }
macro_rules! nfserr_conn_not_bound_to_session { () => { cpu_to_be32!(NFS4ERR_CONN_NOT_BOUND_TO_SESSION) }; }
macro_rules! nfserr_deleg_already_wanted { () => { cpu_to_be32!(NFS4ERR_DELEG_ALREADY_WANTED) }; }
macro_rules! nfserr_back_chan_busy { () => { cpu_to_be32!(NFS4ERR_BACK_CHAN_BUSY) }; }
macro_rules! nfserr_layouttrylater { () => { cpu_to_be32!(NFS4ERR_LAYOUTTRYLATER) }; }
macro_rules! nfserr_layoutunavailable { () => { cpu_to_be32!(NFS4ERR_LAYOUTUNAVAILABLE) }; }
macro_rules! nfserr_nomatching_layout { () => { cpu_to_be32!(NFS4ERR_NOMATCHING_LAYOUT) }; }
macro_rules! nfserr_recallconflict { () => { cpu_to_be32!(NFS4ERR_RECALLCONFLICT) }; }
macro_rules! nfserr_unknown_layouttype { () => { cpu_to_be32!(NFS4ERR_UNKNOWN_LAYOUTTYPE) }; }
macro_rules! nfserr_seq_misordered { () => { cpu_to_be32!(NFS4ERR_SEQ_MISORDERED) }; }
macro_rules! nfserr_sequence_pos { () => { cpu_to_be32!(NFS4ERR_SEQUENCE_POS) }; }
macro_rules! nfserr_req_too_big { () => { cpu_to_be32!(NFS4ERR_REQ_TOO_BIG) }; }
macro_rules! nfserr_rep_too_big { () => { cpu_to_be32!(NFS4ERR_REP_TOO_BIG) }; }
macro_rules! nfserr_rep_too_big_to_cache { () => { cpu_to_be32!(NFS4ERR_REP_TOO_BIG_TO_CACHE) }; }
macro_rules! nfserr_retry_uncached_rep { () => { cpu_to_be32!(NFS4ERR_RETRY_UNCACHED_REP) }; }
macro_rules! nfserr_unsafe_compound { () => { cpu_to_be32!(NFS4ERR_UNSAFE_COMPOUND) }; }
macro_rules! nfserr_too_many_ops { () => { cpu_to_be32!(NFS4ERR_TOO_MANY_OPS) }; }
macro_rules! nfserr_op_not_in_session { () => { cpu_to_be32!(NFS4ERR_OP_NOT_IN_SESSION) }; }
macro_rules! nfserr_hash_alg_unsupp { () => { cpu_to_be32!(NFS4ERR_HASH_ALG_UNSUPP) }; }
macro_rules! nfserr_clientid_busy { () => { cpu_to_be32!(NFS4ERR_CLIENTID_BUSY) }; }
macro_rules! nfserr_pnfs_io_hole { () => { cpu_to_be32!(NFS4ERR_PNFS_IO_HOLE) }; }
macro_rules! nfserr_seq_false_retry { () => { cpu_to_be32!(NFS4ERR_SEQ_FALSE_RETRY) }; }
macro_rules! nfserr_bad_high_slot { () => { cpu_to_be32!(NFS4ERR_BAD_HIGH_SLOT) }; }
macro_rules! nfserr_deadsession { () => { cpu_to_be32!(NFS4ERR_DEADSESSION) }; }
macro_rules! nfserr_encr_alg_unsupp { () => { cpu_to_be32!(NFS4ERR_ENCR_ALG_UNSUPP) }; }
macro_rules! nfserr_pnfs_no_layout { () => { cpu_to_be32!(NFS4ERR_PNFS_NO_LAYOUT) }; }
macro_rules! nfserr_not_only_op { () => { cpu_to_be32!(NFS4ERR_NOT_ONLY_OP) }; }
macro_rules! nfserr_wrong_cred { () => { cpu_to_be32!(NFS4ERR_WRONG_CRED) }; }
macro_rules! nfserr_wrong_type { () => { cpu_to_be32!(NFS4ERR_WRONG_TYPE) }; }
macro_rules! nfserr_dirdeleg_unavail { () => { cpu_to_be32!(NFS4ERR_DIRDELEG_UNAVAIL) }; }
macro_rules! nfserr_reject_deleg { () => { cpu_to_be32!(NFS4ERR_REJECT_DELEG) }; }
macro_rules! nfserr_returnconflict { () => { cpu_to_be32!(NFS4ERR_RETURNCONFLICT) }; }
macro_rules! nfserr_deleg_revoked { () => { cpu_to_be32!(NFS4ERR_DELEG_REVOKED) }; }
macro_rules! nfserr_partner_notsupp { () => { cpu_to_be32!(NFS4ERR_PARTNER_NOTSUPP) }; }
macro_rules! nfserr_partner_no_auth { () => { cpu_to_be32!(NFS4ERR_PARTNER_NO_AUTH) }; }
macro_rules! nfserr_union_notsupp { () => { cpu_to_be32!(NFS4ERR_UNION_NOTSUPP) }; }
macro_rules! nfserr_offload_denied { () => { cpu_to_be32!(NFS4ERR_OFFLOAD_DENIED) }; }
macro_rules! nfserr_wrong_lfs { () => { cpu_to_be32!(NFS4ERR_WRONG_LFS) }; }
macro_rules! nfserr_badlabel { () => { cpu_to_be32!(NFS4ERR_BADLABEL) }; }
macro_rules! nfserr_file_open { () => { cpu_to_be32!(NFS4ERR_FILE_OPEN) }; }
macro_rules! nfserr_xattr2big { () => { cpu_to_be32!(NFS4ERR_XATTR2BIG) }; }
macro_rules! nfserr_noxattr { () => { cpu_to_be32!(NFS4ERR_NOXATTR) }; }

pub const NFSERR_EOF: u32 = 30000;
pub const NFSERR_REPLAY_ME: u32 = 30001;
pub const NFSERR_REPLAY_CACHE: u32 = 30002;
pub const NFSERR_SYMLINK_NOT_DIR: u32 = 30003;
macro_rules! nfserr_eof { () => { cpu_to_be32!(NFSERR_EOF) }; }
macro_rules! nfserr_replay_me { () => { cpu_to_be32!(NFSERR_REPLAY_ME) }; }
macro_rules! nfserr_replay_cache { () => { cpu_to_be32!(NFSERR_REPLAY_CACHE) }; }
macro_rules! nfserr_symlink_not_dir { () => { cpu_to_be32!(NFSERR_SYMLINK_NOT_DIR) }; }

pub const COMPOUND_SLACK_SPACE: i32 = 140;
pub const COMPOUND_ERR_SLACK_SPACE: i32 = 16;
pub const NFSD_LAUNDROMAT_MINTIMEOUT: i32 = 1;
pub const NFSD_COURTESY_CLIENT_TIMEOUT: i32 = 24 * 60 * 60;
pub const NFSD_CLIENT_MAX_TRIM_PER_RUN: i32 = 128;
pub const NFS4_CLIENTS_PER_GB: i32 = 1024;
/* HZ is supplied by the kernel dependency. */
pub const NFSD_DELEGRETURN_TIMEOUT: i32 = HZ / 34;
pub const NFSD_CB_GETATTR_TIMEOUT: i32 = NFSD_DELEGRETURN_TIMEOUT;
#[cfg(feature="config_nfsd_v4")] extern "C" { pub fn nfsd4_is_junction(d:*mut dentry)->i32; pub fn register_cld_notifier()->i32; pub fn unregister_cld_notifier(); pub fn nfsd4_init_leases_net(nn:*mut nfsd_net); }
#[cfg(all(feature="config_nfsd_v4", feature="config_nfsd_v4_2_inter_ssc"))] extern "C" { pub fn nfsd4_ssc_init_umount_work(nn:*mut nfsd_net); }
#[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_is_junction(_: *mut dentry)->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn register_cld_notifier()->i32{0} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn unregister_cld_notifier(){} #[cfg(not(feature="config_nfsd_v4"))] pub unsafe fn nfsd4_init_leases_net(_: *mut nfsd_net){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
