/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * amdtp-stream-trace.h - tracepoint definitions to dump a part of packet data
 *
 * Copyright (c) 2016 Takashi Sakamoto
 */

// TRACE_SYSTEM: snd_firewire_lib
// C include dependency: <linux/tracepoint.h>

pub const TRACE_SYSTEM: &str = "snd_firewire_lib";
pub const TRACE_INCLUDE_PATH: &str = ".";
pub const TRACE_INCLUDE_FILE: &str = "amdtp-stream-trace";

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type __be32 = u32;

unsafe extern "C" {
    pub static CYCLES_PER_SECOND: u32;

    pub fn fw_parent_device(unit: *const fw_unit) -> *mut fw_device;
    pub fn in_softirq() -> ::core::ffi::c_int;
    pub fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: usize,
    ) -> *mut ::core::ffi::c_void;
}

#[repr(C)]
pub struct amdtp_stream {
    pub context: *mut fw_iso_context,
    pub direction: ::core::ffi::c_int,
    pub unit: *mut fw_unit,
}

#[repr(C)]
pub struct fw_iso_context {
    pub channel: ::core::ffi::c_int,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_card {
    pub node_id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct fw_device {
    pub node_id: ::core::ffi::c_int,
    pub card: *mut fw_card,
}

unsafe extern "C" {
    pub static AMDTP_IN_STREAM: ::core::ffi::c_int;
}

#[repr(C)]
pub struct trace_event_raw_amdtp_packet {
    pub cycle_time: ::core::ffi::c_uint,
    pub second: ::core::ffi::c_uint,
    pub cycle: ::core::ffi::c_uint,
    pub channel: ::core::ffi::c_int,
    pub src: ::core::ffi::c_int,
    pub dest: ::core::ffi::c_int,
    pub cip_header: [u8; 8],
    pub cip_header_len: usize,
    pub payload_quadlets: ::core::ffi::c_uint,
    pub data_blocks: ::core::ffi::c_uint,
    pub data_block_counter: ::core::ffi::c_uint,
    pub packet_index: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_uint,
    pub index: ::core::ffi::c_uint,
}

// Translation of TRACE_EVENT(amdtp_packet, ...).
//
// Original prototype:
// const struct amdtp_stream *s, u32 cycles, const __be32 *cip_header,
// unsigned int payload_length, unsigned int data_blocks,
// unsigned int data_block_counter, unsigned int packet_index,
// unsigned int index, u32 curr_cycle_time
pub unsafe fn trace_amdtp_packet_fast_assign(
    entry: *mut trace_event_raw_amdtp_packet,
    s: *const amdtp_stream,
    cycles: u32,
    cip_header: *const __be32,
    payload_length: ::core::ffi::c_uint,
    data_blocks: ::core::ffi::c_uint,
    data_block_counter: ::core::ffi::c_uint,
    packet_index: ::core::ffi::c_uint,
    index: ::core::ffi::c_uint,
    curr_cycle_time: u32,
) {
    unsafe {
        (*entry).cycle_time = curr_cycle_time;
        (*entry).second = cycles / CYCLES_PER_SECOND;
        (*entry).cycle = cycles % CYCLES_PER_SECOND;
        (*entry).channel = (*(*s).context).channel;

        if (*s).direction == AMDTP_IN_STREAM {
            (*entry).src = (*fw_parent_device((*s).unit)).node_id;
            (*entry).dest = (*(*fw_parent_device((*s).unit)).card).node_id;
        } else {
            (*entry).src = (*(*fw_parent_device((*s).unit)).card).node_id;
            (*entry).dest = (*fw_parent_device((*s).unit)).node_id;
        }

        (*entry).cip_header_len = if cip_header.is_null() { 0 } else { 8 };
        if !cip_header.is_null() {
            memcpy(
                (*entry).cip_header.as_mut_ptr() as *mut ::core::ffi::c_void,
                cip_header as *const ::core::ffi::c_void,
                (*entry).cip_header_len,
            );
        }

        (*entry).payload_quadlets =
            payload_length / ::core::mem::size_of::<__be32>() as ::core::ffi::c_uint;
        (*entry).data_blocks = data_blocks;
        (*entry).data_block_counter = data_block_counter;
        (*entry).packet_index = packet_index;
        (*entry).irq = (in_softirq() != 0) as ::core::ffi::c_uint;
        (*entry).index = index;
    }
}

pub const AMDTP_PACKET_PRINTK_FORMAT: &str =
    "%08x %02u %04u %04x %04x %02d %03u %02u %03u %02u %01u %02u %s";

// Original TP_printk arguments, in order:
// cycle_time, second, cycle, src, dest, channel, payload_quadlets, data_blocks,
// data_block_counter, packet_index, irq, index,
// __print_array(cip_header dynamic array, cip_header dynamic array length, 1)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
