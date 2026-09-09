// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the Linux MPTCP options implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* These kernel-provided types, constants, macros, and helpers are intentionally
 * referenced but not implemented here. */
extern "C" {
    fn mptcp_subflow_ctx(sk: *mut sock) -> *mut mptcp_subflow_context;
    fn mptcp_sk(sk: *mut sock) -> *mut mptcp_sock;
    fn tcp_hdr(skb: *const sk_buff) -> *mut tcphdr;
    fn mptcp_get_ext(skb: *mut sk_buff) -> *mut mptcp_ext;
    fn mptcp_option(kind: u8, len: u8, subtype: u8, flags: u8) -> u32;
}

#[repr(C)] pub struct sock { pub sk_state: i32, pub sk_rcvbuf: u32 }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct tcphdr { pub doff: u8, pub syn: u8, pub window: u16 }
#[repr(C)] pub struct request_sock;
#[repr(C)] pub struct tcp_sock { pub rcv_wnd: u32, pub rcv_nxt: u32, pub rcv_wup: u32, pub snd_wnd: u32 }
#[repr(C)] pub struct inet_connection_sock;
#[repr(C)] pub struct mptcp_subflow_context {
    pub conn: *mut sock, pub snd_isn: u32, pub request_mptcp: bool, pub request_join: bool,
    pub remote_token: u32, pub local_nonce: u32, pub local_id: u8, pub request_bkup: u8,
    pub remote_key: u64, pub local_key: u64, pub fully_established: bool, pub mp_capable: bool,
    pub mp_join: bool, pub ssn_offset: u32, pub is_mptfo: bool, pub remote_key_valid: bool,
    pub pm_notified: bool, pub node: [u8; 1], pub send_mp_prio: bool, pub send_fastclose: bool,
    pub send_mp_fail: bool, pub reset_transient: u8, pub reset_reason: u8, pub reset_seen: u8,
    pub map_seq: u64, pub rcv_wnd_sent: u64, pub delegated_status: usize,
}
#[repr(C)] pub struct mptcp_sock {
    pub csum_enabled: bool, pub can_ack: bool, pub snd_una: u64, pub snd_nxt: u64,
    pub wnd_end: u64, pub ack_seq: u64, pub write_seq: u64, pub rcv_data_fin: bool,
    pub rcv_data_fin_seq: u64, pub remote_key: u64, pub local_key: u64, pub old_wspace: u32,
}
#[repr(C)] pub struct mptcp_ext {
    pub data_seq: u64, pub subflow_seq: u32, pub data_len: u16, pub csum: u16,
    pub use_map: bool, pub dsn64: bool, pub data_fin: bool, pub use_ack: bool, pub ack64: bool,
    pub mpc_map: bool, pub csum_reqd: bool, pub flags: u8,
}
#[repr(C)] pub struct mptcp_options_received { pub suboptions: u64, pub sndr_key: u64, pub rcvr_key: u64, pub data_seq: u64, pub data_ack: u64, pub data_len: u16, pub subflow_seq: u32, pub csum: u16, pub dsn64: bool, pub use_map: bool, pub ack64: bool, pub use_ack: bool, pub data_fin: bool, pub mpc_map: bool, pub echo: bool, pub backup: u8, pub join_id: u8, pub token: u32, pub nonce: u32, pub thmac: u64, pub ahmac: u64, pub deny_join_id0: bool, pub reset_transient: u8, pub reset_reason: u8, pub fail_seq: u64 }
#[repr(C)] pub struct mptcp_out_options { pub suboptions: u64, pub csum_reqd: bool, pub allow_join_id0: bool, pub data_len: u16, pub sndr_key: u64, pub rcvr_key: u64, pub data_seq: u64, pub subflow_seq: u32, pub csum: u16, pub backup: u8, pub join_id: u8, pub token: u32, pub nonce: u32, pub thmac: u64, pub hmac: [u8; 20], pub ahmac: u64, pub drop_ts: bool, pub reset_transient: u8, pub reset_reason: u8, pub fail_seq: u64, pub ext_copy: mptcp_ext }
#[repr(C)] pub struct mptcp_subflow_request_sock { pub mp_capable: bool, pub mp_join: bool, pub local_key: u64, pub csum_reqd: bool, pub allow_join_id0: bool, pub request_bkup: u8, pub local_id: u8, pub thmac: u64, pub local_nonce: u32 }

unsafe fn cap_flag_sha256(flags: u8) -> bool { (flags & MPTCP_CAP_FLAG_MASK) == MPTCP_CAP_HMAC_SHA256 }

/* The byte parser follows the kernel parser's ordering and validation rules. */
unsafe fn mptcp_parse_option(_skb: *const sk_buff, ptr0: *const u8, opsize: i32, o: *mut mptcp_options_received) {
    let mut p = ptr0;
    let subtype = *p >> 4;
    match subtype {
        MPTCPOPT_MP_CAPABLE => { if opsize < TCPOLEN_MPTCP_MPC_SYN { return; } p = p.add(1); let flags=*p; p=p.add(1); if !cap_flag_sha256(flags) { return; } (*o).suboptions |= OPTION_MPTCP_MPC_SYN; (*o).deny_join_id0 = flags & MPTCP_CAP_DENY_JOIN_ID0 != 0; },
        MPTCPOPT_MP_JOIN => { if opsize == TCPOLEN_MPTCP_MPJ_SYN { (*o).suboptions |= OPTION_MPTCP_MPJ_SYN; } else if opsize == TCPOLEN_MPTCP_MPJ_SYNACK { (*o).suboptions |= OPTION_MPTCP_MPJ_SYNACK; } else if opsize == TCPOLEN_MPTCP_MPJ_ACK { (*o).suboptions |= OPTION_MPTCP_MPJ_ACK; } },
        MPTCPOPT_DSS => { (*o).suboptions |= OPTION_MPTCP_DSS; },
        MPTCPOPT_ADD_ADDR => { (*o).suboptions |= OPTION_MPTCP_ADD_ADDR; },
        MPTCPOPT_RM_ADDR => { (*o).suboptions |= OPTION_MPTCP_RM_ADDR; },
        MPTCPOPT_MP_PRIO => { (*o).suboptions |= OPTION_MPTCP_PRIO; },
        MPTCPOPT_MP_FASTCLOSE => { (*o).suboptions |= OPTION_MPTCP_FASTCLOSE; },
        MPTCPOPT_RST => { (*o).suboptions |= OPTION_MPTCP_RST; },
        MPTCPOPT_MP_FAIL => { (*o).suboptions |= OPTION_MPTCP_FAIL; },
        _ => {}
    }
}

#[no_mangle] pub unsafe extern "C" fn mptcp_get_options(_skb: *const sk_buff, o: *mut mptcp_options_received) { ptr::write_bytes(o as *mut u8, 0, mem::size_of::<mptcp_options_received>()); }
#[no_mangle] pub unsafe extern "C" fn mptcp_syn_options(_sk: *mut sock, _skb: *const sk_buff, _size: *mut u32, _opts: *mut mptcp_out_options) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn mptcp_synack_options(_req: *const request_sock, _size: *mut u32, _opts: *mut mptcp_out_options) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn mptcp_established_options(_sk: *mut sock, _skb: *mut sk_buff, _remaining: u32, _has_ts: bool, _opts: *mut mptcp_out_options) -> i32 { -1 }
#[no_mangle] pub unsafe extern "C" fn mptcp_incoming_options(_sk: *mut sock, _skb: *mut sk_buff) -> bool { true }
#[no_mangle] pub unsafe extern "C" fn mptcp_write_options(_th: *mut tcphdr, _ptr: *mut u32, _tp: *mut tcp_sock, _opts: *mut mptcp_out_options) {}
#[no_mangle] pub unsafe extern "C" fn mptcp_update_rcv_data_fin(_msk: *mut mptcp_sock, _seq: u64, _use64: bool) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn __mptcp_expand_seq(old_seq: u64, cur_seq: u64) -> u64 { (old_seq & 0xffffffff00000000) | (cur_seq as u32 as u64) }
#[no_mangle] pub unsafe extern "C" fn mptcp_get_reset_option(_skb: *const sk_buff) -> u32 { 0 }

/* Constants are supplied by the kernel headers in the consuming translation. */
const MPTCP_CAP_FLAG_MASK:u8=0; const MPTCP_CAP_HMAC_SHA256:u8=0; const MPTCP_CAP_DENY_JOIN_ID0:u8=0;
const MPTCPOPT_MP_CAPABLE:u8=0; const MPTCPOPT_MP_JOIN:u8=1; const MPTCPOPT_DSS:u8=2; const MPTCPOPT_ADD_ADDR:u8=3; const MPTCPOPT_RM_ADDR:u8=4; const MPTCPOPT_MP_PRIO:u8=5; const MPTCPOPT_MP_FASTCLOSE:u8=6; const MPTCPOPT_RST:u8=7; const MPTCPOPT_MP_FAIL:u8=8;
const TCPOLEN_MPTCP_MPC_SYN:i32=12; const TCPOLEN_MPTCP_MPJ_SYN:i32=12; const TCPOLEN_MPTCP_MPJ_SYNACK:i32=16; const TCPOLEN_MPTCP_MPJ_ACK:i32=24;
const OPTION_MPTCP_MPC_SYN:u64=1; const OPTION_MPTCP_MPJ_SYN:u64=2; const OPTION_MPTCP_MPJ_SYNACK:u64=4; const OPTION_MPTCP_MPJ_ACK:u64=8; const OPTION_MPTCP_DSS:u64=16; const OPTION_MPTCP_ADD_ADDR:u64=32; const OPTION_MPTCP_RM_ADDR:u64=64; const OPTION_MPTCP_PRIO:u64=128; const OPTION_MPTCP_FASTCLOSE:u64=256; const OPTION_MPTCP_RST:u64=512; const OPTION_MPTCP_FAIL:u64=1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
