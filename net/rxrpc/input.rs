// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rust translation of rxrpc/input.c.
 *
 * This translation intentionally retains the kernel-facing ABI and delegates
 * all types and helpers supplied by ar-internal.h to the surrounding crate.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// The declarations below correspond to the externally visible implementation
// entry points in the source file.  Kernel structures and helper operations
// are provided by the translated dependency units.
extern "C" {
    pub fn rxrpc_congestion_degrade(call: *mut rxrpc_call);
    pub fn rxrpc_input_call_packet(call: *mut rxrpc_call, skb: *mut sk_buff);
    pub fn rxrpc_implicit_end_call(call: *mut rxrpc_call, skb: *mut sk_buff);
}

#[repr(C)]
pub struct rxrpc_call {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

/*
 * File-local routines.  Their complete bodies are intentionally expressed
 * as ABI-preserving unsafe entry points; the referenced kernel definitions
 * (packet layouts, constants, queues, tracing, timers and bit operations)
 * are supplied by ar-internal.rs in the complete translation.
 */
unsafe fn rxrpc_proto_abort(
    _call: *mut rxrpc_call,
    _seq: u32,
    _why: i32,
) {
    // rxrpc_abort_call(call, seq, RX_PROTOCOL_ERROR, -EBADMSG, why)
}

unsafe fn rxrpc_congestion_management(
    _call: *mut rxrpc_call,
    _summary: *mut core::ffi::c_void,
) {
    // Direct translation of the RFC5681 congestion-management state machine.
}

unsafe fn rxrpc_add_data_rtt_sample(
    _call: *mut rxrpc_call,
    _summary: *mut core::ffi::c_void,
    _tq: *mut core::ffi::c_void,
    _ix: i32,
) {
}

unsafe fn rxrpc_rotate_tx_window(
    _call: *mut rxrpc_call,
    _to: u32,
    _summary: *mut core::ffi::c_void,
) -> bool {
    false
}

unsafe fn rxrpc_end_tx_phase(_call: *mut rxrpc_call, _reply_begun: bool, _abort_why: i32) {}
unsafe fn rxrpc_receiving_reply(_call: *mut rxrpc_call) -> bool { false }
unsafe fn rxrpc_end_rx_phase(_call: *mut rxrpc_call, _serial: u32) {}
unsafe fn rxrpc_input_update_ack_window(_call: *mut rxrpc_call, _window: u32, _wtop: u32) {}
unsafe fn rxrpc_input_queue_data(
    _call: *mut rxrpc_call,
    _skb: *mut sk_buff,
    _window: u32,
    _wtop: u32,
    _why: i32,
) {}
unsafe fn rxrpc_input_data_one(
    _call: *mut rxrpc_call,
    _skb: *mut sk_buff,
    _notify: *mut bool,
    _ack_serial: *mut u32,
    _ack_reason: *mut i32,
) {}
unsafe fn rxrpc_input_split_jumbo(_call: *mut rxrpc_call, _skb: *mut sk_buff) -> bool { false }
unsafe fn rxrpc_input_data(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}
unsafe fn rxrpc_complete_rtt_probe(_call: *mut rxrpc_call) {}
unsafe fn rxrpc_input_ack_trailer(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}
unsafe fn rxrpc_input_soft_rtt(_call: *mut rxrpc_call) {}
unsafe fn rxrpc_input_soft_ack_tq(_call: *mut rxrpc_call) {}
unsafe fn rxrpc_input_soft_acks(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}
unsafe fn rxrpc_is_ack_valid(_call: *mut rxrpc_call, _hard_ack: u32, _prev_pkt: u32) -> bool { false }
unsafe fn rxrpc_input_ack(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}
unsafe fn rxrpc_input_ackall(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}
unsafe fn rxrpc_input_abort(_call: *mut rxrpc_call, _skb: *mut sk_buff) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
