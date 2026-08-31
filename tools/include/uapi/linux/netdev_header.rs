/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/netdev.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const NETDEV_FAMILY_NAME: &str = "netdev";
pub const NETDEV_FAMILY_VERSION: u32 = 1;

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
pub const NETDEV_XDP_ACT_BASIC: u32 = 1;
pub const NETDEV_XDP_ACT_REDIRECT: u32 = 2;
pub const NETDEV_XDP_ACT_NDO_XMIT: u32 = 4;
pub const NETDEV_XDP_ACT_XSK_ZEROCOPY: u32 = 8;
pub const NETDEV_XDP_ACT_HW_OFFLOAD: u32 = 16;
pub const NETDEV_XDP_ACT_RX_SG: u32 = 32;
pub const NETDEV_XDP_ACT_NDO_XMIT_SG: u32 = 64;

/* private: */
pub const NETDEV_XDP_ACT_MASK: u32 = 127;

/**
 * enum netdev_xdp_rx_metadata
 * @NETDEV_XDP_RX_METADATA_TIMESTAMP: Device is capable of exposing receive HW
 *   timestamp via bpf_xdp_metadata_rx_timestamp().
 * @NETDEV_XDP_RX_METADATA_HASH: Device is capable of exposing receive packet
 *   hash via bpf_xdp_metadata_rx_hash().
 * @NETDEV_XDP_RX_METADATA_VLAN_TAG: Device is capable of exposing receive
 *   packet VLAN tag via bpf_xdp_metadata_rx_vlan_tag().
 */
pub const NETDEV_XDP_RX_METADATA_TIMESTAMP: u32 = 1;
pub const NETDEV_XDP_RX_METADATA_HASH: u32 = 2;
pub const NETDEV_XDP_RX_METADATA_VLAN_TAG: u32 = 4;

/**
 * enum netdev_xsk_flags
 * @NETDEV_XSK_FLAGS_TX_TIMESTAMP: HW timestamping egress packets is supported
 *   by the driver.
 * @NETDEV_XSK_FLAGS_TX_CHECKSUM: L3 checksum HW offload is supported by the
 *   driver.
 * @NETDEV_XSK_FLAGS_TX_LAUNCH_TIME_FIFO: Launch time HW offload is supported
 *   by the driver.
 */
pub const NETDEV_XSK_FLAGS_TX_TIMESTAMP: u32 = 1;
pub const NETDEV_XSK_FLAGS_TX_CHECKSUM: u32 = 2;
pub const NETDEV_XSK_FLAGS_TX_LAUNCH_TIME_FIFO: u32 = 4;

pub const NETDEV_QUEUE_TYPE_RX: u32 = 0;
pub const NETDEV_QUEUE_TYPE_TX: u32 = 1;

pub const NETDEV_QSTATS_SCOPE_QUEUE: u32 = 1;

pub const NETDEV_NAPI_THREADED_DISABLED: u32 = 0;
pub const NETDEV_NAPI_THREADED_ENABLED: u32 = 1;
pub const NETDEV_NAPI_THREADED_BUSY_POLL: u32 = 2;

pub const NETDEV_A_DEV_IFINDEX: u32 = 1;
pub const NETDEV_A_DEV_PAD: u32 = 2;
pub const NETDEV_A_DEV_XDP_FEATURES: u32 = 3;
pub const NETDEV_A_DEV_XDP_ZC_MAX_SEGS: u32 = 4;
pub const NETDEV_A_DEV_XDP_RX_METADATA_FEATURES: u32 = 5;
pub const NETDEV_A_DEV_XSK_FEATURES: u32 = 6;

pub const __NETDEV_A_DEV_MAX: u32 = 7;
pub const NETDEV_A_DEV_MAX: u32 = __NETDEV_A_DEV_MAX - 1;

pub const NETDEV_A_IO_URING_PROVIDER_INFO_RX_BUF_LEN: u32 = 1;

pub const __NETDEV_A_IO_URING_PROVIDER_INFO_MAX: u32 = 2;
pub const NETDEV_A_IO_URING_PROVIDER_INFO_MAX: u32 =
    __NETDEV_A_IO_URING_PROVIDER_INFO_MAX - 1;

pub const NETDEV_A_PAGE_POOL_ID: u32 = 1;
pub const NETDEV_A_PAGE_POOL_IFINDEX: u32 = 2;
pub const NETDEV_A_PAGE_POOL_NAPI_ID: u32 = 3;
pub const NETDEV_A_PAGE_POOL_INFLIGHT: u32 = 4;
pub const NETDEV_A_PAGE_POOL_INFLIGHT_MEM: u32 = 5;
pub const NETDEV_A_PAGE_POOL_DETACH_TIME: u32 = 6;
pub const NETDEV_A_PAGE_POOL_DMABUF: u32 = 7;
pub const NETDEV_A_PAGE_POOL_IO_URING: u32 = 8;

pub const __NETDEV_A_PAGE_POOL_MAX: u32 = 9;
pub const NETDEV_A_PAGE_POOL_MAX: u32 = __NETDEV_A_PAGE_POOL_MAX - 1;

pub const NETDEV_A_PAGE_POOL_STATS_INFO: u32 = 1;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_FAST: u32 = 8;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW: u32 = 9;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_SLOW_HIGH_ORDER: u32 = 10;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_EMPTY: u32 = 11;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_REFILL: u32 = 12;
pub const NETDEV_A_PAGE_POOL_STATS_ALLOC_WAIVE: u32 = 13;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHED: u32 = 14;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_CACHE_FULL: u32 = 15;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING: u32 = 16;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RING_FULL: u32 = 17;
pub const NETDEV_A_PAGE_POOL_STATS_RECYCLE_RELEASED_REFCNT: u32 = 18;

pub const __NETDEV_A_PAGE_POOL_STATS_MAX: u32 = 19;
pub const NETDEV_A_PAGE_POOL_STATS_MAX: u32 = __NETDEV_A_PAGE_POOL_STATS_MAX - 1;

pub const NETDEV_A_NAPI_IFINDEX: u32 = 1;
pub const NETDEV_A_NAPI_ID: u32 = 2;
pub const NETDEV_A_NAPI_IRQ: u32 = 3;
pub const NETDEV_A_NAPI_PID: u32 = 4;
pub const NETDEV_A_NAPI_DEFER_HARD_IRQS: u32 = 5;
pub const NETDEV_A_NAPI_GRO_FLUSH_TIMEOUT: u32 = 6;
pub const NETDEV_A_NAPI_IRQ_SUSPEND_TIMEOUT: u32 = 7;
pub const NETDEV_A_NAPI_THREADED: u32 = 8;

pub const __NETDEV_A_NAPI_MAX: u32 = 9;
pub const NETDEV_A_NAPI_MAX: u32 = __NETDEV_A_NAPI_MAX - 1;

pub const __NETDEV_A_XSK_INFO_MAX: u32 = 0;
pub const NETDEV_A_XSK_INFO_MAX: u32 = __NETDEV_A_XSK_INFO_MAX - 1;

pub const NETDEV_A_QUEUE_ID: u32 = 1;
pub const NETDEV_A_QUEUE_IFINDEX: u32 = 2;
pub const NETDEV_A_QUEUE_TYPE: u32 = 3;
pub const NETDEV_A_QUEUE_NAPI_ID: u32 = 4;
pub const NETDEV_A_QUEUE_DMABUF: u32 = 5;
pub const NETDEV_A_QUEUE_IO_URING: u32 = 6;
pub const NETDEV_A_QUEUE_XSK: u32 = 7;
pub const NETDEV_A_QUEUE_LEASE: u32 = 8;

pub const __NETDEV_A_QUEUE_MAX: u32 = 9;
pub const NETDEV_A_QUEUE_MAX: u32 = __NETDEV_A_QUEUE_MAX - 1;

pub const NETDEV_A_QSTATS_IFINDEX: u32 = 1;
pub const NETDEV_A_QSTATS_QUEUE_TYPE: u32 = 2;
pub const NETDEV_A_QSTATS_QUEUE_ID: u32 = 3;
pub const NETDEV_A_QSTATS_SCOPE: u32 = 4;
pub const NETDEV_A_QSTATS_RX_PACKETS: u32 = 8;
pub const NETDEV_A_QSTATS_RX_BYTES: u32 = 9;
pub const NETDEV_A_QSTATS_TX_PACKETS: u32 = 10;
pub const NETDEV_A_QSTATS_TX_BYTES: u32 = 11;
pub const NETDEV_A_QSTATS_RX_ALLOC_FAIL: u32 = 12;
pub const NETDEV_A_QSTATS_RX_HW_DROPS: u32 = 13;
pub const NETDEV_A_QSTATS_RX_HW_DROP_OVERRUNS: u32 = 14;
pub const NETDEV_A_QSTATS_RX_CSUM_COMPLETE: u32 = 15;
pub const NETDEV_A_QSTATS_RX_CSUM_UNNECESSARY: u32 = 16;
pub const NETDEV_A_QSTATS_RX_CSUM_NONE: u32 = 17;
pub const NETDEV_A_QSTATS_RX_CSUM_BAD: u32 = 18;
pub const NETDEV_A_QSTATS_RX_HW_GRO_PACKETS: u32 = 19;
pub const NETDEV_A_QSTATS_RX_HW_GRO_BYTES: u32 = 20;
pub const NETDEV_A_QSTATS_RX_HW_GRO_WIRE_PACKETS: u32 = 21;
pub const NETDEV_A_QSTATS_RX_HW_GRO_WIRE_BYTES: u32 = 22;
pub const NETDEV_A_QSTATS_RX_HW_DROP_RATELIMITS: u32 = 23;
pub const NETDEV_A_QSTATS_TX_HW_DROPS: u32 = 24;
pub const NETDEV_A_QSTATS_TX_HW_DROP_ERRORS: u32 = 25;
pub const NETDEV_A_QSTATS_TX_CSUM_NONE: u32 = 26;
pub const NETDEV_A_QSTATS_TX_NEEDS_CSUM: u32 = 27;
pub const NETDEV_A_QSTATS_TX_HW_GSO_PACKETS: u32 = 28;
pub const NETDEV_A_QSTATS_TX_HW_GSO_BYTES: u32 = 29;
pub const NETDEV_A_QSTATS_TX_HW_GSO_WIRE_PACKETS: u32 = 30;
pub const NETDEV_A_QSTATS_TX_HW_GSO_WIRE_BYTES: u32 = 31;
pub const NETDEV_A_QSTATS_TX_HW_DROP_RATELIMITS: u32 = 32;
pub const NETDEV_A_QSTATS_TX_STOP: u32 = 33;
pub const NETDEV_A_QSTATS_TX_WAKE: u32 = 34;

pub const __NETDEV_A_QSTATS_MAX: u32 = 35;
pub const NETDEV_A_QSTATS_MAX: u32 = __NETDEV_A_QSTATS_MAX - 1;

pub const NETDEV_A_LEASE_IFINDEX: u32 = 1;
pub const NETDEV_A_LEASE_QUEUE: u32 = 2;
pub const NETDEV_A_LEASE_NETNS_ID: u32 = 3;

pub const __NETDEV_A_LEASE_MAX: u32 = 4;
pub const NETDEV_A_LEASE_MAX: u32 = __NETDEV_A_LEASE_MAX - 1;

pub const NETDEV_A_DMABUF_IFINDEX: u32 = 1;
pub const NETDEV_A_DMABUF_QUEUES: u32 = 2;
pub const NETDEV_A_DMABUF_FD: u32 = 3;
pub const NETDEV_A_DMABUF_ID: u32 = 4;
pub const NETDEV_A_DMABUF_RX_PAGE_SIZE: u32 = 5;

pub const __NETDEV_A_DMABUF_MAX: u32 = 6;
pub const NETDEV_A_DMABUF_MAX: u32 = __NETDEV_A_DMABUF_MAX - 1;

pub const NETDEV_CMD_DEV_GET: u32 = 1;
pub const NETDEV_CMD_DEV_ADD_NTF: u32 = 2;
pub const NETDEV_CMD_DEV_DEL_NTF: u32 = 3;
pub const NETDEV_CMD_DEV_CHANGE_NTF: u32 = 4;
pub const NETDEV_CMD_PAGE_POOL_GET: u32 = 5;
pub const NETDEV_CMD_PAGE_POOL_ADD_NTF: u32 = 6;
pub const NETDEV_CMD_PAGE_POOL_DEL_NTF: u32 = 7;
pub const NETDEV_CMD_PAGE_POOL_CHANGE_NTF: u32 = 8;
pub const NETDEV_CMD_PAGE_POOL_STATS_GET: u32 = 9;
pub const NETDEV_CMD_QUEUE_GET: u32 = 10;
pub const NETDEV_CMD_NAPI_GET: u32 = 11;
pub const NETDEV_CMD_QSTATS_GET: u32 = 12;
pub const NETDEV_CMD_BIND_RX: u32 = 13;
pub const NETDEV_CMD_NAPI_SET: u32 = 14;
pub const NETDEV_CMD_BIND_TX: u32 = 15;
pub const NETDEV_CMD_QUEUE_CREATE: u32 = 16;

pub const __NETDEV_CMD_MAX: u32 = 17;
pub const NETDEV_CMD_MAX: u32 = __NETDEV_CMD_MAX - 1;

pub const NETDEV_MCGRP_MGMT: &str = "mgmt";
pub const NETDEV_MCGRP_PAGE_POOL: &str = "page-pool";
