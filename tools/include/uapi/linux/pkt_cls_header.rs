/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from include/uapi/linux/pkt_cls.h. */
/* Depends on Linux UAPI integer aliases such as __u32, __u64, __be16, __be32,
 * and on struct tc_ratespec from linux/pkt_sched.h.
 */

pub const TC_COOKIE_MAX_SIZE: usize = 16;

/* Action attributes */
pub const TCA_ACT_UNSPEC: u32 = 0;
pub const TCA_ACT_KIND: u32 = 1;
pub const TCA_ACT_OPTIONS: u32 = 2;
pub const TCA_ACT_INDEX: u32 = 3;
pub const TCA_ACT_STATS: u32 = 4;
pub const TCA_ACT_PAD: u32 = 5;
pub const TCA_ACT_COOKIE: u32 = 6;
pub const __TCA_ACT_MAX: u32 = 7;

pub const TCA_ACT_MAX: u32 = __TCA_ACT_MAX;
pub const TCA_OLD_COMPAT: u32 = TCA_ACT_MAX + 1;
pub const TCA_ACT_MAX_PRIO: u32 = 32;
pub const TCA_ACT_BIND: u32 = 1;
pub const TCA_ACT_NOBIND: u32 = 0;
pub const TCA_ACT_UNBIND: u32 = 1;
pub const TCA_ACT_NOUNBIND: u32 = 0;
pub const TCA_ACT_REPLACE: u32 = 1;
pub const TCA_ACT_NOREPLACE: u32 = 0;

pub const TC_ACT_UNSPEC: ::core::ffi::c_int = -1;
pub const TC_ACT_OK: ::core::ffi::c_int = 0;
pub const TC_ACT_RECLASSIFY: ::core::ffi::c_int = 1;
pub const TC_ACT_SHOT: ::core::ffi::c_int = 2;
pub const TC_ACT_PIPE: ::core::ffi::c_int = 3;
pub const TC_ACT_STOLEN: ::core::ffi::c_int = 4;
pub const TC_ACT_QUEUED: ::core::ffi::c_int = 5;
pub const TC_ACT_REPEAT: ::core::ffi::c_int = 6;
pub const TC_ACT_REDIRECT: ::core::ffi::c_int = 7;
/* For hw path, this means "trap to cpu" and don't further process the frame
 * in hardware. For sw path, this is equivalent of TC_ACT_STOLEN - drop the skb
 * and act like everything is alright.
 */
pub const TC_ACT_TRAP: ::core::ffi::c_int = 8;
pub const TC_ACT_VALUE_MAX: ::core::ffi::c_int = TC_ACT_TRAP;

/* There is a special kind of actions called "extended actions",
 * which need a value parameter. These have a local opcode located in
 * the highest nibble, starting from 1. The rest of the bits
 * are used to carry the value. These two parts together make
 * a combined opcode.
 */
pub const __TC_ACT_EXT_SHIFT: u32 = 28;
pub const fn __TC_ACT_EXT(local: u32) -> u32 {
    local << __TC_ACT_EXT_SHIFT
}
pub const TC_ACT_EXT_VAL_MASK: u32 = (1u32 << __TC_ACT_EXT_SHIFT) - 1;
pub const fn TC_ACT_EXT_OPCODE(combined: u32) -> u32 {
    combined & !TC_ACT_EXT_VAL_MASK
}
pub const fn TC_ACT_EXT_CMP(combined: u32, opcode: u32) -> bool {
    TC_ACT_EXT_OPCODE(combined) == opcode
}

pub const TC_ACT_JUMP: u32 = __TC_ACT_EXT(1);
pub const TC_ACT_GOTO_CHAIN: u32 = __TC_ACT_EXT(2);
pub const TC_ACT_EXT_OPCODE_MAX: u32 = TC_ACT_GOTO_CHAIN;

/* Action type identifiers*/
pub const TCA_ID_UNSPEC: u32 = 0;
pub const TCA_ID_POLICE: u32 = 1;
/* other actions go here */
pub const __TCA_ID_MAX: u32 = 255;

pub const TCA_ID_MAX: u32 = __TCA_ID_MAX;

#[repr(C)]
pub struct tc_police {
    pub index: __u32,
    pub action: ::core::ffi::c_int,
    pub limit: __u32,
    pub burst: __u32,
    pub mtu: __u32,
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
    pub refcnt: ::core::ffi::c_int,
    pub bindcnt: ::core::ffi::c_int,
    pub capab: __u32,
}

pub const TC_POLICE_UNSPEC: ::core::ffi::c_int = TC_ACT_UNSPEC;
pub const TC_POLICE_OK: ::core::ffi::c_int = TC_ACT_OK;
pub const TC_POLICE_RECLASSIFY: ::core::ffi::c_int = TC_ACT_RECLASSIFY;
pub const TC_POLICE_SHOT: ::core::ffi::c_int = TC_ACT_SHOT;
pub const TC_POLICE_PIPE: ::core::ffi::c_int = TC_ACT_PIPE;

#[repr(C)]
pub struct tcf_t {
    pub install: __u64,
    pub lastuse: __u64,
    pub expires: __u64,
    pub firstuse: __u64,
}

#[repr(C)]
pub struct tc_cnt {
    pub refcnt: ::core::ffi::c_int,
    pub bindcnt: ::core::ffi::c_int,
}

/* C macro tc_gen expands to:
 * __u32 index; __u32 capab; int action; int refcnt; int bindcnt
 */

pub const TCA_POLICE_UNSPEC: u32 = 0;
pub const TCA_POLICE_TBF: u32 = 1;
pub const TCA_POLICE_RATE: u32 = 2;
pub const TCA_POLICE_PEAKRATE: u32 = 3;
pub const TCA_POLICE_AVRATE: u32 = 4;
pub const TCA_POLICE_RESULT: u32 = 5;
pub const TCA_POLICE_TM: u32 = 6;
pub const TCA_POLICE_PAD: u32 = 7;
pub const __TCA_POLICE_MAX: u32 = 8;

pub const TCA_POLICE_MAX: u32 = __TCA_POLICE_MAX - 1;

/* tca flags definitions */
pub const TCA_CLS_FLAGS_SKIP_HW: u32 = 1 << 0; /* don't offload filter to HW */
pub const TCA_CLS_FLAGS_SKIP_SW: u32 = 1 << 1; /* don't use filter in SW */
pub const TCA_CLS_FLAGS_IN_HW: u32 = 1 << 2; /* filter is offloaded to HW */
pub const TCA_CLS_FLAGS_NOT_IN_HW: u32 = 1 << 3; /* filter isn't offloaded to HW */
pub const TCA_CLS_FLAGS_VERBOSE: u32 = 1 << 4; /* verbose logging */

/* U32 filters */

pub const fn TC_U32_HTID(h: u32) -> u32 {
    h & 0xFFF00000
}
pub const fn TC_U32_USERHTID(h: u32) -> u32 {
    TC_U32_HTID(h) >> 20
}
pub const fn TC_U32_HASH(h: u32) -> u32 {
    (h >> 12) & 0xFF
}
pub const fn TC_U32_NODE(h: u32) -> u32 {
    h & 0xFFF
}
pub const fn TC_U32_KEY(h: u32) -> u32 {
    h & 0xFFFFF
}
pub const TC_U32_UNSPEC: u32 = 0;
pub const TC_U32_ROOT: u32 = 0xFFF00000;

pub const TCA_U32_UNSPEC: u32 = 0;
pub const TCA_U32_CLASSID: u32 = 1;
pub const TCA_U32_HASH: u32 = 2;
pub const TCA_U32_LINK: u32 = 3;
pub const TCA_U32_DIVISOR: u32 = 4;
pub const TCA_U32_SEL: u32 = 5;
pub const TCA_U32_POLICE: u32 = 6;
pub const TCA_U32_ACT: u32 = 7;
pub const TCA_U32_INDEV: u32 = 8;
pub const TCA_U32_PCNT: u32 = 9;
pub const TCA_U32_MARK: u32 = 10;
pub const TCA_U32_FLAGS: u32 = 11;
pub const TCA_U32_PAD: u32 = 12;
pub const __TCA_U32_MAX: u32 = 13;

pub const TCA_U32_MAX: u32 = __TCA_U32_MAX - 1;

#[repr(C)]
pub struct tc_u32_key {
    pub mask: __be32,
    pub val: __be32,
    pub off: ::core::ffi::c_int,
    pub offmask: ::core::ffi::c_int,
}

#[repr(C)]
pub struct tc_u32_sel {
    pub flags: ::core::ffi::c_uchar,
    pub offshift: ::core::ffi::c_uchar,
    pub nkeys: ::core::ffi::c_uchar,
    pub offmask: __be16,
    pub off: __u16,
    pub offoff: ::core::ffi::c_short,
    pub hoff: ::core::ffi::c_short,
    pub hmask: __be32,
    pub keys: [tc_u32_key; 0],
}

#[repr(C)]
pub struct tc_u32_mark {
    pub val: __u32,
    pub mask: __u32,
    pub success: __u32,
}

#[repr(C)]
pub struct tc_u32_pcnt {
    pub rcnt: __u64,
    pub rhit: __u64,
    pub kcnts: [__u64; 0],
}

/* Flags */

pub const TC_U32_TERMINAL: u32 = 1;
pub const TC_U32_OFFSET: u32 = 2;
pub const TC_U32_VAROFFSET: u32 = 4;
pub const TC_U32_EAT: u32 = 8;

pub const TC_U32_MAXDEPTH: u32 = 8;

/* ROUTE filter */

pub const TCA_ROUTE4_UNSPEC: u32 = 0;
pub const TCA_ROUTE4_CLASSID: u32 = 1;
pub const TCA_ROUTE4_TO: u32 = 2;
pub const TCA_ROUTE4_FROM: u32 = 3;
pub const TCA_ROUTE4_IIF: u32 = 4;
pub const TCA_ROUTE4_POLICE: u32 = 5;
pub const TCA_ROUTE4_ACT: u32 = 6;
pub const __TCA_ROUTE4_MAX: u32 = 7;

pub const TCA_ROUTE4_MAX: u32 = __TCA_ROUTE4_MAX - 1;

/* FW filter */

pub const TCA_FW_UNSPEC: u32 = 0;
pub const TCA_FW_CLASSID: u32 = 1;
pub const TCA_FW_POLICE: u32 = 2;
pub const TCA_FW_INDEV: u32 = 3;
pub const TCA_FW_ACT: u32 = 4; /* used by CONFIG_NET_CLS_ACT */
pub const TCA_FW_MASK: u32 = 5;
pub const __TCA_FW_MAX: u32 = 6;

pub const TCA_FW_MAX: u32 = __TCA_FW_MAX - 1;

/* Flow filter */

pub const FLOW_KEY_SRC: u32 = 0;
pub const FLOW_KEY_DST: u32 = 1;
pub const FLOW_KEY_PROTO: u32 = 2;
pub const FLOW_KEY_PROTO_SRC: u32 = 3;
pub const FLOW_KEY_PROTO_DST: u32 = 4;
pub const FLOW_KEY_IIF: u32 = 5;
pub const FLOW_KEY_PRIORITY: u32 = 6;
pub const FLOW_KEY_MARK: u32 = 7;
pub const FLOW_KEY_NFCT: u32 = 8;
pub const FLOW_KEY_NFCT_SRC: u32 = 9;
pub const FLOW_KEY_NFCT_DST: u32 = 10;
pub const FLOW_KEY_NFCT_PROTO_SRC: u32 = 11;
pub const FLOW_KEY_NFCT_PROTO_DST: u32 = 12;
pub const FLOW_KEY_RTCLASSID: u32 = 13;
pub const FLOW_KEY_SKUID: u32 = 14;
pub const FLOW_KEY_SKGID: u32 = 15;
pub const FLOW_KEY_VLAN_TAG: u32 = 16;
pub const FLOW_KEY_RXHASH: u32 = 17;
pub const __FLOW_KEY_MAX: u32 = 18;

pub const FLOW_KEY_MAX: u32 = __FLOW_KEY_MAX - 1;

pub const FLOW_MODE_MAP: u32 = 0;
pub const FLOW_MODE_HASH: u32 = 1;

pub const TCA_FLOW_UNSPEC: u32 = 0;
pub const TCA_FLOW_KEYS: u32 = 1;
pub const TCA_FLOW_MODE: u32 = 2;
pub const TCA_FLOW_BASECLASS: u32 = 3;
pub const TCA_FLOW_RSHIFT: u32 = 4;
pub const TCA_FLOW_ADDEND: u32 = 5;
pub const TCA_FLOW_MASK: u32 = 6;
pub const TCA_FLOW_XOR: u32 = 7;
pub const TCA_FLOW_DIVISOR: u32 = 8;
pub const TCA_FLOW_ACT: u32 = 9;
pub const TCA_FLOW_POLICE: u32 = 10;
pub const TCA_FLOW_EMATCHES: u32 = 11;
pub const TCA_FLOW_PERTURB: u32 = 12;
pub const __TCA_FLOW_MAX: u32 = 13;

pub const TCA_FLOW_MAX: u32 = __TCA_FLOW_MAX - 1;

/* Basic filter */

pub const TCA_BASIC_UNSPEC: u32 = 0;
pub const TCA_BASIC_CLASSID: u32 = 1;
pub const TCA_BASIC_EMATCHES: u32 = 2;
pub const TCA_BASIC_ACT: u32 = 3;
pub const TCA_BASIC_POLICE: u32 = 4;
pub const __TCA_BASIC_MAX: u32 = 5;

pub const TCA_BASIC_MAX: u32 = __TCA_BASIC_MAX - 1;

/* Cgroup classifier */

pub const TCA_CGROUP_UNSPEC: u32 = 0;
pub const TCA_CGROUP_ACT: u32 = 1;
pub const TCA_CGROUP_POLICE: u32 = 2;
pub const TCA_CGROUP_EMATCHES: u32 = 3;
pub const __TCA_CGROUP_MAX: u32 = 4;

pub const TCA_CGROUP_MAX: u32 = __TCA_CGROUP_MAX - 1;

/* BPF classifier */

pub const TCA_BPF_FLAG_ACT_DIRECT: u32 = 1 << 0;

pub const TCA_BPF_UNSPEC: u32 = 0;
pub const TCA_BPF_ACT: u32 = 1;
pub const TCA_BPF_POLICE: u32 = 2;
pub const TCA_BPF_CLASSID: u32 = 3;
pub const TCA_BPF_OPS_LEN: u32 = 4;
pub const TCA_BPF_OPS: u32 = 5;
pub const TCA_BPF_FD: u32 = 6;
pub const TCA_BPF_NAME: u32 = 7;
pub const TCA_BPF_FLAGS: u32 = 8;
pub const TCA_BPF_FLAGS_GEN: u32 = 9;
pub const TCA_BPF_TAG: u32 = 10;
pub const TCA_BPF_ID: u32 = 11;
pub const __TCA_BPF_MAX: u32 = 12;

pub const TCA_BPF_MAX: u32 = __TCA_BPF_MAX - 1;

/* Flower classifier */

pub const TCA_FLOWER_UNSPEC: u32 = 0;
pub const TCA_FLOWER_CLASSID: u32 = 1;
pub const TCA_FLOWER_INDEV: u32 = 2;
pub const TCA_FLOWER_ACT: u32 = 3;
pub const TCA_FLOWER_KEY_ETH_DST: u32 = 4; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ETH_DST_MASK: u32 = 5; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ETH_SRC: u32 = 6; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ETH_SRC_MASK: u32 = 7; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ETH_TYPE: u32 = 8; /* be16 */
pub const TCA_FLOWER_KEY_IP_PROTO: u32 = 9; /* u8 */
pub const TCA_FLOWER_KEY_IPV4_SRC: u32 = 10; /* be32 */
pub const TCA_FLOWER_KEY_IPV4_SRC_MASK: u32 = 11; /* be32 */
pub const TCA_FLOWER_KEY_IPV4_DST: u32 = 12; /* be32 */
pub const TCA_FLOWER_KEY_IPV4_DST_MASK: u32 = 13; /* be32 */
pub const TCA_FLOWER_KEY_IPV6_SRC: u32 = 14; /* struct in6_addr */
pub const TCA_FLOWER_KEY_IPV6_SRC_MASK: u32 = 15; /* struct in6_addr */
pub const TCA_FLOWER_KEY_IPV6_DST: u32 = 16; /* struct in6_addr */
pub const TCA_FLOWER_KEY_IPV6_DST_MASK: u32 = 17; /* struct in6_addr */
pub const TCA_FLOWER_KEY_TCP_SRC: u32 = 18; /* be16 */
pub const TCA_FLOWER_KEY_TCP_DST: u32 = 19; /* be16 */
pub const TCA_FLOWER_KEY_UDP_SRC: u32 = 20; /* be16 */
pub const TCA_FLOWER_KEY_UDP_DST: u32 = 21; /* be16 */
pub const TCA_FLOWER_FLAGS: u32 = 22;
pub const TCA_FLOWER_KEY_VLAN_ID: u32 = 23; /* be16 */
pub const TCA_FLOWER_KEY_VLAN_PRIO: u32 = 24; /* u8 */
pub const TCA_FLOWER_KEY_VLAN_ETH_TYPE: u32 = 25; /* be16 */
pub const TCA_FLOWER_KEY_ENC_KEY_ID: u32 = 26; /* be32 */
pub const TCA_FLOWER_KEY_ENC_IPV4_SRC: u32 = 27; /* be32 */
pub const TCA_FLOWER_KEY_ENC_IPV4_SRC_MASK: u32 = 28; /* be32 */
pub const TCA_FLOWER_KEY_ENC_IPV4_DST: u32 = 29; /* be32 */
pub const TCA_FLOWER_KEY_ENC_IPV4_DST_MASK: u32 = 30; /* be32 */
pub const TCA_FLOWER_KEY_ENC_IPV6_SRC: u32 = 31; /* struct in6_addr */
pub const TCA_FLOWER_KEY_ENC_IPV6_SRC_MASK: u32 = 32; /* struct in6_addr */
pub const TCA_FLOWER_KEY_ENC_IPV6_DST: u32 = 33; /* struct in6_addr */
pub const TCA_FLOWER_KEY_ENC_IPV6_DST_MASK: u32 = 34; /* struct in6_addr */
pub const TCA_FLOWER_KEY_TCP_SRC_MASK: u32 = 35; /* be16 */
pub const TCA_FLOWER_KEY_TCP_DST_MASK: u32 = 36; /* be16 */
pub const TCA_FLOWER_KEY_UDP_SRC_MASK: u32 = 37; /* be16 */
pub const TCA_FLOWER_KEY_UDP_DST_MASK: u32 = 38; /* be16 */
pub const TCA_FLOWER_KEY_SCTP_SRC_MASK: u32 = 39; /* be16 */
pub const TCA_FLOWER_KEY_SCTP_DST_MASK: u32 = 40; /* be16 */
pub const TCA_FLOWER_KEY_SCTP_SRC: u32 = 41; /* be16 */
pub const TCA_FLOWER_KEY_SCTP_DST: u32 = 42; /* be16 */
pub const TCA_FLOWER_KEY_ENC_UDP_SRC_PORT: u32 = 43; /* be16 */
pub const TCA_FLOWER_KEY_ENC_UDP_SRC_PORT_MASK: u32 = 44; /* be16 */
pub const TCA_FLOWER_KEY_ENC_UDP_DST_PORT: u32 = 45; /* be16 */
pub const TCA_FLOWER_KEY_ENC_UDP_DST_PORT_MASK: u32 = 46; /* be16 */
pub const TCA_FLOWER_KEY_FLAGS: u32 = 47; /* be32 */
pub const TCA_FLOWER_KEY_FLAGS_MASK: u32 = 48; /* be32 */
pub const TCA_FLOWER_KEY_ICMPV4_CODE: u32 = 49; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV4_CODE_MASK: u32 = 50; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV4_TYPE: u32 = 51; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV4_TYPE_MASK: u32 = 52; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV6_CODE: u32 = 53; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV6_CODE_MASK: u32 = 54; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV6_TYPE: u32 = 55; /* u8 */
pub const TCA_FLOWER_KEY_ICMPV6_TYPE_MASK: u32 = 56; /* u8 */
pub const TCA_FLOWER_KEY_ARP_SIP: u32 = 57; /* be32 */
pub const TCA_FLOWER_KEY_ARP_SIP_MASK: u32 = 58; /* be32 */
pub const TCA_FLOWER_KEY_ARP_TIP: u32 = 59; /* be32 */
pub const TCA_FLOWER_KEY_ARP_TIP_MASK: u32 = 60; /* be32 */
pub const TCA_FLOWER_KEY_ARP_OP: u32 = 61; /* u8 */
pub const TCA_FLOWER_KEY_ARP_OP_MASK: u32 = 62; /* u8 */
pub const TCA_FLOWER_KEY_ARP_SHA: u32 = 63; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ARP_SHA_MASK: u32 = 64; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ARP_THA: u32 = 65; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_ARP_THA_MASK: u32 = 66; /* ETH_ALEN */
pub const TCA_FLOWER_KEY_MPLS_TTL: u32 = 67; /* u8 - 8 bits */
pub const TCA_FLOWER_KEY_MPLS_BOS: u32 = 68; /* u8 - 1 bit */
pub const TCA_FLOWER_KEY_MPLS_TC: u32 = 69; /* u8 - 3 bits */
pub const TCA_FLOWER_KEY_MPLS_LABEL: u32 = 70; /* be32 - 20 bits */
pub const TCA_FLOWER_KEY_TCP_FLAGS: u32 = 71; /* be16 */
pub const TCA_FLOWER_KEY_TCP_FLAGS_MASK: u32 = 72; /* be16 */
pub const TCA_FLOWER_KEY_IP_TOS: u32 = 73; /* u8 */
pub const TCA_FLOWER_KEY_IP_TOS_MASK: u32 = 74; /* u8 */
pub const TCA_FLOWER_KEY_IP_TTL: u32 = 75; /* u8 */
pub const TCA_FLOWER_KEY_IP_TTL_MASK: u32 = 76; /* u8 */
pub const TCA_FLOWER_KEY_CVLAN_ID: u32 = 77; /* be16 */
pub const TCA_FLOWER_KEY_CVLAN_PRIO: u32 = 78; /* u8 */
pub const TCA_FLOWER_KEY_CVLAN_ETH_TYPE: u32 = 79; /* be16 */
pub const TCA_FLOWER_KEY_ENC_IP_TOS: u32 = 80; /* u8 */
pub const TCA_FLOWER_KEY_ENC_IP_TOS_MASK: u32 = 81; /* u8 */
pub const TCA_FLOWER_KEY_ENC_IP_TTL: u32 = 82; /* u8 */
pub const TCA_FLOWER_KEY_ENC_IP_TTL_MASK: u32 = 83; /* u8 */
pub const TCA_FLOWER_KEY_ENC_OPTS: u32 = 84;
pub const TCA_FLOWER_KEY_ENC_OPTS_MASK: u32 = 85;
pub const TCA_FLOWER_IN_HW_COUNT: u32 = 86;
pub const __TCA_FLOWER_MAX: u32 = 87;

pub const TCA_FLOWER_MAX: u32 = __TCA_FLOWER_MAX - 1;

pub const TCA_FLOWER_KEY_ENC_OPTS_UNSPEC: u32 = 0;
/* Nested TCA_FLOWER_KEY_ENC_OPT_GENEVE_ attributes */
pub const TCA_FLOWER_KEY_ENC_OPTS_GENEVE: u32 = 1;
pub const __TCA_FLOWER_KEY_ENC_OPTS_MAX: u32 = 2;

pub const TCA_FLOWER_KEY_ENC_OPTS_MAX: u32 = __TCA_FLOWER_KEY_ENC_OPTS_MAX - 1;

pub const TCA_FLOWER_KEY_ENC_OPT_GENEVE_UNSPEC: u32 = 0;
pub const TCA_FLOWER_KEY_ENC_OPT_GENEVE_CLASS: u32 = 1; /* u16 */
pub const TCA_FLOWER_KEY_ENC_OPT_GENEVE_TYPE: u32 = 2; /* u8 */
pub const TCA_FLOWER_KEY_ENC_OPT_GENEVE_DATA: u32 = 3; /* 4 to 128 bytes */
pub const __TCA_FLOWER_KEY_ENC_OPT_GENEVE_MAX: u32 = 4;

pub const TCA_FLOWER_KEY_ENC_OPT_GENEVE_MAX: u32 =
    __TCA_FLOWER_KEY_ENC_OPT_GENEVE_MAX - 1;

pub const TCA_FLOWER_KEY_FLAGS_IS_FRAGMENT: u32 = 1 << 0;
pub const TCA_FLOWER_KEY_FLAGS_FRAG_IS_FIRST: u32 = 1 << 1;

/* Match-all classifier */

pub const TCA_MATCHALL_UNSPEC: u32 = 0;
pub const TCA_MATCHALL_CLASSID: u32 = 1;
pub const TCA_MATCHALL_ACT: u32 = 2;
pub const TCA_MATCHALL_FLAGS: u32 = 3;
pub const __TCA_MATCHALL_MAX: u32 = 4;

pub const TCA_MATCHALL_MAX: u32 = __TCA_MATCHALL_MAX - 1;

/* Extended Matches */

#[repr(C)]
pub struct tcf_ematch_tree_hdr {
    pub nmatches: __u16,
    pub progid: __u16,
}

pub const TCA_EMATCH_TREE_UNSPEC: u32 = 0;
pub const TCA_EMATCH_TREE_HDR: u32 = 1;
pub const TCA_EMATCH_TREE_LIST: u32 = 2;
pub const __TCA_EMATCH_TREE_MAX: u32 = 3;

pub const TCA_EMATCH_TREE_MAX: u32 = __TCA_EMATCH_TREE_MAX - 1;

#[repr(C)]
pub struct tcf_ematch_hdr {
    pub matchid: __u16,
    pub kind: __u16,
    pub flags: __u16,
    pub pad: __u16, /* currently unused */
}

/*  0                   1
 *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
 * +-----------------------+-+-+---+
 * |         Unused        |S|I| R |
 * +-----------------------+-+-+---+
 *
 * R(2) ::= relation to next ematch
 *          where: 0 0 END (last ematch)
 *                 0 1 AND
 *                 1 0 OR
 *                 1 1 Unused (invalid)
 * I(1) ::= invert result
 * S(1) ::= simple payload
 */
pub const TCF_EM_REL_END: u32 = 0;
pub const TCF_EM_REL_AND: u32 = 1 << 0;
pub const TCF_EM_REL_OR: u32 = 1 << 1;
pub const TCF_EM_INVERT: u32 = 1 << 2;
pub const TCF_EM_SIMPLE: u32 = 1 << 3;

pub const TCF_EM_REL_MASK: u32 = 3;
pub const fn TCF_EM_REL_VALID(v: u32) -> bool {
    (v & TCF_EM_REL_MASK) != TCF_EM_REL_MASK
}

pub const TCF_LAYER_LINK: u32 = 0;
pub const TCF_LAYER_NETWORK: u32 = 1;
pub const TCF_LAYER_TRANSPORT: u32 = 2;
pub const __TCF_LAYER_MAX: u32 = 3;

pub const TCF_LAYER_MAX: u32 = __TCF_LAYER_MAX - 1;

/* Ematch type assignments
 *   1..32767        Reserved for ematches inside kernel tree
 *   32768..65535   Free to use, not reliable
 */
pub const TCF_EM_CONTAINER: u32 = 0;
pub const TCF_EM_CMP: u32 = 1;
pub const TCF_EM_NBYTE: u32 = 2;
pub const TCF_EM_U32: u32 = 3;
pub const TCF_EM_META: u32 = 4;
pub const TCF_EM_TEXT: u32 = 5;
pub const TCF_EM_VLAN: u32 = 6;
pub const TCF_EM_CANID: u32 = 7;
pub const TCF_EM_IPSET: u32 = 8;
pub const TCF_EM_IPT: u32 = 9;
pub const TCF_EM_MAX: u32 = 9;

pub const TCF_EM_PROG_TC: u32 = 0;

pub const TCF_EM_OPND_EQ: u32 = 0;
pub const TCF_EM_OPND_GT: u32 = 1;
pub const TCF_EM_OPND_LT: u32 = 2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
