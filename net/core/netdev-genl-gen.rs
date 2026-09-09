// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/netdev.yaml
// YNL-GEN kernel source
// To regenerate run: tools/net/ynl/ynl-regen.sh

// C dependencies supplied by the surrounding kernel translation unit.

/* Integer value ranges */
static NETDEV_A_PAGE_POOL_ID_RANGE: netlink_range_validation = netlink_range_validation {
    min: 1u64,
    max: U32_MAX,
};

static NETDEV_A_PAGE_POOL_IFINDEX_RANGE: netlink_range_validation = netlink_range_validation {
    min: 1u64,
    max: S32_MAX,
};

static NETDEV_A_NAPI_DEFER_HARD_IRQS_RANGE: netlink_range_validation = netlink_range_validation {
    max: S32_MAX,
};

static NETDEV_A_DMABUF_RX_PAGE_SIZE_RANGE: netlink_range_validation = netlink_range_validation {
    min: PAGE_SIZE,
    max: U32_MAX,
};

/* Common nested types */
const NETDEV_LEASE_NL_POLICY: [nla_policy; NETDEV_A_LEASE_NETNS_ID + 1] = [
    nla_policy_at!(NETDEV_A_LEASE_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_LEASE_QUEUE, NLA_POLICY_NESTED(netdev_queue_id_nl_policy)),
    nla_policy_at!(NETDEV_A_LEASE_NETNS_ID, NLA_POLICY_MIN(NLA_S32, 0)),
];

const NETDEV_PAGE_POOL_INFO_NL_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_IFINDEX + 1] = [
    nla_policy_at!(NETDEV_A_PAGE_POOL_ID, NLA_POLICY_FULL_RANGE(NLA_UINT, &NETDEV_A_PAGE_POOL_ID_RANGE)),
    nla_policy_at!(NETDEV_A_PAGE_POOL_IFINDEX, NLA_POLICY_FULL_RANGE(NLA_U32, &NETDEV_A_PAGE_POOL_IFINDEX_RANGE)),
];

const NETDEV_QUEUE_ID_NL_POLICY: [nla_policy; NETDEV_A_QUEUE_TYPE + 1] = [
    nla_policy_at!(NETDEV_A_QUEUE_ID, nla_policy { type_: NLA_U32 }),
    nla_policy_at!(NETDEV_A_QUEUE_TYPE, NLA_POLICY_MAX(NLA_U32, 1)),
];

/* NETDEV_CMD_DEV_GET - do */
static NETDEV_DEV_GET_NL_POLICY: [nla_policy; NETDEV_A_DEV_IFINDEX + 1] = [
    nla_policy_at!(NETDEV_A_DEV_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
];

/* NETDEV_CMD_PAGE_POOL_GET - do */
#[cfg(CONFIG_PAGE_POOL)]
static NETDEV_PAGE_POOL_GET_DO_NL_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_ID + 1] = [
    nla_policy_at!(NETDEV_A_PAGE_POOL_ID, NLA_POLICY_FULL_RANGE(NLA_UINT, &NETDEV_A_PAGE_POOL_ID_RANGE)),
];

/* NETDEV_CMD_PAGE_POOL_GET - dump */
#[cfg(CONFIG_PAGE_POOL)]
static NETDEV_PAGE_POOL_GET_DUMP_NL_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_IFINDEX + 1] = [
    nla_policy_at!(NETDEV_A_PAGE_POOL_IFINDEX, NLA_POLICY_FULL_RANGE(NLA_U32, &NETDEV_A_PAGE_POOL_IFINDEX_RANGE)),
];

/* NETDEV_CMD_PAGE_POOL_STATS_GET - do */
#[cfg(CONFIG_PAGE_POOL_STATS)]
static NETDEV_PAGE_POOL_STATS_GET_DO_NL_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_STATS_INFO + 1] = [
    nla_policy_at!(NETDEV_A_PAGE_POOL_STATS_INFO, NLA_POLICY_NESTED(NETDEV_PAGE_POOL_INFO_NL_POLICY)),
];

/* NETDEV_CMD_PAGE_POOL_STATS_GET - dump */
#[cfg(CONFIG_PAGE_POOL_STATS)]
static NETDEV_PAGE_POOL_STATS_GET_DUMP_NL_POLICY: [nla_policy; NETDEV_A_PAGE_POOL_STATS_INFO + 1] = [
    nla_policy_at!(NETDEV_A_PAGE_POOL_STATS_INFO, NLA_POLICY_NESTED(NETDEV_PAGE_POOL_INFO_NL_POLICY)),
];

/* NETDEV_CMD_QUEUE_GET - do */
static NETDEV_QUEUE_GET_DO_NL_POLICY: [nla_policy; NETDEV_A_QUEUE_TYPE + 1] = [
    nla_policy_at!(NETDEV_A_QUEUE_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_QUEUE_TYPE, NLA_POLICY_MAX(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_QUEUE_ID, nla_policy { type_: NLA_U32 }),
];

/* NETDEV_CMD_QUEUE_GET - dump */
static NETDEV_QUEUE_GET_DUMP_NL_POLICY: [nla_policy; NETDEV_A_QUEUE_IFINDEX + 1] = [
    nla_policy_at!(NETDEV_A_QUEUE_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
];

/* NETDEV_CMD_NAPI_GET - do */
static NETDEV_NAPI_GET_DO_NL_POLICY: [nla_policy; NETDEV_A_NAPI_ID + 1] = [
    nla_policy_at!(NETDEV_A_NAPI_ID, nla_policy { type_: NLA_U32 }),
];

/* NETDEV_CMD_NAPI_GET - dump */
static NETDEV_NAPI_GET_DUMP_NL_POLICY: [nla_policy; NETDEV_A_NAPI_IFINDEX + 1] = [
    nla_policy_at!(NETDEV_A_NAPI_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
];

/* NETDEV_CMD_QSTATS_GET - dump */
static NETDEV_QSTATS_GET_NL_POLICY: [nla_policy; NETDEV_A_QSTATS_SCOPE + 1] = [
    nla_policy_at!(NETDEV_A_QSTATS_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_QSTATS_SCOPE, NLA_POLICY_MASK(NLA_UINT, 0x1)),
];

/* NETDEV_CMD_BIND_RX - do */
static NETDEV_BIND_RX_NL_POLICY: [nla_policy; NETDEV_A_DMABUF_RX_PAGE_SIZE + 1] = [
    nla_policy_at!(NETDEV_A_DMABUF_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_DMABUF_FD, nla_policy { type_: NLA_U32 }),
    nla_policy_at!(NETDEV_A_DMABUF_QUEUES, NLA_POLICY_NESTED(NETDEV_QUEUE_ID_NL_POLICY)),
    nla_policy_at!(NETDEV_A_DMABUF_RX_PAGE_SIZE, NLA_POLICY_FULL_RANGE(NLA_U32, &NETDEV_A_DMABUF_RX_PAGE_SIZE_RANGE)),
];

/* NETDEV_CMD_NAPI_SET - do */
static NETDEV_NAPI_SET_NL_POLICY: [nla_policy; NETDEV_A_NAPI_THREADED + 1] = [
    nla_policy_at!(NETDEV_A_NAPI_ID, nla_policy { type_: NLA_U32 }),
    nla_policy_at!(NETDEV_A_NAPI_DEFER_HARD_IRQS, NLA_POLICY_FULL_RANGE(NLA_U32, &NETDEV_A_NAPI_DEFER_HARD_IRQS_RANGE)),
    nla_policy_at!(NETDEV_A_NAPI_GRO_FLUSH_TIMEOUT, nla_policy { type_: NLA_UINT }),
    nla_policy_at!(NETDEV_A_NAPI_IRQ_SUSPEND_TIMEOUT, nla_policy { type_: NLA_UINT }),
    nla_policy_at!(NETDEV_A_NAPI_THREADED, NLA_POLICY_MAX(NLA_U32, 2)),
];

/* NETDEV_CMD_BIND_TX - do */
static NETDEV_BIND_TX_NL_POLICY: [nla_policy; NETDEV_A_DMABUF_FD + 1] = [
    nla_policy_at!(NETDEV_A_DMABUF_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_DMABUF_FD, nla_policy { type_: NLA_U32 }),
];

/* NETDEV_CMD_QUEUE_CREATE - do */
static NETDEV_QUEUE_CREATE_NL_POLICY: [nla_policy; NETDEV_A_QUEUE_LEASE + 1] = [
    nla_policy_at!(NETDEV_A_QUEUE_IFINDEX, NLA_POLICY_MIN(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_QUEUE_TYPE, NLA_POLICY_MAX(NLA_U32, 1)),
    nla_policy_at!(NETDEV_A_QUEUE_LEASE, NLA_POLICY_NESTED(NETDEV_LEASE_NL_POLICY)),
];

/* Ops table for netdev */
static NETDEV_NL_OPS: [genl_split_ops; 14] = [
    genl_split_ops { cmd: NETDEV_CMD_DEV_GET, doit: Some(netdev_nl_dev_get_doit), policy: Some(NETDEV_DEV_GET_NL_POLICY), maxattr: NETDEV_A_DEV_IFINDEX, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_DEV_GET, dumpit: Some(netdev_nl_dev_get_dumpit), flags: GENL_CMD_CAP_DUMP },
    #[cfg(CONFIG_PAGE_POOL)]
    genl_split_ops { cmd: NETDEV_CMD_PAGE_POOL_GET, doit: Some(netdev_nl_page_pool_get_doit), policy: Some(NETDEV_PAGE_POOL_GET_DO_NL_POLICY), maxattr: NETDEV_A_PAGE_POOL_ID, flags: GENL_CMD_CAP_DO },
    #[cfg(CONFIG_PAGE_POOL)]
    genl_split_ops { cmd: NETDEV_CMD_PAGE_POOL_GET, dumpit: Some(netdev_nl_page_pool_get_dumpit), policy: Some(NETDEV_PAGE_POOL_GET_DUMP_NL_POLICY), maxattr: NETDEV_A_PAGE_POOL_IFINDEX, flags: GENL_CMD_CAP_DUMP },
    #[cfg(CONFIG_PAGE_POOL_STATS)]
    genl_split_ops { cmd: NETDEV_CMD_PAGE_POOL_STATS_GET, doit: Some(netdev_nl_page_pool_stats_get_doit), policy: Some(NETDEV_PAGE_POOL_STATS_GET_DO_NL_POLICY), maxattr: NETDEV_A_PAGE_POOL_STATS_INFO, flags: GENL_CMD_CAP_DO },
    #[cfg(CONFIG_PAGE_POOL_STATS)]
    genl_split_ops { cmd: NETDEV_CMD_PAGE_POOL_STATS_GET, dumpit: Some(netdev_nl_page_pool_stats_get_dumpit), policy: Some(NETDEV_PAGE_POOL_STATS_GET_DUMP_NL_POLICY), maxattr: NETDEV_A_PAGE_POOL_STATS_INFO, flags: GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: NETDEV_CMD_QUEUE_GET, doit: Some(netdev_nl_queue_get_doit), policy: Some(NETDEV_QUEUE_GET_DO_NL_POLICY), maxattr: NETDEV_A_QUEUE_TYPE, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_QUEUE_GET, dumpit: Some(netdev_nl_queue_get_dumpit), policy: Some(NETDEV_QUEUE_GET_DUMP_NL_POLICY), maxattr: NETDEV_A_QUEUE_IFINDEX, flags: GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: NETDEV_CMD_NAPI_GET, doit: Some(netdev_nl_napi_get_doit), policy: Some(NETDEV_NAPI_GET_DO_NL_POLICY), maxattr: NETDEV_A_NAPI_ID, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_NAPI_GET, dumpit: Some(netdev_nl_napi_get_dumpit), policy: Some(NETDEV_NAPI_GET_DUMP_NL_POLICY), maxattr: NETDEV_A_NAPI_IFINDEX, flags: GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: NETDEV_CMD_QSTATS_GET, dumpit: Some(netdev_nl_qstats_get_dumpit), policy: Some(NETDEV_QSTATS_GET_NL_POLICY), maxattr: NETDEV_A_QSTATS_SCOPE, flags: GENL_CMD_CAP_DUMP },
    genl_split_ops { cmd: NETDEV_CMD_BIND_RX, doit: Some(netdev_nl_bind_rx_doit), policy: Some(NETDEV_BIND_RX_NL_POLICY), maxattr: NETDEV_A_DMABUF_RX_PAGE_SIZE, flags: GENL_UNS_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_NAPI_SET, doit: Some(netdev_nl_napi_set_doit), policy: Some(NETDEV_NAPI_SET_NL_POLICY), maxattr: NETDEV_A_NAPI_THREADED, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_BIND_TX, doit: Some(netdev_nl_bind_tx_doit), policy: Some(NETDEV_BIND_TX_NL_POLICY), maxattr: NETDEV_A_DMABUF_FD, flags: GENL_CMD_CAP_DO },
    genl_split_ops { cmd: NETDEV_CMD_QUEUE_CREATE, doit: Some(netdev_nl_queue_create_doit), policy: Some(NETDEV_QUEUE_CREATE_NL_POLICY), maxattr: NETDEV_A_QUEUE_LEASE, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO },
];

static NETDEV_NL_MCGRPS: [genl_multicast_group; 2] = [
    genl_multicast_group { name: "mgmt" },
    genl_multicast_group { name: "page-pool" },
];

unsafe fn __netdev_nl_sock_priv_init(priv_: *mut core::ffi::c_void) {
    netdev_nl_sock_priv_init(priv_);
}

unsafe fn __netdev_nl_sock_priv_destroy(priv_: *mut core::ffi::c_void) {
    netdev_nl_sock_priv_destroy(priv_);
}

static mut NETDEV_NL_FAMILY: genl_family = genl_family {
    name: NETDEV_FAMILY_NAME,
    version: NETDEV_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: NETDEV_NL_OPS.as_ptr(),
    n_split_ops: NETDEV_NL_OPS.len(),
    mcgrps: NETDEV_NL_MCGRPS.as_ptr(),
    n_mcgrps: NETDEV_NL_MCGRPS.len(),
    sock_priv_size: core::mem::size_of::<netdev_nl_sock>(),
    sock_priv_init: Some(__netdev_nl_sock_priv_init),
    sock_priv_destroy: Some(__netdev_nl_sock_priv_destroy),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
