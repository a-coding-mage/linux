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
 *
 */

// C dependencies: linux/percpu.h, linux/seq_file.h, linux/proc_fs.h,
// "rds.h", and "tcp.h".

#[repr(C)]
pub struct RdsTcpStatistics {
    pub tcp_data_ready_calls: u64,
    pub tcp_write_space_calls: u64,
    pub tcp_sndbuf_full: u64,
    pub tcp_connect_raced: u64,
    pub tcp_listen_closed_stale: u64,
}

#[repr(C)]
pub struct RdsInfoIterator {
    _private: [u8; 0],
}

extern "C" {
    // DEFINE_PER_CPU(struct rds_tcp_statistics, rds_tcp_stats)
    pub static mut rds_tcp_stats: RdsTcpStatistics;
    pub fn rds_tcp_stats_cpu(cpu: i32) -> *const RdsTcpStatistics;
    pub fn rds_num_online_cpus() -> i32;
    pub fn rds_stats_info_copy(
        iter: *mut RdsInfoIterator,
        values: *const u64,
        names: *const *const u8,
        count: usize,
    );
}

static RDS_TCP_STAT_NAMES: [&[u8]; 5] = [
    b"tcp_data_ready_calls\0",
    b"tcp_write_space_calls\0",
    b"tcp_sndbuf_full\0",
    b"tcp_connect_raced\0",
    b"tcp_listen_closed_stale\0",
];

pub unsafe fn rds_tcp_stats_info_copy(iter: *mut RdsInfoIterator, avail: u32) -> u32 {
    let mut stats = RdsTcpStatistics {
        tcp_data_ready_calls: 0,
        tcp_write_space_calls: 0,
        tcp_sndbuf_full: 0,
        tcp_connect_raced: 0,
        tcp_listen_closed_stale: 0,
    };

    if (avail as usize) < RDS_TCP_STAT_NAMES.len() {
        return RDS_TCP_STAT_NAMES.len() as u32;
    }

    let sum = &mut stats as *mut RdsTcpStatistics as *mut u64;
    let mut cpu = 0;
    while cpu < rds_num_online_cpus() {
        let src = rds_tcp_stats_cpu(cpu) as *mut u64;
        for i in 0..(core::mem::size_of::<RdsTcpStatistics>() / core::mem::size_of::<u64>()) {
            *sum.add(i) = (*sum.add(i)).wrapping_add(*src.add(i));
        }
        cpu += 1;
    }

    let names: [*const u8; 5] = [
        RDS_TCP_STAT_NAMES[0].as_ptr(),
        RDS_TCP_STAT_NAMES[1].as_ptr(),
        RDS_TCP_STAT_NAMES[2].as_ptr(),
        RDS_TCP_STAT_NAMES[3].as_ptr(),
        RDS_TCP_STAT_NAMES[4].as_ptr(),
    ];
    rds_stats_info_copy(iter, sum as *const u64, names.as_ptr(), RDS_TCP_STAT_NAMES.len());
    RDS_TCP_STAT_NAMES.len() as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
