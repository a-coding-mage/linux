/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM: mctp
//
// The original header is a Linux tracepoint definition header.  The
// tracepoint declaration/registration machinery is supplied externally.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MctpTraceKeyReason {
    Timeout = 0,
    Replied,
    Invalidated,
    Closed,
    Dropped,
}

pub const MCTP_TRACE_KEY_TIMEOUT: i32 = MctpTraceKeyReason::Timeout as i32;
pub const MCTP_TRACE_KEY_REPLIED: i32 = MctpTraceKeyReason::Replied as i32;
pub const MCTP_TRACE_KEY_INVALIDATED: i32 = MctpTraceKeyReason::Invalidated as i32;
pub const MCTP_TRACE_KEY_CLOSED: i32 = MctpTraceKeyReason::Closed as i32;
pub const MCTP_TRACE_KEY_DROPPED: i32 = MctpTraceKeyReason::Dropped as i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MctpKeyAcquireEntry {
    pub paddr: u8,
    pub laddr: u8,
    pub tag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MctpKeyReleaseEntry {
    pub paddr: u8,
    pub laddr: u8,
    pub tag: u8,
    pub reason: i32,
}

// TRACE_EVENT(mctp_key_acquire,
//     TP_PROTO(const struct mctp_sk_key *key),
//     TP_ARGS(key),
//     TP_STRUCT__entry(
//         __field(__u8, paddr)
//         __field(__u8, laddr)
//         __field(__u8, tag)
//     ),
//     TP_fast_assign(
//         __entry->paddr = key->peer_addr;
//         __entry->laddr = key->local_addr;
//         __entry->tag = key->tag;
//     ),
//     TP_printk("local %d, peer %d, tag %1x",
//         __entry->laddr, __entry->paddr, __entry->tag)
// )

// TRACE_EVENT(mctp_key_release,
//     TP_PROTO(const struct mctp_sk_key *key, int reason),
//     TP_ARGS(key, reason),
//     TP_STRUCT__entry(
//         __field(__u8, paddr)
//         __field(__u8, laddr)
//         __field(__u8, tag)
//         __field(int, reason)
//     ),
//     TP_fast_assign(
//         __entry->paddr = key->peer_addr;
//         __entry->laddr = key->local_addr;
//         __entry->tag = key->tag;
//         __entry->reason = reason;
//     ),
//     TP_printk("local %d, peer %d, tag %1x %s",
//         __entry->laddr, __entry->paddr, __entry->tag,
//         __print_symbolic(__entry->reason,
//             { MCTP_TRACE_KEY_TIMEOUT, "timeout" },
//             { MCTP_TRACE_KEY_REPLIED, "replied" },
//             { MCTP_TRACE_KEY_INVALIDATED, "invalidated" },
//             { MCTP_TRACE_KEY_CLOSED, "closed" },
//             { MCTP_TRACE_KEY_DROPPED, "dropped" })
// )

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
