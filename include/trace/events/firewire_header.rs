// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Takashi Sakamoto
//
// Rust translation of trace/events/firewire.h.  The Linux tracepoint
// declaration machinery is supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependencies supplied by other translated headers:
// ASYNC_HEADER_* and SELF_ID_* masks/shifts, FW_ISO_CONTEXT_* constants,
// fw_iso_context, fw_iso_packet, self_id_sequence_get_port_*.

pub const QUADLET_SIZE: usize = 4;

#[inline]
pub unsafe fn ASYNC_HEADER_GET_DESTINATION(header: *const u32) -> u32 {
    ((*header & ASYNC_HEADER_Q0_DESTINATION_MASK) >> ASYNC_HEADER_Q0_DESTINATION_SHIFT) as u32
}
#[inline]
pub unsafe fn ASYNC_HEADER_GET_TLABEL(header: *const u32) -> u32 {
    ((*header & ASYNC_HEADER_Q0_TLABEL_MASK) >> ASYNC_HEADER_Q0_TLABEL_SHIFT) as u32
}
#[inline]
pub unsafe fn ASYNC_HEADER_GET_TCODE(header: *const u32) -> u32 {
    ((*header & ASYNC_HEADER_Q0_TCODE_MASK) >> ASYNC_HEADER_Q0_TCODE_SHIFT) as u32
}
#[inline]
pub unsafe fn ASYNC_HEADER_GET_SOURCE(header: *const u32) -> u32 {
    (*header.add(1) & ASYNC_HEADER_Q1_SOURCE_MASK) >> ASYNC_HEADER_Q1_SOURCE_SHIFT
}
#[inline]
pub unsafe fn ASYNC_HEADER_GET_OFFSET(header: *const u32) -> u64 {
    (((( (*header.add(1) & ASYNC_HEADER_Q1_OFFSET_HIGH_MASK) as u64)
        >> ASYNC_HEADER_Q1_OFFSET_HIGH_SHIFT) << 32) | *header.add(2) as u64)
}
#[inline]
pub unsafe fn ASYNC_HEADER_GET_RCODE(header: *const u32) -> u32 {
    (*header.add(1) & ASYNC_HEADER_Q1_RCODE_MASK) >> ASYNC_HEADER_Q1_RCODE_SHIFT
}

#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_PHY_ID(quads: *const u32) -> u32 {
    (*quads & SELF_ID_PHY_ID_MASK) >> SELF_ID_PHY_ID_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_LINK_ACTIVE(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_LINK_ACTIVE_MASK) >> SELF_ID_ZERO_LINK_ACTIVE_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_GAP_COUNT(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_GAP_COUNT_MASK) >> SELF_ID_ZERO_GAP_COUNT_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_SCODE(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_SCODE_MASK) >> SELF_ID_ZERO_SCODE_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_CONTENDER(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_CONTENDER_MASK) >> SELF_ID_ZERO_CONTENDER_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_POWER_CLASS(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_POWER_CLASS_MASK) >> SELF_ID_ZERO_POWER_CLASS_SHIFT
}
#[inline]
pub unsafe fn PHY_PACKET_SELF_ID_GET_INITIATED_RESET(quads: *const u32) -> u32 {
    (*quads & SELF_ID_ZERO_INITIATED_RESET_MASK) >> SELF_ID_ZERO_INITIATED_RESET_SHIFT
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fw_iso_context_completions_cause {
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_FLUSH = 0,
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_INTERRUPT,
    FW_ISO_CONTEXT_COMPLETIONS_CAUSE_HEADER_OVERFLOW,
}

#[inline]
pub const fn show_cause(cause: fw_iso_context_completions_cause) -> &'static str {
    match cause {
        fw_iso_context_completions_cause::FW_ISO_CONTEXT_COMPLETIONS_CAUSE_FLUSH => "FLUSH",
        fw_iso_context_completions_cause::FW_ISO_CONTEXT_COMPLETIONS_CAUSE_INTERRUPT => "INTERRUPT",
        fw_iso_context_completions_cause::FW_ISO_CONTEXT_COMPLETIONS_CAUSE_HEADER_OVERFLOW => "HEADER_OVERFLOW",
    }
}

// Tracepoint declarations and their TP_PROTO/TP_STRUCT__entry/TP_fast_assign/
// TP_printk bodies.  These names are retained as the externally visible event
// interface; the tracepoint backend expands the corresponding declarations.
pub const FIREWIRE_TRACE_EVENTS: &[&str] = &[
    "async_request_outbound_initiate", "async_request_outbound_complete",
    "async_response_inbound", "async_request_inbound",
    "async_response_outbound_initiate", "async_response_outbound_complete",
    "async_phy_outbound_initiate", "async_phy_outbound_complete", "async_phy_inbound",
    "bus_reset_initiate", "bus_reset_schedule", "bus_reset_postpone", "bus_reset_handle",
    "self_id_sequence", "isoc_outbound_allocate", "isoc_inbound_single_allocate",
    "isoc_inbound_multiple_allocate", "isoc_outbound_destroy", "isoc_inbound_single_destroy",
    "isoc_inbound_multiple_destroy", "isoc_inbound_multiple_channels", "isoc_outbound_start",
    "isoc_inbound_single_start", "isoc_inbound_multiple_start", "isoc_outbound_stop",
    "isoc_inbound_single_stop", "isoc_inbound_multiple_stop", "isoc_outbound_flush",
    "isoc_inbound_single_flush", "isoc_inbound_multiple_flush", "isoc_outbound_flush_completions",
    "isoc_inbound_single_flush_completions", "isoc_inbound_multiple_flush_completions",
    "isoc_outbound_queue", "isoc_inbound_single_queue", "isoc_inbound_multiple_queue",
    "isoc_outbound_completions", "isoc_inbound_single_completions", "isoc_inbound_multiple_completions",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
