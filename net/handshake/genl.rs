// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/handshake.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Dependencies supplied by the surrounding kernel/Rust environment:
// netlink, genetlink, genl, linux::handshake, and linux::err.

/* HANDSHAKE_CMD_ACCEPT - do */
static HANDSHAKE_ACCEPT_NL_POLICY: [nla_policy; HANDSHAKE_A_ACCEPT_HANDLER_CLASS + 1] = {
    let mut policy: [nla_policy; HANDSHAKE_A_ACCEPT_HANDLER_CLASS + 1] =
        unsafe { core::mem::zeroed() };
    policy[HANDSHAKE_A_ACCEPT_HANDLER_CLASS] = NLA_POLICY_MAX(NLA_U32, 2);
    policy
};

/* HANDSHAKE_CMD_DONE - do */
static HANDSHAKE_DONE_NL_POLICY: [nla_policy; HANDSHAKE_A_DONE_REMOTE_AUTH + 1] = {
    let mut policy: [nla_policy; HANDSHAKE_A_DONE_REMOTE_AUTH + 1] =
        unsafe { core::mem::zeroed() };
    policy[HANDSHAKE_A_DONE_STATUS] = nla_policy { type_: NLA_POLICY_MAX(NLA_U32, MAX_ERRNO) };
    policy[HANDSHAKE_A_DONE_SOCKFD] = nla_policy { type_: NLA_S32 };
    policy[HANDSHAKE_A_DONE_REMOTE_AUTH] = nla_policy { type_: NLA_U32 };
    policy
};

/* Ops table for handshake */
static HANDSHAKE_NL_OPS: [genl_split_ops; 2] = [
    genl_split_ops {
        cmd: HANDSHAKE_CMD_ACCEPT,
        doit: handshake_nl_accept_doit,
        policy: HANDSHAKE_ACCEPT_NL_POLICY.as_ptr(),
        maxattr: HANDSHAKE_A_ACCEPT_HANDLER_CLASS,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
    genl_split_ops {
        cmd: HANDSHAKE_CMD_DONE,
        doit: handshake_nl_done_doit,
        policy: HANDSHAKE_DONE_NL_POLICY.as_ptr(),
        maxattr: HANDSHAKE_A_DONE_REMOTE_AUTH,
        flags: GENL_ADMIN_PERM | GENL_CMD_CAP_DO,
    },
];

static HANDSHAKE_NL_MCGRPS: [genl_multicast_group; 2] = [
    [HANDSHAKE_NLGRP_NONE] = genl_multicast_group { name: c"none".as_ptr() },
    [HANDSHAKE_NLGRP_TLSHD] = genl_multicast_group { name: c"tlshd".as_ptr() },
];

static mut handshake_nl_family: genl_family = genl_family {
    name: HANDSHAKE_FAMILY_NAME,
    version: HANDSHAKE_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: HANDSHAKE_NL_OPS.as_ptr(),
    n_split_ops: HANDSHAKE_NL_OPS.len(),
    mcgrps: HANDSHAKE_NL_MCGRPS.as_ptr(),
    n_mcgrps: HANDSHAKE_NL_MCGRPS.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
