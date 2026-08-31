/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted.
// C dependency omitted: <stdint.h>

pub const fn array_length<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

pub const ACK_NO_ACK: u32 = 0x0;

pub const fn ACK_DONE(a: u32) -> bool {
    (a >> 2) == 0
}

pub const fn ACK_BUSY(a: u32) -> bool {
    (a >> 2) == 1
}

pub const fn ACK_ERROR(a: u32) -> bool {
    (a >> 2) == 3
}

#[repr(C)]
pub struct phy_packet {
    pub timestamp: u32,
    pub u: phy_packet_u,
    pub inverted: u32,
    pub ack: u32,
}

#[repr(C)]
pub union phy_packet_u {
    pub common: phy_packet_common,
    pub link_on: phy_packet_common,
    pub phy_config: phy_packet_phy_config,
    pub self_id: phy_packet_self_id,
    pub ext_self_id: phy_packet_ext_self_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_packet_common {
    // C bitfields in this u32:
    // zero:24, phy_id:6, identifier:2
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_packet_phy_config {
    // C bitfields in this u32:
    // zero:16, gap_count:6, set_gap_count:1, set_root:1, root_id:6, identifier:2
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_packet_self_id {
    // C bitfields in this u32:
    // more_packets:1, initiated_reset:1, port2:2, port1:2, port0:2,
    // power_class:3, contender:1, phy_delay:2, phy_speed:2, gap_count:6,
    // link_active:1, extended:1, phy_id:6, identifier:2
    pub bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_packet_ext_self_id {
    // C bitfields in this u32:
    // more_packets:1, reserved1:1, porth:2, portg:2, portf:2, porte:2,
    // portd:2, portc:2, portb:2, porta:2, reserved0:2, sequence:3,
    // extended:1, phy_id:6, identifier:2
    pub bits: u32,
}

pub const TCODE_PHY_PACKET: u32 = 0x10;

pub const PHY_PACKET_CONFIGURATION: u32 = 0x00;
pub const PHY_PACKET_LINK_ON: u32 = 0x01;
pub const PHY_PACKET_SELF_ID: u32 = 0x02;

#[repr(C)]
pub struct link_packet {
    pub timestamp: u32,
    pub u: link_packet_u,
}

#[repr(C)]
pub union link_packet_u {
    pub common: link_packet_common,
    pub read_quadlet: link_packet_read_quadlet,
    pub read_quadlet_response: link_packet_read_quadlet_response,
    pub read_block: link_packet_read_block,
    pub read_block_response: link_packet_read_block_response,
    pub write_quadlet: link_packet_write_quadlet,
    pub write_block: link_packet_write_block,
    pub write_response: link_packet_write_response,
    pub cycle_start: link_packet_cycle_start,
    pub iso_data: link_packet_iso_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_common {
    // C bitfields in this u32:
    // priority:4, tcode:4, rt:2, tlabel:6, destination:16
    pub header: u32,
    // C bitfields in this u32:
    // offset_high:16, source:16
    pub offset_high_source: u32,
    pub offset_low: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_read_quadlet {
    pub common: [u32; 3],
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_read_quadlet_response {
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_read_block {
    pub common: [u32; 3],
    // C bitfields in this u32:
    // extended_tcode:16, data_length:16
    pub extended_tcode_data_length: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_read_block_response {
    pub common: [u32; 3],
    // C bitfields in this u32:
    // extended_tcode:16, data_length:16
    pub extended_tcode_data_length: u32,
    pub crc: u32,
    pub data: [u32; 0],
    /* crc and ack follows. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_write_quadlet {
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_write_block {
    pub common: [u32; 3],
    // C bitfields in this u32:
    // extended_tcode:16, data_length:16
    pub extended_tcode_data_length: u32,
    pub crc: u32,
    pub data: [u32; 0],
    /* crc and ack follows. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_write_response {
    pub common: [u32; 3],
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_cycle_start {
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet_iso_data {
    // C bitfields in this u32:
    // sy:4, tcode:4, channel:6, tag:2, data_length:16
    pub header: u32,
    pub crc: u32,
}

#[repr(C)]
pub struct subaction {
    pub ack: u32,
    pub length: usize,
    pub link: list,
    pub packet: link_packet,
}

#[repr(C)]
pub struct link_transaction {
    pub request_node: ::std::os::raw::c_int,
    pub response_node: ::std::os::raw::c_int,
    pub tlabel: ::std::os::raw::c_int,
    pub request: *mut subaction,
    pub response: *mut subaction,
    pub request_list: list,
    pub response_list: list,
    pub link: list,
}

unsafe extern "C" {
    pub fn decode_fcp(t: *mut link_transaction) -> ::std::os::raw::c_int;
}
