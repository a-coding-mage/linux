// SPDX-License-Identifier: GPL-2.0-only

/* WARNING: This implementation is not necessarily the same
 * as the tcp_cubic.c.  The purpose is mainly for testing
 * the kernel BPF logic.
 *
 * Highlights:
 * 1. CONFIG_HZ .kconfig map is used.
 * 2. In bictcp_update(), calculation is changed to use usec
 *    resolution (i.e. USEC_PER_JIFFY) instead of using jiffies.
 *    Thus, usecs_to_jiffies() is not used in the bpf_cubic.c.
 * 3. In bitctcp_update() [under tcp_friendliness], the original
 *    "while (ca->ack_cnt > delta)" loop is changed to the equivalent
 *    "ca->ack_cnt / delta" operation.
 */

// Dependency intent from C includes:
// "bpf_tracing_net.h", <bpf/bpf_tracing.h>, and <errno.h>.

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;

#[repr(C)]
pub struct sock {
    pub sk_pacing_rate: c_ulong,
    pub sk_pacing_status: i32,
}

#[repr(C)]
pub struct tcp_sock {
    pub tcp_mstamp: __u32,
    pub snd_nxt: __u32,
    pub snd_ssthresh: __u32,
    pub lsndtime: __u32,
    pub snd_cwnd: __u32,
}

#[repr(C)]
pub struct ack_sample {
    pub rtt_us: __s32,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: *mut core::ffi::c_void,
    pub ssthresh: *mut core::ffi::c_void,
    pub cong_avoid: *mut core::ffi::c_void,
    pub set_state: *mut core::ffi::c_void,
    pub undo_cwnd: *mut core::ffi::c_void,
    pub cwnd_event_tx_start: *mut core::ffi::c_void,
    pub pkts_acked: *mut core::ffi::c_void,
    pub name: *const i8,
}

pub type c_ulong = u64;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

const fn clamp_u32(val: __u32, lo: __u32, hi: __u32) -> __u32 {
    let max_val = if val > lo { val } else { lo };
    if max_val < hi { max_val } else { hi }
}

extern "C" {
    fn tcp_slow_start(tp: *mut tcp_sock, acked: __u32) -> __u32;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, w: __u32, acked: __u32);
    fn tcp_sk(sk: *const sock) -> *mut tcp_sock;
    fn inet_csk_ca(sk: *const sock) -> *mut bpf_bictcp;
    fn bpf_setsockopt(
        sk: *mut sock,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: __u32,
    ) -> i32;
    fn after(seq1: __u32, seq2: __u32) -> bool;
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *const tcp_sock) -> bool;
    fn tcp_reno_undo_cwnd(sk: *mut sock) -> __u32;

    static mut CONFIG_HZ: c_ulong;
    static mut tcp_jiffies32: __u32;
}

const SOL_TCP: i32 = 6;
const TCP_NODELAY: i32 = 1;
const EOPNOTSUPP: i32 = 95;
const TCP_CA_LOSS: __u8 = 4;
const SK_PACING_NONE: i32 = 0;

const BICTCP_BETA_SCALE: i32 = 1024; /* Scale factor beta calculation
                                      * max_cwnd = snd_cwnd * beta
                                      */
const BICTCP_HZ: __u32 = 10; /* BIC HZ 2^10 = 1024 */

/* Two methods of hybrid slow start */
const HYSTART_ACK_TRAIN: i32 = 0x1;
const HYSTART_DELAY: i32 = 0x2;

/* Number of delay samples for detecting the increase of delay */
const HYSTART_MIN_SAMPLES: __u8 = 8;
const HYSTART_DELAY_MIN: __u32 = 4000; /* 4ms */
const HYSTART_DELAY_MAX: __u32 = 16000; /* 16 ms */

const fn hystart_delay_thresh(x: __u32) -> __u32 {
    clamp_u32(x, HYSTART_DELAY_MIN, HYSTART_DELAY_MAX)
}

static mut fast_convergence: i32 = 1;
const beta: i32 = 717; /* = 717/1024 (BICTCP_BETA_SCALE) */
static mut initial_ssthresh: i32 = 0;
const bic_scale: i32 = 41;
static mut tcp_friendliness: i32 = 1;

static mut hystart: i32 = 1;
static mut hystart_detect: i32 = HYSTART_ACK_TRAIN | HYSTART_DELAY;
static mut hystart_low_window: i32 = 16;
static mut hystart_ack_delta_us: i32 = 2000;

const cube_rtt_scale: __u32 = (bic_scale * 10) as __u32; /* 1024*c/rtt */
const beta_scale: __u32 =
    (8 * (BICTCP_BETA_SCALE + beta) / 3 / (BICTCP_BETA_SCALE - beta)) as __u32;
/* calculate the "K" for (wmax-cwnd) = c/rtt * K^3
 *  so K = cubic_root( (wmax-cwnd)*rtt/c )
 * the unit of K is bictcp_HZ=2^10, not HZ
 *
 *  c = bic_scale >> 10
 *  rtt = 100ms
 *
 * the following code has been designed and tested for
 * cwnd < 1 million packets
 * RTT < 100 seconds
 * HZ < 1,000,00  (corresponding to 10 nano-second)
 */

/* 1/c * 2^2*bictcp_HZ * srtt, 2^40 */
const cube_factor: __u64 = (1u64 << (10 + 3 * BICTCP_HZ)) / ((bic_scale * 10) as __u64);

/* BIC TCP Parameters */
#[repr(C)]
pub struct bpf_bictcp {
    pub cnt: __u32,              /* increase cwnd by 1 after ACKs */
    pub last_max_cwnd: __u32,    /* last maximum snd_cwnd */
    pub last_cwnd: __u32,        /* the last snd_cwnd */
    pub last_time: __u32,        /* time when updated last_cwnd */
    pub bic_origin_point: __u32, /* origin point of bic function */
    pub bic_K: __u32,            /* time to origin point
                                  * from the beginning of the current epoch
                                  */
    pub delay_min: __u32,        /* min delay (usec) */
    pub epoch_start: __u32,      /* beginning of an epoch */
    pub ack_cnt: __u32,          /* number of acks */
    pub tcp_cwnd: __u32,         /* estimated tcp cwnd */
    pub unused: __u16,
    pub sample_cnt: __u8,        /* number of samples to decide curr_rtt */
    pub found: __u8,             /* the exit point is found? */
    pub round_start: __u32,      /* beginning of each round */
    pub end_seq: __u32,          /* end_seq of the round */
    pub last_ack: __u32,         /* last time when the ACK spacing is close */
    pub curr_rtt: __u32,         /* the minimum rtt of current round */
}

unsafe fn bictcp_reset(ca: *mut bpf_bictcp) {
    (*ca).cnt = 0;
    (*ca).last_max_cwnd = 0;
    (*ca).last_cwnd = 0;
    (*ca).last_time = 0;
    (*ca).bic_origin_point = 0;
    (*ca).bic_K = 0;
    (*ca).delay_min = 0;
    (*ca).epoch_start = 0;
    (*ca).ack_cnt = 0;
    (*ca).tcp_cwnd = 0;
    (*ca).found = 0;
}

unsafe fn hz() -> c_ulong {
    CONFIG_HZ
}

const USEC_PER_MSEC: c_ulong = 1000;
const USEC_PER_SEC: c_ulong = 1000000;

unsafe fn usec_per_jiffy() -> c_ulong {
    USEC_PER_SEC / hz()
}

unsafe fn div64_u64(dividend: __u64, divisor: __u64) -> __u64 {
    dividend / divisor
}

unsafe fn div64_ul(dividend: __u64, divisor: c_ulong) -> __u64 {
    div64_u64(dividend, divisor as __u64)
}

const BITS_PER_U64: i32 = (core::mem::size_of::<__u64>() * 8) as i32;

fn fls64(mut x: __u64) -> i32 {
    let mut num = BITS_PER_U64 - 1;

    if x == 0 {
        return 0;
    }

    if (x & (!0u64 << (BITS_PER_U64 - 32))) == 0 {
        num -= 32;
        x <<= 32;
    }
    if (x & (!0u64 << (BITS_PER_U64 - 16))) == 0 {
        num -= 16;
        x <<= 16;
    }
    if (x & (!0u64 << (BITS_PER_U64 - 8))) == 0 {
        num -= 8;
        x <<= 8;
    }
    if (x & (!0u64 << (BITS_PER_U64 - 4))) == 0 {
        num -= 4;
        x <<= 4;
    }
    if (x & (!0u64 << (BITS_PER_U64 - 2))) == 0 {
        num -= 2;
        x <<= 2;
    }
    if (x & (!0u64 << (BITS_PER_U64 - 1))) == 0 {
        num -= 1;
    }

    num + 1
}

unsafe fn bictcp_clock_us(sk: *const sock) -> __u32 {
    (*tcp_sk(sk)).tcp_mstamp
}

unsafe fn bictcp_hystart_reset(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    (*ca).last_ack = bictcp_clock_us(sk);
    (*ca).round_start = (*ca).last_ack;
    (*ca).end_seq = (*tp).snd_nxt;
    (*ca).curr_rtt = !0u32;
    (*ca).sample_cnt = 0;
}

#[no_mangle]
pub static mut nodelay_init_reject: bool = false;
#[no_mangle]
pub static mut nodelay_cwnd_event_tx_start_reject: bool = false;

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let true_val: i32 = 1;

    let ret = bpf_setsockopt(
        sk,
        SOL_TCP,
        TCP_NODELAY,
        &true_val as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&true_val) as __u32,
    );
    if ret == -EOPNOTSUPP {
        nodelay_init_reject = true;
    }

    bictcp_reset(ca);

    if hystart != 0 {
        bictcp_hystart_reset(sk);
    }

    if hystart == 0 && initial_ssthresh != 0 {
        (*tcp_sk(sk)).snd_ssthresh = initial_ssthresh as __u32;
    }
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_cwnd_event_tx_start(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let now: __u32 = tcp_jiffies32;
    let true_val: i32 = 1;

    let ret = bpf_setsockopt(
        sk,
        SOL_TCP,
        TCP_NODELAY,
        &true_val as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&true_val) as __u32,
    );
    if ret == -EOPNOTSUPP {
        nodelay_cwnd_event_tx_start_reject = true;
    }

    let delta: __s32 = now.wrapping_sub((*tcp_sk(sk)).lsndtime) as __s32;

    /* We were application limited (idle) for a while.
     * Shift epoch_start to keep cwnd growth to cubic curve.
     */
    if (*ca).epoch_start != 0 && delta > 0 {
        (*ca).epoch_start = (*ca).epoch_start.wrapping_add(delta as __u32);
        if after((*ca).epoch_start, now) {
            (*ca).epoch_start = now;
        }
    }
}

/*
 * cbrt(x) MSB values for x MSB values in [0..63].
 * Precomputed then refined by hand - Willy Tarreau
 *
 * For x in [0..63],
 *   v = cbrt(x << 18) - 1
 *   cbrt(x) = (v[x] + 10) >> 6
 */
static v: [__u8; 64] = [
    /* 0x00 */ 0, 54, 54, 54, 118, 118, 118, 118,
    /* 0x08 */ 123, 129, 134, 138, 143, 147, 151, 156,
    /* 0x10 */ 157, 161, 164, 168, 170, 173, 176, 179,
    /* 0x18 */ 181, 185, 187, 190, 192, 194, 197, 199,
    /* 0x20 */ 200, 202, 204, 206, 209, 211, 213, 215,
    /* 0x28 */ 217, 219, 221, 222, 224, 225, 227, 229,
    /* 0x30 */ 231, 232, 234, 236, 237, 239, 240, 242,
    /* 0x38 */ 244, 245, 246, 248, 250, 251, 252, 254,
];

/* calculate the cubic root of x using a table lookup followed by one
 * Newton-Raphson iteration.
 * Avg err ~= 0.195%
 */
unsafe fn cubic_root(a: __u64) -> __u32 {
    let mut x: __u32;
    let mut b: __u32;
    let shift: __u32;

    if a < 64 {
        /* a in [0..63] */
        return ((v[a as __u32 as usize] as __u32) + 35) >> 6;
    }

    b = fls64(a) as __u32;
    b = ((b * 84) >> 8) - 1;
    shift = (a >> (b * 3)) as __u32;

    /* it is needed for verifier's bound check on v */
    if shift >= 64 {
        return 0;
    }

    x = (((v[shift as usize] as __u32 + 10) << b) as __u32) >> 6;

    /*
     * Newton-Raphson iteration
     *                         2
     * x    = ( 2 * x  +  a / x  ) / 3
     *  k+1          k         k
     */
    x = 2 * x + div64_u64(a, (x as __u64) * ((x - 1) as __u64)) as __u32;
    x = (x * 341) >> 10;
    x
}

/*
 * Compute congestion window to use.
 */
unsafe fn bictcp_update(ca: *mut bpf_bictcp, cwnd: __u32, acked: __u32) {
    let mut delta: __u32;
    let bic_target: __u32;
    let mut max_cnt: __u32;
    let offs: __u64;
    let mut t: __u64;

    (*ca).ack_cnt = (*ca).ack_cnt.wrapping_add(acked); /* count the number of ACKed packets */

    if (*ca).last_cwnd == cwnd
        && (tcp_jiffies32.wrapping_sub((*ca).last_time) as __s32) <= (hz() / 32) as __s32
    {
        return;
    }

    /* The CUBIC function can update ca->cnt at most once per jiffy.
     * On all cwnd reduction events, ca->epoch_start is set to 0,
     * which will force a recalculation of ca->cnt.
     */
    if (*ca).epoch_start != 0 && tcp_jiffies32 == (*ca).last_time {
        tcp_friendliness_block(ca, cwnd);
        (*ca).cnt = core::cmp::max((*ca).cnt, 2u32);
        return;
    }

    (*ca).last_cwnd = cwnd;
    (*ca).last_time = tcp_jiffies32;

    if (*ca).epoch_start == 0 {
        (*ca).epoch_start = tcp_jiffies32; /* record beginning */
        (*ca).ack_cnt = acked; /* start counting */
        (*ca).tcp_cwnd = cwnd; /* syn with cubic */

        if (*ca).last_max_cwnd <= cwnd {
            (*ca).bic_K = 0;
            (*ca).bic_origin_point = cwnd;
        } else {
            /* Compute new K based on
             * (wmax-cwnd) * (srtt>>3 / HZ) / c * 2^(3*bictcp_HZ)
             */
            (*ca).bic_K = cubic_root(cube_factor * ((*ca).last_max_cwnd - cwnd) as __u64);
            (*ca).bic_origin_point = (*ca).last_max_cwnd;
        }
    }

    /* cubic function - calc*/
    /* calculate c * time^3 / rtt,
     *  while considering overflow in calculation of time^3
     * (so time^3 is done by using 64 bit)
     * and without the support of division of 64bit numbers
     * (so all divisions are done by using 32 bit)
     *  also NOTE the unit of those variables
     *	  time  = (t - K) / 2^bictcp_HZ
     *	  c = bic_scale >> 10
     * rtt  = (srtt >> 3) / HZ
     * !!! The following code does not have overflow problems,
     * if the cwnd < 1 million packets !!!
     */

    t = ((tcp_jiffies32.wrapping_sub((*ca).epoch_start) as __s32) as __u64) * usec_per_jiffy();
    t = t.wrapping_add((*ca).delay_min as __u64);
    /* change the unit from usec to bictcp_HZ */
    t <<= BICTCP_HZ;
    t /= USEC_PER_SEC;

    if t < (*ca).bic_K as __u64 {
        /* t - K */
        offs = (*ca).bic_K as __u64 - t;
    } else {
        offs = t - (*ca).bic_K as __u64;
    }

    /* c/rtt * (t-K)^3 */
    delta = ((cube_rtt_scale as __u64 * offs * offs * offs) >> (10 + 3 * BICTCP_HZ)) as __u32;
    if t < (*ca).bic_K as __u64 {
        /* below origin*/
        bic_target = (*ca).bic_origin_point - delta;
    } else {
        /* above origin*/
        bic_target = (*ca).bic_origin_point + delta;
    }

    /* cubic function - calc bictcp_cnt*/
    if bic_target > cwnd {
        (*ca).cnt = cwnd / (bic_target - cwnd);
    } else {
        (*ca).cnt = 100 * cwnd; /* very small increment*/
    }

    /*
     * The initial growth of cubic function may be too conservative
     * when the available bandwidth is still unknown.
     */
    if (*ca).last_max_cwnd == 0 && (*ca).cnt > 20 {
        (*ca).cnt = 20; /* increase cwnd 5% per RTT */
    }

    tcp_friendliness_block(ca, cwnd);

    /* The maximum rate of cwnd increase CUBIC allows is 1 packet per
     * 2 packets ACKed, meaning cwnd grows at 1.5x per RTT.
     */
    (*ca).cnt = core::cmp::max((*ca).cnt, 2u32);
}

unsafe fn tcp_friendliness_block(ca: *mut bpf_bictcp, cwnd: __u32) {
    let mut delta: __u32;
    let max_cnt: __u32;

    /* TCP Friendly */
    if tcp_friendliness != 0 {
        let scale: __u32 = beta_scale;
        let n: __u32;

        /* update tcp cwnd */
        delta = (cwnd * scale) >> 3;
        if (*ca).ack_cnt > delta && delta != 0 {
            n = (*ca).ack_cnt / delta;
            (*ca).ack_cnt -= n * delta;
            (*ca).tcp_cwnd += n;
        }

        if (*ca).tcp_cwnd > cwnd {
            /* if bic is slower than tcp */
            delta = (*ca).tcp_cwnd - cwnd;
            max_cnt = cwnd / delta;
            if (*ca).cnt > max_cnt {
                (*ca).cnt = max_cnt;
            }
        }
    }
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_cong_avoid(sk: *mut sock, ack: __u32, mut acked: __u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if tcp_in_slow_start(tp) {
        if hystart != 0 && after(ack, (*ca).end_seq) {
            bictcp_hystart_reset(sk);
        }
        acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            return;
        }
    }
    bictcp_update(ca, (*tp).snd_cwnd, acked);
    tcp_cong_avoid_ai(tp, (*ca).cnt, acked);
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_recalc_ssthresh(sk: *mut sock) -> __u32 {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);

    (*ca).epoch_start = 0; /* end of epoch */

    /* Wmax and fast convergence */
    if (*tp).snd_cwnd < (*ca).last_max_cwnd && fast_convergence != 0 {
        (*ca).last_max_cwnd =
            ((*tp).snd_cwnd * (BICTCP_BETA_SCALE + beta) as __u32) / (2 * BICTCP_BETA_SCALE) as __u32;
    } else {
        (*ca).last_max_cwnd = (*tp).snd_cwnd;
    }

    core::cmp::max(((*tp).snd_cwnd * beta as __u32) / BICTCP_BETA_SCALE as __u32, 2u32)
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_state(sk: *mut sock, new_state: __u8) {
    if new_state == TCP_CA_LOSS {
        bictcp_reset(inet_csk_ca(sk));
        bictcp_hystart_reset(sk);
    }
}

const GSO_MAX_SIZE: __u32 = 65536;

/* Account for TSO/GRO delays.
 * Otherwise short RTT flows could get too small ssthresh, since during
 * slow start we begin with small TSO packets and ca->delay_min would
 * not account for long aggregation delay when TSO packets get bigger.
 * Ideally even with a very small RTT we would like to have at least one
 * TSO packet being sent and received by GRO, and another one in qdisc layer.
 * We apply another 100% factor because @rate is doubled at this point.
 * We cap the cushion to 1ms.
 */
unsafe fn hystart_ack_delay(sk: *mut sock) -> __u32 {
    let rate: c_ulong = (*sk).sk_pacing_rate;

    if rate == 0 {
        return 0;
    }
    core::cmp::min(
        USEC_PER_MSEC as __u64,
        div64_ul((GSO_MAX_SIZE as __u64) * 4 * USEC_PER_SEC as __u64, rate),
    ) as __u32
}

unsafe fn hystart_update(sk: *mut sock, delay: __u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    let mut threshold: __u32;

    if (hystart_detect & HYSTART_ACK_TRAIN) != 0 {
        let now: __u32 = bictcp_clock_us(sk);

        /* first detection parameter - ack-train detection */
        if (now.wrapping_sub((*ca).last_ack) as __s32) <= hystart_ack_delta_us {
            (*ca).last_ack = now;

            threshold = (*ca).delay_min + hystart_ack_delay(sk);

            /* Hystart ack train triggers if we get ack past
             * ca->delay_min/2.
             * Pacing might have delayed packets up to RTT/2
             * during slow start.
             */
            if (*sk).sk_pacing_status == SK_PACING_NONE {
                threshold >>= 1;
            }

            if (now.wrapping_sub((*ca).round_start) as __s32) > threshold as __s32 {
                (*ca).found = 1;
                (*tp).snd_ssthresh = (*tp).snd_cwnd;
            }
        }
    }

    if (hystart_detect & HYSTART_DELAY) != 0 {
        /* obtain the minimum delay of more than sampling packets */
        if (*ca).curr_rtt > delay {
            (*ca).curr_rtt = delay;
        }
        if (*ca).sample_cnt < HYSTART_MIN_SAMPLES {
            (*ca).sample_cnt += 1;
        } else if (*ca).curr_rtt > (*ca).delay_min + hystart_delay_thresh((*ca).delay_min >> 3) {
            (*ca).found = 1;
            (*tp).snd_ssthresh = (*tp).snd_cwnd;
        }
    }
}

#[no_mangle]
pub static mut bpf_cubic_acked_called: i32 = 0;

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_acked(sk: *mut sock, sample: *const ack_sample) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    let mut delay: __u32;

    bpf_cubic_acked_called = 1;
    /* Some calls are for duplicates without timestamps */
    if (*sample).rtt_us < 0 {
        return;
    }

    /* Discard delay samples right after fast recovery */
    if (*ca).epoch_start != 0 && (tcp_jiffies32.wrapping_sub((*ca).epoch_start) as __s32) < hz() as __s32 {
        return;
    }

    delay = (*sample).rtt_us as __u32;
    if delay == 0 {
        delay = 1;
    }

    /* first time call or link delay decreases */
    if (*ca).delay_min == 0 || (*ca).delay_min > delay {
        (*ca).delay_min = delay;
    }

    /* hystart triggers when cwnd is larger than some threshold */
    if (*ca).found == 0
        && tcp_in_slow_start(tp)
        && hystart != 0
        && (*tp).snd_cwnd >= hystart_low_window as __u32
    {
        hystart_update(sk, delay);
    }
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_cubic_undo_cwnd(sk: *mut sock) -> __u32 {
    tcp_reno_undo_cwnd(sk)
}

#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut cubic: tcp_congestion_ops = tcp_congestion_ops {
    init: bpf_cubic_init as *mut core::ffi::c_void,
    ssthresh: bpf_cubic_recalc_ssthresh as *mut core::ffi::c_void,
    cong_avoid: bpf_cubic_cong_avoid as *mut core::ffi::c_void,
    set_state: bpf_cubic_state as *mut core::ffi::c_void,
    undo_cwnd: bpf_cubic_undo_cwnd as *mut core::ffi::c_void,
    cwnd_event_tx_start: bpf_cubic_cwnd_event_tx_start as *mut core::ffi::c_void,
    pkts_acked: bpf_cubic_acked as *mut core::ffi::c_void,
    name: b"bpf_cubic\0".as_ptr() as *const i8,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
