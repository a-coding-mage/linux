// SPDX-License-Identifier: GPL-2.0
/* RTT/RTO calculation.
 *
 * Adapted from TCP for AF_RXRPC by David Howells (dhowells@redhat.com)
 *
 * https://tools.ietf.org/html/rfc6298
 * https://tools.ietf.org/html/rfc1122#section-4.2.3.1
 * http://ccr.sigcomm.org/archive/1995/jan95/ccr-9501-partridge87.pdf
 */

// #include <linux/net.h>
// #include "ar-internal.h"

const RXRPC_RTO_MAX: u32 = 120 * USEC_PER_SEC;
const RXRPC_TIMEOUT_INIT: u32 = 1 * USEC_PER_SEC; // RFC6298 2.1 initial RTO value

// As rxrpc_jiffies32.
macro_rules! rxrpc_jiffies32 {
    () => { jiffies as u32 };
}

unsafe fn rxrpc_rto_min_us(_call: *mut rxrpc_call) -> u32 {
    200
}

unsafe fn __rxrpc_set_rto(call: *const rxrpc_call) -> u32 {
    ((*call).srtt_us >> 3).wrapping_add((*call).rttvar_us)
}

fn rxrpc_bound_rto(rto: u32) -> u32 {
    (rto.wrapping_add(100000)).clamp(200000, RXRPC_RTO_MAX)
}

/*
 * Called to compute a smoothed rtt estimate. The data fed to this
 * routine either comes from timestamps, or from segments that were
 * known _not_ to have been retransmitted [see Karn/Partridge
 * Proceedings SIGCOMM 87]. The algorithm is from the SIGCOMM 88
 * piece by Van Jacobson.
 * NOTE: the next three routines used to be one big routine.
 * To save cycles in the RFC 1323 implementation it was better to break
 * it up into three procedures. -- erics
 */
unsafe fn rxrpc_rtt_estimator(call: *mut rxrpc_call, mut sample_rtt_us: i64) {
    let mut m = sample_rtt_us; // RTT
    let mut srtt = (*call).srtt_us;

    /* The following amusing code comes from Jacobson's
     * article in SIGCOMM '88.  Note that rtt and mdev
     * are scaled versions of rtt and mean deviation.
     * This is designed to be as fast as possible
     * m stands for "measurement".
     *
     * On a 1990 paper the rto value is changed to:
     * RTO = rtt + 4 * mdev
     *
     * Funny. This algorithm seems to be very broken.
     * These formulae increase RTO, when it should be decreased, increase
     * too slowly, when it should be increased quickly, decrease too quickly
     * etc. I guess in BSD RTO takes ONE value, so that it is absolutely
     * does not matter how to _calculate_ it. Seems, it was trap
     * that VJ failed to avoid. 8)
     */
    if srtt != 0 {
        m -= (srtt >> 3) as i64; // m is now error in rtt est
        srtt = (srtt as i64 + m) as u32; // rtt = 7/8 rtt + 1/8 new
        if m < 0 {
            m = -m; // m is now abs(error)
            m -= ((*call).mdev_us >> 2) as i64; // similar update on mdev
            /* This is similar to one of Eifel findings.
             * Eifel blocks mdev updates when rtt decreases.
             * This solution is a bit different: we use finer gain
             * for mdev in this case (alpha*beta).
             * Like Eifel it also prevents growth of rto,
             * but also it limits too fast rto decreases,
             * happening in pure Eifel.
             */
            if m > 0 {
                m >>= 3;
            }
        } else {
            m -= ((*call).mdev_us >> 2) as i64; // similar update on mdev
        }

        (*call).mdev_us = ((*call).mdev_us as i64 + m) as u32; // mdev = 3/4 mdev + 1/4 new
        if (*call).mdev_us > (*call).mdev_max_us {
            (*call).mdev_max_us = (*call).mdev_us;
            if (*call).mdev_max_us > (*call).rttvar_us {
                (*call).rttvar_us = (*call).mdev_max_us;
            }
        }
    } else {
        // no previous measure.
        srtt = (m << 3) as u32; // take the measured time to be rtt
        (*call).mdev_us = (m << 1) as u32; // make sure rto = 3*rtt
        (*call).rttvar_us = umax((*call).mdev_us, rxrpc_rto_min_us(call));
        (*call).mdev_max_us = (*call).rttvar_us;
    }

    (*call).srtt_us = umax(srtt, 1);
}

/*
 * Calculate rto without backoff.  This is the second half of Van Jacobson's
 * routine referred to above.
 */
unsafe fn rxrpc_set_rto(call: *mut rxrpc_call) {
    let rto: u32;

    /* 1. If rtt variance happened to be less 50msec, it is hallucination.
     *    It cannot be less due to utterly erratic ACK generation made
     *    at least by solaris and freebsd. "Erratic ACKs" has _nothing_
     *    to do with delayed acks, because at cwnd>2 true delack timeout
     *    is invisible. Actually, Linux-2.4 also generates erratic
     *    ACKs in some circumstances.
     */
    rto = __rxrpc_set_rto(call);

    /* 2. Fixups made earlier cannot be right.
     *    If we do not estimate RTO correctly without them,
     *    all the algo is pure shit and should be replaced
     *    with correct one. It is exactly, which we pretend to do.
     */

    /* NOTE: clamping at RXRPC_RTO_MIN is not required, current algo
     * guarantees that rto is higher.
     */
    (*call).rto_us = rxrpc_bound_rto(rto);
}

unsafe fn rxrpc_update_rtt_min(call: *mut rxrpc_call, resp_time: ktime_t, rtt_us: i64) {
    /* Window size 5mins in approx usec (ipv4.sysctl_tcp_min_rtt_wlen) */
    let wlen_us: u32 = 5 * NSEC_PER_SEC / 1024;

    minmax_running_min(&mut (*call).min_rtt, wlen_us, resp_time / 1024,
                       if rtt_us as u32 != 0 { rtt_us as u32 } else { jiffies_to_usecs(1) });
}

unsafe fn rxrpc_ack_update_rtt(call: *mut rxrpc_call, resp_time: ktime_t, rtt_us: i64) {
    if rtt_us < 0 {
        return;
    }

    /* Update RACK min RTT [RFC8985 6.1 Step 1]. */
    rxrpc_update_rtt_min(call, resp_time, rtt_us);

    rxrpc_rtt_estimator(call, rtt_us);
    rxrpc_set_rto(call);

    /* Only reset backoff on valid RTT measurement [RFC6298]. */
    (*call).backoff = 0;
}

/*
 * Add RTT information to cache.  This is called in softirq mode and has
 * exclusive access to the call RTT data.
 */
pub unsafe fn rxrpc_call_add_rtt(call: *mut rxrpc_call, why: rxrpc_rtt_rx_trace,
                                 rtt_slot: i32,
                                 send_serial: rxrpc_serial_t, resp_serial: rxrpc_serial_t,
                                 send_time: ktime_t, resp_time: ktime_t) {
    let rtt_us = ktime_to_us(ktime_sub(resp_time, send_time));
    if rtt_us < 0 {
        return;
    }

    rxrpc_ack_update_rtt(call, resp_time, rtt_us);
    if (*call).rtt_count < 3 {
        (*call).rtt_count += 1;
    }
    (*call).rtt_taken += 1;

    WRITE_ONCE((*call).peer.recent_srtt_us, (*call).srtt_us / 8);
    WRITE_ONCE((*call).peer.recent_rto_us, (*call).rto_us);

    trace_rxrpc_rtt_rx(call, why, rtt_slot, send_serial, resp_serial,
                        rtt_us, (*call).srtt_us, (*call).rto_us);
}

/*
 * Get the retransmission timeout to set in nanoseconds, backing it off each
 * time we retransmit.
 */
pub unsafe fn rxrpc_get_rto_backoff(call: *mut rxrpc_call, retrans: bool) -> ktime_t {
    let mut timo_us: u64 = (*call).rto_us as u64;
    let backoff: u32 = READ_ONCE((*call).backoff);

    timo_us <<= backoff;
    if retrans && timo_us * 2 <= RXRPC_RTO_MAX as u64 {
        WRITE_ONCE((*call).backoff, backoff + 1);
    }

    if timo_us < 1 {
        timo_us = 1;
    }

    ns_to_ktime(timo_us * NSEC_PER_USEC as u64)
}

pub unsafe fn rxrpc_call_init_rtt(call: *mut rxrpc_call) {
    (*call).rtt_last_req = KTIME_MIN;
    (*call).rto_us = RXRPC_TIMEOUT_INIT;
    (*call).mdev_us = RXRPC_TIMEOUT_INIT;
    (*call).backoff = 0;
    // minmax_reset(&call->rtt_min, rxrpc_jiffies32, ~0U);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
