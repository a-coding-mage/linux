// SPDX-License-Identifier: GPL-2.0

// Dependency intent from C source:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
}

type __u32 = u32;

const USEC_PER_SEC: u64 = 1000000;

extern "C" {
    static SK_PACING_NONE: i32;
    static SK_PACING_NEEDED: i32;
}

#[repr(C)]
pub struct sock {
    pub sk_pacing_status: i32,
    pub sk_pacing_rate: u64,
    pub sk_max_pacing_rate: u64,
}

#[repr(C)]
pub struct tcp_sock {
    pub sacked_out: u32,
    pub lost_out: u32,
    pub packets_out: u32,
    pub retrans_out: u32,
    pub snd_cwnd: u32,
    pub mss_cache: u32,
    pub srtt_us: u32,
    pub app_limited: u32,
    pub delivered: u32,
    pub snd_ssthresh: u32,
}

#[repr(C)]
pub struct rate_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: Option<unsafe extern "C" fn(*mut sock)>,
    pub cong_control: Option<unsafe extern "C" fn(*mut sock, *const rate_sample)>,
    pub ssthresh: Option<unsafe extern "C" fn(*mut sock) -> __u32>,
    pub undo_cwnd: Option<unsafe extern "C" fn(*mut sock) -> __u32>,
    pub name: [u8; 16],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe fn tcp_left_out(tp: *const tcp_sock) -> u32 {
    (*tp).sacked_out.wrapping_add((*tp).lost_out)
}

unsafe fn tcp_packets_in_flight(tp: *const tcp_sock) -> u32 {
    (*tp)
        .packets_out
        .wrapping_sub(tcp_left_out(tp))
        .wrapping_add((*tp).retrans_out)
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn write_sk_pacing_init(sk: *mut sock) {
    // Original C condition:
    // #ifdef ENABLE_ATOMICS_TESTS
    //     __sync_bool_compare_and_swap(&sk->sk_pacing_status, SK_PACING_NONE,
    //                                  SK_PACING_NEEDED);
    // #else
    (*sk).sk_pacing_status = SK_PACING_NEEDED;
    // #endif
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn write_sk_pacing_cong_control(
    sk: *mut sock,
    _rs: *const rate_sample,
) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let srtt_us: u32 = if (*tp).srtt_us != 0 {
        (*tp).srtt_us
    } else {
        1u32 << 3
    };
    let rate: u64 = ((((*tp).snd_cwnd as u64)
        .wrapping_mul((*tp).mss_cache as u64)
        .wrapping_mul(USEC_PER_SEC))
        << 3)
        / (srtt_us as u64);
    (*sk).sk_pacing_rate = if rate < (*sk).sk_max_pacing_rate {
        rate
    } else {
        (*sk).sk_max_pacing_rate
    };

    let app_limited: u32 = (*tp)
        .delivered
        .wrapping_add(tcp_packets_in_flight(tp));
    (*tp).app_limited = if app_limited != 0 { app_limited } else { 1 };
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn write_sk_pacing_ssthresh(sk: *mut sock) -> __u32 {
    (*tcp_sk(sk)).snd_ssthresh
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn write_sk_pacing_undo_cwnd(sk: *mut sock) -> __u32 {
    (*tcp_sk(sk)).snd_cwnd
}

// SEC(".struct_ops")
#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut write_sk_pacing: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(write_sk_pacing_init),
    cong_control: Some(write_sk_pacing_cong_control),
    ssthresh: Some(write_sk_pacing_ssthresh),
    undo_cwnd: Some(write_sk_pacing_undo_cwnd),
    name: *b"bpf_w_sk_pacing\0",
};
