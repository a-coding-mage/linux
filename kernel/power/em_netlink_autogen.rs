// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from:
 * Documentation/netlink/specs/dev-energymodel.yaml
 */
/* YNL-GEN kernel source */

// C dependencies supplied by the surrounding kernel sources:
// net/netlink.h, net/genetlink.h, em_netlink_autogen.h, and
// uapi/linux/dev_energymodel.h.

/* DEV_ENERGYMODEL_CMD_GET_PERF_DOMAINS - do */
static DEV_ENERGYMODEL_GET_PERF_DOMAINS_NL_POLICY: [nla_policy;
    (DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID + 1) as usize] = {
    let mut policy = [nla_policy { r#type: 0 }; 
        (DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID + 1) as usize];
    policy[DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID as usize] =
        nla_policy { r#type: NLA_U32 };
    policy
};

/* DEV_ENERGYMODEL_CMD_GET_PERF_TABLE - do */
static DEV_ENERGYMODEL_GET_PERF_TABLE_NL_POLICY: [nla_policy;
    (DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID + 1) as usize] = {
    let mut policy = [nla_policy { r#type: 0 };
        (DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID + 1) as usize];
    policy[DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID as usize] =
        nla_policy { r#type: NLA_U32 };
    policy
};

extern "C" {
    fn dev_energymodel_nl_get_perf_domains_doit(
        skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn dev_energymodel_nl_get_perf_domains_dumpit(
        skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    fn dev_energymodel_nl_get_perf_table_doit(
        skb: *mut sk_buff, info: *mut genl_info) -> i32;
}

/* Ops table for dev_energymodel */
static DEV_ENERGYMODEL_NL_OPS: [genl_split_ops; 3] = [
    genl_split_ops {
        cmd: DEV_ENERGYMODEL_CMD_GET_PERF_DOMAINS,
        doit: Some(dev_energymodel_nl_get_perf_domains_doit),
        policy: Some(DEV_ENERGYMODEL_GET_PERF_DOMAINS_NL_POLICY.as_ptr()),
        maxattr: DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID,
        flags: GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: DEV_ENERGYMODEL_CMD_GET_PERF_DOMAINS,
        dumpit: Some(dev_energymodel_nl_get_perf_domains_dumpit),
        flags: GENL_CMD_CAP_DUMP,
    },
    genl_split_ops {
        cmd: DEV_ENERGYMODEL_CMD_GET_PERF_TABLE,
        doit: Some(dev_energymodel_nl_get_perf_table_doit),
        policy: Some(DEV_ENERGYMODEL_GET_PERF_TABLE_NL_POLICY.as_ptr()),
        maxattr: DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID,
        flags: GENL_CMD_CAP_DO,
    },
];

static DEV_ENERGYMODEL_NL_MCGRPS: [genl_multicast_group; 1] = [
    genl_multicast_group { name: b"event\0".as_ptr() as *const i8 },
];

static mut DEV_ENERGYMODEL_NL_FAMILY: genl_family = genl_family {
    name: DEV_ENERGYMODEL_FAMILY_NAME,
    version: DEV_ENERGYMODEL_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: DEV_ENERGYMODEL_NL_OPS.as_ptr(),
    n_split_ops: DEV_ENERGYMODEL_NL_OPS.len(),
    mcgrps: DEV_ENERGYMODEL_NL_MCGRPS.as_ptr(),
    n_mcgrps: DEV_ENERGYMODEL_NL_MCGRPS.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
