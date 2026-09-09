/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const NETLINK_ROUTE: i32 = 0;
pub const NETLINK_UNUSED: i32 = 1;
pub const NETLINK_USERSOCK: i32 = 2;
pub const NETLINK_FIREWALL: i32 = 3;
pub const NETLINK_SOCK_DIAG: i32 = 4;
pub const NETLINK_NFLOG: i32 = 5;
pub const NETLINK_XFRM: i32 = 6;
pub const NETLINK_SELINUX: i32 = 7;
pub const NETLINK_ISCSI: i32 = 8;
pub const NETLINK_AUDIT: i32 = 9;
pub const NETLINK_FIB_LOOKUP: i32 = 10;
pub const NETLINK_CONNECTOR: i32 = 11;
pub const NETLINK_NETFILTER: i32 = 12;
pub const NETLINK_IP6_FW: i32 = 13;
pub const NETLINK_DNRTMSG: i32 = 14;
pub const NETLINK_KOBJECT_UEVENT: i32 = 15;
pub const NETLINK_GENERIC: i32 = 16;
pub const NETLINK_SCSITRANSPORT: i32 = 18;
pub const NETLINK_ECRYPTFS: i32 = 19;
pub const NETLINK_RDMA: i32 = 20;
pub const NETLINK_CRYPTO: i32 = 21;
pub const NETLINK_SMC: i32 = 22;
pub const NETLINK_INET_DIAG: i32 = NETLINK_SOCK_DIAG;
pub const MAX_LINKS: i32 = 32;

#[repr(C)]
pub struct sockaddr_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad: u16,
    pub nl_pid: __u32,
    pub nl_groups: __u32,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}

pub const NLM_F_REQUEST: u16 = 0x01;
pub const NLM_F_MULTI: u16 = 0x02;
pub const NLM_F_ACK: u16 = 0x04;
pub const NLM_F_ECHO: u16 = 0x08;
pub const NLM_F_DUMP_INTR: u16 = 0x10;
pub const NLM_F_DUMP_FILTERED: u16 = 0x20;
pub const NLM_F_ROOT: u16 = 0x100;
pub const NLM_F_MATCH: u16 = 0x200;
pub const NLM_F_ATOMIC: u16 = 0x400;
pub const NLM_F_DUMP: u16 = NLM_F_ROOT | NLM_F_MATCH;
pub const NLM_F_REPLACE: u16 = 0x100;
pub const NLM_F_EXCL: u16 = 0x200;
pub const NLM_F_CREATE: u16 = 0x400;
pub const NLM_F_APPEND: u16 = 0x800;
pub const NLM_F_NONREC: u16 = 0x100;
pub const NLM_F_BULK: u16 = 0x200;
pub const NLM_F_CAPPED: u16 = 0x100;
pub const NLM_F_ACK_TLVS: u16 = 0x200;

pub const NLMSG_ALIGNTO: usize = 4;
#[inline]
pub const fn NLMSG_ALIGN(len: usize) -> usize { (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1) }
pub const NLMSG_HDRLEN: usize = NLMSG_ALIGN(core::mem::size_of::<nlmsghdr>());
#[inline]
pub const fn NLMSG_LENGTH(len: usize) -> usize { len + NLMSG_HDRLEN }
#[inline]
pub const fn NLMSG_SPACE(len: usize) -> usize { NLMSG_ALIGN(NLMSG_LENGTH(len)) }
#[inline]
pub unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut core::ffi::c_void { (nlh as *mut u8).add(NLMSG_HDRLEN) as *mut core::ffi::c_void }
#[inline]
pub unsafe fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: &mut i32) -> *mut nlmsghdr {
    let step = NLMSG_ALIGN((*nlh).nlmsg_len as usize);
    *len -= step as i32;
    (nlh as *mut u8).add(step) as *mut nlmsghdr
}
#[inline]
pub unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: i32) -> bool {
    len >= core::mem::size_of::<nlmsghdr>() as i32 && (*nlh).nlmsg_len as usize >= core::mem::size_of::<nlmsghdr>() && (*nlh).nlmsg_len as usize <= len as usize
}
#[inline]
pub unsafe fn NLMSG_PAYLOAD(nlh: *const nlmsghdr, len: usize) -> usize { (*nlh).nlmsg_len as usize - NLMSG_SPACE(len) }

pub const NLMSG_NOOP: u16 = 0x1;
pub const NLMSG_ERROR: u16 = 0x2;
pub const NLMSG_DONE: u16 = 0x3;
pub const NLMSG_OVERRUN: u16 = 0x4;
pub const NLMSG_MIN_TYPE: u16 = 0x10;

#[repr(C)]
pub struct nlmsgerr { pub error: i32, pub msg: nlmsghdr }

#[repr(i32)]
pub enum nlmsgerr_attrs {
    NLMSGERR_ATTR_UNUSED,
    NLMSGERR_ATTR_MSG,
    NLMSGERR_ATTR_OFFS,
    NLMSGERR_ATTR_COOKIE,
    NLMSGERR_ATTR_POLICY,
    NLMSGERR_ATTR_MISS_TYPE,
    NLMSGERR_ATTR_MISS_NEST,
    __NLMSGERR_ATTR_MAX,
}
pub const NLMSGERR_ATTR_MAX: i32 = __NLMSGERR_ATTR_MAX as i32 - 1;

pub const NETLINK_ADD_MEMBERSHIP: i32 = 1;
pub const NETLINK_DROP_MEMBERSHIP: i32 = 2;
pub const NETLINK_PKTINFO: i32 = 3;
pub const NETLINK_BROADCAST_ERROR: i32 = 4;
pub const NETLINK_NO_ENOBUFS: i32 = 5;
// #ifndef __KERNEL__
pub const NETLINK_RX_RING: i32 = 6;
pub const NETLINK_TX_RING: i32 = 7;
// #endif
pub const NETLINK_LISTEN_ALL_NSID: i32 = 8;
pub const NETLINK_LIST_MEMBERSHIPS: i32 = 9;
pub const NETLINK_CAP_ACK: i32 = 10;
pub const NETLINK_EXT_ACK: i32 = 11;
pub const NETLINK_GET_STRICT_CHK: i32 = 12;

#[repr(C)] pub struct nl_pktinfo { pub group: __u32 }
#[repr(C)] pub struct nl_mmap_req { pub nm_block_size: u32, pub nm_block_nr: u32, pub nm_frame_size: u32, pub nm_frame_nr: u32 }
#[repr(C)] pub struct nl_mmap_hdr { pub nm_status: u32, pub nm_len: u32, pub nm_group: __u32, pub nm_pid: __u32, pub nm_uid: __u32, pub nm_gid: __u32 }

// #ifndef __KERNEL__
#[repr(i32)] pub enum nl_mmap_status { NL_MMAP_STATUS_UNUSED, NL_MMAP_STATUS_RESERVED, NL_MMAP_STATUS_VALID, NL_MMAP_STATUS_COPY, NL_MMAP_STATUS_SKIP }
pub const NL_MMAP_MSG_ALIGNMENT: usize = NLMSG_ALIGNTO;
pub const fn NL_MMAP_MSG_ALIGN(sz: usize) -> usize { (sz + NL_MMAP_MSG_ALIGNMENT - 1) & !(NL_MMAP_MSG_ALIGNMENT - 1) }
pub const NL_MMAP_HDRLEN: usize = NL_MMAP_MSG_ALIGN(core::mem::size_of::<nl_mmap_hdr>());
// #endif

pub const NET_MAJOR: i32 = 36;
pub const NETLINK_UNCONNECTED: i32 = 0;
pub const NETLINK_CONNECTED: i32 = 1;

#[repr(C)] pub struct nlattr { pub nla_len: __u16, pub nla_type: __u16 }
pub const NLA_F_NESTED: u16 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: u16 = 1 << 14;
pub const NLA_TYPE_MASK: u16 = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);
pub const NLA_ALIGNTO: usize = 4;
pub const fn NLA_ALIGN(len: usize) -> usize { (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1) }
pub const NLA_HDRLEN: usize = NLA_ALIGN(core::mem::size_of::<nlattr>());

#[repr(C)] pub struct nla_bitfield32 { pub value: __u32, pub selector: __u32 }

#[repr(i32)]
pub enum netlink_attribute_type {
    NL_ATTR_TYPE_INVALID, NL_ATTR_TYPE_FLAG, NL_ATTR_TYPE_U8, NL_ATTR_TYPE_U16, NL_ATTR_TYPE_U32, NL_ATTR_TYPE_U64,
    NL_ATTR_TYPE_S8, NL_ATTR_TYPE_S16, NL_ATTR_TYPE_S32, NL_ATTR_TYPE_S64, NL_ATTR_TYPE_BINARY, NL_ATTR_TYPE_STRING,
    NL_ATTR_TYPE_NUL_STRING, NL_ATTR_TYPE_NESTED, NL_ATTR_TYPE_NESTED_ARRAY, NL_ATTR_TYPE_BITFIELD32,
    NL_ATTR_TYPE_SINT, NL_ATTR_TYPE_UINT,
}

#[repr(i32)]
pub enum netlink_policy_type_attr {
    NL_POLICY_TYPE_ATTR_UNSPEC, NL_POLICY_TYPE_ATTR_TYPE, NL_POLICY_TYPE_ATTR_MIN_VALUE_S,
    NL_POLICY_TYPE_ATTR_MAX_VALUE_S, NL_POLICY_TYPE_ATTR_MIN_VALUE_U, NL_POLICY_TYPE_ATTR_MAX_VALUE_U,
    NL_POLICY_TYPE_ATTR_MIN_LENGTH, NL_POLICY_TYPE_ATTR_MAX_LENGTH, NL_POLICY_TYPE_ATTR_POLICY_IDX,
    NL_POLICY_TYPE_ATTR_POLICY_MAXTYPE, NL_POLICY_TYPE_ATTR_BITFIELD32_MASK, NL_POLICY_TYPE_ATTR_PAD,
    NL_POLICY_TYPE_ATTR_MASK, __NL_POLICY_TYPE_ATTR_MAX,
}
pub const NL_POLICY_TYPE_ATTR_MAX: i32 = __NL_POLICY_TYPE_ATTR_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
