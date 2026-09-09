// SPDX-License-Identifier: GPL-2.0-or-later
//
// phy-packet-definitions.h - The definitions of phy packet for IEEE 1394.
//
// Copyright (c) 2024 Takashi Sakamoto

pub const PACKET_IDENTIFIER_MASK: u32 = 0xc0000000;
pub const PACKET_IDENTIFIER_SHIFT: u32 = 30;

#[inline]
pub fn phy_packet_get_packet_identifier(quadlet: u32) -> u32 {
    (quadlet & PACKET_IDENTIFIER_MASK) >> PACKET_IDENTIFIER_SHIFT
}

#[inline]
pub unsafe fn phy_packet_set_packet_identifier(quadlet: *mut u32, packet_identifier: u32) {
    *quadlet &= !PACKET_IDENTIFIER_MASK;
    *quadlet |= (packet_identifier << PACKET_IDENTIFIER_SHIFT) & PACKET_IDENTIFIER_MASK;
}

pub const PHY_PACKET_PACKET_IDENTIFIER_PHY_CONFIG: u32 = 0;
pub const PHY_CONFIG_ROOT_ID_MASK: u32 = 0x3f000000;
pub const PHY_CONFIG_ROOT_ID_SHIFT: u32 = 24;
pub const PHY_CONFIG_FORCE_ROOT_NODE_MASK: u32 = 0x00800000;
pub const PHY_CONFIG_FORCE_ROOT_NODE_SHIFT: u32 = 23;
pub const PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK: u32 = 0x00400000;
pub const PHY_CONFIG_GAP_COUNT_OPTIMIZATION_SHIFT: u32 = 22;
pub const PHY_CONFIG_GAP_COUNT_MASK: u32 = 0x003f0000;
pub const PHY_CONFIG_GAP_COUNT_SHIFT: u32 = 16;

#[inline] pub fn phy_packet_phy_config_get_root_id(q: u32) -> u32 { (q & PHY_CONFIG_ROOT_ID_MASK) >> PHY_CONFIG_ROOT_ID_SHIFT }
#[inline] pub unsafe fn phy_packet_phy_config_set_root_id(q: *mut u32, v: u32) { *q = (*q & !PHY_CONFIG_ROOT_ID_MASK) | ((v << PHY_CONFIG_ROOT_ID_SHIFT) & PHY_CONFIG_ROOT_ID_MASK); }
#[inline] pub fn phy_packet_phy_config_get_force_root_node(q: u32) -> bool { ((q & PHY_CONFIG_FORCE_ROOT_NODE_MASK) >> PHY_CONFIG_FORCE_ROOT_NODE_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_phy_config_set_force_root_node(q: *mut u32, v: bool) { *q = (*q & !PHY_CONFIG_FORCE_ROOT_NODE_MASK) | (((v as u32) << PHY_CONFIG_FORCE_ROOT_NODE_SHIFT) & PHY_CONFIG_FORCE_ROOT_NODE_MASK); }
#[inline] pub fn phy_packet_phy_config_get_gap_count_optimization(q: u32) -> bool { ((q & PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK) >> PHY_CONFIG_GAP_COUNT_OPTIMIZATION_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_phy_config_set_gap_count_optimization(q: *mut u32, v: bool) { *q = (*q & !PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK) | (((v as u32) << PHY_CONFIG_GAP_COUNT_OPTIMIZATION_SHIFT) & PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK); }
#[inline] pub fn phy_packet_phy_config_get_gap_count(q: u32) -> u32 { (q & PHY_CONFIG_GAP_COUNT_MASK) >> PHY_CONFIG_GAP_COUNT_SHIFT }
#[inline] pub unsafe fn phy_packet_phy_config_set_gap_count(q: *mut u32, v: u32) { *q = (*q & !PHY_CONFIG_GAP_COUNT_MASK) | ((v << PHY_CONFIG_GAP_COUNT_SHIFT) & PHY_CONFIG_GAP_COUNT_MASK); }

pub const PHY_PACKET_PACKET_IDENTIFIER_SELF_ID: u32 = 2;
pub const SELF_ID_PHY_ID_MASK: u32 = 0x3f000000;
pub const SELF_ID_PHY_ID_SHIFT: u32 = 24;
pub const SELF_ID_EXTENDED_MASK: u32 = 0x00800000;
pub const SELF_ID_EXTENDED_SHIFT: u32 = 23;
pub const SELF_ID_MORE_PACKETS_MASK: u32 = 0x00000001;
pub const SELF_ID_MORE_PACKETS_SHIFT: u32 = 0;
pub const SELF_ID_ZERO_LINK_ACTIVE_MASK: u32 = 0x00400000;
pub const SELF_ID_ZERO_LINK_ACTIVE_SHIFT: u32 = 22;
pub const SELF_ID_ZERO_GAP_COUNT_MASK: u32 = 0x003f0000;
pub const SELF_ID_ZERO_GAP_COUNT_SHIFT: u32 = 16;
pub const SELF_ID_ZERO_SCODE_MASK: u32 = 0x0000c000;
pub const SELF_ID_ZERO_SCODE_SHIFT: u32 = 14;
pub const SELF_ID_ZERO_CONTENDER_MASK: u32 = 0x00000800;
pub const SELF_ID_ZERO_CONTENDER_SHIFT: u32 = 11;
pub const SELF_ID_ZERO_POWER_CLASS_MASK: u32 = 0x00000700;
pub const SELF_ID_ZERO_POWER_CLASS_SHIFT: u32 = 8;
pub const SELF_ID_ZERO_INITIATED_RESET_MASK: u32 = 0x00000002;
pub const SELF_ID_ZERO_INITIATED_RESET_SHIFT: u32 = 1;
pub const SELF_ID_EXTENDED_SEQUENCE_MASK: u32 = 0x00700000;
pub const SELF_ID_EXTENDED_SEQUENCE_SHIFT: u32 = 20;
pub const SELF_ID_PORT_STATUS_MASK: u32 = 0x3;
pub const SELF_ID_SEQUENCE_MAXIMUM_QUADLET_COUNT: u32 = 4;

#[inline] pub fn phy_packet_self_id_get_phy_id(q: u32) -> u32 { (q & SELF_ID_PHY_ID_MASK) >> SELF_ID_PHY_ID_SHIFT }
#[inline] pub unsafe fn phy_packet_self_id_set_phy_id(q: *mut u32, v: u32) { *q = (*q & !SELF_ID_PHY_ID_MASK) | ((v << SELF_ID_PHY_ID_SHIFT) & SELF_ID_PHY_ID_MASK); }
#[inline] pub fn phy_packet_self_id_get_extended(q: u32) -> bool { ((q & SELF_ID_EXTENDED_MASK) >> SELF_ID_EXTENDED_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_self_id_set_extended(q: *mut u32, v: bool) { *q = (*q & !SELF_ID_EXTENDED_MASK) | (((v as u32) << SELF_ID_EXTENDED_SHIFT) & SELF_ID_EXTENDED_MASK); }

#[inline] pub fn phy_packet_self_id_zero_get_link_active(q: u32) -> bool { ((q & SELF_ID_ZERO_LINK_ACTIVE_MASK) >> SELF_ID_ZERO_LINK_ACTIVE_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_link_active(q: *mut u32, v: bool) { *q = (*q & !SELF_ID_ZERO_LINK_ACTIVE_MASK) | (((v as u32) << SELF_ID_ZERO_LINK_ACTIVE_SHIFT) & SELF_ID_ZERO_LINK_ACTIVE_MASK); }
#[inline] pub fn phy_packet_self_id_zero_get_gap_count(q: u32) -> u32 { (q & SELF_ID_ZERO_GAP_COUNT_MASK) >> SELF_ID_ZERO_GAP_COUNT_SHIFT }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_gap_count(q: *mut u32, v: u32) { *q = (*q & !SELF_ID_ZERO_GAP_COUNT_MASK) | ((v << SELF_ID_ZERO_GAP_COUNT_SHIFT) & SELF_ID_ZERO_GAP_COUNT_MASK); }
#[inline] pub fn phy_packet_self_id_zero_get_scode(q: u32) -> u32 { (q & SELF_ID_ZERO_SCODE_MASK) >> SELF_ID_ZERO_SCODE_SHIFT }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_scode(q: *mut u32, v: u32) { *q = (*q & !SELF_ID_ZERO_SCODE_MASK) | ((v << SELF_ID_ZERO_SCODE_SHIFT) & SELF_ID_ZERO_SCODE_MASK); }
#[inline] pub fn phy_packet_self_id_zero_get_contender(q: u32) -> bool { ((q & SELF_ID_ZERO_CONTENDER_MASK) >> SELF_ID_ZERO_CONTENDER_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_contender(q: *mut u32, v: bool) { *q = (*q & !SELF_ID_ZERO_CONTENDER_MASK) | (((v as u32) << SELF_ID_ZERO_CONTENDER_SHIFT) & SELF_ID_ZERO_CONTENDER_MASK); }
#[inline] pub fn phy_packet_self_id_zero_get_power_class(q: u32) -> u32 { (q & SELF_ID_ZERO_POWER_CLASS_MASK) >> SELF_ID_ZERO_POWER_CLASS_SHIFT }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_power_class(q: *mut u32, v: u32) { *q = (*q & !SELF_ID_ZERO_POWER_CLASS_MASK) | ((v << SELF_ID_ZERO_POWER_CLASS_SHIFT) & SELF_ID_ZERO_POWER_CLASS_MASK); }
#[inline] pub fn phy_packet_self_id_zero_get_initiated_reset(q: u32) -> bool { ((q & SELF_ID_ZERO_INITIATED_RESET_MASK) >> SELF_ID_ZERO_INITIATED_RESET_SHIFT) != 0 }
#[inline] pub unsafe fn phy_packet_self_id_zero_set_initiated_reset(q: *mut u32, v: bool) { *q = (*q & !SELF_ID_ZERO_INITIATED_RESET_MASK) | (((v as u32) << SELF_ID_ZERO_INITIATED_RESET_SHIFT) & SELF_ID_ZERO_INITIATED_RESET_MASK); }
#[inline] pub fn phy_packet_self_id_get_more_packets(q: u32) -> bool { (q & SELF_ID_MORE_PACKETS_MASK) != 0 }
#[inline] pub unsafe fn phy_packet_self_id_set_more_packets(q: *mut u32, v: bool) { *q = (*q & !SELF_ID_MORE_PACKETS_MASK) | (v as u32); }
#[inline] pub fn phy_packet_self_id_extended_get_sequence(q: u32) -> u32 { (q & SELF_ID_EXTENDED_SEQUENCE_MASK) >> SELF_ID_EXTENDED_SEQUENCE_SHIFT }
#[inline] pub unsafe fn phy_packet_self_id_extended_set_sequence(q: *mut u32, v: u32) { *q = (*q & !SELF_ID_EXTENDED_SEQUENCE_MASK) | ((v << SELF_ID_EXTENDED_SHIFT) & SELF_ID_EXTENDED_SEQUENCE_MASK); }

#[repr(C)]
pub struct self_id_sequence_enumerator { pub cursor: *const u32, pub quadlet_count: u32 }

#[inline]
pub unsafe fn self_id_sequence_enumerator_next(e: *mut self_id_sequence_enumerator, quadlet_count: *mut u32) -> *const u32 {
    if (*e).cursor.is_null() || (*e).quadlet_count == 0 { return core::ptr::null(); }
    let start = (*e).cursor; let mut cursor = start; let mut count = 1u32; let mut sequence = 0u32;
    let mut quadlet = *cursor;
    while phy_packet_self_id_get_more_packets(quadlet) {
        if count >= (*e).quadlet_count || count >= SELF_ID_SEQUENCE_MAXIMUM_QUADLET_COUNT { return core::ptr::null(); }
        cursor = cursor.add(1); count += 1; quadlet = *cursor;
        if !phy_packet_self_id_get_extended(quadlet) || sequence != phy_packet_self_id_extended_get_sequence(quadlet) { return core::ptr::null(); }
        sequence += 1;
    }
    *quadlet_count = count; (*e).cursor = (*e).cursor.add(count as usize); (*e).quadlet_count -= count; start
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_packet_self_id_port_status { NONE = 0, NCONN = 1, PARENT = 2, CHILD = 3 }

#[inline] pub fn self_id_sequence_get_port_capacity(quadlet_count: u32) -> u32 { quadlet_count * 8 - 5 }
#[inline] pub unsafe fn self_id_sequence_get_port_status(s: *const u32, count: u32, port: u32) -> phy_packet_self_id_port_status {
    let index = (port + 5) / 8; let shift = 16 - ((port + 5) % 8) * 2;
    if index < count && index < SELF_ID_SEQUENCE_MAXIMUM_QUADLET_COUNT { match ((*s.add(index as usize) >> shift) & SELF_ID_PORT_STATUS_MASK) { 1 => phy_packet_self_id_port_status::NCONN, 2 => phy_packet_self_id_port_status::PARENT, 3 => phy_packet_self_id_port_status::CHILD, _ => phy_packet_self_id_port_status::NONE } } else { phy_packet_self_id_port_status::NONE }
}
#[inline] pub unsafe fn self_id_sequence_set_port_status(s: *mut u32, count: u32, port: u32, status: phy_packet_self_id_port_status) {
    let index = (port + 5) / 8; let shift = 16 - ((port + 5) % 8) * 2;
    if index < count { let q = s.add(index as usize); *q &= !(SELF_ID_PORT_STATUS_MASK << shift); *q |= (status as u32) << shift; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
