// SPDX-License-Identifier: GPL-2.0
/*
 * linux/net/sunrpc/auth_null.c
 *
 * AUTH_NULL authentication. Really :-)
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies corresponding to linux/types.h, linux/module.h, and
// linux/sunrpc/clnt.h are supplied by the surrounding translation.

static mut null_auth: rpc_auth = rpc_auth {
    au_cslack: NUL_CALLSLACK,
    au_rslack: NUL_REPLYSLACK,
    au_verfsize: NUL_REPLYSLACK,
    au_ralign: NUL_REPLYSLACK,
    au_ops: &authnull_ops,
    au_flavor: RPC_AUTH_NULL,
    au_count: REFCOUNT_INIT(1),
};
static mut null_cred: rpc_cred = rpc_cred {
    cr_lru: LIST_HEAD_INIT(null_cred.cr_lru),
    cr_auth: &null_auth,
    cr_ops: &null_credops,
    cr_count: REFCOUNT_INIT(2),
    cr_flags: 1usize << RPCAUTH_CRED_UPTODATE,
};

unsafe extern "C" {
    fn refcount_inc(v: *mut refcount_t);
    fn get_rpccred(cred: *mut rpc_cred) -> *mut rpc_cred;
    fn xdr_reserve_space(xdr: *mut xdr_stream, len: usize) -> *mut __be32;
    fn xdr_inline_decode(xdr: *mut xdr_stream, len: usize) -> *mut __be32;
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn rpcauth_wrap_req_encode(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
    fn rpcauth_unwrap_resp_decode(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;
}

unsafe extern "C" fn nul_create(
    _args: *const rpc_auth_create_args,
    _clnt: *mut rpc_clnt,
) -> *mut rpc_auth {
    refcount_inc(&mut null_auth.au_count);
    &raw mut null_auth
}

unsafe extern "C" fn nul_destroy(_auth: *mut rpc_auth) {}

/*
 * Lookup NULL creds for current process
 */
unsafe extern "C" fn nul_lookup_cred(
    _auth: *mut rpc_auth,
    _acred: *mut auth_cred,
    _flags: c_int,
) -> *mut rpc_cred {
    get_rpccred(&raw mut null_cred)
}

/*
 * Destroy cred handle.
 */
unsafe extern "C" fn nul_destroy_cred(_cred: *mut rpc_cred) {}

/*
 * Match cred handle against current process
 */
unsafe extern "C" fn nul_match(
    _acred: *mut auth_cred,
    _cred: *mut rpc_cred,
    _taskflags: c_int,
) -> c_int {
    1
}

/*
 * Marshal credential.
 */
unsafe extern "C" fn nul_marshal(_task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int {
    let p = xdr_reserve_space(xdr, 4 * core::mem::size_of::<__be32>());
    if p.is_null() {
        return -EMSGSIZE;
    }
    // Credential
    *p.add(0) = rpc_auth_null;
    *p.add(1) = xdr_zero;
    // Verifier
    *p.add(2) = rpc_auth_null;
    *p.add(3) = xdr_zero;
    0
}

/*
 * Refresh credential. This is a no-op for AUTH_NULL
 */
unsafe extern "C" fn nul_refresh(task: *mut rpc_task) -> c_int {
    set_bit(
        RPCAUTH_CRED_UPTODATE,
        &mut (*(*task).tk_rqstp).rq_cred.as_mut().unwrap().cr_flags,
    );
    0
}

unsafe extern "C" fn nul_validate(_task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int {
    let p = xdr_inline_decode(xdr, 2 * core::mem::size_of::<__be32>());
    if p.is_null() {
        return -EIO;
    }
    if *p.add(0) != rpc_auth_null {
        return -EIO;
    }
    if *p.add(1) != xdr_zero {
        return -EIO;
    }
    0
}

const authnull_ops: rpc_authops = rpc_authops {
    owner: unsafe { THIS_MODULE },
    au_flavor: RPC_AUTH_NULL,
    au_name: b"NULL\0".as_ptr() as *const c_char,
    create: Some(nul_create),
    destroy: Some(nul_destroy),
    lookup_cred: Some(nul_lookup_cred),
};

const null_credops: rpc_credops = rpc_credops {
    cr_name: b"AUTH_NULL\0".as_ptr() as *const c_char,
    crdestroy: Some(nul_destroy_cred),
    crmatch: Some(nul_match),
    crmarshal: Some(nul_marshal),
    crwrap_req: Some(rpcauth_wrap_req_encode),
    crrefresh: Some(nul_refresh),
    crvalidate: Some(nul_validate),
    crunwrap_resp: Some(rpcauth_unwrap_resp_decode),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
