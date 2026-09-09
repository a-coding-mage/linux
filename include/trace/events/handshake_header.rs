/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/handshake.h.
// The Linux tracepoint DSL and its dependent kernel types/macros are supplied
// by the surrounding translation unit.

// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM handshake
// Header guard: _TRACE_HANDSHAKE_H / TRACE_HEADER_MULTI_READ.
// Includes: linux/net.h, net/tls_prot.h, linux/tracepoint.h,
// trace/events/net_probe_common.h.

// TLS_RECORD_TYPE_LIST:
// CHANGE_CIPHER_SPEC, ALERT, HANDSHAKE, DATA, HEARTBEAT, TLS12_CID, ACK (end)
// The C preprocessor expands these through record_type/record_type_end into
// TRACE_DEFINE_ENUM entries and symbolic-print entries.
pub const TLS_RECORD_TYPE_NAMES: &[&str] = &[
    "CHANGE_CIPHER_SPEC", "ALERT", "HANDSHAKE", "DATA", "HEARTBEAT",
    "TLS12_CID", "ACK",
];

// TLS_ALERT_DESCRIPTION_LIST:
// CLOSE_NOTIFY, UNEXPECTED_MESSAGE, BAD_RECORD_MAC, RECORD_OVERFLOW,
// HANDSHAKE_FAILURE, BAD_CERTIFICATE, UNSUPPORTED_CERTIFICATE,
// CERTIFICATE_REVOKED, CERTIFICATE_EXPIRED, CERTIFICATE_UNKNOWN,
// ILLEGAL_PARAMETER, UNKNOWN_CA, ACCESS_DENIED, DECODE_ERROR, DECRYPT_ERROR,
// TOO_MANY_CIDS_REQUESTED, PROTOCOL_VERSION, INSUFFICIENT_SECURITY,
// INTERNAL_ERROR, INAPPROPRIATE_FALLBACK, USER_CANCELED, MISSING_EXTENSION,
// UNSUPPORTED_EXTENSION, UNRECOGNIZED_NAME, BAD_CERTIFICATE_STATUS_RESPONSE,
// UNKNOWN_PSK_IDENTITY, CERTIFICATE_REQUIRED, NO_APPLICATION_PROTOCOL (end)
pub const TLS_ALERT_DESCRIPTION_NAMES: &[&str] = &[
    "CLOSE_NOTIFY", "UNEXPECTED_MESSAGE", "BAD_RECORD_MAC", "RECORD_OVERFLOW",
    "HANDSHAKE_FAILURE", "BAD_CERTIFICATE", "UNSUPPORTED_CERTIFICATE",
    "CERTIFICATE_REVOKED", "CERTIFICATE_EXPIRED", "CERTIFICATE_UNKNOWN",
    "ILLEGAL_PARAMETER", "UNKNOWN_CA", "ACCESS_DENIED", "DECODE_ERROR",
    "DECRYPT_ERROR", "TOO_MANY_CIDS_REQUESTED", "PROTOCOL_VERSION",
    "INSUFFICIENT_SECURITY", "INTERNAL_ERROR", "INAPPROPRIATE_FALLBACK",
    "USER_CANCELED", "MISSING_EXTENSION", "UNSUPPORTED_EXTENSION",
    "UNRECOGNIZED_NAME", "BAD_CERTIFICATE_STATUS_RESPONSE",
    "UNKNOWN_PSK_IDENTITY", "CERTIFICATE_REQUIRED", "NO_APPLICATION_PROTOCOL",
];

// TRACE_DEFINE_ENUM(TLS_ALERT_LEVEL_WARNING);
// TRACE_DEFINE_ENUM(TLS_ALERT_LEVEL_FATAL);
// show_tls_content_type(type) => __print_symbolic(type, TLS_RECORD_TYPE_LIST)
// show_tls_alert_level(level) => __print_symbolic(level, Warning/Fatal)
// show_tls_alert_description(desc) => __print_symbolic(desc, TLS_ALERT_DESCRIPTION_LIST)

// The following invocations preserve the source tracepoint declarations. They
// intentionally remain dependent on the external kernel tracepoint DSL.
DECLARE_EVENT_CLASS!(handshake_event_class,
    TP_PROTO!(const struct net *net, const struct handshake_req *req, const struct sock *sk),
    TP_ARGS!(net, req, sk),
    TP_STRUCT__entry!(__field!(const void *, req), __field!(const void *, sk), __field!(unsigned int, netns_ino)),
    TP_fast_assign!(__entry->req = req; __entry->sk = sk; __entry->netns_ino = net->ns.inum;),
    TP_printk!("req=%p sk=%p", __entry->req, __entry->sk));

DECLARE_EVENT_CLASS!(handshake_fd_class,
    TP_PROTO!(const struct net *net, const struct handshake_req *req, const struct sock *sk, int fd),
    TP_ARGS!(net, req, sk, fd),
    TP_STRUCT__entry!(__field!(const void *, req), __field!(const void *, sk), __field!(int, fd), __field!(unsigned int, netns_ino)),
    TP_fast_assign!(__entry->req = req; __entry->sk = req->hr_sk; __entry->fd = fd; __entry->netns_ino = net->ns.inum;),
    TP_printk!("req=%p sk=%p fd=%d", __entry->req, __entry->sk, __entry->fd));

DECLARE_EVENT_CLASS!(handshake_error_class,
    TP_PROTO!(const struct net *net, const struct handshake_req *req, const struct sock *sk, int err),
    TP_ARGS!(net, req, sk, err),
    TP_STRUCT__entry!(__field!(const void *, req), __field!(const void *, sk), __field!(int, err), __field!(unsigned int, netns_ino)),
    TP_fast_assign!(__entry->req = req; __entry->sk = sk; __entry->err = err; __entry->netns_ino = net->ns.inum;),
    TP_printk!("req=%p sk=%p err=%d", __entry->req, __entry->sk, __entry->err));

DECLARE_EVENT_CLASS!(handshake_alert_class,
    TP_PROTO!(const struct sock *sk, unsigned char level, unsigned char description),
    TP_ARGS!(sk, level, description),
    TP_STRUCT__entry!(/* sockaddr_in6 is always bigger than sockaddr_in */
        __array!(__u8, saddr, sizeof(struct sockaddr_in6)),
        __array!(__u8, daddr, sizeof(struct sockaddr_in6)),
        __field!(unsigned int, netns_ino), __field!(unsigned long, level),
        __field!(unsigned long, description)),
    TP_fast_assign!(const struct inet_sock *inet = inet_sk(sk);
        memset(__entry->saddr, 0, sizeof(struct sockaddr_in6));
        memset(__entry->daddr, 0, sizeof(struct sockaddr_in6));
        TP_STORE_ADDR_PORTS!(__entry, inet, sk);
        __entry->netns_ino = sock_net(sk)->ns.inum;
        __entry->level = level; __entry->description = description;),
    TP_printk!("src=%pISpc dest=%pISpc %s: %s", __entry->saddr, __entry->daddr,
        show_tls_alert_level(__entry->level), show_tls_alert_description(__entry->description)));

DEFINE_HANDSHAKE_EVENT!(handshake_submit);
DEFINE_HANDSHAKE_ERROR!(handshake_submit_err);
DEFINE_HANDSHAKE_EVENT!(handshake_cancel);
DEFINE_HANDSHAKE_EVENT!(handshake_cancel_none);
DEFINE_HANDSHAKE_EVENT!(handshake_cancel_busy);
DEFINE_HANDSHAKE_EVENT!(handshake_destruct);

TRACE_EVENT!(handshake_complete,
    TP_PROTO!(const struct net *net, const struct handshake_req *req, const struct sock *sk, int status),
    TP_ARGS!(net, req, sk, status),
    TP_STRUCT__entry!(__field!(const void *, req), __field!(const void *, sk), __field!(int, status), __field!(unsigned int, netns_ino)),
    TP_fast_assign!(__entry->req = req; __entry->sk = sk; __entry->status = status; __entry->netns_ino = net->ns.inum;),
    TP_printk!("req=%p sk=%p status=%d", __entry->req, __entry->sk, __entry->status));

DEFINE_HANDSHAKE_ERROR!(handshake_notify_err);
DEFINE_HANDSHAKE_FD_EVENT!(handshake_cmd_accept);
DEFINE_HANDSHAKE_ERROR!(handshake_cmd_accept_err);
DEFINE_HANDSHAKE_FD_EVENT!(handshake_cmd_done);
DEFINE_HANDSHAKE_ERROR!(handshake_cmd_done_err);

TRACE_EVENT!(tls_contenttype,
    TP_PROTO!(const struct sock *sk, unsigned char type), TP_ARGS!(sk, type),
    TP_STRUCT__entry!(/* sockaddr_in6 is always bigger than sockaddr_in */
        __array!(__u8, saddr, sizeof(struct sockaddr_in6)),
        __array!(__u8, daddr, sizeof(struct sockaddr_in6)),
        __field!(unsigned int, netns_ino), __field!(unsigned long, type)),
    TP_fast_assign!(const struct inet_sock *inet = inet_sk(sk);
        memset(__entry->saddr, 0, sizeof(struct sockaddr_in6));
        memset(__entry->daddr, 0, sizeof(struct sockaddr_in6));
        TP_STORE_ADDR_PORTS!(__entry, inet, sk);
        __entry->netns_ino = sock_net(sk)->ns.inum; __entry->type = type;),
    TP_printk!("src=%pISpc dest=%pISpc %s", __entry->saddr, __entry->daddr,
        show_tls_content_type(__entry->type)));

DEFINE_HANDSHAKE_ALERT!(tls_alert_send);
DEFINE_HANDSHAKE_ALERT!(tls_alert_recv);

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
