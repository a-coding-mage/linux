/*
 * net/tipc/netlink.c: TIPC configuration handling
 *
 * Rust translation of the original implementation source.  Declarations
 * supplied by the included kernel headers remain external dependencies.
 */

// Dependencies supplied by: core.h, socket.h, name_table.h, bearer.h, link.h,
// node.h, net.h, udp_media.h, and <net/genetlink.h>.

pub static mut tipc_nl_policy: [nla_policy; TIPC_NLA_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
];

pub static mut tipc_nl_name_table_policy: [nla_policy; TIPC_NLA_NAME_TABLE_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_NESTED },
];

pub static mut tipc_nl_monitor_policy: [nla_policy; TIPC_NLA_MON_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

pub static mut tipc_nl_sock_policy: [nla_policy; TIPC_NLA_SOCK_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_FLAG },
];

pub static mut tipc_nl_net_policy: [nla_policy; TIPC_NLA_NET_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_U64 },
    nla_policy { type_: NLA_FLAG },
];

pub static mut tipc_nl_link_policy: [nla_policy; TIPC_NLA_LINK_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_NUL_STRING, len: TIPC_MAX_LINK_NAME },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

pub static mut tipc_nl_node_policy: [nla_policy; TIPC_NLA_NODE_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_BINARY, len: TIPC_NODEID_LEN },
    nla_policy { type_: NLA_BINARY, len: TIPC_AEAD_KEY_SIZE_MAX },
    nla_policy { type_: NLA_FLAG },
    nla_policy { type_: NLA_U32 },
];

/* Properties valid for media, bearer and link */
pub static tipc_nl_mtu_range: netlink_range_validation = netlink_range_validation {
    max: U16_MAX,
};

pub static mut tipc_nl_prop_policy: [nla_policy; TIPC_NLA_PROP_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    NLA_POLICY_FULL_RANGE!(NLA_U32, &tipc_nl_mtu_range),
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

pub static mut tipc_nl_bearer_policy: [nla_policy; TIPC_NLA_BEARER_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_NUL_STRING, len: TIPC_MAX_BEARER_NAME },
    nla_policy { type_: NLA_NESTED },
    nla_policy { type_: NLA_U32 },
];

pub static mut tipc_nl_media_policy: [nla_policy; TIPC_NLA_MEDIA_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_STRING },
    nla_policy { type_: NLA_NESTED },
];

pub static mut tipc_nl_udp_policy: [nla_policy; TIPC_NLA_UDP_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_BINARY, len: core::mem::size_of::<sockaddr_storage>() },
    nla_policy { type_: NLA_BINARY, len: core::mem::size_of::<sockaddr_storage>() },
];

/* Users of the legacy API (tipc-config) can't handle that we add operations,
 * so we have a separate genl handling for the new API.
 */
pub static tipc_genl_v2_ops: [genl_ops; 24] = [
    genl_ops { cmd: TIPC_NL_BEARER_DISABLE, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_bearer_disable) },
    genl_ops { cmd: TIPC_NL_BEARER_ENABLE, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_bearer_enable) },
    genl_ops { cmd: TIPC_NL_BEARER_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: Some(tipc_nl_bearer_get), dumpit: Some(tipc_nl_bearer_dump) },
    genl_ops { cmd: TIPC_NL_BEARER_ADD, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_bearer_add) },
    genl_ops { cmd: TIPC_NL_BEARER_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_bearer_set) },
    genl_ops { cmd: TIPC_NL_SOCK_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, start: Some(tipc_dump_start), dumpit: Some(tipc_nl_sk_dump), done: Some(tipc_dump_done) },
    genl_ops { cmd: TIPC_NL_PUBL_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP_STRICT, dumpit: Some(tipc_nl_publ_dump) },
    genl_ops { cmd: TIPC_NL_LINK_GET, validate: GENL_DONT_VALIDATE_STRICT, doit: Some(tipc_nl_node_get_link), dumpit: Some(tipc_nl_node_dump_link) },
    genl_ops { cmd: TIPC_NL_LINK_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_node_set_link) },
    genl_ops { cmd: TIPC_NL_LINK_RESET_STATS, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_node_reset_link_stats) },
    genl_ops { cmd: TIPC_NL_MEDIA_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: Some(tipc_nl_media_get), dumpit: Some(tipc_nl_media_dump) },
    genl_ops { cmd: TIPC_NL_MEDIA_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_media_set) },
    genl_ops { cmd: TIPC_NL_NODE_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, dumpit: Some(tipc_nl_node_dump) },
    genl_ops { cmd: TIPC_NL_NET_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, dumpit: Some(tipc_nl_net_dump) },
    genl_ops { cmd: TIPC_NL_NET_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_net_set) },
    genl_ops { cmd: TIPC_NL_NAME_TABLE_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, dumpit: Some(tipc_nl_name_table_dump) },
    genl_ops { cmd: TIPC_NL_MON_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_node_set_monitor) },
    genl_ops { cmd: TIPC_NL_MON_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: Some(tipc_nl_node_get_monitor), dumpit: Some(tipc_nl_node_dump_monitor) },
    genl_ops { cmd: TIPC_NL_MON_PEER_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP_STRICT, dumpit: Some(tipc_nl_node_dump_monitor_peer) },
    genl_ops { cmd: TIPC_NL_PEER_REMOVE, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_peer_rm) },
    #[cfg(feature = "CONFIG_TIPC_MEDIA_UDP")]
    genl_ops { cmd: TIPC_NL_UDP_GET_REMOTEIP, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP_STRICT, dumpit: Some(tipc_udp_nl_dump_remoteip) },
    #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
    genl_ops { cmd: TIPC_NL_KEY_SET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_node_set_key) },
    #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
    genl_ops { cmd: TIPC_NL_KEY_FLUSH, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: GENL_UNS_ADMIN_PERM, doit: Some(tipc_nl_node_flush_key) },
    genl_ops { cmd: TIPC_NL_ADDR_LEGACY_GET, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, doit: Some(tipc_nl_net_addr_legacy_get) },
];

pub static mut tipc_genl_family: genl_family = genl_family {
    name: TIPC_GENL_V2_NAME,
    version: TIPC_GENL_V2_VERSION,
    hdrsize: 0,
    maxattr: TIPC_NLA_MAX,
    policy: &tipc_nl_policy,
    netnsok: true,
    module: THIS_MODULE,
    ops: &tipc_genl_v2_ops,
    n_ops: tipc_genl_v2_ops.len(),
    resv_start_op: TIPC_NL_ADDR_LEGACY_GET + 1,
};

pub unsafe fn tipc_netlink_start() -> i32 {
    let res = genl_register_family(&raw mut tipc_genl_family);
    if res != 0 {
        pr_err!("Failed to register netlink interface\n");
        return res;
    }
    0
}

pub unsafe fn tipc_netlink_stop() {
    genl_unregister_family(&raw mut tipc_genl_family);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
