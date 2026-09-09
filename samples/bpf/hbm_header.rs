/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2019 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * Include file for Host Bandwidth Management (HBM) programs
 */

#[repr(C)]
pub struct hbm_vqueue {
    pub lock: bpf_spin_lock,
    /* 4 byte hole */
    pub lasttime: u64, /* In ns */
    pub credit: i32,  /* In bytes */
    pub rate: u32,    /* In bytes per NS << 20 */
}

#[repr(C)]
pub struct hbm_queue_stats {
    pub rate: u64, /* in Mbps */
    /* C bit-fields: stats:1, loopback:1, no_cn:1; remaining bits are unused. */
    pub flags: u64,
    pub pkts_marked: u64,
    pub bytes_marked: u64,
    pub pkts_dropped: u64,
    pub bytes_dropped: u64,
    pub pkts_total: u64,
    pub bytes_total: u64,
    pub firstPacketTime: u64,
    pub lastPacketTime: u64,
    pub pkts_ecn_ce: u64,
    pub returnValCount: [u64; 4],
    pub sum_cwnd: u64,
    pub sum_rtt: u64,
    pub sum_cwnd_cnt: u64,
    pub sum_credit: i64,
}

pub const HBM_QUEUE_STATS_STATS: u64 = 1 << 0;
pub const HBM_QUEUE_STATS_LOOPBACK: u64 = 1 << 1;
pub const HBM_QUEUE_STATS_NO_CN: u64 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
