// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/timer.c
 *
 * Estimate RPC request round trip time.
 *
 * Based on packet round-trip and variance estimator algorithms described
 * in appendix A of "Congestion Avoidance and Control" by Van Jacobson
 * and Michael J. Karels (ACM Computer Communication Review; Proceedings
 * of the Sigcomm '88 Symposium in Stanford, CA, August, 1988).
 *
 * This RTT estimator is used only for RPC over datagram protocols.
 *
 * Copyright (C) 2002 Trond Myklebust <trond.myklebust@fys.uio.no>
 */

// External kernel definitions supplied by the surrounding translation unit:
// `HZ` and `rpc_rtt`.

const RPC_RTO_MAX: c_ulong = 60 * HZ;
const RPC_RTO_INIT: c_ulong = HZ / 5;
const RPC_RTO_MIN: c_ulong = HZ / 10;

/**
 * rpc_init_rtt - Initialize an RPC RTT estimator context
 * @rt: context to initialize
 * @timeo: initial timeout value, in jiffies
 *
 */
pub unsafe fn rpc_init_rtt(rt: *mut rpc_rtt, timeo: c_ulong) {
    let mut init: c_ulong = 0;
    let mut i: c_uint;

    (*rt).timeo = timeo;

    if timeo > RPC_RTO_INIT {
        init = (timeo - RPC_RTO_INIT) << 3;
    }
    i = 0;
    while i < 5 {
        (*rt).srtt[i as usize] = init;
        (*rt).sdrtt[i as usize] = RPC_RTO_INIT;
        (*rt).ntimeouts[i as usize] = 0;
        i += 1;
    }
}

/**
 * rpc_update_rtt - Update an RPC RTT estimator context
 * @rt: context to update
 * @timer: timer array index (request type)
 * @m: recent actual RTT, in jiffies
 *
 * NB: When computing the smoothed RTT and standard deviation,
 *     be careful not to produce negative intermediate results.
 */
pub unsafe fn rpc_update_rtt(rt: *mut rpc_rtt, mut timer: c_uint, mut m: c_long) {
    let srtt: *mut c_long;
    let sdrtt: *mut c_long;

    if timer == 0 {
        return;
    }
    timer -= 1;

    /* jiffies wrapped; ignore this one */
    if m < 0 {
        return;
    }

    if m == 0 {
        m = 1;
    }

    srtt = &mut (*rt).srtt[timer as usize] as *mut _ as *mut c_long;
    m -= *srtt >> 3;
    *srtt += m;

    if m < 0 {
        m = -m;
    }

    sdrtt = &mut (*rt).sdrtt[timer as usize] as *mut _ as *mut c_long;
    m -= *sdrtt >> 2;
    *sdrtt += m;

    /* Set lower bound on the variance */
    if *sdrtt < RPC_RTO_MIN as c_long {
        *sdrtt = RPC_RTO_MIN as c_long;
    }
}

/**
 * rpc_calc_rto - Provide an estimated timeout value
 * @rt: context to use for calculation
 * @timer: timer array index (request type)
 *
 * Estimate RTO for an NFS RPC sent via an unreliable datagram.  Use
 * the mean and mean deviation of RTT for the appropriate type of RPC
 * for frequently issued RPCs, and a fixed default for the others.
 *
 * The justification for doing "other" this way is that these RPCs
 * happen so infrequently that timer estimation would probably be
 * stale.  Also, since many of these RPCs are non-idempotent, a
 * conservative timeout is desired.
 *
 * getattr, lookup,
 * read, write, commit     - A+4D
 * other                   - timeo
 */
pub unsafe fn rpc_calc_rto(rt: *mut rpc_rtt, mut timer: c_uint) -> c_ulong {
    let mut res: c_ulong;

    if timer == 0 {
        return (*rt).timeo;
    }
    timer -= 1;

    res = ((*rt).srtt[timer as usize] + 7) >> 3;
    res += (*rt).sdrtt[timer as usize];
    if res > RPC_RTO_MAX {
        res = RPC_RTO_MAX;
    }

    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
