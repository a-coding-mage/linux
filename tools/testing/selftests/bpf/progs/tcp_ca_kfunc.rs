// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Facebook */

// Dependency intent from C source:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rate_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ack_sample {
    _private: [u8; 0],
}

pub type tcp_ca_event = ::core::ffi::c_uint;

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: *mut c_void,
    pub in_ack_event: *mut c_void,
    pub cong_control: *mut c_void,
    pub cong_avoid: *mut c_void,
    pub sndbuf_expand: *mut c_void,
    pub undo_cwnd: *mut c_void,
    pub cwnd_event: *mut c_void,
    pub cwnd_event_tx_start: *mut c_void,
    pub ssthresh: *mut c_void,
    pub min_tso_segs: *mut c_void,
    pub set_state: *mut c_void,
    pub pkts_acked: *mut c_void,
    pub name: *const c_char,
}

extern "C" {
    #[link_name = "bbr_init"]
    fn bbr_init(sk: *mut sock);
    #[link_name = "bbr_main"]
    fn bbr_main(sk: *mut sock, ack: u32, flag: c_int, rs: *const rate_sample);
    #[link_name = "bbr_sndbuf_expand"]
    fn bbr_sndbuf_expand(sk: *mut sock) -> u32;
    #[link_name = "bbr_undo_cwnd"]
    fn bbr_undo_cwnd(sk: *mut sock) -> u32;
    #[link_name = "bbr_cwnd_event_tx_start"]
    fn bbr_cwnd_event_tx_start(sk: *mut sock);
    #[link_name = "bbr_ssthresh"]
    fn bbr_ssthresh(sk: *mut sock) -> u32;
    #[link_name = "bbr_min_tso_segs"]
    fn bbr_min_tso_segs(sk: *mut sock) -> u32;
    #[link_name = "bbr_set_state"]
    fn bbr_set_state(sk: *mut sock, new_state: u8);

    #[link_name = "dctcp_init"]
    fn dctcp_init(sk: *mut sock);
    #[link_name = "dctcp_update_alpha"]
    fn dctcp_update_alpha(sk: *mut sock, flags: u32);
    #[link_name = "dctcp_cwnd_event"]
    fn dctcp_cwnd_event(sk: *mut sock, ev: tcp_ca_event);
    #[link_name = "dctcp_cwnd_event_tx_start"]
    fn dctcp_cwnd_event_tx_start(sk: *mut sock);
    #[link_name = "dctcp_ssthresh"]
    fn dctcp_ssthresh(sk: *mut sock) -> u32;
    #[link_name = "dctcp_cwnd_undo"]
    fn dctcp_cwnd_undo(sk: *mut sock) -> u32;
    #[link_name = "dctcp_state"]
    fn dctcp_state(sk: *mut sock, new_state: u8);

    #[link_name = "cubictcp_init"]
    fn cubictcp_init(sk: *mut sock);
    #[link_name = "cubictcp_recalc_ssthresh"]
    fn cubictcp_recalc_ssthresh(sk: *mut sock) -> u32;
    #[link_name = "cubictcp_cong_avoid"]
    fn cubictcp_cong_avoid(sk: *mut sock, ack: u32, acked: u32);
    #[link_name = "cubictcp_state"]
    fn cubictcp_state(sk: *mut sock, new_state: u8);
    #[link_name = "cubictcp_cwnd_event_tx_start"]
    fn cubictcp_cwnd_event_tx_start(sk: *mut sock);
    #[link_name = "cubictcp_acked"]
    fn cubictcp_acked(sk: *mut sock, sample: *const ack_sample);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn init(sk: *mut sock) {
    bbr_init(sk);
    dctcp_init(sk);
    cubictcp_init(sk);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn in_ack_event(sk: *mut sock, flags: u32) {
    dctcp_update_alpha(sk, flags);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn cong_control(
    sk: *mut sock,
    ack: u32,
    flag: c_int,
    rs: *const rate_sample,
) {
    bbr_main(sk, ack, flag, rs);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    cubictcp_cong_avoid(sk, ack, acked);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn sndbuf_expand(sk: *mut sock) -> u32 {
    bbr_sndbuf_expand(sk)
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn undo_cwnd(sk: *mut sock) -> u32 {
    bbr_undo_cwnd(sk);
    dctcp_cwnd_undo(sk)
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn cwnd_event(sk: *mut sock, event: tcp_ca_event) {
    dctcp_cwnd_event(sk, event);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn cwnd_event_tx_start(sk: *mut sock) {
    bbr_cwnd_event_tx_start(sk);
    dctcp_cwnd_event_tx_start(sk);
    cubictcp_cwnd_event_tx_start(sk);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ssthresh(sk: *mut sock) -> u32 {
    bbr_ssthresh(sk);
    dctcp_ssthresh(sk);
    cubictcp_recalc_ssthresh(sk)
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn min_tso_segs(sk: *mut sock) -> u32 {
    bbr_min_tso_segs(sk)
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn set_state(sk: *mut sock, new_state: u8) {
    bbr_set_state(sk, new_state);
    dctcp_state(sk, new_state);
    cubictcp_state(sk, new_state);
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn pkts_acked(sk: *mut sock, sample: *const ack_sample) {
    cubictcp_acked(sk, sample);
}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut tcp_ca_kfunc: tcp_congestion_ops = tcp_congestion_ops {
    init: init as *mut c_void,
    in_ack_event: in_ack_event as *mut c_void,
    cong_control: cong_control as *mut c_void,
    cong_avoid: cong_avoid as *mut c_void,
    sndbuf_expand: sndbuf_expand as *mut c_void,
    undo_cwnd: undo_cwnd as *mut c_void,
    cwnd_event: cwnd_event as *mut c_void,
    cwnd_event_tx_start: cwnd_event_tx_start as *mut c_void,
    ssthresh: ssthresh as *mut c_void,
    min_tso_segs: min_tso_segs as *mut c_void,
    set_state: set_state as *mut c_void,
    pkts_acked: pkts_acked as *mut c_void,
    name: b"tcp_ca_kfunc\0".as_ptr() as *const c_char,
};

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [
    b'G' as c_char,
    b'P' as c_char,
    b'L' as c_char,
    0 as c_char,
];
