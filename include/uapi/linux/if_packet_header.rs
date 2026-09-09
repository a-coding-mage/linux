/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the Linux UAPI header. Required type aliases are supplied by linux/types.h.

#[repr(C)]
pub struct sockaddr_pkt {
    pub spkt_family: u16,
    pub spkt_device: [u8; 14],
    pub spkt_protocol: u16,
}

#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8],
}

pub const PACKET_HOST: i32 = 0;
pub const PACKET_BROADCAST: i32 = 1;
pub const PACKET_MULTICAST: i32 = 2;
pub const PACKET_OTHERHOST: i32 = 3;
pub const PACKET_OUTGOING: i32 = 4;
pub const PACKET_LOOPBACK: i32 = 5;
pub const PACKET_USER: i32 = 6;
pub const PACKET_KERNEL: i32 = 7;
pub const PACKET_FASTROUTE: i32 = 6;

pub const PACKET_ADD_MEMBERSHIP: i32 = 1;
pub const PACKET_DROP_MEMBERSHIP: i32 = 2;
pub const PACKET_RECV_OUTPUT: i32 = 3;
pub const PACKET_RX_RING: i32 = 5;
pub const PACKET_STATISTICS: i32 = 6;
pub const PACKET_COPY_THRESH: i32 = 7;
pub const PACKET_AUXDATA: i32 = 8;
pub const PACKET_ORIGDEV: i32 = 9;
pub const PACKET_VERSION: i32 = 10;
pub const PACKET_HDRLEN: i32 = 11;
pub const PACKET_RESERVE: i32 = 12;
pub const PACKET_TX_RING: i32 = 13;
pub const PACKET_LOSS: i32 = 14;
pub const PACKET_VNET_HDR: i32 = 15;
pub const PACKET_TX_TIMESTAMP: i32 = 16;
pub const PACKET_TIMESTAMP: i32 = 17;
pub const PACKET_FANOUT: i32 = 18;
pub const PACKET_TX_HAS_OFF: i32 = 19;
pub const PACKET_QDISC_BYPASS: i32 = 20;
pub const PACKET_ROLLOVER_STATS: i32 = 21;
pub const PACKET_FANOUT_DATA: i32 = 22;
pub const PACKET_IGNORE_OUTGOING: i32 = 23;
pub const PACKET_VNET_HDR_SZ: i32 = 24;

pub const PACKET_FANOUT_HASH: i32 = 0;
pub const PACKET_FANOUT_LB: i32 = 1;
pub const PACKET_FANOUT_CPU: i32 = 2;
pub const PACKET_FANOUT_ROLLOVER: i32 = 3;
pub const PACKET_FANOUT_RND: i32 = 4;
pub const PACKET_FANOUT_QM: i32 = 5;
pub const PACKET_FANOUT_CBPF: i32 = 6;
pub const PACKET_FANOUT_EBPF: i32 = 7;
pub const PACKET_FANOUT_FLAG_ROLLOVER: u16 = 0x1000;
pub const PACKET_FANOUT_FLAG_UNIQUEID: u16 = 0x2000;
pub const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: u16 = 0x4000;
pub const PACKET_FANOUT_FLAG_DEFRAG: u16 = 0x8000;

#[repr(C)]
pub struct tpacket_stats { pub tp_packets: u32, pub tp_drops: u32 }
#[repr(C)]
pub struct tpacket_stats_v3 { pub tp_packets: u32, pub tp_drops: u32, pub tp_freeze_q_cnt: u32 }
#[repr(C)]
pub struct tpacket_rollover_stats { pub tp_all: u64, pub tp_huge: u64, pub tp_failed: u64 }
#[repr(C)]
pub union tpacket_stats_u { pub stats1: tpacket_stats, pub stats3: tpacket_stats_v3 }

#[repr(C)]
pub struct tpacket_auxdata {
    pub tp_status: u32, pub tp_len: u32, pub tp_snaplen: u32,
    pub tp_mac: u16, pub tp_net: u16, pub tp_vlan_tci: u16, pub tp_vlan_tpid: u16,
}

pub const TP_STATUS_KERNEL: u32 = 0;
pub const TP_STATUS_USER: u32 = 1 << 0;
pub const TP_STATUS_COPY: u32 = 1 << 1;
pub const TP_STATUS_LOSING: u32 = 1 << 2;
pub const TP_STATUS_CSUMNOTREADY: u32 = 1 << 3;
pub const TP_STATUS_VLAN_VALID: u32 = 1 << 4;
pub const TP_STATUS_BLK_TMO: u32 = 1 << 5;
pub const TP_STATUS_VLAN_TPID_VALID: u32 = 1 << 6;
pub const TP_STATUS_CSUM_VALID: u32 = 1 << 7;
pub const TP_STATUS_GSO_TCP: u32 = 1 << 8;
pub const TP_STATUS_AVAILABLE: u32 = 0;
pub const TP_STATUS_SEND_REQUEST: u32 = 1 << 0;
pub const TP_STATUS_SENDING: u32 = 1 << 1;
pub const TP_STATUS_WRONG_FORMAT: u32 = 1 << 2;
pub const TP_STATUS_TS_SOFTWARE: u32 = 1 << 29;
pub const TP_STATUS_TS_SYS_HARDWARE: u32 = 1 << 30;
pub const TP_STATUS_TS_RAW_HARDWARE: u32 = 1u32 << 31;
pub const TP_FT_REQ_FILL_RXHASH: u32 = 0x1;

#[repr(C)]
pub struct tpacket_hdr {
    pub tp_status: usize, pub tp_len: u32, pub tp_snaplen: u32,
    pub tp_mac: u16, pub tp_net: u16, pub tp_sec: u32, pub tp_usec: u32,
}

pub const TPACKET_ALIGNMENT: usize = 16;
#[inline]
pub const fn TPACKET_ALIGN(x: usize) -> usize { (x + TPACKET_ALIGNMENT - 1) & !(TPACKET_ALIGNMENT - 1) }
pub const TPACKET_HDRLEN: usize = TPACKET_ALIGN(core::mem::size_of::<tpacket_hdr>()) + core::mem::size_of::<sockaddr_ll>();

#[repr(C)]
pub struct tpacket2_hdr {
    pub tp_status: u32, pub tp_len: u32, pub tp_snaplen: u32,
    pub tp_mac: u16, pub tp_net: u16, pub tp_sec: u32, pub tp_nsec: u32,
    pub tp_vlan_tci: u16, pub tp_vlan_tpid: u16, pub tp_padding: [u8; 4],
}
#[repr(C)]
pub struct tpacket_hdr_variant1 { pub tp_rxhash: u32, pub tp_vlan_tci: u32, pub tp_vlan_tpid: u16, pub tp_padding: u16 }
#[repr(C)]
pub union tpacket3_hdr__bindgen_ty_1 { pub hv1: tpacket_hdr_variant1 }
#[repr(C)]
pub struct tpacket3_hdr {
    pub tp_next_offset: u32, pub tp_sec: u32, pub tp_nsec: u32, pub tp_snaplen: u32,
    pub tp_len: u32, pub tp_status: u32, pub tp_mac: u16, pub tp_net: u16,
    pub hv1: tpacket3_hdr__bindgen_ty_1, pub tp_padding: [u8; 8],
}

#[repr(C)]
pub union tpacket_bd_ts__bindgen_ty_1 { pub ts_usec: u32, pub ts_nsec: u32 }
#[repr(C)]
pub struct tpacket_bd_ts { pub ts_sec: u32, pub ts_usec: tpacket_bd_ts__bindgen_ty_1 }

#[repr(C)]
pub struct tpacket_hdr_v1 {
    pub block_status: u32, pub num_pkts: u32, pub offset_to_first_pkt: u32, pub blk_len: u32,
    pub seq_num: u64, pub ts_first_pkt: tpacket_bd_ts, pub ts_last_pkt: tpacket_bd_ts,
}
#[repr(C)]
pub union tpacket_bd_header_u { pub bh1: tpacket_hdr_v1 }
#[repr(C)]
pub struct tpacket_block_desc { pub version: u32, pub offset_to_priv: u32, pub hdr: tpacket_bd_header_u }

pub const TPACKET2_HDRLEN: usize = TPACKET_ALIGN(core::mem::size_of::<tpacket2_hdr>()) + core::mem::size_of::<sockaddr_ll>();
pub const TPACKET3_HDRLEN: usize = TPACKET_ALIGN(core::mem::size_of::<tpacket3_hdr>()) + core::mem::size_of::<sockaddr_ll>();

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tpacket_versions { TPACKET_V1, TPACKET_V2, TPACKET_V3 }

#[repr(C)]
pub struct tpacket_req { pub tp_block_size: u32, pub tp_block_nr: u32, pub tp_frame_size: u32, pub tp_frame_nr: u32 }
#[repr(C)]
pub struct tpacket_req3 { pub tp_block_size: u32, pub tp_block_nr: u32, pub tp_frame_size: u32, pub tp_frame_nr: u32, pub tp_retire_blk_tov: u32, pub tp_sizeof_priv: u32, pub tp_feature_req_word: u32 }
#[repr(C)]
pub union tpacket_req_u { pub req: tpacket_req, pub req3: tpacket_req3 }
#[repr(C)]
pub struct packet_mreq { pub mr_ifindex: i32, pub mr_type: u16, pub mr_alen: u16, pub mr_address: [u8; 8] }

#[repr(C)]
pub struct fanout_args {
    // __LITTLE_ENDIAN_BITFIELD places id first; big-endian places type_flags first.
    pub id: u16,
    pub type_flags: u16,
    pub max_num_members: u32,
}

pub const PACKET_MR_MULTICAST: i32 = 0;
pub const PACKET_MR_PROMISC: i32 = 1;
pub const PACKET_MR_ALLMULTI: i32 = 2;
pub const PACKET_MR_UNICAST: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
