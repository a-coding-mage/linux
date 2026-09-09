/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Dependency: <linux/netfilter/nfnetlink.h>

#[repr(u32)]
pub enum cntl_msg_types {
    IPCTNL_MSG_CT_NEW,
    IPCTNL_MSG_CT_GET,
    IPCTNL_MSG_CT_DELETE,
    IPCTNL_MSG_CT_GET_CTRZERO,
    IPCTNL_MSG_CT_GET_STATS_CPU,
    IPCTNL_MSG_CT_GET_STATS,
    IPCTNL_MSG_CT_GET_DYING,
    IPCTNL_MSG_CT_GET_UNCONFIRMED,
    IPCTNL_MSG_MAX,
}

#[repr(u32)]
pub enum ctnl_exp_msg_types {
    IPCTNL_MSG_EXP_NEW,
    IPCTNL_MSG_EXP_GET,
    IPCTNL_MSG_EXP_DELETE,
    IPCTNL_MSG_EXP_GET_STATS_CPU,
    IPCTNL_MSG_EXP_MAX,
}

#[repr(u32)]
pub enum ctattr_type {
    CTA_UNSPEC,
    CTA_TUPLE_ORIG,
    CTA_TUPLE_REPLY,
    CTA_STATUS,
    CTA_PROTOINFO,
    CTA_HELP,
    CTA_NAT_SRC,
    CTA_TIMEOUT,
    CTA_MARK,
    CTA_COUNTERS_ORIG,
    CTA_COUNTERS_REPLY,
    CTA_USE,
    CTA_ID,
    CTA_NAT_DST,
    CTA_TUPLE_MASTER,
    CTA_SEQ_ADJ_ORIG,
    CTA_SEQ_ADJ_REPLY,
    CTA_SECMARK, // obsolete
    CTA_ZONE,
    CTA_SECCTX,
    CTA_TIMESTAMP,
    CTA_MARK_MASK,
    CTA_LABELS,
    CTA_LABELS_MASK,
    CTA_SYNPROXY,
    CTA_FILTER,
    CTA_STATUS_MASK,
    CTA_TIMESTAMP_EVENT,
    __CTA_MAX,
}
pub const CTA_NAT: u32 = ctattr_type::CTA_NAT_SRC as u32; // backwards compatibility
pub const CTA_NAT_SEQ_ADJ_ORIG: u32 = ctattr_type::CTA_SEQ_ADJ_ORIG as u32;
pub const CTA_NAT_SEQ_ADJ_REPLY: u32 = ctattr_type::CTA_SEQ_ADJ_REPLY as u32;
pub const CTA_MAX: u32 = (ctattr_type::__CTA_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_tuple { CTA_TUPLE_UNSPEC, CTA_TUPLE_IP, CTA_TUPLE_PROTO, CTA_TUPLE_ZONE, __CTA_TUPLE_MAX }
pub const CTA_TUPLE_MAX: u32 = (ctattr_tuple::__CTA_TUPLE_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_ip { CTA_IP_UNSPEC, CTA_IP_V4_SRC, CTA_IP_V4_DST, CTA_IP_V6_SRC, CTA_IP_V6_DST, __CTA_IP_MAX }
pub const CTA_IP_MAX: u32 = (ctattr_ip::__CTA_IP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_l4proto {
    CTA_PROTO_UNSPEC, CTA_PROTO_NUM, CTA_PROTO_SRC_PORT, CTA_PROTO_DST_PORT,
    CTA_PROTO_ICMP_ID, CTA_PROTO_ICMP_TYPE, CTA_PROTO_ICMP_CODE,
    CTA_PROTO_ICMPV6_ID, CTA_PROTO_ICMPV6_TYPE, CTA_PROTO_ICMPV6_CODE, __CTA_PROTO_MAX,
}
pub const CTA_PROTO_MAX: u32 = (ctattr_l4proto::__CTA_PROTO_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_protoinfo { CTA_PROTOINFO_UNSPEC, CTA_PROTOINFO_TCP, CTA_PROTOINFO_DCCP, CTA_PROTOINFO_SCTP, __CTA_PROTOINFO_MAX }
pub const CTA_PROTOINFO_MAX: u32 = (ctattr_protoinfo::__CTA_PROTOINFO_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_protoinfo_tcp {
    CTA_PROTOINFO_TCP_UNSPEC, CTA_PROTOINFO_TCP_STATE, CTA_PROTOINFO_TCP_WSCALE_ORIGINAL,
    CTA_PROTOINFO_TCP_WSCALE_REPLY, CTA_PROTOINFO_TCP_FLAGS_ORIGINAL, CTA_PROTOINFO_TCP_FLAGS_REPLY,
    __CTA_PROTOINFO_TCP_MAX,
}
pub const CTA_PROTOINFO_TCP_MAX: u32 = (ctattr_protoinfo_tcp::__CTA_PROTOINFO_TCP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_protoinfo_dccp {
    CTA_PROTOINFO_DCCP_UNSPEC, CTA_PROTOINFO_DCCP_STATE, CTA_PROTOINFO_DCCP_ROLE,
    CTA_PROTOINFO_DCCP_HANDSHAKE_SEQ, CTA_PROTOINFO_DCCP_PAD, __CTA_PROTOINFO_DCCP_MAX,
}
pub const CTA_PROTOINFO_DCCP_MAX: u32 = (ctattr_protoinfo_dccp::__CTA_PROTOINFO_DCCP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_protoinfo_sctp {
    CTA_PROTOINFO_SCTP_UNSPEC, CTA_PROTOINFO_SCTP_STATE, CTA_PROTOINFO_SCTP_VTAG_ORIGINAL,
    CTA_PROTOINFO_SCTP_VTAG_REPLY, __CTA_PROTOINFO_SCTP_MAX,
}
pub const CTA_PROTOINFO_SCTP_MAX: u32 = (ctattr_protoinfo_sctp::__CTA_PROTOINFO_SCTP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_counters {
    CTA_COUNTERS_UNSPEC,
    CTA_COUNTERS_PACKETS, // 64bit counters
    CTA_COUNTERS_BYTES, // 64bit counters
    CTA_COUNTERS32_PACKETS, // old 32bit counters, unused
    CTA_COUNTERS32_BYTES, // old 32bit counters, unused
    CTA_COUNTERS_PAD,
    __CTA_COUNTERS_MAX,
}
pub const CTA_COUNTERS_MAX: u32 = (ctattr_counters::__CTA_COUNTERS_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_tstamp { CTA_TIMESTAMP_UNSPEC, CTA_TIMESTAMP_START, CTA_TIMESTAMP_STOP, CTA_TIMESTAMP_PAD, __CTA_TIMESTAMP_MAX }
pub const CTA_TIMESTAMP_MAX: u32 = (ctattr_tstamp::__CTA_TIMESTAMP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_nat {
    CTA_NAT_UNSPEC, CTA_NAT_V4_MINIP, CTA_NAT_V4_MAXIP, CTA_NAT_PROTO,
    CTA_NAT_V6_MINIP, CTA_NAT_V6_MAXIP, __CTA_NAT_MAX,
}
pub const CTA_NAT_MINIP: u32 = ctattr_nat::CTA_NAT_V4_MINIP as u32;
pub const CTA_NAT_MAXIP: u32 = ctattr_nat::CTA_NAT_V4_MAXIP as u32;
pub const CTA_NAT_MAX: u32 = (ctattr_nat::__CTA_NAT_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_protonat { CTA_PROTONAT_UNSPEC, CTA_PROTONAT_PORT_MIN, CTA_PROTONAT_PORT_MAX, __CTA_PROTONAT_MAX }
pub const CTA_PROTONAT_MAX: u32 = (ctattr_protonat::__CTA_PROTONAT_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_seqadj { CTA_SEQADJ_UNSPEC, CTA_SEQADJ_CORRECTION_POS, CTA_SEQADJ_OFFSET_BEFORE, CTA_SEQADJ_OFFSET_AFTER, __CTA_SEQADJ_MAX }
pub const CTA_SEQADJ_MAX: u32 = (ctattr_seqadj::__CTA_SEQADJ_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_natseq { CTA_NAT_SEQ_UNSPEC, CTA_NAT_SEQ_CORRECTION_POS, CTA_NAT_SEQ_OFFSET_BEFORE, CTA_NAT_SEQ_OFFSET_AFTER, __CTA_NAT_SEQ_MAX }
pub const CTA_NAT_SEQ_MAX: u32 = (ctattr_natseq::__CTA_NAT_SEQ_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_synproxy { CTA_SYNPROXY_UNSPEC, CTA_SYNPROXY_ISN, CTA_SYNPROXY_ITS, CTA_SYNPROXY_TSOFF, __CTA_SYNPROXY_MAX }
pub const CTA_SYNPROXY_MAX: u32 = (ctattr_synproxy::__CTA_SYNPROXY_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_expect {
    CTA_EXPECT_UNSPEC, CTA_EXPECT_MASTER, CTA_EXPECT_TUPLE, CTA_EXPECT_MASK, CTA_EXPECT_TIMEOUT,
    CTA_EXPECT_ID, CTA_EXPECT_HELP_NAME, CTA_EXPECT_ZONE, CTA_EXPECT_FLAGS, CTA_EXPECT_CLASS,
    CTA_EXPECT_NAT, CTA_EXPECT_FN, __CTA_EXPECT_MAX,
}
pub const CTA_EXPECT_MAX: u32 = (ctattr_expect::__CTA_EXPECT_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_expect_nat { CTA_EXPECT_NAT_UNSPEC, CTA_EXPECT_NAT_DIR, CTA_EXPECT_NAT_TUPLE, __CTA_EXPECT_NAT_MAX }
pub const CTA_EXPECT_NAT_MAX: u32 = (ctattr_expect_nat::__CTA_EXPECT_NAT_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_help { CTA_HELP_UNSPEC, CTA_HELP_NAME, CTA_HELP_INFO, __CTA_HELP_MAX }
pub const CTA_HELP_MAX: u32 = (ctattr_help::__CTA_HELP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_secctx { CTA_SECCTX_UNSPEC, CTA_SECCTX_NAME, __CTA_SECCTX_MAX }
pub const CTA_SECCTX_MAX: u32 = (ctattr_secctx::__CTA_SECCTX_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_stats_cpu {
    CTA_STATS_UNSPEC, CTA_STATS_SEARCHED, CTA_STATS_FOUND, CTA_STATS_NEW, CTA_STATS_INVALID,
    CTA_STATS_IGNORE, CTA_STATS_DELETE, CTA_STATS_DELETE_LIST, CTA_STATS_INSERT,
    CTA_STATS_INSERT_FAILED, CTA_STATS_DROP, CTA_STATS_EARLY_DROP, CTA_STATS_ERROR,
    CTA_STATS_SEARCH_RESTART, CTA_STATS_CLASH_RESOLVE, CTA_STATS_CHAIN_TOOLONG, __CTA_STATS_MAX,
}
pub const CTA_STATS_MAX: u32 = (ctattr_stats_cpu::__CTA_STATS_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_stats_global { CTA_STATS_GLOBAL_UNSPEC, CTA_STATS_GLOBAL_ENTRIES, CTA_STATS_GLOBAL_MAX_ENTRIES, __CTA_STATS_GLOBAL_MAX }
pub const CTA_STATS_GLOBAL_MAX: u32 = (ctattr_stats_global::__CTA_STATS_GLOBAL_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_expect_stats { CTA_STATS_EXP_UNSPEC, CTA_STATS_EXP_NEW, CTA_STATS_EXP_CREATE, CTA_STATS_EXP_DELETE, __CTA_STATS_EXP_MAX }
pub const CTA_STATS_EXP_MAX: u32 = (ctattr_expect_stats::__CTA_STATS_EXP_MAX as u32) - 1;

#[repr(u32)]
pub enum ctattr_filter { CTA_FILTER_UNSPEC, CTA_FILTER_ORIG_FLAGS, CTA_FILTER_REPLY_FLAGS, __CTA_FILTER_MAX }
pub const CTA_FILTER_MAX: u32 = (ctattr_filter::__CTA_FILTER_MAX as u32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
