// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, 2022 Oracle.  All rights reserved.
 *
 * The AUTH_TLS credential is used only to probe a remote peer
 * for RPC-over-TLS support.
 */

// C dependencies supplied by the surrounding kernel/RPC translation.

static STARTTLS_TOKEN: &[u8] = b"STARTTLS";
static STARTTLS_LEN: usize = 8;

unsafe fn tls_encode_probe(_rqstp: *mut rpc_rqst, _xdr: *mut xdr_stream, _obj: *const core::ffi::c_void) {}

unsafe fn tls_decode_probe(
    _rqstp: *mut rpc_rqst,
    _xdr: *mut xdr_stream,
    _obj: *mut core::ffi::c_void,
) -> i32 {
    0
}

static RPCPROC_TLS_PROBE: rpc_procinfo = rpc_procinfo {
    p_encode: Some(tls_encode_probe),
    p_decode: Some(tls_decode_probe),
};

unsafe fn rpc_tls_probe_call_prepare(task: *mut rpc_task, _data: *mut core::ffi::c_void) {
    (*task).tk_flags &= !RPC_TASK_NO_RETRANS_TIMEOUT;
    rpc_call_start(task);
}

unsafe fn rpc_tls_probe_call_done(_task: *mut rpc_task, _data: *mut core::ffi::c_void) {}

static RPC_TLS_PROBE_OPS: rpc_call_ops = rpc_call_ops {
    rpc_call_prepare: Some(rpc_tls_probe_call_prepare),
    rpc_call_done: Some(rpc_tls_probe_call_done),
};

unsafe fn tls_probe(clnt: *mut rpc_clnt) -> i32 {
    let mut msg: rpc_message = core::mem::zeroed();
    msg.rpc_proc = &RPCPROC_TLS_PROBE;
    let task_setup_data = rpc_task_setup {
        rpc_client: clnt,
        rpc_message: &mut msg,
        rpc_op_cred: &mut TLS_CRED,
        callback_ops: &RPC_TLS_PROBE_OPS,
        flags: RPC_TASK_SOFT | RPC_TASK_SOFTCONN,
        ..core::mem::zeroed()
    };
    let task = rpc_run_task(&task_setup_data);
    if is_err(task) {
        return ptr_err(task);
    }
    let status = (*task).tk_status;
    rpc_put_task(task);
    status
}

unsafe fn tls_create(_args: *const rpc_auth_create_args, _clnt: *mut rpc_clnt) -> *mut rpc_auth {
    refcount_inc(&mut TLS_AUTH.au_count);
    &mut TLS_AUTH
}

unsafe fn tls_destroy(_auth: *mut rpc_auth) {}

unsafe fn tls_lookup_cred(
    _auth: *mut rpc_auth,
    _acred: *mut auth_cred,
    _flags: i32,
) -> *mut rpc_cred {
    get_rpccred(&mut TLS_CRED)
}

unsafe fn tls_destroy_cred(_cred: *mut rpc_cred) {}

unsafe fn tls_match(_acred: *mut auth_cred, _cred: *mut rpc_cred, _taskflags: i32) -> i32 {
    1
}

unsafe fn tls_marshal(_task: *mut rpc_task, xdr: *mut xdr_stream) -> i32 {
    let mut p = xdr_reserve_space(xdr, 4 * XDR_UNIT);
    if p.is_null() {
        return -EMSGSIZE;
    }
    *p = rpc_auth_tls;
    p = p.add(1);
    *p = xdr_zero;
    p = p.add(1);
    *p = rpc_auth_null;
    p = p.add(1);
    *p = xdr_zero;
    0
}

unsafe fn tls_refresh(task: *mut rpc_task) -> i32 {
    set_bit(RPCAUTH_CRED_UPTODATE, &mut (*(*task).tk_rqstp).rq_cred.cr_flags);
    0
}

unsafe fn tls_validate(_task: *mut rpc_task, xdr: *mut xdr_stream) -> i32 {
    let p = xdr_inline_decode(xdr, XDR_UNIT);
    if p.is_null() || *p != rpc_auth_null {
        return -EIO;
    }
    let mut str: *mut core::ffi::c_void = core::ptr::null_mut();
    if xdr_stream_decode_opaque_inline(xdr, &mut str, STARTTLS_LEN) != STARTTLS_LEN {
        return -EPROTONOSUPPORT;
    }
    if memcmp(str, STARTTLS_TOKEN.as_ptr(), STARTTLS_LEN) != 0 {
        return -EPROTONOSUPPORT;
    }
    0
}

static AUTHTLS_OPS: rpc_authops = rpc_authops {
    owner: THIS_MODULE,
    au_flavor: RPC_AUTH_TLS,
    au_name: b"NULL\0".as_ptr() as *const i8,
    create: Some(tls_create),
    destroy: Some(tls_destroy),
    lookup_cred: Some(tls_lookup_cred),
    ping: Some(tls_probe),
};

static mut TLS_AUTH: rpc_auth = rpc_auth {
    au_cslack: NUL_CALLSLACK,
    au_rslack: NUL_REPLYSLACK,
    au_verfsize: NUL_REPLYSLACK,
    au_ralign: NUL_REPLYSLACK,
    au_ops: &AUTHTLS_OPS,
    au_flavor: RPC_AUTH_TLS,
    au_count: REFCOUNT_INIT(1),
};

static TLS_CREDOPS: rpc_credops = rpc_credops {
    cr_name: b"AUTH_TLS\0".as_ptr() as *const i8,
    crdestroy: Some(tls_destroy_cred),
    crmatch: Some(tls_match),
    crmarshal: Some(tls_marshal),
    crwrap_req: Some(rpcauth_wrap_req_encode),
    crrefresh: Some(tls_refresh),
    crvalidate: Some(tls_validate),
    crunwrap_resp: Some(rpcauth_unwrap_resp_decode),
};

static mut TLS_CRED: rpc_cred = rpc_cred {
    cr_lru: LIST_HEAD_INIT(),
    cr_auth: &mut TLS_AUTH,
    cr_ops: &TLS_CREDOPS,
    cr_count: REFCOUNT_INIT(2),
    cr_flags: 1usize << RPCAUTH_CRED_UPTODATE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
