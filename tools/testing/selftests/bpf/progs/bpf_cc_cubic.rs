// SPDX-License-Identifier: GPL-2.0-only

/* Highlights:
 * 1. The major difference between this bpf program and tcp_cubic.c
 *    is that this bpf program relies on `cong_control` rather than
 *    `cong_avoid` in the struct tcp_congestion_ops.
 * 2. Logic such as tcp_cwnd_reduction, tcp_cong_avoid, and
 *    tcp_update_pacing_rate is bypassed when `cong_control` is
 *    defined, so moving these logic to `cong_control`.
 * 3. WARNING: This bpf program is NOT the same as tcp_cubic.c.
 *    The main purpose is to show use cases of the arguments in
 *    `cong_control`. For simplicity's sake, it reuses tcp cubic's
 *    kernel functions.
 */

// Dependencies in the original C source:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

const USEC_PER_SEC: u64 = 1000000;
const TCP_PACING_SS_RATIO: u64 = 200;
const TCP_PACING_CA_RATIO: u64 = 120;
const TCP_REORDERING: u32 = 12;

extern "C" {
    fn cubictcp_init(sk: *mut sock);
    fn cubictcp_cwnd_event_tx_start(sk: *mut sock);
    fn cubictcp_recalc_ssthresh(sk: *mut sock) -> u32;
    fn cubictcp_state(sk: *mut sock, new_state: u8);
    fn tcp_reno_undo_cwnd(sk: *mut sock) -> u32;
    fn cubictcp_acked(sk: *mut sock, sample: *const ack_sample);
    fn cubictcp_cong_avoid(sk: *mut sock, ack: u32, acked: u32);

    fn tcp_sk(sk: *const sock) -> *mut tcp_sock;
    fn inet_csk(sk: *const sock) -> *mut inet_connection_sock;
    fn before(seq1: u32, seq2: u32) -> bool;

    static tcp_jiffies32: u32;
}

#[repr(C)]
pub struct sock {
    pub sk_pacing_rate: u64,
    pub sk_max_pacing_rate: u64,
}

#[repr(C)]
pub struct tcp_sock {
    pub mss_cache: u32,
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub packets_out: u32,
    pub srtt_us: u32,
    pub sacked_out: u32,
    pub lost_out: u32,
    pub retrans_out: u32,
    pub prior_cwnd: u32,
    pub prr_delivered: u32,
    pub prr_out: u32,
    pub reordering: u32,
    pub snd_una: u32,
    pub high_seq: u32,
    pub snd_cwnd_stamp: u32,
}

#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_ca_state: u8,
}

#[repr(C)]
pub struct ack_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rate_sample {
    pub acked_sacked: i32,
    pub losses: i32,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: *mut core::ffi::c_void,
    pub ssthresh: *mut core::ffi::c_void,
    pub cong_control: *mut core::ffi::c_void,
    pub set_state: *mut core::ffi::c_void,
    pub undo_cwnd: *mut core::ffi::c_void,
    pub cwnd_event_tx_start: *mut core::ffi::c_void,
    pub pkts_acked: *mut core::ffi::c_void,
    pub name: *const u8,
}

extern "C" {
    static TCP_CA_CWR: i32;
    static TCP_CA_Recovery: i32;
    static FLAG_SND_UNA_ADVANCED: i32;
    static FLAG_FORWARD_PROGRESS: i32;
    static FLAG_DATA_ACKED: i32;
    static TCP_INFINITE_SSTHRESH: u32;
}

fn div64_u64(dividend: u64, divisor: u64) -> u64 {
    dividend / divisor
}

unsafe fn tcp_update_pacing_rate(sk: *mut sock) {
    let tp: *const tcp_sock = tcp_sk(sk);
    let mut rate: u64;

    /* set sk_pacing_rate to 200 % of current rate (mss * cwnd / srtt) */
    rate = (*tp).mss_cache as u64 * ((USEC_PER_SEC / 100) << 3);

    /* current rate is (cwnd * mss) / srtt
     * In Slow Start [1], set sk_pacing_rate to 200 % the current rate.
     * In Congestion Avoidance phase, set it to 120 % the current rate.
     *
     * [1] : Normal Slow Start condition is (tp->snd_cwnd < tp->snd_ssthresh)
     *	 If snd_cwnd >= (tp->snd_ssthresh / 2), we are approaching
     *	 end of slow start and should slow down.
     */
    if (*tp).snd_cwnd < (*tp).snd_ssthresh / 2 {
        rate = rate.wrapping_mul(TCP_PACING_SS_RATIO);
    } else {
        rate = rate.wrapping_mul(TCP_PACING_CA_RATIO);
    }

    rate = rate.wrapping_mul(core::cmp::max((*tp).snd_cwnd, (*tp).packets_out) as u64);

    if (*tp).srtt_us != 0 {
        rate = div64_u64(rate, (*tp).srtt_us as u64);
    }

    (*sk).sk_pacing_rate = core::cmp::min(rate, (*sk).sk_max_pacing_rate);
}

unsafe fn tcp_cwnd_reduction(
    sk: *mut sock,
    newly_acked_sacked: i32,
    newly_lost: i32,
    flag: i32,
) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let mut sndcnt: i32 = 0;
    let pkts_in_flight: u32 = (*tp)
        .packets_out
        .wrapping_sub((*tp).sacked_out.wrapping_add((*tp).lost_out))
        .wrapping_add((*tp).retrans_out);
    let delta: i32 = (*tp).snd_ssthresh as i32 - pkts_in_flight as i32;

    if newly_acked_sacked <= 0 || (*tp).prior_cwnd == 0 {
        return;
    }

    let prr_delivered: u32 = (*tp).prr_delivered.wrapping_add(newly_acked_sacked as u32);

    if delta < 0 {
        let dividend: u64 = ((*tp).snd_ssthresh as u64)
            .wrapping_mul(prr_delivered as u64)
            .wrapping_add((*tp).prior_cwnd as u64)
            .wrapping_sub(1);
        sndcnt = div64_u64(dividend, (*tp).prior_cwnd as u64) as u32 as i32 - (*tp).prr_out as i32;
    } else {
        sndcnt = core::cmp::max(
            prr_delivered.wrapping_sub((*tp).prr_out) as i32,
            newly_acked_sacked,
        );
        if (flag & FLAG_SND_UNA_ADVANCED) != 0 && newly_lost == 0 {
            sndcnt += 1;
        }
        sndcnt = core::cmp::min(delta, sndcnt);
    }
    /* Force a fast retransmit upon entering fast recovery */
    sndcnt = core::cmp::max(sndcnt, if (*tp).prr_out != 0 { 0 } else { 1 });
    (*tp).snd_cwnd = pkts_in_flight.wrapping_add(sndcnt as u32);
}

/* Decide whether to run the increase function of congestion control. */
unsafe fn tcp_may_raise_cwnd(sk: *const sock, flag: i32) -> bool {
    if (*tcp_sk(sk)).reordering > TCP_REORDERING {
        return (flag & FLAG_FORWARD_PROGRESS) != 0;
    }

    (flag & FLAG_DATA_ACKED) != 0
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_init(sk: *mut sock) {
    cubictcp_init(sk);
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_cwnd_event_tx_start(sk: *mut sock) {
    cubictcp_cwnd_event_tx_start(sk);
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_cong_control(
    sk: *mut sock,
    ack: u32,
    flag: i32,
    rs: *const rate_sample,
) {
    let tp: *mut tcp_sock = tcp_sk(sk);

    if (((1 << TCP_CA_CWR) | (1 << TCP_CA_Recovery)) & (1 << (*inet_csk(sk)).icsk_ca_state)) != 0 {
        /* Reduce cwnd if state mandates */
        tcp_cwnd_reduction(sk, (*rs).acked_sacked, (*rs).losses, flag);

        if !before((*tp).snd_una, (*tp).high_seq) {
            /* Reset cwnd to ssthresh in CWR or Recovery (unless it's undone) */
            if (*tp).snd_ssthresh < TCP_INFINITE_SSTHRESH
                && (*inet_csk(sk)).icsk_ca_state == TCP_CA_CWR as u8
            {
                (*tp).snd_cwnd = (*tp).snd_ssthresh;
                (*tp).snd_cwnd_stamp = tcp_jiffies32;
            }
        }
    } else if tcp_may_raise_cwnd(sk, flag) {
        /* Advance cwnd if state allows */
        cubictcp_cong_avoid(sk, ack, (*rs).acked_sacked as u32);
        (*tp).snd_cwnd_stamp = tcp_jiffies32;
    }

    tcp_update_pacing_rate(sk);
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_recalc_ssthresh(sk: *mut sock) -> u32 {
    cubictcp_recalc_ssthresh(sk)
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_state(sk: *mut sock, new_state: u8) {
    cubictcp_state(sk, new_state);
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_acked(sk: *mut sock, sample: *const ack_sample) {
    cubictcp_acked(sk, sample);
}

// Original section attribute: SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_undo_cwnd(sk: *mut sock) -> u32 {
    tcp_reno_undo_cwnd(sk)
}

// Original section attribute: SEC(".struct_ops")
#[no_mangle]
pub static mut cc_cubic: tcp_congestion_ops = tcp_congestion_ops {
    init: bpf_cubic_init as *mut core::ffi::c_void,
    ssthresh: bpf_cubic_recalc_ssthresh as *mut core::ffi::c_void,
    cong_control: bpf_cubic_cong_control as *mut core::ffi::c_void,
    set_state: bpf_cubic_state as *mut core::ffi::c_void,
    undo_cwnd: bpf_cubic_undo_cwnd as *mut core::ffi::c_void,
    cwnd_event_tx_start: bpf_cubic_cwnd_event_tx_start as *mut core::ffi::c_void,
    pkts_acked: bpf_cubic_acked as *mut core::ffi::c_void,
    name: b"bpf_cc_cubic\0".as_ptr(),
};

// Original section attribute: SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
