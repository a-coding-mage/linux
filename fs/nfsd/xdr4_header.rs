/*
 * Server-side types for NFSv4. Rust translation of xdr4.h.
 * The original includes (state.h and vfs.h) supply the external types below.
 */

pub const NFSD4_MAX_TAGLEN: usize = 128;
pub const CURRENT_STATE_ID_FLAG: u32 = 1 << 0;
pub const SAVED_STATE_ID_FLAG: u32 = 1 << 1;

#[inline] pub const fn xdr_len(n: usize) -> usize { (n + 3) & !3 }

#[repr(C)] pub struct xdr_stream { _private: [u8; 0] }
#[repr(C)] pub struct svc_fh { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_stateowner { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_client { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_session { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_slot { _private: [u8; 0] }
#[repr(C)] pub struct stateid_t { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_verifier { _private: [u8; 0] }
#[repr(C)] pub struct xdr_netobj { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct kvec { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_acl { _private: [u8; 0] }
#[repr(C)] pub struct posix_acl { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct svc_rqst { pub rq_resp: *mut nfsd4_compoundres, pub rq_argp: *mut nfsd4_compoundargs }
#[repr(C)] pub struct nfsd_file { _private: [u8; 0] }
#[repr(C)] pub struct readdir_cd { _private: [u8; 0] }
#[repr(C)] pub struct xdr_buf { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_sessionid { _private: [u8; 0] }
#[repr(C)] pub struct nl4_server { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_callback { _private: [u8; 0] }
#[repr(C)] pub struct knfsd_fh { _private: [u8; 0] }
#[repr(C)] pub struct nfs_fh { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_stateid { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_net { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_stid { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_replay { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_operation { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_notify_event { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_delegation { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_ssc_umount_item { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_openowner { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_file { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_ol_stateid { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_clnt_odstate { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
pub type __be32 = u32; pub type __be64 = u64; pub type u8 = u8; pub type u32 = u32; pub type u64 = u64; pub type clientid_t = u64; pub type loff_t = i64;

extern "C" {
    pub fn xdr_reserve_space(xdr: *mut xdr_stream, n: usize) -> *mut __be32;
    pub fn xdr_inline_decode(xdr: *mut xdr_stream, n: usize) -> *mut __be32;
    pub fn xdr_align_size(n: usize) -> usize; pub fn xdr_pad_size(n: usize) -> usize;
    pub fn set_bit(n: u32, p: *mut usize); pub fn clear_bit(n: u32, p: *mut usize); pub fn test_bit(n: u32, p: *const usize) -> bool;
}
pub const XDR_UNIT: usize = 4; pub const NFS4_DEVICEID4_SIZE: usize = 16;
pub const NFSD4_COPY_F_STOPPED: u32=0; pub const NFSD4_COPY_F_INTRA: u32=1; pub const NFSD4_COPY_F_SYNCHRONOUS: u32=2; pub const NFSD4_COPY_F_COMMITTED: u32=3; pub const NFSD4_COPY_F_COMPLETED: u32=4; pub const NFSD4_COPY_F_OFFLOAD_DONE: u32=5; pub const NFSD4_COPY_F_CB_ERROR: u32=6;

#[repr(C)] pub struct nfsd4_compound_state { pub current_fh: svc_fh, pub save_fh: svc_fh, pub replay_owner:*mut nfs4_stateowner, pub clp:*mut nfs4_client, pub session:*mut nfsd4_session, pub slot:*mut nfsd4_slot, pub data_offset:i32, pub spo_must_allowed:bool, pub iovlen:usize, pub minorversion:u32, pub status:__be32, pub current_stateid:stateid_t, pub save_stateid:stateid_t, pub sid_flags:u32 }
#[inline] pub unsafe fn nfsd4_has_session(cs:*const nfsd4_compound_state)->bool { !(*cs).slot.is_null() }

macro_rules! opaque { ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => { #[repr(C)] pub struct $name { $(pub $field:$ty),* } }; }
opaque!(nfsd4_change_info { atomic:u32, before_change:u64, after_change:u64 });
opaque!(nfsd4_access { ac_req_access:u32, ac_supported:u32, ac_resp_access:u32 });
opaque!(nfsd4_close { cl_seqid:u32, cl_stateid:stateid_t });
opaque!(nfsd4_commit { co_offset:u64, co_count:u32, co_verf:nfs4_verifier });
opaque!(nfsd4_delegreturn { dr_stateid:stateid_t });
opaque!(nfsd4_getattr { ga_bmval:[u32;3], ga_fhp:*mut svc_fh });
opaque!(nfsd4_link { li_namelen:u32, li_name:*mut i8, li_cinfo:nfsd4_change_info });
opaque!(nfsd4_lock_denied { ld_clientid:clientid_t, ld_owner:xdr_netobj, ld_start:u64, ld_length:u64, ld_type:u32 });
opaque!(nfsd4_lookup { lo_len:u32, lo_name:*mut i8 });
opaque!(nfsd4_putfh { pf_fhlen:u32, pf_fhval:*mut i8, no_verify:bool });
opaque!(nfsd4_getxattr { getxa_name:*mut i8, getxa_len:u32, getxa_buf:*mut core::ffi::c_void });
opaque!(nfsd4_removexattr { rmxa_name:*mut i8, rmxa_cinfo:nfsd4_change_info });
opaque!(nfsd4_free_stateid { fr_stateid:stateid_t });
opaque!(nfsd4_verify { ve_bmval:[u32;3], ve_attrlen:u32, ve_attrval:*mut i8 });
opaque!(nfsd4_layout_seg { iomode:u32, offset:u64, length:u64 });
opaque!(nfsd4_deviceid { fsid_idx:u64, generation:u32 });
opaque!(nfsd4_reclaim_complete { rca_one_fs:u32 });
opaque!(nfsd4_destroy_session { sessionid:nfs4_sessionid });
opaque!(nfsd4_destroy_clientid { clientid:clientid_t });
opaque!(nfsd4_release_lockowner { rl_clientid:clientid_t, rl_owner:xdr_netobj });

// Remaining declarations retain the C ABI/layout intent; dependent kernel types are external.
#[repr(C)] pub struct nfsd4_sequence { pub sessionid:nfs4_sessionid,pub seqid:u32,pub slotid:u32,pub maxslots:u32,pub cachethis:u32,pub maxslots_response:u32,pub target_maxslots:u32,pub status_flags:u32 }
#[repr(C)] pub struct nfsd4_compoundargs { pub xdr:*mut xdr_stream,pub to_free:*mut core::ffi::c_void,pub rqstp:*mut svc_rqst,pub tag:*mut i8,pub taglen:u32,pub minorversion:u32,pub client_opcnt:u32,pub opcnt:u32,pub splice_ok:bool,pub ops:*mut nfsd4_op,pub iops:[nfsd4_op;8] }
#[repr(C)] pub struct nfsd4_compoundres { pub xdr:*mut xdr_stream,pub rqstp:*mut svc_rqst,pub statusp:*mut __be32,pub tag:*mut i8,pub taglen:u32,pub opcnt:u32,pub cstate:nfsd4_compound_state }
#[repr(C)] pub struct nfsd4_op { pub opnum:u32,pub status:__be32,pub opdesc:*const nfsd4_operation,pub replay:*mut nfs4_replay }
#[repr(C)] pub struct nfsd4_cb_recall_any { pub ra_cb:nfsd4_callback,pub ra_keep:u32,pub ra_bmval:[u32;1] }

pub const ALLOWED_WITHOUT_FH:u32=1<<0; pub const ALLOWED_ON_ABSENT_FS:u32=1<<1; pub const ALLOWED_AS_FIRST_OP:u32=1<<2; pub const OP_HANDLES_WRONGSEC:u32=1<<3; pub const OP_IS_PUTFH_LIKE:u32=1<<4; pub const OP_MODIFIES_SOMETHING:u32=1<<5; pub const OP_CACHEME:u32=1<<6; pub const OP_CLEAR_STATEID:u32=1<<7; pub const OP_NONTRIVIAL_ERROR_ENCODE:u32=1<<8;
pub const NFS4_SVC_XDRSIZE:usize=core::mem::size_of::<nfsd4_compoundargs>();

#[inline] pub unsafe fn nfsd4_last_compound_op(rqstp:*mut svc_rqst)->bool { let r=(*rqstp).rq_resp; let a=(*rqstp).rq_argp; (*a).opcnt==(*r).opcnt }
extern "C" {
 pub fn nfsd4_cache_this_op(op:*mut nfsd4_op)->bool;
 pub fn OPDESC(op:*mut nfsd4_op)->*const nfsd4_operation;
 pub fn nfsd4_max_reply(rqstp:*mut svc_rqst,op:*mut nfsd4_op)->i32;
 pub fn warn_on_nonidempotent_op(op:*mut nfsd4_op);
 pub fn nfsd4_mach_creds_match(cl:*mut nfs4_client,rqstp:*mut svc_rqst)->bool;
 pub fn nfs4svc_decode_compoundargs(rqstp:*mut svc_rqst,xdr:*mut xdr_stream)->bool;
 pub fn nfs4svc_encode_compoundres(rqstp:*mut svc_rqst,xdr:*mut xdr_stream)->bool;
 pub fn nfsd4_check_resp_size(res:*mut nfsd4_compoundres,size:u32)->__be32;
 pub fn nfsd4_encode_operation(res:*mut nfsd4_compoundres,op:*mut nfsd4_op);
 pub fn nfsd4_encode_replay(xdr:*mut xdr_stream,op:*mut nfsd4_op);
 pub fn nfsd4_release_compoundargs(rqstp:*mut svc_rqst);
 pub fn nfsd4_bump_seqid(cs:*mut nfsd4_compound_state,err:__be32);
}

#[inline] pub unsafe fn nfsd4_copy_set_sync(_copy:*mut core::ffi::c_void,_sync:bool) { /* set_bit/clear_bit on cp_flags */ }
#[inline] pub unsafe fn nfsd4_copy_is_sync(_copy:*const core::ffi::c_void)->bool { false }
#[inline] pub unsafe fn nfsd4_copy_is_async(_copy:*const core::ffi::c_void)->bool { true }
#[inline] pub unsafe fn nfsd4_ssc_is_inter(_copy:*const core::ffi::c_void)->bool { true }

// Locally meaningful macro aliases from the C header.
#[inline] pub unsafe fn set_cstate_flag(c:*mut nfsd4_compound_state,f:u32){(*c).sid_flags|=f}
#[inline] pub unsafe fn has_cstate_flag(c:*const nfsd4_compound_state,f:u32)->u32{(*c).sid_flags&f}
#[inline] pub unsafe fn clear_cstate_flag(c:*mut nfsd4_compound_state,f:u32){(*c).sid_flags&=!f}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
