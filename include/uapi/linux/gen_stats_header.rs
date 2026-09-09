/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependency: <linux/types.h>

#[repr(i32)]
pub enum TcaStats {
    TCA_STATS_UNSPEC,
    TCA_STATS_BASIC,
    TCA_STATS_RATE_EST,
    TCA_STATS_QUEUE,
    TCA_STATS_APP,
    TCA_STATS_RATE_EST64,
    TCA_STATS_PAD,
    TCA_STATS_BASIC_HW,
    TCA_STATS_PKT64,
    __TCA_STATS_MAX,
}

pub const TCA_STATS_MAX: i32 = TcaStats::__TCA_STATS_MAX as i32 - 1;

/**
 * struct gnet_stats_basic - byte/packet throughput statistics
 * @bytes: number of seen bytes
 * @packets: number of seen packets
 */
#[repr(C)]
pub struct gnet_stats_basic {
    pub bytes: __u64,
    pub packets: __u32,
}

/**
 * struct gnet_stats_rate_est - rate estimator
 * @bps: current byte rate
 * @pps: current packet rate
 */
#[repr(C)]
pub struct gnet_stats_rate_est {
    pub bps: __u32,
    pub pps: __u32,
}

/**
 * struct gnet_stats_rate_est64 - rate estimator
 * @bps: current byte rate
 * @pps: current packet rate
 */
#[repr(C)]
pub struct gnet_stats_rate_est64 {
    pub bps: __u64,
    pub pps: __u64,
}

/**
 * struct gnet_stats_queue - queuing statistics
 * @qlen: queue length
 * @backlog: backlog size of queue
 * @drops: number of dropped packets
 * @requeues: number of requeues
 * @overlimits: number of enqueues over the limit
 */
#[repr(C)]
pub struct gnet_stats_queue {
    pub qlen: __u32,
    pub backlog: __u32,
    pub drops: __u32,
    pub requeues: __u32,
    pub overlimits: __u32,
}

/**
 * struct gnet_estimator - rate estimator configuration
 * @interval: sampling period
 * @ewma_log: the log of measurement window weight
 */
#[repr(C)]
pub struct gnet_estimator {
    pub interval: i8,
    pub ewma_log: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
