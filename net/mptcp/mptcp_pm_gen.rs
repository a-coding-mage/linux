// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from:
 * Documentation/netlink/specs/mptcp_pm.yaml
 * YNL-GEN kernel source
 * To regenerate run: tools/net/ynl/ynl-regen.sh
 *
 * C dependencies are supplied by the surrounding kernel translation unit.
 */

/* Common nested types */
pub static MPTCP_PM_ADDRESS_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ADDR_ATTR_FAMILY */ nla_policy { r#type: NLA_U16 },
    /* MPTCP_PM_ADDR_ATTR_ID */ nla_policy { r#type: NLA_U8 },
    /* MPTCP_PM_ADDR_ATTR_ADDR4 */ nla_policy { r#type: NLA_BE32 },
    /* MPTCP_PM_ADDR_ATTR_ADDR6 */ NLA_POLICY_EXACT_LEN!(16),
    /* MPTCP_PM_ADDR_ATTR_PORT */ nla_policy { r#type: NLA_U16 },
    /* MPTCP_PM_ADDR_ATTR_FLAGS */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ADDR_ATTR_IF_IDX */ nla_policy { r#type: NLA_S32 },
];

/* MPTCP_PM_CMD_ADD_ADDR - do */
pub static MPTCP_PM_ADD_ADDR_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ENDPOINT_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* MPTCP_PM_CMD_DEL_ADDR - do */
pub static MPTCP_PM_DEL_ADDR_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ENDPOINT_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* MPTCP_PM_CMD_GET_ADDR - do */
pub static MPTCP_PM_GET_ADDR_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
];

/* MPTCP_PM_CMD_FLUSH_ADDRS - do */
pub static MPTCP_PM_FLUSH_ADDRS_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ENDPOINT_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* MPTCP_PM_CMD_SET_LIMITS - do */
pub static MPTCP_PM_SET_LIMITS_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_RCV_ADD_ADDRS */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_SUBFLOWS */ nla_policy { r#type: NLA_U32 },
];

/* MPTCP_PM_CMD_GET_LIMITS - do */
pub static MPTCP_PM_GET_LIMITS_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_RCV_ADD_ADDRS */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_SUBFLOWS */ nla_policy { r#type: NLA_U32 },
];

/* MPTCP_PM_CMD_SET_FLAGS - do */
pub static MPTCP_PM_SET_FLAGS_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_ADDR_REMOTE */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* MPTCP_PM_CMD_ANNOUNCE - do */
pub static MPTCP_PM_ANNOUNCE_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
];

/* MPTCP_PM_CMD_REMOVE - do */
pub static MPTCP_PM_REMOVE_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_LOC_ID */ nla_policy { r#type: NLA_U8 },
];

/* MPTCP_PM_CMD_SUBFLOW_CREATE - do */
pub static MPTCP_PM_SUBFLOW_CREATE_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_ADDR_REMOTE */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* MPTCP_PM_CMD_SUBFLOW_DESTROY - do */
pub static MPTCP_PM_SUBFLOW_DESTROY_NL_POLICY: &[nla_policy] = &[
    /* MPTCP_PM_ATTR_ADDR */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
    /* MPTCP_PM_ATTR_TOKEN */ nla_policy { r#type: NLA_U32 },
    /* MPTCP_PM_ATTR_ADDR_REMOTE */ NLA_POLICY_NESTED!(MPTCP_PM_ADDRESS_NL_POLICY),
];

/* Ops table for mptcp_pm */
pub static MPTCP_PM_NL_OPS: &[genl_ops] = &[
    genl_ops { cmd: MPTCP_PM_CMD_ADD_ADDR, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_add_addr_doit, dumpit: None, policy: MPTCP_PM_ADD_ADDR_NL_POLICY, maxattr: MPTCP_PM_ENDPOINT_ADDR, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_DEL_ADDR, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_del_addr_doit, dumpit: None, policy: MPTCP_PM_DEL_ADDR_NL_POLICY, maxattr: MPTCP_PM_ENDPOINT_ADDR, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_GET_ADDR, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_get_addr_doit, dumpit: Some(mptcp_pm_nl_get_addr_dumpit), policy: MPTCP_PM_GET_ADDR_NL_POLICY, maxattr: MPTCP_PM_ATTR_TOKEN, flags: 0 },
    genl_ops { cmd: MPTCP_PM_CMD_FLUSH_ADDRS, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_flush_addrs_doit, dumpit: None, policy: MPTCP_PM_FLUSH_ADDRS_NL_POLICY, maxattr: MPTCP_PM_ENDPOINT_ADDR, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_SET_LIMITS, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_set_limits_doit, dumpit: None, policy: MPTCP_PM_SET_LIMITS_NL_POLICY, maxattr: MPTCP_PM_ATTR_SUBFLOWS, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_GET_LIMITS, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_get_limits_doit, dumpit: None, policy: MPTCP_PM_GET_LIMITS_NL_POLICY, maxattr: MPTCP_PM_ATTR_SUBFLOWS, flags: 0 },
    genl_ops { cmd: MPTCP_PM_CMD_SET_FLAGS, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_set_flags_doit, dumpit: None, policy: MPTCP_PM_SET_FLAGS_NL_POLICY, maxattr: MPTCP_PM_ATTR_ADDR_REMOTE, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_ANNOUNCE, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_announce_doit, dumpit: None, policy: MPTCP_PM_ANNOUNCE_NL_POLICY, maxattr: MPTCP_PM_ATTR_TOKEN, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_REMOVE, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_remove_doit, dumpit: None, policy: MPTCP_PM_REMOVE_NL_POLICY, maxattr: MPTCP_PM_ATTR_LOC_ID, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_SUBFLOW_CREATE, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_subflow_create_doit, dumpit: None, policy: MPTCP_PM_SUBFLOW_CREATE_NL_POLICY, maxattr: MPTCP_PM_ATTR_ADDR_REMOTE, flags: GENL_UNS_ADMIN_PERM },
    genl_ops { cmd: MPTCP_PM_CMD_SUBFLOW_DESTROY, validate: GENL_DONT_VALIDATE_STRICT, doit: mptcp_pm_nl_subflow_destroy_doit, dumpit: None, policy: MPTCP_PM_SUBFLOW_DESTROY_NL_POLICY, maxattr: MPTCP_PM_ATTR_ADDR_REMOTE, flags: GENL_UNS_ADMIN_PERM },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
