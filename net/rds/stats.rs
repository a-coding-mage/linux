/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Linux kernel declarations and macros supplied by the surrounding tree.
#[repr(C)]
pub struct rds_statistics {
    pub counters: [u64; 38],
}
#[repr(C)]
pub struct rds_info_iterator { _private: [u8; 0] }
#[repr(C)]
pub struct socket { _private: [u8; 0] }
#[repr(C)]
pub struct rds_info_lengths { pub each: usize, pub nr: usize }
#[repr(C)]
pub struct rds_info_counter { pub name: [c_char; 32], pub value: u64 }

#[no_mangle]
pub static mut rds_stats: rds_statistics = rds_statistics { counters: [0; 38] };

static RDS_STAT_NAMES: [&[u8]; 38] = [
    b"conn_reset\0", b"recv_drop_bad_checksum\0", b"recv_drop_old_seq\0",
    b"recv_drop_no_sock\0", b"recv_drop_dead_sock\0", b"recv_deliver_raced\0",
    b"recv_delivered\0", b"recv_queued\0", b"recv_immediate_retry\0",
    b"recv_delayed_retry\0", b"recv_ack_required\0", b"recv_rdma_bytes\0",
    b"recv_ping\0", b"send_queue_empty\0", b"send_queue_full\0",
    b"send_lock_contention\0", b"send_lock_queue_raced\0", b"send_immediate_retry\0",
    b"send_delayed_retry\0", b"send_drop_acked\0", b"send_ack_required\0",
    b"send_queued\0", b"send_rdma\0", b"send_rdma_bytes\0", b"send_pong\0",
    b"page_remainder_hit\0", b"page_remainder_miss\0", b"copy_to_user\0",
    b"copy_from_user\0", b"cong_update_queued\0", b"cong_update_received\0",
    b"cong_send_error\0", b"cong_send_blocked\0", b"recv_bytes_added_to_sock\0",
    b"recv_bytes_freed_fromsock\0", b"send_stuck_rm\0", b"mprds_catchup_tx0_retries\0",
];

extern "C" {
    fn rds_info_copy(iter: *mut rds_info_iterator, data: *const c_void, len: usize);
    fn rds_trans_stats_info_copy(iter: *mut rds_info_iterator, avail: c_uint) -> c_uint;
    fn rds_info_deregister_func(which: c_uint, func: unsafe extern "C" fn(*mut socket, c_uint, *mut rds_info_iterator, *mut rds_info_lengths));
    fn rds_info_register_func(which: c_uint, func: unsafe extern "C" fn(*mut socket, c_uint, *mut rds_info_iterator, *mut rds_info_lengths));
}

#[no_mangle]
pub unsafe extern "C" fn rds_stats_info_copy(iter: *mut rds_info_iterator, values: *mut u64, _names: *const *const c_char, nr: usize) {
    let mut ctr: rds_info_counter = core::mem::zeroed();
    for i in 0..nr {
        let value = *values.add(i);
        ctr.value = value;
        rds_info_copy(iter, &ctr as *const _ as *const c_void, core::mem::size_of::<rds_info_counter>());
    }
}

unsafe extern "C" fn rds_stats_info(_sock: *mut socket, len: c_uint, iter: *mut rds_info_iterator, lens: *mut rds_info_lengths) {
    let mut stats: rds_statistics = core::mem::zeroed();
    let mut avail = (len as usize) / core::mem::size_of::<rds_info_counter>();
    if avail >= RDS_STAT_NAMES.len() {
        for i in 0..stats.counters.len() { stats.counters[i] = stats.counters[i].wrapping_add(rds_stats.counters[i]); }
        rds_stats_info_copy(iter, stats.counters.as_mut_ptr(), core::ptr::null(), RDS_STAT_NAMES.len());
        avail -= RDS_STAT_NAMES.len();
    }
    (*lens).each = core::mem::size_of::<rds_info_counter>();
    (*lens).nr = rds_trans_stats_info_copy(iter, avail as c_uint) as usize + RDS_STAT_NAMES.len();
}

pub unsafe extern "C" fn rds_stats_exit() { rds_info_deregister_func(0, rds_stats_info); }

pub unsafe extern "C" fn rds_stats_init() -> c_int {
    rds_info_register_func(0, rds_stats_info);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
