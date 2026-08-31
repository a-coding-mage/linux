/* SPDX-License-Identifier: GPL-2.0 */

// C header guard/default definitions:
// ETH_P_IP, ETH_P_IPV6, ETH_P_8021Q, and ETH_P_8021AD are defined here only
// when not already supplied by included headers.
pub const ETH_P_IP: u32 = 0x0800;
pub const ETH_P_IPV6: u32 = 0x86DD;
pub const ETH_P_8021Q: u32 = 0x8100;
pub const ETH_P_8021AD: u32 = 0x88A8;

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

/* Non-existent checksum status */
pub const XDP_CHECKSUM_MAGIC: u32 = BIT(2);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum xdp_meta_field {
    XDP_META_FIELD_TS = BIT(0) as isize,
    XDP_META_FIELD_RSS = BIT(1) as isize,
    XDP_META_FIELD_VLAN_TAG = BIT(2) as isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xdp_meta_rx_timestamp_union {
    pub rx_timestamp: __u64,
    pub rx_timestamp_err: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xdp_meta_rx_hash_union {
    pub rx_hash_type: __u32,
    pub rx_hash_err: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_meta_rx_vlan_tag {
    pub rx_vlan_proto: __be16,
    pub rx_vlan_tci: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xdp_meta_rx_vlan_tag_union {
    pub rx_vlan_tag: xdp_meta_rx_vlan_tag,
    pub rx_vlan_tag_err: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_meta {
    pub rx_timestamp_union: xdp_meta_rx_timestamp_union,
    pub xdp_timestamp: __u64,
    pub rx_hash: __u32,
    pub rx_hash_union: xdp_meta_rx_hash_union,
    pub rx_vlan_tag_union: xdp_meta_rx_vlan_tag_union,
    pub hint_valid: xdp_meta_field,
}
