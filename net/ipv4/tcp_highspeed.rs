// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sally Floyd's High Speed TCP (RFC 3649) congestion control
 *
 * See https://www.icir.org/floyd/hstcp.html
 *
 * John Heffner <jheffner@psc.edu>
 */

/* Dependencies supplied by the Linux kernel translation environment. */

/* From AIMD tables from RFC 3649 appendix B,
 * with fixed-point MD scaled <<8.
 */
#[repr(C)]
struct HstcpAimdVal {
    cwnd: u32,
    md: u32,
}

static HSTCP_AIMD_VALS: &[HstcpAimdVal] = &[
    HstcpAimdVal { cwnd: 38, md: 128 },
    HstcpAimdVal { cwnd: 118, md: 112 },
    HstcpAimdVal { cwnd: 221, md: 104 },
    HstcpAimdVal { cwnd: 347, md: 98 },
    HstcpAimdVal { cwnd: 495, md: 93 },
    HstcpAimdVal { cwnd: 663, md: 89 },
    HstcpAimdVal { cwnd: 851, md: 86 },
    HstcpAimdVal { cwnd: 1058, md: 83 },
    HstcpAimdVal { cwnd: 1284, md: 81 },
    HstcpAimdVal { cwnd: 1529, md: 78 },
    HstcpAimdVal { cwnd: 1793, md: 76 },
    HstcpAimdVal { cwnd: 2076, md: 74 },
    HstcpAimdVal { cwnd: 2378, md: 72 },
    HstcpAimdVal { cwnd: 2699, md: 71 },
    HstcpAimdVal { cwnd: 3039, md: 69 },
    HstcpAimdVal { cwnd: 3399, md: 68 },
    HstcpAimdVal { cwnd: 3778, md: 66 },
    HstcpAimdVal { cwnd: 4177, md: 65 },
    HstcpAimdVal { cwnd: 4596, md: 64 },
    HstcpAimdVal { cwnd: 5036, md: 62 },
    HstcpAimdVal { cwnd: 5497, md: 61 },
    HstcpAimdVal { cwnd: 5979, md: 60 },
    HstcpAimdVal { cwnd: 6483, md: 59 },
    HstcpAimdVal { cwnd: 7009, md: 58 },
    HstcpAimdVal { cwnd: 7558, md: 57 },
    HstcpAimdVal { cwnd: 8130, md: 56 },
    HstcpAimdVal { cwnd: 8726, md: 55 },
    HstcpAimdVal { cwnd: 9346, md: 54 },
    HstcpAimdVal { cwnd: 9991, md: 53 },
    HstcpAimdVal { cwnd: 10661, md: 52 },
    HstcpAimdVal { cwnd: 11358, md: 52 },
    HstcpAimdVal { cwnd: 12082, md: 51 },
    HstcpAimdVal { cwnd: 12834, md: 50 },
    HstcpAimdVal { cwnd: 13614, md: 49 },
    HstcpAimdVal { cwnd: 14424, md: 48 },
    HstcpAimdVal { cwnd: 15265, md: 48 },
    HstcpAimdVal { cwnd: 16137, md: 47 },
    HstcpAimdVal { cwnd: 17042, md: 46 },
    HstcpAimdVal { cwnd: 17981, md: 45 },
    HstcpAimdVal { cwnd: 18955, md: 45 },
    HstcpAimdVal { cwnd: 19965, md: 44 },
    HstcpAimdVal { cwnd: 21013, md: 43 },
    HstcpAimdVal { cwnd: 22101, md: 43 },
    HstcpAimdVal { cwnd: 23230, md: 42 },
    HstcpAimdVal { cwnd: 24402, md: 41 },
    HstcpAimdVal { cwnd: 25618, md: 41 },
    HstcpAimdVal { cwnd: 26881, md: 40 },
    HstcpAimdVal { cwnd: 28193, md: 39 },
    HstcpAimdVal { cwnd: 29557, md: 39 },
    HstcpAimdVal { cwnd: 30975, md: 38 },
    HstcpAimdVal { cwnd: 32450, md: 38 },
    HstcpAimdVal { cwnd: 33986, md: 37 },
    HstcpAimdVal { cwnd: 35586, md: 36 },
    HstcpAimdVal { cwnd: 37253, md: 36 },
    HstcpAimdVal { cwnd: 38992, md: 35 },
    HstcpAimdVal { cwnd: 40808, md: 35 },
    HstcpAimdVal { cwnd: 42707, md: 34 },
    HstcpAimdVal { cwnd: 44694, md: 33 },
    HstcpAimdVal { cwnd: 46776, md: 33 },
    HstcpAimdVal { cwnd: 48961, md: 32 },
    HstcpAimdVal { cwnd: 51258, md: 32 },
    HstcpAimdVal { cwnd: 53677, md: 31 },
    HstcpAimdVal { cwnd: 56230, md: 30 },
    HstcpAimdVal { cwnd: 58932, md: 30 },
    HstcpAimdVal { cwnd: 61799, md: 29 },
    HstcpAimdVal { cwnd: 64851, md: 28 },
    HstcpAimdVal { cwnd: 68113, md: 28 },
    HstcpAimdVal { cwnd: 71617, md: 27 },
    HstcpAimdVal { cwnd: 75401, md: 26 },
    HstcpAimdVal { cwnd: 79517, md: 26 },
    HstcpAimdVal { cwnd: 84035, md: 25 },
    HstcpAimdVal { cwnd: 89053, md: 24 },
];

const HSTCP_AIMD_MAX: usize = HSTCP_AIMD_VALS.len();

#[repr(C)]
struct Hstcp {
    ai: u32,
}

unsafe fn hstcp_init(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    (*ca).ai = 0;

    /* Ensure the MD arithmetic works.  This is somewhat pedantic,
     * since I don't think we will see a cwnd this large. :) */
    (*tp).snd_cwnd_clamp = core::cmp::min((*tp).snd_cwnd_clamp, 0xffffffff / 128);
}

unsafe fn hstcp_cong_avoid(sk: *mut sock, _ack: u32, acked: u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if tcp_in_slow_start(tp) {
        tcp_slow_start(tp, acked);
    } else {
        /* Update AIMD parameters.
         *
         * We want to guarantee that:
         *     hstcp_aimd_vals[ca->ai-1].cwnd <
         *     snd_cwnd <=
         *     hstcp_aimd_vals[ca->ai].cwnd
         */
        if tcp_snd_cwnd(tp) > HSTCP_AIMD_VALS[(*ca).ai as usize].cwnd {
            while tcp_snd_cwnd(tp) > HSTCP_AIMD_VALS[(*ca).ai as usize].cwnd
                && (*ca).ai < (HSTCP_AIMD_MAX - 1) as u32
            {
                (*ca).ai += 1;
            }
        } else if (*ca).ai != 0
            && tcp_snd_cwnd(tp) <= HSTCP_AIMD_VALS[((*ca).ai - 1) as usize].cwnd
        {
            while (*ca).ai != 0
                && tcp_snd_cwnd(tp) <= HSTCP_AIMD_VALS[((*ca).ai - 1) as usize].cwnd
            {
                (*ca).ai -= 1;
            }
        }

        /* Do additive increase */
        if tcp_snd_cwnd(tp) < (*tp).snd_cwnd_clamp {
            /* cwnd = cwnd + a(w) / cwnd */
            (*tp).snd_cwnd_cnt += (*ca).ai + 1;
            if (*tp).snd_cwnd_cnt >= tcp_snd_cwnd(tp) {
                (*tp).snd_cwnd_cnt -= tcp_snd_cwnd(tp);
                tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1);
            }
        }
    }
}

unsafe fn hstcp_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    /* Do multiplicative decrease */
    core::cmp::max(
        tcp_snd_cwnd(tp)
            - ((tcp_snd_cwnd(tp) * HSTCP_AIMD_VALS[(*ca).ai as usize].md) >> 8),
        2,
    )
}

static mut TCP_HIGHSPEED: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(hstcp_init),
    ssthresh: Some(hstcp_ssthresh),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    cong_avoid: Some(hstcp_cong_avoid),
    owner: THIS_MODULE,
    name: *b"highspeed\0",
};

unsafe fn hstcp_register() -> i32 {
    /* BUILD_BUG_ON(sizeof(struct hstcp) > ICSK_CA_PRIV_SIZE); */
    tcp_register_congestion_control(&raw mut TCP_HIGHSPEED)
}

unsafe fn hstcp_unregister() {
    tcp_unregister_congestion_control(&raw mut TCP_HIGHSPEED);
}

/* module_init(hstcp_register); */
/* module_exit(hstcp_unregister); */

/* MODULE_AUTHOR("John Heffner"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("High Speed TCP"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
