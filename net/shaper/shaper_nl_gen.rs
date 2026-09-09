// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/net_shaper.yaml
// YNL-GEN kernel source
// To regenerate run: tools/net/ynl/ynl-regen.sh

// C dependencies: <net/netlink.h>, <net/genetlink.h>, "shaper_nl_gen.h",
// and <uapi/linux/net-shaper.h>.

/* Integer value ranges */
static net_shaper_a_handle_id_range: netlink_range_validation = netlink_range_validation {
    max: NET_SHAPER_MAX_HANDLE_ID,
};

/* Common nested types */
#[no_mangle]
pub static mut net_shaper_handle_nl_policy: [nla_policy; NET_SHAPER_A_HANDLE_ID + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32, max: 3 },
    nla_policy { type_: NLA_U32, range: &net_shaper_a_handle_id_range },
];

#[no_mangle]
pub static mut net_shaper_leaf_info_nl_policy: [nla_policy; NET_SHAPER_A_WEIGHT + 1] = [
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

/* NET_SHAPER_CMD_GET - do */
static mut net_shaper_get_do_nl_policy: [nla_policy; NET_SHAPER_A_IFINDEX + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
];

/* NET_SHAPER_CMD_GET - dump */
static mut net_shaper_get_dump_nl_policy: [nla_policy; NET_SHAPER_A_IFINDEX + 1] = [
    nla_policy { type_: NLA_U32 },
];

/* NET_SHAPER_CMD_SET - do */
static mut net_shaper_set_nl_policy: [nla_policy; NET_SHAPER_A_IFINDEX + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
    nla_policy { type_: NLA_U32, max: 1 },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

/* NET_SHAPER_CMD_DELETE - do */
static mut net_shaper_delete_nl_policy: [nla_policy; NET_SHAPER_A_IFINDEX + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
];

/* NET_SHAPER_CMD_GROUP - do */
static mut net_shaper_group_nl_policy: [nla_policy; NET_SHAPER_A_LEAVES + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_handle_nl_policy.as_ptr() },
    nla_policy { type_: NLA_U32, max: 1 },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_UINT },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_NESTED, nested_policy: net_shaper_leaf_info_nl_policy.as_ptr() },
];

/* NET_SHAPER_CMD_CAP_GET - do */
static mut net_shaper_cap_get_do_nl_policy: [nla_policy; NET_SHAPER_A_CAPS_SCOPE + 1] = [
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32, max: 3 },
];

/* NET_SHAPER_CMD_CAP_GET - dump */
static mut net_shaper_cap_get_dump_nl_policy: [nla_policy; NET_SHAPER_A_CAPS_IFINDEX + 1] = [
    nla_policy { type_: NLA_U32 },
];

/* Ops table for net_shaper */
static net_shaper_nl_ops: [genl_split_ops; 7] = [
    genl_split_ops {
        cmd: NET_SHAPER_CMD_GET,
        pre_doit: Some(net_shaper_nl_pre_doit),
        doit: Some(net_shaper_nl_get_doit),
        post_doit: Some(net_shaper_nl_post_doit),
        policy: net_shaper_get_do_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_IFINDEX,
        flags: GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_GET,
        start: Some(net_shaper_nl_pre_dumpit),
        dumpit: Some(net_shaper_nl_get_dumpit),
        done: Some(net_shaper_nl_post_dumpit),
        policy: net_shaper_get_dump_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_IFINDEX,
        flags: GENL_CMD_CAP_DUMP,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_SET,
        pre_doit: Some(net_shaper_nl_pre_doit_write),
        doit: Some(net_shaper_nl_set_doit),
        post_doit: Some(net_shaper_nl_post_doit_write),
        policy: net_shaper_set_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_IFINDEX,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_DELETE,
        pre_doit: Some(net_shaper_nl_pre_doit_write),
        doit: Some(net_shaper_nl_delete_doit),
        post_doit: Some(net_shaper_nl_post_doit_write),
        policy: net_shaper_delete_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_IFINDEX,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_GROUP,
        pre_doit: Some(net_shaper_nl_pre_doit_write),
        doit: Some(net_shaper_nl_group_doit),
        post_doit: Some(net_shaper_nl_post_doit_write),
        policy: net_shaper_group_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_LEAVES,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_CAP_GET,
        pre_doit: Some(net_shaper_nl_cap_pre_doit),
        doit: Some(net_shaper_nl_cap_get_doit),
        post_doit: Some(net_shaper_nl_cap_post_doit),
        policy: net_shaper_cap_get_do_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_CAPS_SCOPE,
        flags: GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: NET_SHAPER_CMD_CAP_GET,
        start: Some(net_shaper_nl_cap_pre_dumpit),
        dumpit: Some(net_shaper_nl_cap_get_dumpit),
        done: Some(net_shaper_nl_cap_post_dumpit),
        policy: net_shaper_cap_get_dump_nl_policy.as_ptr(),
        maxattr: NET_SHAPER_A_CAPS_IFINDEX,
        flags: GENL_CMD_CAP_DUMP,
    },
];

#[no_mangle]
pub static mut net_shaper_nl_family: genl_family = genl_family {
    name: NET_SHAPER_FAMILY_NAME,
    version: NET_SHAPER_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: unsafe { net_shaper_nl_ops.as_ptr() },
    n_split_ops: 7,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
