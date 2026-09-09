/* SPDX-License-Identifier: MIT */
/* Translation of xen/interface/io/netif.h. */

/* Dependency: ring.h and grant_table.h provide ring definitions and grant_ref_t. */

pub const XEN_NETIF_NR_SLOTS_MIN: u32 = 18;
pub const XEN_NETIF_MAX_XDP_HEADROOM: u32 = 0x7fff;

pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV4: u32 = 0;
pub const XEN_NETIF_CTRL_HASH_TYPE_IPV4: u32 = 1 << _XEN_NETIF_CTRL_HASH_TYPE_IPV4;
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV4_TCP: u32 = 1;
pub const XEN_NETIF_CTRL_HASH_TYPE_IPV4_TCP: u32 = 1 << _XEN_NETIF_CTRL_HASH_TYPE_IPV4_TCP;
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV6: u32 = 2;
pub const XEN_NETIF_CTRL_HASH_TYPE_IPV6: u32 = 1 << _XEN_NETIF_CTRL_HASH_TYPE_IPV6;
pub const _XEN_NETIF_CTRL_HASH_TYPE_IPV6_TCP: u32 = 3;
pub const XEN_NETIF_CTRL_HASH_TYPE_IPV6_TCP: u32 = 1 << _XEN_NETIF_CTRL_HASH_TYPE_IPV6_TCP;

pub const XEN_NETIF_CTRL_HASH_ALGORITHM_NONE: u32 = 0;
pub const XEN_NETIF_CTRL_HASH_ALGORITHM_TOEPLITZ: u32 = 1;

#[cfg(feature = "xen_netif_define_toeplitz")]
pub unsafe fn xen_netif_toeplitz_hash(
    key: *const u8, keylen: u32, buf: *const u8, buflen: u32,
) -> u32 {
    let mut keyi: u32 = 0;
    let mut prefix: u64 = 0;
    let mut hash: u64 = 0;
    while keyi < 8 {
        prefix <<= 8;
        prefix |= if keyi < keylen { *key.add(keyi as usize) as u64 } else { 0 };
        keyi += 1;
    }
    let mut bufi = 0;
    while bufi < buflen {
        let mut byte = *buf.add(bufi as usize);
        let mut bit = 0;
        while bit < 8 {
            if byte & 0x80 != 0 { hash ^= prefix; }
            prefix <<= 1;
            byte <<= 1;
            bit += 1;
        }
        prefix |= if keyi < keylen { *key.add(keyi as usize) as u64 } else { 0 };
        keyi += 1;
        bufi += 1;
    }
    (hash >> 32) as u32
}

#[repr(C)]
pub struct xen_netif_ctrl_request { pub id: u16, pub type_: u16, pub data: [u32; 3] }
pub const XEN_NETIF_CTRL_TYPE_INVALID: u32 = 0;
pub const XEN_NETIF_CTRL_TYPE_GET_HASH_FLAGS: u32 = 1;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_FLAGS: u32 = 2;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_KEY: u32 = 3;
pub const XEN_NETIF_CTRL_TYPE_GET_HASH_MAPPING_SIZE: u32 = 4;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING_SIZE: u32 = 5;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_MAPPING: u32 = 6;
pub const XEN_NETIF_CTRL_TYPE_SET_HASH_ALGORITHM: u32 = 7;

#[repr(C)]
pub struct xen_netif_ctrl_response { pub id: u16, pub type_: u16, pub status: u32, pub data: u32 }
pub const XEN_NETIF_CTRL_STATUS_SUCCESS: u32 = 0;
pub const XEN_NETIF_CTRL_STATUS_NOT_SUPPORTED: u32 = 1;
pub const XEN_NETIF_CTRL_STATUS_INVALID_PARAMETER: u32 = 2;
pub const XEN_NETIF_CTRL_STATUS_BUFFER_OVERFLOW: u32 = 3;

/* DEFINE_RING_TYPES(xen_netif_ctrl, xen_netif_ctrl_request, xen_netif_ctrl_response); */

pub const _XEN_NETTXF_csum_blank: u32 = 0;
pub const XEN_NETTXF_csum_blank: u32 = 1 << _XEN_NETTXF_csum_blank;
pub const _XEN_NETTXF_data_validated: u32 = 1;
pub const XEN_NETTXF_data_validated: u32 = 1 << _XEN_NETTXF_data_validated;
pub const _XEN_NETTXF_more_data: u32 = 2;
pub const XEN_NETTXF_more_data: u32 = 1 << _XEN_NETTXF_more_data;
pub const _XEN_NETTXF_extra_info: u32 = 3;
pub const XEN_NETTXF_extra_info: u32 = 1 << _XEN_NETTXF_extra_info;
pub const XEN_NETIF_MAX_TX_SIZE: u32 = 0xffff;

#[repr(C)]
pub struct xen_netif_tx_request { pub gref: grant_ref_t, pub offset: u16, pub flags: u16, pub id: u16, pub size: u16 }

pub const XEN_NETIF_EXTRA_TYPE_NONE: u8 = 0;
pub const XEN_NETIF_EXTRA_TYPE_GSO: u8 = 1;
pub const XEN_NETIF_EXTRA_TYPE_MCAST_ADD: u8 = 2;
pub const XEN_NETIF_EXTRA_TYPE_MCAST_DEL: u8 = 3;
pub const XEN_NETIF_EXTRA_TYPE_HASH: u8 = 4;
pub const XEN_NETIF_EXTRA_TYPE_XDP: u8 = 5;
pub const XEN_NETIF_EXTRA_TYPE_MAX: u8 = 6;
pub const _XEN_NETIF_EXTRA_FLAG_MORE: u8 = 0;
pub const XEN_NETIF_EXTRA_FLAG_MORE: u8 = 1 << _XEN_NETIF_EXTRA_FLAG_MORE;
pub const XEN_NETIF_GSO_TYPE_NONE: u8 = 0;
pub const XEN_NETIF_GSO_TYPE_TCPV4: u8 = 1;
pub const XEN_NETIF_GSO_TYPE_TCPV6: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xen_netif_extra_info_u {
    pub gso: xen_netif_extra_info_gso,
    pub mcast: xen_netif_extra_info_mcast,
    pub hash: xen_netif_extra_info_hash,
    pub xdp: xen_netif_extra_info_xdp,
    pub pad: [u16; 3],
}
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_netif_extra_info_gso { pub size: u16, pub type_: u8, pub pad: u8, pub features: u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_netif_extra_info_mcast { pub addr: [u8; 6] }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_netif_extra_info_hash { pub type_: u8, pub algorithm: u8, pub value: [u8; 4] }
#[repr(C)] #[derive(Copy, Clone)] pub struct xen_netif_extra_info_xdp { pub headroom: u16, pub pad: [u16; 2] }
#[repr(C)] pub struct xen_netif_extra_info { pub type_: u8, pub flags: u8, pub u: xen_netif_extra_info_u }

#[repr(C)] pub struct xen_netif_tx_response { pub id: u16, pub status: i16 }
#[repr(C)] pub struct xen_netif_rx_request { pub id: u16, pub pad: u16, pub gref: grant_ref_t }
pub const _XEN_NETRXF_data_validated: u32 = 0;
pub const XEN_NETRXF_data_validated: u32 = 1 << _XEN_NETRXF_data_validated;
pub const _XEN_NETRXF_csum_blank: u32 = 1;
pub const XEN_NETRXF_csum_blank: u32 = 1 << _XEN_NETRXF_csum_blank;
pub const _XEN_NETRXF_more_data: u32 = 2;
pub const XEN_NETRXF_more_data: u32 = 1 << _XEN_NETRXF_more_data;
pub const _XEN_NETRXF_extra_info: u32 = 3;
pub const XEN_NETRXF_extra_info: u32 = 1 << _XEN_NETRXF_extra_info;
pub const _XEN_NETRXF_gso_prefix: u32 = 4;
pub const XEN_NETRXF_gso_prefix: u32 = 1 << _XEN_NETRXF_gso_prefix;
#[repr(C)] pub struct xen_netif_rx_response { pub id: u16, pub offset: u16, pub flags: u16, pub status: i16 }

/* DEFINE_RING_TYPES(xen_netif_tx, xen_netif_tx_request, xen_netif_tx_response); */
/* DEFINE_RING_TYPES(xen_netif_rx, xen_netif_rx_request, xen_netif_rx_response); */
pub const XEN_NETIF_RSP_DROPPED: i16 = -2;
pub const XEN_NETIF_RSP_ERROR: i16 = -1;
pub const XEN_NETIF_RSP_OKAY: i16 = 0;
pub const XEN_NETIF_RSP_NULL: i16 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
