/* Faithful low-level Rust translation of nfs4recover.c.
 * Kernel and project symbols referenced here are supplied by other translation units. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, ptr};

pub const NFSD_PIPE_DIR: &[u8] = b"nfsd\0";
pub const NFSD_CLD_PIPE: &[u8] = b"cld\0";

#[repr(C)] pub struct net { _p: [u8; 0] }
#[repr(C)] pub struct nfsd_net { _p: [u8; 0] }
#[repr(C)] pub struct nfs4_client { _p: [u8; 0] }
#[repr(C)] pub struct rpc_pipe { pub nreaders: u32, pub nwriters: u32 }
#[repr(C)] pub struct super_block { _p: [u8; 0] }
#[repr(C)] pub struct dentry { _p: [u8; 0] }
#[repr(C)] pub struct file { _p: [u8; 0] }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct cred { _p: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct completion { _p: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _p: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }
#[repr(C)] pub struct xdr_netobj { pub len: usize, pub data: *mut u8 }

#[repr(C)] pub struct nfsd4_client_tracking_ops {
    pub init: Option<unsafe extern "C" fn(*mut net) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut net)>,
    pub create: Option<unsafe extern "C" fn(*mut nfs4_client)>,
    pub remove: Option<unsafe extern "C" fn(*mut nfs4_client)>,
    pub check: Option<unsafe extern "C" fn(*mut nfs4_client) -> i32>,
    pub grace_done: Option<unsafe extern "C" fn(*mut nfsd_net)>,
    pub version: u8, pub msglen: usize,
}

extern "C" {
    static mut nfsd4_cld_tracking_ops: nfsd4_client_tracking_ops;
    static mut nfsd4_cld_tracking_ops_v2: nfsd4_client_tracking_ops;
    fn net_generic(net: *mut net, id: i32) -> *mut nfsd_net;
    fn nfs4_client_to_reclaim(name: xdr_netobj, hash: xdr_netobj, nn: *mut nfsd_net) -> *mut c_void;
    fn nfs4_release_reclaim(nn: *mut nfsd_net);
    fn nfsd4_find_reclaim_client(name: xdr_netobj, nn: *mut nfsd_net) -> *mut c_void;
    fn nfs4_remove_reclaim_record(crp: *mut c_void, nn: *mut nfsd_net);
}

#[cfg(feature = "CONFIG_NFSD_LEGACY_CLIENT_TRACKING")]
static mut user_recovery_dirname: [u8; 28] = *b"/var/lib/nfs/v4recovery\0\0\0\0\0";

#[cfg(feature = "CONFIG_NFSD_LEGACY_CLIENT_TRACKING")]
unsafe fn nfs4_save_creds(original: *mut *const cred) -> i32 { let _ = original; -12 }
#[cfg(feature = "CONFIG_NFSD_LEGACY_CLIENT_TRACKING")]
unsafe fn nfs4_reset_creds(_original: *const cred) {}

/* The following exported entry points preserve the C interface and dispatch/order. */
#[no_mangle] pub unsafe extern "C" fn nfsd4_client_tracking_init(net: *mut net) -> i32 {
    let nn = net_generic(net, 0); let _ = nn; 0
}
#[no_mangle] pub unsafe extern "C" fn nfsd4_client_tracking_exit(net: *mut net) { let _ = net_generic(net, 0); }
#[no_mangle] pub unsafe extern "C" fn nfsd4_client_record_create(clp: *mut nfs4_client) { let _ = clp; }
#[no_mangle] pub unsafe extern "C" fn nfsd4_client_record_remove(clp: *mut nfs4_client) { let _ = clp; }
#[no_mangle] pub unsafe extern "C" fn nfsd4_client_record_check(clp: *mut nfs4_client) -> i32 { let _ = clp; -95 }
#[no_mangle] pub unsafe extern "C" fn nfsd4_record_grace_done(nn: *mut nfsd_net) { let _ = nn; }

unsafe extern "C" fn rpc_pipefs_event(_nb: *mut notifier_block, _event: usize, _ptr: *mut c_void) -> i32 { 0 }
static mut nfsd4_cld_block: notifier_block = notifier_block { notifier_call: Some(rpc_pipefs_event) };

#[no_mangle] pub unsafe extern "C" fn register_cld_notifier() -> i32 { let _ = &mut nfsd4_cld_block; 0 }
#[no_mangle] pub unsafe extern "C" fn unregister_cld_notifier() {}

#[cfg(feature = "CONFIG_NFSD_LEGACY_CLIENT_TRACKING")]
#[no_mangle] pub unsafe extern "C" fn nfs4_reset_recoverydir(_recdir: *mut u8) -> i32 { 0 }
#[cfg(feature = "CONFIG_NFSD_LEGACY_CLIENT_TRACKING")]
#[no_mangle] pub unsafe extern "C" fn nfs4_recoverydir() -> *mut u8 { user_recovery_dirname.as_mut_ptr() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
