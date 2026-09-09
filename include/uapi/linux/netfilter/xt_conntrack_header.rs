/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/* Header file for kernel module to match connection tracking information.
 * GPL (C) 2001  Marc Boucher (marc@mbsi.ca).
 */

/* Dependencies: linux/types.h, linux/netfilter.h, and
 * linux/netfilter/nf_conntrack_tuple_common.h. */

#[macro_export]
macro_rules! XT_CONNTRACK_STATE_BIT {
    ($ctinfo:expr) => {
        1u32 << (($ctinfo) % IP_CT_IS_REPLY + 1)
    };
}

pub const XT_CONNTRACK_STATE_INVALID: u32 = 1u32 << 0;

pub const XT_CONNTRACK_STATE_SNAT: u32 = 1u32 << (IP_CT_NUMBER + 1);
pub const XT_CONNTRACK_STATE_DNAT: u32 = 1u32 << (IP_CT_NUMBER + 2);
pub const XT_CONNTRACK_STATE_UNTRACKED: u32 = 1u32 << (IP_CT_NUMBER + 3);

/* flags, invflags: */
pub const XT_CONNTRACK_STATE: u32 = 1u32 << 0;
pub const XT_CONNTRACK_PROTO: u32 = 1u32 << 1;
pub const XT_CONNTRACK_ORIGSRC: u32 = 1u32 << 2;
pub const XT_CONNTRACK_ORIGDST: u32 = 1u32 << 3;
pub const XT_CONNTRACK_REPLSRC: u32 = 1u32 << 4;
pub const XT_CONNTRACK_REPLDST: u32 = 1u32 << 5;
pub const XT_CONNTRACK_STATUS: u32 = 1u32 << 6;
pub const XT_CONNTRACK_EXPIRES: u32 = 1u32 << 7;
pub const XT_CONNTRACK_ORIGSRC_PORT: u32 = 1u32 << 8;
pub const XT_CONNTRACK_ORIGDST_PORT: u32 = 1u32 << 9;
pub const XT_CONNTRACK_REPLSRC_PORT: u32 = 1u32 << 10;
pub const XT_CONNTRACK_REPLDST_PORT: u32 = 1u32 << 11;
pub const XT_CONNTRACK_DIRECTION: u32 = 1u32 << 12;
pub const XT_CONNTRACK_STATE_ALIAS: u32 = 1u32 << 13;

#[repr(C)]
pub struct xt_conntrack_mtinfo1 {
    pub origsrc_addr: nf_inet_addr,
    pub origsrc_mask: nf_inet_addr,
    pub origdst_addr: nf_inet_addr,
    pub origdst_mask: nf_inet_addr,
    pub replsrc_addr: nf_inet_addr,
    pub replsrc_mask: nf_inet_addr,
    pub repldst_addr: nf_inet_addr,
    pub repldst_mask: nf_inet_addr,
    pub expires_min: u32,
    pub expires_max: u32,
    pub l4proto: u16,
    pub origsrc_port: __be16,
    pub origdst_port: __be16,
    pub replsrc_port: __be16,
    pub repldst_port: __be16,
    pub match_flags: u16,
    pub invert_flags: u16,
    pub state_mask: u8,
    pub status_mask: u8,
}

#[repr(C)]
pub struct xt_conntrack_mtinfo2 {
    pub origsrc_addr: nf_inet_addr,
    pub origsrc_mask: nf_inet_addr,
    pub origdst_addr: nf_inet_addr,
    pub origdst_mask: nf_inet_addr,
    pub replsrc_addr: nf_inet_addr,
    pub replsrc_mask: nf_inet_addr,
    pub repldst_addr: nf_inet_addr,
    pub repldst_mask: nf_inet_addr,
    pub expires_min: u32,
    pub expires_max: u32,
    pub l4proto: u16,
    pub origsrc_port: __be16,
    pub origdst_port: __be16,
    pub replsrc_port: __be16,
    pub repldst_port: __be16,
    pub match_flags: u16,
    pub invert_flags: u16,
    pub state_mask: u16,
    pub status_mask: u16,
}

#[repr(C)]
pub struct xt_conntrack_mtinfo3 {
    pub origsrc_addr: nf_inet_addr,
    pub origsrc_mask: nf_inet_addr,
    pub origdst_addr: nf_inet_addr,
    pub origdst_mask: nf_inet_addr,
    pub replsrc_addr: nf_inet_addr,
    pub replsrc_mask: nf_inet_addr,
    pub repldst_addr: nf_inet_addr,
    pub repldst_mask: nf_inet_addr,
    pub expires_min: u32,
    pub expires_max: u32,
    pub l4proto: u16,
    pub origsrc_port: u16,
    pub origdst_port: u16,
    pub replsrc_port: u16,
    pub repldst_port: u16,
    pub match_flags: u16,
    pub invert_flags: u16,
    pub state_mask: u16,
    pub status_mask: u16,
    pub origsrc_port_high: u16,
    pub origdst_port_high: u16,
    pub replsrc_port_high: u16,
    pub repldst_port_high: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
