/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Dependencies supplied by the surrounding UAPI translation.

/// Check throughput meter return code for error.
#[inline]
pub const fn batadv_tp_is_error(n: u8) -> u8 {
    if n > 127 { 1 } else { 0 }
}

#[repr(u32)]
pub enum batadv_packettype {
    BATADV_IV_OGM = 0x00,
    BATADV_BCAST = 0x01,
    BATADV_CODED = 0x02,
    BATADV_ELP = 0x03,
    BATADV_OGM2 = 0x04,
    BATADV_MCAST = 0x05,
    BATADV_UNICAST = 0x40,
    BATADV_UNICAST_FRAG = 0x41,
    BATADV_UNICAST_4ADDR = 0x42,
    BATADV_ICMP = 0x43,
    BATADV_UNICAST_TVLV = 0x44,
}
pub const BATADV_UNICAST_MIN: u32 = 0x40;
pub const BATADV_UNICAST_MAX: u32 = 0x7f;

#[repr(u32)]
pub enum batadv_subtype {
    BATADV_P_DATA = 0x01,
    BATADV_P_DAT_DHT_GET = 0x02,
    BATADV_P_DAT_DHT_PUT = 0x03,
    BATADV_P_DAT_CACHE_REPLY = 0x04,
}

pub const BATADV_COMPAT_VERSION: u32 = 15;

#[repr(u64)]
pub enum batadv_iv_flags {
    BATADV_NOT_BEST_NEXT_HOP = 1u64 << 0,
    BATADV_PRIMARIES_FIRST_HOP = 1u64 << 1,
    BATADV_DIRECTLINK = 1u64 << 2,
}

#[repr(u32)]
pub enum batadv_icmp_packettype {
    BATADV_ECHO_REPLY = 0,
    BATADV_DESTINATION_UNREACHABLE = 3,
    BATADV_ECHO_REQUEST = 8,
    BATADV_TTL_EXCEEDED = 11,
    BATADV_PARAMETER_PROBLEM = 12,
    BATADV_TP = 15,
}

#[repr(u64)]
pub enum batadv_mcast_flags {
    BATADV_MCAST_WANT_ALL_UNSNOOPABLES = 1u64 << 0,
    BATADV_MCAST_WANT_ALL_IPV4 = 1u64 << 1,
    BATADV_MCAST_WANT_ALL_IPV6 = 1u64 << 2,
    BATADV_MCAST_WANT_NO_RTR4 = 1u64 << 3,
    BATADV_MCAST_WANT_NO_RTR6 = 1u64 << 4,
    BATADV_MCAST_HAVE_MC_PTYPE_CAPA = 1u64 << 5,
}

pub const BATADV_TT_DATA_TYPE_MASK: u8 = 0x0F;

#[repr(u64)]
pub enum batadv_tt_data_flags {
    BATADV_TT_OGM_DIFF = 1u64 << 0,
    BATADV_TT_REQUEST = 1u64 << 1,
    BATADV_TT_RESPONSE = 1u64 << 2,
    BATADV_TT_FULL_TABLE = 1u64 << 4,
}

#[repr(u64)]
pub enum batadv_vlan_flags {
    BATADV_VLAN_HAS_TAG = 1u64 << 15,
}

#[repr(u32)]
pub enum batadv_bla_claimframe {
    BATADV_CLAIM_TYPE_CLAIM = 0x00,
    BATADV_CLAIM_TYPE_UNCLAIM = 0x01,
    BATADV_CLAIM_TYPE_ANNOUNCE = 0x02,
    BATADV_CLAIM_TYPE_REQUEST = 0x03,
    BATADV_CLAIM_TYPE_LOOPDETECT = 0x04,
}

#[repr(u32)]
pub enum batadv_tvlv_type {
    BATADV_TVLV_GW = 0x01,
    BATADV_TVLV_DAT = 0x02,
    BATADV_TVLV_NC = 0x03,
    BATADV_TVLV_TT = 0x04,
    BATADV_TVLV_ROAM = 0x05,
    BATADV_TVLV_MCAST = 0x06,
    BATADV_TVLV_MCAST_TRACKER = 0x07,
}

#[repr(C, packed(2))]
pub struct batadv_bla_claim_dst { pub magic: [u8; 3], pub r#type: u8, pub group: u16 }

#[repr(C, packed(2))]
pub struct batadv_ogm_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub flags: u8, pub seqno: u32, pub orig: [u8; ETH_ALEN], pub prev_sender: [u8; ETH_ALEN], pub reserved: u8, pub tq: u8, pub tvlv_len: u16 }
pub const BATADV_OGM_HLEN: usize = core::mem::size_of::<batadv_ogm_packet>();

#[repr(C, packed(2))]
pub struct batadv_ogm2_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub flags: u8, pub seqno: u32, pub orig: [u8; ETH_ALEN], pub tvlv_len: u16, pub throughput: u32 }
pub const BATADV_OGM2_HLEN: usize = core::mem::size_of::<batadv_ogm2_packet>();

#[repr(C, packed(2))]
pub struct batadv_elp_packet { pub packet_type: u8, pub version: u8, pub orig: [u8; ETH_ALEN], pub seqno: u32, pub elp_interval: u32 }
pub const BATADV_ELP_HLEN: usize = core::mem::size_of::<batadv_elp_packet>();

#[repr(C, packed(2))]
pub struct batadv_icmp_header { pub packet_type: u8, pub version: u8, pub ttl: u8, pub msg_type: u8, pub dst: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub uid: u8, pub align: [u8; 3] }

#[repr(C, packed(2))]
pub struct batadv_icmp_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub msg_type: u8, pub dst: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub uid: u8, pub reserved: u8, pub seqno: u16 }

#[repr(C, packed(2))]
pub struct batadv_icmp_tp_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub msg_type: u8, pub dst: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub uid: u8, pub subtype: u8, pub session: [u8; 2], pub seqno: u32, pub timestamp: u32 }

#[repr(u32)]
pub enum batadv_icmp_tp_subtype { BATADV_TP_MSG = 0, BATADV_TP_ACK }

pub const BATADV_RR_LEN: usize = 16;
#[repr(C, packed(2))]
pub struct batadv_icmp_packet_rr { pub packet_type: u8, pub version: u8, pub ttl: u8, pub msg_type: u8, pub dst: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub uid: u8, pub rr_cur: u8, pub seqno: u16, pub rr: [[u8; ETH_ALEN]; BATADV_RR_LEN] }
pub const BATADV_ICMP_MAX_PACKET_SIZE: usize = core::mem::size_of::<batadv_icmp_packet_rr>();

#[repr(C, packed(2))]
pub struct batadv_unicast_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub ttvn: u8, pub dest: [u8; ETH_ALEN] }

#[repr(C, packed(2))]
pub struct batadv_unicast_4addr_packet { pub u: batadv_unicast_packet, pub src: [u8; ETH_ALEN], pub subtype: u8, pub reserved: u8 }

#[repr(C, packed(2))]
pub struct batadv_frag_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub flags: u8, pub dest: [u8; ETH_ALEN], pub orig: [u8; ETH_ALEN], pub seqno: u16, pub total_size: u16 }

#[repr(C, packed(2))]
pub struct batadv_bcast_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub reserved: u8, pub seqno: u32, pub orig: [u8; ETH_ALEN] }

#[repr(C, packed(2))]
pub struct batadv_mcast_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub reserved: u8, pub tvlv_len: u16 }

#[repr(C, packed(2))]
pub struct batadv_coded_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub first_ttvn: u8, pub first_source: [u8; ETH_ALEN], pub first_orig_dest: [u8; ETH_ALEN], pub first_crc: u32, pub second_ttl: u8, pub second_ttvn: u8, pub second_dest: [u8; ETH_ALEN], pub second_source: [u8; ETH_ALEN], pub second_orig_dest: [u8; ETH_ALEN], pub second_crc: u32, pub coded_len: u16 }

#[repr(C, packed(2))]
pub struct batadv_unicast_tvlv_packet { pub packet_type: u8, pub version: u8, pub ttl: u8, pub reserved: u8, pub dst: [u8; ETH_ALEN], pub src: [u8; ETH_ALEN], pub tvlv_len: u16, pub align: u16 }

#[repr(C, packed(2))]
pub struct batadv_tvlv_hdr { pub r#type: u8, pub version: u8, pub len: u16 }
#[repr(C, packed(2))]
pub struct batadv_tvlv_gateway_data { pub bandwidth_down: u32, pub bandwidth_up: u32 }
#[repr(C, packed(2))]
pub struct batadv_tvlv_tt_vlan_data { pub crc: u32, pub vid: u16, pub reserved: u16 }
#[repr(C, packed(2))]
pub struct batadv_tvlv_tt_data { pub flags: u8, pub ttvn: u8, pub num_vlan: u16, pub vlan_data: [batadv_tvlv_tt_vlan_data; 0] }
#[repr(C, packed(2))]
pub struct batadv_tvlv_tt_change { pub flags: u8, pub reserved: [u8; 3], pub addr: [u8; ETH_ALEN], pub vid: u16 }
#[repr(C, packed(2))]
pub struct batadv_tvlv_roam_adv { pub client: [u8; ETH_ALEN], pub vid: u16 }
#[repr(C, packed(2))]
pub struct batadv_tvlv_mcast_data { pub flags: u8, pub reserved: [u8; 3] }
#[repr(C, packed(2))]
pub struct batadv_tvlv_mcast_tracker { pub num_dests: u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
