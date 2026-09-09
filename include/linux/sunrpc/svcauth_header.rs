/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/svcauth.h
 *
 * RPC server-side authentication stuff.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

/* Dependencies are supplied by the corresponding translated kernel headers. */

#[repr(C)]
pub struct svc_cred {
    pub cr_uid: kuid_t,
    pub cr_gid: kgid_t,
    pub cr_group_info: *mut group_info,
    pub cr_flavor: u32, /* pseudoflavor */
    /* name of form servicetype/hostname@REALM, passed down by gss-proxy */
    pub cr_raw_principal: *mut c_char,
    /* name of form servicetype@hostname, passed down by rpc.svcgssd */
    pub cr_principal: *mut c_char,
    pub cr_targ_princ: *mut c_char,
    pub cr_gss_mech: *mut gss_api_mech,
}

#[inline]
pub unsafe fn init_svc_cred(cred: *mut svc_cred) {
    (*cred).cr_group_info = core::ptr::null_mut();
    (*cred).cr_raw_principal = core::ptr::null_mut();
    (*cred).cr_principal = core::ptr::null_mut();
    (*cred).cr_targ_princ = core::ptr::null_mut();
    (*cred).cr_gss_mech = core::ptr::null_mut();
}

#[inline]
pub unsafe fn free_svc_cred(cred: *mut svc_cred) {
    if !(*cred).cr_group_info.is_null() {
        put_group_info((*cred).cr_group_info);
    }
    kfree((*cred).cr_raw_principal);
    kfree((*cred).cr_principal);
    kfree((*cred).cr_targ_princ);
    gss_mech_put((*cred).cr_gss_mech);
    init_svc_cred(cred);
}

pub struct svc_rqst; /* forward decl */
pub struct in6_addr;

#[repr(C)]
pub struct auth_domain {
    pub ref_: kref,
    pub hash: hlist_node,
    pub name: *mut c_char,
    pub flavour: *mut auth_ops,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum svc_auth_status {
    SVC_GARBAGE = 1,
    SVC_VALID,
    SVC_NEGATIVE,
    SVC_OK,
    SVC_DROP,
    SVC_CLOSE,
    SVC_DENIED,
    SVC_PENDING,
    SVC_COMPLETE,
}

#[repr(C)]
pub struct auth_ops {
    pub name: *mut c_char,
    pub owner: *mut module,
    pub flavour: i32,
    pub accept: Option<unsafe extern "C" fn(*mut svc_rqst) -> svc_auth_status>,
    pub release: Option<unsafe extern "C" fn(*mut svc_rqst) -> i32>,
    pub domain_release: Option<unsafe extern "C" fn(*mut auth_domain)>,
    pub set_client: Option<unsafe extern "C" fn(*mut svc_rqst) -> svc_auth_status>,
    pub pseudoflavor: Option<unsafe extern "C" fn(*mut svc_rqst) -> rpc_authflavor_t>,
}

pub struct svc_xprt;

extern "C" {
    pub fn svc_auth_flavor(rqstp: *mut svc_rqst) -> rpc_authflavor_t;
    pub fn svc_authorise(rqstp: *mut svc_rqst) -> i32;
    pub fn svc_set_client(rqstp: *mut svc_rqst) -> svc_auth_status;
    pub fn svc_auth_register(flavor: rpc_authflavor_t, aops: *mut auth_ops) -> i32;
    pub fn svc_auth_unregister(flavor: rpc_authflavor_t);
    pub fn svcauth_map_clnt_to_svc_cred_local(
        clnt: *mut rpc_clnt, cred: *const cred, svc_cred: *mut svc_cred,
    );
    pub fn unix_domain_find(name: *mut c_char) -> *mut auth_domain;
    pub fn auth_domain_put(item: *mut auth_domain);
    pub fn auth_domain_lookup(name: *mut c_char, new: *mut auth_domain) -> *mut auth_domain;
    pub fn auth_domain_find(name: *mut c_char) -> *mut auth_domain;
    pub fn svcauth_unix_purge(net: *mut net);
    pub fn svcauth_unix_info_release(xpt: *mut svc_xprt);
    pub fn svcauth_unix_set_client(rqstp: *mut svc_rqst) -> svc_auth_status;
    pub fn unix_gid_cache_create(net: *mut net) -> i32;
    pub fn unix_gid_cache_destroy(net: *mut net);
}

#[inline]
pub unsafe fn hash_str(name: *const c_char, bits: i32) -> c_ulong {
    hashlen_hash(hashlen_string(core::ptr::null_mut(), name)) >> (32 - bits)
}

#[inline]
pub unsafe fn hash_mem(buf: *const c_char, length: i32, bits: i32) -> c_ulong {
    full_name_hash(core::ptr::null_mut(), buf, length) >> (32 - bits)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
