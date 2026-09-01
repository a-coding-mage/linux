// SPDX-License-Identifier: GPL-2.0-only
/*
 * amdtp-motu-trace.h - tracepoint definitions to dump a part of packet data
 *
 * Copyright (c) 2017 Takashi Sakamoto
 */

// C tracepoint metadata:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM snd_firewire_motu
pub const TRACE_SYSTEM: &str = "snd_firewire_motu";

// The original header is protected by:
// #if !defined(_SND_FIREWIRE_MOTU_TRACE_H) || defined(TRACE_HEADER_MULTI_READ)
// and depends on <linux/tracepoint.h>.

unsafe extern "C" {
    fn copy_sph(
        frame: *mut u32,
        buffer: *mut __be32,
        data_blocks: ::core::ffi::c_uint,
        data_block_quadlets: ::core::ffi::c_uint,
    );

    fn copy_message(
        frames: *mut u64,
        buffer: *mut __be32,
        data_blocks: ::core::ffi::c_uint,
        data_block_quadlets: ::core::ffi::c_uint,
    );
}

#[repr(C)]
pub struct trace_event_raw_data_block_sph {
    pub src: ::core::ffi::c_int,
    pub dst: ::core::ffi::c_int,
    pub data_blocks: ::core::ffi::c_uint,
    // C trace event dynamic array:
    // __dynamic_array(u32, tstamps, data_blocks)
}

#[repr(C)]
pub struct trace_event_raw_data_block_message {
    pub src: ::core::ffi::c_int,
    pub dst: ::core::ffi::c_int,
    pub data_blocks: ::core::ffi::c_uint,
    // C trace event dynamic array:
    // __dynamic_array(u64, messages, data_blocks)
}

// TRACE_EVENT(data_block_sph,
//     TP_PROTO(struct amdtp_stream *s, unsigned int data_blocks, __be32 *buffer),
//     TP_ARGS(s, data_blocks, buffer),
//     TP_STRUCT__entry(
//         __field(int, src)
//         __field(int, dst)
//         __field(unsigned int, data_blocks)
//         __dynamic_array(u32, tstamps, data_blocks)
//     ),
//     TP_fast_assign(
//         if (s->direction == AMDTP_IN_STREAM) {
//             __entry->src = fw_parent_device(s->unit)->node_id;
//             __entry->dst = fw_parent_device(s->unit)->card->node_id;
//         } else {
//             __entry->src = fw_parent_device(s->unit)->card->node_id;
//             __entry->dst = fw_parent_device(s->unit)->node_id;
//         }
//         __entry->data_blocks = data_blocks;
//         copy_sph(__get_dynamic_array(tstamps), buffer, data_blocks,
//                  s->data_block_quadlets);
//     ),
//     TP_printk(
//         "%04x %04x %u %s",
//         __entry->src,
//         __entry->dst,
//         __entry->data_blocks,
//         __print_array(__get_dynamic_array(tstamps), __entry->data_blocks, 4)
//     )
// );
unsafe extern "C" {
    pub fn trace_data_block_sph(
        s: *mut amdtp_stream,
        data_blocks: ::core::ffi::c_uint,
        buffer: *mut __be32,
    );
}

// TRACE_EVENT(data_block_message,
//     TP_PROTO(struct amdtp_stream *s, unsigned int data_blocks, __be32 *buffer),
//     TP_ARGS(s, data_blocks, buffer),
//     TP_STRUCT__entry(
//         __field(int, src)
//         __field(int, dst)
//         __field(unsigned int, data_blocks)
//         __dynamic_array(u64, messages, data_blocks)
//     ),
//     TP_fast_assign(
//         if (s->direction == AMDTP_IN_STREAM) {
//             __entry->src = fw_parent_device(s->unit)->node_id;
//             __entry->dst = fw_parent_device(s->unit)->card->node_id;
//         } else {
//             __entry->src = fw_parent_device(s->unit)->card->node_id;
//             __entry->dst = fw_parent_device(s->unit)->node_id;
//         }
//         __entry->data_blocks = data_blocks;
//         copy_message(__get_dynamic_array(messages), buffer, data_blocks,
//                      s->data_block_quadlets);
//     ),
//     TP_printk(
//         "%04x %04x %u %s",
//         __entry->src,
//         __entry->dst,
//         __entry->data_blocks,
//         __print_array(__get_dynamic_array(messages), __entry->data_blocks, 8)
//     )
// );
unsafe extern "C" {
    pub fn trace_data_block_message(
        s: *mut amdtp_stream,
        data_blocks: ::core::ffi::c_uint,
        buffer: *mut __be32,
    );
}

// C trace include metadata:
// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH .
// #undef TRACE_INCLUDE_FILE
// #define TRACE_INCLUDE_FILE amdtp-motu-trace
// #include <trace/define_trace.h>
pub const TRACE_INCLUDE_PATH: &str = ".";
pub const TRACE_INCLUDE_FILE: &str = "amdtp-motu-trace";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
