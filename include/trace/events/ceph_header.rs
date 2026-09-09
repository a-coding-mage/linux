/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Ceph filesystem support module tracepoints
 *
 * Copyright (C) 2025 IONOS SE. All Rights Reserved.
 * Written by Max Kellermann (max.kellermann@ionos.com)
 */

// The Linux tracepoint framework and the structures referenced by these
// declarations are supplied by other translation units.

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CephMdscSuspendReason {
    NoMdsmap = 0,
    NoActiveMds = 1,
    Rejected = 2,
    Session = 3,
}

pub const CEPH_MDSC_SUSPEND_REASONS: &[(CephMdscSuspendReason, &str)] = &[
    (CephMdscSuspendReason::NoMdsmap, "no-mdsmap"),
    (CephMdscSuspendReason::NoActiveMds, "no-active-mds"),
    (CephMdscSuspendReason::Rejected, "rejected"),
    (CephMdscSuspendReason::Session, "session"),
];

/*
 * Tracepoint declarations translated from TRACE_EVENT.  Their bodies retain
 * the original field layout, assignments, control flow, and print formats;
 * TRACE_EVENT, TP_PROTO, TP_ARGS, TP_STRUCT__entry, TP_fast_assign, and
 * TP_printk are Linux tracepoint-framework constructs external to this file.
 */

// ceph_mdsc_submit_request(mdsc: *mut ceph_mds_client, req: *mut ceph_mds_request)
// fields: tid: u64, op: i32, ino: u64, snap: u64
// assign:
//   tid = req->r_tid; op = req->r_op;
//   inode = req->r_inode;
//   if inode == null && req->r_dentry != null { inode = d_inode(req->r_dentry); }
//   if inode != null { ino = ceph_ino(inode); snap = ceph_snap(inode); }
//   else { ino = 0; snap = 0; }
// print: "R=%llu op=%s ino=%llx,%llx", tid, ceph_mds_op_name(op), ino, snap

// ceph_mdsc_suspend_request(mdsc: *mut ceph_mds_client,
//     session: *mut ceph_mds_session, req: *mut ceph_mds_request,
//     reason: CephMdscSuspendReason)
// fields: tid: u64, op: i32, mds: i32, reason: CephMdscSuspendReason
// assign: tid = req->r_tid; op = req->r_op;
//         mds = if session != null { session->s_mds } else { -1 }; reason = reason
// print: "R=%llu op=%s reason=%s", tid, ceph_mds_op_name(op),
//        __print_symbolic(reason, CEPH_MDSC_SUSPEND_REASONS)

// ceph_mdsc_resume_request(mdsc: *mut ceph_mds_client, req: *mut ceph_mds_request)
// fields: tid: u64, op: i32
// assign: tid = req->r_tid; op = req->r_op
// print: "R=%llu op=%s", tid, ceph_mds_op_name(op)

// ceph_mdsc_send_request(session: *mut ceph_mds_session, req: *mut ceph_mds_request)
// fields: tid: u64, op: i32, mds: i32
// assign: tid = req->r_tid; op = req->r_op; mds = session->s_mds
// print: "R=%llu op=%s mds=%d", tid, ceph_mds_op_name(op), mds

// ceph_mdsc_complete_request(mdsc: *mut ceph_mds_client, req: *mut ceph_mds_request)
// fields: tid: u64, op: i32, err: i32, latency_ns: c_ulong
// assign: tid = req->r_tid; op = req->r_op; err = req->r_err;
//         latency_ns = req->r_end_latency - req->r_start_latency
// print: "R=%llu op=%s err=%d latency_ns=%lu", tid, ceph_mds_op_name(op), err, latency_ns

// ceph_handle_caps(mdsc: *mut ceph_mds_client, session: *mut ceph_mds_session,
//     op: i32, vino: *const ceph_vino, inode: *mut ceph_inode_info,
//     seq: u32, mseq: u32, issue_seq: u32)
// fields: mds: i32, op: i32, ino: u64, snap: u64, seq: u32, mseq: u32, issue_seq: u32
// assign: mds = session->s_mds; op = op; ino = vino->ino; snap = vino->snap;
//         seq = seq; mseq = mseq; issue_seq = issue_seq
// print: "mds=%d op=%s vino=%llx.%llx seq=%u iseq=%u mseq=%u", mds,
//        ceph_cap_op_name(op), ino, snap, seq, issue_seq, mseq

// Client reset tracepoints identify the client by its monitor-assigned global_id.
// client_id = if mdsc->fsc->client->monc.auth != null {
//     mdsc->fsc->client->monc.auth->global_id
// } else { 0 };
// ceph_client_reset_schedule(mdsc: *const ceph_mds_client, reason: *const c_char)
//   fields: client_id: u64, reason: String; reason is reason != null ? reason : ""
//   print: "client_id=%llu reason=%s", client_id, reason
// ceph_client_reset_complete(mdsc: *const ceph_mds_client, ret: i32)
//   fields: client_id: u64, ret: i32; print: "client_id=%llu ret=%d", client_id, ret
// ceph_client_reset_blocked(mdsc: *const ceph_mds_client, blocked_count: i32)
//   fields: client_id: u64, blocked_count: i32
//   print: "client_id=%llu blocked_count=%d", client_id, blocked_count
// ceph_client_reset_unblocked(mdsc: *const ceph_mds_client, ret: i32)
//   fields: client_id: u64, ret: i32; print: "client_id=%llu ret=%d", client_id, ret

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
