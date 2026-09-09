// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/sunrpc_cache.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Dependencies supplied by the surrounding kernel/Rust translation.

/* Common nested types */
pub static SUNRPC_IP_MAP_NL_POLICY: [nla_policy; SUNRPC_A_IP_MAP_EXPIRY as usize + 1] = {
    let mut p = [nla_policy { type_: 0 }; SUNRPC_A_IP_MAP_EXPIRY as usize + 1];
    p[SUNRPC_A_IP_MAP_SEQNO as usize] = nla_policy { type_: NLA_U64 };
    p[SUNRPC_A_IP_MAP_CLASS as usize] = nla_policy { type_: NLA_NUL_STRING };
    p[SUNRPC_A_IP_MAP_ADDR as usize] = nla_policy { type_: NLA_NUL_STRING };
    p[SUNRPC_A_IP_MAP_DOMAIN as usize] = nla_policy { type_: NLA_NUL_STRING };
    p[SUNRPC_A_IP_MAP_NEGATIVE as usize] = nla_policy { type_: NLA_FLAG };
    p[SUNRPC_A_IP_MAP_EXPIRY as usize] = nla_policy { type_: NLA_U64 };
    p
};

pub static SUNRPC_UNIX_GID_NL_POLICY: [nla_policy; SUNRPC_A_UNIX_GID_EXPIRY as usize + 1] = {
    let mut p = [nla_policy { type_: 0 }; SUNRPC_A_UNIX_GID_EXPIRY as usize + 1];
    p[SUNRPC_A_UNIX_GID_SEQNO as usize] = nla_policy { type_: NLA_U64 };
    p[SUNRPC_A_UNIX_GID_UID as usize] = nla_policy { type_: NLA_U32 };
    p[SUNRPC_A_UNIX_GID_GIDS as usize] = nla_policy { type_: NLA_U32 };
    p[SUNRPC_A_UNIX_GID_NEGATIVE as usize] = nla_policy { type_: NLA_FLAG };
    p[SUNRPC_A_UNIX_GID_EXPIRY as usize] = nla_policy { type_: NLA_U64 };
    p
};

/* SUNRPC_CMD_IP_MAP_SET_REQS - do */
static SUNRPC_IP_MAP_SET_REQS_NL_POLICY: [nla_policy; SUNRPC_A_IP_MAP_REQS_REQUESTS as usize + 1] = {
    let mut p = [nla_policy { type_: 0 }; SUNRPC_A_IP_MAP_REQS_REQUESTS as usize + 1];
    p[SUNRPC_A_IP_MAP_REQS_REQUESTS as usize] = nla_policy::nested(&SUNRPC_IP_MAP_NL_POLICY);
    p
};

/* SUNRPC_CMD_UNIX_GID_SET_REQS - do */
static SUNRPC_UNIX_GID_SET_REQS_NL_POLICY: [nla_policy; SUNRPC_A_UNIX_GID_REQS_REQUESTS as usize + 1] = {
    let mut p = [nla_policy { type_: 0 }; SUNRPC_A_UNIX_GID_REQS_REQUESTS as usize + 1];
    p[SUNRPC_A_UNIX_GID_REQS_REQUESTS as usize] = nla_policy::nested(&SUNRPC_UNIX_GID_NL_POLICY);
    p
};

/* SUNRPC_CMD_CACHE_FLUSH - do */
static SUNRPC_CACHE_FLUSH_NL_POLICY: [nla_policy; SUNRPC_A_CACHE_FLUSH_MASK as usize + 1] = {
    let mut p = [nla_policy { type_: 0 }; SUNRPC_A_CACHE_FLUSH_MASK as usize + 1];
    p[SUNRPC_A_CACHE_FLUSH_MASK as usize] = nla_policy::mask(NLA_U32, 0x3);
    p
};

/* Ops table for sunrpc */
static SUNRPC_NL_OPS: [genl_split_ops; 5] = [
    genl_split_ops { cmd: SUNRPC_CMD_IP_MAP_GET_REQS, dumpit: Some(sunrpc_nl_ip_map_get_reqs_dumpit), flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DUMP, ..genl_split_ops::default() },
    genl_split_ops { cmd: SUNRPC_CMD_IP_MAP_SET_REQS, doit: Some(sunrpc_nl_ip_map_set_reqs_doit), policy: SUNRPC_IP_MAP_SET_REQS_NL_POLICY.as_ptr(), maxattr: SUNRPC_A_IP_MAP_REQS_REQUESTS, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO, ..genl_split_ops::default() },
    genl_split_ops { cmd: SUNRPC_CMD_UNIX_GID_GET_REQS, dumpit: Some(sunrpc_nl_unix_gid_get_reqs_dumpit), flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DUMP, ..genl_split_ops::default() },
    genl_split_ops { cmd: SUNRPC_CMD_UNIX_GID_SET_REQS, doit: Some(sunrpc_nl_unix_gid_set_reqs_doit), policy: SUNRPC_UNIX_GID_SET_REQS_NL_POLICY.as_ptr(), maxattr: SUNRPC_A_UNIX_GID_REQS_REQUESTS, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO, ..genl_split_ops::default() },
    genl_split_ops { cmd: SUNRPC_CMD_CACHE_FLUSH, doit: Some(sunrpc_nl_cache_flush_doit), policy: SUNRPC_CACHE_FLUSH_NL_POLICY.as_ptr(), maxattr: SUNRPC_A_CACHE_FLUSH_MASK, flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO, ..genl_split_ops::default() },
];

static SUNRPC_NL_MCGRPS: [genl_multicast_group; 2] = [
    genl_multicast_group { name: "none" },
    genl_multicast_group { name: "exportd" },
];

pub static mut sunrpc_nl_family: genl_family = genl_family {
    name: SUNRPC_FAMILY_NAME,
    version: SUNRPC_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: SUNRPC_NL_OPS.as_ptr(),
    n_split_ops: SUNRPC_NL_OPS.len(),
    mcgrps: SUNRPC_NL_MCGRPS.as_ptr(),
    n_mcgrps: SUNRPC_NL_MCGRPS.len(),
    ..genl_family::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
