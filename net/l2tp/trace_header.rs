/* SPDX-License-Identifier: GPL-2.0-only */

// The source is a Linux tracepoint header.  Its declarations are consumed by
// the kernel tracepoint-generation macros; those macros have no direct Rust
// item equivalent, so their structure and field semantics are retained here.

pub const TRACE_SYSTEM: &str = "l2tp";

// External types and constants supplied by the Linux L2TP implementation:
// struct l2tp_tunnel, struct l2tp_session, L2TP_TUNNEL_NAME_MAX,
// L2TP_SESSION_NAME_MAX, enum l2tp_encap_type, enum l2tp_pwtype, and u32.

// Equivalent symbolic-name tables for encap_type_name(e) and pw_type_name(p).
// The numeric values are provided by the external L2TP definitions.
pub const ENCAP_TYPE_NAMES: &[(&str, &str)] = &[("L2TP_ENCAPTYPE_UDP", "UDP"),
                                                ("L2TP_ENCAPTYPE_IP", "IP")];
pub const PW_TYPE_NAMES: &[(&str, &str)] = &[("L2TP_PWTYPE_ETH_VLAN", "ETH_VLAN"),
                                             ("L2TP_PWTYPE_ETH", "ETH"),
                                             ("L2TP_PWTYPE_PPP", "PPP"),
                                             ("L2TP_PWTYPE_PPP_AC", "PPP_AC"),
                                             ("L2TP_PWTYPE_IP", "IP")];

// DECLARE_EVENT_CLASS(tunnel_only_evt)
// TP_PROTO(struct l2tp_tunnel *tunnel)
// TP_ARGS(tunnel)
// Entry: char name[L2TP_TUNNEL_NAME_MAX]
// Assignment: memcpy(entry.name, tunnel->name, L2TP_TUNNEL_NAME_MAX)
// Print: "%s", entry.name

// DECLARE_EVENT_CLASS(session_only_evt)
// TP_PROTO(struct l2tp_session *session)
// TP_ARGS(session)
// Entry: char name[L2TP_SESSION_NAME_MAX]
// Assignment: memcpy(entry.name, session->name, L2TP_SESSION_NAME_MAX)
// Print: "%s", entry.name

// TRACE_EVENT(register_tunnel)
// TP_PROTO(struct l2tp_tunnel *tunnel), TP_ARGS(tunnel)
// Entry: name[L2TP_TUNNEL_NAME_MAX], int fd, u32 tid, u32 ptid,
//        int version, enum l2tp_encap_type encap
// Assignment copies tunnel->name and records fd, tunnel_id, peer_tunnel_id,
// version, and encap.  Print format:
// "%s: type=%s encap=%s version=L2TPv%d tid=%u ptid=%u fd=%d", where type is
// "managed" when fd > 0 and "unmanaged" otherwise.

// DEFINE_EVENT(tunnel_only_evt, delete_tunnel)
// DEFINE_EVENT(tunnel_only_evt, free_tunnel)

// TRACE_EVENT(register_session)
// TP_PROTO(struct l2tp_session *session), TP_ARGS(session)
// Entry: name[L2TP_SESSION_NAME_MAX], u32 tid, u32 ptid, u32 sid, u32 psid,
//        enum l2tp_pwtype pwtype
// Assignment copies session->name; tid and ptid are the containing tunnel's
// tunnel_id and peer_tunnel_id, or zero when session->tunnel is null; sid,
// psid, and pwtype are session_id, peer_session_id, and pwtype.
// Print format: "%s: pseudowire=%s sid=%u psid=%u tid=%u ptid=%u".

// DEFINE_EVENT(session_only_evt, delete_session)
// DEFINE_EVENT(session_only_evt, free_session)
// DEFINE_EVENT(session_only_evt, session_seqnum_lns_enable)
// DEFINE_EVENT(session_only_evt, session_seqnum_lns_disable)

// DECLARE_EVENT_CLASS(session_seqnum_evt)
// TP_PROTO(struct l2tp_session *session), TP_ARGS(session)
// Entry: name[L2TP_SESSION_NAME_MAX], u32 ns, u32 nr
// Assignment copies session->name and records session->ns and session->nr.
// Print format: "%s: ns=%u nr=%u".

// DEFINE_EVENT(session_seqnum_evt, session_seqnum_update)
// DEFINE_EVENT(session_seqnum_evt, session_seqnum_reset)

// DECLARE_EVENT_CLASS(session_pkt_discard_evt)
// TP_PROTO(struct l2tp_session *session, u32 pkt_ns)
// TP_ARGS(session, pkt_ns)
// Entry: name[L2TP_SESSION_NAME_MAX], u32 pkt_ns, u32 my_nr,
//        u32 reorder_q_len
// Assignment copies session->name, records pkt_ns, session->nr, and
// skb_queue_len(&session->reorder_q).  Print format:
// "%s: pkt_ns=%u my_nr=%u reorder_q_len=%u".

// DEFINE_EVENT(session_pkt_discard_evt, session_pkt_expired)
// DEFINE_EVENT(session_pkt_discard_evt, session_pkt_outside_rx_window)
// DEFINE_EVENT(session_pkt_discard_evt, session_pkt_oos)

// The following source directives select the generated trace implementation:
// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
