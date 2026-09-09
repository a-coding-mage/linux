/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/auth.h
 *
 * Declarations for the RPC client authentication machinery.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the corresponding Linux/Rust translation units:
// sched.h, msg_prot.h, xdr.h, atomic.h, rcupdate.h, uidgid.h, and utsname.h.

pub const NUL_CALLSLACK: usize = 4;
pub const NUL_REPLYSLACK: usize = 2;
pub const UNX_MAXNODENAME: usize = __NEW_UTS_LEN;
pub const UNX_CALLSLACK: usize = 21 + XDR_QUADLEN(UNX_MAXNODENAME);
pub const UNX_NGROUPS: usize = 16;

pub struct rpcsec_gss_info;

#[repr(C)]
pub struct auth_cred {
    pub cred: *const cred,
    pub principal: *const core::ffi::c_char, // If present, this is a machine credential
}

pub struct rpc_auth;
pub struct rpc_credops;

#[repr(C)]
pub struct rpc_cred {
    pub cr_hash: hlist_node, // hash chain
    pub cr_lru: list_head, // lru garbage collection
    pub cr_rcu: rcu_head,
    pub cr_auth: *mut rpc_auth,
    pub cr_ops: *const rpc_credops,
    pub cr_expire: core::ffi::c_ulong, // when to gc
    pub cr_flags: core::ffi::c_ulong, // various flags
    pub cr_count: refcount_t, // ref count
    pub cr_cred: *const cred,
    // per-flavor data
}

pub const RPCAUTH_CRED_NEW: usize = 0;
pub const RPCAUTH_CRED_UPTODATE: usize = 1;
pub const RPCAUTH_CRED_HASHED: usize = 2;
pub const RPCAUTH_CRED_NEGATIVE: usize = 3;

unsafe extern "C" {
    pub fn rpc_machine_cred() -> *const cred;
}

pub struct rpc_cred_cache;
pub struct rpc_authops;

#[repr(C)]
pub struct rpc_auth {
    pub au_cslack: core::ffi::c_uint, // call cred size estimate
    pub au_rslack: core::ffi::c_uint, // reply cred size estimate
    pub au_verfsize: core::ffi::c_uint, // size of reply verifier
    pub au_ralign: core::ffi::c_uint, // words before UL header
    pub au_flags: core::ffi::c_ulong,
    pub au_ops: *const rpc_authops,
    pub au_flavor: rpc_authflavor_t, // pseudoflavor
    pub au_count: refcount_t, // Reference counter
    pub au_credcache: *mut rpc_cred_cache,
    // per-flavor data
}

pub const RPCAUTH_AUTH_DATATOUCH: usize = 1;
pub const RPCAUTH_AUTH_UPDATE_SLACK: usize = 2;

#[repr(C)]
pub struct rpc_auth_create_args {
    pub pseudoflavor: rpc_authflavor_t,
    pub target_name: *const core::ffi::c_char,
}

pub const RPCAUTH_LOOKUP_NEW: core::ffi::c_int = 0x01; // Accept an uninitialised cred
pub const RPCAUTH_LOOKUP_ASYNC: core::ffi::c_int = 0x02; // Don't block waiting for memory

#[repr(C)]
pub struct rpc_authops {
    pub owner: *mut module,
    pub au_flavor: rpc_authflavor_t,
    pub au_name: *mut core::ffi::c_char,
    pub create: Option<unsafe extern "C" fn(*const rpc_auth_create_args, *mut rpc_clnt) -> *mut rpc_auth>,
    pub destroy: Option<unsafe extern "C" fn(*mut rpc_auth)>,
    pub hash_cred: Option<unsafe extern "C" fn(*mut auth_cred, core::ffi::c_uint) -> core::ffi::c_int>,
    pub lookup_cred: Option<unsafe extern "C" fn(*mut rpc_auth, *mut auth_cred, core::ffi::c_int) -> *mut rpc_cred>,
    pub crcreate: Option<unsafe extern "C" fn(*mut rpc_auth, *mut auth_cred, core::ffi::c_int, gfp_t) -> *mut rpc_cred>,
    pub info2flavor: Option<unsafe extern "C" fn(*mut rpcsec_gss_info) -> rpc_authflavor_t>,
    pub flavor2info: Option<unsafe extern "C" fn(rpc_authflavor_t, *mut rpcsec_gss_info) -> core::ffi::c_int>,
    pub key_timeout: Option<unsafe extern "C" fn(*mut rpc_auth, *mut rpc_cred) -> core::ffi::c_int>,
    pub ping: Option<unsafe extern "C" fn(*mut rpc_clnt) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct rpc_credops {
    pub cr_name: *const core::ffi::c_char,
    pub cr_init: Option<unsafe extern "C" fn(*mut rpc_auth, *mut rpc_cred) -> core::ffi::c_int>,
    pub crdestroy: Option<unsafe extern "C" fn(*mut rpc_cred)>,
    pub crmatch: Option<unsafe extern "C" fn(*mut auth_cred, *mut rpc_cred, core::ffi::c_int) -> core::ffi::c_int>,
    pub crmarshal: Option<unsafe extern "C" fn(*mut rpc_task, *mut xdr_stream) -> core::ffi::c_int>,
    pub crrefresh: Option<unsafe extern "C" fn(*mut rpc_task) -> core::ffi::c_int>,
    pub crvalidate: Option<unsafe extern "C" fn(*mut rpc_task, *mut xdr_stream) -> core::ffi::c_int>,
    pub crwrap_req: Option<unsafe extern "C" fn(*mut rpc_task, *mut xdr_stream) -> core::ffi::c_int>,
    pub crunwrap_resp: Option<unsafe extern "C" fn(*mut rpc_task, *mut xdr_stream) -> core::ffi::c_int>,
    pub crkey_timeout: Option<unsafe extern "C" fn(*mut rpc_cred) -> core::ffi::c_int>,
    pub crstringify_acceptor: Option<unsafe extern "C" fn(*mut rpc_cred) -> *mut core::ffi::c_char>,
    pub crneed_reencode: Option<unsafe extern "C" fn(*mut rpc_task) -> bool>,
}

unsafe extern "C" {
    pub static authunix_ops: rpc_authops;
    pub static authnull_ops: rpc_authops;
    pub static authtls_ops: rpc_authops;
    pub fn rpc_init_authunix() -> core::ffi::c_int;
    pub fn rpcauth_init_module() -> core::ffi::c_int;
    pub fn rpcauth_remove_module();
    pub fn rpc_destroy_authunix();
    pub fn rpcauth_register(ops: *const rpc_authops) -> core::ffi::c_int;
    pub fn rpcauth_unregister(ops: *const rpc_authops) -> core::ffi::c_int;
    pub fn rpcauth_create(args: *const rpc_auth_create_args, clnt: *mut rpc_clnt) -> *mut rpc_auth;
    pub fn rpcauth_release(auth: *mut rpc_auth);
    pub fn rpcauth_get_pseudoflavor(flavor: rpc_authflavor_t, info: *mut rpcsec_gss_info) -> rpc_authflavor_t;
    pub fn rpcauth_get_gssinfo(flavor: rpc_authflavor_t, info: *mut rpcsec_gss_info) -> core::ffi::c_int;
    pub fn rpcauth_lookup_credcache(auth: *mut rpc_auth, cred: *mut auth_cred, flags: core::ffi::c_int, gfp: gfp_t) -> *mut rpc_cred;
    pub fn rpcauth_init_cred(cred: *mut rpc_cred, auth_cred: *const auth_cred, auth: *mut rpc_auth, ops: *const rpc_credops);
    pub fn rpcauth_lookupcred(auth: *mut rpc_auth, flags: core::ffi::c_int) -> *mut rpc_cred;
    pub fn put_rpccred(cred: *mut rpc_cred);
    pub fn rpcauth_marshcred(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_checkverf(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_wrap_req_encode(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_wrap_req(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_unwrap_resp_decode(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_unwrap_resp(task: *mut rpc_task, xdr: *mut xdr_stream) -> core::ffi::c_int;
    pub fn rpcauth_xmit_need_reencode(task: *mut rpc_task) -> bool;
    pub fn rpcauth_refreshcred(task: *mut rpc_task) -> core::ffi::c_int;
    pub fn rpcauth_invalcred(task: *mut rpc_task);
    pub fn rpcauth_uptodatecred(task: *mut rpc_task) -> core::ffi::c_int;
    pub fn rpcauth_init_credcache(auth: *mut rpc_auth) -> core::ffi::c_int;
    pub fn rpcauth_destroy_credcache(auth: *mut rpc_auth);
    pub fn rpcauth_clear_credcache(cache: *mut rpc_cred_cache);
    pub fn rpcauth_stringify_acceptor(cred: *mut rpc_cred) -> *mut core::ffi::c_char;
}

#[inline]
pub unsafe fn get_rpccred(cred: *mut rpc_cred) -> *mut rpc_cred {
    if !cred.is_null() && refcount_inc_not_zero(&mut (*cred).cr_count) {
        cred
    } else {
        core::ptr::null_mut()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
