/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_echo {
    pub identifier: u16,
    pub sequence: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_nd_advt {
    /* C bitfields; layout depends on the target byte order. */
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_nd_ra {
    pub hop_limit: u8,
    /* C bitfields; layout depends on the target byte order. */
    pub flags: u8,
    pub rt_lifetime: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union icmp6_dataun {
    pub un_data32: [u32; 1],
    pub un_data16: [u16; 2],
    pub un_data8: [u8; 4],
    pub u_echo: icmpv6_echo,
    pub u_nd_advt: icmpv6_nd_advt,
    pub u_nd_ra: icmpv6_nd_ra,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6hdr {
    pub icmp6_type: u8,
    pub icmp6_code: u8,
    pub icmp6_cksum: u16,
    pub icmp6_dataun: icmp6_dataun,
}

impl icmp6hdr {
    pub unsafe fn icmp6_identifier(&self) -> u16 { self.icmp6_dataun.u_echo.identifier }
    pub unsafe fn icmp6_sequence(&self) -> u16 { self.icmp6_dataun.u_echo.sequence }
    pub unsafe fn icmp6_pointer(&self) -> u32 { self.icmp6_dataun.un_data32[0] }
    pub unsafe fn icmp6_mtu(&self) -> u32 { self.icmp6_dataun.un_data32[0] }
    pub unsafe fn icmp6_unused(&self) -> u32 { self.icmp6_dataun.un_data32[0] }
    pub unsafe fn icmp6_maxdelay(&self) -> u16 { self.icmp6_dataun.un_data16[0] }
    pub unsafe fn icmp6_datagram_len(&self) -> u8 { self.icmp6_dataun.un_data8[0] }
    pub unsafe fn icmp6_router(&self) -> u32 { self.icmp6_dataun.u_nd_advt.bits }
    pub unsafe fn icmp6_solicited(&self) -> u32 { self.icmp6_dataun.u_nd_advt.bits }
    pub unsafe fn icmp6_override(&self) -> u32 { self.icmp6_dataun.u_nd_advt.bits }
    pub unsafe fn icmp6_ndiscreserved(&self) -> u32 { self.icmp6_dataun.u_nd_advt.bits }
    pub unsafe fn icmp6_hop_limit(&self) -> u8 { self.icmp6_dataun.u_nd_ra.hop_limit }
    pub unsafe fn icmp6_addrconf_managed(&self) -> u8 { self.icmp6_dataun.u_nd_ra.flags }
    pub unsafe fn icmp6_addrconf_other(&self) -> u8 { self.icmp6_dataun.u_nd_ra.flags }
    pub unsafe fn icmp6_rt_lifetime(&self) -> u16 { self.icmp6_dataun.u_nd_ra.rt_lifetime }
    pub unsafe fn icmp6_router_pref(&self) -> u8 { self.icmp6_dataun.u_nd_ra.flags }
}

pub const ICMPV6_ROUTER_PREF_LOW: u32 = 0x3;
pub const ICMPV6_ROUTER_PREF_MEDIUM: u32 = 0x0;
pub const ICMPV6_ROUTER_PREF_HIGH: u32 = 0x1;
pub const ICMPV6_ROUTER_PREF_INVALID: u32 = 0x2;
pub const ICMPV6_DEST_UNREACH: u32 = 1;
pub const ICMPV6_PKT_TOOBIG: u32 = 2;
pub const ICMPV6_TIME_EXCEED: u32 = 3;
pub const ICMPV6_PARAMPROB: u32 = 4;
pub const ICMPV6_ERRMSG_MAX: u32 = 127;
pub const ICMPV6_INFOMSG_MASK: u32 = 0x80;
pub const ICMPV6_ECHO_REQUEST: u32 = 128;
pub const ICMPV6_ECHO_REPLY: u32 = 129;
pub const ICMPV6_MGM_QUERY: u32 = 130;
pub const ICMPV6_MGM_REPORT: u32 = 131;
pub const ICMPV6_MGM_REDUCTION: u32 = 132;
pub const ICMPV6_NI_QUERY: u32 = 139;
pub const ICMPV6_NI_REPLY: u32 = 140;
pub const ICMPV6_MLD2_REPORT: u32 = 143;
pub const ICMPV6_DHAAD_REQUEST: u32 = 144;
pub const ICMPV6_DHAAD_REPLY: u32 = 145;
pub const ICMPV6_MOBILE_PREFIX_SOL: u32 = 146;
pub const ICMPV6_MOBILE_PREFIX_ADV: u32 = 147;
pub const ICMPV6_MRDISC_ADV: u32 = 151;
pub const ICMPV6_MRDISC_SOL: u32 = 152;
pub const ICMPV6_MSG_MAX: u32 = 255;
pub const ICMPV6_NOROUTE: u32 = 0;
pub const ICMPV6_ADM_PROHIBITED: u32 = 1;
pub const ICMPV6_NOT_NEIGHBOUR: u32 = 2;
pub const ICMPV6_ADDR_UNREACH: u32 = 3;
pub const ICMPV6_PORT_UNREACH: u32 = 4;
pub const ICMPV6_POLICY_FAIL: u32 = 5;
pub const ICMPV6_REJECT_ROUTE: u32 = 6;
pub const ICMPV6_EXC_HOPLIMIT: u32 = 0;
pub const ICMPV6_EXC_FRAGTIME: u32 = 1;
pub const ICMPV6_HDR_FIELD: u32 = 0;
pub const ICMPV6_UNK_NEXTHDR: u32 = 1;
pub const ICMPV6_UNK_OPTION: u32 = 2;
pub const ICMPV6_HDR_INCOMP: u32 = 3;
pub const ICMPV6_EXT_ECHO_REQUEST: u32 = 160;
pub const ICMPV6_EXT_ECHO_REPLY: u32 = 161;
pub const ICMPV6_FILTER: u32 = 1;
pub const ICMPV6_FILTER_BLOCK: u32 = 1;
pub const ICMPV6_FILTER_PASS: u32 = 2;
pub const ICMPV6_FILTER_BLOCKOTHERS: u32 = 3;
pub const ICMPV6_FILTER_PASSONLY: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6_filter {
    pub data: [u32; 8],
}

pub const MLD2_MODE_IS_INCLUDE: u32 = 1;
pub const MLD2_MODE_IS_EXCLUDE: u32 = 2;
pub const MLD2_CHANGE_TO_INCLUDE: u32 = 3;
pub const MLD2_CHANGE_TO_EXCLUDE: u32 = 4;
pub const MLD2_ALLOW_NEW_SOURCES: u32 = 5;
pub const MLD2_BLOCK_OLD_SOURCES: u32 = 6;

/* MLD2_ALL_MCR_INIT: {{ { { 0xff,0x02,0,0,0,0,0,0,0,0,0,0,0,0,0,0x16 } } }} */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
