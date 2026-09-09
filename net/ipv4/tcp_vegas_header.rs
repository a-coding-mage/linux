/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TCP Vegas congestion control interface
 */

/*
 * The C header guard is omitted because Rust items are not conditionally
 * redefined in the same way.
 */

/* Vegas variables */
#[repr(C)]
pub struct vegas {
    pub beg_snd_nxt: u32, /* right edge during last RTT */
    pub beg_snd_una: u32, /* left edge  during last RTT */
    pub beg_snd_cwnd: u32, /* saves the size of the cwnd */
    pub doing_vegas_now: u8, /* if true, do vegas for this RTT */
    pub cntRTT: u16, /* # of RTTs measured within last RTT */
    pub minRTT: u32, /* min of RTTs measured within last RTT (in usec) */
    pub baseRTT: u32, /* the min of all Vegas RTT measurements seen (in usec) */
}

extern "C" {
    pub fn tcp_vegas_init(sk: *mut sock);
    pub fn tcp_vegas_state(sk: *mut sock, ca_state: u8);
    pub fn tcp_vegas_pkts_acked(sk: *mut sock, sample: *const ack_sample);
    pub fn tcp_vegas_cwnd_event(sk: *mut sock, event: tcp_ca_event);
    pub fn tcp_vegas_cwnd_event_tx_start(sk: *mut sock);
    pub fn tcp_vegas_get_info(
        sk: *mut sock,
        ext: u32,
        attr: *mut i32,
        info: *mut tcp_cc_info,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
