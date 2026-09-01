// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2020 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

// C trace header intent:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM intel_catpt
//
// Header guard:
// __SND_SOC_INTEL_CATPT_TRACE_H, with TRACE_HEADER_MULTI_READ support.
//
// Dependencies from the original header:
// <linux/types.h>
// <linux/tracepoint.h>

use core::ffi::c_void;

// Linux-compatible local type aliases for declarations originally using
// linux/types.h names.
pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type size_t = usize;

// DECLARE_EVENT_CLASS(catpt_ipc_msg,
//     TP_PROTO(u32 header),
//     TP_ARGS(header),
//     TP_STRUCT__entry(
//         __field(u32, header)
//     ),
//     TP_fast_assign(
//         __entry->header = header;
//     ),
//     TP_printk("0x%08x", __entry->header)
// );
//
// The DEFINE_EVENT users below share this event class. In Rust, the generated
// tracepoint call sites are represented as external declarations.
unsafe extern "C" {
    // DEFINE_EVENT(catpt_ipc_msg, catpt_irq,
    //     TP_PROTO(u32 header),
    //     TP_ARGS(header)
    // );
    pub fn trace_catpt_irq(header: u32);

    // DEFINE_EVENT(catpt_ipc_msg, catpt_ipc_request,
    //     TP_PROTO(u32 header),
    //     TP_ARGS(header)
    // );
    pub fn trace_catpt_ipc_request(header: u32);

    // DEFINE_EVENT(catpt_ipc_msg, catpt_ipc_reply,
    //     TP_PROTO(u32 header),
    //     TP_ARGS(header)
    // );
    pub fn trace_catpt_ipc_reply(header: u32);

    // DEFINE_EVENT(catpt_ipc_msg, catpt_ipc_notify,
    //     TP_PROTO(u32 header),
    //     TP_ARGS(header)
    // );
    pub fn trace_catpt_ipc_notify(header: u32);

    // TRACE_EVENT_CONDITION(catpt_ipc_payload_chunk,
    //     TP_PROTO(const u8 *data, size_t size, size_t offset, size_t total),
    //     TP_ARGS(data, size, offset, total),
    //     TP_CONDITION(data && size),
    //     TP_STRUCT__entry(
    //         __dynamic_array(u8, buf, size)
    //         __field(size_t, offset)
    //         __field(size_t, pos)
    //         __field(size_t, total)
    //     ),
    //     TP_fast_assign(
    //         memcpy(__get_dynamic_array(buf), data + offset, size);
    //         __entry->offset = offset;
    //         __entry->pos = offset + size;
    //         __entry->total = total;
    //     ),
    //     TP_printk("range %zu-%zu out of %zu bytes%s",
    //               __entry->offset, __entry->pos, __entry->total,
    //               __print_hex_dump("", DUMP_PREFIX_NONE, 16, 4,
    //                                __get_dynamic_array(buf),
    //                                __get_dynamic_array_len(buf), false))
    // );
    pub fn trace_catpt_ipc_payload_chunk(
        data: *const u8,
        size: size_t,
        offset: size_t,
        total: size_t,
    );

    pub fn trace_catpt_ipc_payload(data: *const c_void, size: size_t);
}

// This part must be outside protection.
// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH .
// #define TRACE_INCLUDE_FILE trace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
