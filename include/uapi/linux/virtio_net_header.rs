/* This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers. */

// Dependencies supplied by the surrounding UAPI translation.

/* The feature bitmap for virtio net */
pub const VIRTIO_NET_F_CSUM: u32 = 0;
pub const VIRTIO_NET_F_GUEST_CSUM: u32 = 1;
pub const VIRTIO_NET_F_CTRL_GUEST_OFFLOADS: u32 = 2;
pub const VIRTIO_NET_F_MTU: u32 = 3;
pub const VIRTIO_NET_F_MAC: u32 = 5;
pub const VIRTIO_NET_F_GUEST_TSO4: u32 = 7;
pub const VIRTIO_NET_F_GUEST_TSO6: u32 = 8;
pub const VIRTIO_NET_F_GUEST_ECN: u32 = 9;
pub const VIRTIO_NET_F_GUEST_UFO: u32 = 10;
pub const VIRTIO_NET_F_HOST_TSO4: u32 = 11;
pub const VIRTIO_NET_F_HOST_TSO6: u32 = 12;
pub const VIRTIO_NET_F_HOST_ECN: u32 = 13;
pub const VIRTIO_NET_F_HOST_UFO: u32 = 14;
pub const VIRTIO_NET_F_MRG_RXBUF: u32 = 15;
pub const VIRTIO_NET_F_STATUS: u32 = 16;
pub const VIRTIO_NET_F_CTRL_VQ: u32 = 17;
pub const VIRTIO_NET_F_CTRL_RX: u32 = 18;
pub const VIRTIO_NET_F_CTRL_VLAN: u32 = 19;
pub const VIRTIO_NET_F_CTRL_RX_EXTRA: u32 = 20;
pub const VIRTIO_NET_F_GUEST_ANNOUNCE: u32 = 21;
pub const VIRTIO_NET_F_MQ: u32 = 22;
pub const VIRTIO_NET_F_CTRL_MAC_ADDR: u32 = 23;
pub const VIRTIO_NET_F_DEVICE_STATS: u32 = 50;
pub const VIRTIO_NET_F_VQ_NOTF_COAL: u32 = 52;
pub const VIRTIO_NET_F_NOTF_COAL: u32 = 53;
pub const VIRTIO_NET_F_GUEST_USO4: u32 = 54;
pub const VIRTIO_NET_F_GUEST_USO6: u32 = 55;
pub const VIRTIO_NET_F_HOST_USO: u32 = 56;
pub const VIRTIO_NET_F_HASH_REPORT: u32 = 57;
pub const VIRTIO_NET_F_GUEST_HDRLEN: u32 = 59;
pub const VIRTIO_NET_F_RSS: u32 = 60;
pub const VIRTIO_NET_F_RSC_EXT: u32 = 61;
pub const VIRTIO_NET_F_STANDBY: u32 = 62;
pub const VIRTIO_NET_F_SPEED_DUPLEX: u32 = 63;
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO: u32 = 65;
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_CSUM: u32 = 66;
pub const VIRTIO_NET_F_HOST_UDP_TUNNEL_GSO: u32 = 67;
pub const VIRTIO_NET_F_HOST_UDP_TUNNEL_GSO_CSUM: u32 = 68;
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_MAPPED: u32 = 46;
pub const VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_CSUM_MAPPED: u32 = 47;

// The following item is present unless VIRTIO_NET_NO_LEGACY is defined.
pub const VIRTIO_NET_F_GSO: u32 = 6;

pub const VIRTIO_NET_S_LINK_UP: u32 = 1;
pub const VIRTIO_NET_S_ANNOUNCE: u32 = 2;

pub const VIRTIO_NET_RSS_HASH_TYPE_IPv4: u32 = 1 << 0;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCPv4: u32 = 1 << 1;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDPv4: u32 = 1 << 2;
pub const VIRTIO_NET_RSS_HASH_TYPE_IPv6: u32 = 1 << 3;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCPv6: u32 = 1 << 4;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDPv6: u32 = 1 << 5;
pub const VIRTIO_NET_RSS_HASH_TYPE_IP_EX: u32 = 1 << 6;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCP_EX: u32 = 1 << 7;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDP_EX: u32 = 1 << 8;

#[repr(C, packed)]
pub struct virtio_net_config {
    pub mac: [u8; 6],
    pub status: u16,
    pub max_virtqueue_pairs: u16,
    pub mtu: u16,
    pub speed: u32,
    pub duplex: u8,
    pub rss_max_key_size: u8,
    pub rss_max_indirection_table_length: u16,
    pub supported_hash_types: u32,
}

pub const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8 = 1;
pub const VIRTIO_NET_HDR_F_DATA_VALID: u8 = 2;
pub const VIRTIO_NET_HDR_F_RSC_INFO: u8 = 4;
pub const VIRTIO_NET_HDR_F_UDP_TUNNEL_CSUM: u8 = 8;
pub const VIRTIO_NET_HDR_GSO_NONE: u8 = 0;
pub const VIRTIO_NET_HDR_GSO_TCPV4: u8 = 1;
pub const VIRTIO_NET_HDR_GSO_UDP: u8 = 3;
pub const VIRTIO_NET_HDR_GSO_TCPV6: u8 = 4;
pub const VIRTIO_NET_HDR_GSO_UDP_L4: u8 = 5;
pub const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4: u8 = 0x20;
pub const VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6: u8 = 0x40;
pub const VIRTIO_NET_HDR_GSO_UDP_TUNNEL: u8 = VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4 | VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6;
pub const VIRTIO_NET_HDR_GSO_ECN: u8 = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_csum { pub csum_start: u16, pub csum_offset: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_csum_named { pub start: u16, pub offset: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_rsc { pub segments: u16, pub dup_acks: u16 }
#[repr(C)]
pub union virtio_net_hdr_v1_union {
    pub csum: virtio_net_hdr_v1_csum,
    pub csum_named: virtio_net_hdr_v1_csum_named,
    pub rsc: virtio_net_hdr_v1_rsc,
}

#[repr(C)]
pub struct virtio_net_hdr_v1 {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: u16,
    pub gso_size: u16,
    pub csum: virtio_net_hdr_v1_union,
    pub num_buffers: u16,
}

pub const VIRTIO_NET_HASH_REPORT_NONE: u16 = 0;
pub const VIRTIO_NET_HASH_REPORT_IPv4: u16 = 1;
pub const VIRTIO_NET_HASH_REPORT_TCPv4: u16 = 2;
pub const VIRTIO_NET_HASH_REPORT_UDPv4: u16 = 3;
pub const VIRTIO_NET_HASH_REPORT_IPv6: u16 = 4;
pub const VIRTIO_NET_HASH_REPORT_TCPv6: u16 = 5;
pub const VIRTIO_NET_HASH_REPORT_UDPv6: u16 = 6;
pub const VIRTIO_NET_HASH_REPORT_IPv6_EX: u16 = 7;
pub const VIRTIO_NET_HASH_REPORT_TCPv6_EX: u16 = 8;
pub const VIRTIO_NET_HASH_REPORT_UDPv6_EX: u16 = 9;

#[repr(C)]
pub struct virtio_net_hdr_v1_hash {
    pub hdr: virtio_net_hdr_v1,
    pub hash_value_lo: u16,
    pub hash_value_hi: u16,
    pub hash_report: u16,
    pub padding: u16,
}

#[repr(C)]
pub struct virtio_net_hdr_v1_hash_tunnel {
    pub hash_hdr: virtio_net_hdr_v1_hash,
    pub outer_th_offset: u16,
    pub inner_nh_offset: u16,
}

// Legacy declarations, present unless VIRTIO_NET_NO_LEGACY is defined.
#[repr(C)]
pub struct virtio_net_hdr {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: u16,
    pub gso_size: u16,
    pub csum_start: u16,
    pub csum_offset: u16,
}

#[repr(C)]
pub struct virtio_net_hdr_mrg_rxbuf {
    pub hdr: virtio_net_hdr,
    pub num_buffers: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
