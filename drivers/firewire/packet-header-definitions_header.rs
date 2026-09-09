// SPDX-License-Identifier: GPL-2.0-or-later
//
// packet-header-definitions.h - The definitions of header fields for IEEE 1394 packet.
//
// Copyright (c) 2024 Takashi Sakamoto

// <linux/types.h>

pub const ASYNC_HEADER_QUADLET_COUNT: usize = 4;

pub const ASYNC_HEADER_Q0_DESTINATION_SHIFT: u32 = 16;
pub const ASYNC_HEADER_Q0_DESTINATION_MASK: u32 = 0xffff0000;
pub const ASYNC_HEADER_Q0_TLABEL_SHIFT: u32 = 10;
pub const ASYNC_HEADER_Q0_TLABEL_MASK: u32 = 0x0000fc00;
pub const ASYNC_HEADER_Q0_RETRY_SHIFT: u32 = 8;
pub const ASYNC_HEADER_Q0_RETRY_MASK: u32 = 0x00000300;
pub const ASYNC_HEADER_Q0_TCODE_SHIFT: u32 = 4;
pub const ASYNC_HEADER_Q0_TCODE_MASK: u32 = 0x000000f0;
pub const ASYNC_HEADER_Q0_PRIORITY_SHIFT: u32 = 0;
pub const ASYNC_HEADER_Q0_PRIORITY_MASK: u32 = 0x0000000f;
pub const ASYNC_HEADER_Q1_SOURCE_SHIFT: u32 = 16;
pub const ASYNC_HEADER_Q1_SOURCE_MASK: u32 = 0xffff0000;
pub const ASYNC_HEADER_Q1_RCODE_SHIFT: u32 = 12;
pub const ASYNC_HEADER_Q1_RCODE_MASK: u32 = 0x0000f000;
pub const ASYNC_HEADER_Q1_OFFSET_HIGH_SHIFT: u32 = 0;
pub const ASYNC_HEADER_Q1_OFFSET_HIGH_MASK: u32 = 0x0000ffff;
pub const ASYNC_HEADER_Q3_DATA_LENGTH_SHIFT: u32 = 16;
pub const ASYNC_HEADER_Q3_DATA_LENGTH_MASK: u32 = 0xffff0000;
pub const ASYNC_HEADER_Q3_EXTENDED_TCODE_SHIFT: u32 = 0;
pub const ASYNC_HEADER_Q3_EXTENDED_TCODE_MASK: u32 = 0x0000ffff;

pub unsafe fn async_header_get_destination(header: *const u32) -> u32 { (*header.add(0) & ASYNC_HEADER_Q0_DESTINATION_MASK) >> ASYNC_HEADER_Q0_DESTINATION_SHIFT }
pub unsafe fn async_header_get_tlabel(header: *const u32) -> u32 { (*header.add(0) & ASYNC_HEADER_Q0_TLABEL_MASK) >> ASYNC_HEADER_Q0_TLABEL_SHIFT }
pub unsafe fn async_header_get_retry(header: *const u32) -> u32 { (*header.add(0) & ASYNC_HEADER_Q0_RETRY_MASK) >> ASYNC_HEADER_Q0_RETRY_SHIFT }
pub unsafe fn async_header_get_tcode(header: *const u32) -> u32 { (*header.add(0) & ASYNC_HEADER_Q0_TCODE_MASK) >> ASYNC_HEADER_Q0_TCODE_SHIFT }
pub unsafe fn async_header_get_priority(header: *const u32) -> u32 { (*header.add(0) & ASYNC_HEADER_Q0_PRIORITY_MASK) >> ASYNC_HEADER_Q0_PRIORITY_SHIFT }
pub unsafe fn async_header_get_source(header: *const u32) -> u32 { (*header.add(1) & ASYNC_HEADER_Q1_SOURCE_MASK) >> ASYNC_HEADER_Q1_SOURCE_SHIFT }
pub unsafe fn async_header_get_rcode(header: *const u32) -> u32 { (*header.add(1) & ASYNC_HEADER_Q1_RCODE_MASK) >> ASYNC_HEADER_Q1_RCODE_SHIFT }
pub unsafe fn async_header_get_offset(header: *const u32) -> u64 {
    let hi = (*header.add(1) & ASYNC_HEADER_Q1_OFFSET_HIGH_MASK) >> ASYNC_HEADER_Q1_OFFSET_HIGH_SHIFT;
    ((hi as u64) << 32) | (*header.add(2) as u64)
}
pub unsafe fn async_header_get_quadlet_data(header: *const u32) -> u32 { *header.add(3) }
pub unsafe fn async_header_get_data_length(header: *const u32) -> u32 { (*header.add(3) & ASYNC_HEADER_Q3_DATA_LENGTH_MASK) >> ASYNC_HEADER_Q3_DATA_LENGTH_SHIFT }
pub unsafe fn async_header_get_extended_tcode(header: *const u32) -> u32 { (*header.add(3) & ASYNC_HEADER_Q3_EXTENDED_TCODE_MASK) >> ASYNC_HEADER_Q3_EXTENDED_TCODE_SHIFT }

pub unsafe fn async_header_set_destination(header: *mut u32, destination: u32) { *header.add(0) = (*header.add(0) & !ASYNC_HEADER_Q0_DESTINATION_MASK) | ((destination << ASYNC_HEADER_Q0_DESTINATION_SHIFT) & ASYNC_HEADER_Q0_DESTINATION_MASK); }
pub unsafe fn async_header_set_tlabel(header: *mut u32, tlabel: u32) { *header.add(0) = (*header.add(0) & !ASYNC_HEADER_Q0_TLABEL_MASK) | ((tlabel << ASYNC_HEADER_Q0_TLABEL_SHIFT) & ASYNC_HEADER_Q0_TLABEL_MASK); }
pub unsafe fn async_header_set_retry(header: *mut u32, retry: u32) { *header.add(0) = (*header.add(0) & !ASYNC_HEADER_Q0_RETRY_MASK) | ((retry << ASYNC_HEADER_Q0_RETRY_SHIFT) & ASYNC_HEADER_Q0_RETRY_MASK); }
pub unsafe fn async_header_set_tcode(header: *mut u32, tcode: u32) { *header.add(0) = (*header.add(0) & !ASYNC_HEADER_Q0_TCODE_MASK) | ((tcode << ASYNC_HEADER_Q0_TCODE_SHIFT) & ASYNC_HEADER_Q0_TCODE_MASK); }
pub unsafe fn async_header_set_priority(header: *mut u32, priority: u32) { *header.add(0) = (*header.add(0) & !ASYNC_HEADER_Q0_PRIORITY_MASK) | ((priority << ASYNC_HEADER_Q0_PRIORITY_SHIFT) & ASYNC_HEADER_Q0_PRIORITY_MASK); }
pub unsafe fn async_header_set_source(header: *mut u32, source: u32) { *header.add(1) = (*header.add(1) & !ASYNC_HEADER_Q1_SOURCE_MASK) | ((source << ASYNC_HEADER_Q1_SOURCE_SHIFT) & ASYNC_HEADER_Q1_SOURCE_MASK); }
pub unsafe fn async_header_set_rcode(header: *mut u32, rcode: u32) { *header.add(1) = (*header.add(1) & !ASYNC_HEADER_Q1_RCODE_MASK) | ((rcode << ASYNC_HEADER_Q1_RCODE_SHIFT) & ASYNC_HEADER_Q1_RCODE_MASK); }
pub unsafe fn async_header_set_offset(header: *mut u32, offset: u64) {
    let hi = (offset >> 32) as u32;
    *header.add(1) = (*header.add(1) & !ASYNC_HEADER_Q1_OFFSET_HIGH_MASK) | ((hi << ASYNC_HEADER_Q1_OFFSET_HIGH_SHIFT) & ASYNC_HEADER_Q1_OFFSET_HIGH_MASK);
    *header.add(2) = (offset & 0x00000000ffffffff) as u32;
}
pub unsafe fn async_header_set_quadlet_data(header: *mut u32, quadlet_data: u32) { *header.add(3) = quadlet_data; }
pub unsafe fn async_header_set_data_length(header: *mut u32, data_length: u32) { *header.add(3) = (*header.add(3) & !ASYNC_HEADER_Q3_DATA_LENGTH_MASK) | ((data_length << ASYNC_HEADER_Q3_DATA_LENGTH_SHIFT) & ASYNC_HEADER_Q3_DATA_LENGTH_MASK); }
pub unsafe fn async_header_set_extended_tcode(header: *mut u32, extended_tcode: u32) { *header.add(3) = (*header.add(3) & !ASYNC_HEADER_Q3_EXTENDED_TCODE_MASK) | ((extended_tcode << ASYNC_HEADER_Q3_EXTENDED_TCODE_SHIFT) & ASYNC_HEADER_Q3_EXTENDED_TCODE_MASK); }

pub const ISOC_HEADER_DATA_LENGTH_SHIFT: u32 = 16;
pub const ISOC_HEADER_DATA_LENGTH_MASK: u32 = 0xffff0000;
pub const ISOC_HEADER_TAG_SHIFT: u32 = 14;
pub const ISOC_HEADER_TAG_MASK: u32 = 0x0000c000;
pub const ISOC_HEADER_CHANNEL_SHIFT: u32 = 8;
pub const ISOC_HEADER_CHANNEL_MASK: u32 = 0x00003f00;
pub const ISOC_HEADER_TCODE_SHIFT: u32 = 4;
pub const ISOC_HEADER_TCODE_MASK: u32 = 0x000000f0;
pub const ISOC_HEADER_SY_SHIFT: u32 = 0;
pub const ISOC_HEADER_SY_MASK: u32 = 0x0000000f;

pub fn isoc_header_get_data_length(header: u32) -> u32 { (header & ISOC_HEADER_DATA_LENGTH_MASK) >> ISOC_HEADER_DATA_LENGTH_SHIFT }
pub fn isoc_header_get_tag(header: u32) -> u32 { (header & ISOC_HEADER_TAG_MASK) >> ISOC_HEADER_TAG_SHIFT }
pub fn isoc_header_get_channel(header: u32) -> u32 { (header & ISOC_HEADER_CHANNEL_MASK) >> ISOC_HEADER_CHANNEL_SHIFT }
pub fn isoc_header_get_tcode(header: u32) -> u32 { (header & ISOC_HEADER_TCODE_MASK) >> ISOC_HEADER_TCODE_SHIFT }
pub fn isoc_header_get_sy(header: u32) -> u32 { (header & ISOC_HEADER_SY_MASK) >> ISOC_HEADER_SY_SHIFT }

pub unsafe fn isoc_header_set_data_length(header: *mut u32, data_length: u32) { *header &= !ISOC_HEADER_DATA_LENGTH_MASK; *header |= (data_length << ISOC_HEADER_DATA_LENGTH_SHIFT) & ISOC_HEADER_DATA_LENGTH_MASK; }
pub unsafe fn isoc_header_set_tag(header: *mut u32, tag: u32) { *header &= !ISOC_HEADER_TAG_MASK; *header |= (tag << ISOC_HEADER_TAG_SHIFT) & ISOC_HEADER_TAG_MASK; }
pub unsafe fn isoc_header_set_channel(header: *mut u32, channel: u32) { *header &= !ISOC_HEADER_CHANNEL_MASK; *header |= (channel << ISOC_HEADER_CHANNEL_SHIFT) & ISOC_HEADER_CHANNEL_MASK; }
pub unsafe fn isoc_header_set_tcode(header: *mut u32, tcode: u32) { *header &= !ISOC_HEADER_TCODE_MASK; *header |= (tcode << ISOC_HEADER_TCODE_SHIFT) & ISOC_HEADER_TCODE_MASK; }
pub unsafe fn isoc_header_set_sy(header: *mut u32, sy: u32) { *header &= !ISOC_HEADER_SY_MASK; *header |= (sy << ISOC_HEADER_SY_SHIFT) & ISOC_HEADER_SY_MASK; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
