/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/dlm.h.
// The Linux tracepoint and DLM declarations referenced here are supplied by
// other translation units.

#[allow(dead_code)]
pub const SHOW_LOCK_FLAGS: &[(u32, &str)] = &[
    (DLM_LKF_NOQUEUE, "NOQUEUE"), (DLM_LKF_CANCEL, "CANCEL"),
    (DLM_LKF_CONVERT, "CONVERT"), (DLM_LKF_VALBLK, "VALBLK"),
    (DLM_LKF_QUECVT, "QUECVT"), (DLM_LKF_IVVALBLK, "IVVALBLK"),
    (DLM_LKF_CONVDEADLK, "CONVDEADLK"), (DLM_LKF_PERSISTENT, "PERSISTENT"),
    (DLM_LKF_NODLCKWT, "NODLCKWT"), (DLM_LKF_NODLCKBLK, "NODLCKBLK"),
    (DLM_LKF_EXPEDITE, "EXPEDITE"), (DLM_LKF_NOQUEUEBAST, "NOQUEUEBAST"),
    (DLM_LKF_HEADQUE, "HEADQUE"), (DLM_LKF_NOORDER, "NOORDER"),
    (DLM_LKF_ORPHAN, "ORPHAN"), (DLM_LKF_ALTPR, "ALTPR"),
    (DLM_LKF_ALTCW, "ALTCW"), (DLM_LKF_FORCEUNLOCK, "FORCEUNLOCK"),
    (DLM_LKF_TIMEOUT, "TIMEOUT"),
];
pub const SHOW_LOCK_MODE: &[(i32, &str)] = &[
    (DLM_LOCK_IV, "IV"), (DLM_LOCK_NL, "NL"), (DLM_LOCK_CR, "CR"),
    (DLM_LOCK_CW, "CW"), (DLM_LOCK_PR, "PR"), (DLM_LOCK_PW, "PW"),
    (DLM_LOCK_EX, "EX"),
];
pub const SHOW_DLM_SB_FLAGS: &[(u32, &str)] = &[
    (DLM_SBF_DEMOTED, "DEMOTED"), (DLM_SBF_VALNOTVALID, "VALNOTVALID"),
    (DLM_SBF_ALTMODE, "ALTMODE"),
];
pub const SHOW_LKB_FLAGS: &[(u32, &str)] = &[
    (1u32 << DLM_DFL_USER_BIT, "USER"),
    (1u32 << DLM_DFL_ORPHAN_BIT, "ORPHAN"),
];
pub const SHOW_HEADER_CMD: &[(u8, &str)] = &[
    (DLM_MSG, "MSG"), (DLM_RCOM, "RCOM"), (DLM_OPTS, "OPTS"),
    (DLM_ACK, "ACK"), (DLM_FIN, "FIN"),
];
pub const SHOW_MESSAGE_VERSION: &[(u32, &str)] = &[(DLM_VERSION_3_1, "3.1"), (DLM_VERSION_3_2, "3.2")];
pub const SHOW_MESSAGE_TYPE: &[(u32, &str)] = &[
    (DLM_MSG_REQUEST, "REQUEST"), (DLM_MSG_CONVERT, "CONVERT"),
    (DLM_MSG_UNLOCK, "UNLOCK"), (DLM_MSG_CANCEL, "CANCEL"),
    (DLM_MSG_REQUEST_REPLY, "REQUEST_REPLY"), (DLM_MSG_CONVERT_REPLY, "CONVERT_REPLY"),
    (DLM_MSG_UNLOCK_REPLY, "UNLOCK_REPLY"), (DLM_MSG_CANCEL_REPLY, "CANCEL_REPLY"),
    (DLM_MSG_GRANT, "GRANT"), (DLM_MSG_BAST, "BAST"), (DLM_MSG_LOOKUP, "LOOKUP"),
    (DLM_MSG_REMOVE, "REMOVE"), (DLM_MSG_LOOKUP_REPLY, "LOOKUP_REPLY"),
    (DLM_MSG_PURGE, "PURGE"),
];
pub const SHOW_RCOM_TYPE: &[(u32, &str)] = &[
    (DLM_RCOM_STATUS, "STATUS"), (DLM_RCOM_NAMES, "NAMES"),
    (DLM_RCOM_LOOKUP, "LOOKUP"), (DLM_RCOM_LOCK, "LOCK"),
    (DLM_RCOM_STATUS_REPLY, "STATUS_REPLY"), (DLM_RCOM_NAMES_REPLY, "NAMES_REPLY"),
    (DLM_RCOM_LOOKUP_REPLY, "LOOKUP_REPLY"), (DLM_RCOM_LOCK_REPLY, "LOCK_REPLY"),
];

// Tracepoint declarations.  Their TP_PROTO, TP_ARGS, TP_STRUCT__entry,
// TP_fast_assign, and TP_printk bodies are retained verbatim as Rust comments
// because their implementation is provided by the kernel tracepoint system.
//
//
// TRACE_EVENT(dlm_lock_start,
//   TP_PROTO(struct dlm_ls *ls, struct dlm_lkb *lkb, const void *name,
//            unsigned int namelen, int mode, __u32 flags));
// TRACE_EVENT(dlm_lock_end,
//   TP_PROTO(struct dlm_ls *ls, struct dlm_lkb *lkb, const void *name,
//            unsigned int namelen, int mode, __u32 flags, int error,
//            bool kernel_lock));
// TRACE_EVENT(dlm_bast,
//   TP_PROTO(__u32 ls_id, __u32 lkb_id, int mode, const char *res_name,
//            size_t res_length));
// TRACE_EVENT(dlm_ast,
//   TP_PROTO(__u32 ls_id, __u32 lkb_id, __u8 sb_flags, int sb_status,
//            const char *res_name, size_t res_length));
// TRACE_EVENT(dlm_unlock_start,
//   TP_PROTO(struct dlm_ls *ls, struct dlm_lkb *lkb, __u32 flags));
// TRACE_EVENT(dlm_unlock_end,
//   TP_PROTO(struct dlm_ls *ls, struct dlm_lkb *lkb, __u32 flags, int error));
// DECLARE_EVENT_CLASS(dlm_rcom_template,
//   TP_PROTO(uint32_t dst, uint32_t h_seq, const struct dlm_rcom *rc));
// DEFINE_EVENT(dlm_rcom_template, dlm_send_rcom, dst, h_seq, rc);
// DEFINE_EVENT(dlm_rcom_template, dlm_recv_rcom, dst, h_seq, rc);
// TRACE_EVENT(dlm_send_message,
//   TP_PROTO(uint32_t dst, uint32_t h_seq, const struct dlm_message *ms,
//            const void *name, int namelen));
// TRACE_EVENT(dlm_recv_message,
//   TP_PROTO(uint32_t dst, uint32_t h_seq, const struct dlm_message *ms));
// DECLARE_EVENT_CLASS(dlm_plock_template,
//   TP_PROTO(const struct dlm_plock_info *info));
// DEFINE_EVENT(dlm_plock_template, dlm_plock_read, info);
// DEFINE_EVENT(dlm_plock_template, dlm_plock_write, info);
// TRACE_EVENT(dlm_send, TP_PROTO(int nodeid, int ret));
// TRACE_EVENT(dlm_recv, TP_PROTO(int nodeid, int ret));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
