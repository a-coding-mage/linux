// SPDX-License-Identifier: GPL-2.0
/*
 * linux/net/sunrpc/auth_unix.c
 *
 * UNIX-style authentication; no AUTH_SHORT support
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependency includes from the C source are supplied by other translation units.
// #if IS_ENABLED(CONFIG_SUNRPC_DEBUG)
// #define RPCDBG_FACILITY RPCDBG_AUTH
// #endif

static mut unix_pool: *mut mempool_t = core::ptr::null_mut();

unsafe fn unx_create(_args: *const rpc_auth_create_args, _clnt: *mut rpc_clnt) -> *mut rpc_auth {
    refcount_inc(&mut (*core::ptr::addr_of_mut!(unix_auth)).au_count);
    core::ptr::addr_of_mut!(unix_auth)
}

unsafe fn unx_destroy(_auth: *mut rpc_auth) {}

/* Lookup AUTH_UNIX creds for current process */
unsafe fn unx_lookup_cred(
    auth: *mut rpc_auth,
    acred: *mut auth_cred,
    flags: c_int,
) -> *mut rpc_cred {
    let mut ret: *mut rpc_cred = kmalloc_obj::<rpc_cred>(rpc_task_gfp_mask());
    if ret.is_null() {
        if flags & RPCAUTH_LOOKUP_ASYNC == 0 {
            return ERR_PTR(-ENOMEM);
        }
        ret = mempool_alloc(unix_pool, GFP_NOWAIT) as *mut rpc_cred;
        if ret.is_null() {
            return ERR_PTR(-ENOMEM);
        }
    }
    rpcauth_init_cred(ret, acred, auth, core::ptr::addr_of!(unix_credops));
    (*ret).cr_flags = 1usize << RPCAUTH_CRED_UPTODATE;
    ret
}

unsafe fn unx_free_cred_callback(head: *mut rcu_head) {
    let rpc_cred = container_of!(head, rpc_cred, cr_rcu);
    put_cred((*rpc_cred).cr_cred);
    mempool_free(rpc_cred as *mut core::ffi::c_void, unix_pool);
}

unsafe fn unx_destroy_cred(cred: *mut rpc_cred) {
    call_rcu(&mut (*cred).cr_rcu, unx_free_cred_callback);
}

/* Match credentials against current the auth_cred. */
unsafe fn unx_match(acred: *mut auth_cred, cred: *mut rpc_cred, _flags: c_int) -> c_int {
    let mut groups: c_uint = 0;
    if (*cred).cr_cred == (*acred).cred {
        return 1;
    }
    if !uid_eq((*(*cred).cr_cred).fsuid, (*(*acred).cred).fsuid)
        || !gid_eq((*(*cred).cr_cred).fsgid, (*(*acred).cred).fsgid)
    {
        return 0;
    }
    if !(*(*acred).cred).group_info.is_null() {
        groups = (*(*(*acred).cred).group_info).ngroups;
    }
    if groups > UNX_NGROUPS {
        groups = UNX_NGROUPS;
    }
    if (*(*cred).cr_cred).group_info.is_null() {
        return (groups == 0) as c_int;
    }
    if groups != (*(*(*cred).cr_cred).group_info).ngroups {
        return 0;
    }
    for i in 0..groups {
        if !gid_eq(
            (*(*(*cred).cr_cred).group_info).gid[i as usize],
            (*(*(*acred).cred).group_info).gid[i as usize],
        ) {
            return 0;
        }
    }
    1
}

/* Marshal credentials. Maybe we should keep a cached credential for performance reasons. */
unsafe fn unx_marshal(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int {
    let clnt = (*task).tk_client;
    let cred = (*(*task).tk_rqstp).rq_cred;
    let mut p: *mut __be32;
    let mut cred_len: *mut __be32;
    let mut gidarr_len: *mut __be32;
    let mut i: c_int;
    let gi = (*(*cred).cr_cred).group_info;
    let userns = if !(*clnt).cl_cred.is_null() {
        (*(*clnt).cl_cred).user_ns
    } else {
        core::ptr::addr_of_mut!(init_user_ns)
    };

    p = xdr_reserve_space(xdr, 3 * core::mem::size_of::<__be32>()) as *mut __be32;
    if p.is_null() { return -EMSGSIZE; }
    *p = rpc_auth_unix; p = p.add(1);
    cred_len = p; p = p.add(1);
    *p = xdr_zero; p = p.add(1);
    if xdr_stream_encode_opaque(xdr, (*clnt).cl_nodename, (*clnt).cl_nodelen) < 0 { return -EMSGSIZE; }
    p = xdr_reserve_space(xdr, 3 * core::mem::size_of::<__be32>()) as *mut __be32;
    if p.is_null() { return -EMSGSIZE; }
    *p = cpu_to_be32(from_kuid_munged(userns, (*(*cred).cr_cred).fsuid)); p = p.add(1);
    *p = cpu_to_be32(from_kgid_munged(userns, (*(*cred).cr_cred).fsgid)); p = p.add(1);
    gidarr_len = p; p = p.add(1);
    if !gi.is_null() {
        for i in 0..UNX_NGROUPS.min((*gi).ngroups) {
            *p = cpu_to_be32(from_kgid_munged(userns, (*gi).gid[i as usize])); p = p.add(1);
        }
    }
    *gidarr_len = cpu_to_be32(p.offset_from(gidarr_len).wrapping_sub(1) as u32);
    *cred_len = cpu_to_be32((p.offset_from(cred_len).wrapping_sub(1) << 2) as u32);
    if (xdr_reserve_space(xdr, (p.offset_from(gidarr_len).wrapping_sub(1) << 2) as usize)).is_null() { return -EMSGSIZE; }
    p = xdr_reserve_space(xdr, 2 * core::mem::size_of::<__be32>()) as *mut __be32;
    if p.is_null() { return -EMSGSIZE; }
    *p = rpc_auth_null; *p.add(1) = xdr_zero;
    0
}

/* Refresh credentials. This is a no-op for AUTH_UNIX */
unsafe fn unx_refresh(task: *mut rpc_task) -> c_int {
    set_bit(RPCAUTH_CRED_UPTODATE, &mut (*(*task).tk_rqstp).rq_cred.cr_flags);
    0
}

unsafe fn unx_validate(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int {
    let auth = (*(*task).tk_rqstp).rq_cred.cr_auth;
    let p = xdr_inline_decode(xdr, 2 * core::mem::size_of::<__be32>()) as *mut __be32;
    if p.is_null() { return -EIO; }
    let flavor = *p;
    if flavor != rpc_auth_null && flavor != rpc_auth_unix && flavor != rpc_auth_short { return -EIO; }
    let size = be32_to_cpup(p.add(1));
    if size > RPC_MAX_AUTH_SIZE { return -EIO; }
    if (xdr_inline_decode(xdr, size as usize)).is_null() { return -EIO; }
    (*auth).au_verfsize = XDR_QUADLEN(size) + 2;
    (*auth).au_rslack = XDR_QUADLEN(size) + 2;
    (*auth).au_ralign = XDR_QUADLEN(size) + 2;
    0
}

#[no_mangle]
pub unsafe extern "C" fn rpc_init_authunix() -> c_int {
    unix_pool = mempool_create_kmalloc_pool(16, core::mem::size_of::<rpc_cred>());
    if !unix_pool.is_null() { 0 } else { -ENOMEM }
}

#[no_mangle]
pub unsafe extern "C" fn rpc_destroy_authunix() { mempool_destroy(unix_pool); }

pub static authunix_ops: rpc_authops = rpc_authops {
    owner: THIS_MODULE, au_flavor: RPC_AUTH_UNIX, au_name: c"UNIX".as_ptr(),
    create: Some(unx_create), destroy: Some(unx_destroy), lookup_cred: Some(unx_lookup_cred),
};

// The following static initializers preserve the C definitions and reference external ABI types.
static mut unix_auth: rpc_auth = rpc_auth {
    au_cslack: UNX_CALLSLACK, au_rslack: NUL_REPLYSLACK, au_verfsize: NUL_REPLYSLACK,
    au_ops: &authunix_ops, au_flavor: RPC_AUTH_UNIX, au_count: REFCOUNT_INIT(1),
};

static unix_credops: rpc_credops = rpc_credops {
    cr_name: c"AUTH_UNIX".as_ptr(), crdestroy: Some(unx_destroy_cred), crmatch: Some(unx_match),
    crmarshal: Some(unx_marshal), crwrap_req: Some(rpcauth_wrap_req_encode),
    crrefresh: Some(unx_refresh), crvalidate: Some(unx_validate),
    crunwrap_resp: Some(rpcauth_unwrap_resp_decode),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
