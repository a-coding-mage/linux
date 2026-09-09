// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/nfsd.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the C includes and local header.

/* Common nested types */
pub static nfsd_auth_flavor_nl_policy: [nla_policy; NFSD_A_AUTH_FLAVOR_FLAGS as usize + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_POLICY_MASK(NLA_U32, 0x3ffff) },
];

pub static nfsd_expkey_nl_policy: [nla_policy; NFSD_A_EXPKEY_PATH as usize + 1] = [
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_NUL_STRING },
    nla_policy { type_: NLA_U8 },
    nla_policy { type_: NLA_BINARY },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_NUL_STRING },
];

pub static nfsd_fslocation_nl_policy: [nla_policy; NFSD_A_FSLOCATION_PATH as usize + 1] = [
    nla_policy { type_: NLA_NUL_STRING },
    nla_policy { type_: NLA_NUL_STRING },
];

pub static nfsd_fslocations_nl_policy: [nla_policy; NFSD_A_FSLOCATIONS_LOCATION as usize + 1] = [
    NLA_POLICY_NESTED!(nfsd_fslocation_nl_policy),
];

pub static nfsd_sock_nl_policy: [nla_policy; NFSD_A_SOCK_TRANSPORT_NAME as usize + 1] = [
    nla_policy { type_: NLA_BINARY },
    nla_policy { type_: NLA_NUL_STRING },
];

pub static nfsd_svc_export_nl_policy: [nla_policy; NFSD_A_SVC_EXPORT_FSID as usize + 1] = [
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_NUL_STRING },
    nla_policy { type_: NLA_NUL_STRING },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    NLA_POLICY_NESTED!(nfsd_fslocations_nl_policy),
    nla_policy { type_: NLA_BINARY },
    NLA_POLICY_NESTED!(nfsd_auth_flavor_nl_policy),
    nla_policy { type_: NLA_POLICY_MASK(NLA_U32, 0x7) },
    nla_policy { type_: NLA_POLICY_MASK(NLA_U32, 0x3ffff) },
    nla_policy { type_: NLA_S32 },
];

pub static nfsd_version_nl_policy: [nla_policy; NFSD_A_VERSION_ENABLED as usize + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_FLAG },
];

/* NFSD_CMD_THREADS_SET - do */
static nfsd_threads_set_nl_policy: [nla_policy; NFSD_A_SERVER_FH_KEY as usize + 1] = [
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_NUL_STRING },
    nla_policy { type_: NLA_U32 }, NLA_POLICY_EXACT_LEN!(16),
];

/* NFSD_CMD_VERSION_SET - do */
static nfsd_version_set_nl_policy: [nla_policy; NFSD_A_SERVER_PROTO_VERSION as usize + 1] = [
    NLA_POLICY_NESTED!(nfsd_version_nl_policy),
];

/* NFSD_CMD_LISTENER_SET - do */
static nfsd_listener_set_nl_policy: [nla_policy; NFSD_A_SERVER_SOCK_ADDR as usize + 1] = [
    NLA_POLICY_NESTED!(nfsd_sock_nl_policy),
];

/* NFSD_CMD_POOL_MODE_SET - do */
static nfsd_pool_mode_set_nl_policy: [nla_policy; NFSD_A_POOL_MODE_MODE as usize + 1] = [
    nla_policy { type_: NLA_NUL_STRING },
];

/* NFSD_CMD_SVC_EXPORT_SET_REQS - do */
static nfsd_svc_export_set_reqs_nl_policy: [nla_policy; NFSD_A_SVC_EXPORT_REQS_REQUESTS as usize + 1] = [
    NLA_POLICY_NESTED!(nfsd_svc_export_nl_policy),
];

/* NFSD_CMD_EXPKEY_SET_REQS - do */
static nfsd_expkey_set_reqs_nl_policy: [nla_policy; NFSD_A_EXPKEY_REQS_REQUESTS as usize + 1] = [
    NLA_POLICY_NESTED!(nfsd_expkey_nl_policy),
];

/* NFSD_CMD_CACHE_FLUSH - do */
static nfsd_cache_flush_nl_policy: [nla_policy; NFSD_A_CACHE_FLUSH_MASK as usize + 1] = [
    nla_policy { type_: NLA_POLICY_MASK(NLA_U32, 0x3) },
];

/* NFSD_CMD_UNLOCK_IP - do */
static nfsd_unlock_ip_nl_policy: [nla_policy; NFSD_A_UNLOCK_IP_ADDRESS as usize + 1] = [
    NLA_POLICY_MIN_LEN!(16),
];

/* NFSD_CMD_UNLOCK_FILESYSTEM - do */
static nfsd_unlock_filesystem_nl_policy: [nla_policy; NFSD_A_UNLOCK_FILESYSTEM_PATH as usize + 1] = [
    nla_policy { type_: NLA_NUL_STRING },
];

/* NFSD_CMD_UNLOCK_EXPORT - do */
static nfsd_unlock_export_nl_policy: [nla_policy; NFSD_A_UNLOCK_EXPORT_PATH as usize + 1] = [
    nla_policy { type_: NLA_NUL_STRING },
];

/* Ops table for nfsd */
static nfsd_nl_ops: [genl_split_ops; 18] = [
    genl_split_ops { cmd: NFSD_CMD_RPC_STATUS_GET, dumpit: Some(nfsd_nl_rpc_status_get_dumpit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_THREADS_SET, doit: Some(nfsd_nl_threads_set_doit), policy: Some(&nfsd_threads_set_nl_policy), maxattr: NFSD_A_SERVER_FH_KEY, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_THREADS_GET, doit: Some(nfsd_nl_threads_get_doit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_VERSION_SET, doit: Some(nfsd_nl_version_set_doit), policy: Some(&nfsd_version_set_nl_policy), maxattr: NFSD_A_SERVER_PROTO_VERSION, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_VERSION_GET, doit: Some(nfsd_nl_version_get_doit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_LISTENER_SET, doit: Some(nfsd_nl_listener_set_doit), policy: Some(&nfsd_listener_set_nl_policy), maxattr: NFSD_A_SERVER_SOCK_ADDR, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_LISTENER_GET, doit: Some(nfsd_nl_listener_get_doit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_POOL_MODE_SET, doit: Some(nfsd_nl_pool_mode_set_doit), policy: Some(&nfsd_pool_mode_set_nl_policy), maxattr: NFSD_A_POOL_MODE_MODE, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_POOL_MODE_GET, doit: Some(nfsd_nl_pool_mode_get_doit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_SVC_EXPORT_GET_REQS, dumpit: Some(nfsd_nl_svc_export_get_reqs_dumpit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_SVC_EXPORT_SET_REQS, doit: Some(nfsd_nl_svc_export_set_reqs_doit), policy: Some(&nfsd_svc_export_set_reqs_nl_policy), maxattr: NFSD_A_SVC_EXPORT_REQS_REQUESTS, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_EXPKEY_GET_REQS, dumpit: Some(nfsd_nl_expkey_get_reqs_dumpit), ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_EXPKEY_SET_REQS, doit: Some(nfsd_nl_expkey_set_reqs_doit), policy: Some(&nfsd_expkey_set_reqs_nl_policy), maxattr: NFSD_A_EXPKEY_REQS_REQUESTS, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_CACHE_FLUSH, doit: Some(nfsd_nl_cache_flush_doit), policy: Some(&nfsd_cache_flush_nl_policy), maxattr: NFSD_A_CACHE_FLUSH_MASK, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_UNLOCK_IP, doit: Some(nfsd_nl_unlock_ip_doit), policy: Some(&nfsd_unlock_ip_nl_policy), maxattr: NFSD_A_UNLOCK_IP_ADDRESS, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_UNLOCK_FILESYSTEM, doit: Some(nfsd_nl_unlock_filesystem_doit), policy: Some(&nfsd_unlock_filesystem_nl_policy), maxattr: NFSD_A_UNLOCK_FILESYSTEM_PATH, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_UNLOCK_EXPORT, doit: Some(nfsd_nl_unlock_export_doit), policy: Some(&nfsd_unlock_export_nl_policy), maxattr: NFSD_A_UNLOCK_EXPORT_PATH, ..Default::default() },
    genl_split_ops { cmd: NFSD_CMD_SERVER_STATS_GET, dumpit: Some(nfsd_nl_server_stats_get_dumpit), ..Default::default() },
];

static nfsd_nl_mcgrps: [genl_multicast_group; 2] = [
    genl_multicast_group { name: "none" },
    genl_multicast_group { name: "exportd" },
];

static mut nfsd_nl_family: genl_family = genl_family {
    name: NFSD_FAMILY_NAME,
    version: NFSD_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: &nfsd_nl_ops,
    n_split_ops: nfsd_nl_ops.len(),
    mcgrps: &nfsd_nl_mcgrps,
    n_mcgrps: nfsd_nl_mcgrps.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
