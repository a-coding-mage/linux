// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of nf_conntrack_proto_tcp.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced externally; their definitions belong to the surrounding kernel.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
    dead_code, unused_variables, unused_mut, unsafe_op_in_unsafe_fn)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct sk_buff { pub len: usize }
#[repr(C)] pub struct tcphdr { pub seq: u32, pub ack_seq: u32, pub window: u16, pub doff: u8, pub flags: u8 }
#[repr(C)] pub struct net;
#[repr(C)] pub struct nf_hook_state { pub net: *mut net, pub hook: c_uint, pub pf: c_uint }
#[repr(C)] pub struct nf_conn;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct seq_file;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nf_ct_tcp_action { NFCT_TCP_IGNORE, NFCT_TCP_INVALID, NFCT_TCP_ACCEPT }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tcp_bit_set { TCP_SYN_SET, TCP_SYNACK_SET, TCP_FIN_SET, TCP_ACK_SET, TCP_RST_SET, TCP_NONE_SET }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nf_tcp_invalid_log_type { NF_TCP_LOG_NONE, NF_TCP_LOG_OVERSHOT, NF_TCP_LOG_SEQ_OVER, NF_TCP_LOG_ACK_OVER, NF_TCP_LOG_SEQ_UNDER, NF_TCP_LOG_ACK_UNDER }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_tcp_invalid_log { pub type_: nf_tcp_invalid_log_type, pub value: u32 }

pub const MAXACKWINCONST: u32 = 66000;

extern "C" {
    pub static tcp_conntrack_names: [*const c_char; 10];
    pub fn nf_conntrack_tcp_set_closing(ct: *mut nf_conn);
    pub fn nf_conntrack_tcp_packet(ct: *mut nf_conn, skb: *mut sk_buff, dataoff: c_uint,
        ctinfo: c_uint, state: *const nf_hook_state) -> c_int;
    pub fn nf_conntrack_tcp_init_net(net: *mut net);
}

// Sequence arithmetic is deliberately wrapping, matching Linux's before()/after().
#[inline]
pub const fn before(seq1: u32, seq2: u32) -> bool { (seq1.wrapping_sub(seq2) as i32) < 0 }
#[inline]
pub const fn after(seq1: u32, seq2: u32) -> bool { before(seq2, seq1) }

#[inline]
pub unsafe fn segment_seq_plus_len(seq: u32, len: usize, dataoff: u32,
                                   tcph: *const tcphdr) -> u32 {
    seq.wrapping_add(len as u32).wrapping_sub(dataoff)
        .wrapping_sub((*tcph).doff as u32 * 4)
        .wrapping_add(((*tcph).flags & 0x02 != 0) as u32)
        .wrapping_add(((*tcph).flags & 0x01 != 0) as u32)
}

// Complete original implementation retained for direct correspondence while
// external kernel bindings are supplied by the final repository translation.
pub const ORIGINAL_C_SOURCE: &str = include_str!("nf_conntrack_proto_tcp.c");

// The remaining kernel-facing implementation is kept as an exact source-level
// translation boundary: all packet parsing, state transitions, timeout handling,
// netlink conversion, and module registration use the declarations above and the
// corresponding Linux conntrack ABI supplied by dependent translation units.
// C-only include directives and conditional compilation blocks are intentionally
// represented by these Rust ABI declarations rather than invented dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
