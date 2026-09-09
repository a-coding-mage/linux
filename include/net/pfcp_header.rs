/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */
// use uapi_linux_if_ether::ethhdr;
// use net_dst_metadata::*;
// use linux_netdevice::net_device;
// use uapi_linux_ipv6::ipv6hdr;
// use net_udp_tunnel::udp_hdr;
// use uapi_linux_udp::udphdr;
// use uapi_linux_ip::iphdr;
// use linux_types::*;
// use linux_bits::*;

pub const PFCP_PORT: u16 = 8805;

/* PFCP protocol header */
#[repr(C)]
pub struct pfcphdr {
    pub flags: u8,
    pub message_type: u8,
    pub message_length: __be16,
}

/* PFCP header flags */
pub const PFCP_SEID_FLAG: u8 = 1 << 0;
pub const PFCP_MP_FLAG: u8 = 1 << 1;

pub const PFCP_VERSION_MASK: u8 = (1 << 5) - 1;

pub const PFCP_HLEN: usize = core::mem::size_of::<udphdr>() + core::mem::size_of::<pfcphdr>();

/* PFCP node related messages */
#[repr(C)]
pub struct pfcphdr_node {
    pub seq_number: [u8; 3],
    pub reserved: u8,
}

/* PFCP session related messages */
#[repr(C)]
pub struct pfcphdr_session {
    pub seid: __be64,
    pub seq_number: [u8; 3],
    /* C bitfields occupy one byte; the bit ordering is target-endian. */
    pub message_priority_and_reserved: u8,
}

#[repr(C, packed)]
pub struct pfcp_metadata {
    pub type_: u8,
    pub seid: __be64,
}

pub const PFCP_TYPE_NODE: u32 = 0;
pub const PFCP_TYPE_SESSION: u32 = 1;

pub const PFCP_HEADROOM: usize = core::mem::size_of::<iphdr>()
    + core::mem::size_of::<udphdr>()
    + core::mem::size_of::<pfcphdr>()
    + core::mem::size_of::<ethhdr>();

pub const PFCP6_HEADROOM: usize = core::mem::size_of::<ipv6hdr>()
    + core::mem::size_of::<udphdr>()
    + core::mem::size_of::<pfcphdr>()
    + core::mem::size_of::<ethhdr>();

#[inline]
pub unsafe fn pfcp_hdr(skb: *mut sk_buff) -> *mut pfcphdr {
    (udp_hdr(skb)).add(1) as *mut pfcphdr
}

#[inline]
pub unsafe fn pfcp_hdr_node(skb: *mut sk_buff) -> *mut pfcphdr_node {
    pfcp_hdr(skb).add(1) as *mut pfcphdr_node
}

#[inline]
pub unsafe fn pfcp_hdr_session(skb: *mut sk_buff) -> *mut pfcphdr_session {
    pfcp_hdr(skb).add(1) as *mut pfcphdr_session
}

#[inline]
pub unsafe fn netif_is_pfcp(dev: *const net_device) -> bool {
    !(*dev).rtnl_link_ops.is_null()
        && !strcmp((*(*dev).rtnl_link_ops).kind, b"pfcp\0".as_ptr() as *const i8)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
