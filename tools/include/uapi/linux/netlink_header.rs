/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies from the original C header:
 * <linux/kernel.h>
 * <linux/socket.h> for __kernel_sa_family_t
 * <linux/types.h>
 */

pub const NETLINK_ROUTE: u32 = 0; /* Routing/device hook				*/
pub const NETLINK_UNUSED: u32 = 1; /* Unused number				*/
pub const NETLINK_USERSOCK: u32 = 2; /* Reserved for user mode socket protocols 	*/
pub const NETLINK_FIREWALL: u32 = 3; /* Unused number, formerly ip_queue		*/
pub const NETLINK_SOCK_DIAG: u32 = 4; /* socket monitoring				*/
pub const NETLINK_NFLOG: u32 = 5; /* netfilter/iptables ULOG */
pub const NETLINK_XFRM: u32 = 6; /* ipsec */
pub const NETLINK_SELINUX: u32 = 7; /* SELinux event notifications */
pub const NETLINK_ISCSI: u32 = 8; /* Open-iSCSI */
pub const NETLINK_AUDIT: u32 = 9; /* auditing */
pub const NETLINK_FIB_LOOKUP: u32 = 10;
pub const NETLINK_CONNECTOR: u32 = 11;
pub const NETLINK_NETFILTER: u32 = 12; /* netfilter subsystem */
pub const NETLINK_IP6_FW: u32 = 13;
pub const NETLINK_DNRTMSG: u32 = 14; /* DECnet routing messages */
pub const NETLINK_KOBJECT_UEVENT: u32 = 15; /* Kernel messages to userspace */
pub const NETLINK_GENERIC: u32 = 16;
/* leave room for NETLINK_DM (DM Events) */
pub const NETLINK_SCSITRANSPORT: u32 = 18; /* SCSI Transports */
pub const NETLINK_ECRYPTFS: u32 = 19;
pub const NETLINK_RDMA: u32 = 20;
pub const NETLINK_CRYPTO: u32 = 21; /* Crypto layer */
pub const NETLINK_SMC: u32 = 22; /* SMC monitoring */

pub const NETLINK_INET_DIAG: u32 = NETLINK_SOCK_DIAG;

pub const MAX_LINKS: u32 = 32;

#[repr(C)]
pub struct sockaddr_nl {
    pub nl_family: __kernel_sa_family_t, /* AF_NETLINK	*/
    pub nl_pad: u16,                     /* zero		*/
    pub nl_pid: __u32,                   /* port ID	*/
    pub nl_groups: __u32,                /* multicast groups mask */
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,   /* Length of message including header */
    pub nlmsg_type: __u16,  /* Message content */
    pub nlmsg_flags: __u16, /* Additional flags */
    pub nlmsg_seq: __u32,   /* Sequence number */
    pub nlmsg_pid: __u32,   /* Sending process port ID */
}

/* Flags values */

pub const NLM_F_REQUEST: u32 = 0x01; /* It is request message. 	*/
pub const NLM_F_MULTI: u32 = 0x02; /* Multipart message, terminated by NLMSG_DONE */
pub const NLM_F_ACK: u32 = 0x04; /* Reply with ack, with zero or error code */
pub const NLM_F_ECHO: u32 = 0x08; /* Echo this request 		*/
pub const NLM_F_DUMP_INTR: u32 = 0x10; /* Dump was inconsistent due to sequence change */
pub const NLM_F_DUMP_FILTERED: u32 = 0x20; /* Dump was filtered as requested */

/* Modifiers to GET request */
pub const NLM_F_ROOT: u32 = 0x100; /* specify tree	root	*/
pub const NLM_F_MATCH: u32 = 0x200; /* return all matching	*/
pub const NLM_F_ATOMIC: u32 = 0x400; /* atomic GET		*/
pub const NLM_F_DUMP: u32 = NLM_F_ROOT | NLM_F_MATCH;

/* Modifiers to NEW request */
pub const NLM_F_REPLACE: u32 = 0x100; /* Override existing		*/
pub const NLM_F_EXCL: u32 = 0x200; /* Do not touch, if it exists	*/
pub const NLM_F_CREATE: u32 = 0x400; /* Create, if it does not exist	*/
pub const NLM_F_APPEND: u32 = 0x800; /* Add to end of list		*/

/* Modifiers to DELETE request */
pub const NLM_F_NONREC: u32 = 0x100; /* Do not delete recursively	*/

/* Flags for ACK message */
pub const NLM_F_CAPPED: u32 = 0x100; /* request was capped */
pub const NLM_F_ACK_TLVS: u32 = 0x200; /* extended ACK TVLs were included */

/*
   4.4BSD ADD		NLM_F_CREATE|NLM_F_EXCL
   4.4BSD CHANGE	NLM_F_REPLACE

   True CHANGE		NLM_F_CREATE|NLM_F_REPLACE
   Append		NLM_F_CREATE
   Check		NLM_F_EXCL
 */

pub const NLMSG_ALIGNTO: usize = 4;

#[inline]
pub const fn NLMSG_ALIGN(len: usize) -> usize {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

pub const NLMSG_HDRLEN: i32 = NLMSG_ALIGN(core::mem::size_of::<nlmsghdr>()) as i32;

#[inline]
pub const fn NLMSG_LENGTH(len: usize) -> usize {
    len + NLMSG_HDRLEN as usize
}

#[inline]
pub const fn NLMSG_SPACE(len: usize) -> usize {
    NLMSG_ALIGN(NLMSG_LENGTH(len))
}

#[inline]
pub unsafe fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut core::ffi::c_void {
    (nlh as *const u8).add(NLMSG_LENGTH(0)) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: *mut i32) -> *mut nlmsghdr {
    *len -= NLMSG_ALIGN((*nlh).nlmsg_len as usize) as i32;
    (nlh as *mut u8).add(NLMSG_ALIGN((*nlh).nlmsg_len as usize)) as *mut nlmsghdr
}

#[inline]
pub unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: i32) -> bool {
    len >= core::mem::size_of::<nlmsghdr>() as i32
        && (*nlh).nlmsg_len as usize >= core::mem::size_of::<nlmsghdr>()
        && (*nlh).nlmsg_len <= len as __u32
}

#[inline]
pub unsafe fn NLMSG_PAYLOAD(nlh: *const nlmsghdr, len: usize) -> usize {
    (*nlh).nlmsg_len as usize - NLMSG_SPACE(len)
}

pub const NLMSG_NOOP: u32 = 0x1; /* Nothing.		*/
pub const NLMSG_ERROR: u32 = 0x2; /* Error		*/
pub const NLMSG_DONE: u32 = 0x3; /* End of a dump	*/
pub const NLMSG_OVERRUN: u32 = 0x4; /* Data lost		*/

pub const NLMSG_MIN_TYPE: u32 = 0x10; /* < 0x10: reserved control messages */

#[repr(C)]
pub struct nlmsgerr {
    pub error: core::ffi::c_int,
    pub msg: nlmsghdr,
    /*
     * followed by the message contents unless NETLINK_CAP_ACK was set
     * or the ACK indicates success (error == 0)
     * message length is aligned with NLMSG_ALIGN()
     */
    /*
     * followed by TLVs defined in enum nlmsgerr_attrs
     * if NETLINK_EXT_ACK was set
     */
}

/**
 * enum nlmsgerr_attrs - nlmsgerr attributes
 * @NLMSGERR_ATTR_UNUSED: unused
 * @NLMSGERR_ATTR_MSG: error message string (string)
 * @NLMSGERR_ATTR_OFFS: offset of the invalid attribute in the original
 *	 message, counting from the beginning of the header (u32)
 * @NLMSGERR_ATTR_COOKIE: arbitrary subsystem specific cookie to
 *	be used - in the success case - to identify a created
 *	object or operation or similar (binary)
 * @__NLMSGERR_ATTR_MAX: number of attributes
 * @NLMSGERR_ATTR_MAX: highest attribute number
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nlmsgerr_attrs {
    NLMSGERR_ATTR_UNUSED = 0,
    NLMSGERR_ATTR_MSG = 1,
    NLMSGERR_ATTR_OFFS = 2,
    NLMSGERR_ATTR_COOKIE = 3,

    __NLMSGERR_ATTR_MAX = 4,
    NLMSGERR_ATTR_MAX = 3,
}

pub const NETLINK_ADD_MEMBERSHIP: u32 = 1;
pub const NETLINK_DROP_MEMBERSHIP: u32 = 2;
pub const NETLINK_PKTINFO: u32 = 3;
pub const NETLINK_BROADCAST_ERROR: u32 = 4;
pub const NETLINK_NO_ENOBUFS: u32 = 5;
/* !__KERNEL__ */
pub const NETLINK_RX_RING: u32 = 6;
pub const NETLINK_TX_RING: u32 = 7;
pub const NETLINK_LISTEN_ALL_NSID: u32 = 8;
pub const NETLINK_LIST_MEMBERSHIPS: u32 = 9;
pub const NETLINK_CAP_ACK: u32 = 10;
pub const NETLINK_EXT_ACK: u32 = 11;
pub const NETLINK_GET_STRICT_CHK: u32 = 12;

#[repr(C)]
pub struct nl_pktinfo {
    pub group: __u32,
}

#[repr(C)]
pub struct nl_mmap_req {
    pub nm_block_size: core::ffi::c_uint,
    pub nm_block_nr: core::ffi::c_uint,
    pub nm_frame_size: core::ffi::c_uint,
    pub nm_frame_nr: core::ffi::c_uint,
}

#[repr(C)]
pub struct nl_mmap_hdr {
    pub nm_status: core::ffi::c_uint,
    pub nm_len: core::ffi::c_uint,
    pub nm_group: __u32,
    /* credentials */
    pub nm_pid: __u32,
    pub nm_uid: __u32,
    pub nm_gid: __u32,
}

/* !__KERNEL__ */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nl_mmap_status {
    NL_MMAP_STATUS_UNUSED = 0,
    NL_MMAP_STATUS_RESERVED = 1,
    NL_MMAP_STATUS_VALID = 2,
    NL_MMAP_STATUS_COPY = 3,
    NL_MMAP_STATUS_SKIP = 4,
}

pub const NL_MMAP_MSG_ALIGNMENT: usize = NLMSG_ALIGNTO;

/* Uses __ALIGN_KERNEL from <linux/kernel.h>. */
#[inline]
pub const fn NL_MMAP_MSG_ALIGN(sz: usize) -> usize {
    __ALIGN_KERNEL(sz, NL_MMAP_MSG_ALIGNMENT)
}

pub const NL_MMAP_HDRLEN: usize = NL_MMAP_MSG_ALIGN(core::mem::size_of::<nl_mmap_hdr>());

pub const NET_MAJOR: u32 = 36; /* Major 36 is reserved for networking 						*/

pub const NETLINK_UNCONNECTED: i32 = 0;
pub const NETLINK_CONNECTED: i32 = 1;

/*
 *  <------- NLA_HDRLEN ------> <-- NLA_ALIGN(payload)-->
 * +---------------------+- - -+- - - - - - - - - -+- - -+
 * |        Header       | Pad |     Payload       | Pad |
 * |   (struct nlattr)   | ing |                   | ing |
 * +---------------------+- - -+- - - - - - - - - -+- - -+
 *  <-------------- nlattr->nla_len -------------->
 */

#[repr(C)]
pub struct nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}

/*
 * nla_type (16 bits)
 * +---+---+-------------------------------+
 * | N | O | Attribute Type                |
 * +---+---+-------------------------------+
 * N := Carries nested attributes
 * O := Payload stored in network byte order
 *
 * Note: The N and O flag are mutually exclusive.
 */
pub const NLA_F_NESTED: i32 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: i32 = 1 << 14;
pub const NLA_TYPE_MASK: i32 = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

pub const NLA_ALIGNTO: usize = 4;

#[inline]
pub const fn NLA_ALIGN(len: usize) -> usize {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

pub const NLA_HDRLEN: i32 = NLA_ALIGN(core::mem::size_of::<nlattr>()) as i32;

/* Generic 32 bitflags attribute content sent to the kernel.
 *
 * The value is a bitmap that defines the values being set
 * The selector is a bitmask that defines which value is legit
 *
 * Examples:
 *  value = 0x0, and selector = 0x1
 *  implies we are selecting bit 1 and we want to set its value to 0.
 *
 *  value = 0x2, and selector = 0x2
 *  implies we are selecting bit 2 and we want to set its value to 1.
 *
 */
#[repr(C)]
pub struct nla_bitfield32 {
    pub value: __u32,
    pub selector: __u32,
}
