/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/netdev.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const NETDEV_FAMILY_NAME: &str = "netdev";
pub const NETDEV_FAMILY_VERSION: i32 = 1;

/**
 * enum netdev_xdp_act
 * @NETDEV_XDP_ACT_BASIC: XDP features set supported by all drivers
 *   (XDP_ABORTED, XDP_DROP, XDP_PASS, XDP_TX)
 * @NETDEV_XDP_ACT_REDIRECT: The netdev supports XDP_REDIRECT
 * @NETDEV_XDP_ACT_NDO_XMIT: This feature informs if netdev implements
 *   ndo_xdp_xmit callback.
 * @NETDEV_XDP_ACT_XSK_ZEROCOPY: This feature informs if netdev supports AF_XDP
 *   in zero copy mode.
 * @NETDEV_XDP_ACT_HW_OFFLOAD: This feature informs if netdev supports XDP hw
 *   offloading.
 * @NETDEV_XDP_ACT_RX_SG: This feature informs if netdev implements non-linear
 *   XDP buffer support in the driver napi callback.
 * @NETDEV_XDP_ACT_NDO_XMIT_SG: This feature informs if netdev implements
 *   non-linear XDP buffer support in ndo_xdp_xmit callback.
 */
pub const NETDEV_XDP_ACT_BASIC: i32 = 1;
pub const NETDEV_XDP_ACT_REDIRECT: i32 = 2;
pub const NETDEV_XDP_ACT_NDO_XMIT: i32 = 4;
pub const NETDEV_XDP_ACT_XSK_ZEROCOPY: i32 = 8;
pub const NETDEV_XDP_ACT_HW_OFFLOAD: i32 = 16;
pub const NETDEV_XDP_ACT_RX_SG: i32 = 32;
pub const NETDEV_XDP_ACT_NDO_XMIT_SG: i32 = 64;
/* private: */
pub const NETDEV_XDP_ACT_MASK: i32 = 127;

/** enum netdev_xdp_rx_metadata */
pub const NETDEV_XDP_RX_METADATA_TIMESTAMP: i32 = 1;
pub const NETDEV_XDP_RX_METADATA_HASH: i32 = 2;
pub const NETDEV_XDP_RX_METADATA_VLAN_TAG: i32 = 4;

/** enum netdev_xsk_flags */
pub const NETDEV_XSK_FLAGS_TX_TIMESTAMP: i32 = 1;
pub const NETDEV_XSK_FLAGS_TX_CHECKSUM: i32 = 2;
pub const NETDEV_XSK_FLAGS_TX_LAUNCH_TIME_FIFO: i32 = 4;

pub const NETDEV_QUEUE_TYPE_RX: i32 = 0;
pub const NETDEV_QUEUE_TYPE_TX: i32 = 1;

pub const NETDEV_QSTATS_SCOPE_QUEUE: i32 = 1;

pub const NETDEV_NAPI_THREADED_DISABLED: i32 = 0;
pub const NETDEV_NAPI_THREADED_ENABLED: i32 = 1;
pub const NETDEV_NAPI_THREADED_BUSY_POLL: i32 = 2;

pub const NETDEV_A_DEV_IFINDEX: i32 = 1;
pub const NETDEV_A_DEV_PAD: i32 = 2;
pub const NETDEV_A_DEV_XDP_FEATURES: i32 = 3;
pub const NETDEV_A_DEV_XDP_ZC_MAX_SEGS: i32 = 4;
pub const NETDEV_A_DEV_XDP_RX_METADATA_FEATURES: i32 = 5;
pub const NETDEV_A_DEV_XSK_FEATURES: i32 = 6;
pub const __NETDEV_A_DEV_MAX: i32 = 7;
pub const NETDEV_A_DEV_MAX: i32 = __NETDEV_A_DEV_MAX - 1;

pub const NETDEV_A_IO_URING_PROVIDER_INFO_RX_BUF_LEN: i32 = 1;
pub const __NETDEV_A_IO_URING_PROVIDER_INFO_MAX: i32 = 2;
pub const NETDEV_A_IO_URING_PROVIDER_INFO_MAX: i32 = __NETDEV_A_IO_URING_PROVIDER_INFO_MAX - 1;

pub const NETDEV_A_PAGE_POOL_ID: i32 = 1;
pub const NETDEV_A_PAGE_POOL_IFINDEX: i32 = 2;
pub const NETDEV_A_PAGE_POOL_NAPI_ID: i32 = 3;
pub const NETDEV_A_PAGE_POOL_INFLIGHT: i32 = 4;
pub const NETDEV_A_PAGE_POOL_INFLIGHT_MEM: i32 = 5;
pub const NETDEV_A_PAGE_POOL_DETACH_TIME: i32 = 6;
pub const NETDEV_A_PAGE_POOL_DMABUF: i32 = 7;
pub const NETDEV_A_PAGE_POOL_IO_URING: i32 = 8;
pub const __NETDEV_A_PAGE_POOL_MAX: i32 = 9;
pub const NETDEV_A_PAGE_POOL_MAX: i32 = __NETDEV_A_PAGE_POOL_MAX - 1;

pub const NETDEV_A_PAGE_POOL_STATS_INFO: i32 = 1;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_FAST: i32 = 8;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW: i32 = 9;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW_HIGH_ORDER: i32 = 10;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_EMPTY: i32 = 11;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_REFILL: i32 = 12;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_WAIVE: i32 = 13;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHED: i32 = 14;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHE_FULL: i32 = 15;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING: i32 = 16;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING_FULL: i32 = 17;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RELEASED_REFCNT: i32 = 18;
pub const __NETDEV_A_PAGE_POOL_STATS_MAX: i32 = 19;
pub const NETDEV_A_PAGE_POOL_STATS_MAX: i32 = __NETDEV_A_PAGE_POOL_STATS_MAX - 1;

pub const NETDEV_A_NAPI_IFINDEX: i32 = 1;
pub const NETDEV_A_NAPI_ID: i32 = 2;
pub const NETDEV_A_NAPI_IRQ: i32 = 3;
pub const NETDEV_A_NAPI_PID: i32 = 4;
pub const NETDEV_A_NAPI_DEFER_HARD_IRQS: i32 = 5;
pub const NETDEV_A_NAPI_GRO_FLUSH_TIMEOUT: i32 = 6;
pub const NETDEV_A_NAPI_IRQ_SUSPEND_TIMEOUT: i32 = 7;
pub const NETDEV_A_NAPI_THREADED: i32 = 8;
pub const __NETDEV_A_NAPI_MAX: i32 = 9;
pub const NETDEV_A_NAPI_MAX: i32 = __NETDEV_A_NAPI_MAX - 1;

pub const __NETDEV_A_XSK_INFO_MAX: i32 = 1;
pub const NETDEV_A_XSK_INFO_MAX: i32 = __NETDEV_A_XSK_INFO_MAX - 1;

pub const NETDEV_A_QUEUE_ID: i32 = 1;
pub const NETDEV_A_QUEUE_IFINDEX: i32 = 2;
pub const NETDEV_A_QUEUE_TYPE: i32 = 3;
pub const NETDEV_A_QUEUE_NAPI_ID: i32 = 4;
pub const NETDEV_A_QUEUE_DMABUF: i32 = 5;
pub const NETDEV_A_QUEUE_IO_URING: i32 = 6;
pub const NETDEV_A_QUEUE_XSK: i32 = 7;
pub const NETDEV_A_QUEUE_LEASE: i32 = 8;
pub const __NETDEV_A_QUEUE_MAX: i32 = 9;
pub const NETDEV_A_QUEUE_MAX: i32 = __NETDEV_A_QUEUE_MAX - 1;

pub const NETDEV_A_QSTATS_IFINDEX: i32 = 1;
pub const NETDEV_A_QSTATS_QUEUE_TYPE: i32 = 2;
pub const NETDEV_A_QSTATS_QUEUE_ID: i32 = 3;
pub const NETDEV_A_QSTATS_SCOPE: i32 = 4;
pub const NETDEV_A_QSTATS_RX_PACKETS: i32 = 8;
pub const NETDEV_A_QSTATS_RX_BYTES: i32 = 9;
pub const NETDEV_A_QSTATS_TX_PACKETS: i32 = 10;
pub const NETDEV_A_QSTATS_TX_BYTES: i32 = 11;
pub const NETDEV_A_QSTATS_RX_ALLOC_FAIL: i32 = 12;
pub const NETDEV_A_QSTATS_RX_HW_DROPS: i32 = 13;
pub const NETDEV_A_QSTATS_RX_HW_DROP_OVERRUNS: i32 = 14;
pub const NETDEV_A_QSTATS_RX_CSUM_COMPLETE: i32 = 15;
pub const NETDEV_A_QSTATS_RX_CSUM_UNNECESSARY: i32 = 16;
pub const NETDEV_A_QSTATS_RX_CSUM_NONE: i32 = 17;
pub const NETDEV_A_QSTATS_RX_CSUM_BAD: i32 = 18;
pub const NETDEV_A_QSTATS_RX_HW_GRO_PACKETS: i32 = 19;
pub const NETDEV_A_QSTATS_RX_HW_GRO_BYTES: i32 = 20;
pub const NETDEV_A_QSTATS_RX_HW_GRO_WIRE_PACKETS: i32 = 21;
pub const NETDEV_A_QSTATS_RX_HW_GRO_WIRE_BYTES: i32 = 22;
pub const NETDEV_A_QSTATS_RX_HW_DROP_RATELIMITS: i32 = 23;
pub const NETDEV_A_QSTATS_TX_HW_DROPS: i32 = 24;
pub const NETDEV_A_QSTATS_TX_HW_DROP_ERRORS: i32 = 25;
pub const NETDEV_A_QSTATS_TX_CSUM_NONE: i32 = 26;
pub const NETDEV_A_QSTATS_TX_NEEDS_CSUM: i32 = 27;
pub const NETDEV_A_QSTATS_TX_HW_GSO_PACKETS: i32 = 28;
pub const NETDEV_A_QSTATS_TX_HW_GSO_BYTES: i32 = 29;
pub const NETDEV_A_QSTATS_TX_HW_GSO_WIRE_PACKETS: i32 = 30;
pub const NETDEV_A_QSTATS_TX_HW_GSO_WIRE_BYTES: i32 = 31;
pub const NETDEV_A_QSTATS_TX_HW_DROP_RATELIMITS: i32 = 32;
pub const NETDEV_A_QSTATS_TX_STOP: i32 = 33;
pub const NETDEV_A_QSTATS_TX_WAKE: i32 = 34;
pub const __NETDEV_A_QSTATS_MAX: i32 = 35;
pub const NETDEV_A_QSTATS_MAX: i32 = __NETDEV_A_QSTATS_MAX - 1;

pub const NETDEV_A_LEASE_IFINDEX: i32 = 1;
pub const NETDEV_A_LEASE_QUEUE: i32 = 2;
pub const NETDEV_A_LEASE_NETNS_ID: i32 = 3;
pub const __NETDEV_A_LEASE_MAX: i32 = 4;
pub const NETDEV_A_LEASE_MAX: i32 = __NETDEV_A_LEASE_MAX - 1;

pub const NETDEV_A_DMABUF_IFINDEX: i32 = 1;
pub const NETDEV_A_DMABUF_QUEUES: i32 = 2;
pub const NETDEV_A_DMABUF_FD: i32 = 3;
pub const NETDEV_A_DMABUF_ID: i32 = 4;
pub const NETDEV_A_DMABUF_RX_PAGE_SIZE: i32 = 5;
pub const __NETDEV_A_DMABUF_MAX: i32 = 6;
pub const NETDEV_A_DMABUF_MAX: i32 = __NETDEV_A_DMABUF_MAX - 1;

pub const NETDEV_CMD_DEV_GET: i32 = 1;
pub const NETDEV_CMD_DEV_ADD_NTF: i32 = 2;
pub const NETDEV_CMD_DEV_DEL_NTF: i32 = 3;
pub const NETDEV_CMD_DEV_CHANGE_NTF: i32 = 4;
pub const NETDEV_CMD_PAGE_POOL_GET: i32 = 5;
pub const NETDEV_CMD_PAGE_POOL_ADD_NTF: i32 = 6;
pub const NETDEV_CMD_PAGE_POOL_DEL_NTF: i32 = 7;
pub const NETDEV_CMD_PAGE_POOL_CHANGE_NTF: i32 = 8;
pub const NETDEV_CMD_PAGE_POOL_STATS_GET: i32 = 9;
pub const NETDEV_CMD_QUEUE_GET: i32 = 10;
pub const NETDEV_CMD_NAPI_GET: i32 = 11;
pub const NETDEV_CMD_QSTATS_GET: i32 = 12;
pub const NETDEV_CMD_BIND_RX: i32 = 13;
pub const NETDEV_CMD_NAPI_SET: i32 = 14;
pub const NETDEV_CMD_BIND_TX: i32 = 15;
pub const NETDEV_CMD_QUEUE_CREATE: i32 = 16;
pub const __NETDEV_CMD_MAX: i32 = 17;
pub const NETDEV_CMD_MAX: i32 = __NETDEV_CMD_MAX - 1;

pub const NETDEV_MCGRP_MGMT: &str = "mgmt";
pub const NETDEV_MCGRP_PAGE_POOL: &str = "page-pool";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
