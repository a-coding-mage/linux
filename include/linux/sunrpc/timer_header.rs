/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/include/linux/sunrpc/timer.h
 *
 *  Declarations for the RPC transport timer.
 *
 *  Copyright (C) 2002 Trond Myklebust <trond.myklebust@fys.uio.no>
 */

use core::ffi::{c_int, c_long, c_uint, c_ulong};

#[repr(C)]
pub struct rpc_rtt {
    pub timeo: c_ulong,          /* default timeout value */
    pub srtt: [c_ulong; 5],      /* smoothed round trip time << 3 */
    pub sdrtt: [c_ulong; 5],     /* smoothed medium deviation of RTT */
    pub ntimeouts: [c_int; 5],   /* Number of timeouts for the last request */
}

extern "C" {
    pub fn rpc_init_rtt(rt: *mut rpc_rtt, timeo: c_ulong);
    pub fn rpc_update_rtt(rt: *mut rpc_rtt, timer: c_uint, m: c_long);
    pub fn rpc_calc_rto(rt: *mut rpc_rtt, timer: c_uint) -> c_ulong;
}

pub unsafe fn rpc_set_timeo(rt: *mut rpc_rtt, timer: c_int, mut ntimeo: c_int) {
    let t: *mut c_int;
    if timer == 0 {
        return;
    }
    t = (*rt).ntimeouts.as_mut_ptr().add((timer - 1) as usize);
    if ntimeo < *t {
        if *t > 0 {
            *t -= 1;
        }
    } else {
        if ntimeo > 8 {
            ntimeo = 8;
        }
        *t = ntimeo;
    }
}

pub unsafe fn rpc_ntimeo(rt: *mut rpc_rtt, timer: c_int) -> c_int {
    if timer == 0 {
        return 0;
    }
    (*rt).ntimeouts[(timer - 1) as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
