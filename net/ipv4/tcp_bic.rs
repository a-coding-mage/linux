// SPDX-License-Identifier: GPL-2.0-only
/*
 * Binary Increase Congestion control for TCP
 * Home page:
 *      http://netsrv.csc.ncsu.edu/twiki/bin/view/Main/BIC
 * This is from the implementation of BICTCP in
 * Lison-Xu, Kahaled Harfoush, and Injong Rhee.
 *  "Binary Increase Congestion Control for Fast, Long Distance
 *  Networks" in InfoComm 2004
 * Available from:
 *  http://netsrv.csc.ncsu.edu/export/bitcp.pdf
 *
 * Unless BIC is enabled and congestion window is large
 * this behaves the same as the original Reno.
 */

// C dependencies: linux/mm.h, linux/module.h, and net/tcp.h.

pub const BICTCP_BETA_SCALE: i32 = 1024;
pub const BICTCP_B: u32 = 4;

static mut fast_convergence: i32 = 1;
static mut max_increment: i32 = 16;
static mut low_window: i32 = 14;
static mut beta: i32 = 819;
static mut initial_ssthresh: i32 = 0;
static mut smooth_part: i32 = 20;

/* BIC TCP Parameters */
#[repr(C)]
pub struct bictcp {
    pub cnt: u32,
    pub last_max_cwnd: u32,
    pub last_cwnd: u32,
    pub last_time: u32,
    pub epoch_start: u32,
    pub delayed_ack: u32,
}

pub const ACK_RATIO_SHIFT: u32 = 4;

#[inline]
unsafe fn bictcp_reset(ca: *mut bictcp) {
    (*ca).cnt = 0;
    (*ca).last_max_cwnd = 0;
    (*ca).last_cwnd = 0;
    (*ca).last_time = 0;
    (*ca).epoch_start = 0;
    (*ca).delayed_ack = 2 << ACK_RATIO_SHIFT;
}

unsafe fn bictcp_init(sk: *mut sock) {
    let ca: *mut bictcp = inet_csk_ca(sk);

    bictcp_reset(ca);

    if initial_ssthresh != 0 {
        WRITE_ONCE((*tcp_sk(sk)).snd_ssthresh, initial_ssthresh);
    }
}

/*
 * Compute congestion window to use.
 */
#[inline]
unsafe fn bictcp_update(ca: *mut bictcp, cwnd: u32) {
    if (*ca).last_cwnd == cwnd
        && (tcp_jiffies32.wrapping_sub((*ca).last_time) as i32) <= HZ / 32
    {
        return;
    }

    (*ca).last_cwnd = cwnd;
    (*ca).last_time = tcp_jiffies32;

    if (*ca).epoch_start == 0 {
        (*ca).epoch_start = tcp_jiffies32;
    }

    if (cwnd as i32) <= low_window {
        (*ca).cnt = cwnd;
        return;
    }

    if cwnd < (*ca).last_max_cwnd {
        let dist: u32 = ((*ca).last_max_cwnd - cwnd) / BICTCP_B;

        if (dist as i32) > max_increment {
            (*ca).cnt = cwnd / max_increment as u32;
        } else if dist <= 1 {
            (*ca).cnt = cwnd * smooth_part as u32 / BICTCP_B;
        } else {
            (*ca).cnt = cwnd / dist;
        }
    } else {
        if cwnd < (*ca).last_max_cwnd + BICTCP_B {
            (*ca).cnt = cwnd * smooth_part as u32 / BICTCP_B;
        } else if cwnd < (*ca).last_max_cwnd + (max_increment as u32) * (BICTCP_B - 1) {
            (*ca).cnt = cwnd * (BICTCP_B - 1) / (cwnd - (*ca).last_max_cwnd);
        } else {
            (*ca).cnt = cwnd / max_increment as u32;
        }
    }

    if (*ca).last_max_cwnd == 0 {
        if (*ca).cnt > 20 {
            (*ca).cnt = 20;
        }
    }

    (*ca).cnt = ((*ca).cnt << ACK_RATIO_SHIFT) / (*ca).delayed_ack;
    if (*ca).cnt == 0 {
        (*ca).cnt = 1;
    }
}

unsafe fn bictcp_cong_avoid(sk: *mut sock, ack: u32, mut acked: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let ca: *mut bictcp = inet_csk_ca(sk);

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if tcp_in_slow_start(tp) {
        acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            return;
        }
    }
    bictcp_update(ca, tcp_snd_cwnd(tp));
    tcp_cong_avoid_ai(tp, (*ca).cnt, acked);
}

/*
 * behave like Reno until low_window is reached,
 * then increase congestion window slowly
 */
unsafe fn bictcp_recalc_ssthresh(sk: *mut sock) -> u32 {
    let tp: *const tcp_sock = tcp_sk(sk);
    let ca: *mut bictcp = inet_csk_ca(sk);

    (*ca).epoch_start = 0;

    if tcp_snd_cwnd(tp) < (*ca).last_max_cwnd && fast_convergence != 0 {
        (*ca).last_max_cwnd = (tcp_snd_cwnd(tp) * (BICTCP_BETA_SCALE + beta) as u32)
            / (2 * BICTCP_BETA_SCALE) as u32;
    } else {
        (*ca).last_max_cwnd = tcp_snd_cwnd(tp);
    }

    if (tcp_snd_cwnd(tp) as i32) <= low_window {
        core::cmp::max(tcp_snd_cwnd(tp) >> 1, 2)
    } else {
        core::cmp::max(tcp_snd_cwnd(tp) * beta as u32 / BICTCP_BETA_SCALE as u32, 2)
    }
}

unsafe fn bictcp_state(sk: *mut sock, new_state: u8) {
    if new_state == TCP_CA_Loss {
        bictcp_reset(inet_csk_ca(sk));
    }
}

/* Track delayed acknowledgment ratio using sliding window
 * ratio = (15*ratio + sample) / 16
 */
unsafe fn bictcp_acked(sk: *mut sock, sample: *const ack_sample) {
    let icsk: *const inet_connection_sock = inet_csk(sk);

    if (*icsk).icsk_ca_state == TCP_CA_Open {
        let ca: *mut bictcp = inet_csk_ca(sk);

        (*ca).delayed_ack += (*sample).pkts_acked - ((*ca).delayed_ack >> ACK_RATIO_SHIFT);
    }
}

static mut bictcp: tcp_congestion_ops = tcp_congestion_ops {
    init: Some(bictcp_init),
    ssthresh: Some(bictcp_recalc_ssthresh),
    cong_avoid: Some(bictcp_cong_avoid),
    set_state: Some(bictcp_state),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    pkts_acked: Some(bictcp_acked),
    owner: THIS_MODULE,
    name: *b"bic\0",
};

unsafe fn bictcp_register() -> i32 {
    BUILD_BUG_ON(core::mem::size_of::<bictcp>() > ICSK_CA_PRIV_SIZE);
    tcp_register_congestion_control(&mut bictcp)
}

unsafe fn bictcp_unregister() {
    tcp_unregister_congestion_control(&mut bictcp);
}

// module_init(bictcp_register);
// module_exit(bictcp_unregister);
// MODULE_AUTHOR("Stephen Hemminger");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("BIC TCP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
