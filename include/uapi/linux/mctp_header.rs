/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Management Component Transport Protocol (MCTP)
 *
 * Copyright (c) 2021 Code Construct
 * Copyright (c) 2021 Google
 */

// Dependencies supplied by the surrounding Linux UAPI translation:
// __kernel_sa_family_t, MAX_ADDR_LEN, and SIOCPROTOPRIVATE.

pub type mctp_eid_t = u8;

#[repr(C)]
pub struct mctp_addr {
    pub s_addr: mctp_eid_t,
}

#[repr(C)]
pub struct sockaddr_mctp {
    pub smctp_family: __kernel_sa_family_t,
    pub __smctp_pad0: u16,
    pub smctp_network: u32,
    pub smctp_addr: mctp_addr,
    pub smctp_type: u8,
    pub smctp_tag: u8,
    pub __smctp_pad1: u8,
}

#[repr(C)]
pub struct sockaddr_mctp_ext {
    pub smctp_base: sockaddr_mctp,
    pub smctp_ifindex: i32,
    pub smctp_halen: u8,
    pub __smctp_pad0: [u8; 3],
    pub smctp_haddr: [u8; MAX_ADDR_LEN],
}

/* A "fully qualified" MCTP address, which includes the system-local network ID,
 * required to uniquely resolve a routable EID.
 */
#[repr(C)]
pub struct mctp_fq_addr {
    pub net: u32,
    pub eid: mctp_eid_t,
}

pub const MCTP_NET_ANY: u32 = 0x0;

pub const MCTP_ADDR_NULL: u8 = 0x00;
pub const MCTP_ADDR_ANY: u8 = 0xff;

pub const MCTP_TAG_MASK: u8 = 0x07;
pub const MCTP_TAG_OWNER: u8 = 0x08;
pub const MCTP_TAG_PREALLOC: u8 = 0x10;

pub const MCTP_OPT_ADDR_EXT: u32 = 1;

pub const SIOCMCTPALLOCTAG: u32 = SIOCPROTOPRIVATE + 0;
pub const SIOCMCTPDROPTAG: u32 = SIOCPROTOPRIVATE + 1;
pub const SIOCMCTPALLOCTAG2: u32 = SIOCPROTOPRIVATE + 2;
pub const SIOCMCTPDROPTAG2: u32 = SIOCPROTOPRIVATE + 3;

/* Deprecated: use mctp_ioc_tag_ctl2 / TAG2 ioctls instead, which defines the
 * MCTP network ID as part of the allocated tag. Using this assumes the default
 * net ID for allocated tags, which may not give correct behaviour on system
 * with multiple networks configured.
 */
#[repr(C)]
pub struct mctp_ioc_tag_ctl {
    pub peer_addr: mctp_eid_t,

    /* For SIOCMCTPALLOCTAG: must be passed as zero, kernel will
     * populate with the allocated tag value. Returned tag value will
     * always have TO and PREALLOC set.
     *
     * For SIOCMCTPDROPTAG: userspace provides tag value to drop, from
     * a prior SIOCMCTPALLOCTAG call (and so must have TO and PREALLOC set).
     */
    pub tag: u8,
    pub flags: u16,
}

#[repr(C)]
pub struct mctp_ioc_tag_ctl2 {
    /* Peer details: network ID, peer EID, local EID. All set by the
     * caller.
     *
     * Local EID must be MCTP_ADDR_NULL or MCTP_ADDR_ANY in current
     * kernels.
     */
    pub net: u32,
    pub peer_addr: mctp_eid_t,
    pub local_addr: mctp_eid_t,

    /* Set by caller, but no flags defined currently. Must be 0 */
    pub flags: u16,

    /* For SIOCMCTPALLOCTAG2: must be passed as zero, kernel will
     * populate with the allocated tag value. Returned tag value will
     * always have TO and PREALLOC set.
     *
     * For SIOCMCTPDROPTAG2: userspace provides tag value to drop, from
     * a prior SIOCMCTPALLOCTAG2 call (and so must have TO and PREALLOC set).
     */
    pub tag: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
