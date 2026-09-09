/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. The linux/types.h and
// linux/pkt_cls.h dependencies are supplied by the surrounding translation.

pub const TCA_EM_META_UNSPEC: i32 = 0;
pub const TCA_EM_META_HDR: i32 = 1;
pub const TCA_EM_META_LVALUE: i32 = 2;
pub const TCA_EM_META_RVALUE: i32 = 3;
pub const __TCA_EM_META_MAX: i32 = 4;
pub const TCA_EM_META_MAX: i32 = __TCA_EM_META_MAX - 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_meta_val {
    pub kind: u16,
    pub shift: u8,
    pub op: u8,
}

pub const TCF_META_TYPE_MASK: u32 = 0xf << 12;
#[inline]
pub const fn TCF_META_TYPE(kind: u32) -> u32 {
    (kind & TCF_META_TYPE_MASK) >> 12
}
pub const TCF_META_ID_MASK: u32 = 0x7ff;
#[inline]
pub const fn TCF_META_ID(kind: u32) -> u32 {
    kind & TCF_META_ID_MASK
}

pub const TCF_META_TYPE_VAR: i32 = 0;
pub const TCF_META_TYPE_INT: i32 = 1;
pub const __TCF_META_TYPE_MAX: i32 = 2;
pub const TCF_META_TYPE_MAX: i32 = __TCF_META_TYPE_MAX - 1;

pub const TCF_META_ID_VALUE: i32 = 0;
pub const TCF_META_ID_RANDOM: i32 = 1;
pub const TCF_META_ID_LOADAVG_0: i32 = 2;
pub const TCF_META_ID_LOADAVG_1: i32 = 3;
pub const TCF_META_ID_LOADAVG_2: i32 = 4;
pub const TCF_META_ID_DEV: i32 = 5;
pub const TCF_META_ID_PRIORITY: i32 = 6;
pub const TCF_META_ID_PROTOCOL: i32 = 7;
pub const TCF_META_ID_PKTTYPE: i32 = 8;
pub const TCF_META_ID_PKTLEN: i32 = 9;
pub const TCF_META_ID_DATALEN: i32 = 10;
pub const TCF_META_ID_MACLEN: i32 = 11;
pub const TCF_META_ID_NFMARK: i32 = 12;
pub const TCF_META_ID_TCINDEX: i32 = 13;
pub const TCF_META_ID_RTCLASSID: i32 = 14;
pub const TCF_META_ID_RTIIF: i32 = 15;
pub const TCF_META_ID_SK_FAMILY: i32 = 16;
pub const TCF_META_ID_SK_STATE: i32 = 17;
pub const TCF_META_ID_SK_REUSE: i32 = 18;
pub const TCF_META_ID_SK_BOUND_IF: i32 = 19;
pub const TCF_META_ID_SK_REFCNT: i32 = 20;
pub const TCF_META_ID_SK_SHUTDOWN: i32 = 21;
pub const TCF_META_ID_SK_PROTO: i32 = 22;
pub const TCF_META_ID_SK_TYPE: i32 = 23;
pub const TCF_META_ID_SK_RCVBUF: i32 = 24;
pub const TCF_META_ID_SK_RMEM_ALLOC: i32 = 25;
pub const TCF_META_ID_SK_WMEM_ALLOC: i32 = 26;
pub const TCF_META_ID_SK_OMEM_ALLOC: i32 = 27;
pub const TCF_META_ID_SK_WMEM_QUEUED: i32 = 28;
pub const TCF_META_ID_SK_RCV_QLEN: i32 = 29;
pub const TCF_META_ID_SK_SND_QLEN: i32 = 30;
pub const TCF_META_ID_SK_ERR_QLEN: i32 = 31;
pub const TCF_META_ID_SK_FORWARD_ALLOCS: i32 = 32;
pub const TCF_META_ID_SK_SNDBUF: i32 = 33;
pub const TCF_META_ID_SK_ALLOCS: i32 = 34;
// Unimplemented but already present in the ABI.
pub const __TCF_META_ID_SK_ROUTE_CAPS: i32 = 35;
pub const TCF_META_ID_SK_HASH: i32 = 36;
pub const TCF_META_ID_SK_LINGERTIME: i32 = 37;
pub const TCF_META_ID_SK_ACK_BACKLOG: i32 = 38;
pub const TCF_META_ID_SK_MAX_ACK_BACKLOG: i32 = 39;
pub const TCF_META_ID_SK_PRIO: i32 = 40;
pub const TCF_META_ID_SK_RCVLOWAT: i32 = 41;
pub const TCF_META_ID_SK_RCVTIMEO: i32 = 42;
pub const TCF_META_ID_SK_SNDTIMEO: i32 = 43;
pub const TCF_META_ID_SK_SENDMSG_OFF: i32 = 44;
pub const TCF_META_ID_SK_WRITE_PENDING: i32 = 45;
pub const TCF_META_ID_VLAN_TAG: i32 = 46;
pub const TCF_META_ID_RXHASH: i32 = 47;
pub const __TCF_META_ID_MAX: i32 = 48;
pub const TCF_META_ID_MAX: i32 = __TCF_META_ID_MAX - 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_meta_hdr {
    pub left: tcf_meta_val,
    pub right: tcf_meta_val,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
