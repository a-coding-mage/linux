// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Takashi Sakamoto

// TRACE_SYSTEM: firewire_ohci
// The C tracepoint definitions below are represented as Rust declarations.
// Some macros and helper functions are defined in drivers/firewire/ohci.c.

pub const QUADLET_SIZE: usize = 4;

#[inline]
pub const unsafe fn self_id_count_is_error(reg: u32) -> bool {
    (((reg & OHCI1394_SelfIDCount_selfIDError_MASK)
        >> OHCI1394_SelfIDCount_selfIDError_SHIFT) != 0)
}

#[inline]
pub const unsafe fn self_id_count_get_generation(reg: u32) -> u32 {
    (reg & OHCI1394_SelfIDCount_selfIDGeneration_MASK)
        >> OHCI1394_SelfIDCount_selfIDGeneration_SHIFT
}

#[inline]
pub const unsafe fn self_id_receive_q0_get_generation(quadlet: u32) -> u32 {
    (quadlet & OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_MASK)
        >> OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_SHIFT
}

#[inline]
pub const unsafe fn self_id_receive_q0_get_timestamp(quadlet: u32) -> u32 {
    (quadlet & OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_MASK)
        >> OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_SHIFT
}

// C trace event: irqs
// TP_PROTO(unsigned int card_index, u32 events)
#[repr(C)]
pub struct IrqsEntry {
    pub card_index: u8,
    pub events: u32,
}

// C trace event: self_id_complete
// TP_PROTO(unsigned int card_index, u32 reg, const __le32 *self_id_receive,
//          bool has_be_header_quirk)
#[repr(C)]
pub struct SelfIdCompleteEntry {
    pub card_index: u8,
    pub reg: u32,
    pub self_id_receive: *mut u32,
    pub self_id_receive_len: usize,
}

/// Equivalent to the C `self_id_complete` dynamic-array assignment.
#[inline]
pub unsafe fn self_id_complete_assign(
    entry: *mut SelfIdCompleteEntry,
    card_index: u32,
    reg: u32,
    self_id_receive: *const u32,
    self_id_receive_len: usize,
    has_be_header_quirk: bool,
    cond_le32_to_cpu: unsafe fn(u32, bool) -> u32,
) {
    (*entry).card_index = card_index as u8;
    (*entry).reg = reg;
    (*entry).self_id_receive_len = self_id_receive_len;
    for i in 0..(self_id_receive_len / QUADLET_SIZE) {
        *(*entry).self_id_receive.add(i) =
            cond_le32_to_cpu(*self_id_receive.add(i), has_be_header_quirk);
    }
}

// External symbols supplied by the corresponding OHCI implementation and
// tracepoint infrastructure.
extern "C" {
    static OHCI1394_SelfIDCount_selfIDError_MASK: u32;
    static OHCI1394_SelfIDCount_selfIDError_SHIFT: u32;
    static OHCI1394_SelfIDCount_selfIDGeneration_MASK: u32;
    static OHCI1394_SelfIDCount_selfIDGeneration_SHIFT: u32;
    static OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_MASK: u32;
    static OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_SHIFT: u32;
    static OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_MASK: u32;
    static OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_SHIFT: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
