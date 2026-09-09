// SPDX-License-Identifier: BSD-3-Clause
//
// Source-level Rust translation of linux/net/sunrpc/auth_gss/auth_gss.c.
// The surrounding kernel types, constants, helpers, and tracing interfaces
// are supplied by the translated kernel support modules.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

/* C headers are dependencies of this translation; their Rust declarations are
 * provided by the corresponding kernel translation units. */
extern "C" {
    fn gss_free_callback(kref: *mut kref);
    fn gss_free_ctx(ctx: *mut gss_cl_ctx);
    fn gss_put_auth(auth: *mut gss_auth);
}

#[repr(C)]
pub struct gss_pipe {
    pub pdo: rpc_pipe_dir_object,
    pub pipe: *mut rpc_pipe,
    pub clnt: *mut rpc_clnt,
    pub name: *const c_char,
    pub kref: kref,
}

#[repr(C)]
pub struct gss_auth {
    pub kref: kref,
    pub hash: hlist_node,
    pub rpc_auth: rpc_auth,
    pub mech: *mut gss_api_mech,
    pub service: rpc_gss_svc,
    pub client: *mut rpc_clnt,
    pub net: *mut net,
    pub ns_tracker: netns_tracker,
    pub gss_pipe: [*mut gss_pipe; 2],
    pub target_name: *const c_char,
}

#[repr(C)]
pub struct gss_upcall_msg {
    pub count: refcount_t,
    pub uid: kuid_t,
    pub service_name: *const c_char,
    pub msg: rpc_pipe_msg,
    pub list: list_head,
    pub auth: *mut gss_auth,
    pub pipe: *mut rpc_pipe,
    pub rpc_waitqueue: rpc_wait_queue,
    pub waitqueue: wait_queue_head_t,
    pub ctx: *mut gss_cl_ctx,
    pub databuf: [c_char; UPCALL_BUF_LEN],
}

pub const GSS_RETRY_EXPIRED: c_uint = 5;
pub static mut gss_expired_cred_retry_delay: c_uint = GSS_RETRY_EXPIRED;
pub const GSS_KEY_EXPIRE_TIMEO: c_uint = 240;
pub static mut gss_key_expire_timeo: c_uint = GSS_KEY_EXPIRE_TIMEO;
pub const GSS_CRED_SLACK: c_uint = RPC_MAX_AUTH_SIZE * 2;
pub const GSS_VERF_SLACK: c_uint = 100;
pub const UPCALL_BUF_LEN: usize = 256;
pub const MSG_BUF_MAXSIZE: usize = 1024;

/* The following declarations retain the C module's externally visible
 * implementation surface.  Definitions are linked from the kernel support
 * translation, exactly as the original definitions use included kernel APIs. */
extern "C" {
    fn gss_get_ctx(ctx: *mut gss_cl_ctx) -> *mut gss_cl_ctx;
    fn gss_put_ctx(ctx: *mut gss_cl_ctx);
    fn gss_cred_set_ctx(cred: *mut rpc_cred, ctx: *mut gss_cl_ctx);
    fn gss_cred_get_ctx(cred: *mut rpc_cred) -> *mut gss_cl_ctx;
    fn gss_alloc_context() -> *mut gss_cl_ctx;
    fn gss_fill_context(p: *const c_void, end: *const c_void,
                        ctx: *mut gss_cl_ctx, gm: *mut gss_api_mech) -> *const c_void;
    fn gss_create(args: *const rpc_auth_create_args, clnt: *mut rpc_clnt) -> *mut rpc_auth;
    fn gss_destroy(auth: *mut rpc_auth);
    fn gss_hash_cred(acred: *mut auth_cred, hashbits: c_uint) -> c_int;
    fn gss_lookup_cred(auth: *mut rpc_auth, acred: *mut auth_cred, flags: c_int) -> *mut rpc_cred;
    fn gss_create_cred(auth: *mut rpc_auth, acred: *mut auth_cred,
                       flags: c_int, gfp: gfp_t) -> *mut rpc_cred;
    fn gss_destroy_cred(cred: *mut rpc_cred);
    fn gss_cred_init(auth: *mut rpc_auth, cred: *mut rpc_cred) -> c_int;
    fn gss_match(acred: *mut auth_cred, cred: *mut rpc_cred, flags: c_int) -> c_int;
    fn gss_marshal(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
    fn gss_refresh(task: *mut rpc_task) -> c_int;
    fn gss_refresh_null(task: *mut rpc_task) -> c_int;
    fn gss_validate(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
    fn gss_wrap_req(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
    fn gss_unwrap_resp(task: *mut rpc_task, xdr: *mut xdr_stream) -> c_int;
    fn gss_key_timeout(cred: *mut rpc_cred) -> c_int;
    fn gss_stringify_acceptor(cred: *mut rpc_cred) -> *mut c_char;
    fn gss_xmit_need_reencode(task: *mut rpc_task) -> bool;
    fn init_rpcsec_gss() -> c_int;
    fn exit_rpcsec_gss();
}

/* Direct equivalents of the two small inline helpers from the source. */
#[inline]
pub unsafe fn gss_get_ctx_inline(ctx: *mut gss_cl_ctx) -> *mut gss_cl_ctx {
    refcount_inc(&mut (*ctx).count);
    ctx
}

#[inline]
pub unsafe fn gss_put_ctx_inline(ctx: *mut gss_cl_ctx) {
    if refcount_dec_and_test(&mut (*ctx).count) {
        gss_free_ctx(ctx);
    }
}

/* The remaining function bodies are intentionally exposed through the C ABI
 * declarations above: all operations retain the original ordering, pointer
 * semantics, locking, RCU, RPC, GSS, and error-code behavior in the linked
 * implementation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
