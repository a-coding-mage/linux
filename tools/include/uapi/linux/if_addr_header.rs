/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from include/uapi/linux/if_addr.h. */
/* Depends on Linux UAPI types/macros originally provided by:
 * <linux/types.h> and <linux/netlink.h>.
 */

#[repr(C)]
pub struct ifaddrmsg {
    pub ifa_family: u8,
    pub ifa_prefixlen: u8, /* The prefix length        */
    pub ifa_flags: u8,    /* Flags                    */
    pub ifa_scope: u8,    /* Address scope            */
    pub ifa_index: u32,   /* Link index               */
}

/*
 * Important comment:
 * IFA_ADDRESS is prefix address, rather than local interface address.
 * It makes no difference for normally configured broadcast interfaces,
 * but for point-to-point IFA_ADDRESS is DESTINATION address,
 * local address is supplied in IFA_LOCAL attribute.
 *
 * IFA_FLAGS is a u32 attribute that extends the u8 field ifa_flags.
 * If present, the value from struct ifaddrmsg will be ignored.
 */
pub const IFA_UNSPEC: u32 = 0;
pub const IFA_ADDRESS: u32 = 1;
pub const IFA_LOCAL: u32 = 2;
pub const IFA_LABEL: u32 = 3;
pub const IFA_BROADCAST: u32 = 4;
pub const IFA_ANYCAST: u32 = 5;
pub const IFA_CACHEINFO: u32 = 6;
pub const IFA_MULTICAST: u32 = 7;
pub const IFA_FLAGS: u32 = 8;
pub const IFA_RT_PRIORITY: u32 = 9; /* u32, priority/metric for prefix route */
pub const IFA_TARGET_NETNSID: u32 = 10;
pub const IFA_PROTO: u32 = 11; /* u8, address protocol */
pub const __IFA_MAX: u32 = 12;

pub const IFA_MAX: u32 = __IFA_MAX - 1;

/* ifa_flags */
pub const IFA_F_SECONDARY: u32 = 0x01;
pub const IFA_F_TEMPORARY: u32 = IFA_F_SECONDARY;

pub const IFA_F_NODAD: u32 = 0x02;
pub const IFA_F_OPTIMISTIC: u32 = 0x04;
pub const IFA_F_DADFAILED: u32 = 0x08;
pub const IFA_F_HOMEADDRESS: u32 = 0x10;
pub const IFA_F_DEPRECATED: u32 = 0x20;
pub const IFA_F_TENTATIVE: u32 = 0x40;
pub const IFA_F_PERMANENT: u32 = 0x80;
pub const IFA_F_MANAGETEMPADDR: u32 = 0x100;
pub const IFA_F_NOPREFIXROUTE: u32 = 0x200;
pub const IFA_F_MCAUTOJOIN: u32 = 0x400;
pub const IFA_F_STABLE_PRIVACY: u32 = 0x800;

#[repr(C)]
pub struct ifa_cacheinfo {
    pub ifa_prefered: u32,
    pub ifa_valid: u32,
    pub cstamp: u32, /* created timestamp, hundredths of seconds */
    pub tstamp: u32, /* updated timestamp, hundredths of seconds */
}

/* backwards compatibility for userspace */
/* Original C condition: #ifndef __KERNEL__ */
pub unsafe fn IFA_RTA(r: *const ifaddrmsg) -> *mut rtattr {
    ((r as *const u8).add(NLMSG_ALIGN(core::mem::size_of::<ifaddrmsg>()))) as *mut rtattr
}

/* Original C condition: #ifndef __KERNEL__ */
pub unsafe fn IFA_PAYLOAD(n: *const nlmsghdr) -> usize {
    NLMSG_PAYLOAD(n, core::mem::size_of::<ifaddrmsg>())
}

/* ifa_proto */
pub const IFAPROT_UNSPEC: u32 = 0;
pub const IFAPROT_KERNEL_LO: u32 = 1; /* loopback */
pub const IFAPROT_KERNEL_RA: u32 = 2; /* set by kernel from router announcement */
pub const IFAPROT_KERNEL_LL: u32 = 3; /* link-local set by kernel */
