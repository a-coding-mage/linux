// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/lockd.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies supplied by the surrounding kernel translation unit:
// net/netlink.h, net/genetlink.h, "netlink.h", and uapi/linux/lockd_netlink.h

/* LOCKD_CMD_SERVER_SET - do */
static const lockd_server_set_nl_policy: [nla_policy; (LOCKD_A_SERVER_UDP_PORT + 1) as usize] = [
    /* LOCKD_A_SERVER_GRACETIME */
    nla_policy { type_: NLA_U32 },
    /* LOCKD_A_SERVER_TCP_PORT */
    nla_policy { type_: NLA_U16 },
    /* LOCKD_A_SERVER_UDP_PORT */
    nla_policy { type_: NLA_U16 },
];

/* Ops table for lockd */
static lockd_nl_ops: [genl_split_ops; 2] = [
    genl_split_ops {
        cmd: LOCKD_CMD_SERVER_SET,
        doit: Some(lockd_nl_server_set_doit),
        policy: Some(&lockd_server_set_nl_policy),
        maxattr: LOCKD_A_SERVER_UDP_PORT,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: LOCKD_CMD_SERVER_GET,
        doit: Some(lockd_nl_server_get_doit),
        flags: GENL_CMD_CAP_DO,
        ..unsafe { core::mem::zeroed() }
    },
];

static mut lockd_nl_family: genl_family = genl_family {
    name: LOCKD_FAMILY_NAME,
    version: LOCKD_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: lockd_nl_ops.as_ptr(),
    n_split_ops: lockd_nl_ops.len(),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
