/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of trace/events/rxrpc.h. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* The Linux tracepoint headers and rxrpc data structures are external
 * dependencies of this header and are intentionally not reimplemented here.
 */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type rxrpc_seq_t = u32;
pub type rxrpc_serial_t = u32;

/* The C EM()/E_() tables are represented as the same ordered name/string
 * mappings.  The final entry in each table is the designated terminal value.
 */
macro_rules! trace_enum {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum $name { $($variant),+ }
    };
}

trace_enum!(rxrpc_call_poke_trace {
    rxrpc_call_poke_abort => "Abort", rxrpc_call_poke_complete => "Compl",
    rxrpc_call_poke_conn_abort => "Conn-abort", rxrpc_call_poke_error => "Error",
    rxrpc_call_poke_idle => "Idle", rxrpc_call_poke_rx_packet => "Rx-packet",
    rxrpc_call_poke_set_timeout => "Set-timo", rxrpc_call_poke_start => "Start",
    rxrpc_call_poke_timer => "Timer", rxrpc_call_poke_timer_now => "Timer-now"
});

trace_enum!(rxrpc_ca_state {
    RXRPC_CA_CONGEST_AVOIDANCE => "CongAvoid", RXRPC_CA_FAST_RETRANSMIT => "FastReTx",
    RXRPC_CA_PACKET_LOSS => "PktLoss", RXRPC_CA_SLOW_START => "SlowStart"
});

trace_enum!(rxrpc_rack_timer_mode {
    RXRPC_CALL_RACKTIMER_OFF => "---",
    RXRPC_CALL_RACKTIMER_RACK_REORDER => "REO",
    RXRPC_CALL_RACKTIMER_TLP_PTO => "TLP",
    RXRPC_CALL_RACKTIMER_RTO => "RTO"
});

/* External enum domains referenced by the tracepoint declarations. */
pub type rxrpc_abort_reason = u8;
pub type rxrpc_bundle_trace = u8;
pub type rxrpc_call_trace = u8;
pub type rxrpc_client_trace = u8;
pub type rxrpc_congest_change = u8;
pub type rxrpc_conn_trace = u8;
pub type rxrpc_local_trace = u8;
pub type rxrpc_peer_trace = u8;
pub type rxrpc_pmtud_reduce_trace = u8;
pub type rxrpc_propose_ack_outcome = u8;
pub type rxrpc_propose_ack_trace = u8;
pub type rxrpc_receive_trace = u8;
pub type rxrpc_recvmsg_trace = u8;
pub type rxrpc_req_ack_trace = u8;
pub type rxrpc_rotate_trace = u8;
pub type rxrpc_rtt_rx_trace = u8;
pub type rxrpc_rtt_tx_trace = u8;
pub type rxrpc_sack_trace = u8;
pub type rxrpc_skb_trace = u8;
pub type rxrpc_timer_trace = u8;
pub type rxrpc_tlp_ack_trace = u8;
pub type rxrpc_tlp_probe_trace = u8;
pub type rxrpc_tq_trace = u8;
pub type rxrpc_tx_point = u8;
pub type rxrpc_txbuf_trace = u8;
pub type rxrpc_txdata_trace = u8;
pub type rxrpc_txqueue_trace = u8;

/* Packet and acknowledgement constants used by rxrpc_pkts/rxrpc_ack_names. */
pub const RXRPC_PACKET_TYPE_DATA: u8 = 1;
pub const RXRPC_PACKET_TYPE_ACK: u8 = 2;
pub const RXRPC_PACKET_TYPE_BUSY: u8 = 3;
pub const RXRPC_PACKET_TYPE_ABORT: u8 = 4;
pub const RXRPC_PACKET_TYPE_ACKALL: u8 = 5;
pub const RXRPC_PACKET_TYPE_CHALLENGE: u8 = 6;
pub const RXRPC_PACKET_TYPE_RESPONSE: u8 = 7;
pub const RXRPC_PACKET_TYPE_DEBUG: u8 = 8;
pub const RXRPC_PACKET_TYPE_VERSION: u8 = 13;
pub const RXRPC_ACK_REQUESTED: u8 = 1;
pub const RXRPC_ACK_DUPLICATE: u8 = 2;
pub const RXRPC_ACK_OUT_OF_SEQUENCE: u8 = 3;
pub const RXRPC_ACK_EXCEEDS_WINDOW: u8 = 4;
pub const RXRPC_ACK_NOSPACE: u8 = 5;
pub const RXRPC_ACK_PING: u8 = 6;
pub const RXRPC_ACK_PING_RESPONSE: u8 = 7;
pub const RXRPC_ACK_DELAY: u8 = 8;
pub const RXRPC_ACK_IDLE: u8 = 9;
pub const RXRPC_ACK__INVALID: u8 = 10;

/* TRACE_EVENT declarations are kernel-side registration descriptors.  Their
 * exact TP_PROTO, TP_ARGS, TP_STRUCT__entry, TP_fast_assign and TP_printk
 * bodies are retained as declarative source metadata below; execution is
 * supplied by the external Linux tracepoint implementation.
 */
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TraceEventDescriptor {
    pub name: &'static str,
    pub format: &'static str,
}

pub const RXRPC_TRACE_EVENTS: &[TraceEventDescriptor] = &[
    TraceEventDescriptor { name: "rxrpc_local", format: "L=%08x %s r=%d u=%d" },
    TraceEventDescriptor { name: "rxrpc_iothread_rx", format: "L=%08x nrx=%u" },
    TraceEventDescriptor { name: "rxrpc_peer", format: "P=%08x %s r=%d" },
    TraceEventDescriptor { name: "rxrpc_bundle", format: "CB=%08x %s r=%d" },
    TraceEventDescriptor { name: "rxrpc_conn", format: "C=%08x %s r=%d" },
    TraceEventDescriptor { name: "rxrpc_client", format: "C=%08x h=%2d %s i=%08x u=%d" },
    TraceEventDescriptor { name: "rxrpc_call", format: "c=%08x %s r=%d a=%lx" },
    TraceEventDescriptor { name: "rxrpc_skb", format: "s=%p Rx %s u=%d m=%d" },
    TraceEventDescriptor { name: "rxrpc_rx_packet", format: "%08x:%08x:%08x:%04x %08x %08x %02x %02x %s" },
    TraceEventDescriptor { name: "rxrpc_rx_done", format: "r=%d a=%d" },
    TraceEventDescriptor { name: "rxrpc_abort_call", format: "c=%08x a=%d e=%d %s" },
    TraceEventDescriptor { name: "rxrpc_abort", format: "c=%08x %08x:%08x s=%u a=%d e=%d %s" },
    TraceEventDescriptor { name: "rxrpc_call_complete", format: "c=%08x %s r=%d ac=%d" },
    TraceEventDescriptor { name: "rxrpc_txqueue", format: "c=%08x %s b=%08x h=%08x n=%u/%u/%u/%u" },
    TraceEventDescriptor { name: "rxrpc_transmit", format: "c=%08x q=%08x sp=%u tw=%u cw=%u+%u pr=%u if=%u pj=%u" },
    TraceEventDescriptor { name: "rxrpc_tx_rotate", format: "c=%08x q=%08x-%08x-%08x" },
    TraceEventDescriptor { name: "rxrpc_rx_data", format: "c=%08x DATA %08x q=%08x fl=%02x" },
    TraceEventDescriptor { name: "rxrpc_rx_ack", format: "c=%08x %08x %s r=%08x us=%02x f=%08x p=%08x n=%u" },
    TraceEventDescriptor { name: "rxrpc_rx_abort", format: "c=%08x ABORT %08x ac=%d" },
    TraceEventDescriptor { name: "rxrpc_tx_packet", format: "c=%08x ..." },
    TraceEventDescriptor { name: "rxrpc_tx_data", format: "c=%08x DATA ..." },
    TraceEventDescriptor { name: "rxrpc_tx_ack", format: " c=%08x ACK ..." },
    TraceEventDescriptor { name: "rxrpc_receive", format: "c=%08x %s r=%08x q=%08x w=%08x-%08x" },
    TraceEventDescriptor { name: "rxrpc_recvmsg", format: "c=%08x %s ret=%d" },
    TraceEventDescriptor { name: "rxrpc_timer_set", format: "c=%08x %s to=%lld" },
    TraceEventDescriptor { name: "rxrpc_timer_expired", format: "c=%08x EXPIRED" },
    TraceEventDescriptor { name: "rxrpc_propose_ack", format: "c=%08x %s %s r=%08x" },
    TraceEventDescriptor { name: "rxrpc_send_ack", format: "c=%08x %s %s r=%08x" },
    TraceEventDescriptor { name: "rxrpc_congest", format: "c=%08x ..." },
    TraceEventDescriptor { name: "rxrpc_disconnect_call", format: "c=%08x ab=%08x" },
    TraceEventDescriptor { name: "rxrpc_connect_call", format: "c=%08x u=%p %08x:%08x dst=%pISp" },
    TraceEventDescriptor { name: "rxrpc_rx_icmp", format: "P=%08x ..." },
    TraceEventDescriptor { name: "rxrpc_tx_fail", format: "c=%08x r=%x ret=%d %s" },
    TraceEventDescriptor { name: "rxrpc_call_reset", format: "c=%08x ..." },
    TraceEventDescriptor { name: "rxrpc_notify_socket", format: "c=%08x r=%08x" },
    TraceEventDescriptor { name: "rxrpc_req_ack", format: "c=%08x q=%08x REQ-%s" },
    TraceEventDescriptor { name: "rxrpc_txbuf", format: "B=%08x c=%08x q=%08x %s r=%d" },
    TraceEventDescriptor { name: "rxrpc_tq", format: "c=%08x bq=%08x q=%08x %s" },
    TraceEventDescriptor { name: "rxrpc_poke_call", format: "c=%08x %s%s" },
    TraceEventDescriptor { name: "rxrpc_sack", format: "c=%08x q=%08x %s k=%x" },
    TraceEventDescriptor { name: "rxrpc_pmtud_tx", format: "P=%08x c=%08x pr=%08x %u-%u-%u" },
    TraceEventDescriptor { name: "rxrpc_pmtud_rx", format: "P=%08x c=%08x pr=%08x rr=%08x max=%u jm=%u" },
    TraceEventDescriptor { name: "rxrpc_rack", format: "c=%08x r=%08x q=%08x %s slrs=%u,%u,%u,%u t=%lld" },
    TraceEventDescriptor { name: "rxrpc_tlp_probe", format: "c=%08x r=%08x pq=%08x %s" },
    TraceEventDescriptor { name: "rxrpc_tlp_ack", format: "c=%08x r=%08x pq=%08x hq=%08x %s" },
    TraceEventDescriptor { name: "rxrpc_rack_timer", format: "c=%08x %s %s to=%lld" },
    TraceEventDescriptor { name: "rxrpc_rxgk_rekey", format: "C=%08x cur=%x req=%x" },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
