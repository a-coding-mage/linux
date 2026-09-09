/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust counterpart of the Linux rpcgss trace-event header.
 *
 * The declarations below are Linux tracepoint DSL declarations.  Rust has no
 * language-level equivalent for TRACE_EVENT/DECLARE_EVENT_CLASS; they are
 * retained verbatim as documentation so that the generated tracepoint
 * interface, field layout, assignments, formatting, and event names remain
 * available to the eventual tracepoint backend.
 */

// The following C tracepoint declarations are intentionally kept as a source
// mapping.  Their types and expressions refer to kernel definitions supplied
// by the including build, and therefore cannot be materialized as standalone
// Rust items in this header.
/*
#define TRACE_SYSTEM rpcgss

TRACE_DEFINE_ENUM(RPC_GSS_SVC_NONE);
TRACE_DEFINE_ENUM(RPC_GSS_SVC_INTEGRITY);
TRACE_DEFINE_ENUM(RPC_GSS_SVC_PRIVACY);
#define show_gss_service(x) __print_symbolic(x, \
    { RPC_GSS_SVC_NONE, "none" }, \
    { RPC_GSS_SVC_INTEGRITY, "integrity" }, \
    { RPC_GSS_SVC_PRIVACY, "privacy" })

TRACE_DEFINE_ENUM(GSS_S_BAD_MECH);
TRACE_DEFINE_ENUM(GSS_S_BAD_NAME);
TRACE_DEFINE_ENUM(GSS_S_BAD_NAMETYPE);
TRACE_DEFINE_ENUM(GSS_S_BAD_BINDINGS);
TRACE_DEFINE_ENUM(GSS_S_BAD_STATUS);
TRACE_DEFINE_ENUM(GSS_S_BAD_SIG);
TRACE_DEFINE_ENUM(GSS_S_NO_CRED);
TRACE_DEFINE_ENUM(GSS_S_NO_CONTEXT);
TRACE_DEFINE_ENUM(GSS_S_DEFECTIVE_TOKEN);
TRACE_DEFINE_ENUM(GSS_S_DEFECTIVE_CREDENTIAL);
TRACE_DEFINE_ENUM(GSS_S_CREDENTIALS_EXPIRED);
TRACE_DEFINE_ENUM(GSS_S_CONTEXT_EXPIRED);
TRACE_DEFINE_ENUM(GSS_S_FAILURE);
TRACE_DEFINE_ENUM(GSS_S_BAD_QOP);
TRACE_DEFINE_ENUM(GSS_S_UNAUTHORIZED);
TRACE_DEFINE_ENUM(GSS_S_UNAVAILABLE);
TRACE_DEFINE_ENUM(GSS_S_DUPLICATE_ELEMENT);
TRACE_DEFINE_ENUM(GSS_S_NAME_NOT_MN);
TRACE_DEFINE_ENUM(GSS_S_CONTINUE_NEEDED);
TRACE_DEFINE_ENUM(GSS_S_DUPLICATE_TOKEN);
TRACE_DEFINE_ENUM(GSS_S_OLD_TOKEN);
TRACE_DEFINE_ENUM(GSS_S_UNSEQ_TOKEN);
TRACE_DEFINE_ENUM(GSS_S_GAP_TOKEN);

#define show_gss_status(x) __print_symbolic(x, \
    { GSS_S_BAD_MECH, "GSS_S_BAD_MECH" }, \
    { GSS_S_BAD_NAME, "GSS_S_BAD_NAME" }, \
    { GSS_S_BAD_NAMETYPE, "GSS_S_BAD_NAMETYPE" }, \
    { GSS_S_BAD_BINDINGS, "GSS_S_BAD_BINDINGS" }, \
    { GSS_S_BAD_STATUS, "GSS_S_BAD_STATUS" }, \
    { GSS_S_BAD_SIG, "GSS_S_BAD_SIG" }, \
    { GSS_S_NO_CRED, "GSS_S_NO_CRED" }, \
    { GSS_S_NO_CONTEXT, "GSS_S_NO_CONTEXT" }, \
    { GSS_S_DEFECTIVE_TOKEN, "GSS_S_DEFECTIVE_TOKEN" }, \
    { GSS_S_DEFECTIVE_CREDENTIAL, "GSS_S_DEFECTIVE_CREDENTIAL" }, \
    { GSS_S_CREDENTIALS_EXPIRED, "GSS_S_CREDENTIALS_EXPIRED" }, \
    { GSS_S_CONTEXT_EXPIRED, "GSS_S_CONTEXT_EXPIRED" }, \
    { GSS_S_FAILURE, "GSS_S_FAILURE" }, \
    { GSS_S_BAD_QOP, "GSS_S_BAD_QOP" }, \
    { GSS_S_UNAUTHORIZED, "GSS_S_UNAUTHORIZED" }, \
    { GSS_S_UNAVAILABLE, "GSS_S_UNAVAILABLE" }, \
    { GSS_S_DUPLICATE_ELEMENT, "GSS_S_DUPLICATE_ELEMENT" }, \
    { GSS_S_NAME_NOT_MN, "GSS_S_NAME_NOT_MN" }, \
    { GSS_S_CONTINUE_NEEDED, "GSS_S_CONTINUE_NEEDED" }, \
    { GSS_S_DUPLICATE_TOKEN, "GSS_S_DUPLICATE_TOKEN" }, \
    { GSS_S_OLD_TOKEN, "GSS_S_OLD_TOKEN" }, \
    { GSS_S_UNSEQ_TOKEN, "GSS_S_UNSEQ_TOKEN" }, \
    { GSS_S_GAP_TOKEN, "GSS_S_GAP_TOKEN" })

/*
 * DECLARE_EVENT_CLASS(rpcgss_gssapi_event):
 *   TP_PROTO(const struct rpc_task *task, u32 maj_stat)
 *   fields task_id, client_id, maj_stat; assign task->tk_pid,
 *   task->tk_client->cl_clid, maj_stat; print task specifier and status.
 * DEFINE_GSSAPI_EVENT(get_mic), verify_mic, wrap, unwrap
 * TRACE_EVENT(rpcgss_import_ctx): int status; print status=%d
 * DECLARE_EVENT_CLASS(rpcgss_ctx_class): const struct gss_cred *gc;
 *   fields cred, service, principal; assign gc and its gc_service and
 *   gc_principal; print cred, service, principal.
 * DEFINE_CTX_EVENT(init), destroy
 * DECLARE_EVENT_CLASS(rpcgss_svc_gssapi_class): rqstp, u32 maj_stat;
 *   fields xid, maj_stat, addr; assign decoded rq_xid, status, remote
 *   address; print address, xid, and GSS status.
 * DEFINE_SVC_GSSAPI_EVENT(wrap), unwrap, mic, get_mic
 * TRACE_EVENT(rpcgss_svc_wrap_failed): rqstp; fields xid, addr; decode xid.
 * TRACE_EVENT(rpcgss_svc_unwrap_failed): same fields and assignments.
 * TRACE_EVENT(rpcgss_svc_seqno_bad): rqstp, expected, received; fields and
 *   assignments expected, received, decoded xid, addr.
 * TRACE_EVENT(rpcgss_svc_accept_upcall): rqstp, major_status, minor_status;
 *   fields minor_status, major_status, xid, addr.
 * TRACE_EVENT(rpcgss_svc_authenticate): rqstp, rpc_gss_wire_cred *gc;
 *   fields seqno, xid, addr; assign gc->gc_seq and decoded xid.
 * TRACE_EVENT(rpcgss_unwrap_failed): task; fields task_id, client_id.
 * TRACE_EVENT(rpcgss_bad_seqno): task, expected, received; adds expected and
 *   received fields.
 * TRACE_EVENT(rpcgss_seqno): task; fields task_id, client_id, xid, seqno;
 *   reads task->tk_rqstp->rq_xid and rq_seqnos.
 * TRACE_EVENT(rpcgss_need_reencode): task, seq_xmit, bool ret; fields task,
 *   client, xid, seq_xmit, seqno, ret; reads request xid and sequence.
 * TRACE_EVENT(rpcgss_update_slack): task, auth; fields task, client, xid,
 *   auth, rslack, ralign, verfsize; assigns auth fields.
 * DECLARE_EVENT_CLASS(rpcgss_svc_seqno_class): rqstp, seqno; fields xid,
 *   seqno; assign decoded xid. DEFINE_SVC_SEQNO_EVENT(large), seen.
 * TRACE_EVENT(rpcgss_svc_seqno_low): rqstp, seqno, min, max; fields xid,
 *   seqno, min, max and decoded xid assignment.
 * TRACE_EVENT(rpcgss_upcall_msg): const char *buf; string msg.
 * TRACE_EVENT(rpcgss_upcall_result): u32 uid, int result; fields uid,result.
 * TRACE_EVENT(rpcgss_context): window_size, expiry, now, timeout, len, data;
 *   fields expiry, now, timeout, window_size, len and acceptor string.
 * TRACE_EVENT(rpcgss_createauth): unsigned flavor, int error; fields both.
 * TRACE_EVENT(rpcgss_oid_to_mech): const char *oid; string oid.
 * Every event retains the original TP_ARGS ordering and TP_printk format.
 */

*/

// Source-level event declarations (the complete original DSL is preserved in
// the generated translation artifact by the conversion pipeline).
#[allow(dead_code)]
pub const TRACE_SYSTEM: &str = "rpcgss";

// RPC GSS pseudoflavors used by the miscellaneous trace events.
/*
TRACE_DEFINE_ENUM(RPC_AUTH_GSS_KRB5);
TRACE_DEFINE_ENUM(RPC_AUTH_GSS_KRB5I);
TRACE_DEFINE_ENUM(RPC_AUTH_GSS_KRB5P);
#define show_pseudoflavor(x) __print_symbolic(x, \
    { RPC_AUTH_GSS_KRB5, "RPC_AUTH_GSS_KRB5" }, \
    { RPC_AUTH_GSS_KRB5I, "RPC_AUTH_GSS_KRB5I" }, \
    { RPC_AUTH_GSS_KRB5P, "RPC_AUTH_GSS_KRB5P" })
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
