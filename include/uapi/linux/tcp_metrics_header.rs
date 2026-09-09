/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* tcp_metrics.h - TCP Metrics Interface */

/* Dependency: <linux/types.h> */

/* NETLINK_GENERIC related info
 */
pub const TCP_METRICS_GENL_NAME: &str = "tcp_metrics";
pub const TCP_METRICS_GENL_VERSION: i32 = 0x1;

pub const TCP_METRIC_RTT: i32 = 0; /* in ms units */
pub const TCP_METRIC_RTTVAR: i32 = 1; /* in ms units */
pub const TCP_METRIC_SSTHRESH: i32 = 2;
pub const TCP_METRIC_CWND: i32 = 3;
pub const TCP_METRIC_REORDERING: i32 = 4;

pub const TCP_METRIC_RTT_US: i32 = 5; /* in usec units */
pub const TCP_METRIC_RTTVAR_US: i32 = 6; /* in usec units */

/* Always last.  */
pub const __TCP_METRIC_MAX: i32 = 7;

pub const TCP_METRIC_MAX: i32 = __TCP_METRIC_MAX - 1;

/* Re-define enum tcp_metric_index, again, using the values carried
 * as netlink attribute types.
 */
pub const TCP_METRICS_A_METRICS_RTT: i32 = 1;
pub const TCP_METRICS_A_METRICS_RTTVAR: i32 = 2;
pub const TCP_METRICS_A_METRICS_SSTHRESH: i32 = 3;
pub const TCP_METRICS_A_METRICS_CWND: i32 = 4;
pub const TCP_METRICS_A_METRICS_REODERING: i32 = 5;
pub const TCP_METRICS_A_METRICS_RTT_US: i32 = 6;
pub const TCP_METRICS_A_METRICS_RTTVAR_US: i32 = 7;

pub const __TCP_METRICS_A_METRICS_MAX: i32 = 8;
pub const TCP_METRICS_A_METRICS_MAX: i32 = __TCP_METRICS_A_METRICS_MAX - 1;

pub const TCP_METRICS_ATTR_UNSPEC: i32 = 0;
pub const TCP_METRICS_ATTR_ADDR_IPV4: i32 = 1; /* u32 */
pub const TCP_METRICS_ATTR_ADDR_IPV6: i32 = 2; /* binary */
pub const TCP_METRICS_ATTR_AGE: i32 = 3; /* msecs */
pub const TCP_METRICS_ATTR_TW_TSVAL: i32 = 4; /* u32, raw, rcv tsval */
pub const TCP_METRICS_ATTR_TW_TS_STAMP: i32 = 5; /* s32, sec age */
pub const TCP_METRICS_ATTR_VALS: i32 = 6; /* nested +1, u32 */
pub const TCP_METRICS_ATTR_FOPEN_MSS: i32 = 7; /* u16 */
pub const TCP_METRICS_ATTR_FOPEN_SYN_DROPS: i32 = 8; /* u16, count of drops */
pub const TCP_METRICS_ATTR_FOPEN_SYN_DROP_TS: i32 = 9; /* msecs age */
pub const TCP_METRICS_ATTR_FOPEN_COOKIE: i32 = 10; /* binary */
pub const TCP_METRICS_ATTR_SADDR_IPV4: i32 = 11; /* u32 */
pub const TCP_METRICS_ATTR_SADDR_IPV6: i32 = 12; /* binary */
pub const TCP_METRICS_ATTR_PAD: i32 = 13;

pub const __TCP_METRICS_ATTR_MAX: i32 = 14;
pub const TCP_METRICS_ATTR_MAX: i32 = __TCP_METRICS_ATTR_MAX - 1;

pub const TCP_METRICS_CMD_UNSPEC: i32 = 0;
pub const TCP_METRICS_CMD_GET: i32 = 1;
pub const TCP_METRICS_CMD_DEL: i32 = 2;

pub const __TCP_METRICS_CMD_MAX: i32 = 3;
pub const TCP_METRICS_CMD_MAX: i32 = __TCP_METRICS_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
